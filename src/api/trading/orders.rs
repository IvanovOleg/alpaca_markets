use crate::models::{AlpacaResult, OrderRequest};
use reqwest::{Client, Response};

impl super::TradingApi {
    /// POST /v2/orders
    pub async fn submit_order_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        order_request: &OrderRequest,
    ) -> AlpacaResult<Response> {
        let url = format!("{}/v2/orders", base_url);
        let response = client
            .post(&url)
            .headers(headers)
            .json(order_request)
            .send()
            .await?;
        Ok(response)
    }

    /// GET /v2/orders
    pub async fn get_orders_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        status: Option<&str>,
        limit: Option<u32>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v2/orders", base_url);
        let mut params = Vec::new();

        if let Some(status) = status {
            params.push(format!("status={}", status));
        }
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// DELETE /v2/orders/{order_id}
    pub async fn cancel_order_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        order_id: &str,
    ) -> AlpacaResult<Response> {
        let url = format!("{}/v2/orders/{}", base_url, order_id);
        let response = client.delete(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// GET /v2/orders/{order_id}
    pub async fn get_order_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        order_id: &str,
    ) -> AlpacaResult<Response> {
        let url = format!("{}/v2/orders/{}", base_url, order_id);
        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// PATCH /v2/orders/{order_id}
    pub async fn replace_order_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        order_id: &str,
        order_request: &OrderRequest,
    ) -> AlpacaResult<Response> {
        let url = format!("{}/v2/orders/{}", base_url, order_id);
        let response = client
            .patch(&url)
            .headers(headers)
            .json(order_request)
            .send()
            .await?;
        Ok(response)
    }

    /// DELETE /v2/orders
    pub async fn cancel_all_orders_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
    ) -> AlpacaResult<Response> {
        let url = format!("{}/v2/orders", base_url);
        let response = client.delete(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// GET /v2/orders:by_client_order_id
    pub async fn get_order_by_client_id_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        client_order_id: &str,
    ) -> AlpacaResult<Response> {
        let url = format!("{}/v2/orders:by_client_order_id", base_url);
        let response = client
            .get(&url)
            .headers(headers)
            .query(&[("client_order_id", client_order_id)])
            .send()
            .await?;
        Ok(response)
    }
}
