use crate::api::trading::TradingApi;
use crate::config::AlpacaConfig;
use crate::models::{
    Account, AccountActivity, AccountConfiguration, AddAssetRequest, AlpacaError, AlpacaResult,
    Asset, CreateCryptoTransferRequest, CreateWatchlistRequest, CreateWhitelistedAddressRequest,
    CryptoTransfer, CryptoTransferEstimate, CryptoWallet, MarketCalendar, MarketClock,
    OptionContract, Order, OrderRequest, PortfolioHistory, Position,
    UpdateAccountConfigurationRequest, UpdateWatchlistRequest, Watchlist, WhitelistedAddress,
};
use reqwest::Client;

/// High-level client for trading operations
pub struct TradingClient {
    client: Client,
    config: AlpacaConfig,
}

impl TradingClient {
    /// Create a new trading client
    pub fn new(config: AlpacaConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    /// Get account information
    pub async fn get_account(&self) -> AlpacaResult<Account> {
        let response = TradingApi::get_account_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
        )
        .await?;

        if response.status().is_success() {
            let account: Account = response.json().await?;
            Ok(account)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Get account configurations
    /// Returns the current account configuration values including day trade buying power check,
    /// trade confirmation email, suspend trade, max margin multiplier, and pattern day trader check
    pub async fn get_account_configurations(&self) -> AlpacaResult<AccountConfiguration> {
        let response = TradingApi::get_account_configurations_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
        )
        .await?;

        if response.status().is_success() {
            let config: AccountConfiguration = response.json().await?;
            Ok(config)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Update account configurations
    /// Updates and returns the current account configuration values
    /// You can update any combination of the available configuration options
    ///
    /// # Parameters
    /// - `request`: UpdateAccountConfigurationRequest with the fields to update
    ///
    /// # Available Configuration Options
    /// - `dtbp_check`: Day Trade Buying Power Check (none, entry, both)
    /// - `trade_confirm_email`: Trade Confirmation Email (none, all)
    /// - `suspend_trade`: Suspend Trade (true/false)
    /// - `max_margin_multiplier`: Maximum Margin Multiplier (string value)
    /// - `pdt_check`: Pattern Day Trader Check (none, entry, both)
    pub async fn update_account_configurations(
        &self,
        request: UpdateAccountConfigurationRequest,
    ) -> AlpacaResult<AccountConfiguration> {
        let response = TradingApi::update_account_configurations_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            &request,
        )
        .await?;

        if response.status().is_success() {
            let config: AccountConfiguration = response.json().await?;
            Ok(config)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Get account activities
    /// Returns a list of activities for the account
    ///
    /// # Parameters
    /// - `activity_types`: Comma-separated list of activity types to filter by (optional)
    /// - `date`: Date to filter activities for (YYYY-MM-DD format, optional)
    /// - `until`: Activities until this date (YYYY-MM-DD format, optional)
    /// - `after`: Activities after this date (YYYY-MM-DD format, optional)
    /// - `direction`: Sort direction - "asc" or "desc" (optional, default "desc")
    /// - `page_size`: Number of activities per page (optional, max 5000)
    /// - `page_token`: Pagination token (optional)
    pub async fn get_account_activities(
        &self,
        activity_types: Option<&str>,
        date: Option<&str>,
        until: Option<&str>,
        after: Option<&str>,
        direction: Option<&str>,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> AlpacaResult<Vec<AccountActivity>> {
        let response = TradingApi::get_account_activities_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            activity_types,
            date,
            until,
            after,
            direction,
            page_size,
            page_token,
        )
        .await?;

        if response.status().is_success() {
            let activities: Vec<AccountActivity> = response.json().await?;
            Ok(activities)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Get account activities by type
    /// Returns a list of activities for the account filtered by a specific activity type
    ///
    /// # Parameters
    /// - `activity_type`: The specific activity type to filter by (e.g., "FILL", "DIV", "FEE")
    /// - `date`: Date to filter activities for (YYYY-MM-DD format, optional)
    /// - `until`: Activities until this date (YYYY-MM-DD format, optional)
    /// - `after`: Activities after this date (YYYY-MM-DD format, optional)
    /// - `direction`: Sort direction - "asc" or "desc" (optional, default "desc")
    /// - `page_size`: Number of activities per page (optional, max 5000)
    /// - `page_token`: Pagination token (optional)
    pub async fn get_account_activities_by_type(
        &self,
        activity_type: &str,
        date: Option<&str>,
        until: Option<&str>,
        after: Option<&str>,
        direction: Option<&str>,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> AlpacaResult<Vec<AccountActivity>> {
        let response = TradingApi::get_account_activities_by_type_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            activity_type,
            date,
            until,
            after,
            direction,
            page_size,
            page_token,
        )
        .await?;

        if response.status().is_success() {
            let activities: Vec<AccountActivity> = response.json().await?;
            Ok(activities)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Get market calendar (open/close times) for given date range
    /// Returns a list of calendar objects for trading days
    ///
    /// # Parameters
    /// - `start`: Start date in YYYY-MM-DD format (optional)
    /// - `end`: End date in YYYY-MM-DD format (optional)
    ///
    /// # Example
    /// ```rust
    /// // Get all trading days for January 2023
    /// let calendar = client.get_calendar(Some("2023-01-01"), Some("2023-01-31")).await?;
    /// println!("Market days in January 2023: {}", calendar.len());
    ///
    /// // Get next 10 trading days (no date filters)
    /// let upcoming = client.get_calendar(None, None).await?;
    /// ```
    pub async fn get_calendar(
        &self,
        start: Option<&str>,
        end: Option<&str>,
    ) -> AlpacaResult<Vec<MarketCalendar>> {
        let response = TradingApi::get_calendar_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            start,
            end,
        )
        .await?;

        if response.status().is_success() {
            let calendar: Vec<MarketCalendar> = response.json().await?;
            Ok(calendar)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Get market clock information
    /// Returns current market status, timestamps, and next open/close times
    ///
    /// # Example
    /// ```rust
    /// let clock = client.get_clock().await?;
    /// println!("Market is open: {}", clock.is_open);
    /// println!("Current time: {}", clock.timestamp);
    /// ```
    pub async fn get_clock(&self) -> AlpacaResult<MarketClock> {
        let response = TradingApi::get_clock_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
        )
        .await?;

        if response.status().is_success() {
            let clock: MarketClock = response.json().await?;
            Ok(clock)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// List crypto funding wallets
    /// Returns a list of wallets associated with the account
    ///
    /// # Parameters
    /// - `asset`: Filter by specific asset (optional)
    ///
    /// # Example
    /// ```rust
    /// // Get all wallets
    /// let wallets = client.list_crypto_wallets(None).await?;
    ///
    /// // Get BTC wallet only
    /// let btc_wallets = client.list_crypto_wallets(Some("BTC")).await?;
    /// ```
    pub async fn list_crypto_wallets(
        &self,
        asset: Option<&str>,
    ) -> AlpacaResult<Vec<CryptoWallet>> {
        let response = TradingApi::list_crypto_wallets_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            asset,
        )
        .await?;

        if response.status().is_success() {
            let wallets: Vec<CryptoWallet> = response.json().await?;
            Ok(wallets)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// List crypto funding transfers
    /// Returns a list of all transfers for the account
    ///
    /// # Parameters
    /// - `asset`: Filter by specific asset (optional)
    ///
    /// # Example
    /// ```rust
    /// let transfers = client.list_crypto_transfers(None).await?;
    /// ```
    pub async fn list_crypto_transfers(
        &self,
        asset: Option<&str>,
    ) -> AlpacaResult<Vec<CryptoTransfer>> {
        let response = TradingApi::list_crypto_transfers_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            asset,
        )
        .await?;

        if response.status().is_success() {
            let transfers: Vec<CryptoTransfer> = response.json().await?;
            Ok(transfers)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Create a crypto transfer (withdrawal)
    /// Request a withdrawal to a whitelisted address
    ///
    /// # Parameters
    /// - `request`: Transfer details including asset, amount, and destination address
    ///
    /// # Example
    /// ```rust
    /// let request = CreateCryptoTransferRequest {
    ///     asset: "BTC".to_string(),
    ///     amount: "0.001".to_string(),
    ///     address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
    /// };
    /// let transfer = client.create_crypto_transfer(request).await?;
    /// ```
    pub async fn create_crypto_transfer(
        &self,
        request: CreateCryptoTransferRequest,
    ) -> AlpacaResult<CryptoTransfer> {
        let response = TradingApi::create_crypto_transfer_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            &request,
        )
        .await?;

        if response.status().is_success() {
            let transfer: CryptoTransfer = response.json().await?;
            Ok(transfer)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Get a specific crypto transfer by ID
    ///
    /// # Parameters
    /// - `transfer_id`: The transfer ID to retrieve
    ///
    /// # Example
    /// ```rust
    /// let transfer = client.get_crypto_transfer("transfer_123").await?;
    /// ```
    pub async fn get_crypto_transfer(&self, transfer_id: &str) -> AlpacaResult<CryptoTransfer> {
        let response = TradingApi::get_crypto_transfer_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            transfer_id,
        )
        .await?;

        if response.status().is_success() {
            let transfer: CryptoTransfer = response.json().await?;
            Ok(transfer)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// List whitelisted addresses
    /// Returns addresses approved for withdrawals
    ///
    /// # Parameters
    /// - `asset`: Filter by specific asset (optional)
    ///
    /// # Example
    /// ```rust
    /// let addresses = client.list_whitelisted_addresses(Some("BTC")).await?;
    /// ```
    pub async fn list_whitelisted_addresses(
        &self,
        asset: Option<&str>,
    ) -> AlpacaResult<Vec<WhitelistedAddress>> {
        let response = TradingApi::list_whitelisted_addresses_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            asset,
        )
        .await?;

        if response.status().is_success() {
            let addresses: Vec<WhitelistedAddress> = response.json().await?;
            Ok(addresses)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Create a whitelisted address
    /// Add a new address for future withdrawals (requires 24h activation period)
    ///
    /// # Parameters
    /// - `request`: Address details including asset, address, and optional label
    ///
    /// # Example
    /// ```rust
    /// let request = CreateWhitelistedAddressRequest {
    ///     asset: "BTC".to_string(),
    ///     address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
    ///     label: Some("My BTC Wallet".to_string()),
    /// };
    /// let address = client.create_whitelisted_address(request).await?;
    /// ```
    pub async fn create_whitelisted_address(
        &self,
        request: CreateWhitelistedAddressRequest,
    ) -> AlpacaResult<WhitelistedAddress> {
        let response = TradingApi::create_whitelisted_address_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            &request,
        )
        .await?;

        if response.status().is_success() {
            let address: WhitelistedAddress = response.json().await?;
            Ok(address)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Delete a whitelisted address
    /// Remove an address from the whitelist
    ///
    /// # Parameters
    /// - `address_id`: The address ID to delete
    ///
    /// # Example
    /// ```rust
    /// client.delete_whitelisted_address("address_123").await?;
    /// ```
    pub async fn delete_whitelisted_address(&self, address_id: &str) -> AlpacaResult<()> {
        let response = TradingApi::delete_whitelisted_address_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            address_id,
        )
        .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Get crypto transfer fee estimate
    /// Estimate the fee for a potential transfer
    ///
    /// # Parameters
    /// - `asset`: Asset symbol
    /// - `amount`: Transfer amount
    ///
    /// # Example
    /// ```rust
    /// let estimate = client.get_crypto_transfer_estimate("BTC", "0.001").await?;
    /// println!("Fee: {}, Total: {}", estimate.fee, estimate.total);
    /// ```
    pub async fn get_crypto_transfer_estimate(
        &self,
        asset: &str,
        amount: &str,
    ) -> AlpacaResult<CryptoTransferEstimate> {
        let response = TradingApi::get_crypto_transfer_estimate_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            asset,
            amount,
        )
        .await?;

        if response.status().is_success() {
            let estimate: CryptoTransferEstimate = response.json().await?;
            Ok(estimate)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Get account portfolio history
    /// Returns timeseries data about equity and profit/loss (P/L) of the account in requested timespan
    ///
    /// # Parameters
    /// - `period`: The duration of the data in number of days or timespan (e.g., "1D", "7D", "1M", "1Y")
    /// - `timeframe`: The resolution of time window (e.g., "1Min", "5Min", "15Min", "1Hour", "1Day")
    /// - `date_end`: The end date for the data (YYYY-MM-DD format)
    /// - `asof`: The date to use for historical data (YYYY-MM-DD format)
    /// - `page_token`: Token for pagination
    /// - `intraday_reporting`: "market_hours" (default) or "extended_hours" or "continuous"
    /// - `pnl_reset`: "per_day" (default) or "no_reset"
    pub async fn get_portfolio_history(
        &self,
        period: Option<&str>,
        timeframe: Option<&str>,
        date_end: Option<&str>,
        asof: Option<&str>,
        page_token: Option<&str>,
        intraday_reporting: Option<&str>,
        pnl_reset: Option<&str>,
    ) -> AlpacaResult<PortfolioHistory> {
        let response = TradingApi::get_portfolio_history_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            period,
            timeframe,
            date_end,
            asof,
            page_token,
            intraday_reporting,
            pnl_reset,
        )
        .await?;

        if response.status().is_success() {
            let portfolio_history: PortfolioHistory = response.json().await?;
            Ok(portfolio_history)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Submit a new order
    pub async fn submit_order(&self, order_request: OrderRequest) -> AlpacaResult<Order> {
        let response = TradingApi::submit_order_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            &order_request,
        )
        .await?;

        if response.status().is_success() {
            let order: Order = response.json().await?;
            Ok(order)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Get all orders
    pub async fn get_orders(
        &self,
        status: Option<&str>,
        limit: Option<u32>,
    ) -> AlpacaResult<Vec<Order>> {
        let response = TradingApi::get_orders_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            status,
            limit,
        )
        .await?;

        if response.status().is_success() {
            let orders: Vec<Order> = response.json().await?;
            Ok(orders)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Get all positions
    pub async fn get_positions(&self) -> AlpacaResult<Vec<Position>> {
        let response = TradingApi::get_positions_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
        )
        .await?;

        if response.status().is_success() {
            let positions: Vec<Position> = response.json().await?;
            Ok(positions)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Cancel an order by ID
    pub async fn cancel_order(&self, order_id: &str) -> AlpacaResult<()> {
        let response = TradingApi::cancel_order_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            order_id,
        )
        .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Get a specific order by ID
    pub async fn get_order(&self, order_id: &str) -> AlpacaResult<Order> {
        let response = TradingApi::get_order_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            order_id,
        )
        .await?;

        if response.status().is_success() {
            let order: Order = response.json().await?;
            Ok(order)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Replace an existing order
    pub async fn replace_order(
        &self,
        order_id: &str,
        order_request: OrderRequest,
    ) -> AlpacaResult<Order> {
        let response = TradingApi::replace_order_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            order_id,
            &order_request,
        )
        .await?;

        if response.status().is_success() {
            let order: Order = response.json().await?;
            Ok(order)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Cancel all open orders
    pub async fn cancel_all_orders(&self) -> AlpacaResult<Vec<Order>> {
        let response = TradingApi::cancel_all_orders_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
        )
        .await?;

        if response.status().is_success() {
            let orders: Vec<Order> = response.json().await?;
            Ok(orders)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Get order by client order ID
    pub async fn get_order_by_client_id(&self, client_order_id: &str) -> AlpacaResult<Order> {
        let response = TradingApi::get_order_by_client_id_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            client_order_id,
        )
        .await?;

        if response.status().is_success() {
            let order: Order = response.json().await?;
            Ok(order)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Get a specific position by symbol
    pub async fn get_position(&self, symbol: &str) -> AlpacaResult<Position> {
        let response = TradingApi::get_position_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            symbol,
        )
        .await?;

        if response.status().is_success() {
            let position: Position = response.json().await?;
            Ok(position)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Close a position (liquidate)
    pub async fn close_position(
        &self,
        symbol: &str,
        qty: Option<&str>,
        percentage: Option<&str>,
    ) -> AlpacaResult<Order> {
        let response = TradingApi::close_position_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            symbol,
            qty,
            percentage,
        )
        .await?;

        if response.status().is_success() {
            let order: Order = response.json().await?;
            Ok(order)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Close all positions
    pub async fn close_all_positions(
        &self,
        cancel_orders: Option<bool>,
    ) -> AlpacaResult<Vec<Order>> {
        let response = TradingApi::close_all_positions_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            cancel_orders,
        )
        .await?;

        if response.status().is_success() {
            let orders: Vec<Order> = response.json().await?;
            Ok(orders)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Get all assets with optional filtering
    pub async fn get_assets(
        &self,
        status: Option<&str>,
        asset_class: Option<&str>,
        exchange: Option<&str>,
    ) -> AlpacaResult<Vec<Asset>> {
        let response = TradingApi::get_assets_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            status,
            asset_class,
            exchange,
        )
        .await?;

        if response.status().is_success() {
            let assets: Vec<Asset> = response.json().await?;
            Ok(assets)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Get a specific asset by symbol or ID
    pub async fn get_asset(&self, symbol_or_id: &str) -> AlpacaResult<Asset> {
        let response = TradingApi::get_asset_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            symbol_or_id,
        )
        .await?;

        if response.status().is_success() {
            let asset: Asset = response.json().await?;
            Ok(asset)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Get option contracts with filtering
    pub async fn get_option_contracts(
        &self,
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
    ) -> AlpacaResult<Vec<OptionContract>> {
        let response = TradingApi::get_option_contracts_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            underlying_symbols,
            status,
            expiration_date,
            expiration_date_gte,
            expiration_date_lte,
            root_symbol,
            r#type,
            style,
            strike_price_gte,
            strike_price_lte,
            page_token,
            limit,
        )
        .await?;

        if response.status().is_success() {
            let contracts: Vec<OptionContract> = response.json().await?;
            Ok(contracts)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Get a specific option contract by symbol or ID
    pub async fn get_option_contract(&self, symbol_or_id: &str) -> AlpacaResult<OptionContract> {
        let response = TradingApi::get_option_contract_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            symbol_or_id,
        )
        .await?;

        if response.status().is_success() {
            let contract: OptionContract = response.json().await?;
            Ok(contract)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Exercise an option position
    /// Converts all available held shares of the option contract into the underlying asset
    /// By default, Alpaca will automatically exercise in-the-money (ITM) contracts at expiry
    /// Exercise requests will be processed immediately once received
    /// Exercise requests submitted between market close and midnight will be rejected
    pub async fn exercise_option(&self, symbol_or_contract_id: &str) -> AlpacaResult<Order> {
        let response = TradingApi::exercise_option_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            symbol_or_contract_id,
        )
        .await?;

        if response.status().is_success() {
            let order: Order = response.json().await?;
            Ok(order)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Submit a do-not-exercise (DNE) instruction for an option position
    /// Prevents automatic exercise of in-the-money (ITM) contracts at expiry
    /// By default, Alpaca automatically exercises ITM contracts at expiry
    /// This method allows users to override that behavior and prevent exercise
    /// To cancel a DNE instruction or submit an exercise, contact support
    pub async fn do_not_exercise_option(&self, symbol_or_contract_id: &str) -> AlpacaResult<Order> {
        let response = TradingApi::do_not_exercise_option_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            symbol_or_contract_id,
        )
        .await?;

        if response.status().is_success() {
            let order: Order = response.json().await?;
            Ok(order)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Get all watchlists for the account
    pub async fn get_watchlists(&self) -> AlpacaResult<Vec<Watchlist>> {
        let response = TradingApi::get_watchlists_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
        )
        .await?;

        if response.status().is_success() {
            let watchlists: Vec<Watchlist> = response.json().await?;
            Ok(watchlists)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Create a new watchlist
    pub async fn create_watchlist(
        &self,
        request: CreateWatchlistRequest,
    ) -> AlpacaResult<Watchlist> {
        let response = TradingApi::create_watchlist_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            &request,
        )
        .await?;

        if response.status().is_success() {
            let watchlist: Watchlist = response.json().await?;
            Ok(watchlist)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Get watchlist by ID
    pub async fn get_watchlist_by_id(&self, watchlist_id: &str) -> AlpacaResult<Watchlist> {
        let response = TradingApi::get_watchlist_by_id_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            watchlist_id,
        )
        .await?;

        if response.status().is_success() {
            let watchlist: Watchlist = response.json().await?;
            Ok(watchlist)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Update watchlist by ID
    pub async fn update_watchlist_by_id(
        &self,
        watchlist_id: &str,
        request: UpdateWatchlistRequest,
    ) -> AlpacaResult<Watchlist> {
        let response = TradingApi::update_watchlist_by_id_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            watchlist_id,
            &request,
        )
        .await?;

        if response.status().is_success() {
            let watchlist: Watchlist = response.json().await?;
            Ok(watchlist)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Delete watchlist by ID
    pub async fn delete_watchlist_by_id(&self, watchlist_id: &str) -> AlpacaResult<()> {
        let response = TradingApi::delete_watchlist_by_id_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            watchlist_id,
        )
        .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Add asset to watchlist by ID
    pub async fn add_asset_to_watchlist(
        &self,
        watchlist_id: &str,
        symbol: &str,
    ) -> AlpacaResult<Watchlist> {
        let request = AddAssetRequest {
            symbol: symbol.to_string(),
        };
        let response = TradingApi::add_asset_to_watchlist_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            watchlist_id,
            &request,
        )
        .await?;

        if response.status().is_success() {
            let watchlist: Watchlist = response.json().await?;
            Ok(watchlist)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Remove asset from watchlist by ID
    pub async fn remove_asset_from_watchlist(
        &self,
        watchlist_id: &str,
        symbol: &str,
    ) -> AlpacaResult<Watchlist> {
        let response = TradingApi::remove_asset_from_watchlist_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            watchlist_id,
            symbol,
        )
        .await?;

        if response.status().is_success() {
            let watchlist: Watchlist = response.json().await?;
            Ok(watchlist)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Get watchlist by name
    pub async fn get_watchlist_by_name(&self, name: &str) -> AlpacaResult<Watchlist> {
        let response = TradingApi::get_watchlist_by_name_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            name,
        )
        .await?;

        if response.status().is_success() {
            let watchlist: Watchlist = response.json().await?;
            Ok(watchlist)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Update watchlist by name
    pub async fn update_watchlist_by_name(
        &self,
        name: &str,
        request: UpdateWatchlistRequest,
    ) -> AlpacaResult<Watchlist> {
        let response = TradingApi::update_watchlist_by_name_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            name,
            &request,
        )
        .await?;

        if response.status().is_success() {
            let watchlist: Watchlist = response.json().await?;
            Ok(watchlist)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Add asset to watchlist by name
    pub async fn add_asset_to_watchlist_by_name(
        &self,
        name: &str,
        symbol: &str,
    ) -> AlpacaResult<Watchlist> {
        let request = AddAssetRequest {
            symbol: symbol.to_string(),
        };
        let response = TradingApi::add_asset_to_watchlist_by_name_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            name,
            &request,
        )
        .await?;

        if response.status().is_success() {
            let watchlist: Watchlist = response.json().await?;
            Ok(watchlist)
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }

    /// Delete watchlist by name
    pub async fn delete_watchlist_by_name(&self, name: &str) -> AlpacaResult<()> {
        let response = TradingApi::delete_watchlist_by_name_raw(
            &self.client,
            &self.config.base_url,
            self.config.get_headers(),
            name,
        )
        .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status_code = response.status().as_u16() as u32;
            let error_text = response.text().await?;
            Err(AlpacaError::ApiError {
                code: Some(status_code),
                message: error_text,
            })
        }
    }
}
