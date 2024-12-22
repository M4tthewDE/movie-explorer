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
