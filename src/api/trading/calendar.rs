use crate::models::AlpacaResult;
use reqwest::{Client, Response};

impl super::TradingApi {
    /// GET /v2/calendar
    pub async fn get_calendar_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        start: Option<&str>,
        end: Option<&str>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v2/calendar", base_url);
        let mut params = Vec::new();

        if let Some(start) = start {
            params.push(format!("start={}", start));
        }
        if let Some(end) = end {
            params.push(format!("end={}", end));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// GET /v2/clock
    pub async fn get_clock_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
    ) -> AlpacaResult<Response> {
        let url = format!("{}/v2/clock", base_url);
        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }
}
