use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;

use crate::models::item::{Item, ItemType};

// Economic factors that affect market prices
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EconomicEvent {
    Shortage(String),      // Item name, shortage increases prices
    Surplus(String),       // Item name, surplus decreases prices
    HighDemand(String),    // Item name, high demand increases prices
    LowDemand(String),     // Item name, low demand decreases prices
    TariffIncrease,        // Global effect, increases all prices
    TariffDecrease,        // Global effect, decreases all prices
    MarketCrash,           // Global effect, significantly decreases all prices
    MarketBoom,            // Global effect, significantly increases all prices
    LocalConflict,         // Local effect, increases military goods, decreases civilian goods
    LocalPeace,            // Local effect, decreases military goods, increases civilian goods
}

// Order types for automated trading
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderType {
    Buy,        // Buy order
    Sell        // Sell order
}

// Status of the order
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderStatus {
    Active,     // Order is active and waiting for conditions to be met
    Completed,  // Order has been completed successfully
    Cancelled,  // Order was cancelled by the player
    Failed,     // Order failed to execute (e.g., insufficient funds, inventory full)
    Expired     // Order expired (time limit reached)
}

// Trade order for automatic buying/selling based on price conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeOrder {
    pub id: Uuid,                  // Unique ID for the order
    pub player_id: String,         // ID of the player who placed the order
    pub system_id: String,         // Target star system for the order
    pub item_name: String,         // Name of the item to buy/sell
    pub order_type: OrderType,     // Buy or sell
    pub quantity: u32,             // Quantity to buy/sell
    pub target_price: u32,         // Target price to trigger the order
    pub price_condition: String,   // "below" for buy orders, "above" for sell orders
    pub status: OrderStatus,       // Current status of the order
    pub created_at: u64,           // When the order was created (timestamp)
    pub expires_at: Option<u64>,   // When the order expires (timestamp, optional)
    pub executed_at: Option<u64>,  // When the order was executed (timestamp, if completed)
    pub notes: String,             // Additional notes/comments for the order
}

// Record of price changes for displaying trends
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceHistory {
    pub timestamp: u64,
    pub price: u32,
}

// Enhanced market item with more economic factors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketItem {
    pub item: Item,
    pub quantity: u32,
    pub base_price: u32,
    pub current_price: u32,
    pub price_volatility: f32,     // How much the price fluctuates (0.0-1.0)
    pub supply_level: f32,         // Current supply level (0.0-2.0, 1.0 is normal)
    pub demand_level: f32,         // Current demand level (0.0-2.0, 1.0 is normal)
    pub price_history: Vec<PriceHistory>, // Track recent price points
    pub production_rate: u32,      // How many units are produced per cycle
    pub consumption_rate: u32,     // How many units are consumed per cycle
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MarketType {
    Trading,        // General trading hub with balanced prices
    Industrial,     // Produces manufactured goods, buys resources
    Mining,         // Produces raw materials, buys manufactured goods
    Agricultural,   // Produces food and organics
    HighTech,       // Specializes in advanced technology
    Black,          // Black market, illegal goods, high volatility
    Military,       // Military outpost, weapons and defense equipment
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    pub system_id: String,
    pub items: HashMap<String, MarketItem>,
    pub last_update: u64,
    pub local_events: Vec<EconomicEvent>,
    pub market_type: MarketType,
    pub tax_rate: f32,             // Local tax rate applied to transactions
    pub trade_orders: Vec<TradeOrder>, // Active trade orders in this market
}

impl Market {
    pub fn new(system_id: &str) -> Self {
        Market {
            system_id: system_id.to_string(),
            items: HashMap::new(),
            last_update: 0,
            local_events: Vec::new(),
            market_type: MarketType::Trading, // Default to trading market
            tax_rate: 0.05, // 5% default tax rate
            trade_orders: Vec::new(), // No orders initially
        }
    }

    pub fn with_market_type(system_id: &str, market_type: MarketType) -> Self {
        let mut market = Self::new(system_id);
        market.market_type = market_type.clone();
        
        // Set appropriate tax rate based on market type
        market.tax_rate = match &market_type {
            MarketType::Trading => 0.05,      // Standard tax
            MarketType::Industrial => 0.08,   // Higher industrial taxes
            MarketType::Mining => 0.07,       // Resource extraction tax
            MarketType::Agricultural => 0.04, // Lower agricultural taxes
            MarketType::HighTech => 0.09,     // Premium tech tax
            MarketType::Black => 0.00,        // No taxes (but higher risk)
            MarketType::Military => 0.03,     // Lower military tax
        };
        
        market
    }

