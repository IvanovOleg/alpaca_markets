use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Quote {
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "bp")]
    pub bid_price: f64,
    #[serde(rename = "bs")]
    pub bid_size: u64,
    #[serde(rename = "ap")]
    pub ask_price: f64,
    #[serde(rename = "as")]
    pub ask_size: u64,
    #[serde(rename = "bx")]
    pub bid_exchange: String,
    #[serde(rename = "ax")]
    pub ask_exchange: String,
    #[serde(rename = "c")]
    pub conditions: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuotesResponse {
    pub quote: Quote,
    pub symbol: String,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct QuoteRequest {
    pub symbols: Vec<String>,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub limit: Option<u32>,
    pub page_token: Option<String>,
}
