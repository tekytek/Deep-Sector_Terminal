use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::models::ship::{Ship, ShipType};
use crate::models::universe::StarSystem;
use crate::models::item::Inventory;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub credits: u32,
    pub ship: Ship,
    pub inventory: Inventory,
    pub current_system: StarSystem,
    pub is_docked: bool,
}

impl Player {
    pub fn new(name: &str) -> Self {
        let id = Uuid::new_v4().to_string();
        let ship = Ship::new("Rustbucket", ShipType::Scout);
        
        // Start with a default inventory and some starting credits
        let inventory = Inventory::new(ship.cargo_capacity);
        let credits = 5000;

        // Create a default starting system
        let current_system = StarSystem {
            id: "sol".to_string(),
            name: "Sol".to_string(),
            x: 0.0,
            y: 0.0,
            has_station: true,
            resources: vec![],
        };

        Self {
            id,
            name: name.to_string(),
            credits,
            ship,
            inventory,
            current_system,
            is_docked: true,
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
