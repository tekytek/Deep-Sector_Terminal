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
};
use widgets::status_bar::draw_status_bar;

pub fn draw<B: Backend>(f: &mut Frame<B>, game: &Game) {
    // Create the main layout
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
        GameScreen::MainMenu => draw_main_menu(f, game, chunks[1]),
        GameScreen::Navigation => draw_navigation_screen(f, game, chunks[1]),
        GameScreen::Market => draw_market_screen(f, game, chunks[1]),
        GameScreen::Ship => draw_ship_screen(f, game, chunks[1]),
        GameScreen::Mining => draw_mining_screen(f, game, chunks[1]),
        GameScreen::Crafting => draw_crafting_screen(f, game, chunks[1]),
        GameScreen::Inventory => draw_inventory_screen(f, game, chunks[1]),
        GameScreen::Help => draw_help_screen(f, game, chunks[1]),
        GameScreen::Quit => draw_quit_screen(f, game, chunks[1]),
    }

    // Draw message area
    draw_message_area(f, game, chunks[2]);
}

fn draw_main_menu<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    use tui::widgets::{Block, Borders, Paragraph};
    use tui::text::{Span, Spans};
    use tui::style::{Style, Color};

    let title = format!("SPACE TRADER v0.1.0 - {}'s Ship", game.player.name);
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL);

    let text = vec![
        Spans::from(vec![
            Span::styled("Welcome to Space Trader", Style::default().fg(Color::Green)),
        ]),
        Spans::from(""),
        Spans::from(vec![
            Span::raw("["),
            Span::styled("N", Style::default().fg(Color::Yellow)),
            Span::raw("] Navigation"),
        ]),
        Spans::from(vec![
            Span::raw("["),
            Span::styled("M", Style::default().fg(Color::Yellow)),
            Span::raw("] Market"),
        ]),
        Spans::from(vec![
            Span::raw("["),
            Span::styled("S", Style::default().fg(Color::Yellow)),
            Span::raw("] Ship"),
        ]),
        Spans::from(vec![
            Span::raw("["),
            Span::styled("R", Style::default().fg(Color::Yellow)),
            Span::raw("] Mining"),
        ]),
        Spans::from(vec![
            Span::raw("["),
            Span::styled("C", Style::default().fg(Color::Yellow)),
            Span::raw("] Crafting"),
        ]),
        Spans::from(vec![
            Span::raw("["),
            Span::styled("I", Style::default().fg(Color::Yellow)),
            Span::raw("] Inventory"),
        ]),
        Spans::from(vec![
            Span::raw("["),
            Span::styled("H", Style::default().fg(Color::Yellow)),
            Span::raw("] Help"),
        ]),
        Spans::from(""),
        Spans::from(vec![
            Span::raw("["),
            Span::styled("Q", Style::default().fg(Color::Red)),
            Span::raw("] Quit"),
        ]),
    ];

    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}

fn draw_inventory_screen<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    use tui::widgets::{Block, Borders, Paragraph, Table, Row};
    use tui::text::{Span, Spans};
    use tui::style::{Style, Color};

    let block = Block::default()
        .title("Inventory")
        .borders(Borders::ALL);

    let header = Row::new(vec!["Item", "Quantity", "Value"]).style(Style::default().fg(Color::Yellow));
    
    let inventory_items: Vec<Row> = game.player.inventory.items.iter().map(|(item, quantity)| {
        Row::new(vec![
            item.name.clone(),
            quantity.to_string(),
            format!("{} cr", item.value),
        ])
    }).collect();

    if inventory_items.is_empty() {
        let text = vec![
            Spans::from(vec![
                Span::styled("Your cargo hold is empty", Style::default().fg(Color::Cyan)),
            ]),
            Spans::from(""),
            Spans::from(vec![
                Span::raw("Press ["),
                Span::styled("M", Style::default().fg(Color::Yellow)),
                Span::raw("] to return to the main menu"),
            ]),
        ];

        let paragraph = Paragraph::new(text).block(block);
        f.render_widget(paragraph, area);
    } else {
        let rows = inventory_items.clone();
        
        let widths = [
            Constraint::Percentage(50),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ];

        let table = Table::new(rows)
            .header(header)
            .block(block)
            .widths(&widths);

        f.render_widget(table, area);
    }
}

fn draw_help_screen<B: Backend>(f: &mut Frame<B>, _game: &Game, area: Rect) {
    use tui::widgets::{Block, Borders, Paragraph};
    use tui::text::{Span, Spans};
    use tui::style::{Style, Color};

    let block = Block::default()
        .title("Help")
        .borders(Borders::ALL);

    let text = vec![
        Spans::from(vec![
            Span::styled("SPACE TRADER - Game Controls", Style::default().fg(Color::Green)),
        ]),
        Spans::from(""),
        Spans::from("Navigation:"),
        Spans::from(" - Use number keys to travel to different star systems"),
        Spans::from(" - [D] to dock at a station"),
        Spans::from(" - [U] to undock from a station"),
        Spans::from(""),
        Spans::from("Market:"),
        Spans::from(" - [B] to enter buy mode"),
        Spans::from(" - [S] to enter sell mode"),
        Spans::from(" - Use number keys to select items to buy/sell"),
        Spans::from(""),
        Spans::from("Mining:"),
        Spans::from(" - Use number keys to select a resource to mine"),
        Spans::from(""),
        Spans::from("Crafting:"),
        Spans::from(" - Use number keys to select a blueprint to craft"),
        Spans::from(""),
        Spans::from("General:"),
        Spans::from(" - [M] to return to main menu from any screen"),
        Spans::from(" - [ESC] to cancel current action"),
        Spans::from(" - [Q] to quit the game"),
    ];

    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}

fn draw_quit_screen<B: Backend>(f: &mut Frame<B>, _game: &Game, area: Rect) {
    use tui::widgets::{Block, Borders, Paragraph};
    use tui::text::{Span, Spans};
    use tui::style::{Style, Color};

    let block = Block::default()
        .title("Quit")
        .borders(Borders::ALL);

    let text = vec![
        Spans::from(vec![
            Span::styled("Are you sure you want to quit?", Style::default().fg(Color::Red)),
        ]),
        Spans::from(""),
        Spans::from(vec![
            Span::raw("Press ["),
            Span::styled("Y", Style::default().fg(Color::Green)),
            Span::raw("] to confirm or ["),
            Span::styled("N", Style::default().fg(Color::Red)),
            Span::raw("] to cancel"),
        ]),
    ];

    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}

fn draw_message_area<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    use tui::widgets::{Block, Borders, Paragraph};
    use tui::text::{Span, Spans};
    use tui::style::{Style, Color};

    let block = Block::default()
        .title("Messages")
        .borders(Borders::ALL);

    let text = if let Some(message) = &game.message {
        vec![
            Spans::from(vec![
                Span::styled(message, Style::default().fg(Color::Cyan)),
            ]),
        ]
    } else {
        vec![Spans::from("")]
    };

    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}