    pub fn add_item(&mut self, item: Item, quantity: u32, base_price: u32, volatility: f32) {
        // Adjust volatility based on market type and item type
        let adjusted_volatility = match self.market_type {
            MarketType::Black => volatility * 2.0,  // Black markets are twice as volatile
            MarketType::Trading => volatility * 0.8, // Trading hubs are more stable
            _ => volatility,
        };
        
        // Set supply and demand based on market type and item type
        let (supply, demand) = match (&self.market_type, &item.item_type) {
            // Industrial markets produce components, need resources
            (MarketType::Industrial, ItemType::Component) => (1.5, 0.8),
            (MarketType::Industrial, ItemType::Resource(_)) => (0.7, 1.3),
            
            // Mining markets produce resources, need components
            (MarketType::Mining, ItemType::Resource(_)) => (1.8, 0.6),
            (MarketType::Mining, ItemType::Component) => (0.6, 1.4),
            
            // High-tech markets favor advanced items
            (MarketType::HighTech, ItemType::ShipModule) => (1.2, 0.9),
            (MarketType::HighTech, ItemType::Equipment) => (1.3, 0.8),
            
            // Military markets favor weapons and defense
            (MarketType::Military, ItemType::Equipment) => (1.2, 0.9),
            
            // Default balanced supply/demand
            _ => (1.0, 1.0),
        };
        
        // Calculate production and consumption rates
        let production = if supply > 1.0 {
            (quantity as f32 * 0.05 * supply) as u32
        } else {
            (quantity as f32 * 0.01) as u32
        };
        
        let consumption = if demand > 1.0 {
            (quantity as f32 * 0.05 * demand) as u32
        } else {
            (quantity as f32 * 0.01) as u32
        };
        
        // Calculate initial price with market adjustments
        let adjusted_price = match (&self.market_type, &item.item_type) {
            // Black markets have higher margins
            (MarketType::Black, _) => (base_price as f32 * 1.2) as u32,
            
            // Industrial markets discount their production and mark up their needs
            (MarketType::Industrial, ItemType::Component) => (base_price as f32 * 0.9) as u32,
            (MarketType::Industrial, ItemType::Resource(_)) => (base_price as f32 * 1.1) as u32,
            
            // Mining markets discount resources and mark up components
            (MarketType::Mining, ItemType::Resource(_)) => (base_price as f32 * 0.85) as u32,
            (MarketType::Mining, ItemType::Component) => (base_price as f32 * 1.15) as u32,
            
            // Default price
            _ => base_price,
        };
        
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
        
        let market_item = MarketItem {
            item: item.clone(),
            quantity,
            base_price,
            current_price: adjusted_price,
            price_volatility: adjusted_volatility,
            supply_level: supply,
            demand_level: demand,
            price_history: vec![PriceHistory {
                timestamp: current_time,
                price: adjusted_price,
            }],
            production_rate: production,
            consumption_rate: consumption,
        };
        
        self.items.insert(item.name.clone(), market_item);
    }

