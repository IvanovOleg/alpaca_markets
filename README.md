# Alpaca Markets API Client

A comprehensive Rust client library for the Alpaca Markets API with **90+ production-ready endpoints** across all major asset classes and trading operations.

## Features

- **Complete Trading API** - 50+ endpoints across 9 domains (orders, positions, portfolio, account, assets, watchlists, options, calendar, crypto)
- **Comprehensive Market Data API** - 42+ endpoints across 9 asset classes (stocks, options, crypto, forex, fixed income, news, corporate actions, screener, logos)  
- **WebSocket Streaming** - Real-time data feeds for quotes, trades, and bars
- **Modular Architecture** - Feature-based compilation and domain-driven submodules
- **Async/Await** - Full async support using tokio
- **Type Safety** - Strongly typed responses with comprehensive error handling and serde serialization

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
alpaca_markets = { version = "0.1.0", features = ["market_data"] }
```

### Available Features

- `trading` - Trading API client (default)
- `market_data` - Market data API client (default)  
- `websocket` - WebSocket streaming support
- `broker` - Broker API functionality
- `options` - Options trading support

### Convenience Features

- `rest_api` - Enables both trading and market_data
- `real_time` - Enables market_data and websocket
- `full` - Enables all features

## Quick Start

### Environment Setup

Create a `.env` file with your Alpaca API credentials:

```env
APCA_API_KEY_ID=your_api_key_here
APCA_API_SECRET_KEY=your_secret_key_here
```

### Basic Market Data Example

```rust
use alpaca_markets::{AlpacaConfig, MarketDataClient, AlpacaResult};

#[tokio::main]
async fn main() -> AlpacaResult<()> {
    // Load configuration from environment variables
    let config = AlpacaConfig::from_env()?;
    
    // Create a market data client
    let client = MarketDataClient::new(config);
    
    // Get latest quote for AAPL
    let quote_response = client.get_latest_quote("AAPL").await?;
    let quote = &quote_response.quote;
    
    println!("AAPL: Bid: ${:.2} | Ask: ${:.2}", 
        quote.bid_price, 
        quote.ask_price
    );
    
    Ok(())
}
```

## Examples

### Trading Examples

```bash
# Basic account information
cargo run --example account_info --features "trading"

# Complete order management (submit, cancel, replace orders)
cargo run --example order_management --features "trading"

# Position management and portfolio operations
cargo run --example position_management --features "trading"

# Portfolio performance analytics
cargo run --example portfolio_history --features "trading"

# Watchlist CRUD operations
cargo run --example watchlist_management --features "trading"

# Account configuration and settings
cargo run --example account_configuration --features "trading"

# Account activities and history
cargo run --example account_activities --features "trading"

# Asset discovery and filtering
cargo run --example asset_management --features "trading"

# Crypto trading operations
cargo run --example crypto_funding --features "trading"
```

### Market Data Examples

```bash
# Comprehensive market data usage
cargo run --example market_data --features "market_data"

# Market calendar and trading schedule
cargo run --example market_calendar --features "trading"

# Market clock and current status
cargo run --example market_clock --features "trading"
```

### WebSocket Examples

```bash
# Real-time trading updates
cargo run --example trading_websocket --features "websocket"

