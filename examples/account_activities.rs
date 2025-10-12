use alpaca_markets::{AlpacaConfig, clients::trading::TradingClient, models::AlpacaResult};

#[tokio::main]
async fn main() -> AlpacaResult<()> {
    // Create configuration from environment variables
    let config = AlpacaConfig::from_env()?;

    // Create trading client
    let trading_client = TradingClient::new(config);

    println!("📋 Alpaca Account Activities Management Example");
    println!("============================================");

    // Example 1: Get all recent account activities
    println!("\n📊 All Recent Account Activities");
    println!("-------------------------------");

    match trading_client
        .get_account_activities(
            None,         // All activity types
            None,         // No specific date
            None,         // No until date
            None,         // No after date
            Some("desc"), // Most recent first
            Some(10),     // Limit to 10 activities
            None,         // No pagination token
        )
        .await
    {
        Ok(activities) => {
            if activities.is_empty() {
                println!("ℹ️  No activities found");
            } else {
                println!("✅ Found {} recent activities:", activities.len());
                for (i, activity) in activities.iter().enumerate() {
                    println!(
                        "  {}. {:?} - {} ({})",
                        i + 1,
                        activity.activity_type,
                        activity.description.as_deref().unwrap_or("No description"),
                        activity.transaction_time.format("%Y-%m-%d %H:%M:%S UTC")
                    );

                    if let Some(symbol) = &activity.symbol {
                        println!("     Symbol: {}", symbol);
                    }
                    if let Some(net_amount) = &activity.net_amount {
                        println!("     Net Amount: ${}", net_amount);
                    }
                    if let Some(qty) = &activity.qty {
                        println!("     Quantity: {}", qty);
                    }
                    println!();
                }
            }
        }
        Err(e) => {
            println!("❌ Error fetching account activities: {}", e);
        }
    }

    // Example 2: Get only trade fills (FILL activities)
    println!("\n🎯 Trade Fill Activities Only");
    println!("----------------------------");

    match trading_client
        .get_account_activities_by_type(
            "FILL",       // Only trade fills
            None,         // No specific date
            None,         // No until date
            None,         // No after date
            Some("desc"), // Most recent first
            Some(5),      // Limit to 5 activities
            None,         // No pagination token
        )
        .await
    {
        Ok(activities) => {
            if activities.is_empty() {
                println!("ℹ️  No FILL activities found");
            } else {
                println!("✅ Found {} FILL activities:", activities.len());
                for (i, activity) in activities.iter().enumerate() {
                    println!(
                        "  {}. Trade Fill - {}",
                        i + 1,
                        activity.transaction_time.format("%Y-%m-%d %H:%M:%S UTC")
                    );

                    if let Some(symbol) = &activity.symbol {
                        println!("     Symbol: {}", symbol);
                    }
                    if let Some(side) = &activity.side {
                        println!("     Side: {}", side);
                    }
                    if let Some(qty) = &activity.qty {
                        println!("     Quantity: {}", qty);
                    }
                    if let Some(price) = &activity.price {
                        println!("     Price: ${}", price);
                    }
                    if let Some(net_amount) = &activity.net_amount {
                        println!("     Net Amount: ${}", net_amount);
                    }
                    println!();
                }
            }
        }
        Err(e) => {
            println!("❌ Error fetching FILL activities: {}", e);
        }
    }

    // Example 3: Get dividend activities
    println!("\n💰 Dividend Activities");
    println!("---------------------");

    match trading_client
        .get_account_activities_by_type(
            "DIV",        // Dividend activities
            None,         // No specific date
            None,         // No until date
            None,         // No after date
            Some("desc"), // Most recent first
            Some(5),      // Limit to 5 activities
            None,         // No pagination token
        )
        .await
    {
        Ok(activities) => {
            if activities.is_empty() {
                println!("ℹ️  No dividend activities found");
            } else {
                println!("✅ Found {} dividend activities:", activities.len());
                let mut total_dividends = 0.0;

                for (i, activity) in activities.iter().enumerate() {
                    println!(
                        "  {}. Dividend - {}",
                        i + 1,
                        activity.transaction_time.format("%Y-%m-%d %H:%M:%S UTC")
                    );

                    if let Some(symbol) = &activity.symbol {
                        println!("     Symbol: {}", symbol);
                    }
                    if let Some(net_amount) = &activity.net_amount {
                        if let Ok(amount) = net_amount.parse::<f64>() {
                            total_dividends += amount;
                        }
                        println!("     Dividend Amount: ${}", net_amount);
                    }
                    if let Some(qty) = &activity.qty {
                        println!("     Shares: {}", qty);
                    }
                    println!();
                }

                if total_dividends > 0.0 {
                    println!("💰 Total Recent Dividends: ${:.2}", total_dividends);
                }
            }
        }
        Err(e) => {
            println!("❌ Error fetching dividend activities: {}", e);
        }
    }

    // Example 4: Get fee activities
    println!("\n💳 Fee Activities");
    println!("----------------");

    match trading_client
        .get_account_activities_by_type(
            "FEE",        // Fee activities
            None,         // No specific date
            None,         // No until date
            None,         // No after date
            Some("desc"), // Most recent first
            Some(5),      // Limit to 5 activities
            None,         // No pagination token
        )
        .await
    {
        Ok(activities) => {
            if activities.is_empty() {
                println!("ℹ️  No fee activities found");
            } else {
                println!("✅ Found {} fee activities:", activities.len());
                let mut total_fees = 0.0;

                for (i, activity) in activities.iter().enumerate() {
                    println!(
                        "  {}. Fee - {}",
                        i + 1,
                        activity.transaction_time.format("%Y-%m-%d %H:%M:%S UTC")
                    );

                    if let Some(description) = &activity.description {
                        println!("     Description: {}", description);
                    }
                    if let Some(net_amount) = &activity.net_amount {
                        if let Ok(amount) = net_amount.parse::<f64>() {
                            total_fees += amount.abs(); // Fees are usually negative
                        }
                        println!("     Fee Amount: ${}", net_amount);
                    }
                    println!();
                }

                if total_fees > 0.0 {
                    println!("💳 Total Recent Fees: ${:.2}", total_fees);
                }
            }
        }
        Err(e) => {
            println!("❌ Error fetching fee activities: {}", e);
        }
    }

    // Example 5: Get activities with multiple types filter
    println!("\n🔍 Mixed Activity Types (FILL, DIV, FEE)");
    println!("---------------------------------------");

    match trading_client
        .get_account_activities(
            Some("FILL,DIV,FEE"), // Multiple activity types
            None,                 // No specific date
            None,                 // No until date
            None,                 // No after date
            Some("desc"),         // Most recent first
            Some(10),             // Limit to 10 activities
            None,                 // No pagination token
        )
        .await
    {
        Ok(activities) => {
            if activities.is_empty() {
                println!("ℹ️  No mixed activities found");
            } else {
                println!("✅ Found {} mixed activities:", activities.len());

                let mut fills = 0;
                let mut dividends = 0;
                let mut fees = 0;

                for activity in &activities {
                    match format!("{:?}", activity.activity_type).as_str() {
                        "Fill" => fills += 1,
                        "Div" => dividends += 1,
                        "Fee" => fees += 1,
                        _ => {}
                    }

                    println!(
                        "  • {:?} - {} ({})",
                        activity.activity_type,
                        activity.description.as_deref().unwrap_or("No description"),
                        activity.transaction_time.format("%Y-%m-%d")
                    );
                }

                println!("\n📊 Activity Breakdown:");
                println!("   Trade Fills: {}", fills);
                println!("   Dividends: {}", dividends);
                println!("   Fees: {}", fees);
            }
        }
        Err(e) => {
            println!("❌ Error fetching mixed activities: {}", e);
        }
    }

    println!("\n✨ Account activities examples completed!");

    Ok(())
}
