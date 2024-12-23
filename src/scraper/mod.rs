use std::{fs::File, io::BufRead, ops::Range, time::Instant};

use anyhow::Result;
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::info;

use crate::{
    db,
    tmdb::{self},
    Config,
};

pub struct Scraper {
    config: Config,
    pool: Pool<Postgres>,
}

const TASK_COUNT: i64 = 20;

impl Scraper {
    pub fn new(config: Config, pool: Pool<Postgres>) -> Self {
        Self { config, pool }
    }

    pub async fn scrape(&self) -> Result<()> {
        let people_count = if self.config.import {
            info!("loading movies from disk");
            let movie_count = self.load_movies().await?;
            info!("loaded {} movies from disk", movie_count);

            info!("loading people from disk");
            let people_count = self.load_people().await?;
            info!("loaded {} people from disk", people_count);
            people_count as i64
        } else {
            db::people::count(&self.pool).await?
        };

        let range_size = people_count as i64 / TASK_COUNT;
        let mut ranges = vec![];

        for i in 0..TASK_COUNT {
            if i == TASK_COUNT - 1 && TASK_COUNT != 1 {
                ranges.push(i * range_size..people_count + 1);
            } else if i == 0 {
                ranges.push(1..range_size);
            } else {
                ranges.push(i * range_size..(i + 1) * range_size);
            }
        }

        info!("starting progress tracker");
        let (tx, rx) = tokio::sync::mpsc::channel(1024);
        self.progress_tracker(people_count, rx).await;

        let mut handles = Vec::new();
        info!("starting scraper tasks");
        for range in ranges {
            let pool = self.pool.clone();
            let access_token = self.config.access_token.clone();
            let t = tx.clone();
            let handle =
                tokio::spawn(
                    async move { Self::scrape_people(&pool, &access_token, range, t).await },
                );

            handles.push(handle);
        }

        for handle in handles {
            handle.await??;
        }

        Ok(())
    }

    async fn scrape_people(
        pool: &Pool<Postgres>,
        access_token: &str,
        range: Range<i64>,
        tx: Sender<i64>,
    ) -> Result<()> {
        for i in range {
            let person_id = db::people::get_tmdb_id(pool, i).await? as i64;
            let movies = tmdb::discover_movies_by_cast(access_token, person_id).await?;

            // TODO: insert all edges at once
            for movie1 in &movies.results {
                for movie2 in &movies.results {
                    db::edges::insert(pool, movie1.id, movie2.id, person_id).await?;
                }
            }
            tx.send(person_id).await?;
        }

        Ok(())
    }

    async fn progress_tracker(&self, total: i64, mut rx: Receiver<i64>) {
        tokio::spawn(async move {
            let mut count = 0;
            let mut last_100 = Instant::now();

            while rx.recv().await.is_some() {
                count += 1;

                if count % 100 == 0 {
                    info!(
                        "{:.4}% | {:?}/p",
                        count as f64 / total as f64,
                        last_100.elapsed() / 100
                    );

                    last_100 = Instant::now();
                }
            }
        });
    }

    async fn load_movies(&self) -> Result<usize> {
        let file = File::open(&self.config.movie_path)?;
        let reader = std::io::BufReader::new(file);

        let mut movies = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let movie: Movie = serde_json::from_str(&line)?;
            movies.push(movie);
        }

        db::movies::bulk_insert(&self.pool, &movies).await?;

        Ok(movies.len())
    }

    async fn load_people(&self) -> Result<usize> {
        let file = File::open(&self.config.person_path)?;
        let reader = std::io::BufReader::new(file);

        let mut people = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let person: Person = serde_json::from_str(&line)?;
            people.push(person);
        }

        db::people::bulk_insert(&self.pool, &people).await?;

        Ok(people.len())
    }
}

#[derive(Deserialize, Debug)]
pub struct Movie {
    pub id: i64,
    pub original_title: String,
}

#[derive(Deserialize, Debug)]
pub struct Person {
    pub id: i64,
    pub name: String,
}
