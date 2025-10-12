use crate::models::{AlpacaResult, CreateCryptoTransferRequest, CreateWhitelistedAddressRequest};
use reqwest::{Client, Response};

impl super::TradingApi {
    /// GET /v2/wallets
    pub async fn list_crypto_wallets_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        asset: Option<&str>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v2/wallets", base_url);

        if let Some(asset) = asset {
            url.push_str(&format!("?asset={}", asset));
        }

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// GET /v2/wallets/transfers
    pub async fn list_crypto_transfers_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        asset: Option<&str>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v2/wallets/transfers", base_url);

        if let Some(asset) = asset {
            url.push_str(&format!("?asset={}", asset));
        }

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// POST /v2/wallets/transfers
    pub async fn create_crypto_transfer_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        request: &CreateCryptoTransferRequest,
    ) -> AlpacaResult<Response> {
        let url = format!("{}/v2/wallets/transfers", base_url);
        let response = client
            .post(&url)
            .headers(headers)
            .json(request)
            .send()
            .await?;
        Ok(response)
    }

    /// GET /v2/wallets/transfers/{transfer_id}
    pub async fn get_crypto_transfer_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        transfer_id: &str,
    ) -> AlpacaResult<Response> {
        let url = format!("{}/v2/wallets/transfers/{}", base_url, transfer_id);
        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// GET /v2/wallets/whitelisted_addresses
    pub async fn list_whitelisted_addresses_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        asset: Option<&str>,
    ) -> AlpacaResult<Response> {
        let mut url = format!("{}/v2/wallets/whitelisted_addresses", base_url);

        if let Some(asset) = asset {
            url.push_str(&format!("?asset={}", asset));
        }

        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// POST /v2/wallets/whitelisted_addresses
    pub async fn create_whitelisted_address_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        request: &CreateWhitelistedAddressRequest,
    ) -> AlpacaResult<Response> {
        let url = format!("{}/v2/wallets/whitelisted_addresses", base_url);
        let response = client
            .post(&url)
            .headers(headers)
            .json(request)
            .send()
            .await?;
        Ok(response)
    }

    /// DELETE /v2/wallets/whitelisted_addresses/{address_id}
    pub async fn delete_whitelisted_address_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        address_id: &str,
    ) -> AlpacaResult<Response> {
        let url = format!(
            "{}/v2/wallets/whitelisted_addresses/{}",
            base_url, address_id
        );
        let response = client.delete(&url).headers(headers).send().await?;
        Ok(response)
    }

    /// GET /v2/wallets/transfers/estimate
    pub async fn get_crypto_transfer_estimate_raw(
        client: &Client,
        base_url: &str,
        headers: reqwest::header::HeaderMap,
        asset: &str,
        amount: &str,
    ) -> AlpacaResult<Response> {
        let url = format!(
            "{}/v2/wallets/transfers/estimate?asset={}&amount={}",
            base_url, asset, amount
        );
        let response = client.get(&url).headers(headers).send().await?;
        Ok(response)
    }
}
