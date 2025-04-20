use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, BorderType, Wrap, List, ListItem},
    Frame,
};
use crate::game::{Game, GameScreen};
use crate::ui::{colors, ascii_art};
use crate::models::faction::FactionType;

// Constants for faction and storyline descriptions
const FACTION_NAMES: [&str; 4] = ["Traders", "Miners", "Military", "Scientists"];
const FACTION_DESCRIPTIONS: [&str; 4] = [
    "Masters of commerce who control galactic trade routes and economies.",
    "Experts in resource extraction, operating in the harshest environments.",
    "Protectors of human space, maintaining order and security.",
    "Innovators who push the boundaries of technology and exploration."
];

// Ship class names by faction
const FACTION_SHIP_CLASSES: [&str; 4] = [
    "Merchant Vessel",
    "Mining Barge",
    "Corvette",
    "Research Ship"
];

// Lists of storylines by faction index
const STORYLINE_NAMES: [[&str; 3]; 4] = [
    ["Commerce Pioneer", "Smuggler's Run", "Galactic Entrepreneur"],
    ["Elite Prospector", "Master Refiner", "Asteroid Baron"],
    ["System Defense", "Special Operations", "Fleet Commander"],
    ["Research Pioneer", "Galactic Explorer", "Biotechnology Expert"]
];

const STORYLINE_DESCRIPTIONS: [[&str; 3]; 4] = [
    [
        "Establish trade routes throughout the galaxy and become a wealthy merchant.",
        "Master the art of moving goods through dangerous territories for higher profits.",
        "Build your own trading empire by investing in stations and infrastructure."
    ],
    [
        "Discover and claim the richest mining locations in the galaxy.",
        "Specialize in processing raw materials into high-value refined goods.",
        "Control the asteroid belts and establish mining operations throughout the system."
    ],
    [
        "Protect civilian shipping lanes from pirates and other threats.",
        "Undertake covert missions in rival territories and hostile zones.",
        "Lead a squadron of ships to maintain galactic peace and order."
    ],
    [
        "Discover new technologies by studying cosmic phenomena and artifacts.",
        "Chart unexplored regions of space and document new discoveries.",
        "Research alien biology and develop enhancements for human survival in space."
    ]
];

