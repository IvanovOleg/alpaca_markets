use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Crypto bar data (OHLCV)
#[derive(Debug, Deserialize, Serialize)]
pub struct CryptoBar {
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
    pub volume: f64,
    /// Trade count
    #[serde(rename = "n")]
    pub trade_count: Option<u64>,
    /// Volume weighted average price
    #[serde(rename = "vw")]
    pub vwap: Option<f64>,
}

/// Crypto quote data
#[derive(Debug, Deserialize, Serialize)]
pub struct CryptoQuote {
    /// Timestamp of the quote
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    /// Bid price
    #[serde(rename = "bp")]
    pub bid_price: f64,
    /// Bid size
    #[serde(rename = "bs")]
    pub bid_size: f64,
    /// Ask price
    #[serde(rename = "ap")]
    pub ask_price: f64,
    /// Ask size
    #[serde(rename = "as")]
    pub ask_size: f64,
    /// Exchange identifier
    #[serde(rename = "x")]
    pub exchange: String,
}

/// Crypto trade data
#[derive(Debug, Deserialize, Serialize)]
pub struct CryptoTrade {
    /// Timestamp of the trade
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    /// Trade price
    #[serde(rename = "p")]
    pub price: f64,
    /// Trade size
    #[serde(rename = "s")]
    pub size: f64,
    /// Exchange identifier
    #[serde(rename = "x")]
    pub exchange: String,
    /// Taker side (buy/sell)
    #[serde(rename = "tks")]
    pub taker_side: Option<String>,
    /// Trade ID
    #[serde(rename = "i")]
    pub trade_id: Option<String>,
}

/// Order book entry (bid or ask level)
#[derive(Debug, Deserialize, Serialize)]
pub struct OrderBookEntry {
    /// Price level
    #[serde(rename = "p")]
    pub price: f64,
    /// Size at this price level
    #[serde(rename = "s")]
    pub size: f64,
}

/// Crypto order book data
#[derive(Debug, Deserialize, Serialize)]
pub struct CryptoOrderBook {
    /// Timestamp of the order book snapshot
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    /// Bid levels (sorted by price descending)
    #[serde(rename = "b")]
    pub bids: Vec<OrderBookEntry>,
    /// Ask levels (sorted by price ascending)
    #[serde(rename = "a")]
    pub asks: Vec<OrderBookEntry>,
    /// Exchange identifier
    #[serde(rename = "x")]
    pub exchange: String,
}

/// Crypto market snapshot
#[derive(Debug, Deserialize, Serialize)]
pub struct CryptoSnapshot {
    /// Latest trade information
    #[serde(rename = "latestTrade")]
    pub latest_trade: Option<CryptoTrade>,
    /// Latest quote information
    #[serde(rename = "latestQuote")]
    pub latest_quote: Option<CryptoQuote>,
    /// Minute bar information
    #[serde(rename = "minuteBar")]
    pub minute_bar: Option<CryptoBar>,
    /// Daily bar information
    #[serde(rename = "dailyBar")]
    pub daily_bar: Option<CryptoBar>,
    /// Previous daily bar information
    #[serde(rename = "prevDailyBar")]
    pub prev_daily_bar: Option<CryptoBar>,
    /// 24h change percentage
    #[serde(rename = "change")]
    pub change_percent: Option<f64>,
}

/// Response containing crypto bars
#[derive(Debug, Deserialize, Serialize)]
pub struct CryptoBarsResponse {
    /// Crypto bars keyed by symbol
    pub bars: HashMap<String, Vec<CryptoBar>>,
    /// Next page token for pagination
    pub next_page_token: Option<String>,
}

/// Response containing latest crypto bars
#[derive(Debug, Deserialize, Serialize)]
pub struct CryptoLatestBarsResponse {
    /// Latest crypto bars keyed by symbol
    pub bars: HashMap<String, CryptoBar>,
}

/// Response containing crypto quotes
#[derive(Debug, Deserialize, Serialize)]
pub struct CryptoQuotesResponse {
    /// Crypto quotes keyed by symbol
    pub quotes: HashMap<String, Vec<CryptoQuote>>,
    /// Next page token for pagination
    pub next_page_token: Option<String>,
}

/// Response containing latest crypto quotes
#[derive(Debug, Deserialize, Serialize)]
pub struct CryptoLatestQuotesResponse {
    /// Latest crypto quotes keyed by symbol
    pub quotes: HashMap<String, CryptoQuote>,
}

/// Response containing crypto trades
#[derive(Debug, Deserialize, Serialize)]
pub struct CryptoTradesResponse {
    /// Crypto trades keyed by symbol
    pub trades: HashMap<String, Vec<CryptoTrade>>,
    /// Next page token for pagination
    pub next_page_token: Option<String>,
}

/// Response containing latest crypto trades
#[derive(Debug, Deserialize, Serialize)]
pub struct CryptoLatestTradesResponse {
    /// Latest crypto trades keyed by symbol
    pub trades: HashMap<String, CryptoTrade>,
}

/// Response containing crypto order books
#[derive(Debug, Deserialize, Serialize)]
pub struct CryptoOrderBooksResponse {
    /// Latest crypto order books keyed by symbol
    pub orderbooks: HashMap<String, CryptoOrderBook>,
}

/// Response containing crypto snapshots
#[derive(Debug, Deserialize, Serialize)]
pub struct CryptoSnapshotsResponse {
    /// Crypto snapshots keyed by symbol
    pub snapshots: HashMap<String, CryptoSnapshot>,
}
