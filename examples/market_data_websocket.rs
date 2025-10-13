// Market Data WebSocket Example
//
// This example demonstrates the convenient .run() method which provides:
// - Automatic reconnection on connection loss
// - Graceful shutdown handling (Ctrl+C)
// - Built-in error recovery
// - Simplified message handling
//
// For advanced usage with custom loops, see market_data_websocket_advanced.rs
//
// To run this example:
// 1. Set your environment variables:
//    - APCA_API_KEY_ID=your_api_key
//    - APCA_API_SECRET_KEY=your_secret_key
// 2. Run: cargo run --example market_data_websocket --features "websocket"

use alpaca_markets::AlpacaConfig;

#[cfg(feature = "websocket")]
use alpaca_markets::clients::market_data_stream::{Feed, MarketDataStreamClient};

#[cfg(feature = "websocket")]
use alpaca_markets::wss::{common::RunOptions, market_data::MarketDataMessage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "🚀 Alpaca Market Data WebSocket Example
========================================"
    );

    #[cfg(feature = "websocket")]
    {
        // Create configuration
        let config = match AlpacaConfig::from_env() {
            Ok(config) => {
                println!("✅ Configuration loaded from environment variables");
                config
            }
            Err(_) => {
                println!(
                    "⚠️  Environment variables not found. Using demo configuration.
   To use real data, set APCA_API_KEY_ID and APCA_API_SECRET_KEY"
                );

                AlpacaConfig::new(
                    "DEMO_KEY".to_string(),
                    "DEMO_SECRET".to_string(),
                    true, // Use paper trading endpoint
                )
            }
        };

        // Create market data stream client
        let mut client = MarketDataStreamClient::new(config, Feed::Iex);

        println!("🔌 Connecting to Alpaca Market Data WebSocket (IEX feed)...");

        match client.connect().await {
            Ok(_) => {
                println!("✅ Connected successfully!");

                // Subscribe to some popular stocks
                println!("📈 Subscribing to AAPL, MSFT, TSLA...");
                client
                    .subscribe(
                        Some(&["AAPL", "MSFT", "TSLA"]), // trades
                        Some(&["AAPL", "MSFT"]),         // quotes
                        Some(&["AAPL"]),                 // bars
                    )
                    .await?;

                println!("👂 Listening for market data with enhanced API...\n");
                show_api_comparison();
            }
            Err(e) => {
                eprintln!("❌ Connection failed: {}", e);
                println!("💡 This is expected with demo credentials.");
                println!("   Set real API credentials to connect to Alpaca.");
                return Ok(());
            }
        }

        // DEMONSTRATION: Simple .run() method (recommended for most users)
        println!("🌟 Using simple .run() method (recommended):");

        let options = RunOptions {
            verbose: true,
            timeout_secs: 30, // Demo timeout
            ..Default::default()
        };

        // This single call handles everything:
        // - Message processing loop
        // - Automatic reconnection
        // - Ctrl+C graceful shutdown
        // - Error recovery
        client
            .run_with_options(
                |message| async move {
                    handle_market_data_message(message);
                    Ok(())
                },
                options,
            )
            .await?;
    }

    #[cfg(not(feature = "websocket"))]
    {
        println!("❌ This example requires the 'websocket' feature.");
        println!("   Run with: cargo run --example market_data_enhanced --features \"websocket\"");
    }

    Ok(())
}

#[cfg(feature = "websocket")]
fn handle_market_data_message(message: MarketDataMessage) {
    match message {
        MarketDataMessage::Trade(trade) => {
            println!(
                "📈 Trade: {} @ ${} (size: {})",
                trade.symbol, trade.price, trade.size
            );
        }
        MarketDataMessage::Quote(quote) => {
            println!(
                "💰 Quote: {} ${:.2}/{:.2} (spread: ${:.2})",
                quote.symbol,
                quote.bid_price,
                quote.ask_price,
                quote.ask_price - quote.bid_price
            );
        }
        MarketDataMessage::Bar(bar) => {
            println!(
                "📊 Bar: {} OHLC ${:.2}/{:.2}/{:.2}/{:.2} (vol: {})",
                bar.symbol, bar.open, bar.high, bar.low, bar.close, bar.volume
            );
        }
        MarketDataMessage::Subscription(sub) => {
            println!("📋 Subscription: {:?}", sub);
        }
        MarketDataMessage::Error(error) => {
            println!("❌ Error: {:?}", error);
        }
    }
}

fn show_api_comparison() {
    println!(
        "
📚 API Pattern Comparison:

🌟 NEW: Enhanced .run() method (this example):
┌─────────────────────────────────────────────────────────────┐
│ client.run(|message| async move {{                          │
│     handle_message(message).await;                         │
│     Ok(())                                                 │
│ }}).await?;                                                │
└─────────────────────────────────────────────────────────────┘

⚙️  ADVANCED: Manual loop (for power users):
┌─────────────────────────────────────────────────────────────┐
│ loop {{                                                     │
│     tokio::select! {{                                       │
│         msg = client.next_message() => {{ /* handle */ }}   │
│         _ = tokio::signal::ctrl_c() => {{ break; }}         │
│     }}                                                      │
│ }}                                                          │
└─────────────────────────────────────────────────────────────┘

✨ The .run() method provides:
   • Automatic reconnection with exponential backoff
   • Graceful shutdown on Ctrl+C
   • Configurable error handling and timeouts
   • Verbose logging for debugging
   • Zero boilerplate - just focus on your message handling!
"
    );
}
