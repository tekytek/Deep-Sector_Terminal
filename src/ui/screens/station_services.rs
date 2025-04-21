use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Table, Row},
    Frame,
};

use crate::game::Game;
use crate::ui::colors;
use crate::ui::screens::style_utils;
use crate::ui::ascii_art;

pub fn draw_station_services_screen<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    // Check if player is docked at a station
    if !game.navigation_system.is_docked(&game.player) {
        draw_not_docked_message(f, area);
        return;
    }

    // Split the screen into sections
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(area);

    // Draw station visual and info
    draw_station_visual(f, game, chunks[0]);

    // Draw services
    draw_station_services(f, game, chunks[1]);
}

fn draw_not_docked_message<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let block = style_utils::create_danger_block("STATION ACCESS DENIED");

    let text = vec![
        Spans::from(vec![
            Span::styled("You must be docked at a station to access services", 
                         Style::default().fg(colors::WARNING)),
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

fn draw_station_visual<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    let block = style_utils::create_primary_block("STATION");
    
    // Create a layout for the station art and details
    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Min(10),    // Station ASCII art
            Constraint::Min(5),     // Station details
        ])
        .split(area);
    
    // Draw the station
    let station_art = ascii_art::get_station_art();
    let paragraph = Paragraph::new(station_art);
    f.render_widget(paragraph, inner_chunks[0]);
    
    // Get current station details
    let current_system = &game.player.current_system;
    let station = current_system.stations.first(); // Simplification - assume first station
    
    let mut text = Vec::new();
    if let Some(station) = station {
        text.push(Spans::from(vec![
            Span::raw("Name: "),
            Span::styled(&station.name, Style::default().fg(colors::PRIMARY)),
        ]));
        let station_type = format!("{:?}", station.station_type);
        text.push(Spans::from(vec![
            Span::raw("Type: "),
            Span::styled(station_type, Style::default().fg(colors::INFO)),
        ]));
        text.push(Spans::from(vec![
            Span::raw("Status: "),
            Span::styled("Docked", Style::default().fg(colors::SUCCESS)),
        ]));
    } else {
        text.push(Spans::from(vec![
            Span::styled("Error: No station data", Style::default().fg(colors::DANGER)),
        ]));
    }
    
    let paragraph = Paragraph::new(text);
    f.render_widget(paragraph, inner_chunks[1]);
    f.render_widget(block, area);
}

fn draw_station_services<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    let block = style_utils::create_primary_block("AVAILABLE SERVICES");
    
    // Split the area for services list and service details
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),    // Services list
            Constraint::Min(5),     // Service details
        ])
        .split(area);
    
    // Get current station services
    let current_system = &game.player.current_system;
    let station = current_system.stations.first(); // Simplification - assume first station
    
    if let Some(station) = station {
        // Create a table of services
        let header = Row::new(vec!["#", "Service", "Status"])
            .style(Style::default().fg(colors::INFO));
        
        let mut rows = Vec::new();
        
        // Add common services
        if station.services.contains(&"Market".to_string()) {
            rows.push(Row::new(vec![
                "1".to_string(),
                "Market".to_string(),
                "Available".to_string(),
            ]));
        }
        
        if station.services.contains(&"Refueling".to_string()) {
            // Get ship fuel info
            let current_fuel = game.player.ship.current_fuel;
            let fuel_capacity = game.player.ship.fuel_capacity;
            
            let status = if current_fuel < fuel_capacity {
                format!("Available ({}/{})", current_fuel, fuel_capacity)
            } else {
                "Tank Full".to_string()
            };
            
            rows.push(Row::new(vec![
                "2".to_string(),
                "Refueling".to_string(),
                status,
            ]));
        }
        
        // Add other services based on station type
        let mut service_idx = 3;
        for service in &station.services {
            if service != "Market" && service != "Refueling" {
                rows.push(Row::new(vec![
                    service_idx.to_string(),
                    service.clone(),
                    "Available".to_string(),
                ]));
                service_idx += 1;
            }
        }
        
        let widths = [
            Constraint::Length(3),
            Constraint::Percentage(50),
            Constraint::Percentage(30),
        ];
        
        let services_table = Table::new(rows)
            .header(header)
            .widths(&widths)
            .block(Block::default());
        
        f.render_widget(services_table, chunks[0]);
        
        // Draw service details
        let details_text = vec![
            Spans::from(vec![
                Span::styled("Select a service by number:", Style::default().fg(colors::INFO)),
            ]),
            Spans::from(""),
            Spans::from(vec![
                Span::raw("["),
                Span::styled("2", Style::default().fg(colors::WARNING)),
                Span::raw("] Refuel your ship - "),
                Span::styled("25 credits per unit".to_string(), Style::default().fg(colors::PRIMARY)),
            ]),
            Spans::from(""),
            Spans::from(vec![
                Span::raw("["),
                Span::styled("M", Style::default().fg(colors::WARNING)),
                Span::raw("] Main Menu"),
            ]),
        ];
        
        let paragraph = Paragraph::new(details_text);
        f.render_widget(paragraph, chunks[1]);
    } else {
        let text = vec![
            Spans::from(vec![
                Span::styled("Error: No station data", Style::default().fg(colors::DANGER)),
            ]),
        ];
        
        let paragraph = Paragraph::new(text);
        f.render_widget(paragraph, chunks[0]);
    }
    
    f.render_widget(block, area);
}