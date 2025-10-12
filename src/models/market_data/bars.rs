use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Bar {
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
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
    #[serde(rename = "n")]
    pub trade_count: Option<u64>,
    #[serde(rename = "vw")]
    pub vwap: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BarsResponse {
    pub bars: Vec<Bar>,
    pub symbol: String,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BarRequest {
    pub symbols: Vec<String>,
    pub timeframe: String,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub limit: Option<u32>,
    pub page_token: Option<String>,
}
