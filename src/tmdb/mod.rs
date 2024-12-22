use anyhow::{bail, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MovieDetailsResponse {
    pub id: i64,
    pub title: String,
}

pub async fn get_movie(access_token: &str, movie_id: i64) -> Result<MovieDetailsResponse> {
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
    pub id: i64,
    pub title: String,
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

pub async fn discover_movies_by_cast(
    access_token: &str,
    cast: i64,
) -> Result<DiscoverMoviesResponse> {
    let client = reqwest::Client::new();

    let res = client
        .get(format!(
            "https://api.themoviedb.org/3/discover/movie?with_cast={cast}"
        ))
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;

    if res.status() != 200 {
        bail!("request failed {:?}", res.status());
    }

    let res: DiscoverMoviesResponse = res.json().await?;
    Ok(res)
}

#[derive(Deserialize, Debug)]
pub struct MovieCreditsResponse {
    pub cast: Vec<MovieCastMember>,
}

#[derive(Deserialize, Debug)]
pub struct MovieCastMember {
    pub id: i64,
    pub name: String,
}

pub async fn get_credits(access_token: &str, movie_id: i64) -> Result<MovieCreditsResponse> {
    let client = reqwest::Client::new();
    let res = client
        .get(format!(
            "https://api.themoviedb.org/3/movie/{movie_id}/credits"
        ))
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;

    if res.status() != 200 {
        bail!("request failed {:?}", res.status());
    }

    let res: MovieCreditsResponse = res.json().await?;
    Ok(res)
}
