use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MovieDetailsResponse {
    pub id: u64,
    pub title: String,
    pub release_date: String,
}

pub async fn get_movie(access_token: &str, movie_id: u64) -> Result<MovieDetailsResponse> {
    let client = reqwest::Client::new();
    let res = client
        .get(format!("https://api.themoviedb.org/3/movie/{movie_id}"))
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;
    dbg!(res.status());

    let res: MovieDetailsResponse = res.json().await?;
    Ok(res)
}
