// Alpaca Market Data WebSocket Stream Example (MessagePack)
//
// This example demonstrates how to connect to Alpaca's market data WebSocket stream
// to receive real-time stock quotes, trades, and bars using MessagePack encoding.
//
// To run this example:
// 1. Set your environment variables:
//    - APCA_API_KEY_ID=your_api_key
//    - APCA_API_SECRET_KEY=your_secret_key
// 2. Run: cargo run --example market_data_websocket_demo --features "websocket"

use alpaca_markets::AlpacaConfig;

#[cfg(feature = "websocket")]
use alpaca_markets::clients::market_data_stream::{Feed, MarketDataStreamClient};

#[cfg(feature = "websocket")]
use alpaca_markets::wss::market_data::{BarMessage, MarketDataMessage, QuoteMessage, TradeMessage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "📊 Alpaca Market Data WebSocket Stream Demo
==========================================="
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

        // Create market data stream client (MessagePack only, IEX feed)
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

                println!("👂 Listening for market data... (Ctrl+C to stop)\n");
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
                        Ok(Some(messages)) => {
                            // Market data comes in arrays, process each message
                            for message in messages {
                                count += 1;
                                handle_market_data_message(message, count);
                            }
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

                // Add a timeout for demo purposes
                _ = tokio::time::sleep(tokio::time::Duration::from_secs(60)) => {
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

    #[cfg(not(all(feature = "websocket", feature = "market_data")))]
    {
        println!("❌ This example requires both 'websocket' and 'market_data' features.");
        println!(
            "   Run with: cargo run --example market_data_websocket_demo --features \"market_data,websocket\""
        );
    }

    Ok(())
}

#[cfg(feature = "websocket")]
fn handle_market_data_message(message: MarketDataMessage, count: usize) {
    println!("📨 Message #{}", count);

    match message {
        MarketDataMessage::Trade(trade) => {
            print_trade_message(trade);
        }
        MarketDataMessage::Quote(quote) => {
            print_quote_message(quote);
        }
        MarketDataMessage::Bar(bar) => {
            print_bar_message(bar);
        }
        MarketDataMessage::Subscription(sub) => {
            println!("📋 Subscription Update: {:?}", sub);
        }
        MarketDataMessage::Error(error) => {
            println!("❌ Stream Error: {:?}", error);
        }
    }
    println!(); // Add spacing
}

#[cfg(feature = "websocket")]
fn print_trade_message(trade: TradeMessage) {
    let mut output = format!(
        "📈 TRADE
   └─ Symbol: {}
   └─ Price: ${:.2}
   └─ Size: {}
   └─ Time: {}
   └─ Exchange: {}",
        trade.symbol,
        trade.price,
        trade.size,
        trade.timestamp.format("%H:%M:%S.%3f UTC"),
        trade.exchange
    );

    if !trade.conditions.is_empty() {
        output.push_str(&format!("\n   └─ Conditions: {:?}", trade.conditions));
    }

    println!("{}", output);
}

#[cfg(feature = "websocket")]
fn print_quote_message(quote: QuoteMessage) {
    println!(
        "💰 QUOTE
   └─ Symbol: {}
   └─ Bid: ${:.2} x {}
   └─ Ask: ${:.2} x {}
   └─ Spread: ${:.2}
   └─ Time: {}",
        quote.symbol,
        quote.bid_price,
        quote.bid_size,
        quote.ask_price,
        quote.ask_size,
        quote.ask_price - quote.bid_price,
        quote.timestamp.format("%H:%M:%S.%3f UTC")
    );
}

#[cfg(feature = "websocket")]
fn print_bar_message(bar: BarMessage) {
    println!(
        "📊 BAR (1-minute)
   └─ Symbol: {}
   └─ OHLC: ${:.2} / ${:.2} / ${:.2} / ${:.2}
   └─ Volume: {}
   └─ VWAP: ${:.2}
   └─ Time: {}",
        bar.symbol,
        bar.open,
        bar.high,
        bar.low,
        bar.close,
        bar.volume,
        bar.vwap,
        bar.timestamp.format("%H:%M:%S UTC")
    );
}
