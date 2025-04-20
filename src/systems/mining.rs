use std::time::{Duration, Instant};
use rand::Rng;
use serde::{Serialize, Deserialize};
use crate::utils::serde::option_instant_serde;

use crate::models::player::Player;
use crate::models::item::{Item, ItemType, ResourceType};
use crate::models::universe::{Universe, ResourceFieldType, ResourceField};
use crate::systems::time::TimeSystem;

// Represents an active mining operation
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MiningOperation {
    pub resource_name: String,
    pub resource_type: ResourceType,
    pub field_type: ResourceFieldType,
    pub system_id: String,
    pub yield_per_cycle: u32,     // How much is extracted each cycle
    pub cycle_time_seconds: u32,  // How long each mining cycle takes
    #[serde(with = "option_instant_serde")]
    pub last_cycle: Option<Instant>, // When the last yield was produced
    pub total_mined: u32,         // Total amount mined in this operation
    pub active: bool,             // Whether mining is currently happening
}

#[derive(Serialize, Deserialize)]
pub struct MiningSystem {
    selected_resource: Option<String>,
    selected_field: Option<usize>,
    #[serde(with = "option_instant_serde")]
    mining_start_time: Option<Instant>,
    active_operations: Vec<MiningOperation>,
}

#[allow(dead_code)]
impl MiningSystem {
    pub fn new() -> Self {
        MiningSystem {
            selected_resource: None,
            selected_field: None,
            mining_start_time: None,
            active_operations: Vec::new(),
        }
    }

    pub fn get_resources_for_system(&self, system_id: &str) -> Vec<ResourceField> {
        // Create a universe to get the system (this is a bit inefficient but works for now)
        let universe = Universe::new();
        
        if let Some(system) = universe.get_system(system_id) {
            system.resource_fields.clone()
        } else {
            Vec::new()
        }
    }
    
    // Update mining operations in real-time
    pub fn update(&mut self, player: &mut Player, _time_system: &TimeSystem, _delta_time: Duration) {
        // Create a copy of the operations for processing
        let mut operations_to_process = Vec::new();
        for op in &self.active_operations {
            if op.active {
                operations_to_process.push(op.clone());
            }
        }
        
        // Process each active mining operation outside the mutable borrow
        for operation in operations_to_process {
            let now = Instant::now();
            let cycle_duration = Duration::from_secs(operation.cycle_time_seconds as u64);
            
            // Skip if this is a new operation without a last cycle
            if operation.last_cycle.is_none() {
                continue;
            }
            
            // Calculate how many cycles have passed
            let last_cycle = operation.last_cycle.unwrap();
            let time_since_last_cycle = now.duration_since(last_cycle);
            
            if time_since_last_cycle >= cycle_duration {
                // Calculate how many complete cycles have passed
                let cycles = time_since_last_cycle.as_secs() / operation.cycle_time_seconds as u64;
                
                if cycles > 0 {
                    // Calculate yield based on elapsed cycles
                    let yield_amount = operation.yield_per_cycle * cycles as u32;
                    
                    // Add resources to the player's inventory (avoiding borrow issues)
                    self.add_resource_to_player(player, &operation.resource_name, 
                                              &operation.resource_type, yield_amount);
                    
                    // Now update our operation in the actual vector
                    if let Some(op) = self.active_operations.iter_mut()
                        .find(|o| o.active && o.resource_name == operation.resource_name && 
                              o.system_id == operation.system_id) {
                        // Update operation stats
                        op.total_mined += yield_amount;
                        
                        // Update last cycle time (only counting complete cycles)
                        op.last_cycle = Some(last_cycle + (cycle_duration * cycles as u32));
                    }
                    
                    // Add mining experience to player skills
                    player.skills.gain_mining_experience(yield_amount);
                }
            }
        }
    }
    
    // Helper function to add resources to the appropriate specialized cargo bay
    fn add_resource_to_player(&self, player: &mut Player, resource_name: &str, 
                           resource_type: &ResourceType, amount: u32) {
        // Create the resource item
        let item = Item {
            name: resource_name.to_string(),
            value: 50 + (amount / 2), // Base value plus bonus for larger amounts
            weight: 1,
            item_type: ItemType::Resource(resource_type.clone()),
        };
        
        // TODO: Store resources in specialized cargo bays
        // For now, just add to regular inventory
        player.inventory.add_item(item, amount);
    }

