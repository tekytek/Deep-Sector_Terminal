use std::io;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, mpsc};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use tui::{
    backend::CrosstermBackend,
    Terminal,
};

use crate::game::{Game, GameScreen};
use crate::network::protocol::Message;
use crate::ui::screens::{
    main_menu::draw_main_menu,
    navigation::draw_navigation_screen,
    market::draw_market_screen,
    ship::draw_ship_screen,
    mining::draw_mining_screen,
    crafting::draw_crafting_screen,
    inventory::draw_inventory,
    help::draw_help,
    character_info::draw_character_info,
};

#[allow(dead_code)]
pub struct App {
    game: Arc<Mutex<Game>>,
    #[allow(dead_code)]
    tx_network: mpsc::Sender<Message>,
    rx_ui: mpsc::Receiver<Message>,
    network_messages: Vec<String>, // Store recent messages from server
}

#[allow(dead_code)]
impl App {
    pub fn new(
        game: Arc<Mutex<Game>>, 
        tx_network: mpsc::Sender<Message>,
        rx_ui: mpsc::Receiver<Message>
    ) -> Self {
        Self { 
            game,
            tx_network,
            rx_ui,
            network_messages: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), io::Error> {
        // Terminal initialization
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        stdout.execute(EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // App loop
        let tick_rate = Duration::from_millis(100);
        let mut last_tick = Instant::now();
        
        loop {
            // Check for network messages (non-blocking)
            self.check_network_messages();
            
            // Render the UI
            terminal.draw(|f| {
                // Get a read lock on the game state
                let game = self.game.blocking_lock();
                
                // Draw the appropriate screen based on game state
                match game.current_screen {
                    GameScreen::MainMenu => draw_main_menu(f, &game, f.size()),
                    GameScreen::CharacterCreation => {
                        // Import and call the appropriate function
                        use crate::ui::screens::character_creation::draw_character_creation;
                        draw_character_creation(f, &game, f.size());
                    },
                    GameScreen::Navigation => draw_navigation_screen(f, &game, f.size()),
                    GameScreen::Market => draw_market_screen(f, &game, f.size()),
                    GameScreen::Ship => draw_ship_screen(f, &game, f.size()),
                    GameScreen::Mining => draw_mining_screen(f, &game, f.size()),
                    GameScreen::Crafting => draw_crafting_screen(f, &game, f.size()),
                    GameScreen::Inventory => draw_inventory(f, &game, f.size()),
                    GameScreen::Character => draw_character_info(f, &game, f.size()),
                    GameScreen::Orders => {
                        // Import and call the appropriate function
                        use crate::ui::screens::orders::draw_orders_screen;
                        draw_orders_screen(f, &game, f.size());
                    },
                    GameScreen::Help => draw_help(f, &game, f.size()),
                    GameScreen::Quit => {
                        // Draw quit confirmation
                        draw_main_menu(f, &game, f.size()); // For now, just show the main menu as background
                    }
                }
                
                // If there are network messages, they could be displayed in a popup or status area
                // This would be implemented in a separate UI component
            })?;

            // Handle input
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
                
            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    // Get a write lock on the game state
                    let mut game = self.game.blocking_lock();
                    
                    // Check if game is over
                    if key.code == KeyCode::Char('q') && game.current_screen != GameScreen::Quit {
                        if game.confirm_quit() {
                            break;
                        }
                    } else if key.code == KeyCode::Esc {
                        game.cancel_action();
                    } else {
                        // Handle the key event in the game
                        game.handle_input(key);
                        
                        // Send network messages based on game actions
                        // (This would be implemented based on the specific action)
                        // For example, if navigating to a new system:
                        if game.current_screen == GameScreen::Navigation {
                            // This is just an example - real implementation would check if navigation was requested
                            // self.send_navigation_request(&game.destination_system);
                        }
                        
                        // Check if game is over after handling input
                        if game.is_game_over() {
                            break;
                        }
                    }
                }
            }
            
            // Update game state
            if last_tick.elapsed() >= tick_rate {
                let mut game = self.game.blocking_lock();
                if let Err(e) = game.update() {
                    eprintln!("Game update error: {}", e);
                }
                last_tick = Instant::now();
            }
        }

        // Restore terminal
        disable_raw_mode()?;
        terminal.backend_mut().execute(LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        Ok(())
    }
    
    // Method to check for new network messages
    fn check_network_messages(&mut self) {
        // Poll for network messages without blocking
        if let Ok(msg) = self.rx_ui.try_recv() {
            match msg {
                Message::ActionResponse { success, message, .. } => {
                    self.network_messages.push(format!("{}: {}", 
                        if success { "Success" } else { "Error" },
                        message
                    ));
                    
                    // Limit the number of stored messages
                    if self.network_messages.len() > 10 {
                        self.network_messages.remove(0);
                    }
                },
                Message::Error { code, message } => {
                    self.network_messages.push(format!("Error {}: {}", code, message));
                    
                    // Limit the number of stored messages
                    if self.network_messages.len() > 10 {
                        self.network_messages.remove(0);
                    }
                },
                _ => {
                    // Handle other message types as needed
                }
            }
        }
    }
    
    // Method to send a navigation request to the server
    #[allow(dead_code)]
    fn send_navigation_request(&self, destination: &str) {
        let _game = self.game.blocking_lock(); // Using _ prefix to indicate intentional non-use
        
        // Create the navigation message
        // In a real implementation, you'd get the client_id from somewhere
        let client_id = uuid::Uuid::new_v4(); // Placeholder 
        let message = Message::NavigationAction {
            client_id,
            destination_system: destination.to_string(),
        };
        
        // Send the message (this is a non-blocking operation)
        let tx = self.tx_network.clone();
        tokio::spawn(async move {
            if let Err(e) = tx.send(message).await {
                eprintln!("Failed to send navigation request: {}", e);
            }
        });
    }
}