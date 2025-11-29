use crate::models::{
    AlpacaResult,
    market_data::bars::{Adjustment, Sort},
};
use reqwest::{Client, Response};

impl super::MarketDataApi {
    // ===== MULTI-SYMBOL ENDPOINTS =====

    /// GET /v2/stocks/auctions
    /// Retrieves daily auction data for multiple stocks
    pub async fn get_auctions_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        symbols: &str,
        start: Option<&str>,
        end: Option<&str>,
        limit: Option<u32>,
        page_token: Option<&str>,
        feed: Option<&str>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v2/stocks/auctions", data_url);
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

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// GET /v2/stocks/bars
    /// Retrieves historical bars for multiple stocks
    pub async fn get_bars_multi_raw(
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
        sort: Option<Sort>,
        adjustment: Option<Adjustment>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v2/stocks/bars", data_url);
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

        let sort_value = sort.unwrap_or_default();
        params.push(format!("sort={}", sort_value));

        let adjustment_value = adjustment.unwrap_or_default();
        params.push(format!("adjustment={}", adjustment_value));

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// GET /v2/stocks/bars/latest
    /// Retrieves latest bars for multiple stocks
    pub async fn get_latest_bars_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        symbols: &str,
        feed: Option<&str>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v2/stocks/bars/latest", data_url);
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

    /// GET /v2/stocks/meta/conditions
    /// Retrieves condition codes for stock quotes and trades
    pub async fn get_meta_conditions_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        ticktype: &str,
        tape: Option<&str>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v2/stocks/meta/conditions", data_url);
        let mut params = vec![format!("ticktype={}", ticktype)];

        if let Some(tape) = tape {
            params.push(format!("tape={}", tape));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// GET /v2/stocks/meta/exchanges
    /// Retrieves exchange codes for stock quotes and trades
    pub async fn get_meta_exchanges_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
    ) -> AlpacaResult<Response> {
        let url = format!("{}/v2/stocks/meta/exchanges", data_url);
        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// GET /v2/stocks/quotes
    /// Retrieves historical quotes for multiple stocks
    pub async fn get_quotes_raw(
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
        let mut url = format!("{}/v2/stocks/quotes", data_url);
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

    /// GET /v2/stocks/quotes/latest
    /// Retrieves latest quotes for multiple stocks
    pub async fn get_latest_quotes_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        symbols: &str,
        feed: Option<&str>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v2/stocks/quotes/latest", data_url);
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

    /// GET /v2/stocks/snapshots
    /// Retrieves market snapshots for multiple stocks
    pub async fn get_snapshots_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        symbols: &str,
        feed: Option<&str>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v2/stocks/snapshots", data_url);
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

    /// GET /v2/stocks/trades
    /// Retrieves historical trades for multiple stocks
    pub async fn get_trades_raw(
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
        let mut url = format!("{}/v2/stocks/trades", data_url);
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

    /// GET /v2/stocks/trades/latest
    /// Retrieves latest trades for multiple stocks
    pub async fn get_latest_trades_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        symbols: &str,
        feed: Option<&str>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v2/stocks/trades/latest", data_url);
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

    // ===== SINGLE SYMBOL ENDPOINTS =====

    /// GET /v2/stocks/{symbol}/auctions
    /// Retrieves daily auction data for a single stock
    pub async fn get_auction_single_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        symbol: &str,
        start: Option<&str>,
        end: Option<&str>,
        limit: Option<u32>,
        page_token: Option<&str>,
        feed: Option<&str>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v2/stocks/{}/auctions", data_url, symbol);
        let mut params = Vec::new();

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

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// GET /v2/stocks/{symbol}/bars
    /// Retrieves historical bars for a single stock
    pub async fn get_bars_single_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        symbol: &str,
        timeframe: &str,
        start: Option<&str>,
        end: Option<&str>,
        limit: Option<u32>,
        page_token: Option<&str>,
        feed: Option<&str>,
        sort: Option<Sort>,
        adjustment: Option<Adjustment>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v2/stocks/{}/bars", data_url, symbol);
        let mut params = vec![format!("timeframe={}", timeframe)];

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

        let sort_value = sort.unwrap_or_default();
        params.push(format!("sort={}", sort_value));

        let adjustment_value = adjustment.unwrap_or_default();
        params.push(format!("adjustment={}", adjustment_value));

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// GET /v2/stocks/{symbol}/bars/latest
    /// Retrieves latest bar for a single stock
    pub async fn get_latest_bar_single_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        symbol: &str,
        feed: Option<&str>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v2/stocks/{}/bars/latest", data_url, symbol);
        let mut params = Vec::new();

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

    /// GET /v2/stocks/{symbol}/quotes
    /// Retrieves historical quotes for a single stock
    pub async fn get_quotes_single_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        symbol: &str,
        start: Option<&str>,
        end: Option<&str>,
        limit: Option<u32>,
        page_token: Option<&str>,
        feed: Option<&str>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v2/stocks/{}/quotes", data_url, symbol);
        let mut params = Vec::new();

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

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// GET /v2/stocks/{symbol}/quotes/latest
    /// Retrieves latest quote for a single stock
    pub async fn get_latest_quote_single_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        symbol: &str,
        feed: Option<&str>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v2/stocks/{}/quotes/latest", data_url, symbol);
        let mut params = Vec::new();

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

    /// GET /v2/stocks/{symbol}/snapshot
    /// Retrieves market snapshot for a single stock
    pub async fn get_snapshot_single_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        symbol: &str,
        feed: Option<&str>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v2/stocks/{}/snapshot", data_url, symbol);
        let mut params = Vec::new();

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

    /// GET /v2/stocks/{symbol}/trades
    /// Retrieves historical trades for a single stock
    pub async fn get_trades_single_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        symbol: &str,
        start: Option<&str>,
        end: Option<&str>,
        limit: Option<u32>,
        page_token: Option<&str>,
        feed: Option<&str>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v2/stocks/{}/trades", data_url, symbol);
        let mut params = Vec::new();

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

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// GET /v2/stocks/{symbol}/trades/latest
    /// Retrieves latest trade for a single stock
    pub async fn get_latest_trade_single_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        symbol: &str,
        feed: Option<&str>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v2/stocks/{}/trades/latest", data_url, symbol);
        let mut params = Vec::new();

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
