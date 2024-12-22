use anyhow::{bail, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MovieDetailsResponse {
    pub id: u64,
    pub title: String,
}

pub async fn get_movie(access_token: &str, movie_id: u64) -> Result<MovieDetailsResponse> {
    let client = reqwest::Client::new();
    let res = client
        .get(format!("https://api.themoviedb.org/3/movie/{movie_id}"))
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;

    if res.status() != 200 {
        bail!("request failed {:?}", res.status());
    }

    let res: MovieDetailsResponse = res.json().await?;
    Ok(res)
}

#[derive(Deserialize, Debug)]
pub struct DiscoverMoviesResponse {
    pub results: Vec<DiscoverMoviesResult>,
}

#[derive(Deserialize, Debug)]
pub struct DiscoverMoviesResult {
    pub id: u64,
}

pub async fn discover_movies(access_token: &str) -> Result<DiscoverMoviesResponse> {
    let client = reqwest::Client::new();
    let res = client
        .get(format!("https://api.themoviedb.org/3/discover/movie"))
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;

    if res.status() != 200 {
        bail!("request failed {:?}", res.status());
    }

    let res: DiscoverMoviesResponse = res.json().await?;
    Ok(res)
}
