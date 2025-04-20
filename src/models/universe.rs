use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use noise::{NoiseFn, Perlin};

use crate::models::item::{Item, ItemType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarSystem {
    pub id: String,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub has_station: bool,
    pub resources: Vec<(String, u32)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Universe {
    systems: HashMap<String, StarSystem>,
    market_items: HashMap<String, Vec<(Item, u32)>>,
    rng_seed: u64,
}

impl Universe {
    pub fn new() -> Self {
        let seed = 12345;  // Fixed seed for reproducible universe
        let mut rng = StdRng::seed_from_u64(seed);
        let perlin = Perlin::new(seed as u32);
        
        let mut systems = HashMap::new();
        let mut market_items = HashMap::new();

        // Create Sol system as the starting point
        let sol = StarSystem {
            id: "sol".to_string(),
            name: "Sol".to_string(),
            x: 0.0,
            y: 0.0,
            has_station: true,
            resources: vec![
                ("Iron".to_string(), 70),
                ("Copper".to_string(), 50),
                ("Water".to_string(), 90),
            ],
        };
        
        systems.insert(sol.id.clone(), sol);
        
        // Generate some basic systems
        let system_names = [
            "Alpha Centauri", "Barnard's Star", "Wolf 359", "Lalande 21185", 
            "Sirius", "Luyten 726-8", "Ross 154", "Ross 248", "Epsilon Eridani",
            "Lacaille 9352", "Procyon", "61 Cygni", "Struve 2398", "Groombridge 34",
            "Epsilon Indi", "Tau Ceti", "Gliese 892", "Altair", "Gliese 570"
        ];
        
        for name in system_names.iter() {
            // Generate position using noise to create more realistic clusters
            let angle = rng.gen_range(0.0..std::f64::consts::TAU) as f32;
            let distance = rng.gen_range(4.0..20.0);
            
            let base_x = distance * angle.cos();
            let base_y = distance * angle.sin();
            
            // Add some noise to create more interesting patterns
            let noise_val = perlin.get([base_x as f64 * 0.1, base_y as f64 * 0.1]) as f32 * 2.0;
            
            let x = base_x + noise_val;
            let y = base_y + noise_val;
            
            // Determine if the system has a station (about 60% chance)
            let has_station = rng.gen_bool(0.6);
            
            // Generate available resources for this system
            let mut resources = Vec::new();
            let resource_types = ["Iron", "Copper", "Silver", "Gold", "Titanium", "Water", "Hydrogen", "Oxygen"];
            
            let resource_count = rng.gen_range(1..4);
            for _ in 0..resource_count {
                let resource_idx = rng.gen_range(0..resource_types.len());
                let resource_name = resource_types[resource_idx].to_string();
                
                // Generate abundance based on perlin noise for more realistic resource distribution
                let abundance_noise = (perlin.get([x as f64 * 0.2, y as f64 * 0.2]) * 50.0 + 50.0) as u32;
                resources.push((resource_name, abundance_noise.clamp(10, 100)));
            }
            
            let system = StarSystem {
                id: name.to_lowercase().replace(" ", "_"),
                name: name.to_string(),
                x,
                y,
                has_station,
                resources,
            };
            
            let system_id = system.id.clone();
            systems.insert(system_id.clone(), system);
            
            // Generate market items for systems with stations
            if has_station {
                generate_market_items(&mut market_items, &system_id, &mut rng);
            }
        }
        
        // Also generate market items for Sol
        generate_market_items(&mut market_items, "sol", &mut rng);

        Universe {
            systems,
            market_items,
            rng_seed: seed,
        }
    }

    pub fn get_system(&self, id: &str) -> Option<&StarSystem> {
        self.systems.get(id)
    }

    pub fn get_all_systems(&self) -> Vec<StarSystem> {
        self.systems.values().cloned().collect()
    }

    pub fn get_nearby_systems(&self, current_system: &StarSystem) -> Vec<StarSystem> {
        let mut systems: Vec<StarSystem> = self.systems.values()
            .filter(|s| s.id != current_system.id)
            .cloned()
            .collect();
        
        // Sort by distance from current system
        systems.sort_by(|a, b| {
            let dist_a = distance(current_system.x, current_system.y, a.x, a.y);
            let dist_b = distance(current_system.x, current_system.y, b.x, b.y);
            dist_a.partial_cmp(&dist_b).unwrap()
        });
        
        // Return the nearest systems (max 9 for UI simplicity)
        systems.into_iter().take(9).collect()
    }

    pub fn get_nearby_system(&self, index: usize) -> Option<StarSystem> {
        let sol = self.get_system("sol").unwrap();
        let nearby = self.get_nearby_systems(sol);
        
        if index < nearby.len() {
            Some(nearby[index].clone())
        } else {
            None
        }
    }

    pub fn get_market_items_for_system(&self, system_id: String) -> Vec<(Item, u32)> {
        self.market_items.get(&system_id)
            .cloned()
            .unwrap_or_default()
    }

    pub fn update_market_prices(&mut self) {
        // Update market prices for all systems with some fluctuation
        let mut rng = StdRng::seed_from_u64(self.rng_seed);
        
        for items in self.market_items.values_mut() {
            for (item, _) in items {
                // Fluctuate price by up to 10% in either direction
                let fluctuation = rng.gen_range(0.9..1.1);
                let base_value = match item.item_type {
                    ItemType::Resource => 100,
                    ItemType::Component => 300,
                    ItemType::Product => 800,
                };
                
                item.value = (base_value as f32 * fluctuation) as u32;
            }
        }
    }
}

fn distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

fn generate_market_items(
    market_items: &mut HashMap<String, Vec<(Item, u32)>>,
    system_id: &str, 
    rng: &mut StdRng
) {
    let mut items = Vec::new();
    
    // Resources
    let resources = [
        "Iron", "Copper", "Silver", "Gold", "Titanium", "Water", "Hydrogen", "Oxygen"
    ];
    
    // Components
    let components = [
        "Power Cell", "Computer Chip", "Fusion Core", "Shield Generator", "Warp Drive"
    ];
    
    // Products
    let products = [
        "Medical Supplies", "Luxury Goods", "Military Equipment", "Ship Parts"
    ];
    
    // Add 3-5 random resources
    let resource_count = rng.gen_range(3..6);
    for _ in 0..resource_count {
        let resource_idx = rng.gen_range(0..resources.len());
        let name = resources[resource_idx].to_string();
        
        let base_value = 50 + rng.gen_range(0..100);
        let quantity = rng.gen_range(10..100);
        
        let item = Item {
            name,
            value: base_value,
            weight: 1,
            item_type: ItemType::Resource,
        };
        
        items.push((item, quantity));
    }
    
    // Add 2-3 random components
    let component_count = rng.gen_range(2..4);
    for _ in 0..component_count {
        let component_idx = rng.gen_range(0..components.len());
        let name = components[component_idx].to_string();
        
        let base_value = 200 + rng.gen_range(0..200);
        let quantity = rng.gen_range(5..20);
        
        let item = Item {
            name,
            value: base_value,
            weight: 2,
            item_type: ItemType::Component,
        };
        
        items.push((item, quantity));
    }
    
    // Add 1-2 random products
    let product_count = rng.gen_range(1..3);
    for _ in 0..product_count {
        let product_idx = rng.gen_range(0..products.len());
        let name = products[product_idx].to_string();
        
        let base_value = 500 + rng.gen_range(0..500);
        let quantity = rng.gen_range(2..10);
        
        let item = Item {
            name,
            value: base_value,
            weight: 4,
            item_type: ItemType::Product,
        };
        
        items.push((item, quantity));
    }
    
    market_items.insert(system_id.to_string(), items);
}
