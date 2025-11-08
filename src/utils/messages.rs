use std::{fs::File, sync::Arc};
use cosmic::widget::Id;

use crate::{config::Config, models::Timer, utils::{database::SQLiteDatabase, TimeUnit}};

#[derive(Debug, Clone)]
pub enum ComponentMessage {
    TextChanged(String),
    TimeUnitChanged(TimeUnit),
    SubmitPressed(),
}

#[derive(Debug, Clone)]
pub enum PageMessage {
    StayAwakeButtonPressed,
    FormSubmitted(Id),
    ComponentMessage(ComponentMessage),
}

#[derive(Debug, Clone)]
pub enum DatabaseMessage {
    Initialized(Result<SQLiteDatabase, String>),
    FailedToInitialize(String),
}

#[derive(Debug, Clone)]
pub enum PowerMessage {
    ToggleStayAwake,
    InhibitAcquired(Arc<Result<File, String>>),
    SetSuspendTime(u32),
    SetLogoutTime(u32),
    SetShutdownTime(u32),
    ExecuteSuspend,
    ExecuteLogout,
    ExecuteShutdown,
}

#[derive(Debug, Clone)]
pub enum TimerMessage {
    New(i32, bool),
    Created(Result<Timer, String>),
    ActiveFetched(Result<Vec<Timer>, String>),
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    TogglePopup,
    UpdateConfig(Config),
    Tick,
    PageMessage(PageMessage),
    DatabaseMessage(DatabaseMessage),
    TimerMessage(TimerMessage),
    PowerMessage(PowerMessage),
}

impl From<PageMessage> for AppMessage {
    fn from(msg: PageMessage) -> Self {
        AppMessage::PageMessage(msg)
    }
}

impl From<ComponentMessage> for PageMessage {
    fn from(msg: ComponentMessage) -> Self {
        PageMessage::ComponentMessage(msg)
    }
}
