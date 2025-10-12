use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Portfolio {
    pub timestamp: DateTime<Utc>,
    pub equity: String,
    pub profit_loss: String,
    pub profit_loss_pct: String,
    pub base_value: String,
    pub timeframe: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PortfolioHistory {
    pub timestamp: Vec<i64>,
    pub equity: Vec<f64>,
    pub profit_loss: Vec<f64>,
    pub profit_loss_pct: Vec<f64>,
    pub base_value: f64,
    pub timeframe: String,
}
