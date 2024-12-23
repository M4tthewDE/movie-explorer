use anyhow::Result;
use console_subscriber::ConsoleLayer;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry::KeyValue;
use opentelemetry_otlp::SpanExporter;
use opentelemetry_sdk::trace::{Sampler, TracerProvider};
use opentelemetry_sdk::{runtime, Resource};
use opentelemetry_semantic_conventions::resource::{SERVICE_NAME, SERVICE_VERSION};
use opentelemetry_semantic_conventions::SCHEMA_URL;
use scraper::Scraper;
use sqlx::postgres::PgPoolOptions;
use std::path::PathBuf;
use tracing::{info, level_filters::LevelFilter};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{prelude::*, EnvFilter};

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

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    let otlp_exporter = SpanExporter::builder().with_tonic().build().unwrap();
    let tracer = TracerProvider::builder()
        .with_sampler(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(
            1.0,
        ))))
        .with_batch_exporter(otlp_exporter, runtime::Tokio)
        .with_resource(resource())
        .build()
        .tracer("movie-explorer");

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_filter(env_filter))
        .with(ConsoleLayer::builder().with_default_env().spawn())
        .with(OpenTelemetryLayer::new(tracer))
        .init();

    let config = Config::new()?;

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&config.connection_string)
        .await?;

    db::setup(&pool, config.import).await?;
    let scraper = Scraper::new(config, pool);
    scraper.scrape().await
}

fn resource() -> Resource {
    Resource::from_schema_url(
        [
            KeyValue::new(SERVICE_NAME, env!("CARGO_PKG_NAME")),
            KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
        ],
        SCHEMA_URL,
    )
}
