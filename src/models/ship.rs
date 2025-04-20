use serde::{Serialize, Deserialize};

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
}

impl Ship {
    pub fn new(name: &str, ship_type: ShipType) -> Self {
        let (hull, cargo, speed, jump_range, weapon_power, mining_power) = match ship_type {
            ShipType::Scout => (100, 20, 100, 10, 5, 2),
            ShipType::Freighter => (200, 100, 60, 6, 2, 1),
            ShipType::Miner => (150, 50, 70, 7, 3, 10),
            ShipType::Fighter => (120, 15, 90, 8, 10, 1),
        };

        Ship {
            name: name.to_string(),
            ship_type,
            hull,
            max_hull: hull,
            shield: hull / 2,
            max_shield: hull / 2,
            cargo_capacity: cargo,
            speed,
            jump_range,
            weapon_power,
            mining_power,
        }
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
}
