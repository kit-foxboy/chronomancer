//! System resource utilities for icons and D-Bus power management.
//!
//! This module provides utilities for:
//! - Loading system icons with consistent styling
//! - Interacting with systemd-logind for power management
//! - Managing suspend inhibitor locks
//! - Executing system power operations (suspend, shutdown, reboot, logout)
//!
//! # D-Bus Integration
//!
//! Most functions in this module interact with the systemd-logind D-Bus API
//! to control system power states. These operations require appropriate
//! permissions and may prompt the user for authentication.
//!
//! # Examples
//!
//! ## Loading a system icon
//!
//! ```rust,no_run
//! use chronomancer::utils::resources;
//! use cosmic::Element;
//!
//! #[derive(Clone)]
//! enum Message {}
//!
//! fn create_icon() -> Element<'static, Message> {
//!     resources::system_icon("system-suspend-symbolic", 24)
//! }
//! ```
//!
//! ## Preventing system suspend
//!
//! ```rust,no_run
//! use chronomancer::utils::resources;
//!
//! # async fn example() -> anyhow::Result<()> {
//! // Acquire an inhibitor lock
//! let lock = resources::acquire_suspend_inhibit(
//!     "Chronomancer",
//!     "User requested stay-awake mode",
//!     "block"
//! ).await?;
//!
//! // System won't suspend while lock is held
//! // ... do work ...
//!
//! // Release the lock
//! resources::release_suspend_inhibit(lock);
//! # Ok(())
//! # }
//! ```
//!
//! ## Executing power operations
//!
//! ```rust,no_run
//! use chronomancer::utils::resources;
//!
//! # async fn example() -> anyhow::Result<()> {
//! // Suspend the system
//! resources::execute_system_suspend().await?;
//!
//! // Shutdown the system
//! resources::execute_system_shutdown().await?;
//!
//! // Reboot the system
//! resources::execute_system_reboot().await?;
//!
//! // Logout current session
//! resources::execute_system_logout().await?;
//! # Ok(())
//! # }
//! ```

use anyhow::{Context, Result};
use cosmic::{Element, widget};
use std::{fs::File, os::fd::OwnedFd as StdOwnedFd};

use zbus::{Connection, Proxy, zvariant::OwnedFd};

/// Loads a system icon and returns it as a cosmic [`Element`].
///
/// Creates an icon widget using the system icon theme. The icon is loaded
/// as a symbolic icon (monochrome) which adapts to the current theme.
///
/// # Arguments
///
/// - `name` - Icon name from the freedesktop.org icon naming spec or custom icon
/// - `size` - Icon size in pixels
///
/// # Returns
///
/// An [`Element`] containing the icon widget, ready to be used in a view.
///
/// # Examples
///
/// ```rust,no_run
/// use chronomancer::utils::resources;
/// use cosmic::Element;
///
/// #[derive(Clone)]
/// enum Message {}
///
/// // Load a system icon
/// let suspend_icon: Element<Message> = resources::system_icon("system-suspend-symbolic", 24);
/// let shutdown_icon: Element<Message> = resources::system_icon("system-shutdown-symbolic", 32);
///
/// // Use custom application icons
/// let custom_icon: Element<Message> = resources::system_icon("io.vulpapps.Chronomancer-stay-awake", 36);
/// ```
#[must_use]
pub fn system_icon<Message: 'static>(name: &str, size: u16) -> Element<'static, Message> {
    widget::icon::from_name(name)
        .size(size)
        .symbolic(true)
        .icon()
        .into()
}

