mod game;
mod ui;
mod models;
mod systems;
mod utils;
mod network;

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
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create game instance
    let mut game = Game::new();
    
    // Main game loop
    let res = run_game(&mut terminal, &mut game);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_game<B: tui::backend::Backend>(
    terminal: &mut Terminal<B>,
    game: &mut Game,
) -> Result<(), Box<dyn Error>> {
    loop {
        // Render UI
        terminal.draw(|f| ui::draw(f, game))?;

        // Handle input
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    if game.confirm_quit() {
                        // Save game state before exiting
                        game.save_state()?;
                        break;
                    }
                },
                KeyCode::Esc => game.cancel_action(),
                _ => game.handle_input(key),
            }
        }

        // Update game state
        game.update()?;
        
        // Check game over condition
        if game.is_game_over() {
            break;
        }
    }

    Ok(())
}
