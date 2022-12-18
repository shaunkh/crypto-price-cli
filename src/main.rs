use dotenv::dotenv;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use reqwest::Client;
use std::collections::HashMap;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    // let mut headers = HeaderMap::new();
    // headers.insert(
    //     AUTHORIZATION,
    //     format!("Bearer {}", api_key).parse().unwrap(),
    // );

    let client = Client::new();

    let resp = client
        .get("https://httpbin.org/ip")
        .send()
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);

    Ok(())
}
