use tui::{
    backend::Backend,
    layout::Rect,
    style::Style,
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::ui::colors;

#[allow(dead_code)]
pub fn draw_menu<B: Backend>(
    f: &mut Frame<B>,
    title: &str,
    items: &[(&str, &str)],
    area: Rect,
) {
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL);

    let mut text = Vec::new();
    
    for (key, label) in items {
        text.push(Spans::from(vec![
            Span::raw("["),
            Span::styled(*key, Style::default().fg(colors::WARNING)),
            Span::raw("] "),
            Span::raw(*label),
        ]));
    }

    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}
