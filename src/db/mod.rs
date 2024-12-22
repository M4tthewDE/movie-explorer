use anyhow::Result;
use sqlx::{Pool, Postgres};

pub async fn setup(pool: &Pool<Postgres>) -> Result<()> {
    drop_tables(pool).await?;
    setup_movies(pool).await?;
    setup_people(pool).await?;
    setup_edges(pool).await?;
    Ok(())
}

async fn drop_tables(pool: &Pool<Postgres>) -> Result<()> {
    sqlx::query("DROP TABLE IF EXISTS edges")
        .execute(pool)
        .await?;
    sqlx::query("DROP TABLE IF EXISTS movies")
        .execute(pool)
        .await?;
    sqlx::query("DROP TABLE IF EXISTS people")
        .execute(pool)
        .await?;

    Ok(())
}

async fn setup_movies(pool: &Pool<Postgres>) -> Result<()> {
    sqlx::query(
        "CREATE TABLE movies (
            id SERIAL PRIMARY KEY,
            tmdb_id INTEGER,
            title VARCHAR(255)
        )",
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn setup_people(pool: &Pool<Postgres>) -> Result<()> {
    sqlx::query(
        "CREATE TABLE people (
            id SERIAL PRIMARY KEY,
            tmdb_id INTEGER,
            name VARCHAR(255)
        )",
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn setup_edges(pool: &Pool<Postgres>) -> Result<()> {
    sqlx::query(
        "CREATE TABLE edges (
            previous_node INTEGER REFERENCES movies(id),
            next_node INTEGER REFERENCES movies(id),
            person INTEGER REFERENCES people(id),
            PRIMARY KEY (previous_node, next_node)
        )",
    )
    .execute(pool)
    .await?;

    Ok(())
}
