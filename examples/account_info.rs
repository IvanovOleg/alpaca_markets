use alpaca_markets::{AlpacaConfig, AlpacaResult, TradingClient};

#[tokio::main]
async fn main() -> AlpacaResult<()> {
    // Load configuration from environment variables
    let config = AlpacaConfig::from_env()?;

    // Create a trading client
    let trading_client = TradingClient::new(config);

    // Get account information
    match trading_client.get_account().await {
        Ok(account) => {
            println!("Account ID: {}", account.id);
            println!("Status: {:?}", account.status);
            println!("Buying Power: {}", account.buying_power);
            println!("Cash: {}", account.cash);
            println!("Portfolio Value: {}", account.portfolio_value);
        }
        Err(e) => {
            eprintln!("Error fetching account: {:?}", e);
        }
    }

    Ok(())
}
