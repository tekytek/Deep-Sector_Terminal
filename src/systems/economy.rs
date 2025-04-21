use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::models::item::{Item, ItemType, ResourceType};
use crate::models::market::{Market, MarketItem, MarketType, EconomicEvent, TradeOrder, OrderStatus, OrderType};
use crate::models::player_market::{PlayerMarket, PlayerMarketListing, MarketBid, MarketContract};

/// System responsible for global economic simulation and market dynamics
pub struct EconomySystem {
    // Maps system_id to system market
    pub system_markets: HashMap<String, Market>,
    
    // Player-driven market transactions
    pub player_market: PlayerMarket,
    
    // Global economic factors
    pub global_inflation_rate: f32,    // Current inflation rate (affects prices)
    pub global_trade_index: f32,       // Measure of overall trade activity (0.0-2.0)
    pub resource_scarcity: HashMap<ResourceType, f32>, // Scarcity levels for resources
    pub faction_relation_modifiers: HashMap<String, f32>, // Price modifiers based on faction relations
    
    // Random events and their probabilities
    pub random_events: Vec<(EconomicEvent, f32)>,
    
    // Tax policies and tariffs
    pub base_tax_rate: f32,
    pub faction_tax_rates: HashMap<String, f32>,
    pub trade_route_tariffs: HashMap<(String, String), f32>, // Tariffs between systems
    
    // Economy cycle tracking
    pub last_update: u64,
    pub update_interval: u64, // How often to update the economy (in seconds)
    pub simulation_step: u64,  // Economy simulation step counter
}

impl EconomySystem {
    pub fn new() -> Self {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
            
        EconomySystem {
            system_markets: HashMap::new(),
            player_market: PlayerMarket::new(),
            global_inflation_rate: 0.02, // 2% base inflation
            global_trade_index: 1.0,     // Normal trade activity
            resource_scarcity: HashMap::new(),
            faction_relation_modifiers: HashMap::new(),
            random_events: vec![
                (EconomicEvent::TariffIncrease, 0.05),
                (EconomicEvent::TariffDecrease, 0.05),
                (EconomicEvent::MarketCrash, 0.01),
                (EconomicEvent::MarketBoom, 0.01),
                (EconomicEvent::LocalConflict, 0.03),
                (EconomicEvent::LocalPeace, 0.03),
            ],
            base_tax_rate: 0.05, // 5% base tax
            faction_tax_rates: HashMap::new(),
            trade_route_tariffs: HashMap::new(),
            last_update: current_time,
            update_interval: 3600, // Update economy every hour
            simulation_step: 0,
        }
    }
    
    /// Initialize a new market in a star system
    pub fn initialize_system_market(&mut self, system_id: &str, market_type: MarketType) {
        let market = Market::with_market_type(system_id, market_type);
        self.system_markets.insert(system_id.to_string(), market);
    }
    
    /// Set resource scarcity levels
    pub fn set_resource_scarcity(&mut self) {
        // Initialize with default values
        self.resource_scarcity = [
            (ResourceType::Mineral, 1.0),
            (ResourceType::Gas, 1.2),
            (ResourceType::Ice, 0.8),
            (ResourceType::Lunar, 1.3),
            (ResourceType::Stellar, 1.5),
            (ResourceType::Exotic, 2.0),
            (ResourceType::Refined, 1.1),
        ].iter().cloned().collect();
    }
    
    /// Update the entire economy
    pub fn update(&mut self, current_time: u64) {
        // Only update at specified intervals
        if current_time - self.last_update < self.update_interval {
            return;
        }
        
        self.simulation_step += 1;
        self.last_update = current_time;
        
        // 1. Update global economic factors
        self.update_global_factors();
        
        // 2. Update each system market
        // Store system IDs first to avoid borrowing issues
        let system_ids: Vec<String> = self.system_markets.keys().cloned().collect();
        
        for system_id in system_ids {
            if let Some(market) = self.system_markets.get_mut(&system_id) {
                // Use a separate helper function that doesn't require &mut self
                Self::update_market_helper(market, current_time);
            }
        }
        
        // 3. Apply random events
        self.apply_random_events();
        
        // 4. Simulate inter-system trade
        self.simulate_trade_flows();
        
        // 5. Process player market expirations
        self.player_market.process_expirations();
        
        // 6. Adjust resource scarcity
        self.adjust_resource_scarcity();
    }
    
