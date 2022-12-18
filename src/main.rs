use reqwest::Client;
use std::collections::HashMap;

#[tokio::main]

// JSON needs deserializer
// Can derive but essentially make a struct for it and add the deserialize and debug macro

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let resp = client
        .get("https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&order=market_cap_desc&per_page=100&page=1&sparkline=false")
        .send()
        .await?;
    println!("{:#?}", resp);

    let resp_json = resp.json().await?;

    Ok(())
}
