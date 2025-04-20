use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::game::Game;
use crate::ui::colors;

pub fn draw_inventory<B: Backend>(f: &mut Frame<B>, _game: &Game, area: Rect) {
    let block = Block::default()
        .title(Span::styled(" Inventory ", Style::default().fg(colors::PRIMARY)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::SECONDARY));
    
    f.render_widget(block, area);
    
    // Placeholder for inventory screen - will be implemented later
    let inner = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Min(1)])
        .split(area)[0];
    
    let text = Spans::from(vec![
        Span::styled("Inventory screen coming soon", 
                     Style::default().fg(colors::NORMAL))
    ]);
    
    let paragraph = Paragraph::new(text);
    f.render_widget(paragraph, inner);
}