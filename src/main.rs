use dotenv::dotenv;
use exitfailure::ExitFailure;
use reqwest::Url;
use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};

use serde_derive::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Deb   ug)]
struct CoinGeckoCoinMarketsResponse {
    data: Vec<CoinGeckoCoinFromMarkets>,
}

impl Serialize for CoinGeckoCoinMarketsResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("CoinGeckoCoinMarketsResponse", 1)?;
        state.serialize_field("data", &self.data)?;
        state.end()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CoinGeckoCoinFromMarkets {
    id: String,
    symbol: String,
    name: String,
    image: String,
    current_price: f32,
    market_cap: f32,
    market_cap_rank: i32,
    fully_diluted_valuation: f32,
    total_volume: f32,
    high_24h: f32,
    low_24h: f32,
    price_change_24h: f32,
    price_change_percentage_24h: f32,
    market_cap_change_24h: f32,
    market_cap_change_percentage_24h: f32,
    circulating_supply: f32,
    total_supply: f32,
    max_supply: f32,
    ath: f32,
    ath_change_percentage: f32,
    ath_date: String,
    atl: f32,
    atl_change_percentage: f32,
    atl_date: String,
    roi: f32,
    last_updated: String,
}

impl CoinGeckoCoinMarketsResponse {
    async fn get() -> Result<Self, ExitFailure> {
        let url = format!(
            "https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&order=market_cap_desc&per_page=100&page=1&sparkline=false",
        );

        let url = Url::parse(&*url)?;

        // let res = reqwest::get(url)
        //     .await?
        //     .json::<CoinGeckoCoinMarketsResponse>()
        //     .await?;

        println!("{:?}", reqwest::get(url).await?);

        Ok(CoinGeckoCoinMarketsResponse { data: vec![] })
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();
    let mut ticker: String = "ETH".to_string();

    if args.len() < 2 {
        println!("Since you didn't specify a crypto ticker, it is defaulted to ETH.");
    } else {
        ticker = args[1].clone();
    }

    let res = CoinGeckoCoinMarketsResponse::get().await?;
    println!("{:?}", res.data);

    Ok(())
}
