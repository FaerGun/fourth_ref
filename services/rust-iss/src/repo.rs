use crate::domain::{IssLog, OsdrItem, SpaceCache};
use crate::error::ApiError;
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{PgPool, Row};

pub struct IssRepo;

impl IssRepo {
    pub async fn insert(pool: &PgPool, source_url: &str, payload: Value) -> Result<(), ApiError> {
        sqlx::query(
            "INSERT INTO iss_fetch_log (source_url, payload) VALUES ($1, $2)"
        )
        .bind(source_url)
        .bind(payload)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn get_last(pool: &PgPool) -> Result<Option<IssLog>, ApiError> {
        let row = sqlx::query(
            "SELECT id, fetched_at, source_url, payload
             FROM iss_fetch_log
             ORDER BY id DESC LIMIT 1"
        )
        .fetch_optional(pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(IssLog {
                id: row.get("id"),
                fetched_at: row.get("fetched_at"),
                source_url: row.get("source_url"),
                payload: row.get("payload"),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_last_two(pool: &PgPool) -> Result<Vec<(DateTime<Utc>, Value)>, ApiError> {
        let rows = sqlx::query(
            "SELECT fetched_at, payload FROM iss_fetch_log ORDER BY id DESC LIMIT 2"
        )
        .fetch_all(pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| (r.get("fetched_at"), r.get("payload")))
            .collect())
    }
}

pub struct OsdrRepo;

impl OsdrRepo {
    pub async fn upsert(
        pool: &PgPool,
        dataset_id: Option<String>,
        title: Option<String>,
        status: Option<String>,
        updated_at: Option<DateTime<Utc>>,
        raw: Value,
    ) -> Result<(), ApiError> {
        if let Some(ds_id) = &dataset_id {
            sqlx::query(
                "INSERT INTO osdr_items(dataset_id, title, status, updated_at, raw)
                 VALUES($1,$2,$3,$4,$5)
                 ON CONFLICT (dataset_id) DO UPDATE
                 SET title=EXCLUDED.title, status=EXCLUDED.status,
                     updated_at=EXCLUDED.updated_at, raw=EXCLUDED.raw"
            )
            .bind(ds_id)
            .bind(&title)
            .bind(&status)
            .bind(&updated_at)
            .bind(&raw)
            .execute(pool)
            .await?;
        } else {
            sqlx::query(
                "INSERT INTO osdr_items(dataset_id, title, status, updated_at, raw)
                 VALUES($1,$2,$3,$4,$5)"
            )
            .bind::<Option<String>>(None)
            .bind(&title)
            .bind(&status)
            .bind(&updated_at)
            .bind(&raw)
            .execute(pool)
            .await?;
        }
        Ok(())
    }

    pub async fn list(pool: &PgPool, limit: i64) -> Result<Vec<OsdrItem>, ApiError> {
        let rows = sqlx::query(
            "SELECT id, dataset_id, title, status, updated_at, inserted_at, raw
             FROM osdr_items
             ORDER BY inserted_at DESC
             LIMIT $1"
        )
        .bind(limit)
        .fetch_all(pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| OsdrItem {
                id: r.get("id"),
                dataset_id: r.get("dataset_id"),
                title: r.get("title"),
                status: r.get("status"),
                updated_at: r.get("updated_at"),
                inserted_at: r.get("inserted_at"),
                raw: r.get("raw"),
            })
            .collect())
    }

    pub async fn count(pool: &PgPool) -> Result<i64, ApiError> {
        let row = sqlx::query("SELECT count(*) AS c FROM osdr_items")
            .fetch_one(pool)
            .await?;
        Ok(row.get::<i64, _>("c"))
    }
}

pub struct CacheRepo;

impl CacheRepo {
    pub async fn insert(pool: &PgPool, source: &str, payload: Value) -> Result<(), ApiError> {
        sqlx::query("INSERT INTO space_cache(source, payload) VALUES ($1,$2)")
            .bind(source)
            .bind(payload)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn get_latest(pool: &PgPool, source: &str) -> Result<Option<SpaceCache>, ApiError> {
        let row = sqlx::query(
            "SELECT id, source, fetched_at, payload FROM space_cache
             WHERE source = $1 ORDER BY id DESC LIMIT 1"
        )
        .bind(source)
        .fetch_optional(pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(SpaceCache {
                id: row.get("id"),
                source: row.get("source"),
                fetched_at: row.get("fetched_at"),
                payload: row.get("payload"),
            }))
        } else {
            Ok(None)
        }
    }
}


