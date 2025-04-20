use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Table, Row},
    Frame,
};

use crate::game::Game;
use crate::ui::colors;
use crate::models::universe::ResourceField;

pub fn draw_mining_screen<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    // Check if player is in a system with mineable resources
    let current_system = &game.player.current_system;
    let resource_fields = game.mining_system.get_resources_for_system(&current_system.id);
    
    if resource_fields.is_empty() {
        draw_no_resources_message(f, area);
        return;
    }

    // Split the screen into sections
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),    // Resource Fields
            Constraint::Length(6),  // Active Operations
            Constraint::Length(3),  // Player info
        ])
        .split(area);

    // Draw available resource fields
    draw_resource_fields(f, game, &resource_fields, chunks[0]);
    
    // Draw active mining operations
    draw_active_operations(f, game, chunks[1]);

    // Draw player mining info
    draw_player_mining_info(f, game, chunks[2]);
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

fn draw_resource_fields<B: Backend>(f: &mut Frame<B>, _game: &Game, fields: &[ResourceField], area: Rect) {
    let block = Block::default()
        .title(Span::styled(" DETECTED RESOURCE FIELDS ", Style::default().fg(colors::PRIMARY)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::SECONDARY));

    let header = Row::new(vec!["#", "Field Type", "Size", "Resources", "Required Level"])
        .style(Style::default().fg(colors::INFO));
    
    let rows: Vec<Row> = fields.iter().enumerate().map(|(i, field)| {
        // List first few resources as a sample
        let resource_list = field.resources.iter()
            .take(2)
            .map(|(name, _)| name.clone())
            .collect::<Vec<String>>()
            .join(", ");
        
        let resource_text = if field.resources.len() > 2 {
            format!("{} (+{})", resource_list, field.resources.len() - 2)
        } else {
            resource_list
        };
        
        Row::new(vec![
            format!("{}", i + 1),
            field.field_type.to_string(),
            format!("{}", field.size),
            resource_text,
            format!("{}", field.field_type.required_mining_level()),
        ])
    }).collect();

    let widths = [
        Constraint::Length(3),
        Constraint::Percentage(30),
        Constraint::Length(6),
        Constraint::Percentage(40),
        Constraint::Length(7),
    ];

    let table = Table::new(rows)
        .header(header)
        .block(block)
        .widths(&widths);

    f.render_widget(table, area);
}

fn draw_active_operations<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    let block = Block::default()
        .title(Span::styled(" ACTIVE MINING OPERATIONS ", Style::default().fg(colors::SUCCESS)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::SECONDARY));

    let operations = game.mining_system.get_mining_status();
    
    if operations.is_empty() {
        let text = vec![
            Spans::from(vec![
                Span::raw("No active mining operations. Press "),
                Span::styled("1-9", Style::default().fg(colors::WARNING)),
                Span::raw(" to start mining a resource field."),
            ]),
        ];
        let paragraph = Paragraph::new(text).block(block);
        f.render_widget(paragraph, area);
    } else {
        let text: Vec<Spans> = operations.into_iter()
            .map(|op| Spans::from(vec![Span::raw(op)]))
            .collect();
        
        let paragraph = Paragraph::new(text).block(block);
        f.render_widget(paragraph, area);
    }
}

fn draw_player_mining_info<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    let block = Block::default()
        .title(Span::styled(" MINING EQUIPMENT ", Style::default().fg(colors::WARNING)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::SECONDARY));

    let text = vec![
        Spans::from(vec![
            Span::raw("Mining Power: "),
            Span::styled(format!("{}", game.player.ship.mining_power), Style::default().fg(colors::INFO)),
            Span::raw(" | Mining Level: "),
            Span::styled(format!("{}", game.player.skills.get_mining_level()), Style::default().fg(colors::INFO)),
            Span::raw(" | Press "),
            Span::styled("S", Style::default().fg(colors::WARNING)),
            Span::raw(" to stop mining"),
        ]),
    ];

    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}