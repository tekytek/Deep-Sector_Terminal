use tui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, BorderType},
};
use crate::ui::colors;
use std::time::{Duration, Instant};
use crate::utils::serde::SerializableInstant;

// Create a global animation counter
static mut ANIMATION_COUNTER: u8 = 0;
static mut LAST_UPDATE: Option<SerializableInstant> = None;

// Update animation counter
pub fn update_animation() -> u8 {
    unsafe {
        let now = Instant::now();
        let serializable_now = SerializableInstant::from(now);
        
        if let Some(last) = LAST_UPDATE {
            let last_instant: Instant = last.into();
            if now.duration_since(last_instant) >= Duration::from_millis(200) {
                ANIMATION_COUNTER = (ANIMATION_COUNTER + 1) % 8;
                LAST_UPDATE = Some(serializable_now);
            }
        } else {
            LAST_UPDATE = Some(serializable_now);
        }
        
        ANIMATION_COUNTER
    }
}

/// Returns an animated border character for sci-fi effects
fn get_animated_border_char(counter: u8) -> &'static str {
    match counter % 4 {
        0 => "═",
        1 => "≡",
        2 => "≣",
        _ => "≡",
    }
}

/// Creates a stylized sci-fi title with optional status indicator
pub fn create_sci_fi_title(title: &str, status: Option<&str>) -> Spans<'static> {
    let counter = update_animation();
    let animated_char = get_animated_border_char(counter);
    
    let mut title_spans = vec![
        Span::styled(format!(" {} ", animated_char), Style::default().fg(colors::INFO)),
        Span::styled(format!(" {} ", title), Style::default()
            .fg(colors::PRIMARY).add_modifier(Modifier::BOLD)),
        Span::styled(format!(" {} ", animated_char), Style::default().fg(colors::INFO)),
    ];
    
    // Add status indicator if provided
    if let Some(status_text) = status {
        title_spans.push(Span::styled(" | ", Style::default().fg(colors::DIM)));
        title_spans.push(Span::styled(status_text.to_string(), Style::default().fg(colors::WARNING)));
    }
    
    Spans::from(title_spans)
}

/// Creates a primary styled block with enhanced sci-fi themed title
pub fn create_primary_block(title: &str) -> Block {
    // Update animation for the title
    update_animation();
    
    Block::default()
        .title(create_sci_fi_title(title, None))
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::SECONDARY))
}

/// Creates a secondary styled block with enhanced sci-fi themed title
pub fn create_info_block(title: &str) -> Block {
    // Update animation for the title
    update_animation();
    
    Block::default()
        .title(create_sci_fi_title(title, None))
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::INFO))
}

/// Creates a warning/danger styled block with enhanced sci-fi themed title
pub fn create_danger_block(title: &str) -> Block {
    // Update animation for the title
    update_animation();
    
    Block::default()
        .title(create_sci_fi_title(title, None))
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Double)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::DANGER))
}

/// Creates a status bar styled block with minimal borders
#[allow(dead_code)]
pub fn create_status_bar(title: &str) -> Block {
    Block::default()
        .title(Span::styled(format!(" {} ", title), Style::default().fg(colors::DIM)))
        .borders(Borders::TOP | Borders::BOTTOM)
        .border_style(Style::default().fg(colors::DIM))
}

/// Formats a menu option with highlighted key and description
#[allow(dead_code)]
pub fn format_menu_option(key: char, label: &str, is_selected: bool) -> Spans<'static> {
    let style = if is_selected {
        Style::default().fg(colors::PRIMARY)
    } else {
        Style::default().fg(colors::WARNING)
    };
    
    Spans::from(vec![
        Span::raw("["),
        Span::styled(key.to_string(), style),
        Span::raw("] "),
        Span::styled(label.to_string(), Style::default().fg(colors::NORMAL)),
    ])
}

/// Creates a gauge bar for displaying resources like hull, shield, etc.
#[allow(dead_code)]
pub fn create_gauge_text(label: &str, current: u32, max: u32, color: Color) -> Spans<'static> {
    let percentage = current as f32 / max as f32;
    let bar_width = 10;
    let filled_width = (percentage * bar_width as f32).round() as usize;
    
    let mut bar = String::with_capacity(bar_width);
    for i in 0..bar_width {
        if i < filled_width {
            bar.push('█');
        } else {
            bar.push('░');
        }
    }
    
    Spans::from(vec![
        Span::styled(format!("{}: ", label), Style::default().fg(colors::DIM)),
        Span::styled(bar, Style::default().fg(color)),
        Span::styled(format!(" {}/{}", current, max), Style::default().fg(colors::NORMAL)),
    ])
}