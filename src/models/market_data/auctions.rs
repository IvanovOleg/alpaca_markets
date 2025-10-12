use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Daily auction data for a stock
#[derive(Debug, Deserialize, Serialize)]
pub struct Auction {
    /// Date of the auction
    #[serde(rename = "d")]
    pub date: String,
    /// Opening auction data
    #[serde(rename = "o")]
    pub opening: Option<AuctionData>,
    /// Closing auction data  
    #[serde(rename = "c")]
    pub closing: Option<AuctionData>,
}

/// Auction price and volume data
#[derive(Debug, Deserialize, Serialize)]
pub struct AuctionData {
    /// Auction price
    #[serde(rename = "p")]
    pub price: f64,
    /// Auction volume
    #[serde(rename = "v")]
    pub volume: u64,
    /// Timestamp of the auction
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
}

/// Response containing auction data for stocks
#[derive(Debug, Deserialize, Serialize)]
pub struct AuctionsResponse {
    /// Auction data keyed by symbol
    pub auctions: std::collections::HashMap<String, Vec<Auction>>,
    /// Next page token for pagination
    pub next_page_token: Option<String>,
}

/// Response containing auction data for a single stock
#[derive(Debug, Deserialize, Serialize)]
pub struct AuctionResponse {
    /// Auction data for the symbol
    pub auctions: Vec<Auction>,
    /// Symbol name
    pub symbol: String,
    /// Next page token for pagination  
    pub next_page_token: Option<String>,
}
