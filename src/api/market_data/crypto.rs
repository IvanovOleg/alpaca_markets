use crate::models::AlpacaResult;
use reqwest::{Client, Response};

impl super::MarketDataApi {
    // ===== CRYPTO MARKET DATA ENDPOINTS =====

    /// GET /v1/crypto/bars
    /// Retrieves historical bars for crypto pairs
    pub async fn get_crypto_bars_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        symbols: &str,
        timeframe: &str,
        start: Option<&str>,
        end: Option<&str>,
        limit: Option<u32>,
        page_token: Option<&str>,
        sort: Option<&str>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v1/crypto/bars", data_url);
        let mut params = vec![
            format!("symbols={}", symbols),
            format!("timeframe={}", timeframe),
        ];

        if let Some(start) = start {
            params.push(format!("start={}", start));
        }
        if let Some(end) = end {
            params.push(format!("end={}", end));
        }
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(page_token) = page_token {
            params.push(format!("page_token={}", page_token));
        }
        if let Some(sort) = sort {
            params.push(format!("sort={}", sort));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// GET /v1/crypto/bars/latest
    /// Retrieves latest bars for crypto pairs
    pub async fn get_crypto_latest_bars_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        symbols: &str,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v1/crypto/bars/latest", data_url);
        let params = vec![format!("symbols={}", symbols)];

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// GET /v1/crypto/orderbooks/latest
    /// Retrieves latest order books for crypto pairs
    pub async fn get_crypto_latest_orderbooks_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        symbols: &str,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v1/crypto/orderbooks/latest", data_url);
        let params = vec![format!("symbols={}", symbols)];

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// GET /v1/crypto/quotes/latest
    /// Retrieves latest quotes for crypto pairs
    pub async fn get_crypto_latest_quotes_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        symbols: &str,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v1/crypto/quotes/latest", data_url);
        let params = vec![format!("symbols={}", symbols)];

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// GET /v1/crypto/trades/latest
    /// Retrieves latest trades for crypto pairs
    pub async fn get_crypto_latest_trades_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        symbols: &str,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v1/crypto/trades/latest", data_url);
        let params = vec![format!("symbols={}", symbols)];

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// GET /v1/crypto/quotes
    /// Retrieves historical quotes for crypto pairs
    pub async fn get_crypto_quotes_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        symbols: &str,
        start: Option<&str>,
        end: Option<&str>,
        limit: Option<u32>,
        page_token: Option<&str>,
        sort: Option<&str>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v1/crypto/quotes", data_url);
        let mut params = vec![format!("symbols={}", symbols)];

        if let Some(start) = start {
            params.push(format!("start={}", start));
        }
        if let Some(end) = end {
            params.push(format!("end={}", end));
        }
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(page_token) = page_token {
            params.push(format!("page_token={}", page_token));
        }
        if let Some(sort) = sort {
            params.push(format!("sort={}", sort));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// GET /v1/crypto/snapshots
    /// Retrieves market snapshots for crypto pairs
    pub async fn get_crypto_snapshots_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        symbols: &str,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v1/crypto/snapshots", data_url);
        let params = vec![format!("symbols={}", symbols)];

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// GET /v1/crypto/trades
    /// Retrieves historical trades for crypto pairs
    pub async fn get_crypto_trades_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        symbols: &str,
        start: Option<&str>,
        end: Option<&str>,
        limit: Option<u32>,
        page_token: Option<&str>,
        sort: Option<&str>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v1/crypto/trades", data_url);
        let mut params = vec![format!("symbols={}", symbols)];

        if let Some(start) = start {
            params.push(format!("start={}", start));
        }
        if let Some(end) = end {
            params.push(format!("end={}", end));
        }
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(page_token) = page_token {
            params.push(format!("page_token={}", page_token));
        }
        if let Some(sort) = sort {
            params.push(format!("sort={}", sort));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }
}
