use std::collections::HashMap;
use uuid::Uuid;

use crate::models::item::{Item, ItemType, ResourceType};

/// Represents a recipe/blueprint for crafting an item
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CraftingRecipe {
    pub id: String,
    pub name: String,
    pub description: String,
    pub input_items: Vec<(String, u32)>,  // (item name, quantity)
    pub output_item: Item,
    pub output_quantity: u32,
    pub crafting_time: u32,  // Time in seconds to craft
    pub skill_requirements: HashMap<String, u32>, // Skill name and minimum level required
    pub facility_requirements: Vec<String>, // Required facilities (e.g., "Forge", "Laboratory")
    pub faction_requirements: Option<String>, // Faction-specific recipes
    pub difficulty: u32, // 1-100, affects quality and success chance
    pub discovery_requirements: Option<String>, // How players discover this recipe
}

/// Represents the quality result of crafting
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CraftingQuality {
    Poor,
    Standard,
    Fine,
    Superior,
    Exceptional,
    Masterwork,
}

/// Tracks a crafting job in progress
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CraftingJob {
    pub id: String,
    pub player_id: String,
    pub recipe_id: String,
    pub start_time: u64,
    pub end_time: u64,
    pub quality_bonus: f32,
    pub is_completed: bool,
    pub input_items: Vec<(String, u32)>, // Materials committed
}

/// Blueprint for crafting items
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Blueprint {
    pub id: String,
    pub name: String,
    pub description: String,
    pub recipe_id: String,   // Reference to the underlying recipe
    pub discovered: bool,    // Whether player has discovered this blueprint
    pub learned: bool,       // Whether player has learned this blueprint
    pub discovery_location: Option<String>, // Where this blueprint can be found
    pub rarity: BlueprintRarity,            // How rare/valuable this blueprint is
}

/// Rarity levels for blueprints
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum BlueprintRarity {
    Common,
    Uncommon,
    Rare,
    VeryRare,
    Exceptional,
    Legendary,
}

/// System for handling all crafting mechanics
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CraftingSystem {
    pub recipes: HashMap<String, CraftingRecipe>,
    pub active_jobs: HashMap<String, CraftingJob>,
    pub player_known_recipes: HashMap<String, Vec<String>>, // player_id -> recipe_ids
    pub blueprints: Vec<Blueprint>,             // All blueprints in the game
    pub selected_blueprint_index: Option<usize>, // Currently selected blueprint
}

impl CraftingSystem {
    pub fn new() -> Self {
        CraftingSystem {
            recipes: HashMap::new(),
            active_jobs: HashMap::new(),
            player_known_recipes: HashMap::new(),
            blueprints: Vec::new(),
            selected_blueprint_index: None,
        }
    }
    
    /// Register a new crafting recipe
    pub fn register_recipe(&mut self, recipe: CraftingRecipe) -> String {
        let id = recipe.id.clone();
        self.recipes.insert(id.clone(), recipe);
        id
    }
    
    /// Create a standard recipe with default values
    pub fn create_recipe(
        name: &str,
        description: &str,
        inputs: Vec<(String, u32)>,
        output: Item,
        output_quantity: u32,
        crafting_time: u32,
    ) -> CraftingRecipe {
        CraftingRecipe {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            description: description.to_string(),
            input_items: inputs,
            output_item: output,
            output_quantity,
            crafting_time,
            skill_requirements: HashMap::new(),
            facility_requirements: Vec::new(),
            faction_requirements: None,
            difficulty: 1,
            discovery_requirements: None,
        }
    }
    
    /// Add skill requirements to a recipe
    pub fn add_skill_requirement(&mut self, recipe_id: &str, skill: &str, level: u32) -> bool {
        if let Some(recipe) = self.recipes.get_mut(recipe_id) {
            recipe.skill_requirements.insert(skill.to_string(), level);
            true
        } else {
            false
        }
    }
    
    /// Add facility requirements to a recipe
    pub fn add_facility_requirement(&mut self, recipe_id: &str, facility: &str) -> bool {
        if let Some(recipe) = self.recipes.get_mut(recipe_id) {
            recipe.facility_requirements.push(facility.to_string());
            true
        } else {
            false
        }
    }
    
    /// Check if a player knows a specific recipe
    pub fn player_knows_recipe(&self, player_id: &str, recipe_id: &str) -> bool {
        if let Some(known_recipes) = self.player_known_recipes.get(player_id) {
            known_recipes.contains(&recipe_id.to_string())
        } else {
            false
        }
    }
    
