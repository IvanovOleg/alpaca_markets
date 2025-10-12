use serde::{Deserialize, Serialize};

/// Condition code information for quotes and trades
#[derive(Debug, Deserialize, Serialize)]
pub struct ConditionCode {
    /// Condition code identifier
    pub code: String,
    /// Human-readable description of the condition
    pub description: String,
}

/// Exchange code information
#[derive(Debug, Deserialize, Serialize)]
pub struct ExchangeCode {
    /// Exchange code identifier
    pub code: String,
    /// Exchange name
    pub name: String,
    /// Exchange type (e.g., "stock")
    pub r#type: String,
}

/// Response containing condition codes
#[derive(Debug, Deserialize, Serialize)]
pub struct ConditionsResponse {
    /// Map of condition codes
    pub conditions: std::collections::HashMap<String, ConditionCode>,
}

/// Response containing exchange codes
#[derive(Debug, Deserialize, Serialize)]
pub struct ExchangesResponse {
    /// Map of exchange codes
    pub exchanges: std::collections::HashMap<String, ExchangeCode>,
}
