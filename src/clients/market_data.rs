use crate::api::market_data::MarketDataApi;
use crate::config::AlpacaConfig;
use crate::models::{
    AlpacaError, AlpacaResult, BarsResponse, QuotesResponse, TradesResponse,
    market_data::bars::{Adjustment, Sort},
};
use chrono::{DateTime, Utc};
use reqwest::Client;

/// High-level client for market data operations
#[derive(Debug)]
pub struct MarketDataClient {
    client: Client,
    config: AlpacaConfig,
}

impl MarketDataClient {
    /// Create a new market data client
    pub fn new(config: AlpacaConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    /// Get bars for a symbol
    pub async fn get_bars(
        &self,
        symbol: &str,
        timeframe: &str,
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
        limit: Option<u32>,
        sort: Option<Sort>,
        adjustment: Option<Adjustment>,
    ) -> AlpacaResult<BarsResponse> {
        self.get_bars_with_feed(symbol, timeframe, start, end, limit, None, sort, adjustment)
            .await
    }

    /// Get bars for a symbol with a specific feed
    pub async fn get_bars_with_feed(
        &self,
        symbol: &str,
        timeframe: &str,
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
        limit: Option<u32>,
        feed: Option<&str>,
        sort: Option<Sort>,
        adjustment: Option<Adjustment>,
    ) -> AlpacaResult<BarsResponse> {
        let start_str = start.map(|dt| dt.format("%Y-%m-%dT%H:%M:%SZ").to_string());
        let end_str = end.map(|dt| dt.format("%Y-%m-%dT%H:%M:%SZ").to_string());

        let feed_to_use = feed.or(self.config.default_feed.as_deref());

        let response = MarketDataApi::get_bars_single_raw(
            &self.client,
            &self.config.data_url,
            self.config.get_headers(),
            symbol,
            timeframe,
            start_str.as_deref(),
            end_str.as_deref(),
            limit,
            None, // page_token
            feed_to_use,
            sort,
            adjustment,
        )
        .await?;

        if response.status().is_success() {
            let bars: BarsResponse = response.json().await?;
            Ok(bars)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Get latest quote for a symbol
    pub async fn get_latest_quote(&self, symbol: &str) -> AlpacaResult<QuotesResponse> {
        self.get_latest_quote_with_feed(symbol, None).await
    }

    /// Get latest quote for a symbol with a specific feed
    pub async fn get_latest_quote_with_feed(
        &self,
        symbol: &str,
        feed: Option<&str>,
    ) -> AlpacaResult<QuotesResponse> {
        let feed_to_use = feed.or(self.config.default_feed.as_deref());

        let response = MarketDataApi::get_latest_quote_single_raw(
            &self.client,
            &self.config.data_url,
            self.config.get_headers(),
            symbol,
            feed_to_use,
        )
        .await?;

        if response.status().is_success() {
            let quote: QuotesResponse = response.json().await?;
            Ok(quote)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Get latest trade for a symbol
    pub async fn get_latest_trade(&self, symbol: &str) -> AlpacaResult<TradesResponse> {
        self.get_latest_trade_with_feed(symbol, None).await
    }

    /// Get latest trade for a symbol with a specific feed
    pub async fn get_latest_trade_with_feed(
        &self,
        symbol: &str,
        feed: Option<&str>,
    ) -> AlpacaResult<TradesResponse> {
        let feed_to_use = feed.or(self.config.default_feed.as_deref());

        let response = MarketDataApi::get_latest_trade_single_raw(
            &self.client,
            &self.config.data_url,
            self.config.get_headers(),
            symbol,
            feed_to_use,
        )
        .await?;

        if response.status().is_success() {
            let trade: TradesResponse = response.json().await?;
            Ok(trade)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }
}
