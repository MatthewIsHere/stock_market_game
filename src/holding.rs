use crate::*;
use serde::{Serialize,Deserialize};
use yahoo_finance_api::YahooConnector;

#[derive(Debug,PartialEq,Serialize,Deserialize)]
pub enum Position {
    LONG,
    SHORT
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Holding {
    pub stock: Stock,
    pub position: Position,
    pub shares: i64,
    pub share_price: f64,
}

impl Holding {
    pub fn total_cost(&self) -> f64 {
        self.share_price * self.shares as f64
    }
    pub fn current_value(&self, provider: &YahooConnector) -> f64 {
        self.stock.current_price(&provider) * self.shares as f64
    }
    pub fn unrealized_gains(&self, provider: &YahooConnector) -> f64 {
        match self.position {
            Position::LONG => self.current_value(provider) - self.total_cost(),
            Position::SHORT => (self.total_cost() + self.current_value(provider)) * -1.0
        }
    }
}