use anyhow::Result;
use sqlx::{Pool, Postgres, QueryBuilder};

pub async fn insert_bulk(pool: &Pool<Postgres>, edges: &[(i64, i64, i64)]) -> Result<()> {
    for chunk in edges.chunks(1_000) {
        let mut query_builder =
            QueryBuilder::new("INSERT INTO edges (previous_node, next_node, person) ");
        query_builder.push_values(chunk, |mut b, edge| {
            b.push_bind(edge.0).push_bind(edge.1).push_bind(edge.2);
        });
        query_builder.push(" ON CONFLICT DO NOTHING");

        let query = query_builder.build();
        query.execute(pool).await?;
    }

    Ok(())
}
