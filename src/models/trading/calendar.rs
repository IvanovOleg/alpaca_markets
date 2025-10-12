use serde::{Deserialize, Serialize};

/// Market calendar day information
#[derive(Debug, Deserialize, Serialize)]
pub struct MarketCalendar {
    /// The date of the market day (YYYY-MM-DD format)
    pub date: String,
    /// Market open time (ISO 8601 format)
    pub open: String,
    /// Market close time (ISO 8601 format)
    pub close: String,
    /// Settlement date (YYYY-MM-DD format)
    pub settlement_date: String,
}