    /// Teach a recipe to a player
    pub fn learn_recipe(&mut self, player_id: &str, recipe_id: &str) -> bool {
        // Verify recipe exists
        if !self.recipes.contains_key(recipe_id) {
            return false;
        }
        
        let known_recipes = self.player_known_recipes
            .entry(player_id.to_string())
            .or_insert_with(Vec::new);
            
        if !known_recipes.contains(&recipe_id.to_string()) {
            known_recipes.push(recipe_id.to_string());
        }
        
        true
    }
    
    /// Get all recipes a player knows
    pub fn get_player_recipes(&self, player_id: &str) -> Vec<&CraftingRecipe> {
        let known_ids = match self.player_known_recipes.get(player_id) {
            Some(ids) => ids,
            None => return Vec::new(),
        };
        
        known_ids.iter()
            .filter_map(|id| self.recipes.get(id))
            .collect()
    }
    
    /// Start a crafting job
    pub fn start_crafting_job(
        &mut self,
        player_id: &str,
        recipe_id: &str,
        current_time: u64,
        player_skills: &HashMap<String, u32>,
        available_facilities: &[String],
    ) -> Result<(String, u64), String> {
        // Get the recipe
        let recipe = match self.recipes.get(recipe_id) {
            Some(r) => r,
            None => return Err("Recipe not found".to_string()),
        };
        
        // Check if player knows this recipe
        if !self.player_knows_recipe(player_id, recipe_id) {
            return Err("You don't know this recipe".to_string());
        }
        
        // Check skill requirements
        for (skill, required_level) in &recipe.skill_requirements {
            let player_level = player_skills.get(skill).cloned().unwrap_or(0);
            if player_level < *required_level {
                return Err(format!("You need {} level {} (you have {})",
                                  skill, required_level, player_level));
            }
        }
        
        // Check facility requirements
        for required_facility in &recipe.facility_requirements {
            if !available_facilities.contains(required_facility) {
                return Err(format!("You need access to a {}", required_facility));
            }
        }
        
        // Check faction requirements if any
        // For now we'll skip this since we don't have faction info here
        
        // Calculate quality bonus based on skills
        let mut quality_bonus = 0.0;
        for (skill, required_level) in &recipe.skill_requirements {
            let player_level = player_skills.get(skill).cloned().unwrap_or(0);
            
            // Skill levels above the requirement provide quality bonuses
            if player_level > *required_level {
                let skill_bonus = (player_level - *required_level) as f32 * 0.05;
                quality_bonus += skill_bonus;
            }
        }
        
        // Create the job
        let job_id = Uuid::new_v4().to_string();
        let end_time = current_time + recipe.crafting_time as u64;
        
        let job = CraftingJob {
            id: job_id.clone(),
            player_id: player_id.to_string(),
            recipe_id: recipe_id.to_string(),
            start_time: current_time,
            end_time,
            quality_bonus,
            is_completed: false,
            input_items: recipe.input_items.clone(),
        };
        
        self.active_jobs.insert(job_id.clone(), job);
        
        Ok((job_id, end_time))
    }
    
    /// Check if a crafting job is completed
    pub fn check_job_status(&mut self, job_id: &str, current_time: u64) -> Option<bool> {
        let job = match self.active_jobs.get_mut(job_id) {
            Some(j) => j,
            None => return None, // Job not found
        };
        
        if job.is_completed {
            return Some(true); // Already completed
        }
        
        let is_complete = current_time >= job.end_time;
        job.is_completed = is_complete;
        
        Some(is_complete)
    }
    
