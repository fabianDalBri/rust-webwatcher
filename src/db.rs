use sqlx::{SqlitePool};
use std::time::Duration;

pub type DbPool = SqlitePool;

/// Create a DB pool and run migrations.
pub async fn init_db(database_url: &str) -> Result<DbPool, sqlx::Error> {
    let pool = SqlitePool::builder()
        .max_size(5)
        .connect_timeout(Duration::from_secs(5))
        .build(database_url)
        .await?;

    run_migrations(&pool).await?;
    Ok(pool)
}

async fn run_migrations(pool: &DbPool) -> Result<(), sqlx::Error> {
    // sites table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS sites (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            url TEXT NOT NULL UNIQUE,
            selector TEXT NULL,
            check_interval_seconds INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            last_checked_at TEXT NULL,
            last_snapshot_id INTEGER NULL
        );
        "#,
    )
    .execute(pool)
    .await?;

    // snapshots table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS snapshots (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            site_id INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            content_hash TEXT NOT NULL,
            content TEXT NOT NULL,
            FOREIGN KEY(site_id) REFERENCES sites(id)
        );
        "#,
    )
    .execute(pool)
    .await?;

    // changes table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS changes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            site_id INTEGER NOT NULL,
            old_snapshot_id INTEGER NULL,
            new_snapshot_id INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            summary TEXT NOT NULL,
            FOREIGN KEY(site_id) REFERENCES sites(id),
            FOREIGN KEY(old_snapshot_id) REFERENCES snapshots(id),
            FOREIGN KEY(new_snapshot_id) REFERENCES snapshots(id)
        );
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
