use crate::db::DbPool;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct Site {
    pub id: i64,
    pub url: String,
    pub selector: Option<String>,
    pub check_interval_seconds: i64,
    pub created_at: String,
    pub last_checked_at: Option<String>,
    pub last_snapshot_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct NewSite {
    pub url: String,
    pub selector: Option<String>,
    pub check_interval_seconds: i64,
}

/// Insert a new site and return the inserted row.
pub async fn create_site(pool: &DbPool, new: NewSite) -> Result<Site, sqlx::Error> {
    let now = Utc::now().to_rfc3339();

    let res = sqlx::query(
        r#"
        INSERT INTO sites (url, selector, check_interval_seconds, created_at, last_checked_at, last_snapshot_id)
        VALUES (?1, ?2, ?3, ?4, NULL, NULL)
        "#,
    )
    .bind(&new.url)
    .bind(&new.selector)
    .bind(new.check_interval_seconds)
    .bind(now)
    .execute(pool)
    .await?;

    let id = res.last_insert_rowid();

    let site = sqlx::query_as::<_, Site>(
        r#"
        SELECT
            id,
            url,
            selector,
            check_interval_seconds,
            created_at,
            last_checked_at,
            last_snapshot_id
        FROM sites
        WHERE id = ?1
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(site)
}

/// Get all sites.
pub async fn get_sites(pool: &DbPool) -> Result<Vec<Site>, sqlx::Error> {
    let sites = sqlx::query_as::<_, Site>(
        r#"
        SELECT
            id,
            url,
            selector,
            check_interval_seconds,
            created_at,
            last_checked_at,
            last_snapshot_id
        FROM sites
        ORDER BY id
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(sites)
}

/// Get a single site by id.
pub async fn get_site(pool: &DbPool, id: i64) -> Result<Site, sqlx::Error> {
    let site = sqlx::query_as::<_, Site>(
        r#"
        SELECT
            id,
            url,
            selector,
            check_interval_seconds,
            created_at,
            last_checked_at,
            last_snapshot_id
        FROM sites
        WHERE id = ?1
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(site)
}
