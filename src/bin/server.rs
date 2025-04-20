use std::env;
use tokio::signal;
use dotenv::dotenv;

use space_trader::network::server::GameServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load .env file if available
    
    // Get server port from environment or use default
    let port = env::var("SERVER_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok());
    
    // Get server password from environment (optional)
    let password = env::var("SERVER_PASSWORD").ok();
    
    println!("Starting ASTR Space Trader Game Server");
    
    // Create and start the game server
    let server = GameServer::new(password).await;
    
    // Spawn server task
    let server_task = tokio::spawn(async move {
        if let Err(e) = server.start(port).await {
            eprintln!("Server error: {}", e);
        }
    });
    
    // Wait for interrupt signal to gracefully shutdown
    match signal::ctrl_c().await {
        Ok(()) => {
            println!("Shutting down server...");
        }
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
        }
    }
    
    // Cancel server task
    server_task.abort();
    
    println!("Server shutdown complete");
    
    Ok(())
}