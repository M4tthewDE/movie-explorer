use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
use std::path::PathBuf;
use tracing::info;

use serde::Deserialize;

mod db;
mod scraper;
mod tmdb;

#[derive(Deserialize)]
struct Config {
    access_token: String,
    connection_string: String,
}

impl Config {
    fn new() -> Result<Self> {
        let path = PathBuf::from("config.toml");
        info!("reading config from {path:?}");
        let config: Config = toml::from_str(&std::fs::read_to_string(path)?)?;
        Ok(config)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();
    let config = Config::new()?;

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.connection_string)
        .await?;

    db::setup(&pool).await?;
    scraper::scrape(&config, &pool).await
}
