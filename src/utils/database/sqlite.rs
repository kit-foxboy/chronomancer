use anyhow::{Result, anyhow};
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};

const APP_ID: &str = "com.github.kit-foxboy.chronomancer";

/// `SQLite` database filename
const DB_VERSION: &str = "1";
const DB_FILENAME: &str = constcat::concat!("chronomancer-v", DB_VERSION, ".db");

#[derive(Clone, Debug)]
pub struct SQLiteDatabase {
    pool: SqlitePool,
}

impl SQLiteDatabase {
    /// Create a new `SQLite` database connection pool
    pub async fn new() -> Result<Self> {
        // Determine the database file path, create if necessary
        let data_dir = dirs::data_local_dir()
            .ok_or_else(|| anyhow!("Failed to get data directory"))?
            .join(APP_ID);
        std::fs::create_dir_all(&data_dir)?;

        let db_path = data_dir.join(DB_FILENAME);
        let db_path_str = db_path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid database path"))?;

        let options = SqliteConnectOptions::new()
            .create_if_missing(true)
            .filename(db_path_str);

        // Create connection pool and run migrations
        let pool = SqlitePool::connect_with(options).await?;
        sqlx::migrate!("./migrations").run(&pool).await?;

        println!("Database migrations completed successfully");

        Ok(Self { pool })
    }

    #[allow(dead_code)]
    pub async fn new_in_memory() -> Result<Self> {
        // Create connection pool and run migrations
        let pool = SqlitePool::connect("sqlite::memory:").await?;
        sqlx::migrate!("./migrations").run(&pool).await?;

        println!("In-memory database migrations completed successfully");

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}
