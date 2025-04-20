use std::error::Error;
use std::time::{Duration, Instant};
use crossterm::event::{KeyEvent, KeyCode};
use serde::{Serialize, Deserialize};
use crate::utils::serde::{instant_serde, option_instant_serde};

use crate::models::{
    player::Player,
    universe::Universe,
};
use crate::systems::{
    navigation::NavigationSystem,
    trading::TradingSystem,
    mining::MiningSystem,
    crafting::CraftingSystem,
    time::TimeSystem,
};
use crate::utils::save_load::{save_game, load_game};



#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum GameScreen {
    MainMenu,
    Navigation,
    Market,
    Ship,
    Mining,
    Crafting,
    Inventory,
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
                    current_screen: GameScreen::MainMenu,
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
        self.trading_system.update(&mut self.universe, delta_time);
        
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
            GameScreen::Navigation => self.handle_navigation_input(key),
            GameScreen::Market => self.handle_market_input(key),
            GameScreen::Ship => self.handle_ship_input(key),
            GameScreen::Mining => self.handle_mining_input(key),
            GameScreen::Crafting => self.handle_crafting_input(key),
            GameScreen::Inventory => self.handle_inventory_input(key),
            GameScreen::Help => self.handle_help_input(key),
            GameScreen::Quit => self.handle_quit_input(key),
        }
    }

    fn handle_main_menu_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('n') => self.change_screen(GameScreen::Navigation),
            KeyCode::Char('m') => self.change_screen(GameScreen::Market),
            KeyCode::Char('s') => self.change_screen(GameScreen::Ship),
            KeyCode::Char('r') => self.change_screen(GameScreen::Mining),
            KeyCode::Char('c') => self.change_screen(GameScreen::Crafting),
            KeyCode::Char('i') => self.change_screen(GameScreen::Inventory),
            KeyCode::Char('h') => self.change_screen(GameScreen::Help),
            KeyCode::Char('q') => self.change_screen(GameScreen::Quit),
            _ => {}
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
                        self.show_message(format!("Traveling to {}", destination.name));
                    } else {
                        self.show_message("Cannot travel to that system - too far away".to_string());
                    }
                }
            },
            KeyCode::Char('d') => {
                // Dock at the current system's station if there is one
                if self.navigation_system.can_dock(&self.player) {
                    self.navigation_system.dock(&mut self.player);
                    self.show_message("Docked at station".to_string());
                } else {
                    self.show_message("No station to dock at in this system".to_string());
                }
            },
            KeyCode::Char('u') => {
                // Undock from current station
                if self.navigation_system.is_docked(&self.player) {
                    self.navigation_system.undock(&mut self.player);
                    self.show_message("Undocked from station".to_string());
                } else {
                    self.show_message("Not currently docked".to_string());
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
                        self.show_message(result);
                    }
                } else {
                    if let Some(result) = self.trading_system.sell_item(&mut self.player, num - 1) {
                        self.show_message(result);
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
                    self.show_message(result);
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
                
                if let Some(result) = self.crafting_system.craft_item(&mut self.player, blueprint_idx) {
                    self.show_message(result);
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

    fn handle_help_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('m') => self.change_screen(GameScreen::MainMenu),
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

    fn show_message(&mut self, message: String) {
        self.message = Some(message);
        self.message_time = Some(Instant::now());
    }
}
