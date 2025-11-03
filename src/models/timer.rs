use crate::utils::database::respository::Repository;
use anyhow::{Result, anyhow};
use sqlx::{FromRow, sqlite::SqlitePool};

#[derive(Debug, Clone, FromRow)]
pub struct Timer {
    pub id: i64,
    pub is_recurring: bool,
    pub paused_at: i64,  // Unix timestamp
    pub ends_at: i64,    // Unix timestamp
    pub created_at: i64, // Unix timestamp
}

impl Timer {
    pub fn new(duration_seconds: i32, is_recurring: bool) -> Self {
        Self {
            id: 0,
            is_recurring: is_recurring,
            paused_at: 0,
            ends_at: chrono::Utc::now().timestamp() + duration_seconds as i64,
            created_at: chrono::Utc::now().timestamp(),
        }
    }

    pub fn is_active(&self) -> bool {
        let now = chrono::Utc::now().timestamp();
        now < self.ends_at as i64
    }
}

impl Repository<Timer> for Timer {
    async fn insert(pool: &SqlitePool, item: &Timer) -> Result<Timer> {
        let result = sqlx::query(
            "INSERT INTO timers (paused_at, ends_at, is_recurring, created_at) VALUES (?, ?, ?, ?)",
        )
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

    async fn get_all(pool: &sqlx::SqlitePool) -> Result<Vec<Timer>> {
        let timers = sqlx::query_as::<_, Timer>("SELECT * FROM timers ORDER BY ends_at ASC")
            .fetch_all(pool)
            .await?;
        Ok(timers)
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
