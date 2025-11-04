#[derive(Debug, Clone)]
pub enum PowerMessage {
    ToggleStayAwake,
    SetSuspendTime(String),
    SetLogoutTime(String),
    SetShutdownTime(String),
    ExecuteSuspend,
    ExecuteLogout,
    ExecuteShutdown,
}
