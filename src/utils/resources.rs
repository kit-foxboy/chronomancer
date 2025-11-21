use anyhow::{Context, Result};
use cosmic::{Element, widget};
use std::{fs::File, os::fd::OwnedFd as StdOwnedFd};

use zbus::{Connection, Proxy, zvariant::OwnedFd};

/// Load a system icon using `icon::from_name`
#[must_use]
pub fn system_icon<Message: 'static>(name: &str, size: u16) -> Element<'static, Message> {
    widget::icon::from_name(name)
        .size(size)
        .symbolic(true)
        .icon()
        .into()
}

/// Acquire a systemd-logind suspend inhibitor lock.
///
/// Returns a File handle that represents the inhibitor lock. Keep this alive
/// to prevent the system from suspending. Drop it to release the lock.
///
/// # Arguments
/// * `who` - Application name (e.g., "Chronomancer")
/// * `reason` - Reason for inhibiting (e.g., "User requested stay-awake mode")
/// * `mode` - "block" to block suspend, "delay" to delay it
///
/// # Errors
///
/// Returns an error if:
/// - Failed to connect to the system D-Bus
/// - The D-Bus call to Inhibit fails
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

/// Release a suspend inhibitor lock by dropping the file handle.
/// This is just an explicit wrapper around `drop()` for clarity.
pub fn release_suspend_inhibit(file: File) {
    drop(file);
}

/// Execute system suspend by calling the login1 D-Bus API.
///
/// # Errors
///
/// Returns an error if:
/// - Failed to connect to system bus
/// - The D-Bus call to Suspend fails
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

/// Execute system shutdown by calling the login1 D-Bus API.
///
/// # Errors
///
/// Returns an error if:
/// - Failed to connect to system bus
/// - The D-Bus call to PowerOff fails
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

/// Execute a system logout by calling the login1 D-Bus API to terminate the session.
///
/// # Errors
///
/// Returns an error if:
/// - The `XDG_SESSION_ID` environment variable is not set
/// - Failed to connect to system bus
/// - The D-Bus call to TerminateSession fails
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
