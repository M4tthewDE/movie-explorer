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
        let result = movies
            .results
            .first()
            .context("no movies found in 'discover'")?;

        self.scrape_movies(result.id, &result.title).await
    }

    async fn scrape_movies(&self, root_id: i64, root_title: &str) -> Result<()> {
        let mut stack = vec![(root_id, root_title.to_string())];

        while let Some((movie_id, movie_title)) = stack.pop() {
            info!("scraping movie {movie_id} {}", movie_title);

            let credits = tmdb::get_credits(&self.config.access_token, movie_id).await?;

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

                    // Insert the movie and the edge into the database
                    db::movies::insert(&self.pool, next_movie.id, &next_movie.title).await?;
                    db::edges::insert(&self.pool, movie_id, next_movie.id, member.id).await?;

                    stack.push((next_movie.id, next_movie.title));
                }
            }
        }

        Ok(())
    }
}
