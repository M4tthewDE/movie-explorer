use anyhow::{Context, Result};
use sqlx::{Pool, Postgres};
use tracing::info;

use crate::{db, tmdb, Config};

pub async fn scrape(config: &Config, pool: &Pool<Postgres>) -> Result<()> {
    info!("starting scraper");
    info!("fetching initial movies");

    let movies = tmdb::discover_movies(&config.access_token).await?;
    let tmdb_id = movies
        .results
        .first()
        .context("no movies found in 'discover'")?
        .id;

    scrape_movie(config, pool, tmdb_id).await?;
    Ok(())
}

async fn scrape_movie(config: &Config, pool: &Pool<Postgres>, tmdb_id: u64) -> Result<()> {
    info!("scraping movie {tmdb_id}");
    let movie = tmdb::get_movie(&config.access_token, tmdb_id).await?;
    db::movies::insert(pool, movie.id, &movie.title).await?;
    Ok(())
}
