use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Order {
    pub id: String,
    pub client_order_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub submitted_at: Option<DateTime<Utc>>,
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
    pub order_type: OrderType,
    pub side: OrderSide,
    pub time_in_force: OrderTimeInForce,
    pub limit_price: Option<String>,
    pub stop_price: Option<String>,
    pub status: OrderStatus,
    pub extended_hours: bool,
    pub legs: Option<Vec<Order>>,
    pub trail_percent: Option<String>,
    pub trail_price: Option<String>,
    pub hwm: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderRequest {
    pub symbol: String,
    pub qty: Option<String>,
    pub notional: Option<String>,
    pub side: OrderSide,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub time_in_force: OrderTimeInForce,
    pub limit_price: Option<String>,
    pub stop_price: Option<String>,
    pub extended_hours: Option<bool>,
    pub client_order_id: Option<String>,
    pub order_class: Option<String>,
    pub take_profit: Option<TakeProfitRequest>,
    pub stop_loss: Option<StopLossRequest>,
    pub trail_price: Option<String>,
    pub trail_percent: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    Market,
    Limit,
    Stop,
    #[serde(rename = "stop_limit")]
    StopLimit,
    #[serde(rename = "trailing_stop")]
    TrailingStop,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderTimeInForce {
    Day,
    Gtc,
    Opg,
    Cls,
    Ioc,
    Fok,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    New,
    #[serde(rename = "partially_filled")]
    PartiallyFilled,
    Filled,
    #[serde(rename = "done_for_day")]
    DoneForDay,
    Canceled,
    Expired,
    Replaced,
    #[serde(rename = "pending_cancel")]
    PendingCancel,
    #[serde(rename = "pending_replace")]
    PendingReplace,
    Accepted,
    #[serde(rename = "pending_new")]
    PendingNew,
    #[serde(rename = "accepted_for_bidding")]
    AcceptedForBidding,
    Stopped,
    Rejected,
    Suspended,
    Calculated,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TakeProfitRequest {
    pub limit_price: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StopLossRequest {
    pub stop_price: String,
    pub limit_price: Option<String>,
}
