//! Database module for `SQLite` connection and message types.
//!
//! This module provides database connectivity via `SQLite` and defines
//! messages for database lifecycle events and operation results. Cloning is used under the hood via Arc
//! to allow safe sharing of the database connection across async tasks and UI components.
//!

pub mod repository;
pub mod sqlite;

pub use repository::Repository;
pub use sqlite::SQLiteDatabase;

/// Messages related to database operations.
///
/// These messages represent the lifecycle of database initialization and results
/// from database queries. The database is initialized asynchronously on app startup.
#[derive(Debug, Clone)]
pub enum DatabaseMessage {
    /// Database successfully initialized with the given connection
    Initialized(Result<SQLiteDatabase, String>),
    /// Database initialization failed with an error message
    FailedToInitialize(String),
}
