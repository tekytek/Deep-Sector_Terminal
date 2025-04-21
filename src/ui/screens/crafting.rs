use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style},
    text::{Span, Spans},
    widgets::{Paragraph, Table, Row},
    Frame,
};

use crate::game::Game;
use crate::ui::colors;
use crate::ui::screens::style_utils;

pub fn draw_crafting_screen<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    // Check if player is docked at a station
    if !game.navigation_system.is_docked(&game.player) {
        draw_not_docked_message(f, area);
        return;
    }

    // Get available blueprints
    let blueprints = game.crafting_system.get_available_blueprints();
    
    if blueprints.is_empty() {
        draw_no_blueprints_message(f, area);
        return;
    }

    // Split the screen into sections
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),    // Blueprints
            Constraint::Length(5),  // Ingredients
            Constraint::Length(3),  // Player info
        ])
        .split(area);

    // Draw available blueprints
    draw_blueprints(f, game, blueprints, chunks[0]);

    // Draw selected blueprint ingredients
    draw_ingredients(f, game, chunks[1]);

    // Draw player crafting info
    draw_player_crafting_info(f, game, chunks[2]);
}

fn draw_not_docked_message<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let block = style_utils::create_danger_block("FABRICATION ACCESS DENIED");

    let text = vec![
        Spans::from(vec![
            Span::styled("You must be docked at a station to access crafting", Style::default().fg(colors::WARNING)),
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

fn draw_no_blueprints_message<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let block = style_utils::create_info_block("FABRICATION SCHEMATICS");

    let text = vec![
        Spans::from(vec![
            Span::styled("No blueprints available", Style::default().fg(colors::WARNING)),
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

fn draw_blueprints<B: Backend>(f: &mut Frame<B>, _game: &Game, blueprints: Vec<&crate::systems::crafting::Blueprint>, area: Rect) {
    let block = style_utils::create_primary_block("AVAILABLE SCHEMATICS");

    let header = Row::new(vec!["#", "Blueprint", "Rarity"]).style(Style::default().fg(colors::INFO));
    
    let rows: Vec<Row> = blueprints.iter().enumerate().map(|(i, blueprint)| {
        // Format rarity with appropriate color
        let rarity_str = match blueprint.rarity {
            crate::systems::crafting::BlueprintRarity::Common => "Common",
            crate::systems::crafting::BlueprintRarity::Uncommon => "Uncommon",
            crate::systems::crafting::BlueprintRarity::Rare => "Rare",
            crate::systems::crafting::BlueprintRarity::VeryRare => "Very Rare",
            crate::systems::crafting::BlueprintRarity::Exceptional => "Exceptional",
            crate::systems::crafting::BlueprintRarity::Legendary => "Legendary",
        };
        
        Row::new(vec![
            format!("{}", i + 1),
            blueprint.name.clone(),
            rarity_str.to_string(),
        ])
    }).collect();

    let widths = [
        Constraint::Length(3),
        Constraint::Percentage(70),
        Constraint::Percentage(20),
    ];

    let table = Table::new(rows)
        .header(header)
        .block(block)
        .widths(&widths);

    f.render_widget(table, area);
}

fn draw_ingredients<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    let block = style_utils::create_info_block("REQUIRED COMPONENTS");

    let selected_blueprint = game.crafting_system.get_selected_blueprint_index();
    
    let text = if let Some(_) = selected_blueprint {
        let selected_blueprint = game.crafting_system.get_selected_blueprint();
        
        if let Some(blueprint) = selected_blueprint {
            // Get the recipe for the blueprint
            let recipe_id = &blueprint.recipe_id;
            let mut spans = Vec::new();
            
            if let Some(recipe) = game.crafting_system.recipes.get(recipe_id) {
                spans.push(Spans::from(vec![
                    Span::styled(format!("Blueprint: {}", blueprint.name), Style::default().fg(colors::PRIMARY)),
                ]));
                spans.push(Spans::from("Requires:"));
                
                // Display ingredients from the recipe
                for (ingredient_name, amount) in &recipe.input_items {
                    let owned = game.player.inventory.get_item_quantity(ingredient_name);
                    let style = if owned >= *amount {
                        Style::default().fg(colors::PRIMARY)
                    } else {
                        Style::default().fg(colors::DANGER)
                    };
                    
                    spans.push(Spans::from(vec![
                        Span::raw(format!("- {} x ", ingredient_name)),
                        Span::styled(format!("{} ({}/{})", amount, owned, amount), style),
                    ]));
                }
                
                // Add rarity and other details
                spans.push(Spans::from(""));
                spans.push(Spans::from(vec![
                    Span::raw("Rarity: "),
                    Span::styled(format!("{:?}", blueprint.rarity), Style::default().fg(colors::INFO)),
                ]));
                
                spans
            } else {
                vec![Spans::from("Recipe data not found for this blueprint")]
            }
        } else {
            vec![Spans::from("Blueprint data not found")]
        }
    } else {
        vec![Spans::from("Select a blueprint to see ingredients")]
    };

    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}

fn draw_player_crafting_info<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    let block = style_utils::create_primary_block("FABRICATION STATION");

    let text = vec![
        Spans::from(vec![
            Span::raw("Cargo: "),
            Span::styled(
                format!("{}/{}", game.player.inventory.used_capacity(), game.player.ship.cargo_capacity),
                Style::default().fg(colors::INFO)
            ),
            Span::raw("    ["),
            Span::styled("M", Style::default().fg(colors::WARNING)),
            Span::raw("] Main Menu"),
        ]),
    ];

    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}
