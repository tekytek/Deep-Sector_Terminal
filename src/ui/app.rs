use std::io;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use crate::game::{Game, GameScreen};
use crate::ui::screens::{
    main_menu::draw_main_menu,
    navigation::draw_navigation_screen,
    market::draw_market_screen,
    ship::draw_ship_screen,
    mining::draw_mining_screen,
    crafting::draw_crafting_screen,
    inventory::draw_inventory_screen,
    help::draw_help_screen,
};

pub struct App {
    game: Arc<Mutex<Game>>,
}

impl App {
    pub fn new(game: Arc<Mutex<Game>>) -> Self {
        Self { game }
    }

    pub fn run(&self) -> Result<(), io::Error> {
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
            // Render the UI
            terminal.draw(|f| {
                // Get a read lock on the game state
                let game = self.game.blocking_lock();
                
                // Draw the appropriate screen based on game state
                match game.current_screen {
                    GameScreen::MainMenu => draw_main_menu(f, &game),
                    GameScreen::Navigation => draw_navigation_screen(f, &game),
                    GameScreen::Market => draw_market_screen(f, &game),
                    GameScreen::Ship => draw_ship_screen(f, &game),
                    GameScreen::Mining => draw_mining_screen(f, &game),
                    GameScreen::Crafting => draw_crafting_screen(f, &game),
                    GameScreen::Inventory => draw_inventory_screen(f, &game),
                    GameScreen::Help => draw_help_screen(f, &game),
                    GameScreen::Quit => {
                        // Draw quit confirmation
                        draw_main_menu(f, &game); // For now, just show the main menu as background
                    }
                }
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
                
                // Save game state periodically
                // In client-server mode, this would happen on the server
            }
        }

        // Restore terminal
        disable_raw_mode()?;
        terminal.backend_mut().execute(LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        Ok(())
    }
}