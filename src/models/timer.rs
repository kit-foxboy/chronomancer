use crate::utils::database::respository::Repository;
use anyhow::{Result, anyhow};
use sqlx::{FromRow, sqlite::SqlitePool};

#[derive(Debug, Clone, FromRow)]
pub struct Timer {
    pub id: i64,
    pub description: String,
    pub is_recurring: bool,
    pub paused_at: i64,  // Unix timestamp
    pub ends_at: i64,    // Unix timestamp
    pub created_at: i64, // Unix timestamp
}

pub enum TimerType {
    UserDefined(String),
    Suspend,
    Logout,
    Shutdown,
}

impl TimerType {
    pub fn as_str(&self) -> &str {
        match self {
            TimerType::UserDefined(name) => name,
            TimerType::Suspend => "System Suspend",
            TimerType::Logout => "System Logout",
            TimerType::Shutdown => "System Shutdown",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "System Suspend" => TimerType::Suspend,
            "System Logout" => TimerType::Logout,
            "System Shutdown" => TimerType::Shutdown,
            other => TimerType::UserDefined(other.to_string()),
        }
    }
}

impl Timer {
    pub fn new(duration_seconds: i32, is_recurring: bool, timer_type: &TimerType) -> Self {
        Self {
            id: 0,
            description: timer_type.as_str().into(),
            is_recurring,
            paused_at: 0,
            ends_at: chrono::Utc::now().timestamp() + i64::from(duration_seconds),
            created_at: chrono::Utc::now().timestamp(),
        }
    }

    pub fn is_active(&self) -> bool {
        let now = chrono::Utc::now().timestamp();
        now < self.ends_at
    }
}

impl Repository<Timer> for Timer {
    async fn insert(pool: &SqlitePool, item: &Timer) -> Result<Timer> {
        let result = sqlx::query(
            "INSERT INTO timers (description, paused_at, ends_at, is_recurring, created_at) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&item.description)
        .bind(item.paused_at)
        .bind(item.ends_at)
        .bind(item.is_recurring)
        .bind(item.created_at)
        .execute(pool)
        .await?;

        // get inserted item
        let item = Timer::get_by_id(pool, &result.last_insert_rowid()).await?;
        if item.is_none() {
            return Err(anyhow!("Failed to save timer"));
        }

        Ok(item.unwrap())
    }

    async fn get_all_active(pool: &SqlitePool) -> Result<Vec<Timer>> {
        let now = chrono::Utc::now().timestamp();
        let timers = sqlx::query_as::<_, Timer>(
            "SELECT * FROM timers WHERE ends_at > ? ORDER BY ends_at ASC",
        )
        .bind(now)
        .fetch_all(pool)
        .await?;
        Ok(timers)
    }

    async fn get_by_id(pool: &sqlx::SqlitePool, id: &i64) -> Result<Option<Timer>> {
        let timer = sqlx::query_as::<_, Timer>("SELECT * FROM timers WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?;
        Ok(timer)
    }

    async fn delete_by_id(pool: &sqlx::SqlitePool, id: &i64) -> Result<()> {
        sqlx::query("DELETE FROM timers WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
