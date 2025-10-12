use crate::models::AlpacaResult;
use reqwest::{Client, Response};

impl super::TradingApi {
    /// GET /v2/account
    pub async fn get_account_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
    ) -> AlpacaResult<Response> {
        let url = format!("{}/v2/account", base_url);
        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// GET /v2/account/configurations
    pub async fn get_account_configurations_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
    ) -> AlpacaResult<Response> {
        let url = format!("{}/v2/account/configurations", base_url);
        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// PATCH /v2/account/configurations
    pub async fn update_account_configurations_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        request: &crate::models::UpdateAccountConfigurationRequest,
    ) -> AlpacaResult<Response> {
        let url = format!("{}/v2/account/configurations", base_url);
        let response = client
            .patch(&url)
            .headers(headers)
            .json(request)
            .send()
            .await?;
        Ok(response)
    }

    /// GET /v2/account/activities
    pub async fn get_account_activities_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        activity_types: Option<&str>,
        date: Option<&str>,
        until: Option<&str>,
        after: Option<&str>,
        direction: Option<&str>,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v2/account/activities", base_url);
        let mut params = Vec::new();

        if let Some(types) = activity_types {
            params.push(format!("activity_types={}", types));
        }
        if let Some(date) = date {
            params.push(format!("date={}", date));
        }
        if let Some(until) = until {
            params.push(format!("until={}", until));
        }
        if let Some(after) = after {
            params.push(format!("after={}", after));
        }
        if let Some(direction) = direction {
            params.push(format!("direction={}", direction));
        }
        if let Some(page_size) = page_size {
            params.push(format!("page_size={}", page_size));
        }
        if let Some(page_token) = page_token {
            params.push(format!("page_token={}", page_token));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// GET /v2/account/activities/{activity_type}
    pub async fn get_account_activities_by_type_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        activity_type: &str,
        date: Option<&str>,
        until: Option<&str>,
        after: Option<&str>,
        direction: Option<&str>,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v2/account/activities/{}", base_url, activity_type);
        let mut params = Vec::new();

        if let Some(date) = date {
            params.push(format!("date={}", date));
        }
        if let Some(until) = until {
            params.push(format!("until={}", until));
        }
        if let Some(after) = after {
            params.push(format!("after={}", after));
        }
        if let Some(direction) = direction {
            params.push(format!("direction={}", direction));
        }
        if let Some(page_size) = page_size {
            params.push(format!("page_size={}", page_size));
        }
        if let Some(page_token) = page_token {
            params.push(format!("page_token={}", page_token));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }
}
