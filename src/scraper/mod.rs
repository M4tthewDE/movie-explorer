use std::time::Duration;

use anyhow::{Context, Result};
use sqlx::{Pool, Postgres};
use tracing::info;

use crate::{
    db,
    tmdb::{self, DiscoverMoviesResult},
    Config,
};

pub struct Scraper {
    config: Config,
    pool: Pool<Postgres>,
}

impl Scraper {
    pub fn new(config: Config, pool: Pool<Postgres>) -> Self {
        Self { config, pool }
    }

    // TODO: load all movies and people up front using data dumps
    // then
    // then batch movies and only calculate edges
    // no movie or person will be visited multiple times
    pub async fn scrape(&self) -> Result<()> {
        info!("starting progress tracker");
        self.progress_tracker().await;
        info!("starting scraper");
        info!("fetching initial movies");

        let movies = tmdb::discover_movies(&self.config.access_token).await?;
        let results: Vec<DiscoverMoviesResult> = movies.results[0..10].to_vec();

        let mut handles = Vec::new();

        for result in results {
            let pool = self.pool.clone();
            let access_token = self.config.access_token.clone();
            let handle =
                tokio::spawn(
                    async move { Self::scrape_movies(&pool, &access_token, result.id).await },
                );

            handles.push(handle);
        }

        for handle in handles {
            handle.await??;
        }

        Ok(())
    }

    async fn scrape_movies(pool: &Pool<Postgres>, access_token: &str, root_id: i64) -> Result<()> {
        let mut stack = vec![root_id];

        while let Some(movie_id) = stack.pop() {
            let credits = tmdb::get_credits(access_token, movie_id).await?;

            for member in credits.cast {
                if db::people::exists(pool, member.id).await? {
                    continue;
                }

                db::people::insert(pool, member.id, &member.name).await?;

                let next_movies = tmdb::discover_movies_by_cast(access_token, member.id).await?;

                for next_movie in next_movies.results {
                    if db::movies::exists(pool, next_movie.id).await? {
                        db::edges::insert(pool, movie_id, next_movie.id, member.id).await?;
                        continue;
                    }

                    // Insert the movie and the edge into the database
                    db::movies::insert(pool, next_movie.id, &next_movie.title).await?;
                    db::edges::insert(pool, movie_id, next_movie.id, member.id).await?;

                    stack.push(next_movie.id);
                }
            }
        }

        Ok(())
    }

    async fn progress_tracker(&self) {
        let pool = self.pool.clone();
        tokio::spawn(async move {
            let mut last_count = 0;
            let interval = 5;
            loop {
                tokio::time::sleep(Duration::from_secs(interval)).await;
                let count = db::movies::count(&pool)
                    .await
                    .context("progress tracker has crashed")
                    .unwrap();

                info!(
                    "movies per second: {}",
                    (count - last_count) / interval as i64
                );

                last_count = count;
            }
        });
    }
}
