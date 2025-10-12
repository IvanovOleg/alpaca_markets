use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Fixed income price data
#[derive(Debug, Deserialize, Serialize)]
pub struct FixedIncomePrice {
    /// Timestamp of the price
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    /// Price of the fixed income security
    #[serde(rename = "p")]
    pub price: f64,
    /// Yield of the fixed income security
    #[serde(rename = "y")]
    pub yield_value: Option<f64>,
    /// Accrued interest
    #[serde(rename = "ai")]
    pub accrued_interest: Option<f64>,
    /// Modified duration
    #[serde(rename = "md")]
    pub modified_duration: Option<f64>,
    /// Convexity
    #[serde(rename = "cv")]
    pub convexity: Option<f64>,
    /// Credit spread
    #[serde(rename = "cs")]
    pub credit_spread: Option<f64>,
    /// Option adjusted spread
    #[serde(rename = "oas")]
    pub option_adjusted_spread: Option<f64>,
    /// Z-spread
    #[serde(rename = "zs")]
    pub z_spread: Option<f64>,
    /// Exchange or venue identifier
    #[serde(rename = "x")]
    pub exchange: String,
}

/// Response containing latest fixed income prices
#[derive(Debug, Deserialize, Serialize)]
pub struct FixedIncomeLatestPricesResponse {
    /// Latest fixed income prices keyed by symbol
    pub prices: HashMap<String, FixedIncomePrice>,
}
