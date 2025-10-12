use crate::models::AlpacaResult;
use reqwest::{Client, Response};

impl super::MarketDataApi {
    // ===== CORPORATE ACTIONS MARKET DATA ENDPOINTS =====

    /// GET /v1/corporate-actions
    /// This endpoint provides data about the corporate actions for each given symbol
    /// over a specified time period.
    pub async fn get_corporate_actions_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        symbols: Option<&str>,    // Comma-separated list of symbols
        since: Option<&str>,      // Start date for filtering (RFC3339 or YYYY-MM-DD)
        until: Option<&str>,      // End date for filtering (RFC3339 or YYYY-MM-DD)
        ca_types: Option<&str>,   // Comma-separated list of corporate action types
        limit: Option<u32>,       // Number of records to return (default: 1000, max: 10000)
        page_token: Option<&str>, // Pagination token
        sort: Option<&str>,       // Sort order ("asc" or "desc")
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v1/corporate-actions", data_url);
        let mut params = Vec::new();

        if let Some(symbols_param) = symbols {
            params.push(format!("symbols={}", symbols_param));
        }

        if let Some(since_param) = since {
            params.push(format!("since={}", since_param));
        }

        if let Some(until_param) = until {
            params.push(format!("until={}", until_param));
        }

        if let Some(ca_types_param) = ca_types {
            params.push(format!("ca_types={}", ca_types_param));
        }

        if let Some(limit_param) = limit {
            params.push(format!("limit={}", limit_param));
        }

        if let Some(page_token_param) = page_token {
            params.push(format!("page_token={}", page_token_param));
        }

        if let Some(sort_param) = sort {
            params.push(format!("sort={}", sort_param));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }
}
