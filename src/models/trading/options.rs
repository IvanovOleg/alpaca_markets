use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct OptionContract {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub status: String,
    pub tradable: bool,
    pub expiration_date: NaiveDate,
    pub root_symbol: String,
    pub underlying_symbol: String,
    pub underlying_asset_id: String,
    pub r#type: OptionType,
    pub style: OptionStyle,
    pub strike_price: String,
    pub multiplier: String,
    pub size: String,
    pub open_interest: Option<String>,
    pub open_interest_date: Option<NaiveDate>,
    pub close_price: Option<String>,
    pub close_price_date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OptionType {
    Call,
    Put,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OptionStyle {
    American,
    European,
}