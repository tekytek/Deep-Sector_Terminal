use std::error::Error;
use std::time::{Duration, Instant};
use crossterm::event::{KeyEvent, KeyCode};
use serde::{Serialize, Deserialize};
use crate::utils::serde::{instant_serde, option_instant_serde};

use crate::models::{
    player::Player,
    universe::Universe,
    faction::{FactionType, Storyline},
    market::OrderType,
};
use crate::systems::{
    navigation::NavigationSystem,
    trading::TradingSystem,
    mining::MiningSystem,
    crafting::CraftingSystem,
    time::TimeSystem,
};
use crate::utils::save_load::{save_game, load_game};



#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum GameScreen {
    MainMenu,
    CharacterCreation,
    Navigation,
    Market,
    Ship,
    Mining,
    Crafting,
    Inventory,
    Character, // New screen for character and skills information
    Orders,    // Trade order management screen
    StationServices, // Screen for station services including refueling
    Help,
    Quit,
}

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub player: Player,
    pub universe: Universe,
    pub current_screen: GameScreen,
    pub previous_screen: GameScreen,
    pub navigation_system: NavigationSystem,
    pub trading_system: TradingSystem,
    pub mining_system: MiningSystem,
    pub crafting_system: CraftingSystem,
    pub time_system: TimeSystem,
    #[serde(with = "instant_serde")]
    pub last_update: Instant,
    pub game_over: bool,
    pub quit_confirmed: bool,
    pub message: Option<String>,
    #[serde(with = "option_instant_serde", skip_serializing_if = "Option::is_none")]
    pub message_time: Option<Instant>,
    // Animation related fields
    #[serde(default)]
    pub animation_frame: u64,
    #[serde(default)]
    pub show_animation_effects: bool,
    
    // Character creation related fields
    #[serde(default)]
    pub character_name: String,
    #[serde(default)]
    pub selected_faction: usize,
    #[serde(default)]
    pub selected_storyline: usize,
    #[serde(default)]
    pub creation_stage: u8, // 0=name, 1=faction, 2=storyline, 3=confirm
    
    // Character info screen related fields
    #[serde(default)]
    pub character_info_tab: usize, // 0=skills, 1=reputation, 2=assets, 3=background
    
    // Orders screen related fields
    #[serde(default)]
    pub orders_view_active: bool, // true=viewing active orders, false=viewing completed orders
}

impl Game {
    pub fn new() -> Self {
        // Try to load saved game, or create a new one
        match load_game() {
            Ok(game) => game,
            Err(_) => {
                let player = Player::new("Commander");
                let universe = Universe::new();
                
                Self {
                    player,
                    universe,
                    current_screen: GameScreen::CharacterCreation, // Start with character creation
                    previous_screen: GameScreen::MainMenu,
                    navigation_system: NavigationSystem::new(),
                    trading_system: TradingSystem::new(),
                    mining_system: MiningSystem::new(),
                    crafting_system: CraftingSystem::new(),
                    time_system: TimeSystem::new(),
                    last_update: Instant::now(),
                    game_over: false,
                    quit_confirmed: false,
                    message: None,
                    message_time: None,
                    animation_frame: 0,
                    show_animation_effects: true,
                    // Initialize character creation fields
                    character_name: String::new(),
                    selected_faction: 0,
                    selected_storyline: 0,
                    creation_stage: 0,
                    character_info_tab: 0,
                    orders_view_active: true,
                }
            }
        }
    }