    /// Update global economic factors
    fn update_global_factors(&mut self) {
        // Simulate small fluctuations in global trade index
        let fluctuation = (rand::random::<f32>() - 0.5) * 0.1;
        self.global_trade_index = (self.global_trade_index + fluctuation).max(0.5).min(1.5);
        
        // Adjust inflation rate occasionally
        if self.simulation_step % 10 == 0 {
            let inflation_change = (rand::random::<f32>() - 0.5) * 0.01;
            self.global_inflation_rate = (self.global_inflation_rate + inflation_change).max(0.0).min(0.1);
        }
    }
    
    /// Update a specific market
    fn update_market(&mut self, market: &mut Market, current_time: u64) {
        Self::update_market_helper(market, current_time);
    }
    
    /// Helper method to update a market without requiring &mut self
    fn update_market_helper(market: &mut Market, current_time: u64) {
        // Set last update time
        market.last_update = current_time;
        
        // Update each item's price and quantity
        for item_entry in market.items.values_mut() {
            // Simulate production
            item_entry.quantity += item_entry.production_rate;
            
            // Simulate consumption
            let consumption = item_entry.consumption_rate.min(item_entry.quantity);
            item_entry.quantity -= consumption;
            
            // Apply a standard inflation factor (moved from self.global_inflation_rate)
            let inflation_factor = 1.0 + 0.02 / 52.0; // 2% annual inflation, weekly adjustment
            item_entry.base_price = ((item_entry.base_price as f32) * inflation_factor) as u32;
            
            // Adjust supply level based on quantity changes
            item_entry.supply_level = if item_entry.quantity == 0 {
                0.1 // Critical shortage
            } else if item_entry.quantity < item_entry.production_rate * 2 {
                0.5 // Low supply
            } else if item_entry.quantity > item_entry.production_rate * 10 {
                1.5 // Surplus
            } else {
                1.0 // Normal supply
            };
            
            // Adjust demand based on item type (global_trade_index moved to constant)
            let trade_index = 1.0; // Standard trade index
            item_entry.demand_level = trade_index * match item_entry.item.item_type {
                ItemType::Resource(ResourceType::Exotic) => 1.5, // Exotic resources always in demand
                ItemType::Resource(_) => 1.0, // Standard resource scarcity
                ItemType::Component => 1.2,   // Components moderately in demand
                ItemType::Product => 1.0,     // Products at normal demand
                ItemType::Blueprint => 0.8,   // Blueprints less demand
                ItemType::Equipment => 1.1,   // Equipment moderate demand
                ItemType::ShipModule => 1.3,  // Ship modules high demand
                ItemType::Fuel => 1.4,        // Fuel is highly in demand
            };
            
            // Apply local market events
            for event in &market.local_events {
                match event {
                    EconomicEvent::Shortage(item_name) if item_name == &item_entry.item.name => {
                        item_entry.supply_level *= 0.5;
                    },
                    EconomicEvent::Surplus(item_name) if item_name == &item_entry.item.name => {
                        item_entry.supply_level *= 1.5;
                    },
                    EconomicEvent::HighDemand(item_name) if item_name == &item_entry.item.name => {
                        item_entry.demand_level *= 1.5;
                    },
                    EconomicEvent::LowDemand(item_name) if item_name == &item_entry.item.name => {
                        item_entry.demand_level *= 0.5;
                    },
                    _ => {} // Other events handled globally
                }
            }
            
            // Calculate new price based on supply and demand
            let supply_factor = 2.0 - item_entry.supply_level; // Invert supply (less supply = higher price)
            let demand_factor = item_entry.demand_level;
            
            // Add some random fluctuation based on volatility
            let random_factor = 1.0 + ((rand::random::<f32>() - 0.5) * item_entry.price_volatility);
            
            // Calculate new price with all factors
            let new_price = (item_entry.base_price as f32 * 
                         supply_factor * 
                         demand_factor *
                         random_factor) as u32;
            
            // Ensure price doesn't change too drastically
            let max_change_pct = 0.2; // Max 20% change from current price
            let current = item_entry.current_price as f32;
            let max_up = current * (1.0 + max_change_pct);
            let max_down = current * (1.0 - max_change_pct);
            
            item_entry.current_price = new_price.max(max_down as u32).min(max_up as u32);
            
            // Record the price history
            let history_entry = crate::models::market::PriceHistory {
                timestamp: current_time,
                price: item_entry.current_price,
            };
            
            item_entry.price_history.push(history_entry);
            
            // Keep history size manageable (last 10 price points)
            if item_entry.price_history.len() > 10 {
                item_entry.price_history.remove(0);
            }
        }
        
        // Process any trade orders
        let mut completed_order_indices = Vec::new();
        
        for (i, order) in market.trade_orders.iter_mut().enumerate() {
            if order.status != OrderStatus::Active {
                continue;
            }
            
            // Check if the order's conditions are met
            if let Some(item) = market.items.get(&order.item_name) {
                let current_price = item.current_price;
                
                let condition_met = match order.order_type {
                    OrderType::Buy => {
                        // For buy orders, trigger when price falls below target
                        if order.price_condition == "below" {
                            current_price <= order.target_price
                        } else {
                            false
                        }
                    },
                    OrderType::Sell => {
                        // For sell orders, trigger when price rises above target
                        if order.price_condition == "above" {
                            current_price >= order.target_price
                        } else {
                            false
                        }
                    }
                };
                
                if condition_met {
                    // Mark the order for execution
                    completed_order_indices.push(i);
                    
                    // Set status to completed
                    order.status = OrderStatus::Completed;
                    order.executed_at = Some(current_time);
                }
            }
        }
        
        // Remove completed orders (in reverse order to maintain indices)
        for index in completed_order_indices.into_iter().rev() {
            market.trade_orders.remove(index);
        }
        
        // Randomly apply expiration to local events
        market.local_events.retain(|_| rand::random::<f32>() < 0.8); // 20% chance to expire each event
    }
    