    // Start a new mining operation at a resource field
    pub fn start_mining_operation(&mut self, player: &Player, field_index: usize) -> Option<String> {
        // Get resource fields in current system
        let fields = self.get_resources_for_system(&player.current_system.id);
        
        if field_index >= fields.len() {
            return Some("Invalid resource field selection".to_string());
        }
        
        let field = &fields[field_index];
        
        // Check if player has the required mining skill level
        let required_level = field.field_type.required_mining_level();
        if player.skills.get_mining_level() < required_level {
            return Some(format!(
                "Mining level {} required for {} (current level: {})",
                required_level,
                field.field_type.to_string(),
                player.skills.get_mining_level()
            ));
        }
        
        // Determine resource type based on field type
        let resource_type = match field.field_type {
            ResourceFieldType::AsteroidField => ResourceType::Mineral,
            ResourceFieldType::IceField => ResourceType::Ice,
            ResourceFieldType::GasField => ResourceType::Gas,
            ResourceFieldType::MoonResidue => ResourceType::Lunar,
            ResourceFieldType::StarCorona => ResourceType::Stellar,
            ResourceFieldType::BlackHoleAccretion => ResourceType::Exotic,
        };
        
        // Calculate yield based on ship mining power and player skill
        let base_yield = player.ship.mining_power;
        let skill_multiplier = 1.0 + (player.skills.get_mining_level() as f32 * 0.1);
        let yield_per_cycle = (base_yield as f32 * skill_multiplier) as u32;
        
        // Choose a resource from the field
        if field.resources.is_empty() {
            return Some("No resources available in this field".to_string());
        }
        
        // Select a resource based on abundance
        let mut rng = rand::thread_rng();
        let resource_index = rng.gen_range(0..field.resources.len());
        let (resource_name, _) = &field.resources[resource_index];
        
        // Create and add the mining operation
        let operation = MiningOperation {
            resource_name: resource_name.clone(),
            resource_type,
            field_type: field.field_type.clone(),
            system_id: player.current_system.id.clone(),
            yield_per_cycle,
            cycle_time_seconds: 10, // Base cycle time of 10 seconds
            last_cycle: Some(Instant::now()),
            total_mined: 0,
            active: true,
        };
        
        self.active_operations.push(operation);
        self.selected_field = Some(field_index);
        
        Some(format!(
            "Started mining {} in {}. Yield: {} units per cycle.",
            resource_name,
            field.field_type.to_string(),
            yield_per_cycle
        ))
    }
    
    // Stop all active mining operations
    pub fn stop_mining(&mut self) -> Option<String> {
        let mut total_mined = 0;
        
        for operation in &mut self.active_operations {
            if operation.active {
                operation.active = false;
                total_mined += operation.total_mined;
            }
        }
        
        if total_mined > 0 {
            Some(format!("Mining operations stopped. Total yield: {} units.", total_mined))
        } else {
            Some("No active mining operations to stop.".to_string())
        }
    }

    // Get the status of active mining operations
    pub fn get_mining_status(&self) -> Vec<String> {
        let mut status = Vec::new();
        
        for operation in &self.active_operations {
            if operation.active {
                status.push(format!(
                    "{}: Mining {} from {}. Total mined: {} units.",
                    operation.system_id,
                    operation.resource_name,
                    operation.field_type.to_string(),
                    operation.total_mined
                ));
            }
        }
        
        status
    }

    // Legacy methods for backward compatibility
    pub fn mine_resource(&mut self, player: &mut Player, resource_index: usize) -> Option<String> {
        // Get available resource fields in the current system
        let fields = self.get_resources_for_system(&player.current_system.id);
        
        if fields.is_empty() {
            return Some("No resource fields available in this system".to_string());
        }
        
        // Use the first field for legacy compatibility
        let field = &fields[0];
        
        if resource_index >= field.resources.len() {
            return Some("Invalid resource selection".to_string());
        }
        
        let (resource_name, abundance) = &field.resources[resource_index];
        
        // Check if player has cargo space
        if player.inventory.remaining_capacity() < 1 {
            return Some("Not enough cargo space for mining".to_string());
        }
        
        // Calculate mining success chance based on ship's mining power and resource abundance
        let success_chance = (player.ship.mining_power as f32 * *abundance as f32 / 100.0) as u32;
        let success = rand::thread_rng().gen_range(0..100) < success_chance;
        
        if success {
            // Determine resource type based on field type
            let resource_type = match field.field_type {
                ResourceFieldType::AsteroidField => ResourceType::Mineral,
                ResourceFieldType::IceField => ResourceType::Ice,
                ResourceFieldType::GasField => ResourceType::Gas,
                ResourceFieldType::MoonResidue => ResourceType::Lunar,
                ResourceFieldType::StarCorona => ResourceType::Stellar,
                ResourceFieldType::BlackHoleAccretion => ResourceType::Exotic,
            };
            
            // Create the resource item
            let item = Item {
                name: resource_name.clone(),
                value: 50 + (*abundance / 2),
                weight: 1,
                item_type: ItemType::Resource(resource_type),
            };
            
            // Add to inventory
            player.inventory.add_item(item, 1);
            
            // Add a small amount of mining experience
            player.skills.gain_mining_experience(1);
            
            Some(format!("Successfully mined 1 {}", resource_name))
        } else {
            Some(format!("Failed to mine {}", resource_name))
        }
    }

    pub fn select_resource(&mut self, resource_name: String) {
        self.selected_resource = Some(resource_name);
    }
}