    /// Craft an item from a blueprint
    pub fn craft_item(&mut self, player: &mut crate::models::player::Player, blueprint_idx: usize) -> Result<String, String> {
        if blueprint_idx >= self.blueprints.len() {
            return Err("Invalid blueprint index".to_string());
        }
        
        // First, clone the necessary data to avoid borrowing issues
        let blueprint_clone = self.blueprints[blueprint_idx].clone();
        
        // Check if player has discovered and learned the blueprint
        if !blueprint_clone.discovered {
            return Err("You haven't discovered this blueprint yet".to_string());
        }
        
        if !blueprint_clone.learned {
            return Err("You haven't learned how to craft this item yet".to_string());
        }
        
        // Get the recipe
        let recipe_id = blueprint_clone.recipe_id.clone();
        let recipe_clone = match self.recipes.get(&recipe_id) {
            Some(r) => r.clone(),
            None => return Err("Recipe not found for this blueprint".to_string()),
        };
        
        // Check if player knows the recipe
        if !self.player_knows_recipe(&player.id, &recipe_id) {
            return Err("You don't know this recipe".to_string());
        }
        
        // Check if player has required materials
        for (item_name, required_qty) in &recipe_clone.input_items {
            let player_qty = player.inventory.get_item_quantity(item_name);
            if player_qty < *required_qty {
                return Err(format!("Not enough {}. Need {}, have {}", 
                                  item_name, required_qty, player_qty));
            }
        }
        
        // Check skill requirements
        for (skill, required_level) in &recipe_clone.skill_requirements {
            // Extract skill level from SkillSet by converting skill name to SkillCategory
            let skill_category = match skill.as_str() {
                "Mining" => Some(crate::models::skills::SkillCategory::Mining),
                "Trading" => Some(crate::models::skills::SkillCategory::Trading),
                "Combat" => Some(crate::models::skills::SkillCategory::Combat),
                "Navigation" => Some(crate::models::skills::SkillCategory::Navigation),
                "Research" => Some(crate::models::skills::SkillCategory::Research),
                "Engineering" => Some(crate::models::skills::SkillCategory::Engineering),
                _ => None,
            };
            
            let player_level = if let Some(category) = skill_category {
                player.skills.get_skill(&category).map_or(0, |s| s.level as u32)
            } else {
                0
            };
            if player_level < *required_level {
                return Err(format!("You need {} level {} (you have {})",
                                 skill, required_level, player_level));
            }
        }
        
        // Start crafting job - determine appropriate facility types
        let available_facilities = vec!["Basic Crafting".to_string()]; // Placeholder - should come from player location
        
        // Get current time
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or(std::time::Duration::from_secs(0))
            .as_secs();
            
        // Convert skill set to map for compatibility
        let skills_map = convert_skillset_to_map(&player.skills);
        
        // Start the job
        let result = self.start_crafting_job(
            &player.id,
            &recipe_id,
            current_time,
            &skills_map,
            &available_facilities,
        )?;
        
        // Consume materials from player inventory
        for (item_name, required_qty) in &recipe_clone.input_items {
            // Using remove_item also handles removing completely if quantity is zero
            player.inventory.remove_item(item_name, *required_qty);
        }
        
        // Return the job ID
        Ok(result.0)
    }
    
    /// Collect a completed crafting job
    pub fn collect_completed_job(
        &mut self,
        job_id: &str,
        player_id: &str,
        current_time: u64,
    ) -> Result<(Item, u32, CraftingQuality), String> {
        // Check job exists and belongs to player
        let job = match self.active_jobs.get(job_id) {
            Some(j) => {
                if j.player_id != player_id {
                    return Err("This job doesn't belong to you".to_string());
                }
                j.clone()
            },
            None => return Err("Job not found".to_string()),
        };
        
        // Check if job is ready
        if !job.is_completed && current_time < job.end_time {
            return Err(format!("Job not completed yet. Ready in {} seconds", 
                            job.end_time - current_time));
        }
        
        // Get the recipe
        let recipe = match self.recipes.get(&job.recipe_id) {
            Some(r) => r,
            None => return Err("Recipe data missing".to_string()),
        };
        
        // Remove the job
        self.active_jobs.remove(job_id);
        
        // Calculate job quality
        let quality = self.calculate_job_quality(&job, recipe);
        
        // Apply quality bonuses to the item
        let mut output_item = recipe.output_item.clone();
        
        // Adjust item value based on quality
        let quality_multiplier = match quality {
            CraftingQuality::Poor => 0.7,
            CraftingQuality::Standard => 1.0,
            CraftingQuality::Fine => 1.2,
            CraftingQuality::Superior => 1.5,
            CraftingQuality::Exceptional => 2.0,
            CraftingQuality::Masterwork => 3.0,
        };
        
        output_item.value = (output_item.value as f32 * quality_multiplier) as u32;
        
        // Return the crafted item and quantity
        Ok((output_item, recipe.output_quantity, quality))
    }
    
