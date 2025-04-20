use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs, Wrap},
    Frame,
};
use crate::game::Game;
use crate::ui::colors;

// Character info screen enum to track which tab is active
#[derive(PartialEq, Clone, Copy)]
pub enum CharacterInfoTab {
    Skills,
    Reputation,
    Assets,
    Background,
}

impl CharacterInfoTab {
    pub fn index(&self) -> usize {
        match self {
            CharacterInfoTab::Skills => 0,
            CharacterInfoTab::Reputation => 1,
            CharacterInfoTab::Assets => 2,
            CharacterInfoTab::Background => 3,
        }
    }

    pub fn from_index(index: usize) -> Self {
        match index {
            0 => CharacterInfoTab::Skills,
            1 => CharacterInfoTab::Reputation,
            2 => CharacterInfoTab::Assets,
            3 => CharacterInfoTab::Background,
            _ => CharacterInfoTab::Skills,
        }
    }
}

pub fn draw_character_info<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    // Create the main border
    let block = Block::default()
        .title(Span::styled(" CHARACTER INFORMATION ", Style::default().fg(colors::PRIMARY)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::PRIMARY));
    
    let inner_area = block.inner(area);
    f.render_widget(block, area);

    // Create tabs for different character info sections
    let tab_titles = vec!["Skills", "Reputation", "Assets", "Background"];
    let tabs = Tabs::new(
        tab_titles.iter().map(|t| {
            Spans::from(vec![Span::styled(*t, Style::default().fg(colors::PRIMARY))])
        }).collect()
    )
    .block(Block::default().borders(Borders::BOTTOM))
    .style(Style::default().fg(colors::DIM))
    .highlight_style(Style::default().fg(colors::PRIMARY).add_modifier(Modifier::BOLD))
    .select(game.character_info_tab); // Set the active tab based on game state
    
    // Create layout for tabs and content
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Tabs
            Constraint::Min(10),   // Content
        ])
        .split(inner_area);
    
    f.render_widget(tabs, chunks[0]);
    
    // Determine which tab to display using the game state
    let active_tab = CharacterInfoTab::from_index(game.character_info_tab);
    
    match active_tab {
        CharacterInfoTab::Skills => draw_skills_tab(f, game, chunks[1]),
        CharacterInfoTab::Reputation => draw_reputation_tab(f, game, chunks[1]),
        CharacterInfoTab::Assets => draw_assets_tab(f, game, chunks[1]),
        CharacterInfoTab::Background => draw_background_tab(f, game, chunks[1]),
    }
    
    // Draw instructions at the bottom
    let instructions = Paragraph::new(vec![
        Spans::from(vec![
            Span::raw("Press ["),
            Span::styled("1", Style::default().fg(colors::PRIMARY)),
            Span::raw("-"),
            Span::styled("4", Style::default().fg(colors::PRIMARY)),
            Span::raw("] to switch tabs, ["),
            Span::styled("M", Style::default().fg(colors::PRIMARY)),
            Span::raw("] to return to main menu")
        ])
    ])
    .block(Block::default().borders(Borders::TOP))
    .alignment(tui::layout::Alignment::Center);
    
    let footer_area = Rect::new(
        inner_area.x,
        inner_area.y + inner_area.height - 3,
        inner_area.width,
        3
    );
    
    f.render_widget(instructions, footer_area);
}

fn draw_skills_tab<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    let player = &game.player;
    let skills = &player.skills.skills;
    
    let block = Block::default()
        .title(Span::styled(" SKILLS ", Style::default().fg(colors::INFO)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::DIM));
    
    let inner_area = block.inner(area);
    f.render_widget(block.clone(), area);
    
    // Build list of skills with levels and progress
    let skill_items: Vec<ListItem> = skills.iter().map(|skill| {
        let skill_name = skill.category.to_string();
        let skill_level = skill.level;
        let progress = skill.get_progress_to_next_level();
        
        ListItem::new(vec![
            Spans::from(vec![
                Span::styled(
                    format!("{}: Level {}", skill_name, skill_level),
                    Style::default().fg(colors::PRIMARY).add_modifier(Modifier::BOLD)
                ),
            ]),
            Spans::from(vec![
                Span::styled(
                    format!("Progress: {:.1}% | Points: {}", progress, skill.points),
                    Style::default().fg(colors::INFO)
                ),
            ]),
            Spans::from(""),
        ])
    }).collect();
    
    let skills_list = List::new(skill_items)
        .block(Block::default())
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    
    f.render_widget(skills_list, inner_area);
}

