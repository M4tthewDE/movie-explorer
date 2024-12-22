use anyhow::{Context, Result};
use sqlx::{Pool, Postgres};
use tracing::info;

use crate::{db, tmdb, Config};

pub struct Scraper {
    config: Config,
    pool: Pool<Postgres>,
}

impl Scraper {
    pub fn new(config: Config, pool: Pool<Postgres>) -> Self {
        Self { config, pool }
    }

    pub async fn scrape(&self) -> Result<()> {
        info!("starting scraper");
        info!("fetching initial movies");

        let movies = tmdb::discover_movies(&self.config.access_token).await?;
        let tmdb_id = movies
            .results
            .first()
            .context("no movies found in 'discover'")?
            .id;

        self.scrape_movie(tmdb_id).await?;
        Ok(())
    }

    async fn scrape_movie(&self, tmdb_id: i64) -> Result<()> {
        let movie = tmdb::get_movie(&self.config.access_token, tmdb_id).await?;
        info!("scraping movie {tmdb_id} {}", movie.title);
        db::movies::insert(&self.pool, movie.id, &movie.title).await?;
        let credits = tmdb::get_credits(&self.config.access_token, movie.id).await?;

        for member in credits.cast {
            if db::people::exists(&self.pool, member.id).await? {
                continue;
            }

            db::people::insert(&self.pool, member.id, &member.name).await?;
            info!("scraping person {}", member.name);
            let next_movies =
                tmdb::discover_movies_by_cast(&self.config.access_token, member.id).await?;

            for next_movie in next_movies.results {
                if db::movies::exists(&self.pool, next_movie.id).await? {
                    continue;
                }
                db::movies::insert(&self.pool, next_movie.id, &next_movie.title).await?;
                db::edges::insert(&self.pool, movie.id, next_movie.id, member.id).await?;
            }
        }

        Ok(())
    }
}
