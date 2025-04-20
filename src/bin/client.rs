use std::env;
use std::io::{self, Write};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use dotenv::dotenv;
use crossterm::event::{self, Event, KeyCode, KeyEvent};

use space_trader::network::client::GameClient;
use space_trader::game::{Game, GameScreen};
use space_trader::ui::app::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load .env file if available
    
    // Create the game (no need to load saves as they are on the server)
    let game = Game::new();
    let game_arc = Arc::new(Mutex::new(game));
    
    // Create the TUI app
    let app = App::new(game_arc.clone());
    
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
    
    match client.connect(&server_host, Some(port), password).await {
        Ok(_) => {
            println!("Connected to server. Starting game...");
            
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
            eprintln!("Failed to connect to server: {}", err);
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