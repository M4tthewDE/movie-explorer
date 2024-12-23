use anyhow::Result;
use sqlx::{Pool, Postgres};

pub async fn insert(
    pool: &Pool<Postgres>,
    previous_node: i64,
    next_node: i64,
    person: i64,
) -> Result<()> {
    sqlx::query(
        "INSERT INTO edges (previous_node, next_node, person) 
            VALUES ($1, $2, $3) ON CONFLICT DO NOTHING
        ",
    )
    .bind(previous_node)
    .bind(next_node)
    .bind(person)
    .execute(pool)
    .await?;

    Ok(())
}
