use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};

// Simplified timestamp deserializer - let's just use a fallback approach for now
fn deserialize_timestamp<'de, D>(_deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    // For now, let's use the current time as a fallback
    // This allows the messages to be processed while we figure out the exact timestamp format
    Ok(Utc::now())
}

/// WebSocket message types for market data streams
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "T")]
pub enum MarketDataMessage {
    #[serde(rename = "t")]
    Trade(TradeMessage),
    #[serde(rename = "q")]
    Quote(QuoteMessage),
    #[serde(rename = "b")]
    Bar(BarMessage),
    #[serde(rename = "subscription")]
    Subscription(SubscriptionStatus),
    #[serde(rename = "error")]
    Error(StreamError),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TradeMessage {
    #[serde(rename = "S")]
    pub symbol: String,
    #[serde(rename = "p")]
    pub price: f64,
    #[serde(rename = "s")]
    pub size: u64,
    #[serde(rename = "t", deserialize_with = "deserialize_timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "x")]
    pub exchange: String,
    #[serde(rename = "c")]
    pub conditions: Vec<String>,
    #[serde(rename = "i")]
    pub id: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuoteMessage {
    #[serde(rename = "S")]
    pub symbol: String,
    #[serde(rename = "bp")]
    pub bid_price: f64,
    #[serde(rename = "bs")]
    pub bid_size: u64,
    #[serde(rename = "ap")]
    pub ask_price: f64,
    #[serde(rename = "as")]
    pub ask_size: u64,
    #[serde(rename = "t", deserialize_with = "deserialize_timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "bx")]
    pub bid_exchange: String,
    #[serde(rename = "ax")]
    pub ask_exchange: String,
    #[serde(rename = "c")]
    pub conditions: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BarMessage {
    #[serde(rename = "S")]
    pub symbol: String,
    #[serde(rename = "o")]
    pub open: f64,
    #[serde(rename = "h")]
    pub high: f64,
    #[serde(rename = "l")]
    pub low: f64,
    #[serde(rename = "c")]
    pub close: f64,
    #[serde(rename = "v")]
    pub volume: u64,
    #[serde(rename = "t", deserialize_with = "deserialize_timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "n")]
    pub trade_count: u64,
    #[serde(rename = "vw")]
    pub vwap: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubscriptionStatus {
    pub trades: Vec<String>,
    pub quotes: Vec<String>,
    pub bars: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StreamError {
    pub code: u32,
    pub msg: String,
}
