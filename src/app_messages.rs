// SPDX-License-Identifier: MIT

use std::{fs::File, sync::Arc};

use crate::{
    config::Config, models::Timer, pages::power_controls, utils::database::SQLiteDatabase,
};

/// Database-related messages
#[derive(Debug, Clone)]
pub enum DatabaseMessage {
    Initialized(Result<SQLiteDatabase, String>),
    FailedToInitialize(String),
}

/// Power management-related messages
#[derive(Debug, Clone)]
pub enum PowerMessage {
    ToggleStayAwake,
    InhibitAcquired(Arc<Result<File, String>>),
    SetSuspendTime(i32),
    SetLogoutTime(i32),
    SetShutdownTime(i32),
    ExecuteSuspend,
    ExecuteLogout,
    ExecuteShutdown,
}

/// Timer-related messages
#[derive(Debug, Clone)]
pub enum TimerMessage {
    Created(Result<Timer, String>),
    ActiveFetched(Result<Vec<Timer>, String>),
}

/// Application-level messages
#[derive(Debug, Clone)]
pub enum AppMessage {
    TogglePopup,
    UpdateConfig(Config),
    Tick,
    PowerControlsMessage(power_controls::Message),
    DatabaseMessage(DatabaseMessage),
    TimerMessage(TimerMessage),
    PowerMessage(PowerMessage),
}

/// Conversion implementations for page-level message routing
impl From<power_controls::Message> for AppMessage {
    fn from(msg: power_controls::Message) -> Self {
        AppMessage::PowerControlsMessage(msg)
    }
}
