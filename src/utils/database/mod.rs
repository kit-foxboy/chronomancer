pub mod sqlite;
pub mod respository;

pub use sqlite::SQLiteDatabase;
pub use respository::Repository;

#[derive(Debug, Clone)]
pub enum DatabaseMessage {
    Initialized(Result<SQLiteDatabase, String>),
    FailedToInitialize(String),
}