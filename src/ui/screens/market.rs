use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Table, Row},
    Frame,
};

use crate::game::Game;
use crate::ui::colors;
use crate::ui::screens::style_utils;

pub fn draw_market_screen<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    // Check if player is docked at a station
    if !game.navigation_system.is_docked(&game.player) {
        draw_not_docked_message(f, area);
        return;
    }

    // Split the screen into sections
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Mode selection
            Constraint::Min(10),    // Market items
            Constraint::Length(3),  // Player info
            Constraint::Length(3),  // Comms
        ])
        .split(area);

    // Draw mode selection
    draw_market_mode(f, game, chunks[0]);

    // Draw market items
    draw_market_items(f, game, chunks[1]);

    // Draw player info
    draw_player_market_info(f, game, chunks[2]);
    
    // Draw comms
    draw_comms(f, chunks[3]);
}

fn draw_not_docked_message<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let block = style_utils::create_danger_block("MARKET ACCESS DENIED");

    let text = vec![
        Spans::from(vec![
            Span::styled("You must be docked at a station to access the market", Style::default().fg(colors::WARNING)),
        ]),
        Spans::from(""),
        Spans::from(vec![
            Span::raw("Press ["),
            Span::styled("M", Style::default().fg(colors::WARNING)),
            Span::raw("] to return to the main menu"),
        ]),
    ];

    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}

fn draw_market_mode<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    let block = style_utils::create_primary_block("TRADING CONSOLE");

    let is_buy_mode = game.trading_system.is_buy_mode();

    let text = vec![
        Spans::from(vec![
            Span::raw("["),
            Span::styled("B", Style::default().fg(if is_buy_mode { colors::PRIMARY } else { colors::WARNING })),
            Span::raw("] Buy Items    ["),
            Span::styled("S", Style::default().fg(if !is_buy_mode { colors::PRIMARY } else { colors::WARNING })),
            Span::raw("] Sell Items    ["),
            Span::styled("M", Style::default().fg(colors::WARNING)),
            Span::raw("] Main Menu"),
        ]),
    ];

    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}

fn draw_market_items<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    let title = if game.trading_system.is_buy_mode() {
        "AVAILABLE MERCHANDISE"
    } else {
        "CARGO MANIFEST"
    };

    let block = style_utils::create_primary_block(title);

    let header = Row::new(vec!["#", "Item", "Quantity", "Price"]).style(Style::default().fg(colors::INFO));
    
    let items = if game.trading_system.is_buy_mode() {
        // Get market items for current system
        let current_system = &game.player.current_system;
        game.universe.get_market_items_for_system(current_system.id.clone())
    } else {
        // Get player's inventory items
        game.player.inventory.items.iter().map(|(item, quantity)| {
            (item.clone(), *quantity)
        }).collect()
    };

    if items.is_empty() {
        let text = vec![
            Spans::from(vec![
                Span::styled(
                    if game.trading_system.is_buy_mode() {
                        "No items available for purchase"
                    } else {
                        "Your cargo hold is empty"
                    },
                    Style::default().fg(colors::DIM)
                ),
            ]),
        ];

        let paragraph = Paragraph::new(text).block(block);
        f.render_widget(paragraph, area);
    } else {
        let rows: Vec<Row> = items.iter().enumerate().map(|(i, (item, quantity))| {
            let price = if game.trading_system.is_buy_mode() {
                item.value
            } else {
                // Sell price is slightly lower than buy price
                (item.value as f32 * 0.9) as u32
            };
            
            Row::new(vec![
                format!("{}", i + 1),
                item.name.clone(),
                format!("{}", quantity),
                format!("{} cr", price),
            ])
        }).collect();

        let widths = [
            Constraint::Length(3),
            Constraint::Percentage(50),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
        ];

        let table = Table::new(rows)
            .header(header)
            .block(block)
            .widths(&widths);

        f.render_widget(table, area);
    }
}

fn draw_player_market_info<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    let block = style_utils::create_info_block("FINANCIAL STATUS");

    let text = vec![
        Spans::from(vec![
            Span::raw("Credits: "),
            Span::styled(format!("{} cr", game.player.credits), Style::default().fg(colors::INFO)),
            Span::raw("    Cargo: "),
            Span::styled(
                format!("{}/{}", game.player.inventory.used_capacity(), game.player.ship.cargo_capacity),
                Style::default().fg(colors::INFO)
            ),
        ]),
    ];

    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}

fn draw_comms<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let block = style_utils::create_info_block("COMMS");
    
    // Empty for now, can be used for market messages or notifications
    let text = vec![
        Spans::from(""),
    ];
    
    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}
