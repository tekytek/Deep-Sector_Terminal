use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Mineral,     // From asteroid fields
    Gas,         // From gas fields
    Ice,         // From ice fields
    Lunar,       // From moon residues
    Stellar,     // From star coronas
    Exotic,      // From black hole accretion disks
    Refined,     // Processed resources
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ItemType {
    Resource(ResourceType), // Various types of raw materials
    Component,   // Processed materials and parts
    Product,     // Final products
    Blueprint,   // Item blueprints for crafting
    Equipment,   // Ships parts and components that can be installed
    ShipModule,  // Full ship modules/upgrades
    Fuel,        // Ship fuel for travel
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Item {
    pub name: String,
    pub value: u32,      // Base value in credits
    pub weight: u32,     // Weight in cargo units
    pub item_type: ItemType,
}

#[allow(dead_code)]
impl Item {
    pub fn new(name: &str, value: u32, weight: u32, item_type: ItemType) -> Self {
        Item {
            name: name.to_string(),
            value,
            weight,
            item_type,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub items: HashMap<Item, u32>,
    pub capacity: u32,
}

impl Inventory {
    pub fn new(capacity: u32) -> Self {
        Inventory {
            items: HashMap::new(),
            capacity,
        }
    }

    pub fn add_item(&mut self, item: Item, quantity: u32) -> bool {
        let total_weight = item.weight * quantity;
        
        if self.used_capacity() + total_weight > self.capacity {
            return false;  // Not enough space
        }
        
        let entry = self.items.entry(item).or_insert(0);
        *entry += quantity;
        
        true
    }

    pub fn remove_item(&mut self, item_name: &str, quantity: u32) -> Option<Item> {
        let item = self.items.iter()
            .find(|(i, _)| i.name == item_name)
            .map(|(i, _)| i.clone());
        
        if let Some(item) = item {
            if let Some(current_quantity) = self.items.get_mut(&item) {
                if *current_quantity < quantity {
                    return None;  // Not enough items
                }
                
                *current_quantity -= quantity;
                
                if *current_quantity == 0 {
                    self.items.remove(&item);
                }
                
                return Some(item);
            }
        }
        
        None
    }

    pub fn has_item(&self, item_name: &str, quantity: u32) -> bool {
        self.items.iter()
            .any(|(item, qty)| item.name == item_name && *qty >= quantity)
    }

    pub fn get_item_quantity(&self, item_name: &str) -> u32 {
        self.items.iter()
            .find(|(item, _)| item.name == item_name)
            .map(|(_, qty)| *qty)
            .unwrap_or(0)
    }

    pub fn used_capacity(&self) -> u32 {
        self.items.iter()
            .map(|(item, quantity)| item.weight * quantity)
            .sum()
    }

    pub fn remaining_capacity(&self) -> u32 {
        self.capacity - self.used_capacity()
    }
}