    pub fn buy_item(&mut self, item_name: &str, quantity: u32) -> Option<(Item, u32, u32)> {
        // First check if we have the item and enough quantity
        let can_fulfill = self.items.get(item_name)
            .map(|item| item.quantity >= quantity)
            .unwrap_or(false);
            
        if !can_fulfill {
            return None;
        }
        
        // Get the current price and calculate costs
        let (base_cost, current_price) = {
            let item = self.items.get(item_name).unwrap(); // Safe because we checked above
            let price = item.current_price;
            (price * quantity, price)
        };
        
        let tax = (base_cost as f32 * self.tax_rate) as u32;
        let total_cost = base_cost + tax;
        
        // Now modify the item
        if let Some(market_item) = self.items.get_mut(item_name) {
            // Reduce available quantity
            market_item.quantity -= quantity;
            
            // After purchase, decrease supply (more scarcity)
            market_item.supply_level = (market_item.supply_level * 0.98).max(0.5);
            
            // Calculate new price based on supply and demand
            let supply_factor = 2.0 - market_item.supply_level; // Invert supply (less supply = higher price)
            let demand_factor = market_item.demand_level;
            
            // Calculate new price based on base price and economic factors
            let new_price = (market_item.base_price as f32 * 
                          supply_factor * 
                          demand_factor) as u32;
            
            // Ensure price doesn't change too drastically
            let max_change_pct = 0.2; // Max 20% change from current price
            let current = market_item.current_price as f32;
            let max_up = current * (1.0 + max_change_pct);
            let max_down = current * (1.0 - max_change_pct);
            
            market_item.current_price = new_price.max(max_down as u32).min(max_up as u32);
            
            // Also update the item's value to match
            market_item.item.value = market_item.current_price;
            
            // Record the price history
            let current_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or(Duration::from_secs(0))
                .as_secs();
                
            market_item.price_history.push(PriceHistory {
                timestamp: current_time,
                price: market_item.current_price,
            });
            
            // Keep history size manageable (last 10 price points)
            if market_item.price_history.len() > 10 {
                market_item.price_history.remove(0);
            }
            
            // Clone the item to return it
            let mut item = market_item.item.clone();
            item.value = current_price; // Set the value to the price at time of purchase
            
            return Some((item, quantity, total_cost));
        }
        
        None // Should never reach here but needed for completeness
    }

    pub fn sell_item(&mut self, item: Item, quantity: u32) -> u32 {
        // Calculate sell price (base value minus market margin)
        let (sell_price, exists_in_market) = match self.items.get(&item.name) {
            Some(market_item) => {
                // If item exists in market, use its current price with a sell margin
                let margin = match self.market_type {
                    MarketType::Black => 0.15, // Black markets take 15% margin
                    MarketType::Trading => 0.05, // Trading hubs only take 5%
                    _ => 0.10, // Standard 10% margin
                };
                ((market_item.current_price as f32 * (1.0 - margin)) as u32, true)
            },
            None => {
                // If item doesn't exist, use its intrinsic value
                ((item.value as f32 * 0.85) as u32, false) // Standard 15% markdown
            }
        };
        
        let revenue = sell_price * quantity;
        
        if exists_in_market {
            // Get a mutable reference to the item
            if let Some(market_item) = self.items.get_mut(&item.name) {
                // Increase available quantity
                market_item.quantity += quantity;
                
                // After sale, increase supply and decrease price slightly
                market_item.supply_level = (market_item.supply_level * 1.02).min(1.5);
                
                // Update the price directly here instead of calling self.update_item_price
                // to avoid a second mutable borrow
                let supply_factor = 2.0 - market_item.supply_level; // Invert supply (less supply = higher price)
                let demand_factor = market_item.demand_level;
                
                // Calculate new price based on base price and economic factors
                let new_price = (market_item.base_price as f32 * 
                              supply_factor * 
                              demand_factor) as u32;
                
                // Ensure price doesn't change too drastically
                let max_change_pct = 0.2; // Max 20% change from current price
                let current = market_item.current_price as f32;
                let max_up = current * (1.0 + max_change_pct);
                let max_down = current * (1.0 - max_change_pct);
                
                market_item.current_price = new_price.max(max_down as u32).min(max_up as u32);
                
                // Also update the item's value to match
                market_item.item.value = market_item.current_price;
                
                // Record the price history
                let current_time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or(Duration::from_secs(0))
                    .as_secs();
                    
                market_item.price_history.push(PriceHistory {
                    timestamp: current_time,
                    price: market_item.current_price,
                });
                
                // Keep history size manageable (last 10 price points)
                if market_item.price_history.len() > 10 {
                    market_item.price_history.remove(0);
                }
            }
        } else {
            // Add new item to market if it wasn't there before
            self.add_item(
                item.clone(), 
                quantity, 
                (item.value as f32 * 0.9) as u32, // Base price at 90% of item value
                0.1 // Default volatility
            );
        }
        
        revenue
    }

