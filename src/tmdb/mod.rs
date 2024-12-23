use anyhow::{bail, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DiscoverMoviesResponse {
    total_pages: i64,
    pub results: Vec<DiscoverMoviesResult>,
}

#[derive(Deserialize, Debug, Clone)]
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

    let mut res: DiscoverMoviesResponse = res.json().await?;

    for page in 2..res.total_pages {
        let res_with_page = discover_movies_by_cast_with_page(access_token, cast, page).await?;
        res.results.extend(res_with_page.results);
    }

    Ok(res)
}

pub async fn discover_movies_by_cast_with_page(
    access_token: &str,
    cast: i64,
    page: i64,
) -> Result<DiscoverMoviesResponse> {
    let client = reqwest::Client::new();
    let res = client
        .get(format!(
            "https://api.themoviedb.org/3/discover/movie?with_cast={cast}&page={page}"
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
