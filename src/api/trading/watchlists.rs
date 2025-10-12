use crate::models::{
    AddAssetRequest, AlpacaResult, CreateWatchlistRequest, UpdateWatchlistRequest,
};
use reqwest::{Client, Response};

impl super::TradingApi {
    /// GET /v2/watchlists
    pub async fn get_watchlists_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
    ) -> AlpacaResult<Response> {
        let url = format!("{}/v2/watchlists", base_url);
        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// POST /v2/watchlists
    pub async fn create_watchlist_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        request: &CreateWatchlistRequest,
    ) -> AlpacaResult<Response> {
        let url = format!("{}/v2/watchlists", base_url);
        let response = client
            .post(&url)
            .headers(headers)
            .json(request)
            .send()
            .await?;
        Ok(response)
    }

    /// GET /v2/watchlists/{watchlist_id}
    pub async fn get_watchlist_by_id_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        watchlist_id: &str,
    ) -> AlpacaResult<Response> {
        let url = format!("{}/v2/watchlists/{}", base_url, watchlist_id);
        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// PUT /v2/watchlists/{watchlist_id}
    pub async fn update_watchlist_by_id_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        watchlist_id: &str,
        request: &UpdateWatchlistRequest,
    ) -> AlpacaResult<Response> {
        let url = format!("{}/v2/watchlists/{}", base_url, watchlist_id);
        let response = client
            .put(&url)
            .headers(headers)
            .json(request)
            .send()
            .await?;
        Ok(response)
    }

    /// DELETE /v2/watchlists/{watchlist_id}
    pub async fn delete_watchlist_by_id_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        watchlist_id: &str,
    ) -> AlpacaResult<Response> {
        let url = format!("{}/v2/watchlists/{}", base_url, watchlist_id);
        let response = client.delete(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// POST /v2/watchlists/{watchlist_id}
    pub async fn add_asset_to_watchlist_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        watchlist_id: &str,
        request: &AddAssetRequest,
    ) -> AlpacaResult<Response> {
        let url = format!("{}/v2/watchlists/{}", base_url, watchlist_id);
        let response = client
            .post(&url)
            .headers(headers)
            .json(request)
            .send()
            .await?;
        Ok(response)
    }

    /// DELETE /v2/watchlists/{watchlist_id}/{symbol}
    pub async fn remove_asset_from_watchlist_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        watchlist_id: &str,
        symbol: &str,
    ) -> AlpacaResult<Response> {
        let url = format!("{}/v2/watchlists/{}/{}", base_url, watchlist_id, symbol);
        let response = client.delete(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// GET /v2/watchlists:by_name
    pub async fn get_watchlist_by_name_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        name: &str,
    ) -> AlpacaResult<Response> {
        let url = format!("{}/v2/watchlists:by_name", base_url);
        let response = client
            .get(&url)
            .headers(headers)
            .query(&[("name", name)])
            .send()
            .await?;
        Ok(response)
    }

    /// PUT /v2/watchlists:by_name
    pub async fn update_watchlist_by_name_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        name: &str,
        request: &UpdateWatchlistRequest,
    ) -> AlpacaResult<Response> {
        let url = format!("{}/v2/watchlists:by_name", base_url);
        let response = client
            .put(&url)
            .headers(headers)
            .query(&[("name", name)])
            .json(request)
            .send()
            .await?;
        Ok(response)
    }

    /// POST /v2/watchlists:by_name
    pub async fn add_asset_to_watchlist_by_name_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        name: &str,
        request: &AddAssetRequest,
    ) -> AlpacaResult<Response> {
        let url = format!("{}/v2/watchlists:by_name", base_url);
        let response = client
            .post(&url)
            .headers(headers)
            .query(&[("name", name)])
            .json(request)
            .send()
            .await?;
        Ok(response)
    }

    /// DELETE /v2/watchlists:by_name
    pub async fn delete_watchlist_by_name_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        name: &str,
    ) -> AlpacaResult<Response> {
        let url = format!("{}/v2/watchlists:by_name", base_url);
        let response = client
            .delete(&url)
            .headers(headers)
            .query(&[("name", name)])
            .send()
            .await?;
        Ok(response)
    }
}
