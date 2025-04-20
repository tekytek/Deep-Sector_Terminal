use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style},
    text::{Span, Spans},
    widgets::{Paragraph},
    Frame,
};
use crate::game::Game;
use crate::ui::colors;
use crate::ui::screens::style_utils;

pub fn draw_inventory<B: Backend>(f: &mut Frame<B>, _game: &Game, area: Rect) {
    let block = style_utils::create_primary_block("CARGO MANIFEST");
    
    f.render_widget(block, area);
    
    // Placeholder for inventory screen - will be implemented later
    let inner = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Min(1)])
        .split(area)[0];
    
    let text = Spans::from(vec![
        Span::styled("Cargo manifest interface available in future update", 
                     Style::default().fg(colors::INFO))
    ]);
    
    let paragraph = Paragraph::new(text);
    f.render_widget(paragraph, inner);
}