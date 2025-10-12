// Feature-gated client modules
#[cfg(feature = "trading")]
pub mod trading;
#[cfg(feature = "trading")]
pub use trading::*;

#[cfg(feature = "market_data")]
pub mod market_data;
#[cfg(feature = "market_data")]
pub use market_data::*;

#[cfg(feature = "websocket")]
pub mod trading_stream;
#[cfg(feature = "websocket")]
pub use trading_stream::*;

#[cfg(feature = "websocket")]
pub mod market_data_stream;
#[cfg(feature = "websocket")]
pub use market_data_stream::*;