pub fn draw_character_creation<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    // Create a layout to organize the character creation UI
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Length(3),  // Subtitle/Prompt
            Constraint::Min(5),     // Content
            Constraint::Length(5),  // Instructions
        ].as_ref())
        .split(area);
    
    // Draw title
    let title = Paragraph::new("CHARACTER CREATION")
        .style(Style::default().fg(colors::PRIMARY).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default());
    f.render_widget(title, chunks[0]);
    
    // Based on the creation stage, show different UI elements
    match game.creation_stage {
        // Character Name Input
        0 => {
            // Prompt
            let name_prompt = Paragraph::new("Enter your character name:")
                .style(Style::default().fg(colors::SECONDARY))
                .alignment(Alignment::Center);
            f.render_widget(name_prompt, chunks[1]);
            
            // Current name input
            let name_input = Paragraph::new(game.character_name.clone())
                .style(Style::default().fg(colors::PRIMARY))
                .alignment(Alignment::Center)
                .block(Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(colors::SECONDARY)));
            f.render_widget(name_input, chunks[2]);
            
            // Instructions
            let instructions = Paragraph::new(
                "Type your character name and press Enter to continue\nBackspace to delete characters"
            )
            .style(Style::default().fg(colors::INFO))
            .alignment(Alignment::Center);
            f.render_widget(instructions, chunks[3]);
        },
        
        // Faction Selection
        1 => {
            // Prompt
            let faction_prompt = Paragraph::new("Select your faction:")
                .style(Style::default().fg(colors::SECONDARY))
                .alignment(Alignment::Center);
            f.render_widget(faction_prompt, chunks[1]);
            
            // Create a vertical layout for the faction options
            let faction_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Traders
                    Constraint::Length(3), // Miners
                    Constraint::Length(3), // Military
                    Constraint::Length(3), // Scientists
                ].as_ref())
                .split(chunks[2]);
            
            // Display faction options
            for i in 0..4 {
                let faction_text = format!(
                    "{}. {} - {}  (Ship: {})",
                    i + 1,
                    FACTION_NAMES[i],
                    FACTION_DESCRIPTIONS[i],
                    FACTION_SHIP_CLASSES[i]
                );
                
                let style = if i == game.selected_faction {
                    Style::default().fg(colors::PRIMARY).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(colors::DIM)
                };
                
                let faction_item = Paragraph::new(faction_text)
                    .style(style)
                    .alignment(Alignment::Left);
                f.render_widget(faction_item, faction_chunks[i]);
            }
            
            // Instructions
            let instructions = Paragraph::new(
                "Press 1-4 to select a faction\nBackspace to return to name entry"
            )
            .style(Style::default().fg(colors::INFO))
            .alignment(Alignment::Center);
            f.render_widget(instructions, chunks[3]);
        },
        
        // Storyline Selection
        2 => {
            // Get the selected faction
            let faction_index = game.selected_faction;
            let faction_name = FACTION_NAMES[faction_index];
            
            // Prompt
            let storyline_prompt = Paragraph::new(format!("Select your {} storyline:", faction_name))
                .style(Style::default().fg(colors::SECONDARY))
                .alignment(Alignment::Center);
            f.render_widget(storyline_prompt, chunks[1]);
            
            // Create a vertical layout for the storyline options
            let storyline_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Storyline 1
                    Constraint::Length(3), // Storyline 2
                    Constraint::Length(3), // Storyline 3
                ].as_ref())
                .split(chunks[2]);
            
            // Display storyline options
            for i in 0..3 {
                let storyline_text = format!(
                    "{}. {} - {}",
                    i + 1,
                    STORYLINE_NAMES[faction_index][i],
                    STORYLINE_DESCRIPTIONS[faction_index][i]
                );
                
                let style = if i == game.selected_storyline {
                    Style::default().fg(colors::PRIMARY).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(colors::DIM)
                };
                
                let storyline_item = Paragraph::new(storyline_text)
                    .style(style)
                    .alignment(Alignment::Left);
                f.render_widget(storyline_item, storyline_chunks[i]);
            }
            
            // Instructions
            let instructions = Paragraph::new(
                "Press 1-3 to select a storyline\nBackspace to return to faction selection"
            )
            .style(Style::default().fg(colors::INFO))
            .alignment(Alignment::Center);
            f.render_widget(instructions, chunks[3]);
        },
        
        // Confirmation
        3 => {
            // Get the selected faction and storyline details
            let faction_index = game.selected_faction;
            let storyline_index = game.selected_storyline;
            
            let faction_name = FACTION_NAMES[faction_index];
            let faction_ship = FACTION_SHIP_CLASSES[faction_index];
            let storyline_name = STORYLINE_NAMES[faction_index][storyline_index];
            
            // Prompt
            let confirm_prompt = Paragraph::new("Confirm your character:")
                .style(Style::default().fg(colors::SECONDARY))
                .alignment(Alignment::Center);
            f.render_widget(confirm_prompt, chunks[1]);
            
            // Character summary
            let summary_text = vec![
                Spans::from(vec![
                    Span::styled("Name: ", Style::default().fg(colors::INFO)),
                    Span::styled(&game.character_name, Style::default().fg(colors::PRIMARY).add_modifier(Modifier::BOLD)),
                ]),
                Spans::from(Span::raw("")),
                Spans::from(vec![
                    Span::styled("Faction: ", Style::default().fg(colors::INFO)),
                    Span::styled(faction_name, Style::default().fg(colors::PRIMARY).add_modifier(Modifier::BOLD)),
                ]),
                Spans::from(vec![
                    Span::styled("Starting Ship: ", Style::default().fg(colors::INFO)),
                    Span::styled(faction_ship, Style::default().fg(colors::SECONDARY)),
                ]),
                Spans::from(Span::raw("")),
                Spans::from(vec![
                    Span::styled("Storyline: ", Style::default().fg(colors::INFO)),
                    Span::styled(storyline_name, Style::default().fg(colors::PRIMARY).add_modifier(Modifier::BOLD)),
                ]),
                Spans::from(vec![
                    Span::styled("Description: ", Style::default().fg(colors::INFO)),
                    Span::styled(
                        STORYLINE_DESCRIPTIONS[faction_index][storyline_index],
                        Style::default().fg(colors::SECONDARY)
                    ),
                ]),
            ];
            
            let summary = Paragraph::new(summary_text)
                .style(Style::default())
                .alignment(Alignment::Left)
                .block(Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(colors::SECONDARY)));
            f.render_widget(summary, chunks[2]);
            
            // Instructions
            let instructions = Paragraph::new(
                "Press Y to confirm and start game\nPress N to start over\nBackspace to return to storyline selection"
            )
            .style(Style::default().fg(colors::INFO))
            .alignment(Alignment::Center);
            f.render_widget(instructions, chunks[3]);
        },
        
        // Default case - should never happen
        _ => {
            // Error message
            let error_text = Paragraph::new("Error in character creation process. Please restart the game.")
                .style(Style::default().fg(colors::DANGER))
                .alignment(Alignment::Center);
            f.render_widget(error_text, chunks[2]);
        }
    }
}