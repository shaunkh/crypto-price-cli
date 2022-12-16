use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();
    let mut ticker: String = "ETH".to_string();

    if args.len() < 2 {
        println!("Since you didn't specify a crypto ticker, it is defaulted to ETH.");
    } else {
        ticker = args[1].clone();
    }

    let _coinmarketcap_api_key =
        env::var("COINMARKETCAP_API_KEY").expect("COINMARKETCAP_API_KEY must be set.");

    println!("{:?}", ticker);
}