    /// Calculate the quality of a crafting job
    fn calculate_job_quality(&self, job: &CraftingJob, _recipe: &CraftingRecipe) -> CraftingQuality {
        // Base chance for quality levels
        let base_chances = [
            (CraftingQuality::Poor, 0.1),
            (CraftingQuality::Standard, 0.6),
            (CraftingQuality::Fine, 0.2),
            (CraftingQuality::Superior, 0.07),
            (CraftingQuality::Exceptional, 0.025),
            (CraftingQuality::Masterwork, 0.005),
        ];
        
        // Apply quality bonus
        let mut adjusted_chances = base_chances.clone();
        
        // Apply skill quality bonus - reduces chance of poor, increases chance of better results
        if job.quality_bonus > 0.0 {
            // Reduce poor chance
            adjusted_chances[0].1 = (adjusted_chances[0].1 - job.quality_bonus * 0.1).max(0.01);
            
            // Reduce standard chance if bonus is high enough
            if job.quality_bonus > 0.2 {
                adjusted_chances[1].1 = (adjusted_chances[1].1 - (job.quality_bonus - 0.2) * 0.1).max(0.2);
            }
            
            // Redistribute to higher qualities
            let reduced = 0.1 + (job.quality_bonus - 0.2).max(0.0) * 0.1;
            let to_distribute = reduced / 4.0; // Distribute to Fine, Superior, Exceptional, Masterwork
            
            adjusted_chances[2].1 += to_distribute;
            adjusted_chances[3].1 += to_distribute;
            adjusted_chances[4].1 += to_distribute;
            adjusted_chances[5].1 += to_distribute;
        }
        
        // Roll for quality
        let roll = rand::random::<f32>();
        let mut cumulative = 0.0;
        
        for (quality, chance) in adjusted_chances.iter() {
            cumulative += chance;
            if roll <= cumulative {
                return *quality;
            }
        }
        
        // Default to standard
        CraftingQuality::Standard
    }
    
    /// Set up a list of basic crafting recipes for testing
    pub fn setup_basic_recipes(&mut self) {
        // Basic resources refining
        let mineral_alloy = Item::new(
            "Mineral Alloy", 
            150, 
            1, 
            ItemType::Component
        );
        
        let recipe = Self::create_recipe(
            "Basic Mineral Alloy", 
            "Refine raw minerals into a useful alloy", 
            vec![
                ("Iron Ore".to_string(), 2),
                ("Copper Ore".to_string(), 1),
            ],
            mineral_alloy,
            1,
            300, // 5 minutes
        );
        
        self.register_recipe(recipe);
        
        // Advanced component
        let power_cell = Item::new(
            "Power Cell", 
            500, 
            2, 
            ItemType::Component
        );
        
        let recipe = Self::create_recipe(
            "Standard Power Cell", 
            "Craft a basic power cell for various devices", 
            vec![
                ("Mineral Alloy".to_string(), 1),
                ("Energy Crystal".to_string(), 2),
                ("Conducting Wire".to_string(), 3),
            ],
            power_cell,
            1,
            600, // 10 minutes
        );
        
        let recipe_id = self.register_recipe(recipe);
        self.add_facility_requirement(&recipe_id, "Electronics Lab");
        self.add_skill_requirement(&recipe_id, "Electronics", 2);
        
        // Ship module
        let shield_generator = Item::new(
            "Basic Shield Generator", 
            2000, 
            10, 
            ItemType::ShipModule
        );
        
        let recipe = Self::create_recipe(
            "Basic Shield Generator", 
            "A defensive module that generates an energy shield around your ship", 
            vec![
                ("Power Cell".to_string(), 3),
                ("Shield Emitter".to_string(), 1),
                ("Mineral Alloy".to_string(), 5),
                ("Energy Capacitor".to_string(), 2),
            ],
            shield_generator,
            1,
            1800, // 30 minutes
        );
        
        let recipe_id = self.register_recipe(recipe);
        self.add_facility_requirement(&recipe_id, "Shipyard");
        self.add_skill_requirement(&recipe_id, "Engineering", 3);
        self.add_skill_requirement(&recipe_id, "Electronics", 2);
    }
    
    /// Process all active crafting jobs
    pub fn update_jobs(&mut self, current_time: u64) -> Vec<(String, String)> { // (job_id, player_id)
        let mut completed_jobs = Vec::new();
        
        for (job_id, job) in &mut self.active_jobs {
            if !job.is_completed && current_time >= job.end_time {
                job.is_completed = true;
                completed_jobs.push((job_id.clone(), job.player_id.clone()));
            }
        }
        
        completed_jobs
    }
    
