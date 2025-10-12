use serde::{Deserialize, Serialize};

/// Logo information for a symbol
#[derive(Debug, Deserialize, Serialize)]
pub struct Logo {
    /// The symbol/ticker
    pub symbol: String,
    /// URL to the logo image
    pub url: String,
}

/// Response containing logo information
#[derive(Debug, Deserialize, Serialize)]
pub struct LogoResponse {
    /// Logo data for the requested symbol
    pub logo: Logo,
}
