use alpaca_markets::{AlpacaConfig, clients::trading::TradingClient, models::AlpacaResult};

#[tokio::main]
async fn main() -> AlpacaResult<()> {
    // Create configuration from environment variables
    let config = AlpacaConfig::from_env()?;

    // Create trading client
    let trading_client = TradingClient::new(config);

    println!("📊 Alpaca Portfolio History Example");
    println!("==================================");

    // Example 1: Get portfolio history for the last 30 days with daily timeframe
    println!("\n📈 Portfolio History - Last 30 Days (Daily)");
    println!("-------------------------------------------");

    match trading_client
        .get_portfolio_history(
            Some("30D"), // period: last 30 days
            Some("1D"),  // timeframe: daily data
            None,        // date_end: use current date
            None,        // asof: use current date
            None,        // page_token: no pagination
            None,        // intraday_reporting: default (market_hours)
            None,        // pnl_reset: default (per_day)
        )
        .await
    {
        Ok(history) => {
            println!("✅ Portfolio history retrieved successfully!");
            println!("   Timeframe: {}", history.timeframe);
            println!("   Base Value: ${:.2}", history.base_value);
            println!("   Data Points: {}", history.timestamp.len());

            if !history.timestamp.is_empty() {
                println!("\n   📊 Portfolio Summary:");
                let latest_equity = history.equity.last().unwrap_or(&0.0);
                let latest_pl = history.profit_loss.last().unwrap_or(&0.0);
                let latest_pl_pct = history.profit_loss_pct.last().unwrap_or(&0.0);

                println!("   Current Equity: ${:.2}", latest_equity);
                println!("   Current P&L: ${:.2}", latest_pl);
                println!("   Current P&L %: {:.2}%", latest_pl_pct * 100.0);

                // Show first and last few data points
                println!("\n   📈 Recent Data Points:");
                let show_points = std::cmp::min(5, history.timestamp.len());
                for i in
                    (history.timestamp.len().saturating_sub(show_points))..history.timestamp.len()
                {
                    let timestamp = chrono::DateTime::from_timestamp(history.timestamp[i], 0)
                        .unwrap_or_default()
                        .format("%Y-%m-%d");
                    println!(
                        "   {} | Equity: ${:.2} | P&L: ${:.2} ({:.2}%)",
                        timestamp,
                        history.equity[i],
                        history.profit_loss[i],
                        history.profit_loss_pct[i] * 100.0
                    );
                }
            }
        }
        Err(e) => {
            println!("❌ Error fetching portfolio history: {:?}", e);
        }
    }

    // Example 2: Get intraday portfolio history with hourly timeframe
    println!("\n⏰ Portfolio History - Last 3 Days (Hourly)");
    println!("------------------------------------------");

    match trading_client
        .get_portfolio_history(
            Some("3D"),             // period: last 3 days
            Some("1H"),             // timeframe: hourly data
            None,                   // date_end: use current date
            None,                   // asof: use current date
            None,                   // page_token: no pagination
            Some("extended_hours"), // intraday_reporting: include pre/after market
            Some("no_reset"),       // pnl_reset: continuous P&L calculation
        )
        .await
    {
        Ok(history) => {
            println!("✅ Intraday portfolio history retrieved!");
            println!("   Timeframe: {}", history.timeframe);
            println!("   Base Value: ${:.2}", history.base_value);
            println!("   Data Points: {}", history.timestamp.len());

            if !history.timestamp.is_empty() {
                // Calculate performance metrics
                let first_equity = history.equity.first().unwrap_or(&0.0);
                let latest_equity = history.equity.last().unwrap_or(&0.0);
                let total_return = (latest_equity - first_equity) / first_equity * 100.0;

                println!("\n   📊 Performance Metrics:");
                println!("   Starting Equity: ${:.2}", first_equity);
                println!("   Current Equity: ${:.2}", latest_equity);
                println!("   Total Return: {:.2}%", total_return);

                // Find highest and lowest equity values
                let max_equity = history
                    .equity
                    .iter()
                    .fold(f64::NEG_INFINITY, |a, &b| a.max(b));
                let min_equity = history.equity.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                let max_drawdown = (max_equity - min_equity) / max_equity * 100.0;

                println!("   Highest Equity: ${:.2}", max_equity);
                println!("   Lowest Equity: ${:.2}", min_equity);
                println!("   Max Drawdown: {:.2}%", max_drawdown);
            }
        }
        Err(e) => {
            println!("❌ Error fetching intraday history: {:?}", e);
        }
    }

    // Example 3: Get crypto-optimized portfolio history
    println!("\n₿ Portfolio History - Crypto Optimized (24/7)");
    println!("--------------------------------------------");

    match trading_client
        .get_portfolio_history(
            Some("7D"),         // period: last 7 days
            Some("1H"),         // timeframe: hourly data
            None,               // date_end: use current date
            None,               // asof: use current date
            None,               // page_token: no pagination
            Some("continuous"), // intraday_reporting: 24/7 data for crypto
            Some("no_reset"),   // pnl_reset: continuous P&L for crypto
        )
        .await
    {
        Ok(history) => {
            println!("✅ Crypto-optimized portfolio history retrieved!");
            println!("   Timeframe: {}", history.timeframe);
            println!("   Base Value: ${:.2}", history.base_value);
            println!("   Data Points: {}", history.timestamp.len());
            println!("   24/7 Continuous Data: Yes");

            if !history.timestamp.is_empty() {
                let latest_equity = history.equity.last().unwrap_or(&0.0);
                let latest_pl_pct = history.profit_loss_pct.last().unwrap_or(&0.0);

                println!("\n   📊 Current Status:");
                println!("   Current Equity: ${:.2}", latest_equity);
                println!("   Current P&L %: {:.2}%", latest_pl_pct * 100.0);
            }
        }
        Err(e) => {
            println!("❌ Error fetching crypto history: {:?}", e);
        }
    }

    println!("\n✨ Portfolio history example completed successfully!");

    Ok(())
}
