use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::models::ship::{Ship, ShipType};
use crate::models::universe::StarSystem;
use crate::models::item::Inventory;
use crate::models::faction::{FactionType, Storyline};
use crate::models::skills::SkillSet;
use crate::models::blueprint::BlueprintLibrary;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub faction: FactionType,
    pub active_storyline: Option<Storyline>,
    pub completed_storylines: Vec<Storyline>,
}

impl Character {
    pub fn new(name: &str, faction: FactionType, storyline: Storyline) -> Self {
        Self {
            name: name.to_string(),
            faction,
            active_storyline: Some(storyline),
            completed_storylines: Vec::new(),
        }
    }
    
    pub fn advance_storyline(&mut self) -> bool {
        if let Some(ref mut storyline) = self.active_storyline {
            let result = storyline.advance();
            
            // If storyline is completed, move it to completed_storylines
            if storyline.completed {
                let completed = self.active_storyline.take().unwrap();
                self.completed_storylines.push(completed);
            }
            
            result
        } else {
            false
        }
    }
    
    pub fn set_storyline(&mut self, storyline: Storyline) -> bool {
        if self.active_storyline.is_some() {
            false
        } else {
            self.active_storyline = Some(storyline);
            true
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub character: Character,
    pub credits: u32,
    pub ship: Ship,
    pub inventory: Inventory,
    pub current_system: StarSystem,
    pub is_docked: bool,
    pub skills: SkillSet,
    pub blueprints: BlueprintLibrary,
    pub last_update: Option<f64>, // Serializable timestamp for real-time events
}

impl Player {
    pub fn new(name: &str) -> Self {
        let id = Uuid::new_v4().to_string();
        let ship = Ship::default("Rustbucket", ShipType::Scout);
        
        // Start with a default inventory and some starting credits
        let inventory = Inventory::new(ship.cargo_capacity);
        let credits = 5000;

        // Create a default starting system (empty - will be replaced during character creation)
        let current_system = StarSystem {
            id: "sol".to_string(),
            name: "Sol".to_string(),
            x: 0.0,
            y: 0.0,
            celestial_bodies: Vec::new(),
            resource_fields: Vec::new(),
            stations: Vec::new(),
            resources: vec![],
        };

        // Default character with Traders faction
        let character = Character::new(
            name,
            FactionType::Traders,
            Storyline::new(
                "default_storyline",
                FactionType::Traders,
                "Getting Started",
                "Learn the basics of trading and navigation.",
                5
            )
        );

        Self {
            id,
            character,
            credits,
            ship,
            inventory,
            current_system,
            is_docked: true,
            skills: SkillSet::new(),
            blueprints: BlueprintLibrary::new(),
            last_update: Some(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64()),
        }
    }
    
    pub fn with_character(name: &str, faction: FactionType, storyline: Storyline) -> Self {
        let mut player = Self::new(name);
        
        // Set character details
        player.character = Character::new(name, faction.clone(), storyline);
        
        // Assign faction-specific ship
        player.ship = faction.starting_ship();
        player.inventory = Inventory::new(player.ship.cargo_capacity);
        
        player
    }
    
    pub fn update_skills(&mut self) {
        // Update real-time progression for skills
        let current_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64();
        
        if let Some(last_time) = self.last_update {
            // Only update if some time has passed
            if current_time > last_time {
                // Update skills
                self.skills.update_all();
                
                // Update blueprints research
                if let Some(research_skill) = self.skills.get_skill(&crate::models::skills::SkillCategory::Research) {
                    self.blueprints.update_research(research_skill);
                }
                
                // Update last update time
                self.last_update = Some(current_time);
            }
        }
    }

    pub fn add_credits(&mut self, amount: u32) {
        self.credits += amount;
    }

    pub fn remove_credits(&mut self, amount: u32) -> bool {
        if amount <= self.credits {
            self.credits -= amount;
            true
        } else {
            false
        }
    }
}
