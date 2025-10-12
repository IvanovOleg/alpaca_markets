use alpaca_markets::{
    AlpacaConfig,
    clients::trading::TradingClient,
    models::{AlpacaError, AlpacaResult},
};

#[tokio::main]
async fn main() -> AlpacaResult<()> {
    // Create configuration from environment variables
    let config = AlpacaConfig::from_env()?;

    // Create trading client
    let trading_client = TradingClient::new(config);

    // Example 1: Get all open positions
    println!(
        "🏦 Alpaca Position Management Example
===================================="
    );

    println!(
        "
📊 All Open Positions
---------------------"
    );

    match trading_client.get_positions().await {
        Ok(positions) => {
            if positions.is_empty() {
                println!("✓ No open positions found.");
            } else {
                println!("📈 Found {} open position(s):", positions.len());

                let mut total_market_value = 0.0;
                let mut total_unrealized_pl = 0.0;

                for position in &positions {
                    println!(
                        "
  🏷️ Symbol: {}
     Shares: {}
     Avg Entry: ${}
     Market Value: ${}
     Unrealized P&L: ${}
     Unrealized P&L %: {}%",
                        position.symbol,
                        position.qty,
                        position.avg_entry_price,
                        position.market_value,
                        position.unrealized_pl,
                        position.unrealized_plpc
                    );

                    // Parse numeric values for totals (with error handling)
                    if let (Ok(mv), Ok(upl)) = (
                        position.market_value.parse::<f64>(),
                        position.unrealized_pl.parse::<f64>(),
                    ) {
                        total_market_value += mv;
                        total_unrealized_pl += upl;
                    }
                }

                let plpc_display = if total_market_value > 0.0 {
                    let total_plpc =
                        (total_unrealized_pl / (total_market_value - total_unrealized_pl)) * 100.0;
                    format!("\n     Total P&L %: {:.2}%", total_plpc)
                } else {
                    String::new()
                };

                println!(
                    "
  📊 Portfolio Summary:
     Total Market Value: ${:.2}
     Total Unrealized P&L: ${:.2}{}",
                    total_market_value, total_unrealized_pl, plpc_display
                );
            }
        }
        Err(e) => {
            println!("❌ Error fetching positions: {:?}", e);
        }
    }

    // Example 2: Get specific position by symbol
    println!(
        "
🎯 Individual Position Lookup
-----------------------------"
    );

    let test_symbols = ["AAPL", "GOOGL", "TSLA", "MSFT"];

    for symbol in test_symbols.iter() {
        match trading_client.get_position(symbol).await {
            Ok(position) => {
                println!(
                    "
  ✅ {} Position Found:
     Shares: {}
     Avg Entry: ${}
     Current Price: ${}
     Market Value: ${}
     Unrealized P&L: ${} ({}%)
     Day Change: ${} ({}%)",
                    symbol,
                    position.qty,
                    position.avg_entry_price,
                    position.current_price,
                    position.market_value,
                    position.unrealized_pl,
                    position.unrealized_plpc,
                    position.unrealized_intraday_pl,
                    position.unrealized_intraday_plpc
                );
            }
            Err(AlpacaError::ApiError {
                code: Some(404), ..
            }) => {
                println!("  ⚪ No {} position found", symbol);
            }
            Err(e) => {
                println!("  ❌ Error looking up {} position: {:?}", symbol, e);
            }
        }
    }

    // Example 3: Position analysis
    println!(
        "
📈 Position Analysis
-------------------"
    );

    match trading_client.get_positions().await {
        Ok(positions) => {
            if !positions.is_empty() {
                let profitable_positions: Vec<_> = positions
                    .iter()
                    .filter(|p| p.unrealized_pl.parse::<f64>().unwrap_or(0.0) > 0.0)
                    .collect();

                let losing_positions: Vec<_> = positions
                    .iter()
                    .filter(|p| p.unrealized_pl.parse::<f64>().unwrap_or(0.0) < 0.0)
                    .collect();

                println!(
                    "  📊 Position Breakdown:
     Profitable positions: {}
     Losing positions: {}
     Break-even positions: {}",
                    profitable_positions.len(),
                    losing_positions.len(),
                    positions.len() - profitable_positions.len() - losing_positions.len()
                );

                if !profitable_positions.is_empty() {
                    let best_performer = profitable_positions
                        .iter()
                        .max_by(|a, b| {
                            let a_plpc = a.unrealized_plpc.parse::<f64>().unwrap_or(0.0);
                            let b_plpc = b.unrealized_plpc.parse::<f64>().unwrap_or(0.0);
                            a_plpc.partial_cmp(&b_plpc).unwrap()
                        })
                        .unwrap();
                    println!(
                        "  📈 Best Performer: {} ({}%)",
                        best_performer.symbol, best_performer.unrealized_plpc
                    );
                }

                if !losing_positions.is_empty() {
                    let worst_performer = losing_positions
                        .iter()
                        .min_by(|a, b| {
                            let a_plpc = a.unrealized_plpc.parse::<f64>().unwrap_or(0.0);
                            let b_plpc = b.unrealized_plpc.parse::<f64>().unwrap_or(0.0);
                            a_plpc.partial_cmp(&b_plpc).unwrap()
                        })
                        .unwrap();
                    println!(
                        "  📉 Worst Performer: {} ({}%)",
                        worst_performer.symbol, worst_performer.unrealized_plpc
                    );
                }
            }
        }
        Err(e) => {
            println!("❌ Error analyzing positions: {:?}", e);
        }
    }

    println!("\n✨ Position management example completed successfully!");

    Ok(())
}
