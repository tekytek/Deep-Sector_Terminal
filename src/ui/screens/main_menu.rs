use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, BorderType, Wrap},
    Frame,
};
use crate::game::{Game, GameScreen};
use crate::ui::{colors, ascii_art};

// Animation constants
const MENU_ANIMATION_SPEED: u128 = 150; // ms per frame
const BLINK_SPEED: u128 = 500; // ms per blink

pub fn draw_main_menu<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    // Create animated stars
    let stars = ascii_art::get_star_field(&game.last_update, game.animation_frame);
    
    // Use a Canvas widget to draw stars instead of direct buffer access
    use tui::widgets::canvas::{Canvas, Points};
    
    // Create a canvas for the background stars
    let star_canvas = Canvas::default()
        .x_bounds([0.0, area.width as f64])
        .y_bounds([0.0, area.height as f64])
        .paint(|ctx| {
            // Draw stars using points
            let star_points: Vec<(f64, f64)> = stars.iter()
                .map(|(x, y, _)| (*x as f64, *y as f64))
                .collect();
            
            ctx.draw(&Points {
                coords: &star_points,
                color: colors::DIM,
            });
        });
    
    f.render_widget(star_canvas, area);
    
    // Split the screen into sections: title, menu, status
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),  // Title
            Constraint::Min(12),    // Menu
            Constraint::Length(4),  // Status
        ])
        .split(area);
    
    // Draw title with ASCII art
    let title_art = ascii_art::get_title_art();
    let title_paragraph = Paragraph::new(title_art)
        .style(Style::default().fg(colors::PRIMARY))
        .alignment(Alignment::Center);
    f.render_widget(title_paragraph, chunks[0]);
    
    // Engine particles animation with Canvas
    let particles = ascii_art::get_engine_particles(&game.last_update);
    let particles_coords: Vec<(f64, f64)> = particles.iter()
        .map(|(x, y, _)| (*x as f64, *y as f64))
        .collect();
    
    let particle_canvas = Canvas::default()
        .x_bounds([0.0, area.width as f64])
        .y_bounds([0.0, area.height as f64])
        .paint(|ctx| {
            ctx.draw(&Points {
                coords: &particles_coords,
                color: colors::WARNING,
            });
        });
    
    f.render_widget(particle_canvas, chunks[0]);
    
    // Create menu block with sci-fi border
    let menu_block = Block::default()
        .title(Span::styled(" COMMAND CONSOLE ", Style::default().fg(colors::PRIMARY)))
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(colors::SECONDARY));
    
    // Menu content area
    let menu_area = menu_block.inner(chunks[1]);
    f.render_widget(menu_block, chunks[1]);
    
    // Create menu items with animation
    let menu_items = get_animated_menu_items(game);
    
    // Layout for menu items
    let menu_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1); menu_items.len() + 4])
        .margin(2)
        .split(menu_area);
    
    // Render each menu item
    for (i, item) in menu_items.iter().enumerate() {
        let menu_text = Paragraph::new(item.clone())
            .alignment(Alignment::Center);
        f.render_widget(menu_text, menu_layout[i + 2]);
    }
    
    // Draw status bar at the bottom
    let status_style = Style::default().fg(colors::INFO);
    let commander_info = format!(
        "Commander: {} | Ship: {} | Credits: {} | Location: {} | Time: {}", 
        game.player.character.name,
        game.player.ship.name,
        game.player.credits,
        game.player.current_system.name,
        game.time_system.get_formatted_time()
    );
    
    let status_block = Block::default()
        .borders(Borders::TOP)
        .border_style(Style::default().fg(colors::DIM));
    
    let status_text = Paragraph::new(Spans::from(vec![
        Span::styled(commander_info, status_style)
    ]))
    .block(status_block)
    .alignment(Alignment::Center)
    .wrap(Wrap { trim: true });
    
    f.render_widget(status_text, chunks[2]);
}

fn get_animated_menu_items(game: &Game) -> Vec<Spans> {
    let elapsed_ms = game.last_update.elapsed().as_millis();
    let mut menu_items = Vec::new();
    
    // Define menu options with their hotkeys
    let options = vec![
        ("[N]", "Navigation", GameScreen::Navigation),
        ("[M]", "Market", GameScreen::Market),
        ("[S]", "Ship", GameScreen::Ship),
        ("[R]", "Mining", GameScreen::Mining),
        ("[C]", "Crafting", GameScreen::Crafting),
        ("[I]", "Inventory", GameScreen::Inventory),
        ("[P]", "Character Profile", GameScreen::Character),
        ("[H]", "Help", GameScreen::Help),
        ("[Q]", "Quit", GameScreen::Quit),
    ];
    
    // Animation logic - different items animate at different times
    for (i, (key, text, screen)) in options.iter().enumerate() {
        // Calculate animation phase for this item (offset by index)
        let phase = (elapsed_ms + (i as u128 * MENU_ANIMATION_SPEED)) % (MENU_ANIMATION_SPEED * 4);
        
        // Colors based on selection and animation phase
        let (key_color, text_color) = if *screen == game.current_screen {
            // Selected item - cycle through colors
            match phase / MENU_ANIMATION_SPEED {
                0 => (colors::HIGHLIGHT, colors::PRIMARY),
                1 => (colors::PRIMARY, colors::HIGHLIGHT),
                2 => (colors::HIGHLIGHT, colors::PRIMARY),
                _ => (colors::PRIMARY, colors::HIGHLIGHT),
            }
        } else {
            // Blink effect for non-selected items
            let blink_phase = elapsed_ms % (BLINK_SPEED * 2);
            if blink_phase < BLINK_SPEED {
                (colors::SECONDARY, colors::NORMAL)
            } else {
                (colors::INFO, colors::DIM)
            }
        };
        
        // Build the styled menu item
        let styled_item = Spans::from(vec![
            Span::styled(*key, Style::default().fg(key_color).add_modifier(Modifier::BOLD)),
            Span::raw(" "),
            Span::styled(*text, Style::default().fg(text_color)),
        ]);
        
        menu_items.push(styled_item);
    }
    
    menu_items
}