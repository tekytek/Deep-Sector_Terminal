use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::game::Game;
use crate::ui::colors;

pub fn draw_status_bar<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),  // Player info
            Constraint::Percentage(25),  // Ship info
            Constraint::Percentage(25),  // Location
            Constraint::Percentage(25),  // Time
        ])
        .split(area);

    draw_player_info(f, game, chunks[0]);
    draw_ship_info(f, game, chunks[1]);
    draw_location_info(f, game, chunks[2]);
    draw_time_info(f, game, chunks[3]);
}

fn draw_player_info<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    let text = Spans::from(vec![
        Span::raw(format!("{}  ", game.player.character.name)),
        Span::styled(
            format!("Credits: {} cr", game.player.credits),
            Style::default().fg(colors::INFO),
        ),
    ]);

    let paragraph = Paragraph::new(text);
    f.render_widget(paragraph, area);
}

fn draw_ship_info<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    let ship = &game.player.ship;
    let text = Spans::from(vec![
        Span::raw(format!("{}  ", ship.name)),
        Span::styled(
            format!("Hull: {}/{}", ship.hull, ship.max_hull),
            Style::default().fg(colors::PRIMARY),
        ),
    ]);

    let paragraph = Paragraph::new(text);
    f.render_widget(paragraph, area);
}

fn draw_location_info<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    let system = &game.player.current_system;
    let location = if game.navigation_system.is_docked(&game.player) {
        format!("{} (Docked)", system.name)
    } else {
        format!("{} (Space)", system.name)
    };

    let text = Spans::from(vec![
        Span::raw("Location: "),
        Span::styled(location, Style::default().fg(colors::INFO)),
    ]);

    let paragraph = Paragraph::new(text);
    f.render_widget(paragraph, area);
}

fn draw_time_info<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    let game_time = game.time_system.get_formatted_time();
    
    let text = Spans::from(vec![
        Span::raw("Time: "),
        Span::styled(game_time, Style::default().fg(colors::NORMAL)),
    ]);

    let paragraph = Paragraph::new(text);
    f.render_widget(paragraph, area);
}
