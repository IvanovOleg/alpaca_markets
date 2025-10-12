use crate::models::AlpacaResult;
use reqwest::{Client, Response};

impl super::MarketDataApi {
    // ===== FIXED INCOME MARKET DATA ENDPOINTS =====

    /// GET /v1/fixed-income/prices/latest
    /// Retrieves latest prices for fixed income securities
    pub async fn get_fixed_income_latest_prices_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        symbols: &str,
        feed: Option<&str>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v1/fixed-income/prices/latest", data_url);
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
