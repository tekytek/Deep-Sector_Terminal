use std::env;
use std::io::{self, Write};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::signal;
use tokio::time::interval;
use dotenv::dotenv;

// Import debugging utilities
use space_trader::debug::{self, LogLevel};
use space_trader::debug::network::NetworkDiagnostics;
use space_trader::debug::error_analysis;

use space_trader::game::Game;
use space_trader::models::universe::Universe;
use space_trader::network::server::GameServer;
use space_trader::network::protocol::Message;

// Initialize the debug system
fn init_debug_system() {
    // Initialize with default settings
    debug::init(
        Some("logs/server.log"),      // Log to a file
        true,                         // Also print to console
        debug::get_log_level_from_env(), // Get log level from env var or default to INFO
    );
    
    // Configure module-specific log levels
    debug::set_module_level("network", LogLevel::Debug);
    debug::set_module_level("game", LogLevel::Info);
    
    // Log initialization
    log_info!("Server debug system initialized");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the debugging system first
    init_debug_system();
    
    dotenv().ok(); // Load .env file if available
    log_info!("Server starting up");
    
    // Log system environment information
    log_debug!("System information:\n{}", debug::system_info());
    
    // Check network environment
    let network_report = NetworkDiagnostics::network_environment_report();
    log_debug!("Network environment report:\n{}", network_report);
    
    // Get server port from environment or use default
    let port_str = env::var("SERVER_PORT").unwrap_or_else(|_| {
        prompt("Enter server port (default: 7890): ")
            .unwrap_or_else(|_| "7890".to_string())
    });
    
    let port = port_str.parse::<u16>().unwrap_or(7890);
    
    // Get server password from environment (optional)
    let password = env::var("SERVER_PASSWORD").ok().or_else(|| {
        prompt("Set server password (leave empty for no password): ").ok()
            .filter(|s| !s.is_empty())
    });
    
    println!("Starting ASTR Space Trader Game Server on port {}", port);
    
    // Create game universe
    let mut game = Game::new();
    
    // Skip character creation in server mode
    if game.current_screen == space_trader::game::GameScreen::CharacterCreation {
        // Create default admin character
        let admin_player = space_trader::models::player::Player::with_character(
            "Server Admin",
            space_trader::models::faction::FactionType::Traders,
            space_trader::models::faction::Storyline::new(
                "server_admin",
                space_trader::models::faction::FactionType::Traders,
                "Server Administrator",
                "Manage the game universe and oversee player activities.",
                10
            )
        );
        
        game.player = admin_player;
        game.current_screen = space_trader::game::GameScreen::MainMenu;
    }
    
    // Create shared game state
    let game_state = Arc::new(Mutex::new(game));
    
    // Create and start the game server
    let server = GameServer::new(password, game_state.clone()).await;
    
    // Spawn server task
    let server_task = tokio::spawn(async move {
        if let Err(e) = server.start(Some(port)).await {
            eprintln!("Server error: {}", e);
        }
    });
    
    // Spawn universe simulation task (runs the game simulation on the server)
    let game_state_clone = game_state.clone();
    let universe_task = tokio::spawn(async move {
        let mut interval = interval(Duration::from_millis(100));
        
        loop {
            interval.tick().await;
            
            // Update game state
            let mut game = game_state_clone.lock().await;
            if let Err(e) = game.update() {
                eprintln!("Game update error: {}", e);
            }
            
            // Every 60 seconds, save the game state
            // This would be implemented with a proper timer
            
            // Drop the lock to prevent holding it too long
            drop(game);
        }
    });
    
    // Spawn REPL for server commands
    let game_state_clone = game_state.clone();
    let command_task = tokio::spawn(async move {
        println!("Server console ready. Type 'help' for commands.");
        
        loop {
            print!("server> ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                continue;
            }
            
            let input = input.trim().to_lowercase();
            
            match input.as_str() {
                "help" => {
                    println!("Available commands:");
                    println!("  status    - Show server status and connected players");
                    println!("  save      - Save game state");
                    println!("  clients   - List connected clients");
                    println!("  debug     - Show debug information and perform diagnostics");
                    println!("  stop      - Stop the server");
                    println!("  help      - Show this help");
                },
                "status" => {
                    let game = game_state_clone.lock().await;
                    println!("Server status: Running");
                    println!("Game time: {}", game.time_system.get_formatted_time());
                    println!("Universe size: {} star systems", game.universe.get_systems_count());
                    // Add more status info as needed
                },
                "save" => {
                    let mut game = game_state_clone.lock().await;
                    match game.save_state() {
                        Ok(_) => println!("Game state saved successfully."),
                        Err(e) => println!("Error saving game state: {}", e),
                    }
                },
                "clients" => {
                    println!("Connected clients: [Feature not implemented]");
                    // This would show connected clients in a real implementation
                },
                "debug" => {
                    println!("Debug menu:");
                    println!("1. System information");
                    println!("2. Network diagnostics");
                    println!("3. Error report");
                    println!("4. Connection test");
                    println!("5. Back to main menu");
                    print!("Select option: ");
                    io::stdout().flush().unwrap();
                    
                    let mut option = String::new();
                    if io::stdin().read_line(&mut option).is_err() {
                        println!("Invalid input");
                        continue;
                    }
                    
                    match option.trim() {
                        "1" => {
                            // System information
                            println!("{}", debug::system_info());
                        },
                        "2" => {
                            // Network diagnostics
                            println!("Running network diagnostics...");
                            println!("{}", NetworkDiagnostics::network_environment_report());
                            
                            // Check local port availability
                            let port = 7890; // Default port
                            let is_available = NetworkDiagnostics::check_port_available(port);
                            println!("Port {} is {}available", port, if is_available { "" } else { "not " });
                        },
                        "3" => {
                            // Error report
                            println!("Error Report:");
                            println!("{}", error_analysis::generate_error_report());
                        },
                        "4" => {
                            // Connection test
                            print!("Enter address to test (host:port): ");
                            io::stdout().flush().unwrap();
                            
                            let mut addr = String::new();
                            if io::stdin().read_line(&mut addr).is_err() {
                                println!("Invalid input");
                                continue;
                            }
                            
                            println!("Running connection test (this may take a few seconds)...");
                            
                            // This would be async in a production environment
                            // For now, we'll just run some basic checks
                            if let Some((host, port_str)) = addr.trim().split_once(':') {
                                if let Ok(port) = port_str.parse::<u16>() {
                                    let is_reachable = NetworkDiagnostics::check_port(host, port, 5000);
                                    println!("Host {}:{} is {}reachable", host, port, if is_reachable { "" } else { "not " });
                                } else {
                                    println!("Invalid port number");
                                }
                            } else {
                                println!("Invalid address format. Use host:port");
                            }
                        },
                        "5" | "" => {
                            println!("Returning to main menu");
                        },
                        _ => {
                            println!("Invalid option");
                        }
                    }
                },
                "stop" => {
                    println!("Stopping server...");
                    break;
                },
                "" => {
                    // Skip empty lines
                },
                _ => {
                    println!("Unknown command: {}", input);
                    println!("Type 'help' for available commands");
                }
            }
        }
    });
    
    // Wait for interrupt signal to gracefully shutdown
    let shutdown = async {
        match signal::ctrl_c().await {
            Ok(()) => {
                println!("Received interrupt signal.");
            }
            Err(err) => {
                eprintln!("Unable to listen for shutdown signal: {}", err);
            }
        }
    };
    
    // Wait for either Ctrl+C or the command interface to request shutdown
    tokio::select! {
        _ = shutdown => {},
        _ = command_task => {},
    }
    
    println!("Shutting down server...");
    
    // Cancel tasks
    server_task.abort();
    universe_task.abort();
    
    // Save final game state
    let mut game = game_state.lock().await;
    if let Err(e) = game.save_state() {
        eprintln!("Error saving final game state: {}", e);
    }
    
    println!("Server shutdown complete");
    
    Ok(())
}

fn prompt(text: &str) -> io::Result<String> {
    print!("{}", text);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    Ok(input.trim().to_string())
}