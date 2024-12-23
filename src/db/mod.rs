use anyhow::Result;
use sqlx::{Pool, Postgres};

pub mod edges;
pub mod movies;
pub mod people;

pub async fn setup(pool: &Pool<Postgres>, import: bool) -> Result<()> {
    drop_tables(pool, import).await?;

    if import {
        setup_movies(pool).await?;
        setup_people(pool).await?;
    }

    setup_edges(pool).await?;
    Ok(())
}

async fn drop_tables(pool: &Pool<Postgres>, import: bool) -> Result<()> {
    sqlx::query("DROP TABLE IF EXISTS edges")
        .execute(pool)
        .await?;
    if import {
        sqlx::query("DROP TABLE IF EXISTS movies")
            .execute(pool)
            .await?;
        sqlx::query("DROP TABLE IF EXISTS people")
            .execute(pool)
            .await?;
    }

    Ok(())
}

async fn setup_movies(pool: &Pool<Postgres>) -> Result<()> {
    sqlx::query(
        "CREATE TABLE movies (
            id SERIAL PRIMARY KEY,
            tmdb_id INTEGER UNIQUE,
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
            tmdb_id INTEGER UNIQUE,
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
            previous_node INTEGER REFERENCES movies(tmdb_id),
            next_node INTEGER REFERENCES movies(tmdb_id),
            person INTEGER REFERENCES people(tmdb_id),
            PRIMARY KEY (previous_node, next_node)
        )",
    )
    .execute(pool)
    .await?;

    Ok(())
}