    // Update the price of a specific item based on supply and demand
    fn update_item_price(&mut self, item_name: &str) {
        if let Some(market_item) = self.items.get_mut(item_name) {
            // Calculate price adjustment based on supply and demand
            let supply_factor = 2.0 - market_item.supply_level; // Invert supply (less supply = higher price)
            let demand_factor = market_item.demand_level;
            
            // Calculate new price based on base price and economic factors
            let new_price = (market_item.base_price as f32 * 
                            supply_factor * 
                            demand_factor) as u32;
            
            // Ensure price doesn't change too drastically
            let max_change_pct = 0.2; // Max 20% change from current price
            let current = market_item.current_price as f32;
            let max_up = current * (1.0 + max_change_pct);
            let max_down = current * (1.0 - max_change_pct);
            
            market_item.current_price = new_price.max(max_down as u32).min(max_up as u32);
            
            // Also update the item's value to match
            market_item.item.value = market_item.current_price;
            
            // Record the price history
            let current_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or(Duration::from_secs(0))
                .as_secs();
                
            market_item.price_history.push(PriceHistory {
                timestamp: current_time,
                price: market_item.current_price,
            });
            
            // Keep history size manageable (last 10 price points)
            if market_item.price_history.len() > 10 {
                market_item.price_history.remove(0);
            }
        }
    }

    pub fn update_market(&mut self, game_time: u64) {
        self.last_update = game_time;
        
        // Process economic events
        for event in &self.local_events.clone() { // Clone to avoid borrowing issues
            self.apply_economic_event(event);
        }
        
        // We need to keep track of item names and their updated values
        // to avoid borrowing issues with self.update_item_price
        let mut item_updates = Vec::new();
        
        // First pass: update supply, demand, and consumption
        {
            let items = &mut self.items;
            for (item_name, market_item) in items.iter_mut() {
                // Simulate production
                if market_item.production_rate > 0 {
                    market_item.quantity += market_item.production_rate;
                    market_item.supply_level = (market_item.supply_level * 1.01).min(2.0);
                }
                
                // Simulate consumption
                if market_item.consumption_rate > 0 {
                    let consumed = market_item.consumption_rate.min(market_item.quantity);
                    market_item.quantity -= consumed;
                    
                    if consumed == market_item.consumption_rate {
                        // Full consumption indicates demand is being met
                        market_item.demand_level = (market_item.demand_level * 0.99).max(0.5);
                    } else {
                        // Partial consumption indicates demand exceeds supply
                        market_item.demand_level = (market_item.demand_level * 1.01).min(2.0);
                    }
                }
                
                // Apply random market fluctuation based on volatility
                let random_factor = rand::random::<f32>() * market_item.price_volatility * 0.2;
                if rand::random::<bool>() {
                    market_item.supply_level = (market_item.supply_level * (1.0 + random_factor)).min(2.0);
                } else {
                    market_item.supply_level = (market_item.supply_level * (1.0 - random_factor)).max(0.5);
                }
                
                // Calculate the price update directly here
                let supply_factor = 2.0 - market_item.supply_level; // Invert supply (less supply = higher price)
                let demand_factor = market_item.demand_level;
                
                // Calculate new price based on base price and economic factors
                let new_price = (market_item.base_price as f32 * 
                                supply_factor * 
                                demand_factor) as u32;
                
                // Ensure price doesn't change too drastically
                let max_change_pct = 0.2; // Max 20% change from current price
                let current = market_item.current_price as f32;
                let max_up = current * (1.0 + max_change_pct);
                let max_down = current * (1.0 - max_change_pct);
                
                let new_price = new_price.max(max_down as u32).min(max_up as u32);
                
                // Store the item_name and new price for the second pass
                item_updates.push((item_name.clone(), new_price));
                
                // Update the item's value
                market_item.current_price = new_price;
                market_item.item.value = new_price;
                
                // Record the price history
                let current_time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or(Duration::from_secs(0))
                    .as_secs();
                    
                market_item.price_history.push(PriceHistory {
                    timestamp: current_time,
                    price: new_price,
                });
                
                // Keep history size manageable (last 10 price points)
                if market_item.price_history.len() > 10 {
                    market_item.price_history.remove(0);
                }
            }
        }
        
        // Update trade orders - check if any orders should be executed based on current prices
        let items_copy = self.items.clone(); // Clone to avoid borrowing issues
        let order_current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
            
        for order in &mut self.trade_orders {
            if order.status != OrderStatus::Active {
                continue;
            }
            
            // Get the current market price for the item
            if let Some(market_item) = items_copy.get(&order.item_name) {
                let current_price = market_item.current_price;
                
                match order.order_type {
                    OrderType::Buy => {
                        // Execute buy orders when price falls below or equals target price
                        if current_price <= order.target_price {
                            order.status = OrderStatus::Completed;
                            order.executed_at = Some(order_current_time);
                            
                            // In a real implementation, we would transfer money and goods here
                            // For now, we just mark the order as completed
                        }
                    },
                    OrderType::Sell => {
                        // Execute sell orders when price rises above or equals target price
                        if current_price >= order.target_price {
                            order.status = OrderStatus::Completed;
                            order.executed_at = Some(order_current_time);
                            
                            // In a real implementation, we would transfer money and goods here
                            // For now, we just mark the order as completed
                        }
                    }
                }
            }
        }
        
        // Mark expired orders
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
            
        for order in &mut self.trade_orders {
            if order.status != OrderStatus::Active {
                continue;
            }
            
            // Check for expiration
            if let Some(expires_at) = order.expires_at {
                if current_time > expires_at {
                    order.status = OrderStatus::Cancelled;
                }
            }
        }
        
        // Randomly generate new economic events (5% chance per update)
        if rand::random::<f32>() < 0.05 {
            self.generate_random_event();
        }
        
        // Clear old events (events only last for a limited time)
        self.local_events.clear();
    }
    
