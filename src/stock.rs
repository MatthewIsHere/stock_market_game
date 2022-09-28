use std::fmt::Display;
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use yahoo_finance_api::YahooConnector;

#[derive(Debug,Serialize,Deserialize)]
pub struct Stock {
    pub symbol: String
}
impl FromStr for Stock {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            symbol: s.to_string().to_uppercase()
        })
    }
    type Err = bool;
}
impl Display for Stock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol)
    }
}

impl Stock {
    pub fn current_price(&self, provider: &YahooConnector) -> f64 {
        match provider.get_latest_quotes(&self.symbol, "1h") {
            Ok(quotes) => {
                quotes.last_quote().unwrap().close
            },
            Err(error) => {
                panic!("Failed to fetch latest {} stock price: {}", self.symbol, error)
            }
        }
    }
}