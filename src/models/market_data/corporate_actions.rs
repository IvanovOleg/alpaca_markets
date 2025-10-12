use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Corporate Action Entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorporateAction {
    /// Corporate action ID
    pub id: String,
    /// Corporate action type
    pub ca_type: String,
    /// Corporate action subtype (if applicable)
    pub ca_sub_type: Option<String>,
    /// Symbol affected by the corporate action
    pub symbol: String,
    /// Announcement date of the corporate action
    pub announcement_date: Option<DateTime<Utc>>,
    /// Ex-date of the corporate action
    pub ex_date: Option<DateTime<Utc>>,
    /// Record date of the corporate action
    pub record_date: Option<DateTime<Utc>>,
    /// Payable/payment date of the corporate action
    pub payable_date: Option<DateTime<Utc>>,
    /// Effective date of the corporate action
    pub effective_date: Option<DateTime<Utc>>,
    /// Rate of the corporate action (e.g., dividend rate, split ratio)
    pub rate: Option<f64>,
    /// Old rate (for actions involving rate changes)
    pub old_rate: Option<f64>,
    /// New rate (for actions involving rate changes)
    pub new_rate: Option<f64>,
    /// Currency of the corporate action (for dividends, etc.)
    pub currency: Option<String>,
    /// Description of the corporate action
    pub description: Option<String>,
    /// Additional details or notes
    pub details: Option<String>,
}

/// Corporate Action Type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum CorporateActionType {
    /// Dividend payment
    Dividend,
    /// Stock split
    Split,
    /// Stock merger
    Merger,
    /// Spin-off
    Spinoff,
    /// Symbol change
    SymbolChange,
    /// Rights issue
    Rights,
    /// Bonus issue
    Bonus,
    /// Liquidation
    Liquidation,
    /// Delisting
    Delisting,
    /// Other corporate action type
    Other,
}

/// Response for corporate actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorporateActionsResponse {
    /// List of corporate actions
    pub corporate_actions: Vec<CorporateAction>,
    /// Next page token for pagination
    pub next_page_token: Option<String>,
}
