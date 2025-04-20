pub mod colors;
pub mod screens;
pub mod widgets;
pub mod ascii_art;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::game::{Game, GameScreen};
use screens::{
    navigation::draw_navigation_screen,
    market::draw_market_screen,
    ship::draw_ship_screen,
    mining::draw_mining_screen,
    crafting::draw_crafting_screen,
    inventory::draw_inventory,
    help::draw_help,
    main_menu::draw_main_menu,
    character_creation::draw_character_creation,
    character_info::draw_character_info,
};
use widgets::status_bar::draw_status_bar;

pub fn draw<B: Backend>(f: &mut Frame<B>, game: &Game) {
    // For main menu and character creation, use the full screen
    match game.current_screen {
        GameScreen::MainMenu => {
            draw_main_menu(f, game, f.size());
            return;
        },
        GameScreen::CharacterCreation => {
            draw_character_creation(f, game, f.size());
            return;
        },
        _ => {}
    }

    // For other screens, create the standard layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),  // Status bar
            Constraint::Min(10),    // Main content
            Constraint::Length(3),  // Message area
        ])
        .split(f.size());

    // Draw the status bar
    draw_status_bar(f, game, chunks[0]);

    // Draw the main content based on the current screen
    match game.current_screen {
        GameScreen::MainMenu => {}, // Already handled
        GameScreen::CharacterCreation => {}, // Already handled
        GameScreen::Navigation => draw_navigation_screen(f, game, chunks[1]),
        GameScreen::Market => draw_market_screen(f, game, chunks[1]),
        GameScreen::Ship => draw_ship_screen(f, game, chunks[1]),
        GameScreen::Mining => draw_mining_screen(f, game, chunks[1]),
        GameScreen::Crafting => draw_crafting_screen(f, game, chunks[1]),
        GameScreen::Inventory => draw_inventory(f, game, chunks[1]),
        GameScreen::Character => draw_character_info(f, game, chunks[1]),
        GameScreen::Help => draw_help(f, game, chunks[1]),
        GameScreen::Quit => draw_quit_screen(f, game, chunks[1]),
    }

    // Draw message area
    draw_message_area(f, game, chunks[2]);
}

// Old screen functions removed and replaced with new module imports

fn draw_quit_screen<B: Backend>(f: &mut Frame<B>, _game: &Game, area: Rect) {
    use tui::widgets::{Block, Borders, Paragraph};
    use tui::text::{Span, Spans};
    use tui::style::{Style};
    use crate::ui::colors;

    let block = Block::default()
        .title(Span::styled(" QUIT ", Style::default().fg(colors::DANGER)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::DANGER));

    let text = vec![
        Spans::from(vec![
            Span::styled("Are you sure you want to quit?", Style::default().fg(colors::WARNING)),
        ]),
        Spans::from(""),
        Spans::from(vec![
            Span::raw("Press ["),
            Span::styled("Y", Style::default().fg(colors::PRIMARY)),
            Span::raw("] to confirm or ["),
            Span::styled("N", Style::default().fg(colors::DANGER)),
            Span::raw("] to cancel"),
        ]),
    ];

    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}

fn draw_message_area<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    use tui::widgets::{Block, Borders, Paragraph};
    use tui::text::{Span, Spans};
    use tui::style::{Style};
    use crate::ui::colors;

    let block = Block::default()
        .title(Span::styled(" COMMS ", Style::default().fg(colors::INFO)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::DIM));

    let text = if let Some(message) = &game.message {
        vec![
            Spans::from(vec![
                Span::styled(message, Style::default().fg(colors::INFO)),
            ]),
        ]
    } else {
        vec![Spans::from("")]
    };

    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}
