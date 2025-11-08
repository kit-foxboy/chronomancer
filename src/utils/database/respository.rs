use sqlx::SqlitePool;

/// Defines traits for common database repository operations.
// This doesn't account for joins or composite keys but as this is a simple sqlite driven app with likely one model, it's fine
pub trait Repository<T> {

    /// Inserts a new item into the repository.
    async fn insert(pool: &SqlitePool, item: &T) -> anyhow::Result<T>;

    /// Fetches all active items from the repository.
    async fn get_all_active(pool: &SqlitePool) -> anyhow::Result<Vec<T>>;

    /// Retrieves an item by its ID.
    async fn get_by_id(pool: &SqlitePool, id: &i64) -> anyhow::Result<Option<T>>;

    /// Deletes an item by its ID.
    async fn delete_by_id(pool: &SqlitePool, id: &i64) -> anyhow::Result<()>;
}