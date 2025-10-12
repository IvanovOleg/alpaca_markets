// Alpaca Trading WebSocket Stream Example
//
// This example demonstrates how to connect to Alpaca's trading WebSocket stream
// to receive real-time updates about orders and account changes.
//
// To run this example:
// 1. Set your environment variables (optional):
//    - APCA_API_KEY_ID=your_api_key
//    - APCA_API_SECRET_KEY=your_secret_key
// 2. Run: cargo run --example trading_websocket_demo --features "websocket"

use alpaca_markets::AlpacaConfig;

#[cfg(feature = "websocket")]
use alpaca_markets::clients::trading_stream::TradingStreamClient;

#[cfg(feature = "websocket")]
use alpaca_markets::wss::trading::{AccountUpdate, TradeUpdate, TradingWebSocketMessage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "🚀 Alpaca Trading WebSocket Stream Demo
========================================="
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
                    true, // Use paper trading
                )
            }
        };

        // Create trading stream client
        let mut client = TradingStreamClient::new(config);

        println!("🔌 Connecting to Alpaca Trading WebSocket...");

        match client.connect().await {
            Ok(_) => {
                println!(
                    "✅ Connected successfully!
👂 Listening for updates... (Ctrl+C to stop)\n"
                );
            }
            Err(e) => {
                eprintln!("❌ Connection failed: {}", e);
                println!("💡 This is expected with demo credentials.");
                println!("   Set real API credentials to connect to Alpaca.");
                return Ok(());
            }
        }

        // Message counter for demo purposes
        let mut count = 0;

        // Listen for messages
        loop {
            tokio::select! {
                // Handle WebSocket messages
                message_result = client.next_message() => {
                    match message_result {
                        Ok(Some(message)) => {
                            count += 1;
                            handle_websocket_message(message, count);
                        }
                        Ok(None) => {
                            // No message, continue
                            continue;
                        }
                        Err(e) => {
                            eprintln!("❌ Message error: {}", e);
                            break;
                        }
                    }
                }

                // Handle Ctrl+C
                _ = tokio::signal::ctrl_c() => {
                    println!("\n👋 Shutting down...");
                    break;
                }

                // Add a timeout for demo purposes (remove in real usage)
                _ = tokio::time::sleep(tokio::time::Duration::from_secs(30)) => {
                    println!("\n⏰ Demo timeout reached. In real usage, this would run indefinitely.");
                    break;
                }
            }
        }

        // Clean disconnect
        if let Err(e) = client.disconnect().await {
            eprintln!("⚠️  Disconnect error: {}", e);
        } else {
            println!("✅ Disconnected cleanly");
        }
    }

    #[cfg(not(all(feature = "websocket", feature = "trading")))]
    {
        println!("❌ This example requires both 'websocket' and 'trading' features.");
        println!(
            "   Run with: cargo run --example trading_websocket_demo --features \"trading,websocket\""
        );
    }

    Ok(())
}

#[cfg(feature = "websocket")]
fn handle_websocket_message(message: TradingWebSocketMessage, count: usize) {
    println!("📨 Message #{}", count);

    match message {
        TradingWebSocketMessage::TradeUpdate(trade_update) => {
            print_trade_update(trade_update);
        }
        TradingWebSocketMessage::AccountUpdate(account_update) => {
            print_account_update(account_update);
        }
        TradingWebSocketMessage::Connected(connected) => {
            println!("🔗 Connection: {}", connected.msg);
        }
        TradingWebSocketMessage::Authorization(auth) => {
            println!("🔐 Auth: {} -> {}", auth.action, auth.status);
        }
        TradingWebSocketMessage::Listening(listening) => {
            println!("👂 Subscribed to: {:?}", listening.data.streams);
        }
        TradingWebSocketMessage::Error(error) => {
            println!("❌ Error [{}]: {}", error.code, error.msg);
        }
        TradingWebSocketMessage::Unknown(data) => {
            println!("❓ Unknown message: {}", data);
        }
    }
    println!(); // Add spacing
}

#[cfg(feature = "websocket")]
fn print_trade_update(update: TradeUpdate) {
    let mut output = format!(
        "📈 TRADE UPDATE
   └─ Event: {:?}
   └─ Time: {}
   └─ Order:
      ├─ ID: {}
      ├─ Symbol: {}
      ├─ Side: {}
      ├─ Type: {}
      ├─ Status: {}
      ├─ Qty: {:?}
      ├─ Filled: {}",
        update.event,
        update.timestamp.format("%H:%M:%S UTC"),
        update.order.id,
        update.order.symbol,
        update.order.side,
        update.order.order_type,
        update.order.status,
        update.order.qty,
        update.order.filled_qty
    );

    if let Some(price) = &update.order.limit_price {
        output.push_str(&format!("\n      ├─ Limit: ${}", price));
    }
    if let Some(avg_price) = &update.order.filled_avg_price {
        output.push_str(&format!("\n      ├─ Avg Fill: ${}", avg_price));
    }

    output.push_str(&format!(
        "\n      ├─ TIF: {}
      └─ Extended Hrs: {}",
        update.order.time_in_force, update.order.extended_hours
    ));

    println!("{}", output);
}

#[cfg(feature = "websocket")]
fn print_account_update(update: AccountUpdate) {
    println!(
        "💰 ACCOUNT UPDATE
   └─ Event: {}
   └─ Time: {}
   └─ Balances:
      ├─ Buying Power: ${}
      ├─ Portfolio Value: ${}
      ├─ Cash: ${}
      └─ Withdrawable: ${}",
        update.event,
        update.timestamp.format("%H:%M:%S UTC"),
        update.buying_power,
        update.total_portfolio_value,
        update.cash,
        update.cash_withdrawable
    );
}
