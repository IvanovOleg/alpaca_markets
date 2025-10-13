use crate::models::AlpacaError;

/// Configuration for Alpaca API clients
#[derive(Clone)]
pub struct AlpacaConfig {
    pub api_key: String,
    pub secret_key: String,
    pub is_paper: bool,
    pub base_url: String,
    pub data_url: String,
    pub ws_url: String,
    pub default_feed: Option<String>,
}

impl AlpacaConfig {
    /// Create a new configuration
    pub fn new(api_key: String, secret_key: String, is_paper: bool) -> Self {
        let (base_url, data_url, ws_url) = if is_paper {
            (
                "https://paper-api.alpaca.markets".to_string(),
                "https://data.alpaca.markets".to_string(),
                "wss://stream.data.alpaca.markets".to_string(),
            )
        } else {
            (
                "https://api.alpaca.markets".to_string(),
                "https://data.alpaca.markets".to_string(),
                "wss://stream.data.alpaca.markets".to_string(),
            )
        };

        Self {
            api_key,
            secret_key,
            is_paper,
            base_url,
            data_url,
            ws_url,
            default_feed: None, // Default to None, will use API default (SIP)
        }
    }

    /// Create configuration from environment variables
    pub fn from_env() -> Result<Self, AlpacaError> {
        dotenv::dotenv().ok();

        let api_key = std::env::var("APCA_API_KEY_ID")
            .map_err(|_| AlpacaError::ConfigError("APCA_API_KEY_ID not found".to_string()))?;

        let secret_key = std::env::var("APCA_API_SECRET_KEY")
            .map_err(|_| AlpacaError::ConfigError("APCA_API_SECRET_KEY not found".to_string()))?;

        let is_paper = std::env::var("ALPACA_PAPER")
            .unwrap_or_else(|_| "true".to_string())
            .parse()
            .unwrap_or(true);

        let feed = std::env::var("ALPACA_FEED").ok();

        let mut config = Self::new(api_key, secret_key, is_paper);
        config.default_feed = feed;
        Ok(config)
    }

    /// Set the default data feed (e.g., "iex" for free tier, "sip" for paid)
    pub fn with_feed(mut self, feed: impl Into<String>) -> Self {
        self.default_feed = Some(feed.into());
        self
    }

    /// Use IEX feed (free tier)
    pub fn with_iex_feed(mut self) -> Self {
        self.default_feed = Some("iex".to_string());
        self
    }

    /// Use SIP feed (paid subscription required)
    pub fn with_sip_feed(mut self) -> Self {
        self.default_feed = Some("sip".to_string());
        self
    }

    /// Get headers for API requests
    pub fn get_headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("APCA-API-KEY-ID", self.api_key.parse().unwrap());
        headers.insert("APCA-API-SECRET-KEY", self.secret_key.parse().unwrap());
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers
    }
}

impl std::fmt::Debug for AlpacaConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AlpacaConfig")
            .field("api_key", &"***REDACTED***")
            .field("secret_key", &"***REDACTED***")
            .field("is_paper", &self.is_paper)
            .field("base_url", &self.base_url)
            .field("data_url", &self.data_url)
            .field("ws_url", &self.ws_url)
            .field("default_feed", &self.default_feed)
            .finish()
    }
}
