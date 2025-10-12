use crate::models::AlpacaResult;
use reqwest::{Client, Response};

impl super::TradingApi {
    /// GET /v2/positions
    pub async fn get_positions_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
    ) -> AlpacaResult<Response> {
        let url = format!("{}/v2/positions", base_url);
        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// GET /v2/positions/{symbol_or_asset_id}
    pub async fn get_position_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        symbol_or_asset_id: &str,
    ) -> AlpacaResult<Response> {
        let url = format!("{}/v2/positions/{}", base_url, symbol_or_asset_id);
        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// DELETE /v2/positions/{symbol_or_asset_id}
    pub async fn close_position_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        symbol_or_asset_id: &str,
        qty: Option<&str>,
        percentage: Option<&str>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v2/positions/{}", base_url, symbol_or_asset_id);
        let mut params = Vec::new();

        if let Some(qty) = qty {
            params.push(format!("qty={}", qty));
        }
        if let Some(percentage) = percentage {
            params.push(format!("percentage={}", percentage));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = client.delete(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// DELETE /v2/positions
    pub async fn close_all_positions_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        cancel_orders: Option<bool>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v2/positions", base_url);

        if let Some(cancel) = cancel_orders {
            url.push_str(&format!("?cancel_orders={}", cancel));
        }

        let response = client.delete(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// POST /v2/positions/{symbol_or_contract_id}/exercise
    pub async fn exercise_option_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        symbol_or_contract_id: &str,
    ) -> AlpacaResult<Response> {
        let url = format!(
            "{}/v2/positions/{}/exercise",
            base_url, symbol_or_contract_id
        );
        let response = client.post(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// POST /v2/positions/{symbol_or_contract_id}/do-not-exercise
    pub async fn do_not_exercise_option_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        symbol_or_contract_id: &str,
    ) -> AlpacaResult<Response> {
        let url = format!(
            "{}/v2/positions/{}/do-not-exercise",
            base_url, symbol_or_contract_id
        );
        let response = client.post(&url).headers(headers).send().await?;
        Ok(response)
    }
}
