mod game;
mod ui;
mod models;
mod systems;
mod utils;

use std::io;
use std::error::Error;
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
    // Setup terminal
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
