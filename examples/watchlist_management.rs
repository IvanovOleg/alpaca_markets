use alpaca_markets::{
    AlpacaConfig,
    clients::trading::TradingClient,
    models::{AlpacaResult, CreateWatchlistRequest, UpdateWatchlistRequest},
};

#[tokio::main]
async fn main() -> AlpacaResult<()> {
    // Create configuration from environment variables
    let config = AlpacaConfig::from_env()?;

    // Create trading client
    let client = TradingClient::new(config);

    println!("🔍 Comprehensive Watchlist Management Demo");
    println!("{}", "=".repeat(50));

    // 1. Get all existing watchlists
    println!("\n📋 Fetching all watchlists...");
    match client.get_watchlists().await {
        Ok(watchlists) => {
            println!("Found {} watchlist(s):", watchlists.len());
            for watchlist in &watchlists {
                println!(
                    "  • {} (ID: {}) - {} assets",
                    watchlist.name,
                    watchlist.id,
                    watchlist.assets.len()
                );
            }
        }
        Err(e) => println!("Error fetching watchlists: {}", e),
    }

    // 2. Create a new watchlist with symbols
    println!("\n🆕 Creating new watchlist 'Tech Giants'...");
    let create_request = CreateWatchlistRequest {
        name: "Tech Giants".to_string(),
        symbols: Some(vec![
            "AAPL".to_string(),
            "MSFT".to_string(),
            "GOOGL".to_string(),
            "AMZN".to_string(),
        ]),
    };

    let mut created_watchlist_id: Option<String> = None;
    match client.create_watchlist(create_request).await {
        Ok(watchlist) => {
            println!("✅ Created watchlist: {}", watchlist.name);
            println!("   ID: {}", watchlist.id);
            println!("   Assets: {} symbols", watchlist.assets.len());

            for asset in &watchlist.assets {
                println!("     - {} ({})", asset.symbol, asset.class);
            }
            created_watchlist_id = Some(watchlist.id.clone());
        }
        Err(e) => println!("❌ Error creating watchlist: {}", e),
    }

    // 3. Get watchlist by ID (if we created one)
    if let Some(watchlist_id) = &created_watchlist_id {
        println!("\n🔍 Fetching watchlist by ID: {}...", watchlist_id);
        match client.get_watchlist_by_id(watchlist_id).await {
            Ok(watchlist) => {
                println!(
                    "✅ Retrieved watchlist: {}
   Created at: {}
   Updated at: {}
   Account: {}",
                    watchlist.name,
                    watchlist.created_at,
                    watchlist.updated_at,
                    watchlist.account_id
                );
            }
            Err(e) => println!("❌ Error fetching watchlist by ID: {}", e),
        }
    }

    // 4. Add more assets to the watchlist
    if let Some(watchlist_id) = &created_watchlist_id {
        println!("\n➕ Adding TSLA to watchlist...");
        match client.add_asset_to_watchlist(watchlist_id, "TSLA").await {
            Ok(watchlist) => {
                println!(
                    "✅ Added TSLA. Watchlist now has {} assets",
                    watchlist.assets.len()
                );
            }
            Err(e) => println!("❌ Error adding asset: {}", e),
        }

        println!("\n➕ Adding NVDA to watchlist...");
        match client.add_asset_to_watchlist(watchlist_id, "NVDA").await {
            Ok(watchlist) => {
                println!(
                    "✅ Added NVDA. Watchlist now has {} assets",
                    watchlist.assets.len()
                );
            }
            Err(e) => println!("❌ Error adding asset: {}", e),
        }
    }

    // 5. Update watchlist name and description
    if let Some(watchlist_id) = &created_watchlist_id {
        println!("\n📝 Updating watchlist name to 'Mega Tech Stocks'...");
        let update_request = UpdateWatchlistRequest {
            name: Some("Mega Tech Stocks".to_string()),
            symbols: None, // Don't change symbols in this update
        };

        match client
            .update_watchlist_by_id(watchlist_id, update_request)
            .await
        {
            Ok(watchlist) => {
                println!("✅ Updated watchlist name to: {}", watchlist.name);
            }
            Err(e) => println!("❌ Error updating watchlist: {}", e),
        }
    }

    // 6. Demonstrate name-based operations
    println!("\n🏷️ Testing name-based operations...");

    // Get by name
    match client.get_watchlist_by_name("Mega Tech Stocks").await {
        Ok(watchlist) => {
            println!("✅ Retrieved watchlist by name: {}", watchlist.name);
            println!(
                "   Assets: {:?}",
                watchlist
                    .assets
                    .iter()
                    .map(|a| &a.symbol)
                    .collect::<Vec<_>>()
            );
        }
        Err(e) => println!("❌ Error fetching by name: {}", e),
    }

    // Add asset by name
    println!("\n➕ Adding AMD using name-based method...");
    match client
        .add_asset_to_watchlist_by_name("Mega Tech Stocks", "AMD")
        .await
    {
        Ok(watchlist) => {
            println!(
                "✅ Added AMD. Watchlist now has {} assets",
                watchlist.assets.len()
            );
        }
        Err(e) => println!("❌ Error adding asset by name: {}", e),
    }

    // 7. Remove an asset
    if let Some(watchlist_id) = &created_watchlist_id {
        println!("\n➖ Removing AMZN from watchlist...");
        match client
            .remove_asset_from_watchlist(watchlist_id, "AMZN")
            .await
        {
            Ok(watchlist) => {
                println!(
                    "✅ Removed AMZN. Watchlist now has {} assets",
                    watchlist.assets.len()
                );
                println!(
                    "   Remaining symbols: {:?}",
                    watchlist
                        .assets
                        .iter()
                        .map(|a| &a.symbol)
                        .collect::<Vec<_>>()
                );
            }
            Err(e) => println!("❌ Error removing asset: {}", e),
        }
    }

    // 8. Create a crypto watchlist
    println!("\n₿ Creating crypto watchlist...");
    let crypto_request = CreateWatchlistRequest {
        name: "Crypto Portfolio".to_string(),
        symbols: Some(vec![
            "BTC/USD".to_string(),
            "ETH/USD".to_string(),
            "DOGE/USD".to_string(),
        ]),
    };

    let mut crypto_watchlist_id: Option<String> = None;
    match client.create_watchlist(crypto_request).await {
        Ok(watchlist) => {
            println!("✅ Created crypto watchlist: {}", watchlist.name);
            crypto_watchlist_id = Some(watchlist.id.clone());
        }
        Err(e) => println!("❌ Error creating crypto watchlist: {}", e),
    }

    // 9. Update crypto watchlist with replacement symbols
    if let Some(watchlist_id) = &crypto_watchlist_id {
        println!("\n🔄 Replacing crypto watchlist symbols...");
        let replace_request = UpdateWatchlistRequest {
            name: None, // Keep the same name
            symbols: Some(vec![
                "BTC/USD".to_string(),
                "ETH/USD".to_string(),
                "ADA/USD".to_string(),
                "SOL/USD".to_string(),
            ]),
        };

        match client
            .update_watchlist_by_id(watchlist_id, replace_request)
            .await
        {
            Ok(watchlist) => {
                println!("✅ Updated crypto watchlist symbols");
                println!(
                    "   New symbols: {:?}",
                    watchlist
                        .assets
                        .iter()
                        .map(|a| &a.symbol)
                        .collect::<Vec<_>>()
                );
            }
            Err(e) => println!("❌ Error updating crypto watchlist: {}", e),
        }
    }

    // 10. Final summary - list all watchlists
    println!("\n📊 Final watchlist summary:");
    match client.get_watchlists().await {
        Ok(watchlists) => {
            println!("Total watchlists: {}", watchlists.len());
            for (i, watchlist) in watchlists.iter().enumerate() {
                println!("  {}. {} (ID: {})", i + 1, watchlist.name, watchlist.id);

                println!(
                    "     Assets ({}): {}",
                    watchlist.assets.len(),
                    watchlist
                        .assets
                        .iter()
                        .take(5) // Show first 5 assets
                        .map(|a| a.symbol.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                );

                if watchlist.assets.len() > 5 {
                    println!("     ... and {} more", watchlist.assets.len() - 5);
                }
            }
        }
        Err(e) => println!("❌ Error fetching final summary: {}", e),
    }

    // Cleanup - Delete the watchlists we created (optional)
    println!("\n🧹 Cleanup - Deleting demo watchlists...");

    if let Some(watchlist_id) = created_watchlist_id {
        match client.delete_watchlist_by_id(&watchlist_id).await {
            Ok(()) => println!("✅ Deleted 'Mega Tech Stocks' watchlist"),
            Err(e) => println!("❌ Error deleting tech watchlist: {}", e),
        }
    }

    if crypto_watchlist_id.is_some() {
        match client.delete_watchlist_by_name("Crypto Portfolio").await {
            Ok(()) => println!("✅ Deleted 'Crypto Portfolio' watchlist"),
            Err(e) => println!("❌ Error deleting crypto watchlist: {}", e),
        }
    }

    println!(
        "
🎉 Watchlist management demo completed!

Key Features Demonstrated:
  • Create watchlists with initial symbols
  • Get all watchlists
  • Get watchlist by ID and by name
  • Add/remove individual assets
  • Update watchlist name and replace all symbols
  • Delete watchlists by ID and by name
  • Handle both stock and crypto symbols
  • Comprehensive error handling"
    );

    Ok(())
}
