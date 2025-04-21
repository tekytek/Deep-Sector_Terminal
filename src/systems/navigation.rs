use std::time::Duration;
use serde::{Serialize, Deserialize};

use crate::models::player::Player;
use crate::models::universe::StarSystem;
use crate::models::universe::Universe;
use crate::systems::time::TimeSystem;

#[derive(Serialize, Deserialize)]
pub struct NavigationSystem {
    travel_in_progress: bool,
    destination: Option<StarSystem>,
    travel_time_remaining: Duration,
}

#[allow(dead_code)]
impl NavigationSystem {
    pub fn new() -> Self {
        NavigationSystem {
            travel_in_progress: false,
            destination: None,
            travel_time_remaining: Duration::from_secs(0),
        }
    }

    pub fn calculate_distance(&self, from: &StarSystem, to: &StarSystem) -> f32 {
        ((to.x - from.x).powi(2) + (to.y - from.y).powi(2)).sqrt()
    }

    pub fn calculate_travel_time(&self, distance: f32) -> Duration {
        // 1 light-year takes 5 minutes of game time
        let minutes = distance * 5.0;
        Duration::from_secs((minutes * 60.0) as u64)
    }

    pub fn is_in_range(&self, player: &Player, distance: f32) -> bool {
        distance <= player.ship.jump_range as f32
    }

    pub fn can_travel_to(&self, player: &Player, destination: &StarSystem) -> bool {
        // Check if player is already traveling
        if self.travel_in_progress {
            return false;
        }
        
        // Check if player is docked - must undock first
        if player.is_docked {
            return false;
        }
        
        // Check if destination is within jump range
        let distance = self.calculate_distance(&player.current_system, destination);
        if !self.is_in_range(player, distance) {
            return false;
        }
        
        // Check if player has enough fuel
        let fuel_required = self.calculate_fuel_required(distance);
        player.ship.current_fuel >= fuel_required
    }
    
    // Calculate how much fuel is required for a given distance
    pub fn calculate_fuel_required(&self, distance: f32) -> u32 {
        // Base fuel consumption is 1 unit per light year
        let base_consumption = distance.ceil() as u32;
        
        // Ensure a minimum consumption
        if base_consumption == 0 {
            1
        } else {
            base_consumption
        }
    }

    pub fn travel_to(&mut self, player: &mut Player, destination: StarSystem) {
        let distance = self.calculate_distance(&player.current_system, &destination);
        let travel_time = self.calculate_travel_time(distance);
        
        // Consume fuel
        let fuel_required = self.calculate_fuel_required(distance);
        if player.ship.current_fuel >= fuel_required {
            player.ship.current_fuel -= fuel_required;
            
            self.travel_in_progress = true;
            self.destination = Some(destination);
            self.travel_time_remaining = travel_time;
        }
    }

    pub fn update(&mut self, player: &mut Player, _universe: &Universe, _time_system: &TimeSystem, delta_time: Duration) {
        if self.travel_in_progress {
            if self.travel_time_remaining <= delta_time {
                // Travel complete
                if let Some(destination) = self.destination.take() {
                    player.current_system = destination;
                }
                self.travel_in_progress = false;
                self.travel_time_remaining = Duration::from_secs(0);
            } else {
                // Still traveling
                self.travel_time_remaining -= delta_time;
            }
        }
    }

    pub fn can_dock(&self, player: &Player) -> bool {
        !player.is_docked && !player.current_system.stations.is_empty()
    }

    pub fn dock(&mut self, player: &mut Player) {
        player.is_docked = true;
    }

    pub fn undock(&mut self, player: &mut Player) {
        player.is_docked = false;
    }

    pub fn is_docked(&self, player: &Player) -> bool {
        player.is_docked
    }

    pub fn is_traveling(&self) -> bool {
        self.travel_in_progress
    }

    pub fn get_remaining_travel_time(&self) -> Duration {
        self.travel_time_remaining
    }
}
