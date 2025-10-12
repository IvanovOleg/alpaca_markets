use alpaca_markets::{AlpacaConfig, clients::trading::TradingClient, models::AlpacaResult};

#[tokio::main]
async fn main() -> AlpacaResult<()> {
    println!(
        "🚀 Alpaca Markets - Assets Management Example
=============================================="
    );

    // Create configuration from environment variables
    let config = AlpacaConfig::from_env()?;

    // Create trading client
    let trading_client = TradingClient::new(config);

    // Example 1: Get all assets with active status filter (first 10)
    println!(
        "
📊 Active Assets (first 10):
-----------------------------"
    );

    match trading_client.get_assets(Some("active"), None, None).await {
        Ok(assets) => {
            println!("Found {} active assets", assets.len());
            for asset in assets.iter().take(10) {
                println!(
                    "{}: {} - {} | Tradable: {} | Marginable: {} | Shortable: {}",
                    asset.symbol,
                    asset.name,
                    asset.class,
                    asset.tradable,
                    asset.marginable,
                    asset.shortable
                );
            }
        }
        Err(e) => {
            println!("❌ Error fetching active assets: {:?}", e);
        }
    }

    // Example 2: Get all stocks (US equities)
    println!(
        "
📈 US Equities (first 10):
---------------------------"
    );

    match trading_client
        .get_assets(Some("active"), Some("us_equity"), None)
        .await
    {
        Ok(stocks) => {
            println!("Found {} US equity assets", stocks.len());
            for stock in stocks.iter().take(10) {
                println!(
                    "{}: {} | Exchange: {} | Fractionable: {} | Easy to Borrow: {}",
                    stock.symbol,
                    stock.name,
                    stock.exchange,
                    stock.fractionable,
                    stock.easy_to_borrow
                );
            }
        }
        Err(e) => {
            println!("❌ Error fetching stocks: {:?}", e);
        }
    }

    // Example 3: Get crypto assets
    println!(
        "
💰 Crypto Assets:
-----------------"
    );

    match trading_client
        .get_assets(Some("active"), Some("crypto"), None)
        .await
    {
        Ok(crypto_assets) => {
            if crypto_assets.is_empty() {
                println!("No crypto assets found (may require special account permissions)");
            } else {
                println!("Found {} crypto assets", crypto_assets.len());
                for crypto in crypto_assets.iter().take(10) {
                    println!(
                        "{}: {} | Exchange: {} | Tradable: {}",
                        crypto.symbol, crypto.name, crypto.exchange, crypto.tradable
                    );
                }
            }
        }
        Err(e) => {
            println!("❌ Error fetching crypto assets: {:?}", e);
        }
    }

    // Example 4: Get specific assets by symbol
    println!(
        "
🎯 Specific Asset Lookup:
-------------------------"
    );

    let symbols_to_check = vec!["AAPL", "GOOGL", "MSFT", "TSLA", "SPY"];

    for symbol in symbols_to_check {
        match trading_client.get_asset(symbol).await {
            Ok(asset) => {
                println!(
                    "{}: {} | Class: {} | Exchange: {} | Status: {}",
                    asset.symbol, asset.name, asset.class, asset.exchange, asset.status
                );

                // Show detailed trading characteristics
                let characteristics = vec![
                    format!("Tradable: {}", asset.tradable),
                    format!("Marginable: {}", asset.marginable),
                    format!("Shortable: {}", asset.shortable),
                    format!("Fractionable: {}", asset.fractionable),
                    format!("Easy to Borrow: {}", asset.easy_to_borrow),
                ];
                println!("  Characteristics: {}", characteristics.join(" | "));
            }
            Err(e) => {
                println!("❌ Error fetching {}: {:?}", symbol, e);
            }
        }
    }

    // Example 5: Filter assets by exchange
    println!(
        "
🏛️ Assets by Exchange (NASDAQ, first 5):
------------------------------------------"
    );

    match trading_client
        .get_assets(Some("active"), None, Some("NASDAQ"))
        .await
    {
        Ok(nasdaq_assets) => {
            println!("Found {} NASDAQ assets", nasdaq_assets.len());
            for asset in nasdaq_assets.iter().take(5) {
                println!(
                    "{}: {} | Status: {} | Marginable: {}",
                    asset.symbol, asset.name, asset.status, asset.marginable
                );
            }
        }
        Err(e) => {
            println!("❌ Error fetching NASDAQ assets: {:?}", e);
        }
    }

    // Example 6: Show available asset filtering options
    println!(
        "
⚙️ Available Asset Filtering Methods:
-------------------------------------

✅ get_assets(status, asset_class, exchange) - Full filtering control
  • Status options: 'active', 'inactive'
  • Asset class options: 'us_equity', 'crypto'
  • Exchange options: 'NASDAQ', 'NYSE', 'AMEX', etc.

✅ get_asset(symbol_or_id) - Get specific asset by symbol or ID"
    ); // Example 7: Asset statistics
    println!(
        "
📊 Asset Statistics:
--------------------"
    );

    match trading_client.get_assets(Some("active"), None, None).await {
        Ok(all_assets) => {
            let total_assets = all_assets.len();
            let marginable_count = all_assets.iter().filter(|a| a.marginable).count();
            let shortable_count = all_assets.iter().filter(|a| a.shortable).count();
            let fractionable_count = all_assets.iter().filter(|a| a.fractionable).count();

            println!("Total Tradable Assets: {}", total_assets);
            println!(
                "Marginable Assets: {} ({:.1}%)",
                marginable_count,
                (marginable_count as f64 / total_assets as f64) * 100.0
            );
            println!(
                "Shortable Assets: {} ({:.1}%)",
                shortable_count,
                (shortable_count as f64 / total_assets as f64) * 100.0
            );
            println!(
                "Fractionable Assets: {} ({:.1}%)",
                fractionable_count,
                (fractionable_count as f64 / total_assets as f64) * 100.0
            );
        }
        Err(e) => {
            println!("❌ Error calculating asset statistics: {:?}", e);
        }
    }

    println!(
        "
✅ Assets management example completed!
💡 Use these methods to discover tradable assets and their characteristics"
    );

    Ok(())
}
