use serde::{Serialize, Deserialize};
use crate::models::item::ResourceType;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ShipType {
    Scout,
    Freighter,
    Miner,
    Fighter,
}

impl ShipType {
    pub fn to_string(&self) -> String {
        match self {
            ShipType::Scout => "Scout".to_string(),
            ShipType::Freighter => "Freighter".to_string(),
            ShipType::Miner => "Miner".to_string(),
            ShipType::Fighter => "Fighter".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ship {
    pub name: String,
    pub ship_type: ShipType,
    pub hull: u32,
    pub max_hull: u32,
    pub shield: u32,
    pub max_shield: u32,
    pub cargo_capacity: u32,
    pub speed: u32,
    pub jump_range: u32,
    pub weapon_power: u32,
    pub mining_power: u32,
    // Specialized cargo bays for different resource types
    pub mineral_bay_capacity: u32,  // For mineral ores
    pub gas_bay_capacity: u32,      // For gas resources
    pub ice_bay_capacity: u32,      // For ice resources
    pub exotic_bay_capacity: u32,   // For exotic materials
    // Fuel system
    pub fuel_capacity: u32,        // Maximum fuel capacity
    pub current_fuel: u32,         // Current fuel level
    pub fuel_consumption_rate: f32, // Fuel used per light year of travel
}

#[allow(dead_code)]
impl Ship {
    pub fn new(
        name: &str, 
        ship_type: ShipType,
        cargo_override: Option<u32>,
        speed_override: Option<u32>,
        jump_range_override: Option<u32>,
        weapon_power_override: Option<u32>,
        mining_power_override: Option<u32>
    ) -> Self {
        let (hull, cargo, speed, jump_range, weapon_power, mining_power) = match ship_type {
            ShipType::Scout => (100, 20, 100, 10, 5, 2),
            ShipType::Freighter => (200, 100, 60, 6, 2, 1),
            ShipType::Miner => (150, 50, 70, 7, 3, 10),
            ShipType::Fighter => (120, 15, 90, 8, 10, 1),
        };

        // Initialize specialized cargo bays based on ship type
        let (mineral_bay, gas_bay, ice_bay, exotic_bay) = match ship_type {
            ShipType::Miner => (30, 10, 10, 5),  // Mining ships get large specialized bays
            ShipType::Freighter => (10, 10, 10, 5), // Freighters get balanced bay sizes
            _ => (5, 5, 5, 2),  // Other ships get minimal specialized storage
        };
        
        // Calculate fuel capacity and consumption based on ship type
        let (fuel_capacity, fuel_consumption_rate) = match ship_type {
            ShipType::Scout => (800, 0.8),      // Efficient, long range
            ShipType::Freighter => (1500, 1.5), // Large capacity, higher consumption
            ShipType::Miner => (1000, 1.2),     // Balanced for local operations
            ShipType::Fighter => (600, 1.0),    // Balanced consumption
        };

        Ship {
            name: name.to_string(),
            ship_type,
            hull,
            max_hull: hull,
            shield: hull / 2,
            max_shield: hull / 2,
            cargo_capacity: cargo_override.unwrap_or(cargo),
            speed: speed_override.unwrap_or(speed),
            jump_range: jump_range_override.unwrap_or(jump_range),
            weapon_power: weapon_power_override.unwrap_or(weapon_power),
            mining_power: mining_power_override.unwrap_or(mining_power),
            // Set specialized cargo bay capacities
            mineral_bay_capacity: mineral_bay,
            gas_bay_capacity: gas_bay,
            ice_bay_capacity: ice_bay,
            exotic_bay_capacity: exotic_bay,
            // Initialize fuel system
            fuel_capacity,
            current_fuel: fuel_capacity, // Start with a full tank
            fuel_consumption_rate,
        }
    }
    
    // Legacy constructor for backward compatibility
    pub fn default(name: &str, ship_type: ShipType) -> Self {
        Self::new(name, ship_type, None, None, None, None, None)
    }

    pub fn repair(&mut self, amount: u32) {
        self.hull = (self.hull + amount).min(self.max_hull);
    }

    pub fn recharge_shield(&mut self, amount: u32) {
        self.shield = (self.shield + amount).min(self.max_shield);
    }

    pub fn take_damage(&mut self, amount: u32) -> bool {
        // First absorb damage with shields
        if self.shield >= amount {
            self.shield -= amount;
            return false;
        }
        
        let remaining_damage = amount - self.shield;
        self.shield = 0;
        
        // Then apply to hull
        if self.hull <= remaining_damage {
            self.hull = 0;
            return true;  // Ship destroyed
        }
        
        self.hull -= remaining_damage;
        false  // Ship still intact
    }
    
    // Added for client-server functionality
    pub fn get_cargo_space_available(&self) -> u32 {
        // In a full implementation, we would subtract the used cargo space from cargo_capacity
        // For this example, just return the full capacity
        self.cargo_capacity
    }
    
    // Get total specialized cargo capacity
    pub fn get_total_specialized_cargo(&self) -> u32 {
        self.mineral_bay_capacity + self.gas_bay_capacity + 
        self.ice_bay_capacity + self.exotic_bay_capacity
    }
    
    // Check capacity for specific resource type
    pub fn get_capacity_for_resource(&self, resource_type: &ResourceType) -> u32 {
        match resource_type {
            ResourceType::Mineral => self.mineral_bay_capacity,
            ResourceType::Gas => self.gas_bay_capacity,
            ResourceType::Ice => self.ice_bay_capacity,
            ResourceType::Exotic => self.exotic_bay_capacity,
            _ => 0, // Other resources use standard cargo
        }
    }
    
    // Display specialized cargo bay information
    pub fn get_specialized_cargo_info(&self) -> String {
        format!(
            "Mining Bays: Mineral: {}/{} | Gas: {}/{} | Ice: {}/{} | Exotic: {}/{}", 
            0, self.mineral_bay_capacity, // Used/Total
            0, self.gas_bay_capacity,     // These would track actual usage in full implementation
            0, self.ice_bay_capacity,
            0, self.exotic_bay_capacity
        )
    }
    
    // Fuel management methods
    
    // Calculate fuel required for a jump of given distance
    pub fn calculate_fuel_for_distance(&self, distance: f32) -> u32 {
        (distance * self.fuel_consumption_rate).ceil() as u32
    }
    
    // Check if ship has enough fuel for a jump
    pub fn has_fuel_for_jump(&self, distance: f32) -> bool {
        let required_fuel = self.calculate_fuel_for_distance(distance);
        self.current_fuel >= required_fuel
    }
    
    // Consume fuel for a jump and return success/failure
    pub fn consume_fuel_for_jump(&mut self, distance: f32) -> bool {
        let required_fuel = self.calculate_fuel_for_distance(distance);
        
        if self.current_fuel < required_fuel {
            return false; // Not enough fuel
        }
        
        self.current_fuel -= required_fuel;
        true
    }
    
    // Refuel the ship by a specified amount
    pub fn refuel(&mut self, amount: u32) -> u32 {
        let before = self.current_fuel;
        self.current_fuel = (self.current_fuel + amount).min(self.fuel_capacity);
        self.current_fuel - before // Return the actual amount added
    }
    
    // Get fuel status as a percentage
    pub fn get_fuel_percentage(&self) -> f32 {
        (self.current_fuel as f32 / self.fuel_capacity as f32) * 100.0
    }
    
    // Get maximum travel range with current fuel
    pub fn get_max_range_with_current_fuel(&self) -> f32 {
        self.current_fuel as f32 / self.fuel_consumption_rate
    }
    
    // Get fuel status display string
    pub fn get_fuel_status(&self) -> String {
        format!(
            "Fuel: {}/{} ({:.1}%) | Range: {:.1} LY", 
            self.current_fuel, 
            self.fuel_capacity,
            self.get_fuel_percentage(),
            self.get_max_range_with_current_fuel()
        )
    }
}
