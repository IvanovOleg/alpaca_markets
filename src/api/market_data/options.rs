use crate::models::AlpacaResult;
use reqwest::{Client, Response};

impl super::MarketDataApi {
    // ===== OPTIONS MARKET DATA ENDPOINTS =====

    /// GET /v1/options/bars
    /// Retrieves historical bars for options contracts
    pub async fn get_option_bars_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        symbols: &str,
        timeframe: &str,
        start: Option<&str>,
        end: Option<&str>,
        limit: Option<u32>,
        page_token: Option<&str>,
        feed: Option<&str>,
        sort: Option<&str>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v1/options/bars", data_url);
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
        if let Some(feed) = feed {
            params.push(format!("feed={}", feed));
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

    /// GET /v1/options/meta/conditions
    /// Retrieves condition codes for options quotes and trades
    pub async fn get_option_meta_conditions_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        ticktype: &str,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v1/options/meta/conditions", data_url);
        let params = vec![format!("ticktype={}", ticktype)];

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// GET /v1/options/meta/exchanges
    /// Retrieves exchange codes for options quotes and trades
    pub async fn get_option_meta_exchanges_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
    ) -> AlpacaResult<Response> {
        let url = format!("{}/v1/options/meta/exchanges", data_url);
        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// GET /v1/options/quotes/latest
    /// Retrieves latest quotes for options contracts
    pub async fn get_option_latest_quotes_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        symbols: &str,
        feed: Option<&str>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v1/options/quotes/latest", data_url);
        let mut params = vec![format!("symbols={}", symbols)];

        if let Some(feed) = feed {
            params.push(format!("feed={}", feed));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// GET /v1/options/snapshots
    /// Retrieves market snapshots for options contracts
    pub async fn get_option_snapshots_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        symbols: &str,
        feed: Option<&str>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v1/options/snapshots", data_url);
        let mut params = vec![format!("symbols={}", symbols)];

        if let Some(feed) = feed {
            params.push(format!("feed={}", feed));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// GET /v1/options/snapshots/{underlying_symbol}
    /// Retrieves options chain (all options for an underlying symbol)
    pub async fn get_option_chain_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        underlying_symbol: &str,
        feed: Option<&str>,
        expiration_date: Option<&str>,
        expiration_date_gte: Option<&str>,
        expiration_date_lte: Option<&str>,
        root_symbol: Option<&str>,
        r#type: Option<&str>,
        strike_price_gte: Option<&str>,
        strike_price_lte: Option<&str>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v1/options/snapshots/{}", data_url, underlying_symbol);
        let mut params = Vec::new();

        if let Some(feed) = feed {
            params.push(format!("feed={}", feed));
        }
        if let Some(exp_date) = expiration_date {
            params.push(format!("expiration_date={}", exp_date));
        }
        if let Some(exp_date_gte) = expiration_date_gte {
            params.push(format!("expiration_date_gte={}", exp_date_gte));
        }
        if let Some(exp_date_lte) = expiration_date_lte {
            params.push(format!("expiration_date_lte={}", exp_date_lte));
        }
        if let Some(root) = root_symbol {
            params.push(format!("root_symbol={}", root));
        }
        if let Some(option_type) = r#type {
            params.push(format!("type={}", option_type));
        }
        if let Some(strike_gte) = strike_price_gte {
            params.push(format!("strike_price_gte={}", strike_gte));
        }
        if let Some(strike_lte) = strike_price_lte {
            params.push(format!("strike_price_lte={}", strike_lte));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// GET /v1/options/trades
    /// Retrieves historical trades for options contracts
    pub async fn get_option_trades_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        symbols: &str,
        start: Option<&str>,
        end: Option<&str>,
        limit: Option<u32>,
        page_token: Option<&str>,
        feed: Option<&str>,
        sort: Option<&str>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v1/options/trades", data_url);
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
        if let Some(feed) = feed {
            params.push(format!("feed={}", feed));
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

    /// GET /v1/options/trades/latest
    /// Retrieves latest trades for options contracts
    pub async fn get_option_latest_trades_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        symbols: &str,
        feed: Option<&str>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v1/options/trades/latest", data_url);
        let mut params = vec![format!("symbols={}", symbols)];

        if let Some(feed) = feed {
            params.push(format!("feed={}", feed));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }
}
