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

pub fn draw_mining_screen<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    // Check if player is in a system with mineable resources
    let current_system = &game.player.current_system;
    let resources = game.mining_system.get_resources_for_system(current_system.id.clone());
    
    if resources.is_empty() {
        draw_no_resources_message(f, area);
        return;
    }

    // Split the screen into sections
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),    // Resources
            Constraint::Length(3),  // Player info
        ])
        .split(area);

    // Draw available resources
    draw_resources(f, game, resources, chunks[0]);

    // Draw player mining info
    draw_player_mining_info(f, game, chunks[1]);
}

fn draw_no_resources_message<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let block = Block::default()
        .title(Span::styled(" MINING SCAN RESULTS ", Style::default().fg(colors::WARNING)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::SECONDARY));

    let text = vec![
        Spans::from(vec![
            Span::styled("No mineable resources in this system", Style::default().fg(colors::WARNING)),
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

fn draw_resources<B: Backend>(f: &mut Frame<B>, _game: &Game, resources: Vec<(String, u32)>, area: Rect) {
    let block = Block::default()
        .title(Span::styled(" DETECTED RESOURCE DEPOSITS ", Style::default().fg(colors::PRIMARY)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::SECONDARY));

    let header = Row::new(vec!["#", "Resource", "Abundance"]).style(Style::default().fg(colors::INFO));
    
    let rows: Vec<Row> = resources.iter().enumerate().map(|(i, (name, abundance))| {
        Row::new(vec![
            format!("{}", i + 1),
            name.clone(),
            format!("{}", abundance),
        ])
    }).collect();

    let widths = [
        Constraint::Length(3),
        Constraint::Percentage(50),
        Constraint::Percentage(30),
    ];

    let table = Table::new(rows)
        .header(header)
        .block(block)
        .widths(&widths);

    f.render_widget(table, area);
}

fn draw_player_mining_info<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    let block = Block::default()
        .title(Span::styled(" EXTRACTION EQUIPMENT ", Style::default().fg(colors::WARNING)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::SECONDARY));

    let text = vec![
        Spans::from(vec![
            Span::raw("Mining Power: "),
            Span::styled(format!("{}", game.player.ship.mining_power), Style::default().fg(colors::INFO)),
            Span::raw("    Cargo: "),
            Span::styled(
                format!("{}/{}", game.player.inventory.used_capacity(), game.player.ship.cargo_capacity),
                Style::default().fg(colors::INFO)
            ),
            Span::raw("    ["),
            Span::styled("M", Style::default().fg(colors::WARNING)),
            Span::raw("] Main Menu"),
        ]),
    ];

    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}
