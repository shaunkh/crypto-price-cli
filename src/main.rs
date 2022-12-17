use dotenv::dotenv;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use std::collections::HashMap;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_key = env::var("BREWS_API_KEY");
    let api_token = env::var("BREWS_API_TOKEN");

    let headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, "");

    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
