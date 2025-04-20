use rand::Rng;
use serde::{Serialize, Deserialize};

use crate::models::player::Player;
use crate::models::item::{Item, ItemType};

#[derive(Serialize, Deserialize)]
pub struct MiningSystem {
    selected_resource: Option<String>,
}

impl MiningSystem {
    pub fn new() -> Self {
        MiningSystem {
            selected_resource: None,
        }
    }

    pub fn get_resources_for_system(&self, system_id: String) -> Vec<(String, u32)> {
        // Create a universe to get the system (this is a bit inefficient but works for now)
        let universe = crate::models::universe::Universe::new();
        
        if let Some(system) = universe.get_system(&system_id) {
            system.resources.clone()
        } else {
            Vec::new()
        }
    }

    pub fn mine_resource(&mut self, player: &mut Player, resource_index: usize) -> Option<String> {
        // Get available resources in the current system
        let resources = self.get_resources_for_system(player.current_system.id.clone());
        
        if resource_index >= resources.len() {
            return Some("Invalid resource selection".to_string());
        }
        
        let (resource_name, abundance) = &resources[resource_index];
        
        // Check if player has cargo space
        if player.inventory.remaining_capacity() < 1 {
            return Some("Not enough cargo space for mining".to_string());
        }
        
        // Calculate mining success chance based on ship's mining power and resource abundance
        let success_chance = (player.ship.mining_power as f32 * *abundance as f32 / 100.0) as u32;
        let success = rand::thread_rng().gen_range(0..100) < success_chance;
        
        if success {
            // Create the resource item
            let item = Item {
                name: resource_name.clone(),
                value: 50 + (*abundance / 2), // More abundant resources are worth less
                weight: 1,
                item_type: ItemType::Resource,
            };
            
            // Add to inventory
            player.inventory.add_item(item, 1);
            
            Some(format!("Successfully mined 1 {}", resource_name))
        } else {
            Some(format!("Failed to mine {}", resource_name))
        }
    }

    pub fn select_resource(&mut self, resource_name: String) {
        self.selected_resource = Some(resource_name);
    }
}
