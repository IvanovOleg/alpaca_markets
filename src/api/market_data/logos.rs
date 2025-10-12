use crate::models::AlpacaResult;
use reqwest::{Client, Response};

impl super::MarketDataApi {
    // ===== LOGOS MARKET DATA ENDPOINTS =====

    /// GET /v1/logos/{symbol}
    /// Retrieves logo URL for a given symbol
    pub async fn get_logo_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        symbol: &str,
    ) -> AlpacaResult<Response> {
        let url = format!("{}/v1/logos/{}", data_url, symbol);
        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }
}
