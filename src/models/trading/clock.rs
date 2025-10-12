use serde::{Deserialize, Serialize};

/// Market clock information
#[derive(Debug, Deserialize, Serialize)]
pub struct MarketClock {
    /// Current timestamp (ISO 8601 format)
    pub timestamp: String,
    /// Whether the market is currently open
    pub is_open: bool,
    /// Next time the market will open (ISO 8601 format)
    pub next_open: String,
    /// Next time the market will close (ISO 8601 format)
    pub next_close: String,
}