    fn apply_economic_event(&mut self, event: &EconomicEvent) {
        match event {
            EconomicEvent::Shortage(item_name) => {
                if let Some(market_item) = self.items.get_mut(item_name) {
                    market_item.supply_level = (market_item.supply_level * 0.7).max(0.3);
                }
            },
            EconomicEvent::Surplus(item_name) => {
                if let Some(market_item) = self.items.get_mut(item_name) {
                    market_item.supply_level = (market_item.supply_level * 1.3).min(2.0);
                }
            },
            EconomicEvent::HighDemand(item_name) => {
                if let Some(market_item) = self.items.get_mut(item_name) {
                    market_item.demand_level = (market_item.demand_level * 1.3).min(2.0);
                }
            },
            EconomicEvent::LowDemand(item_name) => {
                if let Some(market_item) = self.items.get_mut(item_name) {
                    market_item.demand_level = (market_item.demand_level * 0.7).max(0.3);
                }
            },
            EconomicEvent::TariffIncrease => {
                self.tax_rate = (self.tax_rate * 1.2).min(0.25); // Cap at 25%
            },
            EconomicEvent::TariffDecrease => {
                self.tax_rate = (self.tax_rate * 0.8).max(0.01); // Minimum 1%
            },
            EconomicEvent::MarketCrash => {
                // Significant price decrease across all items
                for market_item in self.items.values_mut() {
                    market_item.current_price = (market_item.current_price as f32 * 0.7) as u32;
                    market_item.item.value = market_item.current_price;
                }
            },
            EconomicEvent::MarketBoom => {
                // Significant price increase across all items
                for market_item in self.items.values_mut() {
                    market_item.current_price = (market_item.current_price as f32 * 1.3) as u32;
                    market_item.item.value = market_item.current_price;
                }
            },
            EconomicEvent::LocalConflict => {
                // Military goods increase in price, civilian goods decrease
                for market_item in self.items.values_mut() {
                    match market_item.item.item_type {
                        ItemType::Equipment => {
                            // Assume equipment could be military
                            market_item.demand_level = (market_item.demand_level * 1.2).min(2.0);
                        },
                        ItemType::Product => {
                            // Civilian goods
                            market_item.demand_level = (market_item.demand_level * 0.8).max(0.5);
                        },
                        _ => {}
                    }
                }
            },
            EconomicEvent::LocalPeace => {
                // Opposite of conflict: military goods decrease, civilian increase
                for market_item in self.items.values_mut() {
                    match market_item.item.item_type {
                        ItemType::Equipment => {
                            // Military goods
                            market_item.demand_level = (market_item.demand_level * 0.8).max(0.5);
                        },
                        ItemType::Product => {
                            // Civilian goods
                            market_item.demand_level = (market_item.demand_level * 1.2).min(2.0);
                        },
                        _ => {}
                    }
                }
            }
        }
    }
    
