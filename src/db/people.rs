use anyhow::Result;
use sqlx::{Pool, Postgres};

pub async fn insert(pool: &Pool<Postgres>, tmdb_id: i64, name: &str) -> Result<()> {
    sqlx::query(
        "INSERT INTO people (tmdb_id, name) 
            VALUES ($1, $2) 
        ",
    )
    .bind(tmdb_id)
    .bind(name)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn exists(pool: &Pool<Postgres>, tmdb_id: i64) -> Result<bool> {
    Ok(sqlx::query("SELECT * FROM people WHERE tmdb_id = $1")
        .bind(tmdb_id)
        .fetch_optional(pool)
        .await?
        .is_some())
}
