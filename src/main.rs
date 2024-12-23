use anyhow::Result;
use console_subscriber::ConsoleLayer;
use scraper::Scraper;
use sqlx::postgres::PgPoolOptions;
use std::path::PathBuf;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{layer::SubscriberExt, prelude::*, EnvFilter};

use serde::Deserialize;

mod db;
mod scraper;
mod tmdb;

#[derive(Deserialize)]
struct Config {
    access_token: String,
    connection_string: String,
    movie_path: PathBuf,
    person_path: PathBuf,
    import: bool,
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
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_filter(env_filter))
        .with(ConsoleLayer::builder().with_default_env().spawn())
        .init();

    let config = Config::new()?;

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.connection_string)
        .await?;

    db::setup(&pool, config.import).await?;
    let scraper = Scraper::new(config, pool);
    scraper.scrape().await
}
