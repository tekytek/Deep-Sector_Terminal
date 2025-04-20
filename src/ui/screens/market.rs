use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style},
    text::{Span, Spans},
    widgets::{Paragraph, Table, Row},
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
    
    // Draw comms with market info
    draw_comms(f, game, chunks[3]);
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

    // Enhanced header with trend information
    let header = if game.trading_system.is_buy_mode() {
        Row::new(vec!["#", "Item", "Quantity", "Price", "Trend"]).style(Style::default().fg(colors::INFO))
    } else {
        Row::new(vec!["#", "Item", "Quantity", "Sell Price"]).style(Style::default().fg(colors::INFO))
    };
    
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
            
            if game.trading_system.is_buy_mode() {
                // Get price trend info from market if available
                let trend_info = game.trading_system.get_price_trend_info(&item.name);
                let trend_text = match trend_info {
                    Some((_, trend)) => {
                        // Determine visual indicator and color based on trend
                        match trend.as_str() {
                            "Skyrocketing" => "▲▲▲", // Triple up arrow
                            "Rising" => "▲▲",       // Double up arrow
                            "Increasing" => "▲",    // Single up arrow
                            "Stable" => "◆",        // Diamond
                            "Decreasing" => "▼",    // Single down arrow
                            "Falling" => "▼▼",      // Double down arrow
                            "Plummeting" => "▼▼▼",  // Triple down arrow
                            _ => "◆",               // Default diamond
                        }
                    },
                    None => "◆" // Default if no trend
                };
                
                Row::new(vec![
                    format!("{}", i + 1),
                    item.name.clone(),
                    format!("{}", quantity),
                    format!("{} cr", price),
                    trend_text.to_string(),
                ])
            } else {
                // Simpler row for sell mode (player inventory)
                Row::new(vec![
                    format!("{}", i + 1),
                    item.name.clone(),
                    format!("{}", quantity),
                    format!("{} cr", price),
                ])
            }
        }).collect();

        let widths = if game.trading_system.is_buy_mode() {
            [
                Constraint::Length(3),         // #
                Constraint::Percentage(45),    // Item name
                Constraint::Percentage(15),    // Quantity
                Constraint::Percentage(20),    // Price
                Constraint::Percentage(15),    // Trend
            ]
        } else {
            [
                Constraint::Length(3),         // #
                Constraint::Percentage(55),    // Item name
                Constraint::Percentage(20),    // Quantity
                Constraint::Percentage(20),    // Price
                Constraint::Percentage(0),     // Hidden column (to match the 5-column structure)
            ]
        };

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

fn draw_comms<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    let block = style_utils::create_info_block("MARKET INFO");
    
    let market_type = match game.trading_system.is_buy_mode() {
        true => {
            // Get the market type for the current system
            // This would normally come from the universe market system
            let market_type = "Trading Hub"; // Placeholder
            let tax_rate = 5; // Placeholder tax rate percentage
            
            Spans::from(vec![
                Span::raw("Market Type: "),
                Span::styled(market_type, Style::default().fg(colors::INFO)),
                Span::raw("  |  Tax Rate: "),
                Span::styled(format!("{}%", tax_rate), Style::default().fg(colors::INFO)),
                Span::raw("  |  Press ["),
                Span::styled("1-9", Style::default().fg(colors::PRIMARY)),
                Span::raw("] to buy an item"),
            ])
        },
        false => {
            Spans::from(vec![
                Span::raw("SELLING FROM CARGO: "),
                Span::styled(format!("{}/{} units", game.player.inventory.used_capacity(), game.player.ship.cargo_capacity), 
                    Style::default().fg(colors::INFO)),
                Span::raw("  |  Press ["),
                Span::styled("1-9", Style::default().fg(colors::PRIMARY)),
                Span::raw("] to sell an item"),
            ])
        }
    };
    
    let paragraph = Paragraph::new(vec![market_type]).block(block);
    f.render_widget(paragraph, area);
}
