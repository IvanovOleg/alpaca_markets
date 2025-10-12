use alpaca_markets::{AlpacaConfig, TradingClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file if present
    dotenv::dotenv().ok();

    // Create configuration from environment variables
    let config = AlpacaConfig::from_env()?;

    // Create trading client
    let client = TradingClient::new(config);

    println!("🗓️  Market Calendar Demo");
    println!("========================\n");

    // Example 1: Get all upcoming trading days (next few days)
    println!("1️⃣  Getting upcoming trading days...");
    match client.get_calendar(None, None).await {
        Ok(calendar) => {
            println!("   📅 Found {} upcoming trading days", calendar.len());
            if let Some(next_day) = calendar.first() {
                println!("   🔜 Next trading day: {}", next_day.date);
                println!("   🕘 Market opens: {}", next_day.open);
                println!("   🕕 Market closes: {}", next_day.close);
                println!("   📅 Settlement date: {}", next_day.settlement_date);
            }
        }
        Err(e) => eprintln!("   ❌ Error getting calendar: {}", e),
    }
    println!();

    // Example 2: Get calendar for a specific month (January 2024)
    println!("2️⃣  Getting calendar for January 2024...");
    match client
        .get_calendar(Some("2024-01-01"), Some("2024-01-31"))
        .await
    {
        Ok(calendar) => {
            println!("   📅 Trading days in January 2024: {}", calendar.len());

            // Show first few days
            for (i, day) in calendar.iter().take(5).enumerate() {
                println!(
                    "   {}: {} - Open: {}, Close: {}",
                    i + 1,
                    day.date,
                    day.open,
                    day.close
                );
            }

            if calendar.len() > 5 {
                println!("   ... and {} more trading days", calendar.len() - 5);
            }
        }
        Err(e) => eprintln!("   ❌ Error getting January calendar: {}", e),
    }
    println!();

    // Example 3: Get calendar for a specific week
    println!("3️⃣  Getting calendar for first week of December 2023...");
    match client
        .get_calendar(Some("2023-12-01"), Some("2023-12-08"))
        .await
    {
        Ok(calendar) => {
            println!(
                "   📅 Trading days in first week of December 2023: {}",
                calendar.len()
            );

            for day in &calendar {
                println!(
                    "   📊 {}: Open at {}, Close at {}",
                    day.date, day.open, day.close
                );
            }
        }
        Err(e) => eprintln!("   ❌ Error getting December calendar: {}", e),
    }
    println!();

    // Example 4: Check if a specific date is a trading day
    println!("4️⃣  Checking if 2024-07-04 (July 4th) is a trading day...");
    match client
        .get_calendar(Some("2024-07-04"), Some("2024-07-04"))
        .await
    {
        Ok(calendar) => {
            if calendar.is_empty() {
                println!("   ❌ July 4th, 2024 is NOT a trading day (Independence Day)");
            } else {
                let trading_day = &calendar[0];
                println!("   ✅ July 4th, 2024 IS a trading day!");
                println!("   🕘 Market opens: {}", trading_day.open);
                println!("   🕕 Market closes: {}", trading_day.close);
            }
        }
        Err(e) => eprintln!("   ❌ Error checking July 4th calendar: {}", e),
    }
    println!();

    // Example 5: Calculate total trading days in 2023
    println!("5️⃣  Calculating total trading days in 2023...");
    match client
        .get_calendar(Some("2023-01-01"), Some("2023-12-31"))
        .await
    {
        Ok(calendar) => {
            println!("   📈 Total trading days in 2023: {}", calendar.len());

            // Count days by month
            use std::collections::HashMap;
            let mut monthly_counts: HashMap<&str, u32> = HashMap::new();

            for day in &calendar {
                // Extract month from YYYY-MM-DD format
                if let Some(month_str) = day.date.get(5..7) {
                    *monthly_counts.entry(month_str).or_insert(0) += 1;
                }
            }

            println!("   📊 Trading days by month in 2023:");
            for month_num in &[
                "01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12",
            ] {
                let month_name = match *month_num {
                    "01" => "Jan",
                    "02" => "Feb",
                    "03" => "Mar",
                    "04" => "Apr",
                    "05" => "May",
                    "06" => "Jun",
                    "07" => "Jul",
                    "08" => "Aug",
                    "09" => "Sep",
                    "10" => "Oct",
                    "11" => "Nov",
                    "12" => "Dec",
                    _ => "???",
                };
                let count = monthly_counts.get(month_num).unwrap_or(&0);
                println!("      {}: {} days", month_name, count);
            }
        }
        Err(e) => eprintln!("   ❌ Error getting 2023 calendar: {}", e),
    }

    println!("\n✅ Calendar demo completed!");
    Ok(())
}
