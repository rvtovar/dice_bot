use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct UnsplashResponse {
    urls: Urls,
}

#[derive(Deserialize)]
struct Urls {
    regular: String,
}

pub async fn get_frog_photo(api_key: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let res = client
        .get("https://api.unsplash.com/photos/random")
        .query(&[("query", "frog"), ("client_id", api_key)])
        .send()
        .await?
        .json::<UnsplashResponse>()
        .await?;
    Ok(res.urls.regular)
}