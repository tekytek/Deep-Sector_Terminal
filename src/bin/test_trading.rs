use std::time::Duration;
use std::thread;
use space_trader::models::universe::Universe;
use space_trader::systems::trading::TradingSystem;

fn main() {
    println!("Starting Automated Trading System Test");
    println!("=====================================");
    
    // Create universe and trading system
    let mut universe = Universe::new();
    let mut trading_system = TradingSystem::new();
    
    println!("\nRunning automated trading test for 60 seconds...");
    println!("(Market prices will update every 5 seconds)");
    println!("Press Ctrl+C to exit");
    println!();
    
    // Run for 60 seconds (12 price updates)
    for i in 1..=12 {
        println!("Test iteration {} (t+{}s)", i, i * 5);
        
        // Update the trading system to check for order execution
        let executed_orders = trading_system.update(&mut universe, Duration::from_secs(10)); // Force update
        
        // If no orders were executed this time, print a status message
        if executed_orders.is_empty() {
            println!("No trade orders executed in this iteration, waiting for price changes...");
        }
        
        // Wait for 5 seconds before next iteration
        thread::sleep(Duration::from_secs(5));
    }
    
    println!("\nTrading test complete");
}