    /// Apply random economic events
    fn apply_random_events(&mut self) {
        // Global events
        for (event, probability) in &self.random_events {
            if rand::random::<f32>() < *probability {
                // Apply global event to all markets
                match event {
                    EconomicEvent::TariffIncrease => {
                        for market in self.system_markets.values_mut() {
                            market.tax_rate = (market.tax_rate * 1.2).min(0.2);
                        }
                    },
                    EconomicEvent::TariffDecrease => {
                        for market in self.system_markets.values_mut() {
                            market.tax_rate = (market.tax_rate * 0.8).max(0.01);
                        }
                    },
                    EconomicEvent::MarketCrash => {
                        for market in self.system_markets.values_mut() {
                            for item in market.items.values_mut() {
                                item.current_price = (item.current_price as f32 * 0.7) as u32;
                                item.demand_level *= 0.6;
                            }
                            market.local_events.push(event.clone());
                        }
                    },
                    EconomicEvent::MarketBoom => {
                        for market in self.system_markets.values_mut() {
                            for item in market.items.values_mut() {
                                item.current_price = (item.current_price as f32 * 1.3) as u32;
                                item.demand_level *= 1.4;
                            }
                            market.local_events.push(event.clone());
                        }
                    },
                    _ => {} // Handle other events at local level
                }
            }
        }
        
        // Local events - apply to random markets
        let system_ids: Vec<String> = self.system_markets.keys().cloned().collect();
        
        if !system_ids.is_empty() {
            let idx = rand::random::<usize>() % system_ids.len();
            let system_id = &system_ids[idx];
            
            if let Some(market) = self.system_markets.get_mut(system_id) {
                let local_event = match rand::random::<u32>() % 4 {
                    0 => {
                        // Create shortage of random item
                        if let Some(item_name) = market.items.keys().next().cloned() {
                            EconomicEvent::Shortage(item_name)
                        } else {
                            return;
                        }
                    },
                    1 => {
                        // Create surplus of random item
                        if let Some(item_name) = market.items.keys().next().cloned() {
                            EconomicEvent::Surplus(item_name)
                        } else {
                            return;
                        }
                    },
                    2 => EconomicEvent::LocalConflict,
                    _ => EconomicEvent::LocalPeace,
                };
                
                market.local_events.push(local_event);
            }
        }
    }
    