# Real-time market data streaming
cargo run --example market_data_websocket --features "websocket"
```

```bash
cargo run --example account_configuration
```

Run the account activities example:

```bash
cargo run --example account_activities
```

These examples demonstrate:
- Latest quotes and trades
- Historical bars (requires paid subscription for recent data)
- Complete position management including options
- Portfolio history analysis with multiple timeframes
- Full watchlist CRUD operations and asset management
- Account configuration management and risk settings
- Concurrent API calls for performance
- Error handling and data parsing

## API Coverage

### Market Data API ✅ (42+ endpoints across 9 asset classes)
- **Stocks** (18 endpoints): Bars, quotes, trades, snapshots, auctions, conditions, exchanges, latest data
- **Options** (8 endpoints): Bars, trades, quotes, snapshots, chain, latest, exchanges, contracts
- **Crypto** (8 endpoints): Bars, trades, quotes, orderbooks, snapshots, latest, exchanges, historical
- **Fixed Income**: Bond and treasury snapshots
- **Forex**: Currency rates and latest rates
- **Screener**: Most active stocks, market movers (gainers/losers)
- **News**: Financial news articles with advanced filtering and content control
- **Corporate Actions**: Dividends, splits, mergers, spin-offs, and other corporate events
- **Logos**: Company symbol logos and branding
- Real-time WebSocket streaming (with `websocket` feature)

### Trading API ✅ (50+ endpoints across 9 domains)
- **Account Management**: Account information, configuration, activities, and risk settings
- **Order Management**: Complete order lifecycle (submit, cancel, replace, get by ID)
- **Position Management**: Individual and bulk position operations (get, close)
- **Portfolio Analytics**: Historical portfolio performance with multiple timeframes
- **Asset Discovery**: Stocks, crypto, and options contract discovery with extensive filtering
- **Watchlist Operations**: Full CRUD operations and asset management
- **Options Trading**: Exercise and do-not-exercise instructions
- **Market Calendar & Clock**: Trading schedule and market status
- **Crypto Trading**: Cryptocurrency trading operations

### WebSocket Streaming ✅
- **Trading Streams**: Real-time account and trade updates with JSON format
- **Market Data Streams**: Real-time quotes, trades, and bars with MessagePack format
- State-based connection management with automatic authentication
- Object-oriented API design with connection instance methods

## Project Structure

```
src/
├── lib.rs                 # Main library entry point
├── config.rs              # Configuration management (AlpacaConfig)
├── utils.rs               # Utility functions
├── models/                # Data models organized by domain
│   ├── common/           # Shared types and errors
│   ├── trading/          # Trading models (9 files)
│   └── market_data/      # Market data models (14 files)
├── clients/              # High-level API clients
│   ├── trading.rs        # Trading REST client
│   ├── market_data.rs    # Market data REST client
│   ├── trading_stream.rs # Trading WebSocket client
│   └── market_data_stream.rs # Market data WebSocket client
├── api/                  # Modular API endpoint definitions
│   ├── trading.rs        # Trading API coordinator
│   ├── trading/          # Trading submodules (9 domains)
│   ├── market_data.rs    # Market data API coordinator  
│   └── market_data/      # Market data submodules (9 asset classes)
└── wss/                  # WebSocket connection and message types
    ├── common.rs         # WebSocket connection management
    ├── trading.rs        # Trading WebSocket messages
    └── market_data.rs    # Market data WebSocket messages
```

## Configuration

The library supports both paper trading and live trading:

```rust
// Paper trading (default)
let config = AlpacaConfig::paper(api_key, secret_key);

// Live trading
let config = AlpacaConfig::live(api_key, secret_key);

// From environment variables (auto-detects paper vs live)
let config = AlpacaConfig::from_env()?;
```

## Error Handling

The library provides comprehensive error types with detailed information:

```rust
use alpaca_markets::AlpacaError;

match client.get_latest_quote("AAPL").await {
    Ok(response) => println!("Quote: {:?}", response.quote),
    Err(AlpacaError::ApiError { code, message }) => {
        eprintln!("API Error {}: {}", code.unwrap_or(0), message);
    }
    Err(AlpacaError::NetworkError(e)) => {
        eprintln!("Network Error: {}", e);
    }
    Err(AlpacaError::RateLimitError(e)) => {
        eprintln!("Rate Limit: {}", e);
    }
    Err(AlpacaError::WebSocketError(e)) => {
        eprintln!("WebSocket Error: {}", e);
    }
    Err(e) => eprintln!("Other Error: {:?}", e),
}
```

### Trading API Usage

The trading client provides complete order and position management:

```rust
use alpaca_markets::{
    models::{AlpacaResult, OrderRequest, OrderSide, OrderType, TimeInForce}, 
    clients::trading::TradingClient,
    AlpacaConfig
};

