use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize};

// Constants for skill leveling
const SKILL_POINTS_PER_SECOND: f32 = 0.1; // Base rate for skill point accumulation
const SKILL_LEVEL_THRESHOLDS: [u32; 5] = [100, 300, 600, 1000, 1500]; // Points needed for each level

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SkillCategory {
    Mining,      // Resource extraction
    Trading,     // Commerce and market operations
    Combat,      // Ship combat and weapons
    Navigation,  // Piloting and travel
    Research,    // Blueprint and technology research
    Engineering, // Ship and equipment crafting/repair
}

impl SkillCategory {
    pub fn to_string(&self) -> String {
        match self {
            SkillCategory::Mining => "Mining".to_string(),
            SkillCategory::Trading => "Trading".to_string(),
            SkillCategory::Combat => "Combat".to_string(),
            SkillCategory::Navigation => "Navigation".to_string(),
            SkillCategory::Research => "Research".to_string(),
            SkillCategory::Engineering => "Engineering".to_string(),
        }
    }
    
    pub fn description(&self) -> String {
        match self {
            SkillCategory::Mining => "Improves resource extraction efficiency, reduces mining time, and increases yield.".to_string(),
            SkillCategory::Trading => "Improves buying and selling prices, unlocks special market deals, and increases market information.".to_string(),
            SkillCategory::Combat => "Improves weapon efficacy, targeting, and ship maneuverability in combat situations.".to_string(),
            SkillCategory::Navigation => "Improves ship speed, jump range, and fuel efficiency.".to_string(),
            SkillCategory::Research => "Reduces blueprint research time and improves research outcomes.".to_string(),
            SkillCategory::Engineering => "Reduces craft time, improves equipment quality, and reduces materials required.".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub category: SkillCategory,
    pub points: u32,
    pub level: u8,
    pub active: bool,
    pub last_update: Option<f64>, // Serializable timestamp
}

impl Skill {
    pub fn new(category: SkillCategory) -> Self {
        Self {
            category,
            points: 0,
            level: 0,
            active: false,
            last_update: Some(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64()),
        }
    }
    
    pub fn activate(&mut self) -> bool {
        if !self.active {
            self.active = true;
            self.last_update = Some(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64());
            true
        } else {
            false
        }
    }
    
    pub fn deactivate(&mut self) -> bool {
        if self.active {
            self.active = false;
            false
        } else {
            false
        }
    }
    
    pub fn update(&mut self, current_time: f64) {
        if !self.active || self.last_update.is_none() {
            return;
        }
        
        let last_time = self.last_update.unwrap();
        let elapsed_seconds = current_time - last_time;
        
        if elapsed_seconds <= 0.0 {
            return;
        }
        
        // Calculate skill points gained
        let points_gained = (elapsed_seconds as f32 * SKILL_POINTS_PER_SECOND) as u32;
        self.points += points_gained;
        
        // Update level based on thresholds
        self.level = self.calculate_level();
        
        // Update last update time
        self.last_update = Some(current_time);
    }
    
    fn calculate_level(&self) -> u8 {
        for (i, threshold) in SKILL_LEVEL_THRESHOLDS.iter().enumerate() {
            if self.points < *threshold {
                return i as u8;
            }
        }
        SKILL_LEVEL_THRESHOLDS.len() as u8
    }
    
    pub fn get_progress_to_next_level(&self) -> f32 {
        if self.level as usize >= SKILL_LEVEL_THRESHOLDS.len() {
            return 100.0; // Max level reached
        }
        
        let current_threshold = if self.level == 0 { 0 } else { SKILL_LEVEL_THRESHOLDS[self.level as usize - 1] };
        let next_threshold = SKILL_LEVEL_THRESHOLDS[self.level as usize];
        
        let points_in_level = self.points - current_threshold;
        let points_required = next_threshold - current_threshold;
        
        (points_in_level as f32 / points_required as f32) * 100.0
    }
    
    // Calculate bonuses based on skill level
    pub fn get_efficiency_bonus(&self) -> f32 {
        // Each level gives a 10% efficiency bonus
        (self.level as f32) * 0.1
    }
    
    pub fn get_time_reduction(&self) -> f32 {
        // Each level reduces time by 5%, up to 25% at max level
        (self.level as f32 * 0.05).min(0.25)
    }
    
    pub fn get_quality_bonus(&self) -> f32 {
        // Each level improves quality by 5%
        (self.level as f32) * 0.05
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillSet {
    pub skills: Vec<Skill>,
}

impl SkillSet {
    pub fn new() -> Self {
        // Create skills for all categories
        let skills = vec![
            Skill::new(SkillCategory::Mining),
            Skill::new(SkillCategory::Trading),
            Skill::new(SkillCategory::Combat),
            Skill::new(SkillCategory::Navigation),
            Skill::new(SkillCategory::Research),
            Skill::new(SkillCategory::Engineering),
        ];
        
        Self { skills }
    }
    
    pub fn get_skill(&self, category: &SkillCategory) -> Option<&Skill> {
        self.skills.iter().find(|s| s.category == *category)
    }
    
    pub fn get_skill_mut(&mut self, category: &SkillCategory) -> Option<&mut Skill> {
        self.skills.iter_mut().find(|s| s.category == *category)
    }
    
    pub fn update_all(&mut self) {
        let current_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64();
        for skill in &mut self.skills {
            skill.update(current_time);
        }
    }
    
    pub fn activate_skill(&mut self, category: &SkillCategory) -> bool {
        if let Some(skill) = self.get_skill_mut(category) {
            skill.activate()
        } else {
            false
        }
    }
    
    pub fn deactivate_skill(&mut self, category: &SkillCategory) -> bool {
        if let Some(skill) = self.get_skill_mut(category) {
            skill.deactivate()
        } else {
            false
        }
    }
}