    /// Get available recipes for a player that match item type and skill requirements
    pub fn find_available_recipes(
        &self,
        player_id: &str,
        item_type: Option<&ItemType>,
        player_skills: &HashMap<String, u32>,
    ) -> Vec<&CraftingRecipe> {
        // Get all recipes the player knows
        let known_recipes = self.get_player_recipes(player_id);
        
        // Filter by item type and skill requirements
        known_recipes.into_iter()
            .filter(|recipe| {
                // Filter by item type if specified
                if let Some(required_type) = item_type {
                    if &recipe.output_item.item_type != required_type {
                        return false;
                    }
                }
                
                // Check if player meets skill requirements
                for (skill, required_level) in &recipe.skill_requirements {
                    let player_level = player_skills.get(skill).cloned().unwrap_or(0);
                    if player_level < *required_level {
                        return false;
                    }
                }
                
                true
            })
            .collect()
    }
    
    /// Find recipes that can be crafted with the player's available items
    pub fn find_craftable_recipes(
        &self,
        player_id: &str,
        player_inventory: &Inventory,
        player_skills: &HashMap<String, u32>,
        available_facilities: &[String],
    ) -> Vec<&CraftingRecipe> {
        // Get all recipes the player knows and meets skill requirements for
        let available_recipes = self.find_available_recipes(
            player_id,
            None,
            player_skills,
        );
        
        // Filter by available materials and facilities
        available_recipes.into_iter()
            .filter(|recipe| {
                // Check facilities first (cheaper check)
                for facility in &recipe.facility_requirements {
                    if !available_facilities.contains(facility) {
                        return false;
                    }
                }
                
                // Check if player has all required materials
                for (item_name, required_qty) in &recipe.input_items {
                    let player_qty = player_inventory.get_item_quantity(item_name);
                    if player_qty < *required_qty {
                        return false;
                    }
                }
                
                true
            })
            .collect()
    }
    
    /// Create a new blueprint from a recipe
    pub fn create_blueprint(
        &mut self,
        recipe_id: &str,
        name: &str,
        description: &str,
        discovered: bool,
        learned: bool,
        discovery_location: Option<&str>,
        rarity: BlueprintRarity,
    ) -> Result<String, String> {
        // Verify recipe exists
        if !self.recipes.contains_key(recipe_id) {
            return Err("Recipe does not exist".to_string());
        }
        
        let blueprint = Blueprint {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            description: description.to_string(),
            recipe_id: recipe_id.to_string(),
            discovered,
            learned,
            discovery_location: discovery_location.map(|s| s.to_string()),
            rarity,
        };
        
        let id = blueprint.id.clone();
        self.blueprints.push(blueprint);
        
        Ok(id)
    }
    
    /// Get all available blueprints
    pub fn get_available_blueprints(&self) -> Vec<&Blueprint> {
        self.blueprints.iter().filter(|bp| bp.discovered).collect()
    }
    
    /// Get the currently selected blueprint index
    pub fn get_selected_blueprint_index(&self) -> Option<usize> {
        self.selected_blueprint_index
    }
    
    /// Set the selected blueprint index
    pub fn set_selected_blueprint_index(&mut self, index: Option<usize>) {
        // Verify the index is valid
        if let Some(idx) = index {
            if idx >= self.blueprints.len() {
                return;
            }
        }
        
        self.selected_blueprint_index = index;
    }
    
    /// Get the currently selected blueprint
    pub fn get_selected_blueprint(&self) -> Option<&Blueprint> {
        match self.selected_blueprint_index {
            Some(idx) => self.blueprints.get(idx),
            None => None,
        }
    }
    
    /// Discover a blueprint (makes it visible to the player)
    pub fn discover_blueprint(&mut self, blueprint_id: &str) -> bool {
        for bp in &mut self.blueprints {
            if bp.id == blueprint_id {
                bp.discovered = true;
                return true;
            }
        }
        false
    }
    
    /// Learn a blueprint (player can now craft this item)
    pub fn learn_blueprint(&mut self, blueprint_id: &str, player_id: &str) -> bool {
        // First find the blueprint
        let recipe_id = match self.blueprints.iter_mut().find(|bp| bp.id == blueprint_id) {
            Some(bp) => {
                bp.learned = true;
                bp.recipe_id.clone()
            },
            None => return false,
        };
        
        // Now learn the associated recipe
        self.learn_recipe(player_id, &recipe_id)
    }
    
