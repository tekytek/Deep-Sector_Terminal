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
use crate::ui::ascii_art;
use crate::ui::screens::style_utils;

pub fn draw_ship_screen<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    // Split the screen into sections
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(area);

    // Draw ship ASCII art and basic info
    draw_ship_visual(f, game, chunks[0]);

    // Draw ship stats
    draw_ship_stats(f, game, chunks[1]);
}

fn draw_ship_visual<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    let vessel_title = format!("VESSEL: {}", game.player.ship.name);
    let block = style_utils::create_primary_block(&vessel_title);

    let ship_art = ascii_art::get_ship_art(&game.player.ship.ship_type);
    
    let mut text = vec![
        Spans::from(vec![
            Span::styled("Class: ", Style::default().fg(colors::DIM)),
            Span::styled(
                game.player.ship.ship_type.to_string(),
                Style::default().fg(colors::PRIMARY)
            ),
        ]),
        Spans::from(""),
    ];

    // Add ship ASCII art
    for line in ship_art.lines() {
        text.push(Spans::from(line.to_string()));
    }

    text.push(Spans::from(""));
    text.push(Spans::from(vec![
        Span::raw("["),
        Span::styled("M", Style::default().fg(colors::WARNING)),
        Span::raw("] Main Menu"),
    ]));

    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}

fn draw_ship_stats<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    let block = style_utils::create_info_block("TECHNICAL SPECIFICATIONS");

    let ship = &game.player.ship;

    let text = vec![
        Spans::from(vec![
            Span::styled("Hull: ", Style::default().fg(colors::DIM)),
            Span::styled(
                format!("{}/{}", ship.hull, ship.max_hull),
                Style::default().fg(colors::PRIMARY)
            ),
        ]),
        Spans::from(vec![
            Span::styled("Shield: ", Style::default().fg(colors::DIM)),
            Span::styled(
                format!("{}/{}", ship.shield, ship.max_shield),
                Style::default().fg(colors::INFO)
            ),
        ]),
        Spans::from(vec![
            Span::styled("Cargo Capacity: ", Style::default().fg(colors::DIM)),
            Span::styled(
                format!("{}/{} units", game.player.inventory.used_capacity(), ship.cargo_capacity),
                Style::default().fg(colors::NORMAL)
            ),
        ]),
        Spans::from(""),
        Spans::from(vec![
            Span::styled("Speed: ", Style::default().fg(colors::DIM)),
            Span::styled(
                format!("{} m/s", ship.speed),
                Style::default().fg(colors::NORMAL)
            ),
        ]),
        Spans::from(vec![
            Span::styled("Jump Range: ", Style::default().fg(colors::DIM)),
            Span::styled(
                format!("{} LY", ship.jump_range),
                Style::default().fg(colors::NORMAL)
            ),
        ]),
        Spans::from(""),
        Spans::from(vec![
            Span::styled("Weapons: ", Style::default().fg(colors::DIM)),
            Span::styled(
                format!("{}", ship.weapon_power),
                Style::default().fg(colors::DANGER)
            ),
        ]),
        Spans::from(vec![
            Span::styled("Mining Power: ", Style::default().fg(colors::DIM)),
            Span::styled(
                format!("{}", ship.mining_power),
                Style::default().fg(colors::WARNING)
            ),
        ]),
    ];

    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}