/// Acquires a systemd-logind suspend inhibitor lock.
///
/// Creates an inhibitor lock that prevents the system from suspending. The lock
/// is represented by a file descriptor - keep the returned `File` alive to
/// maintain the lock. When the `File` is dropped, the lock is automatically released.
///
/// # Inhibitor Modes
///
/// - `"block"` - Completely prevents suspend until the lock is released
/// - `"delay"` - Delays suspend briefly to allow cleanup (typically a few seconds)
///
/// # Arguments
///
/// - `who` - Application identifier (e.g., "Chronomancer")
/// - `reason` - Human-readable reason for the lock (shown in system logs)
/// - `mode` - Inhibitor mode: "block" or "delay"
///
/// # Returns
///
/// A `File` handle representing the inhibitor lock. Drop it to release the lock.
///
/// # Errors
///
/// Returns an error if:
/// - Failed to connect to the system D-Bus
/// - The D-Bus call to `Inhibit` fails (insufficient permissions, systemd not running, etc.)
///
/// # Examples
///
/// ```rust,no_run
/// use chronomancer::utils::resources;
///
/// # async fn example() -> anyhow::Result<()> {
/// // Acquire a blocking inhibitor lock
/// let lock = resources::acquire_suspend_inhibit(
///     "Chronomancer",
///     "User requested stay-awake mode",
///     "block"
/// ).await?;
///
/// // System cannot suspend while lock exists
/// println!("System suspend is now blocked");
///
/// // Lock is automatically released when dropped
/// drop(lock);
/// println!("System can now suspend");
/// # Ok(())
/// # }
/// ```
///
/// # D-Bus API
///
/// This function calls the systemd-logind D-Bus method:
/// ```text
/// org.freedesktop.login1.Manager.Inhibit(
///     what: "sleep",
///     who: <who>,
///     why: <reason>,
///     mode: <mode>
/// ) -> FileDescriptor
/// ```
pub async fn acquire_suspend_inhibit(who: &str, reason: &str, mode: &str) -> Result<File> {
    let connection = Connection::system()
        .await
        .context("Failed to connect to system bus")?;

    let proxy = Proxy::new(
        &connection,
        "org.freedesktop.login1",
        "/org/freedesktop/login1",
        "org.freedesktop.login1.Manager",
    )
    .await?;

    let (owned_fd,): (OwnedFd,) = proxy
        .call("Inhibit", &("sleep", who, reason, mode))
        .await
        .context("D-Bus call to Inhibit failed")?;

    let std_fd: StdOwnedFd = owned_fd.into();
    Ok(File::from(std_fd))
}

/// Releases a suspend inhibitor lock by dropping the file handle.
///
/// This is an explicit wrapper around `drop()` for clarity and documentation.
/// Dropping the file handle closes the file descriptor, which signals systemd-logind
/// to release the inhibitor lock.
///
/// # Arguments
///
/// - `file` - The inhibitor lock file handle returned by [`acquire_suspend_inhibit`]
///
/// # Examples
///
/// ```rust,no_run
/// use chronomancer::utils::resources;
///
/// # async fn example() -> anyhow::Result<()> {
/// let lock = resources::acquire_suspend_inhibit(
///     "Chronomancer",
///     "Processing task",
///     "block"
/// ).await?;
///
/// // Explicitly release the lock
/// resources::release_suspend_inhibit(lock);
///
/// // Alternatively, just let it drop:
/// // {
/// //     let lock = acquire_suspend_inhibit(...).await?;
/// //     // lock is dropped here automatically
/// // }
/// # Ok(())
/// # }
/// ```
pub fn release_suspend_inhibit(file: File) {
    drop(file);
}

/// Suspends the system to RAM (sleep mode).
///
/// Calls the systemd-logind D-Bus API to suspend the system. This is equivalent
/// to the "Suspend" or "Sleep" option in system menus.
///
/// **Note**: This requires appropriate permissions. The system may prompt the
/// user for authentication depending on `PolicyKit` configuration.
///
/// # Returns
///
/// Returns `Ok(())` if the suspend command was successfully sent.
///
/// # Errors
///
/// Returns an error if:
/// - Failed to connect to the system D-Bus
/// - The D-Bus call to `Suspend` fails
/// - User lacks permission to suspend the system
///
/// # Examples
///
/// ```rust,no_run
/// use chronomancer::utils::resources;
///
/// # async fn example() -> anyhow::Result<()> {
/// // Suspend the system
/// resources::execute_system_suspend().await?;
/// println!("System is suspending...");
/// # Ok(())
/// # }
/// ```
///
/// # D-Bus API
///
/// This function calls:
/// ```text
/// org.freedesktop.login1.Manager.Suspend(interactive: true)
/// ```
pub async fn execute_system_suspend() -> Result<()> {
    let connection = Connection::system()
        .await
        .context("Failed to connect to system bus")?;

    let proxy = Proxy::new(
        &connection,
        "org.freedesktop.login1",
        "/org/freedesktop/login1",
        "org.freedesktop.login1.Manager",
    )
    .await?;

    let _: () = proxy
        .call("Suspend", &(true,))
        .await
        .context("D-Bus call to Suspend failed")?;

    Ok(())
}

