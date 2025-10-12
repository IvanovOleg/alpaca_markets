//! Alpaca Markets API Client Library
//!
//! This library provides access to Alpaca Markets APIs with optional features
//! to reduce compilation overhead.

pub mod config;
pub mod utils;

// Always include common models and errors
pub mod models;
pub use models::*;

// API modules - conditionally compiled
pub mod api;

#[cfg(feature = "websocket")]
pub mod wss;

#[cfg(feature = "websocket")]
pub use wss::common::SerializationFormat;

// Conditionally compile clients
#[cfg(any(feature = "trading", feature = "market_data", feature = "websocket"))]
pub mod clients;

pub use config::AlpacaConfig;

#[cfg(feature = "trading")]
pub use clients::trading::TradingClient;

#[cfg(feature = "market_data")]
pub use clients::market_data::MarketDataClient;

// WebSocket client functionality is now provided through specific stream clients

#[cfg(feature = "websocket")]
pub use clients::trading_stream::TradingStreamClient;

#[cfg(feature = "websocket")]
pub use clients::market_data_stream::{Feed, MarketDataStreamClient};

// Re-export key types for easier access
#[cfg(feature = "websocket")]
pub use wss::trading::{AccountUpdate, TradeEventType, TradeUpdate, TradingWebSocketMessage};
