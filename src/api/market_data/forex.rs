use crate::models::AlpacaResult;
use reqwest::{Client, Response};

impl super::MarketDataApi {
    // ===== FOREX MARKET DATA ENDPOINTS =====

    /// GET /v1/forex/rates/latest
    /// Retrieves latest exchange rates for currency pairs
    pub async fn get_forex_latest_rates_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        currency_pairs: &str,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v1/forex/rates/latest", data_url);
        let params = vec![format!("currency_pairs={}", currency_pairs)];

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// GET /v1/forex/rates
    /// Retrieves historical exchange rates for currency pairs
    pub async fn get_forex_rates_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        currency_pairs: &str,
        timeframe: &str,
        start: Option<&str>,
        end: Option<&str>,
        limit: Option<u32>,
        page_token: Option<&str>,
        sort: Option<&str>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v1/forex/rates", data_url);
        let mut params = vec![
            format!("currency_pairs={}", currency_pairs),
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
}
