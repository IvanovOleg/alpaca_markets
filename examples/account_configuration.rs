use alpaca_markets::{
    AlpacaConfig,
    clients::trading::TradingClient,
    models::{
        AlpacaResult, DtbpCheck, PdtCheck, TradeConfirmEmail, UpdateAccountConfigurationRequest,
    },
};

#[tokio::main]
async fn main() -> AlpacaResult<()> {
    // Create configuration from environment variables
    let config = AlpacaConfig::from_env()?;

    // Create trading client
    let trading_client = TradingClient::new(config);

    println!("⚙️ Alpaca Account Configuration Management Example");
    println!("===============================================");

    // Example 1: Get current account configurations
    println!("\n📊 Current Account Configurations");
    println!("--------------------------------");

    match trading_client.get_account_configurations().await {
        Ok(config) => {
            println!("✅ Current account configuration:");
            println!("   Day Trade Buying Power Check: {:?}", config.dtbp_check);
            println!(
                "   Trade Confirmation Email: {:?}",
                config.trade_confirm_email
            );
            println!("   Suspend Trade: {}", config.suspend_trade);
            println!("   Max Margin Multiplier: {}", config.max_margin_multiplier);
            println!("   Pattern Day Trader Check: {:?}", config.pdt_check);
        }
        Err(e) => {
            println!("❌ Error fetching account configurations: {}", e);
        }
    }

    // Example 2: Update account configurations
    println!("\n🔧 Updating Account Configurations");
    println!("----------------------------------");

    // Create an update request with some configuration changes
    let update_request = UpdateAccountConfigurationRequest {
        dtbp_check: Some(DtbpCheck::Entry), // Enable day trade buying power check on entry
        trade_confirm_email: Some(TradeConfirmEmail::All), // Enable trade confirmation emails
        suspend_trade: Some(false),         // Ensure trading is not suspended
        max_margin_multiplier: None,        // Keep current value
        pdt_check: Some(PdtCheck::Entry),   // Enable PDT check on entry
    };

    match trading_client
        .update_account_configurations(update_request)
        .await
    {
        Ok(updated_config) => {
            println!("✅ Account configuration updated successfully!");
            println!(
                "   Updated Day Trade Buying Power Check: {:?}",
                updated_config.dtbp_check
            );
            println!(
                "   Updated Trade Confirmation Email: {:?}",
                updated_config.trade_confirm_email
            );
            println!("   Updated Suspend Trade: {}", updated_config.suspend_trade);
            println!(
                "   Updated Max Margin Multiplier: {}",
                updated_config.max_margin_multiplier
            );
            println!(
                "   Updated Pattern Day Trader Check: {:?}",
                updated_config.pdt_check
            );
        }
        Err(e) => {
            println!("❌ Error updating account configurations: {}", e);
        }
    }

    // Example 3: Demonstrate different configuration options
    println!("\n📋 Available Configuration Options");
    println!("---------------------------------");

    println!("🔸 Day Trade Buying Power Check (DTBP):");
    println!("   • None - No check performed");
    println!("   • Entry - Check only when entering a position");
    println!("   • Both - Check both entry and exit");
    println!();

    println!("🔸 Trade Confirmation Email:");
    println!("   • None - No confirmation emails");
    println!("   • All - Email confirmations for all trades");
    println!();

    println!("🔸 Pattern Day Trader Check (PDT):");
    println!("   • None - No PDT check");
    println!("   • Entry - Check only when entering a position");
    println!("   • Both - Check both entry and exit");
    println!();

    println!("🔸 Suspend Trade:");
    println!("   • true - All trading is suspended");
    println!("   • false - Trading is allowed");
    println!();

    println!("🔸 Max Margin Multiplier:");
    println!("   • String value representing the maximum margin multiplier");
    println!("   • Controls the maximum leverage available for margin trading");

    // Example 4: Selective updates (update only specific fields)
    println!("\n🎯 Selective Configuration Updates");
    println!("---------------------------------");

    // Example: Only update email settings
    let email_only_update = UpdateAccountConfigurationRequest {
        dtbp_check: None,
        trade_confirm_email: Some(TradeConfirmEmail::None), // Disable emails
        suspend_trade: None,
        max_margin_multiplier: None,
        pdt_check: None,
    };

    match trading_client
        .update_account_configurations(email_only_update)
        .await
    {
        Ok(config) => {
            println!("✅ Updated email configuration only:");
            println!(
                "   Trade Confirmation Email: {:?}",
                config.trade_confirm_email
            );
        }
        Err(e) => {
            println!("❌ Error updating email configuration: {}", e);
        }
    }

    // Example 5: Safety settings example
    println!("\n🛡️ Safety Settings Configuration");
    println!("-------------------------------");

    let safety_settings = UpdateAccountConfigurationRequest {
        dtbp_check: Some(DtbpCheck::Both), // Maximum protection for day trading
        trade_confirm_email: Some(TradeConfirmEmail::All), // Get all confirmations
        suspend_trade: None,               // Don't change current setting
        max_margin_multiplier: None,       // Don't change margin settings
        pdt_check: Some(PdtCheck::Both),   // Maximum PDT protection
    };

    match trading_client
        .update_account_configurations(safety_settings)
        .await
    {
        Ok(config) => {
            println!("✅ Safety settings configured:");
            println!(
                "   DTBP Check: {:?} (Maximum protection)",
                config.dtbp_check
            );
            println!(
                "   Trade Emails: {:?} (All confirmations)",
                config.trade_confirm_email
            );
            println!("   PDT Check: {:?} (Maximum protection)", config.pdt_check);
        }
        Err(e) => {
            println!("❌ Error configuring safety settings: {}", e);
        }
    }

    println!("\n✨ Account configuration management example completed!");

    Ok(())
}