    fn generate_random_event(&mut self) {
        // Get a random item from the market
        if self.items.is_empty() {
            return;
        }
        
        let item_names: Vec<String> = self.items.keys().cloned().collect();
        let random_index = rand::random::<usize>() % item_names.len();
        let random_item = &item_names[random_index];
        
        // Generate a random event type
        let event_roll = rand::random::<f32>();
        let event = if event_roll < 0.15 {
            EconomicEvent::Shortage(random_item.clone())
        } else if event_roll < 0.30 {
            EconomicEvent::Surplus(random_item.clone())
        } else if event_roll < 0.45 {
            EconomicEvent::HighDemand(random_item.clone())
        } else if event_roll < 0.60 {
            EconomicEvent::LowDemand(random_item.clone())
        } else if event_roll < 0.65 {
            EconomicEvent::TariffIncrease
        } else if event_roll < 0.70 {
            EconomicEvent::TariffDecrease
        } else if event_roll < 0.75 {
            EconomicEvent::MarketCrash
        } else if event_roll < 0.80 {
            EconomicEvent::MarketBoom
        } else if event_roll < 0.90 {
            EconomicEvent::LocalConflict
        } else {
            EconomicEvent::LocalPeace
        };
        
        self.local_events.push(event);
    }
    
    // Get price trend information for an item
    pub fn get_price_trend(&self, item_name: &str) -> Option<(f32, String)> {
        if let Some(market_item) = self.items.get(item_name) {
            if market_item.price_history.len() < 2 {
                return Some((0.0, "Stable".to_string()));
            }
            
            let latest = market_item.price_history.last().unwrap().price as f32;
            let previous = market_item.price_history[market_item.price_history.len() - 2].price as f32;
            
            let percent_change = (latest - previous) / previous * 100.0;
            
            let trend = if percent_change > 10.0 {
                "Skyrocketing"
            } else if percent_change > 5.0 {
                "Rising"
            } else if percent_change > 1.0 {
                "Increasing"
            } else if percent_change < -10.0 {
                "Plummeting"
            } else if percent_change < -5.0 {
                "Falling"
            } else if percent_change < -1.0 {
                "Decreasing"
            } else {
                "Stable"
            };
            
            return Some((percent_change, trend.to_string()));
        }
        
        None
    }
    
    // === Trade Order Methods ===
    
    // Create a new buy order
    pub fn create_buy_order(&mut self, player_id: &str, item_name: &str, quantity: u32, target_price: u32, expires_at: Option<u64>, notes: &str) -> Option<Uuid> {
        // Verify item exists in market
        if !self.items.contains_key(item_name) {
            return None;
        }
        
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
            
        let order_id = Uuid::new_v4();
        
        let order = TradeOrder {
            id: order_id,
            player_id: player_id.to_string(),
            system_id: self.system_id.clone(),
            item_name: item_name.to_string(),
            order_type: OrderType::Buy,
            quantity,
            target_price,
            price_condition: "below".to_string(),
            status: OrderStatus::Active,
            created_at: current_time,
            expires_at,
            executed_at: None,
            notes: notes.to_string(),
        };
        
        self.trade_orders.push(order);
        Some(order_id)
    }
    
    // Create a new sell order
    pub fn create_sell_order(&mut self, player_id: &str, item_name: &str, quantity: u32, target_price: u32, expires_at: Option<u64>, notes: &str) -> Option<Uuid> {
        // For sell orders, we don't need to verify if the item exists in the market
        // since the player could be selling something not currently available
        
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
            
        let order_id = Uuid::new_v4();
        
        let order = TradeOrder {
            id: order_id,
            player_id: player_id.to_string(),
            system_id: self.system_id.clone(),
            item_name: item_name.to_string(),
            order_type: OrderType::Sell,
            quantity,
            target_price,
            price_condition: "above".to_string(),
            status: OrderStatus::Active,
            created_at: current_time,
            expires_at,
            executed_at: None,
            notes: notes.to_string(),
        };
        
        self.trade_orders.push(order);
        Some(order_id)
    }
    
    // Cancel an existing order
    pub fn cancel_order(&mut self, order_id: &Uuid, player_id: &str) -> bool {
        if let Some(pos) = self.trade_orders.iter().position(|order| {
            order.id == *order_id && order.player_id == player_id && order.status == OrderStatus::Active
        }) {
            self.trade_orders[pos].status = OrderStatus::Cancelled;
            true
        } else {
            false
        }
    }
    
    // Get all active orders for a player
    pub fn get_player_orders(&self, player_id: &str) -> Vec<&TradeOrder> {
        self.trade_orders.iter()
            .filter(|order| order.player_id == player_id && order.status == OrderStatus::Active)
            .collect()
    }
    