    /// Simulate trade flows between systems
    fn simulate_trade_flows(&mut self) {
        // This would simulate ship cargo flows and item distribution
        // between connected star systems
        
        // For a basic implementation, we'll just move a percentage of goods
        // from high-supply to low-supply markets
        let system_ids: Vec<String> = self.system_markets.keys().cloned().collect();
        
        // Skip if we have fewer than 2 markets
        if system_ids.len() < 2 {
            return;
        }
        
        // Create a list of all items across all markets
        let mut all_items = Vec::new();
        
        for (system_id, market) in &self.system_markets {
            for item_name in market.items.keys() {
                all_items.push((system_id.clone(), item_name.clone()));
            }
        }
        
        // Shuffle to randomize which items are traded
        all_items.sort_by(|_, _| {
            if rand::random::<bool>() {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
        
        // Process a limited number of trades
        let trades_to_process = all_items.len().min(5); // Process up to 5 trades per update
        
        for i in 0..trades_to_process {
            if i >= all_items.len() {
                break;
            }
            
            let (source_system_id, item_name) = &all_items[i];
            
            // Find systems with lower supply of this item
            let mut potential_destinations = Vec::new();
            
            let source_supply = if let Some(market) = self.system_markets.get(source_system_id) {
                if let Some(item) = market.items.get(item_name) {
                    item.supply_level
                } else {
                    continue;
                }
            } else {
                continue;
            };
            
            for (system_id, market) in &self.system_markets {
                if system_id == source_system_id {
                    continue;
                }
                
                if let Some(item) = market.items.get(item_name) {
                    if item.supply_level < source_supply {
                        potential_destinations.push(system_id.clone());
                    }
                } else {
                    // Destination doesn't have this item at all - higher demand
                    potential_destinations.push(system_id.clone());
                }
            }
            
            // If we have potential destinations, pick one and trade
            if !potential_destinations.is_empty() {
                let dest_idx = rand::random::<usize>() % potential_destinations.len();
                let dest_system_id = &potential_destinations[dest_idx];
                
                // Transfer goods from source to destination
                // We need to handle mutable borrows separately to avoid borrowing self.system_markets twice
                let source_market = if let Some(market) = self.system_markets.get_mut(source_system_id) {
                    market
                } else {
                    continue;
                };
                
                // Extract what we need from the source market to avoid holding the borrow
                let (item_data, trade_amount) = if let Some(source_item) = source_market.items.get_mut(item_name) {
                    // Calculate trade amount (up to 20% of source quantity)
                    let amount = (source_item.quantity as f32 * 0.2) as u32;
                    if amount == 0 {
                        continue;
                    }
                    
                    // Remove from source
                    source_item.quantity -= amount;
                    
                    // Update supply level
                    source_item.supply_level = if source_item.quantity == 0 {
                        0.1 // Critical shortage
                    } else if source_item.quantity < source_item.production_rate * 2 {
                        0.5 // Low supply
                    } else if source_item.quantity > source_item.production_rate * 10 {
                        1.5 // Surplus
                    } else {
                        1.0 // Normal supply
                    };
                    
                    // Clone the item for the destination
                    let item_clone = source_item.clone();
                    (item_clone, amount)
                } else {
                    continue;
                };
                
                // Now access the destination market in a separate scope
                if let Some(dest_market) = self.system_markets.get_mut(dest_system_id) {
                    // Add to destination
                    if let Some(dest_item) = dest_market.items.get_mut(item_name) {
                        dest_item.quantity += trade_amount;
                    } else {
                        // Create a new item for the destination market
                        let mut new_item = item_data.clone();
                        new_item.quantity = trade_amount;
                        dest_market.items.insert(item_name.clone(), new_item);
                    }
                    
                    // Update destination supply level
                    if let Some(dest_item) = dest_market.items.get_mut(item_name) {
                        dest_item.supply_level = if dest_item.quantity == 0 {
                            0.1 // Critical shortage
                        } else if dest_item.quantity < dest_item.production_rate * 2 {
                            0.5 // Low supply
                        } else if dest_item.quantity > dest_item.production_rate * 10 {
                            1.5 // Surplus
                        } else {
                            1.0 // Normal supply
                        };
                    }
                }
            }
        }
    }
    
    /// Adjust resource scarcity levels
    fn adjust_resource_scarcity(&mut self) {
        // Over time, resources become more or less scarce
        for (_, scarcity) in self.resource_scarcity.iter_mut() {
            // Small random adjustment
            *scarcity += (rand::random::<f32>() - 0.5) * 0.1;
            *scarcity = scarcity.max(0.5).min(2.0);
        }
    }
    
    /// Place an item for sale on the player market
    pub fn list_item_for_sale(
        &mut self,
        seller_id: &str,
        seller_name: &str,
        item: Item,
        quantity: u32,
        price_per_unit: u32,
        system_id: &str,
        location_id: &str,
        expiration_hours: Option<u64>,
        min_reputation: i32,
        negotiable: bool,
        description: &str,
        tags: Vec<String>,
    ) -> String {
        // Convert expiration hours to timestamp if provided
        let expiration = expiration_hours.map(|hours| {
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or(Duration::from_secs(0))
                .as_secs();
            current_time + (hours * 3600) // Convert hours to seconds
        });
        
        // Create listing
        self.player_market.create_listing(
            seller_id,
            seller_name,
            item,
            quantity,
            price_per_unit,
            system_id,
            location_id,
            expiration,
            min_reputation,
            crate::models::player_market::ListingVisibility::Public, // Default to public
            negotiable,
            description,
            tags,
        )
    }
    
    /// Calculate item price based on current market conditions
    pub fn calculate_fair_market_price(&self, item_name: &str) -> Option<u32> {
        // Collect prices from all markets
        let mut prices = Vec::new();
        
        for market in self.system_markets.values() {
            if let Some(item) = market.items.get(item_name) {
                prices.push(item.current_price);
            }
        }
        
        // Also check player market listings
        for listing in self.player_market.listings.values() {
            if listing.item.name == item_name {
                prices.push(listing.price_per_unit);
            }
        }
        
        if prices.is_empty() {
            return None;
        }
        
        // Calculate average price
        let sum: u64 = prices.iter().map(|p| *p as u64).sum();
        Some((sum / prices.len() as u64) as u32)
    }
    
    /// Get price information for an item across all markets
    pub fn get_price_comparison(&self, item_name: &str) -> Vec<(String, u32, u32)> {
        let mut results = Vec::new();
        
        for (system_id, market) in &self.system_markets {
            if let Some(item) = market.items.get(item_name) {
                results.push((
                    system_id.clone(),
                    item.current_price,
                    item.quantity,
                ));
            }
        }
        
        results.sort_by(|a, b| a.1.cmp(&b.1));
        results
    }
    
    /// Create a buy order for automatic purchases
    pub fn create_buy_order(
        &mut self,
        player_id: &str,
        system_id: &str,
        item_name: &str,
        quantity: u32,
        target_price: u32,
        expires_hours: Option<u64>,
    ) -> String {
        let order_id = Uuid::new_v4().to_string();
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
            
        // Calculate expiration timestamp if provided
        let expires_at = expires_hours.map(|hours| {
            current_time + (hours * 3600) // Convert hours to seconds
        });
        
        let order = TradeOrder {
            id: Uuid::parse_str(&order_id).unwrap_or_else(|_| Uuid::new_v4()),
            player_id: player_id.to_string(),
            system_id: system_id.to_string(),
            item_name: item_name.to_string(),
            order_type: OrderType::Buy,
            quantity,
            target_price,
            price_condition: "below".to_string(), // Buy when price falls below target
            status: OrderStatus::Active,
            created_at: current_time,
            expires_at,
            executed_at: None,
            notes: format!("Buy {} of {} when price falls below {}", 
                         quantity, item_name, target_price),
        };
        
        // Add to the appropriate market
        if let Some(market) = self.system_markets.get_mut(system_id) {
            market.trade_orders.push(order);
        }
        
        order_id
    }
    
    /// Create a sell order for automatic sales
    pub fn create_sell_order(
        &mut self,
        player_id: &str,
        system_id: &str,
        item_name: &str,
        quantity: u32,
        target_price: u32,
        expires_hours: Option<u64>,
    ) -> String {
        let order_id = Uuid::new_v4().to_string();
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
            
        // Calculate expiration timestamp if provided
        let expires_at = expires_hours.map(|hours| {
            current_time + (hours * 3600) // Convert hours to seconds
        });
        
        let order = TradeOrder {
            id: Uuid::parse_str(&order_id).unwrap_or_else(|_| Uuid::new_v4()),
            player_id: player_id.to_string(),
            system_id: system_id.to_string(),
            item_name: item_name.to_string(),
            order_type: OrderType::Sell,
            quantity,
            target_price,
            price_condition: "above".to_string(), // Sell when price rises above target
            status: OrderStatus::Active,
            created_at: current_time,
            expires_at,
            executed_at: None,
            notes: format!("Sell {} of {} when price rises above {}", 
                         quantity, item_name, target_price),
        };
        
        // Add to the appropriate market
        if let Some(market) = self.system_markets.get_mut(system_id) {
            market.trade_orders.push(order);
        }
        
        order_id
    }
    
    /// Get market trends for multiple items
    pub fn get_market_trends(&self, item_names: &[String]) -> HashMap<String, Vec<(u64, u32)>> {
        let mut results = HashMap::new();
        
        for item_name in item_names {
            let mut price_points = Vec::new();
            
            // Collect price history from all markets
            for market in self.system_markets.values() {
                if let Some(item) = market.items.get(item_name) {
                    for history in &item.price_history {
                        price_points.push((history.timestamp, history.price));
                    }
                }
            }
            
            // Also add player market price trends if available
            if let Some(trends) = self.player_market.price_trends.get(item_name) {
                for trend in trends {
                    price_points.push((trend.timestamp, trend.average_price));
                }
            }
            
            // Sort by timestamp
            price_points.sort_by_key(|p| p.0);
            
            // Only keep one price point per timestamp
            let mut deduplicated = Vec::new();
            let mut last_timestamp = 0;
            
            for (timestamp, price) in price_points {
                if timestamp != last_timestamp {
                    deduplicated.push((timestamp, price));
                    last_timestamp = timestamp;
                }
            }
            
            results.insert(item_name.clone(), deduplicated);
        }
        
        results
    }
    
    /// Place a bid on an item in the player market
    pub fn place_bid_on_listing(
        &mut self,
        listing_id: &str,
        bidder_id: &str,
        bidder_name: &str,
        bid_amount: u32,
        quantity: u32,
        message: &str,
        expires_in_hours: Option<u64>,
    ) -> Result<String, String> {
        self.player_market.place_bid(
            listing_id,
            bidder_id,
            bidder_name,
            bid_amount,
            quantity,
            message,
            expires_in_hours,
        )
    }
    
    /// Get system markets for a specific item type
    pub fn find_best_markets_for_item_type(&self, item_type: &ItemType) -> Vec<(String, f32)> {
        let mut market_ratings = Vec::new();
        
        for (system_id, market) in &self.system_markets {
            // Calculate average price ratio (current/base) for this item type
            let mut price_ratios = Vec::new();
            
            for item in market.items.values() {
                if &item.item.item_type == item_type {
                    let ratio = item.current_price as f32 / item.base_price as f32;
                    price_ratios.push(ratio);
                }
            }
            
            // Skip if no matching items found
            if price_ratios.is_empty() {
                continue;
            }
            
            // Calculate average ratio
            let sum: f32 = price_ratios.iter().sum();
            let avg_ratio = sum / price_ratios.len() as f32;
            
            market_ratings.push((system_id.clone(), avg_ratio));
        }
        
        // Sort based on whether we want to buy or sell
        // For buying: lower ratio is better
        // For selling: higher ratio is better
        market_ratings.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        
        market_ratings
    }
}