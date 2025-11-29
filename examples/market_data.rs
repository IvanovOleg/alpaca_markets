use alpaca_markets::{
    AlpacaConfig,
    clients::market_data::MarketDataClient,
    models::{
        AlpacaResult,
        market_data::{Adjustment, Sort},
    },
};
use chrono::{Duration, Utc};

#[tokio::main]
async fn main() -> AlpacaResult<()> {
    println!("🚀 Alpaca Markets - Market Data Example");
    println!("========================================");

    // Create configuration from environment variables
    let config = AlpacaConfig::from_env()?.with_iex_feed(); // Use IEX feed for free tier access

    println!("📡 Using IEX feed (free tier)");

    // Create market data client
    let market_data_client = MarketDataClient::new(config);

    // Example symbols to test with
    let symbols = vec!["AAPL", "GOOGL", "MSFT", "TSLA"];

    // Get latest quotes
    println!("\n📈 Latest Quotes:");
    println!("-----------------");

    for symbol in &symbols {
        match market_data_client.get_latest_quote(symbol).await {
            Ok(quote_response) => {
                let quote = &quote_response.quote;
                println!(
                    "{}: Bid: ${:.2} x {} | Ask: ${:.2} x {} @ {}",
                    symbol,
                    quote.bid_price,
                    quote.bid_size,
                    quote.ask_price,
                    quote.ask_size,
                    quote.timestamp.format("%H:%M:%S")
                );
            }
            Err(e) => {
                println!("❌ Error fetching quote for {}: {:?}", symbol, e);
            }
        }
    }

    // Get latest trades
    println!("\n💹 Latest Trades:");
    println!("-----------------");

    for symbol in &symbols {
        match market_data_client.get_latest_trade(symbol).await {
            Ok(trade_response) => {
                let trade = &trade_response.trade;
                println!(
                    "{}: ${:.2} x {} @ {} on {}",
                    symbol,
                    trade.price,
                    trade.size,
                    trade.timestamp.format("%H:%M:%S"),
                    trade.exchange
                );
            }
            Err(e) => {
                println!("❌ Error fetching trade for {}: {:?}", symbol, e);
            }
        }
    }

    // Historical bars with IEX feed
    println!("\n📊 Historical Bars (1-minute, last hour, IEX feed):");
    println!("---------------------------------------------------");

    let end_time = Utc::now();
    let start_time = end_time - Duration::hours(1);

    for symbol in &symbols[..1] {
        match market_data_client
            .get_bars_with_feed(
                symbol,
                "1Min",
                Some(start_time),
                Some(end_time),
                Some(10),
                Some("iex"),
                Some(Sort::Asc),       // or None to use default
                Some(Adjustment::Raw), // or None to use default
            )
            .await
        {
            Ok(bars_response) => {
                let bars = &bars_response.bars;
                if !bars.is_empty() {
                    println!("{}:", symbol);
                    for bar in bars.iter().take(5) {
                        println!(
                            "  {} | O: ${:.2} H: ${:.2} L: ${:.2} C: ${:.2} V: {}",
                            bar.timestamp.format("%H:%M:%S"),
                            bar.open,
                            bar.high,
                            bar.low,
                            bar.close,
                            bar.volume
                        );
                    }
                } else {
                    println!("{}: No bars data available", symbol);
                }
            }
            Err(e) => {
                println!("❌ Error fetching bars for {}: {:?}", symbol, e);
            }
        }
    }

    println!("\n✅ Market data example completed!");
    Ok(())
}
