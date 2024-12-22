use anyhow::Result;
use sqlx::{Pool, Postgres};

pub async fn insert(pool: &Pool<Postgres>, tmdb_id: u64, title: &str) -> Result<()> {
    sqlx::query(
        "INSERT INTO movies (tmdb_id, title) 
            VALUES ($1, $2) 
        ",
    )
    .bind(tmdb_id as i64)
    .bind(title)
    .execute(pool)
    .await?;

    Ok(())
}
