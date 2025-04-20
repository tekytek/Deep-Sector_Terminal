use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use noise::{NoiseFn, Perlin};

use crate::models::item::{Item, ItemType, ResourceType};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResourceFieldType {
    AsteroidField,
    IceField,
    GasField,
    MoonResidue,
    StarCorona,
    BlackHoleAccretion,
}

impl ResourceFieldType {
    pub fn to_string(&self) -> String {
        match self {
            ResourceFieldType::AsteroidField => "Asteroid Field".to_string(),
            ResourceFieldType::IceField => "Ice Field".to_string(),
            ResourceFieldType::GasField => "Gas Field".to_string(),
            ResourceFieldType::MoonResidue => "Moon Residue".to_string(),
            ResourceFieldType::StarCorona => "Star Corona".to_string(),
            ResourceFieldType::BlackHoleAccretion => "Black Hole Accretion Disk".to_string(),
        }
    }
    
    pub fn primary_resources(&self) -> Vec<(String, u32)> {
        match self {
            ResourceFieldType::AsteroidField => vec![
                ("Iron".to_string(), 70),
                ("Nickel".to_string(), 50),
                ("Cobalt".to_string(), 30),
                ("Rare Metals".to_string(), 15),
            ],
            ResourceFieldType::IceField => vec![
                ("Water".to_string(), 80),
                ("Methane".to_string(), 60),
                ("Ammonia".to_string(), 40),
                ("Carbon Dioxide".to_string(), 50),
            ],
            ResourceFieldType::GasField => vec![
                ("Hydrogen".to_string(), 75),
                ("Helium".to_string(), 65),
                ("Nitrogen".to_string(), 55),
                ("Exotic Gas".to_string(), 20),
            ],
            ResourceFieldType::MoonResidue => vec![
                ("Titanium".to_string(), 60),
                ("Regolith".to_string(), 70),
                ("Silicates".to_string(), 55),
                ("Rare Earth Elements".to_string(), 25),
            ],
            ResourceFieldType::StarCorona => vec![
                ("Plasma".to_string(), 80),
                ("Stellar Matter".to_string(), 60),
                ("Fusion Particles".to_string(), 40),
                ("Solar Wind".to_string(), 70),
            ],
            ResourceFieldType::BlackHoleAccretion => vec![
                ("Dark Matter".to_string(), 40),
                ("Exotic Particles".to_string(), 35),
                ("Gravitonium".to_string(), 20),
                ("Cosmic Dust".to_string(), 60),
            ],
        }
    }
    
    pub fn required_mining_level(&self) -> u8 {
        match self {
            ResourceFieldType::AsteroidField => 1,
            ResourceFieldType::IceField => 1,
            ResourceFieldType::GasField => 2,
            ResourceFieldType::MoonResidue => 2,
            ResourceFieldType::StarCorona => 4,
            ResourceFieldType::BlackHoleAccretion => 5,
        }
    }
    
