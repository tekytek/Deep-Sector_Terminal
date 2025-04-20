use std::time::Duration;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::models::item::ItemType;
use crate::models::skills::Skill;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BlueprintCategory {
    ShipPart,      // Components for ship upgrades
    Weapon,        // Weapons and defense systems
    Mining,        // Mining equipment
    Consumable,    // One-time use items
    Special,       // Rare or unique items
}

#[allow(dead_code)]
impl BlueprintCategory {
    pub fn to_string(&self) -> String {
        match self {
            BlueprintCategory::ShipPart => "Ship Part".to_string(),
            BlueprintCategory::Weapon => "Weapon".to_string(),
            BlueprintCategory::Mining => "Mining Equipment".to_string(),
            BlueprintCategory::Consumable => "Consumable".to_string(),
            BlueprintCategory::Special => "Special Item".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BlueprintType {
    Original,   // Can produce unlimited items, requires more research
    Copy,       // Limited uses, no further research possible
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintIngredient {
    pub item_name: String,
    pub item_type: ItemType,
    pub quantity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blueprint {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: BlueprintCategory,
    pub bp_type: BlueprintType,
    pub ingredients: Vec<BlueprintIngredient>,
    pub output_item: String,
    pub output_item_type: ItemType,
    pub output_quantity: u32,
    pub research_progress: u32,
    pub research_required: u32,
    pub research_started: bool,
    pub research_start_time: Option<f64>,
    pub research_complete: bool,
    pub quality_level: u8,   // 1-5 stars quality
    pub complexity: u32,     // Affects crafting time
    pub mass: u32,           // Affects resources needed
    pub remaining_uses: Option<u32>, // Only for Copy type
}

#[allow(dead_code)]
impl Blueprint {
    pub fn new(
        name: &str, 
        description: &str,
        category: BlueprintCategory,
        bp_type: BlueprintType,
        ingredients: Vec<BlueprintIngredient>,
        output_item: &str,
        output_item_type: ItemType,
        output_quantity: u32,
        complexity: u32,
        mass: u32,
        remaining_uses: Option<u32>,
    ) -> Self {
        // Calculate research required based on complexity and quality
        let research_required = complexity * 100;
        
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            description: description.to_string(),
            category,
            bp_type,
            ingredients,
            output_item: output_item.to_string(),
            output_item_type,
            output_quantity,
            research_progress: 0,
            research_required,
            research_started: false,
            research_start_time: None,
            research_complete: false,
            quality_level: 1,
            complexity,
            mass,
            remaining_uses,
        }
    }
    
    pub fn start_research(&mut self) -> bool {
        if self.research_complete || self.research_started || self.bp_type == BlueprintType::Copy {
            return false;
        }
        
        self.research_started = true;
        self.research_start_time = Some(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64());
        true
    }
    
    pub fn update_research(&mut self, research_skill: &Skill) -> bool {
        if !self.research_started || self.research_complete || self.research_start_time.is_none() {
            return false;
        }
        
        let current_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64();
        let elapsed_seconds = current_time - self.research_start_time.unwrap();
        
        if elapsed_seconds <= 0.0 {
            return false;
        }
        
        // Calculate progress based on time and skill
        let base_progress = (elapsed_seconds / 10.0) as u32; // 1 point per 10 seconds
        let skill_multiplier = 1.0 + research_skill.get_efficiency_bonus();
        let progress = (base_progress as f32 * skill_multiplier) as u32;
        
        self.research_progress += progress;
        
        // Update start time for next calculation
        self.research_start_time = Some(current_time);
        
        // Check if research is complete
        if self.research_progress >= self.research_required {
            self.research_complete = true;
            self.research_started = false;
            return true;
        }
        
        false
    }
    
    pub fn get_research_progress_percentage(&self) -> f32 {
        if self.research_required == 0 {
            return 0.0;
        }
        
        (self.research_progress as f32 / self.research_required as f32) * 100.0
    }
    
    pub fn improve_quality(&mut self) -> bool {
        if self.quality_level >= 5 || !self.research_complete {
            return false;
        }
        
        self.quality_level += 1;
        
        // Reset research for next quality level
        self.research_progress = 0;
        self.research_started = false;
        self.research_complete = false;
        self.research_start_time = None;
        
        // Increase research required for next level
        self.research_required = (self.research_required as f32 * 1.5) as u32;
        
        true
    }
    
    pub fn get_crafting_time(&self, engineering_skill: &Skill) -> Duration {
        // Base time depends on complexity
        let base_seconds = self.complexity as f32 * 60.0; // 1 minute per complexity
        
        // Apply skill time reduction
        let reduction = engineering_skill.get_time_reduction();
        let adjusted_seconds = base_seconds * (1.0 - reduction);
        
        Duration::from_secs(adjusted_seconds as u64)
    }
    
    pub fn create_copy(&self) -> Option<Self> {
        if self.bp_type == BlueprintType::Copy || !self.research_complete {
            return None;
        }
        
        // Copies have limited uses
        let uses = match self.quality_level {
            1 => 1,
            2 => 3,
            3 => 5,
            4 => 10,
            5 => 20,
            _ => 1,
        };
        
        Some(Self {
            id: Uuid::new_v4().to_string(),
            name: format!("{} (Copy)", self.name),
            description: self.description.clone(),
            category: self.category.clone(),
            bp_type: BlueprintType::Copy,
            ingredients: self.ingredients.clone(),
            output_item: self.output_item.clone(),
            output_item_type: self.output_item_type.clone(),
            output_quantity: self.output_quantity,
            research_progress: self.research_required, // Fully researched
            research_required: self.research_required,
            research_started: false,
            research_start_time: None,
            research_complete: true, // Copies are ready to use
            quality_level: self.quality_level,
            complexity: self.complexity,
            mass: self.mass,
            remaining_uses: Some(uses),
        })
    }
    
    pub fn use_blueprint(&mut self) -> bool {
        match self.bp_type {
            BlueprintType::Original => true, // Unlimited uses
            BlueprintType::Copy => {
                if let Some(uses) = self.remaining_uses {
                    if uses > 0 {
                        self.remaining_uses = Some(uses - 1);
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintLibrary {
    pub blueprints: Vec<Blueprint>,
}

#[allow(dead_code)]
impl BlueprintLibrary {
    pub fn new() -> Self {
        Self {
            blueprints: Vec::new(),
        }
    }
    
    pub fn add_blueprint(&mut self, blueprint: Blueprint) {
        self.blueprints.push(blueprint);
    }
    
    pub fn get_blueprint(&self, id: &str) -> Option<&Blueprint> {
        self.blueprints.iter().find(|bp| bp.id == id)
    }
    
    pub fn get_blueprint_mut(&mut self, id: &str) -> Option<&mut Blueprint> {
        self.blueprints.iter_mut().find(|bp| bp.id == id)
    }
    
    pub fn remove_blueprint(&mut self, id: &str) -> bool {
        if let Some(index) = self.blueprints.iter().position(|bp| bp.id == id) {
            self.blueprints.remove(index);
            true
        } else {
            false
        }
    }
    
    pub fn update_research(&mut self, research_skill: &Skill) {
        for blueprint in &mut self.blueprints {
            if blueprint.research_started && !blueprint.research_complete {
                blueprint.update_research(research_skill);
            }
        }
    }
}