    pub fn update(&mut self) -> Result<(), Box<dyn Error>> {
        let now = Instant::now();
        let delta_time = now.duration_since(self.last_update);
        self.last_update = now;

        // Update game systems
        self.time_system.update(delta_time);
        self.navigation_system.update(&mut self.player, &self.universe, &self.time_system, delta_time);
        
        // Update trading system and check for executed orders
        let executed_orders = self.trading_system.update(&mut self.universe, delta_time);
        
        // Show notification if there are executed orders
        if !executed_orders.is_empty() {
            // Build a notification message for the player
            let mut message = String::from("Order(s) executed: ");
            for (i, order) in executed_orders.iter().enumerate() {
                if i > 0 {
                    message.push_str(", ");
                }
                
                match order.order_type {
                    OrderType::Buy => {
                        message.push_str(&format!("Bought {} {} at {} credits", 
                            order.quantity, order.item_name, order.target_price));
                    },
                    OrderType::Sell => {
                        message.push_str(&format!("Sold {} {} at {} credits", 
                            order.quantity, order.item_name, order.target_price));
                    }
                }
                
                // Don't make message too long
                if i >= 2 && executed_orders.len() > 3 {
                    message.push_str(&format!(" and {} more", executed_orders.len() - 3));
                    break;
                }
            }
            
            // Display the notification
            self.show_message(&message);
        }
        
        // Update animation frame
        if self.show_animation_effects {
            // Update animation frame counter approximately every 100ms
            if delta_time.as_millis() > 100 {
                self.animation_frame += 1;
            }
        }
        
        // Clear temporary messages after 3 seconds
        if let Some(message_time) = self.message_time {
            if now.duration_since(message_time) > Duration::from_secs(3) {
                self.message = None;
                self.message_time = None;
            }
        }

        Ok(())
    }

    pub fn handle_input(&mut self, key: KeyEvent) {
        match self.current_screen {
            GameScreen::MainMenu => self.handle_main_menu_input(key),
            GameScreen::CharacterCreation => self.handle_character_creation_input(key),
            GameScreen::Navigation => self.handle_navigation_input(key),
            GameScreen::Market => self.handle_market_input(key),
            GameScreen::Ship => self.handle_ship_input(key),
            GameScreen::Mining => self.handle_mining_input(key),
            GameScreen::Crafting => self.handle_crafting_input(key),
            GameScreen::Inventory => self.handle_inventory_input(key),
            GameScreen::Character => self.handle_character_input(key),
            GameScreen::Orders => self.handle_orders_input(key),
            GameScreen::StationServices => self.handle_station_services_input(key),
            GameScreen::Help => self.handle_help_input(key),
            GameScreen::Quit => self.handle_quit_input(key),
        }
    }
    
    fn handle_station_services_input(&mut self, key: KeyEvent) {
        match key.code {
            // Refuel ship
            KeyCode::Char('2') => {
                self.refuel_ship();
            },
            KeyCode::Char('m') => self.change_screen(GameScreen::MainMenu),
            _ => {}
        }
    }
    
    // Refuel the player's ship
    fn refuel_ship(&mut self) {
        // Check if player is docked
        if !self.navigation_system.is_docked(&self.player) {
            self.show_message("You must be docked at a station to refuel");
            return;
        }
        
        // Check if ship is already at full fuel
        if self.player.ship.current_fuel >= self.player.ship.fuel_capacity {
            self.show_message("Ship fuel tank is already full");
            return;
        }
        
        // Calculate refuel amount and cost
        let missing_fuel = self.player.ship.fuel_capacity - self.player.ship.current_fuel;
        let fuel_price = 25; // 25 credits per unit
        let total_cost = missing_fuel as u32 * fuel_price;
        
        // Check if player has enough credits
        if self.player.credits < total_cost {
            self.show_message(&format!("Not enough credits. Refueling costs {} credits", total_cost));
            return;
        }
        
        // Deduct credits and refill fuel
        self.player.remove_credits(total_cost);
        self.player.ship.current_fuel = self.player.ship.fuel_capacity;
        
        self.show_formatted_message(format!(
            "Ship refueled for {} credits. Fuel now at {}/{}", 
            total_cost, 
            self.player.ship.current_fuel, 
            self.player.ship.fuel_capacity
        ));
    }

