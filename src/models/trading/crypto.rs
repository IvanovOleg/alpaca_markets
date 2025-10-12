use serde::{Deserialize, Serialize};

/// Crypto funding wallet information
#[derive(Debug, Deserialize, Serialize)]
pub struct CryptoWallet {
    /// Unique wallet ID
    pub id: String,
    /// Account ID associated with this wallet
    pub account_id: String,
    /// Asset symbol (e.g., "BTC", "ETH")
    pub asset: String,
    /// Wallet address for deposits
    pub address: String,
    /// Current wallet balance
    pub balance: String,
    /// Available balance for trading/withdrawal
    pub available_balance: String,
    /// Wallet creation timestamp
    pub created_at: String,
    /// Last update timestamp
    pub updated_at: String,
}

/// Crypto funding transfer information
#[derive(Debug, Deserialize, Serialize)]
pub struct CryptoTransfer {
    /// Unique transfer ID
    pub id: String,
    /// Account ID
    pub account_id: String,
    /// Asset symbol
    pub asset: String,
    /// Transfer amount
    pub amount: String,
    /// Transfer direction ("incoming" or "outgoing")
    pub direction: String,
    /// Transfer status (e.g., "pending", "completed", "rejected")
    pub status: String,
    /// Source/destination address
    pub address: String,
    /// Transaction hash (if available)
    pub txn_hash: Option<String>,
    /// Transfer fee
    pub fee: Option<String>,
    /// Transfer creation timestamp
    pub created_at: String,
    /// Last update timestamp
    pub updated_at: String,
}

/// Request to create a crypto transfer (withdrawal)
#[derive(Debug, Serialize)]
pub struct CreateCryptoTransferRequest {
    /// Asset symbol to transfer
    pub asset: String,
    /// Amount to transfer
    pub amount: String,
    /// Destination address (must be whitelisted)
    pub address: String,
}

/// Whitelisted address information
#[derive(Debug, Deserialize, Serialize)]
pub struct WhitelistedAddress {
    /// Address ID
    pub id: String,
    /// Account ID
    pub account_id: String,
    /// Asset symbol this address is for
    pub asset: String,
    /// The whitelisted address
    pub address: String,
    /// Address label/name
    pub label: Option<String>,
    /// Address status
    pub status: String,
    /// Creation timestamp
    pub created_at: String,
    /// Activation timestamp (24hrs after creation)
    pub activated_at: Option<String>,
}

/// Request to create a whitelisted address
#[derive(Debug, Serialize)]
pub struct CreateWhitelistedAddressRequest {
    /// Asset symbol
    pub asset: String,
    /// Address to whitelist
    pub address: String,
    /// Optional label for the address
    pub label: Option<String>,
}

/// Crypto transfer fee estimate
#[derive(Debug, Deserialize, Serialize)]
pub struct CryptoTransferEstimate {
    /// Asset symbol
    pub asset: String,
    /// Transfer amount
    pub amount: String,
    /// Estimated fee
    pub fee: String,
    /// Estimated total (amount + fee)
    pub total: String,
}
