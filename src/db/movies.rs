use anyhow::Result;
use sqlx::{Pool, Postgres};

pub async fn insert(pool: &Pool<Postgres>, tmdb_id: i64, title: &str) -> Result<()> {
    sqlx::query(
        "INSERT INTO movies (tmdb_id, title) 
            VALUES ($1, $2) ON CONFLICT (tmdb_id) DO NOTHING;
        ",
    )
    .bind(tmdb_id)
    .bind(title)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn exists(pool: &Pool<Postgres>, tmdb_id: i64) -> Result<bool> {
    Ok(sqlx::query("SELECT * FROM movies WHERE tmdb_id = $1")
        .bind(tmdb_id)
        .fetch_optional(pool)
        .await?
        .is_some())
}
