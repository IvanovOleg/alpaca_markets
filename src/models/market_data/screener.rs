use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Most Active Stock Entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MostActiveStock {
    /// Symbol of the stock
    pub symbol: String,
    /// Volume of the stock
    pub volume: u64,
    /// Number of trades
    pub trade_count: Option<u64>,
}

/// Response for most active stocks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MostActivesResponse {
    /// List of most active stocks
    pub most_actives: Vec<MostActiveStock>,
    /// Last updated timestamp
    pub last_updated: Option<DateTime<Utc>>,
}

/// Market Mover Entry (Gainer or Loser)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketMover {
    /// Symbol of the stock/crypto
    pub symbol: String,
    /// Current price
    pub price: f64,
    /// Price change
    pub change: f64,
    /// Percentage change
    pub percent_change: f64,
}

/// Response for market movers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoversResponse {
    /// Top gainers
    pub gainers: Option<Vec<MarketMover>>,
    /// Top losers
    pub losers: Option<Vec<MarketMover>>,
    /// Last updated timestamp
    pub last_updated: Option<DateTime<Utc>>,
}
