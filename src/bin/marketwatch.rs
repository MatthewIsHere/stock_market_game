use std::env;
use std::fs::read_to_string;
use stock_market_game::Holding;
use yahoo_finance_api::YahooConnector as yf;

fn main() {
    let provider = yf::new();
    let mut total_cost: f64 = 0.0;
    let mut total_equity: f64 = 0.0;
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { return println!("Usage: ./marketwatch [Holdings].json"); }
    let json_path = args.get(1).unwrap();
    let json = read_to_string(json_path).expect("Failed to read JSON file");
    let holdings: Vec<Holding> = serde_json::from_str(&json).expect("Failed to deserialize JSON");
    for holding in holdings.iter() {
        total_cost += holding.total_cost();
        total_equity += holding.current_value(&provider);
    }
    println!("Total Cost: {}", total_cost);
    println!("Total Equity: {}", total_equity);
    println!("Net Gain: {}", total_equity - total_cost);
}