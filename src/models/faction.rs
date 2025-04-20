use serde::{Serialize, Deserialize};
use crate::models::ship::{Ship, ShipType};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FactionType {
    Traders,      // Focus on trading and commerce
    Miners,       // Focus on resource extraction
    Military,     // Focus on combat and security
    Scientists,   // Focus on research and development
}

impl FactionType {
    pub fn to_string(&self) -> String {
        match self {
            FactionType::Traders => "United Trade Federation".to_string(),
            FactionType::Miners => "Mining Consortium".to_string(),
            FactionType::Military => "Galactic Security Force".to_string(),
            FactionType::Scientists => "Scientific Academy".to_string(),
        }
    }
    
    pub fn description(&self) -> String {
        match self {
            FactionType::Traders => "The United Trade Federation specializes in commerce and profit. Starting bonuses include better prices at markets and a cargo-focused ship.".to_string(),
            FactionType::Miners => "The Mining Consortium focuses on resource extraction. Starting bonuses include improved mining efficiency and a specialized mining vessel.".to_string(),
            FactionType::Military => "The Galactic Security Force maintains order in the systems. Starting bonuses include combat bonuses and a well-armed patrol ship.".to_string(),
            FactionType::Scientists => "The Scientific Academy pursues knowledge and discovery. Starting bonuses include research speed bonuses and an exploration vessel with advanced scanners.".to_string(),
        }
    }
    
    pub fn starting_ship(&self) -> Ship {
        match self {
            FactionType::Traders => Ship::new(
                &format!("{} Merchant Vessel", self.to_string()), 
                ShipType::Freighter,
                Some(150),    // Extra cargo capacity
                None,         // Default speed
                None,         // Default jump range
                None,         // Default weapon power
                None,         // Default mining power
            ),
            FactionType::Miners => Ship::new(
                &format!("{} Mining Vessel", self.to_string()), 
                ShipType::Miner,
                None,         // Default cargo
                None,         // Default speed
                None,         // Default jump range
                None,         // Default weapon power
                Some(15),     // Extra mining power
            ),
            FactionType::Military => Ship::new(
                &format!("{} Patrol Ship", self.to_string()), 
                ShipType::Fighter,
                None,         // Default cargo
                Some(100),    // Extra speed
                None,         // Default jump range
                Some(15),     // Extra weapon power
                None,         // Default mining power
            ),
            FactionType::Scientists => Ship::new(
                &format!("{} Explorer", self.to_string()), 
                ShipType::Scout,
                None,         // Default cargo
                None,         // Default speed
                Some(12),     // Extra jump range
                None,         // Default weapon power
                None,         // Default mining power
            ),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Storyline {
    pub id: String,
    pub faction: FactionType,
    pub name: String,
    pub description: String,
    pub progress: usize,
    pub total_steps: usize,
    pub completed: bool,
}

impl Storyline {
    pub fn new(id: &str, faction: FactionType, name: &str, description: &str, total_steps: usize) -> Self {
        Self {
            id: id.to_string(),
            faction,
            name: name.to_string(),
            description: description.to_string(),
            progress: 0,
            total_steps,
            completed: false,
        }
    }
    
    pub fn advance(&mut self) -> bool {
        if self.completed {
            return false;
        }
        
        self.progress += 1;
        if self.progress >= self.total_steps {
            self.completed = true;
        }
        
        true
    }
    
    pub fn get_progress_percentage(&self) -> f32 {
        if self.total_steps == 0 {
            return 0.0;
        }
        
        (self.progress as f32 / self.total_steps as f32) * 100.0
    }
}

pub fn get_storylines_for_faction(faction: &FactionType) -> Vec<Storyline> {
    match faction {
        FactionType::Traders => vec![
            Storyline::new(
                "traders_dominance", 
                FactionType::Traders.clone(),
                "Market Dominance", 
                "Establish a trading empire that controls the galactic economy.", 
                10
            ),
            Storyline::new(
                "traders_discovery", 
                FactionType::Traders.clone(),
                "Profit Frontiers", 
                "Discover new trade routes and exotic goods to maximize profits.", 
                8
            ),
            Storyline::new(
                "traders_influence", 
                FactionType::Traders.clone(),
                "Political Influence", 
                "Use your wealth to gain political power and influence galactic policy.", 
                12
            ),
        ],
        FactionType::Miners => vec![
            Storyline::new(
                "miners_motherlode", 
                FactionType::Miners.clone(),
                "The Motherlode", 
                "Discover the legendary Motherlode asteroid with untold riches.", 
                9
            ),
            Storyline::new(
                "miners_technology", 
                FactionType::Miners.clone(),
                "Resource Revolution", 
                "Develop revolutionary mining technology to transform the industry.", 
                10
            ),
            Storyline::new(
                "miners_empire", 
                FactionType::Miners.clone(),
                "Resource Empire", 
                "Control the resource supply chain across multiple star systems.", 
                11
            ),
        ],
        FactionType::Military => vec![
            Storyline::new(
                "military_threat", 
                FactionType::Military.clone(),
                "The Rising Threat", 
                "Combat a mysterious alien force threatening galactic security.", 
                12
            ),
            Storyline::new(
                "military_peacekeeping", 
                FactionType::Military.clone(),
                "Galactic Peacekeeping", 
                "Establish order in lawless regions and fight pirate organizations.", 
                10
            ),
            Storyline::new(
                "military_supremacy", 
                FactionType::Military.clone(),
                "Military Supremacy", 
                "Build the ultimate combat fleet and establish military dominance.", 
                9
            ),
        ],
        FactionType::Scientists => vec![
            Storyline::new(
                "scientists_discovery", 
                FactionType::Scientists.clone(),
                "Ancient Discovery", 
                "Uncover the mysteries of an ancient alien civilization and their technology.", 
                11
            ),
            Storyline::new(
                "scientists_breakthrough", 
                FactionType::Scientists.clone(),
                "Technological Breakthrough", 
                "Achieve a revolutionary breakthrough in FTL travel technology.", 
                10
            ),
            Storyline::new(
                "scientists_anomaly", 
                FactionType::Scientists.clone(),
                "The Anomaly", 
                "Investigate a spacetime anomaly that could change our understanding of the universe.", 
                12
            ),
        ],
    }
}