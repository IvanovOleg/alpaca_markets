use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Bar {
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "o")]
    pub open: f64,
    #[serde(rename = "h")]
    pub high: f64,
    #[serde(rename = "l")]
    pub low: f64,
    #[serde(rename = "c")]
    pub close: f64,
    #[serde(rename = "v")]
    pub volume: u64,
    #[serde(rename = "n")]
    pub trade_count: Option<u64>,
    #[serde(rename = "vw")]
    pub vwap: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BarsResponse {
    pub bars: Vec<Bar>,
    pub symbol: String,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BarRequest {
    pub symbols: Vec<String>,
    pub timeframe: String,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub limit: Option<u32>,
    pub page_token: Option<String>,
}

/// Sort order for market data queries
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Sort {
    /// Ascending order (oldest first)
    Asc,
    /// Descending order (newest first)
    Desc,
}

impl Sort {
    /// Convert to string slice
    pub fn as_str(&self) -> &'static str {
        match self {
            Sort::Asc => "asc",
            Sort::Desc => "desc",
        }
    }
}

impl Default for Sort {
    fn default() -> Self {
        Sort::Asc
    }
}

impl fmt::Display for Sort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<Sort> for &'static str {
    fn from(sort: Sort) -> Self {
        sort.as_str()
    }
}

/// Adjustment type for historical bar data
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Adjustment {
    /// No adjustments applied
    Raw,
    /// Split adjusted only
    Split,
    /// Dividend adjusted only
    Dividend,
    /// Fully adjusted (both split and dividend)
    All,
}

impl Adjustment {
    /// Convert to string slice
    pub fn as_str(&self) -> &'static str {
        match self {
            Adjustment::Raw => "raw",
            Adjustment::Split => "split",
            Adjustment::Dividend => "dividend",
            Adjustment::All => "all",
        }
    }
}

impl Default for Adjustment {
    fn default() -> Self {
        Adjustment::Raw
    }
}

impl fmt::Display for Adjustment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<Adjustment> for &'static str {
    fn from(adjustment: Adjustment) -> Self {
        adjustment.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_default() {
        assert_eq!(Sort::default(), Sort::Asc);
    }

    #[test]
    fn test_sort_as_str() {
        assert_eq!(Sort::Asc.as_str(), "asc");
        assert_eq!(Sort::Desc.as_str(), "desc");
    }

    #[test]
    fn test_sort_display() {
        assert_eq!(format!("{}", Sort::Asc), "asc");
        assert_eq!(format!("{}", Sort::Desc), "desc");
    }

    #[test]
    fn test_adjustment_default() {
        assert_eq!(Adjustment::default(), Adjustment::Raw);
    }

    #[test]
    fn test_adjustment_as_str() {
        assert_eq!(Adjustment::Raw.as_str(), "raw");
        assert_eq!(Adjustment::Split.as_str(), "split");
        assert_eq!(Adjustment::Dividend.as_str(), "dividend");
        assert_eq!(Adjustment::All.as_str(), "all");
    }

    #[test]
    fn test_adjustment_display() {
        assert_eq!(format!("{}", Adjustment::Raw), "raw");
        assert_eq!(format!("{}", Adjustment::Split), "split");
        assert_eq!(format!("{}", Adjustment::Dividend), "dividend");
        assert_eq!(format!("{}", Adjustment::All), "all");
    }
}
