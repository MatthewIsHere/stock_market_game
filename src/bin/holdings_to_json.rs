use std::{env, ops::Deref, str::FromStr};
use std::fs::File;
use std::io::Write;
use stock_market_game::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 { return println!("Usage: holdings_to_json.rs [Source].csv [Dest].json")}
    let source = args.get(1).unwrap();
    let dest = args.get(2).unwrap();
    let mut holdings: Vec<Holding> = Vec::new();
    let mut reader = csv::Reader::from_path(source.deref()).expect("Failed to parse CSV!");
    for row in reader.records() {
        let values = row.expect("Failed to parse row!");
        let stock = Stock::from_str(&values[0]).unwrap();
        let position = match &values[1] {
            "Long" => Position::LONG,
            "Short" => Position::SHORT,
            _ => panic!("CSV position field invalid!")
        };
        let shares = values[2].parse::<i64>().expect("Failed to parse number of shares!");
        let share_price = values[4][1..].parse::<f64>().expect("Failed to parse share price!");
        holdings.push(Holding { stock, position, shares, share_price });
    }
    let mut destfile = File::create(dest.deref()).expect("Failed to open dest file!");
    let json = serde_json::to_string_pretty(&holdings).expect("Failed to serialize holdings!"); // I like pretty
    destfile.write_all(json.as_bytes()).expect("Failed to write to dest file!");
}