    fn handle_main_menu_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('n') => self.change_screen(GameScreen::Navigation),
            KeyCode::Char('m') => self.change_screen(GameScreen::Market),
            KeyCode::Char('s') => self.change_screen(GameScreen::Ship),
            KeyCode::Char('r') => self.change_screen(GameScreen::Mining),
            KeyCode::Char('c') => self.change_screen(GameScreen::Crafting),
            KeyCode::Char('i') => self.change_screen(GameScreen::Inventory),
            KeyCode::Char('p') => self.change_screen(GameScreen::Character), // 'p' for profile/pilot
            KeyCode::Char('o') => self.change_screen(GameScreen::Orders),    // 'o' for orders
            KeyCode::Char('t') => self.change_screen(GameScreen::StationServices), // 't' for station
            KeyCode::Char('h') => self.change_screen(GameScreen::Help),
            KeyCode::Char('q') => self.change_screen(GameScreen::Quit),
            _ => {}
        }
    }
    
    fn handle_character_creation_input(&mut self, key: KeyEvent) {
        
        match self.creation_stage {
            // Character Name Input
            0 => {
                match key.code {
                    KeyCode::Char(c) => {
                        // Add character to name
                        self.character_name.push(c);
                    },
                    KeyCode::Backspace => {
                        // Remove last character from name
                        if !self.character_name.is_empty() {
                            self.character_name.pop();
                        }
                    },
                    KeyCode::Enter => {
                        // Advance to faction selection if name isn't empty
                        if !self.character_name.is_empty() {
                            self.creation_stage = 1;
                        } else {
                            self.show_message("Please enter a name");
                        }
                    },
                    _ => {}
                }
            },
            // Faction Selection
            1 => {
                match key.code {
                    KeyCode::Char('1') => {
                        self.selected_faction = 0; // Traders
                        self.creation_stage = 2;
                    },
                    KeyCode::Char('2') => {
                        self.selected_faction = 1; // Miners
                        self.creation_stage = 2;
                    },
                    KeyCode::Char('3') => {
                        self.selected_faction = 2; // Military
                        self.creation_stage = 2;
                    },
                    KeyCode::Char('4') => {
                        self.selected_faction = 3; // Scientists
                        self.creation_stage = 2;
                    },
                    KeyCode::Backspace => {
                        // Go back to name input
                        self.creation_stage = 0;
                    },
                    _ => {}
                }
            },
            // Storyline Selection
            2 => {
                match key.code {
                    KeyCode::Char('1') => {
                        self.selected_storyline = 0;
                        self.creation_stage = 3;
                    },
                    KeyCode::Char('2') => {
                        self.selected_storyline = 1;
                        self.creation_stage = 3;
                    },
                    KeyCode::Char('3') => {
                        self.selected_storyline = 2;
                        self.creation_stage = 3;
                    },
                    KeyCode::Backspace => {
                        // Go back to faction selection
                        self.creation_stage = 1;
                    },
                    _ => {}
                }
            },
            // Confirmation
            3 => {
                match key.code {
                    KeyCode::Char('y') => {
                        // Create the character and start the game
                        let faction_type = match self.selected_faction {
                            0 => FactionType::Traders,
                            1 => FactionType::Miners,
                            2 => FactionType::Military,
                            3 => FactionType::Scientists,
                            _ => FactionType::Traders, // Default fallback
                        };
                        
                        let storyline = self.get_selected_storyline(faction_type.clone());
                        
                        // Create new player with selected options
                        self.player = crate::models::player::Player::with_character(
                            &self.character_name,
                            faction_type,
                            storyline
                        );
                        
                        // Proceed to main game
                        self.change_screen(GameScreen::MainMenu);
                    },
                    KeyCode::Char('n') => {
                        // Start over
                        self.character_name.clear();
                        self.selected_faction = 0;
                        self.selected_storyline = 0;
                        self.creation_stage = 0;
                    },
                    KeyCode::Backspace => {
                        // Go back to storyline selection
                        self.creation_stage = 2;
                    },
                    _ => {}
                }
            },
            _ => {
                // Reset to first stage if somehow outside valid stages
                self.creation_stage = 0;
            }
        }
    }
    
    fn get_selected_storyline(&self, faction: FactionType) -> Storyline {
        match faction {
            FactionType::Traders => {
                match self.selected_storyline {
                    0 => Storyline::new(
                        "trader_commerce",
                        FactionType::Traders,
                        "Commerce Pioneer",
                        "Establish trade routes throughout the galaxy and become a wealthy merchant.",
                        5
                    ),
                    1 => Storyline::new(
                        "trader_smuggler",
                        FactionType::Traders,
                        "Smuggler's Run",
                        "Master the art of moving goods through dangerous territories for higher profits.",
                        5
                    ),
                    2 => Storyline::new(
                        "trader_entrepreneur",
                        FactionType::Traders,
                        "Galactic Entrepreneur",
                        "Build your own trading empire by investing in stations and infrastructure.",
                        5
                    ),
                    _ => Storyline::new(
                        "trader_default",
                        FactionType::Traders,
                        "Getting Started",
                        "Learn the basics of trading and navigation.",
                        5
                    ),
                }
            },
            FactionType::Miners => {
                match self.selected_storyline {
                    0 => Storyline::new(
                        "miner_prospector",
                        FactionType::Miners,
                        "Elite Prospector",
                        "Discover and claim the richest mining locations in the galaxy.",
                        5
                    ),
                    1 => Storyline::new(
                        "miner_refiner",
                        FactionType::Miners,
                        "Master Refiner",
                        "Specialize in processing raw materials into high-value refined goods.",
                        5
                    ),
                    2 => Storyline::new(
                        "miner_asteroids",
                        FactionType::Miners,
                        "Asteroid Baron",
                        "Control the asteroid belts and establish mining operations throughout the system.",
                        5
                    ),
                    _ => Storyline::new(
                        "miner_default",
                        FactionType::Miners,
                        "Getting Started",
                        "Learn the basics of mining and resource gathering.",
                        5
                    ),
                }
            },
            FactionType::Military => {
                match self.selected_storyline {
                    0 => Storyline::new(
                        "military_defense",
                        FactionType::Military,
                        "System Defense",
                        "Protect civilian shipping lanes from pirates and other threats.",
                        5
                    ),
                    1 => Storyline::new(
                        "military_special_ops",
                        FactionType::Military,
                        "Special Operations",
                        "Undertake covert missions in rival territories and hostile zones.",
                        5
                    ),
                    2 => Storyline::new(
                        "military_fleet",
                        FactionType::Military,
                        "Fleet Commander",
                        "Lead a squadron of ships to maintain galactic peace and order.",
                        5
                    ),
                    _ => Storyline::new(
                        "military_default",
                        FactionType::Military,
                        "Getting Started",
                        "Learn the basics of combat and fleet operations.",
                        5
                    ),
                }
            },
            FactionType::Scientists => {
                match self.selected_storyline {
                    0 => Storyline::new(
                        "scientist_researcher",
                        FactionType::Scientists,
                        "Research Pioneer",
                        "Discover new technologies by studying cosmic phenomena and artifacts.",
                        5
                    ),
                    1 => Storyline::new(
                        "scientist_explorer",
                        FactionType::Scientists,
                        "Galactic Explorer",
                        "Chart unexplored regions of space and document new discoveries.",
                        5
                    ),
                    2 => Storyline::new(
                        "scientist_biotech",
                        FactionType::Scientists,
                        "Biotechnology Expert",
                        "Research alien biology and develop enhancements for human survival in space.",
                        5
                    ),
                    _ => Storyline::new(
                        "scientist_default",
                        FactionType::Scientists,
                        "Getting Started",
                        "Learn the basics of research and exploration.",
                        5
                    ),
                }
            },
        }
    }

    fn handle_navigation_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('1') | KeyCode::Char('2') | KeyCode::Char('3') | 
            KeyCode::Char('4') | KeyCode::Char('5') | KeyCode::Char('6') |
            KeyCode::Char('7') | KeyCode::Char('8') | KeyCode::Char('9') => {
                let num = match key.code {
                    KeyCode::Char(c) => c.to_digit(10).unwrap() as usize,
                    _ => 0, // Default fallback, though this match arm should never be reached
                };
                if let Some(destination) = self.universe.get_nearby_system(num - 1) {
                    if self.navigation_system.can_travel_to(&self.player, &destination) {
                        self.navigation_system.travel_to(&mut self.player, destination.clone());
                        self.show_formatted_message(format!("Traveling to {}", destination.name));
                    } else {
                        self.show_message("Cannot travel to that system - too far away");
                    }
                }
            },
            KeyCode::Char('d') => {
                // Dock at the current system's station if there is one
                if self.navigation_system.can_dock(&self.player) {
                    self.navigation_system.dock(&mut self.player);
                    self.show_message("Docked at station");
                } else {
                    self.show_message("No station to dock at in this system");
                }
            },
            KeyCode::Char('u') => {
                // Undock from current station
                if self.navigation_system.is_docked(&self.player) {
                    self.navigation_system.undock(&mut self.player);
                    self.show_message("Undocked from station");
                } else {
                    self.show_message("Not currently docked");
                }
            },
            KeyCode::Char('t') => {
                // Access station services when docked
                if self.navigation_system.is_docked(&self.player) {
                    self.change_screen(GameScreen::StationServices);
                } else {
                    self.show_message("You must be docked at a station to access services");
                }
            },
            KeyCode::Char('m') => self.change_screen(GameScreen::MainMenu),
            _ => {}
        }
    }

    fn handle_market_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('b') => {
                // Buy mode
                self.trading_system.set_buy_mode(true);
            },
            KeyCode::Char('s') => {
                // Sell mode
                self.trading_system.set_buy_mode(false);
            },
            KeyCode::Char('1') | KeyCode::Char('2') | KeyCode::Char('3') | 
            KeyCode::Char('4') | KeyCode::Char('5') | KeyCode::Char('6') |
            KeyCode::Char('7') | KeyCode::Char('8') | KeyCode::Char('9') => {
                let num = match key.code {
                    KeyCode::Char(c) => c.to_digit(10).unwrap() as usize,
                    _ => 0, // Default fallback, though this match arm should never be reached
                };
                
                // Buy or sell the selected item
                if self.trading_system.is_buy_mode() {
                    if let Some(result) = self.trading_system.buy_item(&mut self.player, num - 1) {
                        self.show_formatted_message(result);
                    }
                } else {
                    if let Some(result) = self.trading_system.sell_item(&mut self.player, num - 1) {
                        self.show_formatted_message(result);
                    }
                }
            },
            KeyCode::Char('m') => self.change_screen(GameScreen::MainMenu),
            _ => {}
        }
    }

    fn handle_ship_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('m') => self.change_screen(GameScreen::MainMenu),
            _ => {}
        }
    }

    fn handle_mining_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('1') | KeyCode::Char('2') | KeyCode::Char('3') => {
                let resource_idx = match key.code {
                    KeyCode::Char(c) => c.to_digit(10).unwrap() as usize - 1,
                    _ => 0, // Default fallback, though this match arm should never be reached
                };
                
                if let Some(result) = self.mining_system.mine_resource(&mut self.player, resource_idx) {
                    self.show_formatted_message(result);
                }
            },
            KeyCode::Char('m') => self.change_screen(GameScreen::MainMenu),
            _ => {}
        }
    }

    fn handle_crafting_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('1') | KeyCode::Char('2') | KeyCode::Char('3') | 
            KeyCode::Char('4') | KeyCode::Char('5') => {
                let blueprint_idx = match key.code {
                    KeyCode::Char(c) => c.to_digit(10).unwrap() as usize - 1,
                    _ => 0, // Default fallback, though this match arm should never be reached
                };
                
                match self.crafting_system.craft_item(&mut self.player, blueprint_idx) {
                    Ok(result) => self.show_formatted_message(format!("Started crafting job: {}", result)),
                    Err(error) => self.show_message(&error),
                }
            },
            KeyCode::Char('m') => self.change_screen(GameScreen::MainMenu),
            _ => {}
        }
    }

    fn handle_inventory_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('m') => self.change_screen(GameScreen::MainMenu),
            _ => {}
        }
    }
    
    fn handle_character_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('m') | KeyCode::Esc => self.change_screen(GameScreen::MainMenu),
            // Set the active tab based on numeric input
            KeyCode::Char('1') => {
                self.character_info_tab = 0;
                self.show_message("Viewing skills information");
            },
            KeyCode::Char('2') => {
                self.character_info_tab = 1;
                self.show_message("Viewing reputation information");
            },
            KeyCode::Char('3') => {
                self.character_info_tab = 2;
                self.show_message("Viewing assets information");
            },
            KeyCode::Char('4') => {
                self.character_info_tab = 3;
                self.show_message("Viewing background information");
            },
            _ => {}
        }
    }

    fn handle_help_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('m') => self.change_screen(GameScreen::MainMenu),
            _ => {}
        }
    }
    
    // Handle input for the Orders screen
    fn handle_orders_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('m') | KeyCode::Esc => self.change_screen(GameScreen::MainMenu),
            KeyCode::Char('b') => {
                // Create buy order
                if self.orders_view_active && self.player.is_docked {
                    // In a real implementation, this would open a form or dialog
                    // For now, we'll create a sample buy order
                    let item_name = "Iron"; // Would come from user input in full implementation
                    let quantity = 10;
                    let target_price = 50;
                    let notes = "Test buy order";
                    
                    match self.trading_system.create_buy_order(
                        &mut self.player, 
                        item_name, 
                        quantity, 
                        target_price, 
                        notes
                    ) {
                        Ok(_) => self.show_formatted_message(format!("Created buy order for {} {}", quantity, item_name)),
                        Err(e) => self.show_formatted_message(format!("Error creating buy order: {}", e)),
                    }
                } else if !self.player.is_docked {
                    self.show_message("You must be docked at a station to create orders");
                } else {
                    self.show_message("You can only create orders in the active orders view");
                }
            },
            KeyCode::Char('s') => {
                // Create sell order
                if self.orders_view_active && self.player.is_docked {
                    // In a real implementation, this would open a form or dialog
                    // For now, we'll create a sample sell order for an item the player might have
                    
                    // Check if player has any items to sell
                    // First get an item if available
                    let item_to_sell = self.player.inventory.items.iter().next().map(|(item, _qty)| {
                        (item.name.clone(), item.value)
                    });
                    
                    if let Some((item_name, item_value)) = item_to_sell {
                        let quantity = 1; // Keep it simple for testing
                        let target_price = item_value + 10; // Sell for profit
                        let notes = "Test sell order";
                        
                        match self.trading_system.create_sell_order(
                            &mut self.player, 
                            &item_name, 
                            quantity, 
                            target_price, 
                            notes
                        ) {
                            Ok(_) => self.show_formatted_message(format!("Created sell order for {} {}", quantity, item_name)),
                            Err(e) => self.show_formatted_message(format!("Error creating sell order: {}", e)),
                        }
                    } else {
                        self.show_message("You have no items to sell");
                    }
                } else if !self.player.is_docked {
                    self.show_message("You must be docked at a station to create orders");
                } else {
                    self.show_message("You can only create orders in the active orders view");
                }
            },
            KeyCode::Char('c') => {
                // Cancel selected order
                if self.orders_view_active && self.player.is_docked {
                    match self.trading_system.cancel_selected_order(&mut self.player) {
                        Ok(_) => self.show_message("Order cancelled successfully"),
                        Err(e) => self.show_formatted_message(format!("Error cancelling order: {}", e)),
                    }
                } else if !self.player.is_docked {
                    self.show_message("You must be docked at a station to cancel orders");
                } else {
                    self.show_message("You can only cancel orders in the active orders view");
                }
            },
            KeyCode::Up => {
                // Navigate up through orders
                self.trading_system.select_previous_order();
            },
            KeyCode::Down => {
                // Navigate down through orders
                self.trading_system.select_next_order();
            },
            KeyCode::Tab => {
                // Toggle between active and completed orders
                self.orders_view_active = !self.orders_view_active;
                // Clear selection when changing views
                self.trading_system.deselect_order();
                let status = if self.orders_view_active { "active" } else { "completed" };
                self.show_formatted_message(format!("Viewing {} orders", status));
            },
            _ => {}
        }
    }

    fn handle_quit_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('y') => self.quit_confirmed = true,
            KeyCode::Char('n') => {
                self.quit_confirmed = false;
                self.change_screen(self.previous_screen.clone());
            },
            _ => {}
        }
    }

    pub fn confirm_quit(&mut self) -> bool {
        if self.current_screen == GameScreen::Quit {
            return self.quit_confirmed;
        }
        
        self.change_screen(GameScreen::Quit);
        false
    }

    pub fn cancel_action(&mut self) {
        if self.current_screen != GameScreen::MainMenu {
            self.change_screen(GameScreen::MainMenu);
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over || self.quit_confirmed
    }

    pub fn save_state(&self) -> Result<(), Box<dyn Error>> {
        save_game(self)
    }

    fn change_screen(&mut self, screen: GameScreen) {
        self.previous_screen = self.current_screen.clone();
        self.current_screen = screen;
    }

    fn show_message(&mut self, message: &str) {
        self.message = Some(message.to_string());
        self.message_time = Some(Instant::now());
    }
    
    // Helper method for formatted messages
    fn show_formatted_message(&mut self, message: String) {
        self.message = Some(message);
        self.message_time = Some(Instant::now());
    }
}
