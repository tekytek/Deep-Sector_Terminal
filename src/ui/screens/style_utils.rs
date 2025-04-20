use tui::{
    style::Style,
    text::Span,
    widgets::{Block, Borders},
};
use crate::ui::colors;

/// Creates a primary styled block with sci-fi themed title
pub fn create_primary_block(title: &str) -> Block {
    Block::default()
        .title(Span::styled(format!(" {} ", title), Style::default().fg(colors::PRIMARY)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::SECONDARY))
}

/// Creates a secondary styled block with sci-fi themed title
pub fn create_info_block(title: &str) -> Block {
    Block::default()
        .title(Span::styled(format!(" {} ", title), Style::default().fg(colors::INFO)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::SECONDARY))
}

/// Creates a warning/danger styled block with sci-fi themed title
pub fn create_danger_block(title: &str) -> Block {
    Block::default()
        .title(Span::styled(format!(" {} ", title), Style::default().fg(colors::DANGER)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::DANGER))
}