    /// Initialize test blueprints
    pub fn setup_basic_blueprints(&mut self) {
        // First, make sure we have recipes
        if self.recipes.is_empty() {
            self.setup_basic_recipes();
        }
        
        // Collect recipe data first to avoid borrowing conflicts
        let recipe_data: Vec<(String, String, String, u32)> = self.recipes
            .iter()
            .map(|(id, recipe)| (
                id.clone(),
                recipe.name.clone(),
                recipe.description.clone(),
                recipe.difficulty
            ))
            .collect();
        
        // Now add blueprints using the collected data
        for (recipe_id, name, description, difficulty) in recipe_data {
            let rarity = match difficulty {
                0..=20 => BlueprintRarity::Common,
                21..=40 => BlueprintRarity::Uncommon,
                41..=60 => BlueprintRarity::Rare,
                61..=80 => BlueprintRarity::VeryRare,
                81..=90 => BlueprintRarity::Exceptional,
                _ => BlueprintRarity::Legendary,
            };
            
            let _ = self.create_blueprint(
                &recipe_id,
                &name,
                &description,
                true, // Discovered for testing
                false, // Not learned yet
                None,
                rarity,
            );
        }
    }
}

/// Crafting table - a facility where crafting can be performed
#[derive(Debug, Clone)]
pub struct CraftingTable {
    pub id: String,
    pub name: String,
    pub location_id: String,
    pub facility_types: Vec<String>,    // What kind of crafting this table supports
    pub available_tools: Vec<String>,   // Special tools available at this table
    pub quality_modifier: f32,          // Bonus to crafting quality at this table
    pub speed_modifier: f32,            // Crafting speed modifier (1.0 = normal)
    pub owner_id: Option<String>,       // Player who owns this table, if any
    pub public_access: bool,            // Whether other players can use it
    pub usage_fee: Option<u32>,         // Fee to use the table if not the owner
}

impl CraftingTable {
    pub fn new(
        name: &str,
        location_id: &str,
        facility_types: Vec<String>,
    ) -> Self {
        CraftingTable {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            location_id: location_id.to_string(),
            facility_types,
            available_tools: Vec::new(),
            quality_modifier: 1.0,
            speed_modifier: 1.0,
            owner_id: None,
            public_access: true,
            usage_fee: None,
        }
    }
    
    pub fn upgrade_quality(&mut self, amount: f32) {
        self.quality_modifier += amount;
    }
    
    pub fn upgrade_speed(&mut self, amount: f32) {
        self.speed_modifier += amount;
    }
    
    pub fn add_tool(&mut self, tool: &str) {
        if !self.available_tools.contains(&tool.to_string()) {
            self.available_tools.push(tool.to_string());
        }
    }
    
    pub fn set_owner(&mut self, player_id: &str, public_access: bool, usage_fee: Option<u32>) {
        self.owner_id = Some(player_id.to_string());
        self.public_access = public_access;
        self.usage_fee = usage_fee;
    }
    
    pub fn can_use(&self, player_id: &str) -> Result<u32, String> {
        // Check if table is public or player is owner
        if let Some(owner_id) = &self.owner_id {
            if owner_id == player_id {
                return Ok(0); // Owner uses for free
            }
            
            if !self.public_access {
                return Err("This crafting table is private".to_string());
            }
            
            // Return usage fee if any
            if let Some(fee) = self.usage_fee {
                return Ok(fee);
            }
        }
        
        // Public table with no fee
        Ok(0)
    }
}

/// Helper function to convert a SkillSet to a HashMap<String, u32> for compatibility
fn convert_skillset_to_map(skills: &crate::models::skills::SkillSet) -> HashMap<String, u32> {
    let mut skill_map = HashMap::new();
    
    for skill in &skills.skills {
        let skill_name = match skill.category {
            crate::models::skills::SkillCategory::Mining => "Mining",
            crate::models::skills::SkillCategory::Trading => "Trading",
            crate::models::skills::SkillCategory::Combat => "Combat",
            crate::models::skills::SkillCategory::Navigation => "Navigation",
            crate::models::skills::SkillCategory::Research => "Research",
            crate::models::skills::SkillCategory::Engineering => "Engineering",
        };
        
        skill_map.insert(skill_name.to_string(), skill.level as u32);
    }
    
    skill_map
}