use crate::models::AlpacaResult;
use reqwest::{Client, Response};

impl super::TradingApi {
    /// GET /v2/assets
    pub async fn get_assets_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        status: Option<&str>,
        asset_class: Option<&str>,
        exchange: Option<&str>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v2/assets", base_url);
        let mut params = Vec::new();

        if let Some(status) = status {
            params.push(format!("status={}", status));
        }
        if let Some(asset_class) = asset_class {
            params.push(format!("asset_class={}", asset_class));
        }
        if let Some(exchange) = exchange {
            params.push(format!("exchange={}", exchange));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// GET /v2/assets/{symbol_or_asset_id}
    pub async fn get_asset_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        symbol_or_asset_id: &str,
    ) -> AlpacaResult<Response> {
        let url = format!("{}/v2/assets/{}", base_url, symbol_or_asset_id);
        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }
}
