// Market data models
pub mod auctions;
pub mod bars;
pub mod corporate_actions;
pub mod crypto;
pub mod fixed_income;
pub mod forex;
pub mod logos;
pub mod meta;
pub mod news;
pub mod options;
pub mod quotes;
pub mod screener;
pub mod snapshots;
pub mod trades;

// Re-export all public types for convenience
pub use auctions::*;
pub use bars::*;
pub use corporate_actions::*;
pub use logos::*;
pub use meta::*;
pub use news::*;
pub use quotes::*;
pub use screener::*;
pub use snapshots::*;
pub use trades::*;

// Specialized models are available via their respective modules to avoid naming conflicts
// Use market_data::options::* to access options market data models
// Use market_data::crypto::* to access crypto market data models
// Use market_data::fixed_income::* to access fixed income market data models
// Use market_data::forex::* to access forex market data models
// Use market_data::screener::* to access screener market data models
// Use market_data::news::* to access news market data models
// Use market_data::corporate_actions::* to access corporate actions market data models