fn draw_reputation_tab<B: Backend>(f: &mut Frame<B>, _game: &Game, area: Rect) {
    let block = Block::default()
        .title(Span::styled(" REPUTATION ", Style::default().fg(colors::INFO)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::DIM));
    
    let inner_area = block.inner(area);
    f.render_widget(block.clone(), area);
    
    // In a full implementation, we would display faction reputation here
    let placeholder = Paragraph::new(vec![
        Spans::from(vec![
            Span::styled(
                "Faction Relations:", 
                Style::default().fg(colors::PRIMARY).add_modifier(Modifier::BOLD)
            ),
        ]),
        Spans::from(""),
        Spans::from(vec![
            Span::raw("United Trade Federation: "),
            Span::styled("Neutral", Style::default().fg(colors::INFO)),
        ]),
        Spans::from(vec![
            Span::raw("Mining Consortium: "),
            Span::styled("Friendly", Style::default().fg(colors::SUCCESS)),
        ]),
        Spans::from(vec![
            Span::raw("Galactic Security Force: "),
            Span::styled("Neutral", Style::default().fg(colors::INFO)),
        ]),
        Spans::from(vec![
            Span::raw("Scientific Academy: "),
            Span::styled("Neutral", Style::default().fg(colors::INFO)),
        ]),
    ])
    .block(Block::default())
    .wrap(Wrap { trim: true });
    
    f.render_widget(placeholder, inner_area);
}

fn draw_assets_tab<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    let player = &game.player;
    
    let block = Block::default()
        .title(Span::styled(" ASSETS ", Style::default().fg(colors::INFO)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::DIM));
    
    let inner_area = block.inner(area);
    f.render_widget(block.clone(), area);
    
    // Display player's assets (credits, ships, property)
    let assets_info = Paragraph::new(vec![
        Spans::from(vec![
            Span::styled(
                "Financial Assets:", 
                Style::default().fg(colors::PRIMARY).add_modifier(Modifier::BOLD)
            ),
        ]),
        Spans::from(vec![
            Span::raw("Credits: "),
            Span::styled(
                format!("{}", player.credits), 
                Style::default().fg(colors::SUCCESS)
            ),
        ]),
        Spans::from(""),
        Spans::from(vec![
            Span::styled(
                "Ships:", 
                Style::default().fg(colors::PRIMARY).add_modifier(Modifier::BOLD)
            ),
        ]),
        Spans::from(vec![
            Span::raw("Current Ship: "),
            Span::styled(
                player.ship.name.clone(), 
                Style::default().fg(colors::INFO)
            ),
        ]),
        Spans::from(vec![
            Span::raw("Ship Type: "),
            Span::styled(
                format!("{:?}", player.ship.ship_type), 
                Style::default().fg(colors::INFO)
            ),
        ]),
        Spans::from(vec![
            Span::raw("Ship Value: "),
            Span::styled(
                "50,000 credits (estimated)", 
                Style::default().fg(colors::SUCCESS)
            ),
        ]),
        Spans::from(""),
        Spans::from(vec![
            Span::styled(
                "Total Net Worth:", 
                Style::default().fg(colors::PRIMARY).add_modifier(Modifier::BOLD)
            ),
            Span::styled(
                format!(" {} credits", player.credits + 50000), 
                Style::default().fg(colors::SUCCESS).add_modifier(Modifier::BOLD)
            ),
        ]),
    ])
    .block(Block::default())
    .wrap(Wrap { trim: true });
    
    f.render_widget(assets_info, inner_area);
}

fn draw_background_tab<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    let player = &game.player;
    
    let block = Block::default()
        .title(Span::styled(" BACKGROUND ", Style::default().fg(colors::INFO)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::DIM));
    
    let inner_area = block.inner(area);
    f.render_widget(block.clone(), area);
    
    // Display player's faction, storyline, and background info
    let faction_name = player.character.faction.to_string();
    let storyline_name = if let Some(storyline) = &player.character.active_storyline {
        storyline.name.clone()
    } else {
        "No active storyline".to_string()
    };
    let background = if let Some(storyline) = &player.character.active_storyline {
        if !storyline.background.is_empty() {
            storyline.background.clone()
        } else {
            "You are a space trader seeking fortune among the stars.".to_string()
        }
    } else {
        "You are a space trader seeking fortune among the stars.".to_string()
    };
    
    let background_info = Paragraph::new(vec![
        Spans::from(vec![
            Span::styled(
                format!("Commander: {}", player.character.name), 
                Style::default().fg(colors::PRIMARY).add_modifier(Modifier::BOLD)
            ),
        ]),
        Spans::from(vec![
            Span::raw("Faction: "),
            Span::styled(
                faction_name, 
                Style::default().fg(colors::INFO)
            ),
        ]),
        Spans::from(vec![
            Span::raw("Storyline: "),
            Span::styled(
                storyline_name, 
                Style::default().fg(colors::INFO)
            ),
        ]),
        Spans::from(""),
        Spans::from(vec![
            Span::styled(
                "Background:", 
                Style::default().fg(colors::PRIMARY).add_modifier(Modifier::BOLD)
            ),
        ]),
        Spans::from(""),
        Spans::from(vec![
            Span::raw(background),
        ]),
    ])
    .block(Block::default())
    .wrap(Wrap { trim: true });
    
    f.render_widget(background_info, inner_area);
}