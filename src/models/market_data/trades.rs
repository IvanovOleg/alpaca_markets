use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Trade {
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "p")]
    pub price: f64,
    #[serde(rename = "s")]
    pub size: u64,
    #[serde(rename = "x")]
    pub exchange: String,
    #[serde(rename = "c")]
    pub conditions: Vec<String>,
    #[serde(rename = "i")]
    pub id: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TradesResponse {
    pub trade: Trade,
    pub symbol: String,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TradeRequest {
    pub symbols: Vec<String>,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub limit: Option<u32>,
    pub page_token: Option<String>,
}
