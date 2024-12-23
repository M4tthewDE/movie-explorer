use anyhow::Result;
use sqlx::{Pool, Postgres, QueryBuilder};

use crate::scraper::Movie;

pub async fn bulk_insert(pool: &Pool<Postgres>, movies: &[Movie]) -> Result<()> {
    for chunk in movies.chunks(1_000) {
        let mut query_builder = QueryBuilder::new("INSERT INTO movies (tmdb_id, title) ");
        query_builder.push_values(chunk, |mut b, movie| {
            b.push_bind(movie.id).push_bind(&movie.original_title);
        });

        let query = query_builder.build();
        query.execute(pool).await?;
    }

    Ok(())
}
