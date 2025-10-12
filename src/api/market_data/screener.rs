use crate::models::AlpacaResult;
use reqwest::{Client, Response};

impl super::MarketDataApi {
    // ===== SCREENER MARKET DATA ENDPOINTS =====

    /// GET /v1beta1/screener/stocks/most-actives
    /// Returns the most active stocks by volume or trade count based on real time SIP data.
    /// By default, returns the top 10 symbols by volume.
    pub async fn get_most_actives_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        by: Option<&str>, // "volume" or "trades"
        top: Option<u32>, // Number of symbols to return (default: 10)
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v1beta1/screener/stocks/most-actives", data_url);
        let mut params = Vec::new();

        if let Some(by_param) = by {
            params.push(format!("by={}", by_param));
        }

        if let Some(top_param) = top {
            params.push(format!("top={}", top_param));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// GET /v1beta1/screener/{market_type}/movers
    /// Returns the top market movers (gainers and losers) based on real time SIP data.
    /// The change for each symbol is calculated from the previous closing price and the latest closing price.
    pub async fn get_movers_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        market_type: &str, // "stocks" or "crypto"
        top: Option<u32>,  // Number of gainers and losers to return
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v1beta1/screener/{}/movers", data_url, market_type);

        if let Some(top_param) = top {
            url.push_str(&format!("?top={}", top_param));
        }

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }
}
