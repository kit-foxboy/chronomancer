use cosmic::widget::Id;
use std::{fs::File, sync::Arc};

use crate::{
    config::Config,
    models::Timer,
    utils::{TimeUnit, database::SQLiteDatabase},
};

#[derive(Debug, Clone)]
pub enum ComponentMessage {
    TextChanged(String),
    TimeUnitChanged(TimeUnit),
    SubmitPressed,
    RadioOptionSelected(usize),
}

#[derive(Debug, Clone)]
pub enum PageMessage {
    PowerFormSubmitted(i32),
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
    SetSuspendTime(i32),
    SetLogoutTime(i32),
    SetShutdownTime(i32),
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