/// Powers off the system.
///
/// Calls the systemd-logind D-Bus API to shut down the system. This is equivalent
/// to the "Power Off" or "Shutdown" option in system menus.
///
/// **Warning**: This will immediately shut down the system. Ensure all work is
/// saved before calling this function.
///
/// **Note**: This requires appropriate permissions. The system may prompt the
/// user for authentication depending on `PolicyKit` configuration.
///
/// # Returns
///
/// Returns `Ok(())` if the shutdown command was successfully sent.
///
/// # Errors
///
/// Returns an error if:
/// - Failed to connect to the system D-Bus
/// - The D-Bus call to `PowerOff` fails
/// - User lacks permission to shut down the system
///
/// # Examples
///
/// ```rust,no_run
/// use chronomancer::utils::resources;
///
/// # async fn example() -> anyhow::Result<()> {
/// // Shut down the system
/// resources::execute_system_shutdown().await?;
/// println!("System is shutting down...");
/// # Ok(())
/// # }
/// ```
///
/// # D-Bus API
///
/// This function calls:
/// ```text
/// org.freedesktop.login1.Manager.PowerOff(interactive: true)
/// ```
pub async fn execute_system_shutdown() -> Result<()> {
    let connection = Connection::system()
        .await
        .context("Failed to connect to system bus")?;

    let proxy = Proxy::new(
        &connection,
        "org.freedesktop.login1",
        "/org/freedesktop/login1",
        "org.freedesktop.login1.Manager",
    )
    .await?;

    let _: () = proxy
        .call("PowerOff", &(true,))
        .await
        .context("D-Bus call to PowerOff failed")?;

    Ok(())
}

/// Logs out the current user session.
///
/// Calls the systemd-logind D-Bus API to terminate the current user session.
/// This closes all applications and returns to the login screen.
///
/// **Warning**: This will immediately log out. Ensure all work is saved before
/// calling this function.
///
/// **Note**: This requires the `XDG_SESSION_ID` environment variable to be set
/// (which is normally the case in desktop sessions).
///
/// # Returns
///
/// Returns `Ok(())` if the logout command was successfully sent.
///
/// # Errors
///
/// Returns an error if:
/// - The `XDG_SESSION_ID` environment variable is not set
/// - Failed to connect to the system D-Bus
/// - The D-Bus call to `TerminateSession` fails
/// - User lacks permission to terminate the session
///
/// # Examples
///
/// ```rust,no_run
/// use chronomancer::utils::resources;
///
/// # async fn example() -> anyhow::Result<()> {
/// // Log out current session
/// resources::execute_system_logout().await?;
/// println!("Logging out...");
/// # Ok(())
/// # }
/// ```
///
/// # D-Bus API
///
/// This function calls:
/// ```text
/// org.freedesktop.login1.Manager.TerminateSession(session_id: XDG_SESSION_ID)
/// ```
pub async fn execute_system_logout() -> Result<()> {
    let xdg_session_id =
        std::env::var("XDG_SESSION_ID").context("XDG_SESSION_ID environment variable not set")?;

    let connection = Connection::system()
        .await
        .context("Failed to connect to system bus")?;

    let proxy = Proxy::new(
        &connection,
        "org.freedesktop.login1",
        "/org/freedesktop/login1",
        "org.freedesktop.login1.Manager",
    )
    .await?;

    let _: () = proxy
        .call("TerminateSession", &(xdg_session_id,))
        .await
        .context("D-Bus call to TerminateSession failed")?;

    Ok(())
}

/// Reboots the system.
///
/// Calls the systemd-logind D-Bus API to reboot the system. This is equivalent
/// to the "Restart" or "Reboot" option in system menus.
///
/// **Warning**: This will immediately reboot the system. Ensure all work is
/// saved before calling this function.
///
/// **Note**: This requires appropriate permissions. The system may prompt the
/// user for authentication depending on `PolicyKit` configuration.
///
/// # Returns
///
/// Returns `Ok(())` if the reboot command was successfully sent.
///
/// # Errors
///
/// Returns an error if:
/// - Failed to connect to the system D-Bus
/// - The D-Bus call to `Reboot` fails
/// - User lacks permission to reboot the system
///
/// # Examples
///
/// ```rust,no_run
/// use chronomancer::utils::resources;
///
/// # async fn example() -> anyhow::Result<()> {
/// // Reboot the system
/// resources::execute_system_reboot().await?;
/// println!("System is rebooting...");
/// # Ok(())
/// # }
/// ```
///
/// # D-Bus API
///
/// This function calls:
/// ```text
/// org.freedesktop.login1.Manager.Reboot(interactive: true)
/// ```
pub async fn execute_system_reboot() -> Result<()> {
    let connection = Connection::system()
        .await
        .context("Failed to connect to system bus")?;

    let proxy = Proxy::new(
        &connection,
        "org.freedesktop.login1",
        "/org/freedesktop/login1",
        "org.freedesktop.login1.Manager",
    )
    .await?;

    let _: () = proxy
        .call("Reboot", &(true,))
        .await
        .context("D-Bus call to Reboot failed")?;

    Ok(())
}
