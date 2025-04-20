use tui::{
    backend::Backend,
    layout::Rect,
    style::Color,
    widgets::{Block, Borders, canvas::Canvas},
    Frame,
};

use crate::game::Game;
use crate::ui::colors;

pub fn draw_starmap<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    let block = Block::default()
        .title("Star Map")
        .borders(Borders::ALL);

    // Get the current system and all systems in the universe
    let current_system = &game.player.current_system;
    let all_systems = game.universe.get_all_systems();
    
    // Calculate the map boundaries with some padding
    let padding = 2.0;
    let mut min_x = current_system.x as f64 - game.player.ship.jump_range as f64 - padding;
    let mut max_x = current_system.x as f64 + game.player.ship.jump_range as f64 + padding;
    let mut min_y = current_system.y as f64 - game.player.ship.jump_range as f64 - padding;
    let mut max_y = current_system.y as f64 + game.player.ship.jump_range as f64 + padding;
    
    // Make sure we can see all nearby systems
    for system in &all_systems {
        if game.navigation_system.calculate_distance(current_system, system) <= game.player.ship.jump_range as f32 * 1.5 {
            min_x = min_x.min(system.x as f64 - padding);
            max_x = max_x.max(system.x as f64 + padding);
            min_y = min_y.min(system.y as f64 - padding);
            max_y = max_y.max(system.y as f64 + padding);
        }
    }

    // Draw the star map using Canvas
    let canvas = Canvas::default()
        .block(block)
        .x_bounds([min_x, max_x])
        .y_bounds([min_y, max_y])
        .paint(|ctx| {
            // Draw jump range circle
            // Create points in a circle
            let center_x = current_system.x as f64;
            let center_y = current_system.y as f64;
            let radius = game.player.ship.jump_range as f64;
            let circle_color = Color::Rgb(50, 50, 180);
            
            // Generate points around the circle
            let mut circle_points = Vec::new();
            for i in 0..60 {
                let angle = (i as f64) * 2.0 * std::f64::consts::PI / 60.0;
                let x = center_x + radius * angle.cos();
                let y = center_y + radius * angle.sin();
                circle_points.push((x, y));
            }
            
            // Draw circle
            ctx.draw(&tui::widgets::canvas::Points {
                coords: &circle_points,
                color: circle_color,
            });
            
            // Draw systems
            for system in &all_systems {
                let distance = game.navigation_system.calculate_distance(current_system, system);
                let in_range = distance <= game.player.ship.jump_range as f32;
                
                // Choose color based on system properties
                let color = if system.id == current_system.id {
                    colors::PRIMARY  // Current system
                } else if in_range {
                    if system.has_station {
                        colors::INFO // System with station in range
                    } else {
                        colors::NORMAL // System in range
                    }
                } else {
                    colors::DIM // System out of range
                };
                
                // Draw system point as colored dots
                ctx.draw(&tui::widgets::canvas::Points {
                    coords: &[(system.x as f64, system.y as f64)],
                    color: color,
                });
                
                // Draw system name if it's the current system or in range
                if system.id == current_system.id || in_range {
                    // For names, we can't set colors easily with the canvas print, 
                    // so we'll just print the name without color
                    ctx.print(
                        system.x as f64,
                        system.y as f64 - 0.5,
                        system.name.clone()
                    );
                }
            }
        });

    f.render_widget(canvas, area);
}
