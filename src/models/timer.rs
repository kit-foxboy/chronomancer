use anyhow::{Result, anyhow};
use sqlx::{FromRow, sqlite::SqlitePool};
use crate::utils::database::respository::Repository;

#[derive(Debug, Clone, FromRow)]
pub struct Timer {
    pub id: i64,
    pub duration_seconds: i32,
    pub is_recurring: bool,
    pub created_at: i64, // Unix timestamp
}

impl Timer {
    pub fn new(duration_seconds: i32, is_recurring: bool) -> Self {
        Self {
            id: 0,
            duration_seconds: duration_seconds,
            is_recurring: is_recurring,
            created_at: chrono::Utc::now().timestamp()
        }
    }
}

impl Repository<Timer> for Timer {

    async fn insert(pool: &SqlitePool, item: &Timer) -> Result<Timer> {
        
        let result = sqlx::query(
            "INSERT INTO timers (duration_seconds, is_recurring, created_at) VALUES (?, ?, ?)",
        )
        .bind(item.duration_seconds)
        .bind(item.is_recurring)
        .bind(item.created_at)
        .execute(pool)
        .await?;

        // get inserted item
        let item = Timer::get_by_id(pool, result.last_insert_rowid()).await?;
        if item.is_none() {
            return Err(anyhow!("Failed to save timer"));
        }

        Ok(item.unwrap())
    }


    async fn get_all(pool: &sqlx::SqlitePool) -> Result<Vec<Timer>> {
        let timers = sqlx::query_as::<_, Timer>("SELECT * FROM timers")
            .fetch_all(pool)
            .await?;
        Ok(timers)
    }

    async fn get_by_id(pool: &sqlx::SqlitePool, id: i64) -> Result<Option<Timer>> {
        let timer = sqlx::query_as::<_, Timer>("SELECT * FROM timers WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?;
        Ok(timer)
    }

    async fn delete_by_id(pool: &sqlx::SqlitePool, id: i64) -> Result<()> {
        sqlx::query("DELETE FROM timers WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}