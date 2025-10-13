// Trading WebSocket Example
//
// This example demonstrates the convenient .run() method for trading streams which provides:
// - Automatic reconnection on connection loss
// - Graceful shutdown handling (Ctrl+C)
// - Built-in error recovery
// - Simplified message handling
//
// For advanced usage with custom loops, see trading_websocket_advanced.rs
//
// To run this example:
// 1. Set your environment variables (optional):
//    - APCA_API_KEY_ID=your_api_key
//    - APCA_API_SECRET_KEY=your_secret_key
// 2. Run: cargo run --example trading_websocket --features "websocket,trading"

use alpaca_markets::AlpacaConfig;

#[cfg(all(feature = "websocket", feature = "trading"))]
use alpaca_markets::clients::trading_stream::TradingStreamClient;

#[cfg(all(feature = "websocket", feature = "trading"))]
use alpaca_markets::wss::{common::RunOptions, trading::TradingWebSocketMessage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "🚀 Alpaca Trading WebSocket Example
===================================="
    );

    #[cfg(all(feature = "websocket", feature = "trading"))]
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
                    true, // Use paper trading
                )
            }
        };

        // Create trading stream client
        let mut client = TradingStreamClient::new(config);

        println!("🔌 Connecting to Alpaca Trading WebSocket...");

        match client.connect().await {
            Ok(_) => {
                println!("✅ Connected successfully!");
                println!("👂 Listening for trading updates with enhanced API...\n");
                show_api_features();
            }
            Err(e) => {
                eprintln!("❌ Connection failed: {}", e);
                println!("💡 This is expected with demo credentials.");
                println!("   Set real API credentials to connect to Alpaca.");
                return Ok(());
            }
        }

        // DEMONSTRATION: Enhanced .run() method with full configuration
        let options = RunOptions {
            auto_reconnect: true,
            max_reconnect_attempts: 3,
            reconnect_delay_ms: 2000,
            stop_on_handler_error: false,
            timeout_secs: 60, // Demo timeout
            verbose: true,
        };

        // This single call handles everything:
        client
            .run_with_options(
                |message| async move {
                    handle_trading_message(message);
                    Ok(())
                },
                options,
            )
            .await?;
    }

    #[cfg(not(all(feature = "websocket", feature = "trading")))]
    {
        println!("❌ This example requires both 'websocket' and 'trading' features.");
        println!(
            "   Run with: cargo run --example trading_enhanced --features \"websocket,trading\""
        );
    }

    Ok(())
}

#[cfg(all(feature = "websocket", feature = "trading"))]
fn handle_trading_message(message: TradingWebSocketMessage) {
    match message {
        TradingWebSocketMessage::StreamMessage(stream_msg) => match stream_msg.data {
            alpaca_markets::wss::trading::StreamData::TradeUpdate(trade_update) => {
                println!(
                    "🔄 Trade Update [{}]: Order {} ({}) is now {} - {} shares @ ${}",
                    trade_update.event,
                    trade_update.order.id,
                    trade_update.order.symbol,
                    trade_update.order.status,
                    trade_update.order.qty.as_deref().unwrap_or("N/A"),
                    trade_update.price.as_deref().unwrap_or("market")
                );
            }
            alpaca_markets::wss::trading::StreamData::AccountUpdate(account_update) => {
                println!(
                    "💰 Account Update: Buying Power: ${}, Cash: ${}",
                    account_update.buying_power, account_update.cash
                );
            }
            alpaca_markets::wss::trading::StreamData::Listening(listening) => {
                println!("� Subscribed to: {:?}", listening.streams);
            }
        },
        TradingWebSocketMessage::Connected(connected) => {
            println!("� Connection: {}", connected.msg);
        }
        TradingWebSocketMessage::Authorization(auth) => {
            println!("� Auth: {} -> {}", auth.action, auth.status);
        }
        TradingWebSocketMessage::Error(error) => {
            println!("❌ Error [{}]: {}", error.code, error.msg);
        }
        TradingWebSocketMessage::Unknown(data) => {
            println!("❓ Unknown message: {}", data);
        }
    }
}

fn show_api_features() {
    println!(
        "
🌟 Enhanced WebSocket API Features:

✨ Automatic Reconnection:
   • Exponential backoff retry strategy
   • Configurable max attempts and delays
   • Maintains subscriptions across reconnections

🛡️ Error Handling:
   • Handler errors can be ignored or stop execution
   • Connection errors trigger automatic recovery
   • Verbose logging for debugging

⏰ Flexible Control:
   • Optional timeouts for demos/testing
   • Graceful shutdown on Ctrl+C
   • Channel-based decoupling available

📝 Usage Patterns:

   Simple (most common):
   client.run(|msg| async {{ handle(msg); Ok(()) }}).await?;

   Configured:
   client.run_with_options(handler, custom_options).await?;

   Channel-based:
   client.run_with_channel(sender).await?;

   Manual (power users):
   while let Ok(Some(msg)) = client.next_message().await {{ }}
"
    );
}
