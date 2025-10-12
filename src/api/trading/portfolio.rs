use crate::models::AlpacaResult;
use reqwest::{Client, Response};

impl super::TradingApi {
    /// GET /v2/account/portfolio/history
    pub async fn get_portfolio_history_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        period: Option<&str>,
        timeframe: Option<&str>,
        date_end: Option<&str>,
        asof: Option<&str>,
        page_token: Option<&str>,
        intraday_reporting: Option<&str>,
        pnl_reset: Option<&str>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v2/account/portfolio/history", base_url);
        let mut params = Vec::new();

        if let Some(period) = period {
            params.push(format!("period={}", period));
        }
        if let Some(timeframe) = timeframe {
            params.push(format!("timeframe={}", timeframe));
        }
        if let Some(date_end) = date_end {
            params.push(format!("date_end={}", date_end));
        }
        if let Some(asof) = asof {
            params.push(format!("asof={}", asof));
        }
        if let Some(page_token) = page_token {
            params.push(format!("page_token={}", page_token));
        }
        if let Some(intraday_reporting) = intraday_reporting {
            params.push(format!("intraday_reporting={}", intraday_reporting));
        }
        if let Some(pnl_reset) = pnl_reset {
            params.push(format!("pnl_reset={}", pnl_reset));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }
}
