use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Market snapshot for a stock
#[derive(Debug, Deserialize, Serialize)]
pub struct Snapshot {
    /// Latest trade information
    #[serde(rename = "latestTrade")]
    pub latest_trade: Option<LatestTrade>,
    /// Latest quote information
    #[serde(rename = "latestQuote")]
    pub latest_quote: Option<LatestQuote>,
    /// Minute bar information
    #[serde(rename = "minuteBar")]
    pub minute_bar: Option<MinuteBar>,
    /// Daily bar information
    #[serde(rename = "dailyBar")]
    pub daily_bar: Option<DailyBar>,
    /// Previous daily bar information
    #[serde(rename = "prevDailyBar")]
    pub prev_daily_bar: Option<DailyBar>,
    /// Market status
    #[serde(rename = "marketStatus")]
    pub market_status: String,
}

/// Latest trade data in snapshot
#[derive(Debug, Deserialize, Serialize)]
pub struct LatestTrade {
    /// Trade price
    #[serde(rename = "p")]
    pub price: f64,
    /// Trade size
    #[serde(rename = "s")]
    pub size: u64,
    /// Trade timestamp
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    /// Trade conditions
    #[serde(rename = "c")]
    pub conditions: Vec<String>,
    /// Exchange where trade occurred
    #[serde(rename = "x")]
    pub exchange: String,
}

/// Latest quote data in snapshot
#[derive(Debug, Deserialize, Serialize)]
pub struct LatestQuote {
    /// Bid price
    #[serde(rename = "bp")]
    pub bid_price: f64,
    /// Bid size
    #[serde(rename = "bs")]
    pub bid_size: u64,
    /// Ask price
    #[serde(rename = "ap")]
    pub ask_price: f64,
    /// Ask size
    #[serde(rename = "as")]
    pub ask_size: u64,
    /// Quote timestamp
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    /// Bid exchange
    #[serde(rename = "bx")]
    pub bid_exchange: String,
    /// Ask exchange
    #[serde(rename = "ax")]
    pub ask_exchange: String,
}

/// Minute bar data in snapshot
#[derive(Debug, Deserialize, Serialize)]
pub struct MinuteBar {
    /// Open price
    #[serde(rename = "o")]
    pub open: f64,
    /// High price
    #[serde(rename = "h")]
    pub high: f64,
    /// Low price
    #[serde(rename = "l")]
    pub low: f64,
    /// Close price
    #[serde(rename = "c")]
    pub close: f64,
    /// Volume
    #[serde(rename = "v")]
    pub volume: u64,
    /// Trade count
    #[serde(rename = "n")]
    pub trade_count: u64,
    /// Volume weighted average price
    #[serde(rename = "vw")]
    pub vwap: f64,
    /// Timestamp
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
}

/// Daily bar data in snapshot
#[derive(Debug, Deserialize, Serialize)]
pub struct DailyBar {
    /// Open price
    #[serde(rename = "o")]
    pub open: f64,
    /// High price
    #[serde(rename = "h")]
    pub high: f64,
    /// Low price
    #[serde(rename = "l")]
    pub low: f64,
    /// Close price
    #[serde(rename = "c")]
    pub close: f64,
    /// Volume
    #[serde(rename = "v")]
    pub volume: u64,
    /// Trade count
    #[serde(rename = "n")]
    pub trade_count: u64,
    /// Volume weighted average price
    #[serde(rename = "vw")]
    pub vwap: f64,
}

/// Response containing snapshots for multiple stocks
#[derive(Debug, Deserialize, Serialize)]
pub struct SnapshotsResponse {
    /// Snapshots keyed by symbol
    pub snapshots: std::collections::HashMap<String, Snapshot>,
}

/// Response containing snapshot for a single stock
#[derive(Debug, Deserialize, Serialize)]
pub struct SnapshotResponse {
    /// Snapshot data for the symbol
    pub snapshot: Snapshot,
}
