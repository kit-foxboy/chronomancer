#[derive(Debug, Clone)]
pub enum PowerMessage {
    ToggleStayAwake,
    InhibitAcquired(std::sync::Arc<Result<std::fs::File, String>>),
    SetSuspendTime(String),
    SetLogoutTime(String),
    SetShutdownTime(String),
    ExecuteSuspend,
    ExecuteLogout,
    ExecuteShutdown,
}
