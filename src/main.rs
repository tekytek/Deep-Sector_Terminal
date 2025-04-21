mod game;
mod ui;
mod models;
mod systems;
mod utils;
mod network;
mod debug;

#[macro_use]
extern crate lazy_static;

// Import the debug module
use crate::debug::LogLevel;

use std::io::{self, Write};
use std::error::Error;
use std::env;
use std::process::Command;
use dotenv::dotenv;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    Terminal,
};

use game::Game;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize debugging system
    debug::init(
        Some("logs/game.log"),       // Log to a file
        true,                        // Also print to console
        debug::get_log_level_from_env(), // Get log level from env var or default to INFO
    );
    
    // Set module-specific log levels
    debug::set_module_level("network", LogLevel::Debug);
    debug::set_module_level("game", LogLevel::Info);
    
    debug::info("Starting ASTR Space Trader");
    debug::debug(&format!("System information:\n{}", debug::system_info()));
    
    // Load environment variables
    dotenv().ok();
    
    // Parse command line arguments for client/server mode
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        match args[1].as_str() {
            "server" => {
                println!("Starting in SERVER mode...");
                return Command::new("cargo")
                    .args(["run", "--bin", "server"])
                    .status()
                    .map_err(|e| Box::new(e) as Box<dyn Error>)
                    .map(|_| ());
            }
            "client" => {
                println!("Starting in CLIENT mode...");
                return Command::new("cargo")
                    .args(["run", "--bin", "client"])
                    .status()
                    .map_err(|e| Box::new(e) as Box<dyn Error>)
                    .map(|_| ());
            }
            _ => {
                // Continue with normal execution or prompt
            }
        }
    } else {
        // Show mode selection menu
        println!(" █████╗ ███████╗████████╗██████╗ ");
        println!("██╔══██╗██╔════╝╚══██╔══╝██╔══██╗");
        println!("███████║███████╗   ██║   ██████╔╝");
        println!("██╔══██║╚════██║   ██║   ██╔══██╗");
        println!("██║  ██║███████║   ██║   ██║  ██║");
        println!("╚═╝  ╚═╝╚══════╝   ╚═╝   ╚═╝  ╚═╝");
        println!("═══ SPACE TRADER CLIENT-SERVER ═══");
        println!();
        println!("Select mode:");
        println!("1. Start SERVER");
        println!("2. Start CLIENT");
        println!("3. Start STANDALONE (legacy mode)");
        print!("Enter choice (1-3): ");
        io::stdout().flush()?;
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        
        match choice.trim() {
            "1" => {
                // Launch server
                println!("Starting server...");
                return Command::new("cargo")
                    .args(["run", "--bin", "server"])
                    .status()
                    .map_err(|e| Box::new(e) as Box<dyn Error>)
                    .map(|_| ());
            }
            "2" => {
                // Launch client
                println!("Starting client...");
                return Command::new("cargo")
                    .args(["run", "--bin", "client"])
                    .status()
                    .map_err(|e| Box::new(e) as Box<dyn Error>)
                    .map(|_| ());
            }
            "3" => {
                // Continue with standalone mode
                println!("Starting in STANDALONE mode...");
            }
            _ => {
                println!("Invalid choice. Using STANDALONE mode by default.");
            }
        }
    }
    
    // Setup terminal for standalone mode
    debug::info("Initializing terminal UI");
    
    match enable_raw_mode() {
        Ok(_) => debug::debug("Raw mode enabled"),
        Err(e) => {
            debug::error(&format!("Failed to enable raw mode: {}", e));
            debug::error_analysis::record_error(
                "terminal", 
                "raw_mode_error", 
                &format!("Failed to enable raw mode: {}", e),
                None
            );
            return Err(Box::new(e));
        }
    }
    
    let mut stdout = io::stdout();
    
    if let Err(e) = execute!(stdout, EnterAlternateScreen, EnableMouseCapture) {
        debug::error(&format!("Failed to configure terminal: {}", e));
        disable_raw_mode()?; // Try to restore terminal state
        return Err(Box::new(e));
    }
    
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = match Terminal::new(backend) {
        Ok(t) => {
            debug::debug("Terminal UI initialized successfully");
            t
        },
        Err(e) => {
            debug::error(&format!("Failed to create terminal: {}", e));
            // Try to restore terminal state
            disable_raw_mode()?;
            return Err(Box::new(e));
        }
    };

    // Create game instance
    debug::info("Creating new game instance");
    let mut game = Game::new();
    
    // Main game loop
    debug::info("Starting game main loop");
    let res = run_game(&mut terminal, &mut game);

    // Restore terminal
    debug::debug("Cleaning up terminal");
    
    let cleanup_result = (|| -> Result<(), Box<dyn Error>> {
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;
        Ok(())
    })();
    
    if let Err(e) = cleanup_result {
        debug::error(&format!("Failed to restore terminal: {}", e));
    }

    // Handle any game loop errors
    if let Err(err) = res {
        debug::error(&format!("Game terminated with error: {:?}", err));
        println!("Error: {:?}", err);
        
        debug::error_analysis::record_error(
            "game", 
            "fatal_error", 
            &format!("Game loop terminated with error: {:?}", err),
            None
        );
    }
    
    debug::info("Game session ended");

    Ok(())
}

