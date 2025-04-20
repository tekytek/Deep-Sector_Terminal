use std::env;
use std::io::{self, Write};
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use uuid::Uuid;
use dotenv::dotenv;

use space_trader::network::client::GameClient;
use space_trader::network::protocol::Message;
use space_trader::game::Game;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load .env file if available
    
    // Create the game (no need to load saves as they are on the server)
    let game = Game::new();
    let game_arc = Arc::new(Mutex::new(game));
    
    // Create message channels for network communication
    let (tx_network, rx_network) = mpsc::channel::<Message>(100);
    let (tx_ui, rx_ui) = mpsc::channel::<Message>(100);
    
    // Create the TUI app
    let mut app = space_trader::ui::app::App::new(game_arc.clone(), tx_network, rx_ui);
    
    // Get connection details - use environment variables if available
    let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| {
        prompt("Enter server host (default: localhost): ")
            .unwrap_or_else(|_| "localhost".to_string())
    });
    
    let port_str = env::var("SERVER_PORT").unwrap_or_else(|_| {
        prompt("Enter server port (default: 7890): ")
            .unwrap_or_else(|_| "7890".to_string())
    });
    
    let port = port_str.parse::<u16>().unwrap_or(7890);
    
    let username = env::var("USERNAME").unwrap_or_else(|_| {
        prompt("Enter your commander name: ")
            .unwrap_or_else(|_| format!("Commander-{}", Uuid::new_v4().as_simple()))
    });
    
    let password = env::var("SERVER_PASSWORD").ok().or_else(|| {
        prompt("Enter server password (leave empty for no password): ").ok()
            .filter(|s| !s.is_empty())
    });
    
    println!("Connecting to server at {}:{}...", server_host, port);
    
    // Create a client and connect to the server
    let mut client = GameClient::new(username.clone());
    
    // Try to connect to the server
    match client.connect(&server_host, Some(port), password).await {
        Ok(_) => {
            println!("Connected to server. Starting game in online mode...");
            
            // TODO: Set up game state synchronization with server
            
            // Start the TUI application
            if let Err(err) = app.run() {
                eprintln!("Application error: {}", err);
            }
            
            // Disconnect from server when done
            if let Err(err) = client.disconnect().await {
                eprintln!("Error disconnecting: {}", err);
            }
        }
        Err(err) => {
            println!("Failed to connect to server: {}", err);
            println!("Starting in OFFLINE mode. Your progress will not be synchronized.");
            
            // Load local game state if available
            {
                let mut game = game_arc.lock().await;
                if let Err(e) = game.load_game() {
                    println!("No saved game found or error loading: {}", e);
                    println!("Starting a new game...");
                }
            }
            
            // Start the TUI application in offline mode
            if let Err(err) = app.run() {
                eprintln!("Application error: {}", err);
            }
        }
    }
    
    Ok(())
}

fn prompt(text: &str) -> io::Result<String> {
    print!("{}", text);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    Ok(input.trim().to_string())
}