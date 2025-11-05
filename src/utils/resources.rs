use anyhow::{Context, Result};
use cosmic::{widget, Element};
use std::fs::File;
use std::os::fd::OwnedFd as StdOwnedFd;
use zbus::zvariant::OwnedFd;
use zbus::{Connection, Proxy};

/// Load a system icon using icon::from_name
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
/// * `why` - Reason for inhibiting (e.g., "User requested stay-awake mode")
/// * `mode` - "block" to block suspend, "delay" to delay it
pub async fn acquire_suspend_inhibit(who: &str, why: &str, mode: &str) -> Result<File> {
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
        .call("Inhibit", &("sleep", who, why, mode))
        .await
        .context("D-Bus call to Inhibit failed")?;

    let std_fd: StdOwnedFd = owned_fd.into();
    Ok(File::from(std_fd))
}

/// Release a suspend inhibitor lock by dropping the file handle.
/// This is just an explicit wrapper around drop() for clarity.
pub fn release_suspend_inhibit(file: File) {
    drop(file);
}
