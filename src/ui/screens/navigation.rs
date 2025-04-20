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
use crate::ui::widgets::starmap::draw_starmap;

pub fn draw_navigation_screen<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    // Split the screen into two parts: starmap and info panel
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(70),
            Constraint::Percentage(30),
        ])
        .split(area);

    // Draw the starmap
    draw_starmap(f, game, chunks[0]);

    // Draw the navigation info panel
    draw_navigation_info(f, game, chunks[1]);
}

fn draw_navigation_info<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    let current_system = game.player.current_system.clone();
    let current_location = if game.navigation_system.is_docked(&game.player) {
        format!("{} (Docked)", current_system.name)
    } else {
        format!("{} (Space)", current_system.name)
    };

    let block = Block::default()
        .title(Span::styled(" NAVIGATION SYSTEMS ", Style::default().fg(colors::PRIMARY)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::SECONDARY));

    let mut text = vec![
        Spans::from(vec![
            Span::raw("Current Location: "),
            Span::styled(&current_location, Style::default().fg(colors::INFO)),
        ]),
        Spans::from(vec![
            Span::raw("Coordinates: "),
            Span::styled(
                format!("({}, {})", current_system.x, current_system.y),
                Style::default().fg(colors::INFO),
            ),
        ]),
        Spans::from(""),
        Spans::from(vec![
            Span::styled("Nearby Systems:", Style::default().fg(colors::PRIMARY)),
        ]),
    ];

    // Get nearby systems that are in range
    let nearby_systems = game.universe.get_nearby_systems(&current_system);
    
    if nearby_systems.is_empty() {
        text.push(Spans::from(vec![
            Span::styled("No systems in range", Style::default().fg(colors::WARNING)),
        ]));
    } else {
        for (i, system) in nearby_systems.iter().enumerate() {
            let distance = game.navigation_system.calculate_distance(&current_system, system);
            let travel_time = game.navigation_system.calculate_travel_time(distance);
            
            let in_range = game.navigation_system.is_in_range(&game.player, distance);
            let style = if in_range {
                Style::default().fg(colors::NORMAL)
            } else {
                Style::default().fg(colors::DIM)
            };
            
            text.push(Spans::from(vec![
                Span::styled(
                    format!("[{}] {} - ", i + 1, system.name),
                    style,
                ),
                Span::styled(
                    format!("{:.1} LY, {} mins", distance, travel_time.as_secs() / 60),
                    style,
                ),
            ]));
        }
    }

    text.push(Spans::from(""));

    // Add station information if there is one
    if current_system.has_station {
        if game.navigation_system.is_docked(&game.player) {
            text.push(Spans::from(vec![
                Span::raw("Station: "),
                Span::styled("Docked", Style::default().fg(colors::INFO)),
            ]));
            text.push(Spans::from(vec![
                Span::raw("["),
                Span::styled("U", Style::default().fg(colors::WARNING)),
                Span::raw("] Undock"),
            ]));
        } else {
            text.push(Spans::from(vec![
                Span::raw("Station: "),
                Span::styled(format!("{} Station", current_system.name), Style::default().fg(colors::INFO)),
            ]));
            text.push(Spans::from(vec![
                Span::raw("["),
                Span::styled("D", Style::default().fg(colors::WARNING)),
                Span::raw("] Dock"),
            ]));
        }
    } else {
        text.push(Spans::from(vec![
            Span::raw("Station: "),
            Span::styled("None", Style::default().fg(colors::DIM)),
        ]));
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
