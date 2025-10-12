use crate::models::AlpacaResult;
use reqwest::{Client, Response};

impl super::MarketDataApi {
    // ===== NEWS MARKET DATA ENDPOINTS =====

    /// GET /v1beta1/news
    /// Returns the latest news articles across stocks and crypto.
    /// By default, returns the latest 10 news articles.
    pub async fn get_news_raw(
        client: &Client,
        data_url: &str,
        headers: reqwest::header::HeaderMap,
        symbols: Option<&str>,             // Comma-separated list of symbols
        start: Option<&str>,               // Start timestamp (RFC3339 or YYYY-MM-DD)
        end: Option<&str>,                 // End timestamp (RFC3339 or YYYY-MM-DD)
        sort: Option<&str>,                // "asc" or "desc"
        include_content: Option<bool>,     // Include article content
        exclude_contentless: Option<bool>, // Exclude articles without content
        limit: Option<u32>,                // Number of articles to return (default: 10, max: 50)
        page_token: Option<&str>,          // Pagination token
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v1beta1/news", data_url);
        let mut params = Vec::new();

        if let Some(symbols_param) = symbols {
            params.push(format!("symbols={}", symbols_param));
        }

        if let Some(start_param) = start {
            params.push(format!("start={}", start_param));
        }

        if let Some(end_param) = end {
            params.push(format!("end={}", end_param));
        }

        if let Some(sort_param) = sort {
            params.push(format!("sort={}", sort_param));
        }

        if let Some(include_content_param) = include_content {
            params.push(format!("include_content={}", include_content_param));
        }

        if let Some(exclude_contentless_param) = exclude_contentless {
            params.push(format!("exclude_contentless={}", exclude_contentless_param));
        }

        if let Some(limit_param) = limit {
            params.push(format!("limit={}", limit_param));
        }

        if let Some(page_token_param) = page_token {
            params.push(format!("page_token={}", page_token_param));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }
}
