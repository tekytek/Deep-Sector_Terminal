use serde::{Serialize, Deserialize};
use crate::models::ship::{Ship, ShipType};
use crate::models::skills::{Skill, SkillCategory};

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
    pub background: String,     // Detailed character background
    pub progress: usize,
    pub total_steps: usize,
    pub completed: bool,
    pub starting_skills: Vec<(SkillCategory, u32)>, // Starting skill categories and levels
}

impl Storyline {
    pub fn new(id: &str, faction: FactionType, name: &str, description: &str, total_steps: usize) -> Self {
        // Default implementation for backward compatibility
        Self {
            id: id.to_string(),
            faction,
            name: name.to_string(),
            description: description.to_string(),
            background: "".to_string(),
            progress: 0,
            total_steps,
            completed: false,
            starting_skills: Vec::new(),
        }
    }
    
    // Enhanced constructor with background and starting skills
    pub fn new_with_background(
        id: &str, 
        faction: FactionType, 
        name: &str, 
        description: &str, 
        background: &str,
        total_steps: usize,
        starting_skills: Vec<(SkillCategory, u32)>
    ) -> Self {
        Self {
            id: id.to_string(),
            faction,
            name: name.to_string(),
            description: description.to_string(),
            background: background.to_string(),
            progress: 0,
            total_steps,
            completed: false,
            starting_skills,
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
            Storyline::new_with_background(
                "traders_dominance", 
                FactionType::Traders.clone(),
                "Market Dominance", 
                "Establish a trading empire that controls the galactic economy.", 
                "Born into a merchant family, you've spent your life learning the intricacies of galactic markets. \
                As a shrewd negotiator, your reputation for fair but profitable deals has spread across several systems. \
                Now, with your own vessel, you aim to expand your influence and become a market power across the galaxy.",
                10,
                vec![
                    (SkillCategory::Trading, 3),   // Advanced trading skills
                    (SkillCategory::Navigation, 2), // Good navigation
                    (SkillCategory::Engineering, 1), // Basic engineering
                ]
            ),
            Storyline::new_with_background(
                "traders_discovery", 
                FactionType::Traders.clone(),
                "Profit Frontiers", 
                "Discover new trade routes and exotic goods to maximize profits.", 
                "You made your name as an explorer who identified valuable trade opportunities in unmapped systems. \
                Your earlier expeditions yielded enough profit to acquire your own ship, and now you seek to \
                find undiscovered resources and establish new trade routes that will revolutionize the galactic economy.",
                8,
                vec![
                    (SkillCategory::Trading, 2),    // Good trading
                    (SkillCategory::Navigation, 3),  // Excellent navigation
                    (SkillCategory::Research, 1),    // Basic research
                ]
            ),
            Storyline::new_with_background(
                "traders_influence", 
                FactionType::Traders.clone(),
                "Political Influence", 
                "Use your wealth to gain political power and influence galactic policy.", 
                "A former advisor to planetary governors, you've witnessed how commerce shapes politics across the stars. \
                With connections throughout diplomatic circles and a knack for brokering mutually beneficial agreements, \
                you've acquired your first ship to begin building a commercial empire with political aspirations.",
                12,
                vec![
                    (SkillCategory::Trading, 3),    // Advanced trading
                    (SkillCategory::Combat, 1),     // Basic combat
                    (SkillCategory::Engineering, 2), // Good engineering
                ]
            ),
        ],
        FactionType::Miners => vec![
            Storyline::new_with_background(
                "miners_motherlode", 
                FactionType::Miners.clone(),
                "The Motherlode", 
                "Discover the legendary Motherlode asteroid with untold riches.", 
                "Growing up in an asteroid mining colony, you learned to read mineral compositions like others read books. \
                Your exceptional talent for finding valuable deposits earned you respect among veteran miners. \
                You've invested everything in your own mining vessel to seek the fabled Motherlode asteroid that could make you a legend.",
                9,
                vec![
                    (SkillCategory::Mining, 3),     // Advanced mining
                    (SkillCategory::Engineering, 2), // Good engineering
                    (SkillCategory::Navigation, 1),  // Basic navigation
                ]
            ),
            Storyline::new_with_background(
                "miners_technology", 
                FactionType::Miners.clone(),
                "Resource Revolution", 
                "Develop revolutionary mining technology to transform the industry.", 
                "As a mining engineer with innovative ideas, you've developed prototypes that could revolutionize resource extraction. \
                After corporate interests tried to steal your designs, you've struck out on your own with a mining vessel to \
                prove your technology in the field and change the industry forever.",
                10,
                vec![
                    (SkillCategory::Mining, 2),      // Good mining
                    (SkillCategory::Engineering, 3),  // Advanced engineering
                    (SkillCategory::Research, 1),     // Basic research
                ]
            ),
            Storyline::new_with_background(
                "miners_empire", 
                FactionType::Miners.clone(),
                "Resource Empire", 
                "Control the resource supply chain across multiple star systems.", 
                "Once a small-time prospector, you gained notoriety after discovering a rare mineral vein that funded your first ship purchase. \
                With a keen understanding of supply chains and resource logistics, you plan to establish mining operations across multiple systems \
                and control the flow of critical materials throughout the galaxy.",
                11,
                vec![
                    (SkillCategory::Mining, 3),     // Advanced mining
                    (SkillCategory::Trading, 2),    // Good trading
                    (SkillCategory::Combat, 1),     // Basic combat for defense
                ]
            ),
        ],
        FactionType::Military => vec![
            Storyline::new_with_background(
                "military_threat", 
                FactionType::Military.clone(),
                "The Rising Threat", 
                "Combat a mysterious alien force threatening galactic security.", 
                "As a decorated veteran of the Frontier Wars, you've seen more combat than most. \
                Recently discharged after reporting unusual activity in the outer systems that command ignored, \
                you've acquired your own vessel to investigate these anomalies yourself and protect humanity from what you believe is coming.",
                12,
                vec![
                    (SkillCategory::Combat, 3),      // Advanced combat
                    (SkillCategory::Navigation, 2),  // Good navigation
                    (SkillCategory::Engineering, 1), // Basic engineering
                ]
            ),
            Storyline::new_with_background(
                "military_peacekeeping", 
                FactionType::Military.clone(),
                "Galactic Peacekeeping", 
                "Establish order in lawless regions and fight pirate organizations.", 
                "Your career as a military police officer showed you how lawlessness destroys communities across the frontier. \
                After budget cuts eliminated your peacekeeping division, you invested your savings in a patrol vessel to continue \
                your mission: bringing justice to lawless systems and dismantling criminal organizations.",
                10,
                vec![
                    (SkillCategory::Combat, 2),     // Good combat
                    (SkillCategory::Navigation, 2), // Good navigation
                    (SkillCategory::Trading, 2),    // Good trading (for information networks)
                ]
            ),
            Storyline::new_with_background(
                "military_supremacy", 
                FactionType::Military.clone(),
                "Military Supremacy", 
                "Build the ultimate combat fleet and establish military dominance.", 
                "A tactical genius who rose quickly through military ranks, you became frustrated with political constraints on military operations. \
                Resigning your commission, you've secured private funding for your own combat vessel - the first of what you plan to be \
                an elite private fleet capable of outperforming standard military formations.",
                9,
                vec![
                    (SkillCategory::Combat, 3),       // Advanced combat
                    (SkillCategory::Engineering, 2),  // Good engineering
                    (SkillCategory::Research, 1),     // Basic research for weapons R&D
                ]
            ),
        ],
        FactionType::Scientists => vec![
            Storyline::new_with_background(
                "scientists_discovery", 
                FactionType::Scientists.clone(),
                "Ancient Discovery", 
                "Uncover the mysteries of an ancient alien civilization and their technology.", 
                "Your academic career in xenoarchaeology was derailed when you proposed controversial theories about advanced ancient civilizations. \
                Ridiculed by the scientific establishment, you've used family inheritance to purchase a research vessel and prove your theories \
                by finding concrete evidence of these mysterious precursor species and their advanced technology.",
                11,
                vec![
                    (SkillCategory::Research, 3),     // Advanced research
                    (SkillCategory::Navigation, 2),   // Good navigation
                    (SkillCategory::Mining, 1),       // Basic mining for excavation
                ]
            ),
            Storyline::new_with_background(
                "scientists_breakthrough", 
                FactionType::Scientists.clone(),
                "Technological Breakthrough", 
                "Achieve a revolutionary breakthrough in FTL travel technology.", 
                "A theoretical physicist with radical ideas about space-time manipulation, you've developed equations that suggest \
                new FTL travel methods are possible. After research institutions refused to fund your 'impossible' experiments, \
                you've acquired an explorer vessel to gather data from space anomalies and prove your revolutionary theories.",
                10,
                vec![
                    (SkillCategory::Research, 3),      // Advanced research
                    (SkillCategory::Engineering, 2),   // Good engineering
                    (SkillCategory::Navigation, 1),    // Basic navigation
                ]
            ),
            Storyline::new_with_background(
                "scientists_anomaly", 
                FactionType::Scientists.clone(),
                "The Anomaly", 
                "Investigate a spacetime anomaly that could change our understanding of the universe.", 
                "As a former government astrophysicist, you detected unusual readings that suggest a massive spacetime anomaly is developing in a remote sector. \
                When your reports were classified and research terminated, you resigned in protest. Now with your own explorer vessel, \
                you're determined to reach and study this phenomenon that could revolutionize our understanding of physics.",
                12,
                vec![
                    (SkillCategory::Research, 3),      // Advanced research
                    (SkillCategory::Navigation, 2),    // Good navigation
                    (SkillCategory::Engineering, 1),   // Basic engineering
                ]
            ),
        ],
    }
}