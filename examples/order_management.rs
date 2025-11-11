use alpaca_markets::{
    AlpacaConfig,
    clients::trading::TradingClient,
    models::{AlpacaResult, OrderRequest, OrderSide, OrderTimeInForce, OrderType},
};

#[tokio::main]
async fn main() -> AlpacaResult<()> {
    println!("🚀 Alpaca Markets - Order Management Example");
    println!("============================================");

    // Create configuration from environment variables
    let config = AlpacaConfig::from_env()?;

    // Create trading client
    let trading_client = TradingClient::new(config);

    // Example 1: Get account information
    println!("\n💼 Account Information:");
    println!("-----------------------");

    match trading_client.get_account().await {
        Ok(account) => {
            println!("Account ID: {}", account.id);
            println!("Status: {:?}", account.status);
            println!("Cash: ${}", account.cash);
            println!("Buying Power: ${}", account.buying_power);
        }
        Err(e) => {
            println!("❌ Error fetching account: {:?}", e);
        }
    }

    // Example 2: Get all orders
    println!("\n📋 Current Orders:");
    println!("------------------");

    match trading_client.get_orders(Some("all"), Some(10)).await {
        Ok(orders) => {
            if orders.is_empty() {
                println!("No orders found.");
            } else {
                for order in orders.iter().take(5) {
                    println!(
                        "Order {}: {:?} {} {} @ {:?} - Status: {:?}",
                        order.id,
                        order.side,
                        order.qty.as_deref().unwrap_or("0"),
                        order.symbol,
                        order.order_type,
                        order.status
                    );
                }
            }
        }
        Err(e) => {
            println!("❌ Error fetching orders: {:?}", e);
        }
    }

    // Example 3: Get all positions
    println!("\n📊 Current Positions:");
    println!("--------------------");

    match trading_client.get_positions().await {
        Ok(positions) => {
            if positions.is_empty() {
                println!("No positions found.");
            } else {
                for position in positions {
                    println!(
                        "{}: {} shares @ ${} (Market Value: ${})",
                        position.symbol,
                        position.qty,
                        position.avg_entry_price,
                        position.market_value
                    );
                }
            }
        }
        Err(e) => {
            println!("❌ Error fetching positions: {:?}", e);
        }
    }

    // Example 4: Get individual position (if exists)
    println!("\n🎯 Individual Position Lookup:");
    match trading_client.get_position("AAPL").await {
        Ok(position) => {
            println!(
                "AAPL Position: {} shares @ ${} (P&L: ${})",
                position.qty, position.avg_entry_price, position.unrealized_pl
            );
        }
        Err(e) => {
            println!("No AAPL position found or error: {:?}", e);
        }
    }

    // Example 6: Demo order creation (commented for safety)
    println!("\n📝 Example Order Request Structure:");
    let example_order = OrderRequest {
        symbol: "AAPL".to_string(),
        qty: Some("1".to_string()),
        notional: None,
        side: OrderSide::Buy,
        order_type: OrderType::Limit,
        time_in_force: OrderTimeInForce::Day,
        limit_price: Some("150.00".to_string()),
        stop_price: None,
        extended_hours: Some(false),
        client_order_id: Some("my_custom_id_123".to_string()),
        order_class: None,
        take_profit: None,
        stop_loss: None,
        trail_price: None,
        trail_percent: None,
    };

    println!(
        "Order Request: {}",
        serde_json::to_string_pretty(&example_order).unwrap()
    );

    println!("\n✅ Order management example completed!");
    println!("💡 All new order management methods are now available in TradingClient");

    Ok(())
}