    // Process all active orders and execute any that meet their conditions
    pub fn process_orders(&mut self, player_inventory: &mut HashMap<Item, u32>, player_credits: &mut u32) -> Vec<TradeOrder> {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
            
        // First, collect all orders that need to be processed
        let mut orders_to_process = Vec::new();
        let mut orders_to_mark_expired = Vec::new();
        
        // Gather info without mutably borrowing self
        for (order_idx, order) in self.trade_orders.iter().enumerate() {
            // Skip if not active
            if order.status != OrderStatus::Active {
                continue;
            }
            
            // Check for expiration
            if let Some(expires_at) = order.expires_at {
                if current_time > expires_at {
                    orders_to_mark_expired.push(order_idx);
                    continue;
                }
            }
            
            // Get current price of the item
            if let Some(market_item) = self.items.get(&order.item_name) {
                let current_price = market_item.current_price;
                let market_quantity = market_item.quantity;
                
                match order.order_type {
                    OrderType::Buy => {
                        // Execute if price falls below target price
                        if current_price <= order.target_price {
                            // Calculate total cost
                            let base_cost = current_price * order.quantity;
                            let tax = (base_cost as f32 * self.tax_rate) as u32;
                            let total_cost = base_cost + tax;
                            
                            // Check if the player has enough credits and market has enough quantity
                            if *player_credits >= total_cost && market_quantity >= order.quantity {
                                orders_to_process.push((
                                    order_idx,
                                    order.clone(),
                                    total_cost,
                                    None
                                ));
                            }
                        }
                    },
                    OrderType::Sell => {
                        // Execute if price rises above target price
                        if current_price >= order.target_price {
                            // Find matching item in player inventory
                            let mut item_to_sell = None;
                            let mut available_quantity = 0;
                            
                            // Check if player has the item to sell
                            for (item, quantity) in player_inventory.iter() {
                                if item.name == order.item_name {
                                    item_to_sell = Some(item.clone());
                                    available_quantity = *quantity;
                                    break;
                                }
                            }
                            
                            if let Some(item) = item_to_sell {
                                // Check if player has enough quantity
                                if available_quantity >= order.quantity {
                                    // Calculate expected revenue (will be more accurate when we actually sell)
                                    let expected_revenue = (current_price as f32 * 0.9) as u32 * order.quantity;
                                    
                                    orders_to_process.push((
                                        order_idx,
                                        order.clone(),
                                        expected_revenue,
                                        Some(item)
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Now mark expired orders
        for order_idx in orders_to_mark_expired {
            if let Some(order) = self.trade_orders.get_mut(order_idx) {
                order.status = OrderStatus::Cancelled;
            }
        }
        
        // Track which orders were executed
        let mut executed_orders = Vec::new();
        
        // Now process the orders that meet the conditions
        // We can do this without borrowing self.trade_orders at the same time
        for (order_idx, order, price, item_to_sell) in orders_to_process {
            match order.order_type {
                OrderType::Buy => {
                    // Execute the buy order - this will modify market state
                    if let Some((item, quantity, _)) = self.buy_item(&order.item_name, order.quantity) {
                        // Deduct cost from player credits
                        *player_credits -= price;
                        
                        // Add item to player inventory
                        *player_inventory.entry(item).or_insert(0) += quantity;
                        
                        // Mark order as completed - but we need to do it after all processing
                        if let Some(order) = self.trade_orders.get_mut(order_idx) {
                            order.status = OrderStatus::Completed;
                            order.executed_at = Some(current_time);
                            
                            // Track the completed order
                            executed_orders.push(order.clone());
                        }
                    }
                },
                OrderType::Sell => {
                    if let Some(item) = item_to_sell {
                        // Execute the sell order
                        let revenue = self.sell_item(item.clone(), order.quantity);
                        
                        // Add revenue to player credits
                        *player_credits += revenue;
                        
                        // Reduce player inventory
                        if let Some(qty) = player_inventory.get_mut(&item) {
                            *qty -= order.quantity;
                            if *qty == 0 {
                                player_inventory.remove(&item);
                            }
                        }
                        
                        // Mark order as completed
                        if let Some(order) = self.trade_orders.get_mut(order_idx) {
                            order.status = OrderStatus::Completed;
                            order.executed_at = Some(current_time);
                            
                            // Track the completed order
                            executed_orders.push(order.clone());
                        }
                    }
                }
            }
        }
        
        // Return list of executed orders
        executed_orders
    }
}