fn run_game<B: tui::backend::Backend>(
    terminal: &mut Terminal<B>,
    game: &mut Game,
) -> Result<(), Box<dyn Error>> {
    debug::info("Starting game main loop in STANDALONE mode");
    
    // Main game loop
    loop {
        // Render UI with error handling
        match terminal.draw(|f| ui::draw(f, game)) {
            Ok(_) => {
                // UI rendered successfully
            },
            Err(e) => {
                // Record UI rendering error
                let mut context = std::collections::HashMap::new();
                context.insert("screen".to_string(), format!("{:?}", game.current_screen));
                context.insert("terminal_size".to_string(), format!("{:?}", terminal.size().unwrap_or_default()));
                
                debug::error(&format!("UI rendering error: {}", e));
                debug::error_analysis::record_error(
                    "ui", 
                    "render_error", 
                    &format!("Failed to render UI: {}", e),
                    Some(context)
                );
                
                return Err(Box::new(e));
            }
        }
        
        // Handle input with error analysis
        match event::read() {
            Ok(Event::Key(key)) => {
                match key.code {
                    KeyCode::Char('q') => {
                        if game.confirm_quit() {
                            debug::info("Player requested quit, saving game state");
                            // Save game state before exiting
                            if let Err(e) = game.save_state() {
                                debug::error(&format!("Failed to save game state during exit: {}", e));
                            }
                            break;
                        }
                    },
                    KeyCode::Esc => {
                        game.cancel_action();
                        debug::debug("Player canceled current action");
                    },
                    KeyCode::Char('d') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                        // Secret debug key combination
                        debug::debug("Debug mode activated");
                        let report = debug::error_analysis::generate_error_report();
                        debug::info(&format!("Error Report:\n{}", report));
                    },
                    _ => {
                        game.handle_input(key);
                    },
                }
            },
            Ok(_) => {},  // Ignore other events
            Err(e) => {
                debug::error(&format!("Input error: {}", e));
                debug::error_analysis::record_error(
                    "input", 
                    "read_error", 
                    &format!("Failed to read input: {}", e),
                    None
                );
            }
        }

        // Update game state with error handling
        if let Err(e) = game.update() {
            debug::error(&format!("Game update error: {}", e));
            debug::error_analysis::record_error(
                "game", 
                "update_error", 
                &format!("Failed to update game state: {}", e),
                None
            );
            
            // Show error to player (would be better with a proper UI)
            terminal.draw(|f| {
                let size = f.size();
                let error_message = format!("Error: {}", e);
                let error_widget = tui::widgets::Paragraph::new(error_message)
                    .style(tui::style::Style::default().fg(tui::style::Color::Red))
                    .alignment(tui::layout::Alignment::Center);
                f.render_widget(error_widget, size);
            })?;
            
            // Pause briefly to show the error
            std::thread::sleep(std::time::Duration::from_millis(2000));
        }
        
        // Check game over condition
        if game.is_game_over() {
            debug::info("Game over condition reached");
            break;
        }
        
        // Brief yield to prevent CPU hogging
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    debug::info("Exiting game main loop");
    Ok(())
}
