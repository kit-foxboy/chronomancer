// SPDX-License-Identifier: MIT

//! Power management message types.
//!
//! This module defines messages related to power management operations,
//! including suspend inhibitor locks and timed power operations (suspend,
//! logout, shutdown, reboot).

use std::{fs::File, sync::Arc};

/// Messages related to power management operations.
///
/// Handles stay-awake inhibit locks, timed power operations (suspend, logout,
/// shutdown, reboot), and immediate execution of those operations. Inhibit
/// locks prevent the system from sleeping while active without overriding user settings.
#[derive(Debug, Clone)]
pub enum PowerMessage {
    /// Toggle the stay-awake inhibit lock on/off
    ToggleStayAwake,
    /// Result of acquiring a systemd inhibit lock (wrapped in Arc for cheap cloning)
    InhibitAcquired(Arc<Result<File, String>>),
    /// Schedule a suspend operation after the given number of seconds
    SetSuspendTime(i32),
    /// Schedule a logout operation after the given number of seconds
    SetLogoutTime(i32),
    /// Schedule a shutdown operation after the given number of seconds
    SetShutdownTime(i32),
    /// Schedule a reboot operation after the given number of seconds
    SetRebootTime(i32),
    /// Immediately execute a system suspend
    ExecuteSuspend,
    /// Immediately execute a user logout
    ExecuteLogout,
    /// Immediately execute a system shutdown
    ExecuteShutdown,
    /// Immediately execute a system reboot
    ExecuteReboot,
}
