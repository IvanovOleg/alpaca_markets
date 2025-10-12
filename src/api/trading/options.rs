use crate::models::AlpacaResult;
use reqwest::{Client, Response};

impl super::TradingApi {
    /// GET /v2/options/contracts
    pub async fn get_option_contracts_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        underlying_symbols: Option<&str>,
        status: Option<&str>,
        expiration_date: Option<&str>,
        expiration_date_gte: Option<&str>,
        expiration_date_lte: Option<&str>,
        root_symbol: Option<&str>,
        r#type: Option<&str>,
        style: Option<&str>,
        strike_price_gte: Option<&str>,
        strike_price_lte: Option<&str>,
        page_token: Option<&str>,
        limit: Option<u32>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v2/options/contracts", base_url);
        let mut params = Vec::new();

        if let Some(symbols) = underlying_symbols {
            params.push(format!("underlying_symbols={}", symbols));
        }
        if let Some(status) = status {
            params.push(format!("status={}", status));
        }
        if let Some(exp_date) = expiration_date {
            params.push(format!("expiration_date={}", exp_date));
        }
        if let Some(exp_gte) = expiration_date_gte {
            params.push(format!("expiration_date_gte={}", exp_gte));
        }
        if let Some(exp_lte) = expiration_date_lte {
            params.push(format!("expiration_date_lte={}", exp_lte));
        }
        if let Some(root) = root_symbol {
            params.push(format!("root_symbol={}", root));
        }
        if let Some(contract_type) = r#type {
            params.push(format!("type={}", contract_type));
        }
        if let Some(style) = style {
            params.push(format!("style={}", style));
        }
        if let Some(strike_gte) = strike_price_gte {
            params.push(format!("strike_price_gte={}", strike_gte));
        }
        if let Some(strike_lte) = strike_price_lte {
            params.push(format!("strike_price_lte={}", strike_lte));
        }
        if let Some(token) = page_token {
            params.push(format!("page_token={}", token));
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

    /// GET /v2/options/contracts/{symbol_or_id}
    pub async fn get_option_contract_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        symbol_or_id: &str,
    ) -> AlpacaResult<Response> {
        let url = format!("{}/v2/options/contracts/{}", base_url, symbol_or_id);
        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }
}
