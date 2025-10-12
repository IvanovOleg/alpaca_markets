use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Forex exchange rate data
#[derive(Debug, Deserialize, Serialize)]
pub struct ForexRate {
    /// Timestamp of the rate
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    /// Exchange rate (base currency to quote currency)
    #[serde(rename = "r")]
    pub rate: f64,
    /// Open rate (for historical rates)
    #[serde(rename = "o")]
    pub open: Option<f64>,
    /// High rate (for historical rates)
    #[serde(rename = "h")]
    pub high: Option<f64>,
    /// Low rate (for historical rates)
    #[serde(rename = "l")]
    pub low: Option<f64>,
    /// Close rate (for historical rates)
    #[serde(rename = "c")]
    pub close: Option<f64>,
    /// Volume (for historical rates)
    #[serde(rename = "v")]
    pub volume: Option<f64>,
}

/// Response containing latest forex rates
#[derive(Debug, Deserialize, Serialize)]
pub struct ForexLatestRatesResponse {
    /// Latest forex rates keyed by currency pair (e.g., "EUR/USD")
    pub rates: HashMap<String, ForexRate>,
}

/// Response containing historical forex rates
#[derive(Debug, Deserialize, Serialize)]
pub struct ForexRatesResponse {
    /// Historical forex rates keyed by currency pair
    pub rates: HashMap<String, Vec<ForexRate>>,
    /// Next page token for pagination
    pub next_page_token: Option<String>,
}
