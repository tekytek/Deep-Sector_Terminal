use crate::models::player::Player;
use crate::models::item::{Item, ItemType};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Blueprint {
    pub name: String,
    pub ingredients: Vec<(String, u32)>,
    pub output: Item,
    pub output_quantity: u32,
}

#[derive(Serialize, Deserialize)]
pub struct CraftingSystem {
    blueprints: Vec<Blueprint>,
    selected_blueprint: Option<usize>,
}

impl CraftingSystem {
    pub fn new() -> Self {
        let mut blueprints = Vec::new();
        
        // Add some basic blueprints
        
        // Power Cell
        blueprints.push(Blueprint {
            name: "Power Cell".to_string(),
            ingredients: vec![
                ("Copper".to_string(), 2),
                ("Iron".to_string(), 1),
            ],
            output: Item {
                name: "Power Cell".to_string(),
                value: 250,
                weight: 1,
                item_type: ItemType::Component,
            },
            output_quantity: 1,
        });
        
        // Computer Chip
        blueprints.push(Blueprint {
            name: "Computer Chip".to_string(),
            ingredients: vec![
                ("Silver".to_string(), 1),
                ("Gold".to_string(), 1),
            ],
            output: Item {
                name: "Computer Chip".to_string(),
                value: 350,
                weight: 1,
                item_type: ItemType::Component,
            },
            output_quantity: 1,
        });
        
        // Shield Generator
        blueprints.push(Blueprint {
            name: "Shield Generator".to_string(),
            ingredients: vec![
                ("Power Cell".to_string(), 2),
                ("Titanium".to_string(), 3),
            ],
            output: Item {
                name: "Shield Generator".to_string(),
                value: 800,
                weight: 2,
                item_type: ItemType::Component,
            },
            output_quantity: 1,
        });
        
        // Medical Supplies
        blueprints.push(Blueprint {
            name: "Medical Supplies".to_string(),
            ingredients: vec![
                ("Water".to_string(), 2),
                ("Oxygen".to_string(), 1),
            ],
            output: Item {
                name: "Medical Supplies".to_string(),
                value: 600,
                weight: 3,
                item_type: ItemType::Product,
            },
            output_quantity: 1,
        });
        
        // Ship Parts
        blueprints.push(Blueprint {
            name: "Ship Parts".to_string(),
            ingredients: vec![
                ("Titanium".to_string(), 3),
                ("Computer Chip".to_string(), 1),
                ("Power Cell".to_string(), 1),
            ],
            output: Item {
                name: "Ship Parts".to_string(),
                value: 1200,
                weight: 4,
                item_type: ItemType::Product,
            },
            output_quantity: 1,
        });
        
        CraftingSystem {
            blueprints,
            selected_blueprint: None,
        }
    }

    pub fn get_available_blueprints(&self) -> Vec<(String, Vec<(String, u32)>)> {
        self.blueprints.iter()
            .map(|bp| (bp.name.clone(), bp.ingredients.clone()))
            .collect()
    }

    pub fn select_blueprint(&mut self, index: usize) {
        if index < self.blueprints.len() {
            self.selected_blueprint = Some(index);
        }
    }

    pub fn get_selected_blueprint_index(&self) -> Option<usize> {
        self.selected_blueprint
    }

    pub fn craft_item(&mut self, player: &mut Player, blueprint_index: usize) -> Option<String> {
        // Check if player is docked
        if !player.is_docked {
            return Some("You must be docked at a station to craft items".to_string());
        }
        
        if blueprint_index >= self.blueprints.len() {
            return Some("Invalid blueprint selection".to_string());
        }
        
        let blueprint = &self.blueprints[blueprint_index];
        
        // Check if player has all required ingredients
        for (ingredient_name, amount) in &blueprint.ingredients {
            if !player.inventory.has_item(ingredient_name, *amount) {
                return Some(format!("Missing ingredient: {} x {}", ingredient_name, amount));
            }
        }
        
        // Check if player has enough cargo space for the output
        let required_space = blueprint.output.weight * blueprint.output_quantity;
        if player.inventory.remaining_capacity() < required_space {
            return Some("Not enough cargo space for crafted item".to_string());
        }
        
        // Remove ingredients from inventory
        for (ingredient_name, amount) in &blueprint.ingredients {
            player.inventory.remove_item(ingredient_name, *amount);
        }
        
        // Add crafted item to inventory
        player.inventory.add_item(blueprint.output.clone(), blueprint.output_quantity);
        
        Some(format!("Successfully crafted {} x {}", blueprint.output_quantity, blueprint.output.name))
    }
}
