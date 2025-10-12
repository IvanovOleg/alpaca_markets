use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    pub id: String,
    pub account_number: String,
    pub status: AccountStatus,
    pub crypto_status: Option<AccountStatus>,
    pub currency: String,
    pub buying_power: String,
    pub regt_buying_power: String,
    pub daytrading_buying_power: String,
    pub non_marginable_buying_power: String,
    pub cash: String,
    pub accrued_fees: String,
    pub pending_transfer_out: Option<String>,
    pub pending_transfer_in: Option<String>,
    pub portfolio_value: String,
    pub pattern_day_trader: bool,
    pub trading_blocked: bool,
    pub transfers_blocked: bool,
    pub account_blocked: bool,
    pub created_at: DateTime<Utc>,
    pub trade_suspended_by_user: bool,
    pub multiplier: String,
    pub shorting_enabled: bool,
    pub equity: String,
    pub last_equity: String,
    pub long_market_value: String,
    pub short_market_value: String,
    pub initial_margin: String,
    pub maintenance_margin: String,
    pub last_maintenance_margin: String,
    pub sma: String,
    pub daytrade_count: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AccountStatus {
    Onboarding,
    SubmissionFailed,
    Submitted,
    AccountUpdated,
    ApprovalPending,
    Active,
    Rejected,
}

/// Account configuration settings
#[derive(Debug, Deserialize, Serialize)]
pub struct AccountConfiguration {
    /// Day Trade Buying Power Check
    pub dtbp_check: DtbpCheck,
    /// Trade Confirmation Email
    pub trade_confirm_email: TradeConfirmEmail,
    /// Suspend Trade
    pub suspend_trade: bool,
    /// Maximum Margin Multiplier
    pub max_margin_multiplier: String,
    /// Pattern Day Trader Check
    pub pdt_check: PdtCheck,
}

/// Request model for updating account configuration
#[derive(Debug, Serialize)]
pub struct UpdateAccountConfigurationRequest {
    /// Day Trade Buying Power Check
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dtbp_check: Option<DtbpCheck>,
    /// Trade Confirmation Email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_confirm_email: Option<TradeConfirmEmail>,
    /// Suspend Trade
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspend_trade: Option<bool>,
    /// Maximum Margin Multiplier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_margin_multiplier: Option<String>,
    /// Pattern Day Trader Check
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdt_check: Option<PdtCheck>,
}

/// Day Trade Buying Power Check setting
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DtbpCheck {
    /// No check
    None,
    /// Entry check only
    Entry,
    /// Entry and exit check
    Both,
}

/// Trade Confirmation Email setting
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TradeConfirmEmail {
    /// No email
    None,
    /// All trades
    All,
}

/// Pattern Day Trader Check setting
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PdtCheck {
    /// No check
    None,
    /// Entry check only
    Entry,
    /// Entry and exit check
    Both,
}

/// Account activity entry
#[derive(Debug, Deserialize, Serialize)]
pub struct AccountActivity {
    /// Activity ID
    pub id: String,
    /// Activity type
    pub activity_type: ActivityType,
    /// Account ID
    pub account_id: String,
    /// Symbol (for trade activities)
    pub symbol: Option<String>,
    /// Transaction time
    pub transaction_time: DateTime<Utc>,
    /// Activity description
    pub description: Option<String>,
    /// Status of the activity
    pub status: ActivityStatus,
    /// Quantity (for trade activities)
    pub qty: Option<String>,
    /// Price (for trade activities)
    pub price: Option<String>,
    /// Net amount
    pub net_amount: Option<String>,
    /// Per share amount
    pub per_share_amount: Option<String>,
    /// Order ID (for trade activities)
    pub order_id: Option<String>,
    /// Side (buy/sell for trade activities)
    pub side: Option<String>,
}

/// Account activity type
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActivityType {
    /// Stock or option trade fill
    Fill,
    /// Partial fill for a multi-leg order
    PartialFill,
    /// Transaction fees
    Fee,
    /// Dividend payment
    Div,
    /// Dividend adjustment
    Divnra,
    /// Interest payment
    Int,
    /// Interest adjustment
    Intnra,
    /// Journal entry
    Jnl,
    /// Journal entry adjustment
    Jnlnra,
    /// Regulatory fees
    RegFee,
    /// Deposit/withdrawal
    Acatc,
    /// Position transfer
    Acats,
    /// Cash disbursement
    CashDisbursement,
    /// Cash receipt
    CashReceipt,
    /// Stock dividend
    StockDividend,
    /// Stock split
    StockSplit,
    /// Symbol change
    SymbolChange,
    /// Spin off
    SpinOff,
    /// Corporate action
    CorporateAction,
    /// Position adjustment
    PositionAdjustment,
    /// Transfer
    Transfer,
}

/// Activity status
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ActivityStatus {
    /// Executed activity
    Executed,
    /// Pending activity
    Pending,
    /// Canceled activity
    Canceled,
}
