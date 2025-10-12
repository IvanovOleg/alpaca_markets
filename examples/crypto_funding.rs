use alpaca_markets::{AlpacaConfig, TradingClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file if present
    dotenv::dotenv().ok();

    // Create configuration from environment variables
    let config = AlpacaConfig::from_env()?;
    let client = TradingClient::new(config);

    println!("💰 Crypto Funding Management Demo");
    println!("=================================\n");

    // Example 1: List all crypto wallets
    println!("1️⃣  Listing Crypto Wallets");
    println!("---------------------------");
    match client.list_crypto_wallets(None).await {
        Ok(wallets) => {
            if wallets.is_empty() {
                println!("   📭 No crypto wallets found");
            } else {
                println!("   📊 Found {} crypto wallet(s):", wallets.len());
                for wallet in &wallets {
                    println!(
                        "   💳 Asset: {} | Balance: {} | Address: {}...",
                        wallet.asset,
                        wallet.balance,
                        &wallet.address[..std::cmp::min(wallet.address.len(), 20)]
                    );
                }
            }
        }
        Err(e) => eprintln!("   ❌ Error listing wallets: {}", e),
    }
    println!();

    // Example 2: Get BTC wallet specifically
    println!("2️⃣  Getting BTC Wallet Details");
    println!("-------------------------------");
    match client.list_crypto_wallets(Some("BTC")).await {
        Ok(btc_wallets) => {
            if btc_wallets.is_empty() {
                println!("   📭 No BTC wallet found - one will be created on first deposit");
            } else {
                let wallet = &btc_wallets[0];
                println!("   🅱️  BTC Wallet Details:");
                println!("      ID: {}", wallet.id);
                println!("      Address: {}", wallet.address);
                println!("      Balance: {} BTC", wallet.balance);
                println!("      Available: {} BTC", wallet.available_balance);
                println!("      Created: {}", wallet.created_at);
            }
        }
        Err(e) => eprintln!("   ❌ Error getting BTC wallet: {}", e),
    }
    println!();

    // Example 3: List crypto transfers
    println!("3️⃣  Listing Crypto Transfers");
    println!("-----------------------------");
    match client.list_crypto_transfers(None).await {
        Ok(transfers) => {
            if transfers.is_empty() {
                println!("   📭 No crypto transfers found");
            } else {
                println!("   📊 Found {} transfer(s):", transfers.len());
                for (i, transfer) in transfers.iter().take(5).enumerate() {
                    let direction_emoji = match transfer.direction.as_str() {
                        "incoming" => "⬇️",
                        "outgoing" => "⬆️",
                        _ => "↔️",
                    };
                    let status_emoji = match transfer.status.as_str() {
                        "completed" => "✅",
                        "pending" => "⏳",
                        "rejected" => "❌",
                        _ => "❓",
                    };

                    println!(
                        "   {} {} {}: {} {} {} ({} {})",
                        i + 1,
                        direction_emoji,
                        status_emoji,
                        transfer.amount,
                        transfer.asset,
                        transfer.direction,
                        transfer.status,
                        transfer.created_at.split('T').next().unwrap_or("")
                    );
                }
                if transfers.len() > 5 {
                    println!("   ... and {} more transfers", transfers.len() - 5);
                }
            }
        }
        Err(e) => eprintln!("   ❌ Error listing transfers: {}", e),
    }
    println!();

    // Example 4: List whitelisted addresses
    println!("4️⃣  Listing Whitelisted Addresses");
    println!("----------------------------------");
    match client.list_whitelisted_addresses(None).await {
        Ok(addresses) => {
            if addresses.is_empty() {
                println!("   📭 No whitelisted addresses found");
                println!("   💡 Add addresses to enable withdrawals (24h activation required)");
            } else {
                println!("   🔐 Found {} whitelisted address(es):", addresses.len());
                for addr in &addresses {
                    let status_emoji = match addr.status.as_str() {
                        "active" => "✅",
                        "pending" => "⏳",
                        _ => "❓",
                    };

                    println!(
                        "   {} {} ({}): {}...{}",
                        status_emoji,
                        addr.asset,
                        addr.label.as_deref().unwrap_or("No label"),
                        &addr.address[..std::cmp::min(addr.address.len(), 10)],
                        &addr.address[addr.address.len().saturating_sub(10)..]
                    );
                }
            }
        }
        Err(e) => eprintln!("   ❌ Error listing addresses: {}", e),
    }

    println!("\n✅ Crypto funding demo completed!");
    Ok(())
}
