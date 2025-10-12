use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Options bar data
#[derive(Debug, Deserialize, Serialize)]
pub struct OptionBar {
    /// Timestamp of the bar
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
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
    pub trade_count: Option<u64>,
    /// Volume weighted average price
    #[serde(rename = "vw")]
    pub vwap: Option<f64>,
}

/// Options quote data
#[derive(Debug, Deserialize, Serialize)]
pub struct OptionQuote {
    /// Timestamp of the quote
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    /// Bid price
    #[serde(rename = "bp")]
    pub bid_price: f64,
    /// Bid size
    #[serde(rename = "bs")]
    pub bid_size: u32,
    /// Ask price
    #[serde(rename = "ap")]
    pub ask_price: f64,
    /// Ask size
    #[serde(rename = "as")]
    pub ask_size: u32,
    /// Bid exchange
    #[serde(rename = "bx")]
    pub bid_exchange: String,
    /// Ask exchange
    #[serde(rename = "ax")]
    pub ask_exchange: String,
    /// Quote conditions
    #[serde(rename = "c")]
    pub conditions: Vec<String>,
}

/// Options trade data
#[derive(Debug, Deserialize, Serialize)]
pub struct OptionTrade {
    /// Timestamp of the trade
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    /// Trade price
    #[serde(rename = "p")]
    pub price: f64,
    /// Trade size
    #[serde(rename = "s")]
    pub size: u32,
    /// Exchange where trade occurred
    #[serde(rename = "x")]
    pub exchange: String,
    /// Trade conditions
    #[serde(rename = "c")]
    pub conditions: Vec<String>,
}

/// Options snapshot data
#[derive(Debug, Deserialize, Serialize)]
pub struct OptionSnapshot {
    /// Latest trade information
    #[serde(rename = "latestTrade")]
    pub latest_trade: Option<OptionTrade>,
    /// Latest quote information
    #[serde(rename = "latestQuote")]
    pub latest_quote: Option<OptionQuote>,
    /// Implied volatility
    #[serde(rename = "impliedVolatility")]
    pub implied_volatility: Option<f64>,
    /// Greeks (delta, gamma, theta, vega, rho)
    #[serde(rename = "greeks")]
    pub greeks: Option<OptionGreeks>,
}

/// Options Greeks data
#[derive(Debug, Deserialize, Serialize)]
pub struct OptionGreeks {
    /// Delta - price sensitivity to underlying price changes
    pub delta: Option<f64>,
    /// Gamma - rate of change of delta
    pub gamma: Option<f64>,
    /// Theta - time decay
    pub theta: Option<f64>,
    /// Vega - sensitivity to implied volatility
    pub vega: Option<f64>,
    /// Rho - sensitivity to interest rate changes
    pub rho: Option<f64>,
}

/// Response containing option bars
#[derive(Debug, Deserialize, Serialize)]
pub struct OptionBarsResponse {
    /// Option bars keyed by symbol
    pub bars: HashMap<String, Vec<OptionBar>>,
    /// Next page token for pagination
    pub next_page_token: Option<String>,
}

/// Response containing option quotes
#[derive(Debug, Deserialize, Serialize)]
pub struct OptionQuotesResponse {
    /// Option quotes keyed by symbol
    pub quotes: HashMap<String, OptionQuote>,
}

/// Response containing option trades
#[derive(Debug, Deserialize, Serialize)]
pub struct OptionTradesResponse {
    /// Option trades keyed by symbol
    pub trades: HashMap<String, Vec<OptionTrade>>,
    /// Next page token for pagination
    pub next_page_token: Option<String>,
}

/// Response containing latest option trades
#[derive(Debug, Deserialize, Serialize)]
pub struct OptionLatestTradesResponse {
    /// Latest option trades keyed by symbol
    pub trades: HashMap<String, OptionTrade>,
}

/// Response containing option snapshots
#[derive(Debug, Deserialize, Serialize)]
pub struct OptionSnapshotsResponse {
    /// Option snapshots keyed by symbol
    pub snapshots: HashMap<String, OptionSnapshot>,
}

/// Response containing options chain for an underlying symbol
#[derive(Debug, Deserialize, Serialize)]
pub struct OptionChainResponse {
    /// Options snapshots for the underlying symbol
    pub snapshots: HashMap<String, OptionSnapshot>,
    /// Next page token for pagination
    pub next_page_token: Option<String>,
}