    pub fn danger_level(&self) -> u8 {
        match self {
            ResourceFieldType::AsteroidField => 1,
            ResourceFieldType::IceField => 1,
            ResourceFieldType::GasField => 2,
            ResourceFieldType::MoonResidue => 2,
            ResourceFieldType::StarCorona => 4,
            ResourceFieldType::BlackHoleAccretion => 5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceField {
    pub field_type: ResourceFieldType,
    pub size: u32, // 1-100 scale
    pub resources: Vec<(String, u32)>, // Resource name, abundance
    pub position_x: f32, // Location within the system
    pub position_y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StationType {
    Trading,    // Trading hub
    Military,   // Security and combat operations
    Mining,     // Resource processing and refining
    Research,   // Blueprint research and technology
    Industrial, // Manufacturing and production
    Habitation, // Civilian habitats
}

impl StationType {
    pub fn to_string(&self) -> String {
        match self {
            StationType::Trading => "Trading Hub".to_string(),
            StationType::Military => "Military Outpost".to_string(),
            StationType::Mining => "Mining Facility".to_string(),
            StationType::Research => "Research Station".to_string(),
            StationType::Industrial => "Industrial Complex".to_string(),
            StationType::Habitation => "Habitat Ring".to_string(),
        }
    }
    
    pub fn services(&self) -> Vec<String> {
        match self {
            StationType::Trading => vec![
                "Market".to_string(), 
                "Trade Missions".to_string(),
                "Passenger Transport".to_string(),
            ],
            StationType::Military => vec![
                "Bounty Office".to_string(),
                "Combat Training".to_string(),
                "Weapons Shop".to_string(),
            ],
            StationType::Mining => vec![
                "Ore Refining".to_string(),
                "Mining Equipment".to_string(),
                "Resource Market".to_string(),
            ],
            StationType::Research => vec![
                "Blueprint Research".to_string(),
                "Technology Exchange".to_string(),
                "Science Missions".to_string(),
            ],
            StationType::Industrial => vec![
                "Manufacturing".to_string(),
                "Ship Construction".to_string(),
                "Equipment Crafting".to_string(),
            ],
            StationType::Habitation => vec![
                "Supplies".to_string(),
                "Medical Services".to_string(),
                "Crew Recruitment".to_string(),
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Station {
    pub id: String,
    pub name: String,
    pub station_type: StationType,
    pub position_x: f32,
    pub position_y: f32,
    pub faction: Option<String>,
    pub services: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CelestialBodyType {
    Star,
    Planet,
    Moon,
    Asteroid,
    BlackHole,
    JumpGate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CelestialBody {
    pub id: String,
    pub name: String,
    pub body_type: CelestialBodyType,
    pub position_x: f32,
    pub position_y: f32,
    pub size: f32, // Relative size, 1.0 = Earth-like
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarSystem {
    pub id: String,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub celestial_bodies: Vec<CelestialBody>,
    pub resource_fields: Vec<ResourceField>,
    pub stations: Vec<Station>,
    pub resources: Vec<(String, u32)>, // Legacy field - general resource abundance
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
        let sol = create_sol_system();
        
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
            
            // Create a new star system with procedurally generated content
            let mut celestial_bodies = Vec::new();
            let mut resource_fields = Vec::new();
            let mut stations = Vec::new();
            
            // Add a star
            celestial_bodies.push(CelestialBody {
                id: format!("{}_{}", name.to_lowercase().replace(" ", "_"), "star"),
                name: format!("{} Star", name),
                body_type: CelestialBodyType::Star,
                position_x: 0.0,
                position_y: 0.0,
                size: rng.gen_range(0.8..1.5), // Varying star sizes
            });
            
            // Always add a jump gate
            celestial_bodies.push(CelestialBody {
                id: format!("{}_{}", name.to_lowercase().replace(" ", "_"), "jump_gate"),
                name: format!("{} Jump Gate", name),
                body_type: CelestialBodyType::JumpGate,
                position_x: rng.gen_range(-10.0..10.0),
                position_y: rng.gen_range(-10.0..10.0),
                size: 0.5,
            });
            
            // Add 1-3 planets
            let planet_count = rng.gen_range(1..4);
            for i in 0..planet_count {
                let planet_angle = rng.gen_range(0.0..std::f64::consts::TAU) as f32;
                let planet_distance = rng.gen_range(2.0..8.0);
                
                let planet_x = planet_distance * planet_angle.cos();
                let planet_y = planet_distance * planet_angle.sin();
                
                celestial_bodies.push(CelestialBody {
                    id: format!("{}_planet_{}", name.to_lowercase().replace(" ", "_"), i+1),
                    name: format!("{} Planet {}", name, i+1),
                    body_type: CelestialBodyType::Planet,
                    position_x: planet_x,
                    position_y: planet_y,
                    size: rng.gen_range(0.3..1.2), // Varying planet sizes
                });
                
                // Maybe add a moon to the planet (50% chance)
                if rng.gen_bool(0.5) {
                    let moon_offset_x = rng.gen_range(0.2..0.5);
                    let moon_offset_y = rng.gen_range(0.2..0.5);
                    
                    celestial_bodies.push(CelestialBody {
                        id: format!("{}_moon_{}", name.to_lowercase().replace(" ", "_"), i+1),
                        name: format!("{} Moon {}", name, i+1),
                        body_type: CelestialBodyType::Moon,
                        position_x: planet_x + moon_offset_x,
                        position_y: planet_y + moon_offset_y,
                        size: rng.gen_range(0.1..0.3), // Moons are smaller
                    });
                    
                    // Add moon residue for mining (35% chance)
                    if rng.gen_bool(0.35) {
                        resource_fields.push(ResourceField {
                            field_type: ResourceFieldType::MoonResidue,
                            size: rng.gen_range(30..60),
                            resources: ResourceFieldType::MoonResidue.primary_resources(),
                            position_x: planet_x + moon_offset_x * 1.1,
                            position_y: planet_y + moon_offset_y * 1.1,
                        });
                    }
                }
            }
            
            // Add resource fields
            
            // Asteroid field (75% chance)
            if rng.gen_bool(0.75) {
                resource_fields.push(ResourceField {
                    field_type: ResourceFieldType::AsteroidField,
                    size: rng.gen_range(40..90),
                    resources: ResourceFieldType::AsteroidField.primary_resources(),
                    position_x: rng.gen_range(-8.0..8.0),
                    position_y: rng.gen_range(-8.0..8.0),
                });
            }
            
            // Ice field (50% chance)
            if rng.gen_bool(0.5) {
                resource_fields.push(ResourceField {
                    field_type: ResourceFieldType::IceField,
                    size: rng.gen_range(30..80),
                    resources: ResourceFieldType::IceField.primary_resources(),
                    position_x: rng.gen_range(-10.0..10.0),
                    position_y: rng.gen_range(-10.0..10.0),
                });
            }
            
            // Gas field (40% chance)
            if rng.gen_bool(0.4) {
                resource_fields.push(ResourceField {
                    field_type: ResourceFieldType::GasField,
                    size: rng.gen_range(50..100),
                    resources: ResourceFieldType::GasField.primary_resources(),
                    position_x: rng.gen_range(-6.0..6.0),
                    position_y: rng.gen_range(-6.0..6.0),
                });
            }
            
            // Black hole (5% chance)
            if rng.gen_bool(0.05) {
                celestial_bodies.push(CelestialBody {
                    id: format!("{}_black_hole", name.to_lowercase().replace(" ", "_")),
                    name: format!("{} Black Hole", name),
                    body_type: CelestialBodyType::BlackHole,
                    position_x: rng.gen_range(-4.0..4.0),
                    position_y: rng.gen_range(-4.0..4.0),
                    size: rng.gen_range(0.5..1.0),
                });
                
                // Black holes have accretion disks for high-level mining
                resource_fields.push(ResourceField {
                    field_type: ResourceFieldType::BlackHoleAccretion,
                    size: rng.gen_range(70..100),
                    resources: ResourceFieldType::BlackHoleAccretion.primary_resources(),
                    position_x: rng.gen_range(-4.0..4.0),
                    position_y: rng.gen_range(-4.0..4.0),
                });
            }
            
            // Add stations if has_station is true
            if has_station {
                // Determine station types based on a weighted system
                let station_types = [
                    (StationType::Trading, 0.3),
                    (StationType::Mining, 0.2),
                    (StationType::Military, 0.15),
                    (StationType::Research, 0.1),
                    (StationType::Industrial, 0.15),
                    (StationType::Habitation, 0.1),
                ];
                
                // Add 1-2 stations
                let station_count = rng.gen_range(1..3);
                let mut station_types_used = Vec::new();
                
                for i in 0..station_count {
                    // Choose station type based on weights
                    let mut total_weight = 0.0;
                    for (_, weight) in &station_types {
                        total_weight += weight;
                    }
                    
                    let mut random_val = rng.gen_range(0.0..total_weight);
                    let mut selected_type = &StationType::Trading; // Default
                    
                    for (station_type, weight) in &station_types {
                        if random_val < *weight {
                            selected_type = station_type;
                            break;
                        }
                        random_val -= weight;
                    }
                    
                    // Check if this type was already used
                    if station_types_used.contains(selected_type) {
                        continue; // Skip duplicate types
                    }
                    
                    station_types_used.push(selected_type.clone());
                    
                    // Add the station
                    stations.push(Station {
                        id: format!("{}_{}_station_{}", name.to_lowercase().replace(" ", "_"), selected_type.to_string().to_lowercase().replace(" ", "_"), i+1),
                        name: format!("{} {}", name, selected_type.to_string()),
                        station_type: selected_type.clone(),
                        position_x: rng.gen_range(-6.0..6.0),
                        position_y: rng.gen_range(-6.0..6.0),
                        faction: None, // No faction assignment yet
                        services: selected_type.services(),
                    });
                }
            }
            
            // Create the system
            let system = StarSystem {
                id: name.to_lowercase().replace(" ", "_"),
                name: name.to_string(),
                x,
                y,
                celestial_bodies,
                resource_fields,
                stations,
                resources, // Keep the old resources for compatibility
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
                let base_value = match &item.item_type {
                    ItemType::Resource(res_type) => {
                        match res_type {
                            ResourceType::Mineral => 100,
                            ResourceType::Gas => 120,
                            ResourceType::Ice => 90,
                            ResourceType::Lunar => 150,
                            ResourceType::Stellar => 200,
                            ResourceType::Exotic => 300,
                            ResourceType::Refined => 180,
                        }
                    },
                    ItemType::Component => 300,
                    ItemType::Product => 800,
                    ItemType::Blueprint => 1200,
                    ItemType::Equipment => 500,
                    ItemType::ShipModule => 1000,
                };
                
                item.value = (base_value as f32 * fluctuation) as u32;
            }
        }
    }
}

fn distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

fn create_sol_system() -> StarSystem {
    // Create celestial bodies for the Sol system
    let mut celestial_bodies = Vec::new();
    
    // The Sun
    celestial_bodies.push(CelestialBody {
        id: "sol_sun".to_string(),
        name: "The Sun".to_string(),
        body_type: CelestialBodyType::Star,
        position_x: 0.0,
        position_y: 0.0,
        size: 109.0, // 109 times Earth's diameter
    });
    
    // Mercury
    celestial_bodies.push(CelestialBody {
        id: "sol_mercury".to_string(),
        name: "Mercury".to_string(),
        body_type: CelestialBodyType::Planet,
        position_x: 0.4,
        position_y: 0.0,
        size: 0.38,
    });
    
    // Venus
    celestial_bodies.push(CelestialBody {
        id: "sol_venus".to_string(),
        name: "Venus".to_string(),
        body_type: CelestialBodyType::Planet,
        position_x: 0.0,
        position_y: 0.7,
        size: 0.95,
    });
    
    // Earth
    celestial_bodies.push(CelestialBody {
        id: "sol_earth".to_string(),
        name: "Earth".to_string(),
        body_type: CelestialBodyType::Planet,
        position_x: 1.0,
        position_y: 0.0,
        size: 1.0,
    });
    
    // Moon
    celestial_bodies.push(CelestialBody {
        id: "sol_moon".to_string(),
        name: "Moon".to_string(),
        body_type: CelestialBodyType::Moon,
        position_x: 1.05,
        position_y: 0.05,
        size: 0.27,
    });
    
    // Mars
    celestial_bodies.push(CelestialBody {
        id: "sol_mars".to_string(),
        name: "Mars".to_string(),
        body_type: CelestialBodyType::Planet,
        position_x: 0.0,
        position_y: 1.5,
        size: 0.53,
    });
    
    // Jupiter
    celestial_bodies.push(CelestialBody {
        id: "sol_jupiter".to_string(),
        name: "Jupiter".to_string(),
        body_type: CelestialBodyType::Planet,
        position_x: -5.2,
        position_y: 0.0,
        size: 11.2,
    });
    
    // Saturn
    celestial_bodies.push(CelestialBody {
        id: "sol_saturn".to_string(),
        name: "Saturn".to_string(),
        body_type: CelestialBodyType::Planet,
        position_x: 0.0,
        position_y: -9.5,
        size: 9.5,
    });
    
    // Jump Gate
    celestial_bodies.push(CelestialBody {
        id: "sol_jump_gate".to_string(),
        name: "Sol Jump Gate".to_string(),
        body_type: CelestialBodyType::JumpGate,
        position_x: 12.0,
        position_y: 12.0,
        size: 0.5,
    });
    
    // Add resource fields
    let mut resource_fields = Vec::new();
    
    // Asteroid belt
    resource_fields.push(ResourceField {
        field_type: ResourceFieldType::AsteroidField,
        size: 80,
        resources: ResourceFieldType::AsteroidField.primary_resources(),
        position_x: 2.8,
        position_y: 0.0,
    });
    
    // Ice field near Saturn
    resource_fields.push(ResourceField {
        field_type: ResourceFieldType::IceField,
        size: 70,
        resources: ResourceFieldType::IceField.primary_resources(),
        position_x: 0.0,
        position_y: -10.5,
    });
    
    // Gas field near Jupiter
    resource_fields.push(ResourceField {
        field_type: ResourceFieldType::GasField,
        size: 90,
        resources: ResourceFieldType::GasField.primary_resources(),
        position_x: -6.2,
        position_y: 0.5,
    });
    
    // Moon residue
    resource_fields.push(ResourceField {
        field_type: ResourceFieldType::MoonResidue,
        size: 40,
        resources: ResourceFieldType::MoonResidue.primary_resources(),
        position_x: 1.2,
        position_y: 0.1,
    });
    
    // Star corona
    resource_fields.push(ResourceField {
        field_type: ResourceFieldType::StarCorona,
        size: 100,
        resources: ResourceFieldType::StarCorona.primary_resources(),
        position_x: 0.0,
        position_y: 0.0,
    });
    
    // Stations
    let mut stations = Vec::new();
    
    // Earth Trading Station
    stations.push(Station {
        id: "sol_earth_station".to_string(),
        name: "Earth Trading Hub".to_string(),
        station_type: StationType::Trading,
        position_x: 1.2,
        position_y: 0.2,
        faction: None,
        services: StationType::Trading.services(),
    });
    
    // Mars Mining Facility
    stations.push(Station {
        id: "sol_mars_station".to_string(),
        name: "Mars Mining Facility".to_string(),
        station_type: StationType::Mining,
        position_x: 0.2,
        position_y: 1.5,
        faction: None,
        services: StationType::Mining.services(),
    });
    
    // Jupiter Research Station
    stations.push(Station {
        id: "sol_jupiter_station".to_string(),
        name: "Jupiter Research Outpost".to_string(),
        station_type: StationType::Research,
        position_x: -5.0,
        position_y: 0.5,
        faction: None,
        services: StationType::Research.services(),
    });
    
    // Saturn Industrial Complex
    stations.push(Station {
        id: "sol_saturn_station".to_string(),
        name: "Saturn Industrial Complex".to_string(),
        station_type: StationType::Industrial,
        position_x: 0.5,
        position_y: -9.0,
        faction: None,
        services: StationType::Industrial.services(),
    });
    
    // General resources (legacy support)
    let resources = vec![
        ("Iron".to_string(), 70),
        ("Copper".to_string(), 50),
        ("Water".to_string(), 90),
        ("Hydrogen".to_string(), 80),
        ("Oxygen".to_string(), 60),
    ];
    
    StarSystem {
        id: "sol".to_string(),
        name: "Sol".to_string(),
        x: 0.0,
        y: 0.0,
        celestial_bodies,
        resource_fields,
        stations,
        resources,
    }
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
        
        // Determine resource type based on name
        let resource_type = if name.contains("Water") || name.contains("Ice") {
            ResourceType::Ice
        } else if name.contains("Hydrogen") || name.contains("Oxygen") || name.contains("Gas") {
            ResourceType::Gas
        } else if name.contains("Titanium") || name.contains("Gold") {
            ResourceType::Mineral
        } else {
            // Default to mineral for other common metals
            ResourceType::Mineral
        };
        
        let item = Item {
            name,
            value: base_value,
            weight: 1,
            item_type: ItemType::Resource(resource_type),
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
