use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// WebSocket message types for trading updates
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TradingWebSocketMessage {
    // Control messages with "stream" field (subscription confirmations, etc.)
    Listening(ListeningMessage),

    // Actual trading updates with "T" field
    TradeUpdate(TradeUpdate),
    AccountUpdate(AccountUpdate),

    // Authorization responses
    Authorization(AuthMessage),

    // Connection status messages
    Connected(ConnectedMessage),

    // Error messages
    Error(ErrorMessage),

    // Raw JSON for unknown messages (fallback)
    Unknown(serde_json::Value),
}

/// Listening message for subscription confirmations
#[derive(Debug, Deserialize, Serialize)]
pub struct ListeningMessage {
    pub stream: String, // "listening"
    pub data: ListeningData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConnectedMessage {
    #[serde(rename = "T")]
    pub message_type: String, // "success"
    pub msg: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TradeUpdate {
    #[serde(rename = "T")]
    pub message_type: String, // "trade_updates"
    pub event: TradeEventType,
    pub timestamp: DateTime<Utc>,
    pub order: TradeUpdateOrder,
    pub execution_id: Option<String>,
    pub position_qty: Option<String>,
    pub price: Option<String>,
    pub qty: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TradeEventType {
    New,
    Fill,
    #[serde(rename = "partial_fill")]
    PartialFill,
    Canceled,
    Expired,
    #[serde(rename = "done_for_day")]
    DoneForDay,
    Replaced,
    Rejected,
    #[serde(rename = "pending_new")]
    PendingNew,
    Stopped,
    #[serde(rename = "pending_cancel")]
    PendingCancel,
    #[serde(rename = "pending_replace")]
    PendingReplace,
    Calculated,
    Suspended,
    #[serde(rename = "order_replace_rejected")]
    OrderReplaceRejected,
    #[serde(rename = "order_cancel_rejected")]
    OrderCancelRejected,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TradeUpdateOrder {
    pub id: String,
    pub client_order_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub submitted_at: DateTime<Utc>,
    pub filled_at: Option<DateTime<Utc>>,
    pub expired_at: Option<DateTime<Utc>>,
    pub canceled_at: Option<DateTime<Utc>>,
    pub failed_at: Option<DateTime<Utc>>,
    pub replaced_at: Option<DateTime<Utc>>,
    pub replaced_by: Option<String>,
    pub replaces: Option<String>,
    pub asset_id: String,
    pub symbol: String,
    pub asset_class: String,
    pub notional: Option<String>,
    pub qty: Option<String>,
    pub filled_qty: String,
    pub filled_avg_price: Option<String>,
    pub order_class: String,
    pub order_type: String,
    pub side: String,
    pub time_in_force: String,
    pub limit_price: Option<String>,
    pub stop_price: Option<String>,
    pub status: String,
    pub extended_hours: bool,
    pub legs: Option<Vec<OrderLeg>>,
    pub trail_percent: Option<String>,
    pub trail_price: Option<String>,
    pub hwm: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderLeg {
    pub id: String,
    pub symbol: String,
    pub side: String,
    pub qty: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountUpdate {
    #[serde(rename = "T")]
    pub message_type: String, // "account_updates"
    pub event: String,
    pub timestamp: DateTime<Utc>,
    pub buying_power: String,
    pub total_portfolio_value: String,
    pub cash: String,
    pub cash_withdrawable: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListeningData {
    pub streams: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthMessage {
    pub action: String,
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorMessage {
    pub code: u32,
    pub msg: String,
}

/// WebSocket subscription requests
#[derive(Debug, Serialize)]
pub struct AuthRequest {
    pub action: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct TradingSubscribeRequest {
    pub action: String, // "listen"
    pub data: TradingSubscribeData,
}

#[derive(Debug, Serialize)]
pub struct TradingSubscribeData {
    pub streams: Vec<String>, // ["trade_updates", "account_updates"]
}
