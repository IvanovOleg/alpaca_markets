use alpaca_markets::{AlpacaConfig, TradingClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file if present
    dotenv::dotenv().ok();

    // Create configuration from environment variables
    let config = AlpacaConfig::from_env()?;
    let client = TradingClient::new(config);

    println!("🕐 Market Clock Complete Demo");
    println!("============================\n");

    // Example 1: Basic market status check
    println!("1️⃣  Current Market Status");
    println!("------------------------");
    match client.get_clock().await {
        Ok(clock) => {
            println!("🕐 Server time: {}", clock.timestamp);

            if clock.is_open {
                println!("🟢 Market Status: OPEN");
                println!("🔚 Market closes at: {}", clock.next_close);
                println!("📈 You can place trades now!");
            } else {
                println!("🔴 Market Status: CLOSED");
                println!("🔜 Market opens at: {}", clock.next_open);
                println!("⏰ Trading is currently unavailable");
            }
        }
        Err(e) => eprintln!("❌ Error: {}", e),
    }
    println!();

    // Example 2: Trading hours information
    println!("2️⃣  Trading Hours Information");
    println!("-----------------------------");
    match client.get_clock().await {
        Ok(clock) => {
            println!("📅 Trading Schedule:");
            println!("   Next Open:  {}", clock.next_open);
            println!("   Next Close: {}", clock.next_close);

            // Parse times to show more user-friendly format
            if let Ok(next_open_parsed) = chrono::DateTime::parse_from_rfc3339(&clock.next_open) {
                println!(
                    "   Next Open (Local): {}",
                    next_open_parsed.format("%Y-%m-%d %I:%M %p %Z")
                );
            }
            if let Ok(next_close_parsed) = chrono::DateTime::parse_from_rfc3339(&clock.next_close) {
                println!(
                    "   Next Close (Local): {}",
                    next_close_parsed.format("%Y-%m-%d %I:%M %p %Z")
                );
            }
        }
        Err(e) => eprintln!("❌ Error: {}", e),
    }
    println!();

    // Example 3: Market timing for trading logic
    println!("3️⃣  Trading Logic Examples");
    println!("--------------------------");
    match client.get_clock().await {
        Ok(clock) => {
            if clock.is_open {
                println!("✅ Safe to execute trades");
                println!("✅ Real-time market data available");
                println!("✅ Orders will be processed immediately");

                // Calculate time until market close
                if let (Ok(current), Ok(close)) = (
                    chrono::DateTime::parse_from_rfc3339(&clock.timestamp),
                    chrono::DateTime::parse_from_rfc3339(&clock.next_close),
                ) {
                    let time_left = close.signed_duration_since(current);
                    let hours = time_left.num_hours();
                    let minutes = time_left.num_minutes() % 60;
                    println!("⏰ Time until market close: {}h {}m", hours, minutes);
                }
            } else {
                println!("⚠️  Market is closed - consider the following:");
                println!("   📋 Queue orders for next market open");
                println!("   📊 Review positions and prepare strategies");
                println!("   📈 Analyze after-hours price movements");

                // Calculate time until market opens
                if let (Ok(current), Ok(open)) = (
                    chrono::DateTime::parse_from_rfc3339(&clock.timestamp),
                    chrono::DateTime::parse_from_rfc3339(&clock.next_open),
                ) {
                    let time_until = open.signed_duration_since(current);
                    let hours = time_until.num_hours();
                    let minutes = time_until.num_minutes() % 60;

                    if hours < 0 || minutes < 0 {
                        println!("   🔄 Market opens today at: {}", clock.next_open);
                    } else {
                        println!("   ⏰ Market opens in: {}h {}m", hours, minutes);
                    }
                }
            }
        }
        Err(e) => eprintln!("❌ Error: {}", e),
    }
    println!();

    // Example 4: Weekend/Holiday detection
    println!("4️⃣  Market Schedule Analysis");
    println!("----------------------------");
    match client.get_clock().await {
        Ok(clock) => {
            if let (Ok(current), Ok(next_open)) = (
                chrono::DateTime::parse_from_rfc3339(&clock.timestamp),
                chrono::DateTime::parse_from_rfc3339(&clock.next_open),
            ) {
                let current_date = current.date_naive();
                let next_open_date = next_open.date_naive();

                if current_date == next_open_date {
                    if clock.is_open {
                        println!("📈 Regular trading day - market is open");
                    } else {
                        println!("🕐 Regular trading day - market opens later today");
                    }
                } else {
                    let days_diff = next_open_date
                        .signed_duration_since(current_date)
                        .num_days();
                    match days_diff {
                        1 => println!("🌙 Market closed today - opens tomorrow"),
                        2..=3 => println!(
                            "🏖️  Weekend - market opens on {}",
                            next_open_date.format("%A")
                        ),
                        _ => println!("🏛️  Extended closure - possibly a holiday period"),
                    }
                }
            }
        }
        Err(e) => eprintln!("❌ Error: {}", e),
    }

    println!("\n✅ Market clock demo completed!");
    Ok(())
}
