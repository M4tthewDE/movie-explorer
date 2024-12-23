use anyhow::Result;
use sqlx::Row;
use sqlx::{Pool, Postgres, QueryBuilder};
use tracing::{instrument, trace_span, Instrument};

use crate::scraper::Person;

#[instrument(level = "trace", skip(pool))]
pub async fn bulk_insert(pool: &Pool<Postgres>, people: &[Person]) -> Result<()> {
    for chunk in people.chunks(1_000) {
        let mut query_builder = QueryBuilder::new("INSERT INTO people (tmdb_id, name) ");
        query_builder.push_values(chunk, |mut b, person| {
            b.push_bind(person.id).push_bind(&person.name);
        });

        let query = query_builder.build();
        query
            .execute(pool)
            .instrument(trace_span!("execute"))
            .await?;
    }

    Ok(())
}

#[instrument(level = "trace", skip(pool))]
pub async fn get_tmdb_id(pool: &Pool<Postgres>, id: i64) -> Result<i32> {
    Ok(sqlx::query("SELECT * FROM people WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .instrument(trace_span!("fetch_one"))
        .await?
        .try_get("tmdb_id")?)
}

#[instrument(level = "trace", skip(pool))]
pub async fn count(pool: &Pool<Postgres>) -> Result<i64> {
    Ok(sqlx::query_scalar("SELECT COUNT(*) FROM people")
        .fetch_one(pool)
        .instrument(trace_span!("fetch_one"))
        .await?)
}