#[tokio::main]
async fn main() -> AlpacaResult<()> {
    let config = AlpacaConfig::from_env()?;
    let client = TradingClient::new(config);

    // Account information
    let account = client.get_account().await?;
    println!("Buying Power: ${}", account.buying_power);

    // Submit an order
    let order_request = OrderRequest {
        symbol: "AAPL".to_string(),
        qty: Some("1".to_string()),
        side: OrderSide::Buy,
        order_type: OrderType::Market,
        time_in_force: TimeInForce::Day,
        // ... other fields
    };
    let order = client.submit_order(order_request).await?;

    // Get order by ID
    let fetched_order = client.get_order(&order.id).await?;

    // Replace order
    let new_request = OrderRequest { /* modified order */ };
    let updated_order = client.replace_order(&order.id, new_request).await?;

    // Cancel order
    client.cancel_order(&order.id).await?;

    // Get all positions
    let positions = client.get_positions().await?;

    // Get specific position
    let aapl_position = client.get_position("AAPL").await?;

    // Close position (25% of holdings)
    let close_order = client.close_position("AAPL", None, Some("25")).await?;

    // Cancel all orders
    let canceled_orders = client.cancel_all_orders().await?;

    // Close all positions
    let close_orders = client.close_all_positions(Some(true)).await?;

    // Asset discovery
    let all_stocks = client.get_stocks().await?;
    let crypto_assets = client.get_crypto_assets().await?;
    let nasdaq_assets = client.get_assets(Some("active"), None, Some("NASDAQ")).await?;
    
    // Get specific asset details
    let aapl_asset = client.get_asset("AAPL").await?;
    println!("AAPL is marginable: {}", aapl_asset.marginable);
    println!("AAPL is shortable: {}", aapl_asset.shortable);
    println!("AAPL is fractionable: {}", aapl_asset.fractionable);

    Ok(())
}
```

#### Available Trading Methods

**Order Management:**
- `submit_order(request)` - Submit a new order
- `get_orders(status, limit)` - Get all orders with filtering
- `get_order(order_id)` - Get specific order by ID
- `get_order_by_client_id(client_id)` - Get order by client order ID
- `replace_order(order_id, new_request)` - Modify existing order
- `cancel_order(order_id)` - Cancel specific order
- `cancel_all_orders()` - Cancel all open orders

**Position Management:**
- `get_positions()` - Get all positions
- `get_position(symbol)` - Get specific position
- `close_position(symbol, qty, percentage)` - Close position (partial or full)
- `close_all_positions(cancel_orders)` - Liquidate entire portfolio

**Account Information:**
- `get_account()` - Get account details and buying power

**Asset Discovery:**
- `get_assets(status, asset_class, exchange)` - Get assets with filtering
- `get_asset(symbol_or_id)` - Get specific asset by symbol or ID

**Options Contracts:**
- `get_option_contracts(...)` - Get options contracts with extensive filtering
- `get_option_contract(symbol_or_id)` - Get specific option contract



## WebSocket Streaming

### Real-Time Trading Updates

The WebSocket implementation provides real-time streaming of trading updates and account changes:

```rust
use alpaca_markets::{AlpacaConfig, TradingStreamClient, TradingWebSocketMessage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AlpacaConfig::from_env()?;
    let mut client = TradingStreamClient::new(config);
    
    // Connect and authenticate
    client.connect().await?;
    
    println!("Connected! Listening for trading updates...");
    
    // Listen for messages
    while let Ok(Some(message)) = client.next_message().await {
        match message {
            TradingWebSocketMessage::TradeUpdate(trade) => {
                println!("Order {} is now {}", trade.order.id, trade.order.status);
            }
            TradingWebSocketMessage::AccountUpdate(account) => {
                println!("Buying power: ${}", account.buying_power);
            }
            _ => {} // Handle other message types
        }
    }
    
    Ok(())
}
```

**Run the example:**
```bash
cargo run --example trading_websocket --features "websocket"
```

### Real-Time Market Data

Stream real-time market data with MessagePack encoding for optimal performance:

```rust
use alpaca_markets::{AlpacaConfig, MarketDataStreamClient, Feed, MarketDataMessage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AlpacaConfig::from_env()?;
    let mut client = MarketDataStreamClient::new(config, Feed::Iex);
    
    // Connect and authenticate
    client.connect().await?;
    
    // Subscribe to data streams
    client.subscribe(
        Some(&["AAPL", "TSLA"]), // trades
        Some(&["AAPL"]),         // quotes  
        Some(&["AAPL"])          // bars
    ).await?;
    
    println!("Subscribed! Listening for market data...");
    
    // Process incoming messages
    while let Ok(Some(messages)) = client.next_message().await {
        for message in messages {
            match message {
                MarketDataMessage::Trade(trade) => {
                    println!("{}: ${} x {}", trade.symbol, trade.price, trade.size);
                }
                MarketDataMessage::Quote(quote) => {
                    println!("{}: ${}/{}", quote.symbol, quote.bid_price, quote.ask_price);
                }
                MarketDataMessage::Bar(bar) => {
                    println!("{}: OHLC ${}/{}/{}/{}", bar.symbol, bar.open, bar.high, bar.low, bar.close);
                }
                _ => {}
            }
        }
    }
    
    Ok(())
}
```

**Run the example:**
```bash
cargo run --example market_data_websocket --features "websocket"
```

### WebSocket Features

- **State-based Authentication**: Automatic authentication with connection management
- **Format Optimization**: JSON for trading streams, MessagePack for market data
- **Type Safety**: Strongly typed message handling with comprehensive error types
- **Multiple Feeds**: IEX (free), SIP (paid), Options, Crypto, and News feeds
- **Subscription Management**: Flexible subscribe/unsubscribe for multiple data types

### Message Format Details

**Trading Streams** use binary-encoded JSON:
- Alpaca sends JSON content in binary WebSocket frames
- Provides performance benefits while maintaining JSON compatibility
- All trading updates (orders, account changes) use this format

**Market Data Streams** use true MessagePack encoding:
- Full MessagePack binary serialization for maximum efficiency
- Significantly smaller message sizes for high-volume market data
- Automatic format negotiation via Content-Type headers

Both formats provide better performance than text-based JSON while maintaining compatibility with Alpaca's API requirements.

## Quick Reference

### Market Data Endpoints by Asset Class
- **Stocks**: 18 endpoints (bars, quotes, trades, snapshots, auctions, conditions, exchanges, latest)
- **Options**: 8 endpoints (bars, trades, quotes, snapshots, chain, latest, exchanges, contracts)
- **Crypto**: 8 endpoints (bars, trades, quotes, orderbooks, snapshots, latest, exchanges)
- **Fixed Income**: 1 endpoint (bonds/treasury snapshots)
- **Forex**: 2 endpoints (rates, latest rates)
- **News**: 1 endpoint (articles with filtering)
- **Corporate Actions**: 1 endpoint (dividends, splits, mergers)
- **Screener**: 2 endpoints (most active, movers)
- **Logos**: 1 endpoint (company logos)

### Trading Endpoints by Domain
- **Orders**: 7 endpoints (submit, get, cancel, replace operations)
- **Positions**: 4 endpoints (get, close operations)  
- **Account**: 4 endpoints (info, config, activities)
- **Assets**: 4 endpoints (discovery, filtering, contracts)
- **Watchlists**: 8 endpoints (CRUD operations)
- **Portfolio**: 1 endpoint (historical performance)
- **Options**: 2 endpoints (exercise instructions)
- **Calendar**: 1 endpoint (market schedule)

## Subscription Requirements

Some features require a paid Alpaca subscription:
- Recent historical data (last 15 minutes)
- Real-time data feeds
- Higher rate limits

The free tier includes:
- Latest quotes and trades
- Historical data older than 15 minutes
- Basic API access

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## License

This project is licensed under the MIT License.
