use std::time::Duration;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::models::player::Player;
use crate::models::universe::Universe;
use crate::models::item::Item;
use crate::models::market::{TradeOrder, OrderType, OrderStatus};

#[derive(Serialize, Deserialize)]
pub struct TradingSystem {
    buy_mode: bool,
    transaction_cooldown: Duration,
    last_transaction: Duration,
    
    // Order management fields
    #[serde(default)]
    current_order_tab: usize,
    #[serde(default)]
    order_buy_mode: bool,
    #[serde(default, skip)]
    selected_order_index: Option<usize>,
}

#[allow(dead_code)]
impl TradingSystem {
    pub fn new() -> Self {
        let trading_system = TradingSystem {
            buy_mode: true,
            transaction_cooldown: Duration::from_millis(500),
            last_transaction: Duration::from_secs(0),
            current_order_tab: 0,
            order_buy_mode: true,
            selected_order_index: None,
        };
        
        // For testing purposes: create some test orders in the market
        trading_system.create_test_orders();
        
        trading_system
    }
    
    // Create some test orders for development/testing
    fn create_test_orders(&self) {
        // Note: This is a testing function that creates sample trade orders
        // In a production environment, this would be removed or disabled
        
        // Get the universe singleton
        let mut universe = Universe::new();
        
        println!("\n===== CREATING TEST TRADE ORDERS =====");
        
        // Get the first system with a market
        if let Some(system_id) = universe.get_all_system_ids().first() {
            if let Some(mut market) = universe.get_market_mut(system_id) {
                // Add some test orders for our player
                let player_id = "test_player_id".to_string();
                
                // Create a buy order for Iron at a high target price (should execute quickly)
                let iron_id = market.create_buy_order(
                    &player_id,
                    "Iron",
                    100,
                    500, // High target price, should execute when price is <= 500
                    None,
                    "Test buy order - Iron"
                );
                
                println!("✓ Created buy order for Iron (ID: {:?})", iron_id);
                
                // Create a sell order for Gold at a low target price (should execute quickly)
                let gold_id = market.create_sell_order(
                    &player_id,
                    "Gold",
                    50,
                    100, // Low target price, should execute when price is >= 100
                    None,
                    "Test sell order - Gold"
                );
                
                println!("✓ Created sell order for Gold (ID: {:?})", gold_id);
                
                // Create more test orders for other items
                let oxygen_id = market.create_buy_order(
                    &player_id,
                    "Oxygen",
                    200,
                    300, // Target price
                    None,
                    "Test buy order - Oxygen"
                );
                
                println!("✓ Created buy order for Oxygen (ID: {:?})", oxygen_id);
                
                let titanium_id = market.create_sell_order(
                    &player_id,
                    "Titanium",
                    75,
                    900, // Target price
                    None,
                    "Test sell order - Titanium"
                );
                
                println!("✓ Created sell order for Titanium (ID: {:?})", titanium_id);
                
                // Update the market in the universe
                universe.update_market(market);
                
                println!("✓ Created test orders in market of system {}", system_id);
            }
        }
        
        println!("=======================================\n");
    }

    pub fn set_buy_mode(&mut self, buy: bool) {
        self.buy_mode = buy;
    }

    pub fn is_buy_mode(&self) -> bool {
        self.buy_mode
    }

    pub fn buy_item(&mut self, player: &mut Player, item_index: usize) -> Option<String> {
        // Check if player is docked
        if !player.is_docked {
            return Some("You must be docked at a station to trade".to_string());
        }
        
        // Try to find market data in the universe
        let market_items = player.current_system.id.clone();
        
        let items = Universe::new().get_market_items_for_system(market_items);
        
        if item_index >= items.len() {
            return Some("Invalid item selection".to_string());
        }
        
        let (item, _quantity_available) = &items[item_index];
        let item_price = item.value;
        
        // Check if player can afford one unit
        if player.credits < item_price {
            return Some(format!("Cannot afford {} (need {} cr)", item.name, item_price));
        }
        
        // Check if player has cargo space
        if player.inventory.remaining_capacity() < item.weight {
            return Some(format!("Not enough cargo space for {}", item.name));
        }
        
        // Buy one unit of the item
        player.remove_credits(item_price);
        player.inventory.add_item(item.clone(), 1);
        
        Some(format!("Purchased 1 {} for {} cr", item.name, item_price))
    }

    pub fn sell_item(&mut self, player: &mut Player, item_index: usize) -> Option<String> {
        // Check if player is docked
        if !player.is_docked {
            return Some("You must be docked at a station to trade".to_string());
        }
        
        // Get player's inventory items
        let inventory_items: Vec<(Item, u32)> = player.inventory.items.iter()
            .map(|(item, qty)| (item.clone(), *qty))
            .collect();
        
        if item_index >= inventory_items.len() {
            return Some("Invalid item selection".to_string());
        }
        
        let (item, _) = &inventory_items[item_index];
        
        // Calculate sell price (90% of value)
        let sell_price = (item.value as f32 * 0.9) as u32;
        
        // Sell one unit of the item
        if let Some(item) = player.inventory.remove_item(&item.name, 1) {
            player.add_credits(sell_price);
            return Some(format!("Sold 1 {} for {} cr", item.name, sell_price));
        }
        
        Some("Error selling item".to_string())
    }

    pub fn update(&mut self, universe: &mut Universe, delta_time: Duration) -> Vec<TradeOrder> {
        // Update market prices periodically
        self.last_transaction += delta_time;
        let mut executed_orders = Vec::new();
        
        // For testing purposes, update more frequently (every 5 seconds)
        if self.last_transaction > Duration::from_secs(5) {
            universe.update_market_prices();
            self.last_transaction = Duration::from_secs(0);
            
            // Process all active orders in every market after updating prices
            executed_orders = self.process_all_orders(universe);
        }
        
        executed_orders
    }
    
    // Process all active orders in all markets
    fn process_all_orders(&mut self, universe: &mut Universe) -> Vec<TradeOrder> {
        // Get all systems with markets
        let system_ids = universe.get_all_system_ids();
        let mut executed_orders: Vec<TradeOrder> = Vec::new();
        
        // Process orders for each market
        for system_id in &system_ids {
            if let Some(mut market) = universe.get_market_mut(system_id) {
                // In a real implementation, we'd get the player's inventory and credits
                // For now, this is a placeholder that won't actually execute orders
                // But would process and check them
                
                // TODO: In the full implementation, get player inventory and credits
                // let executed = market.process_orders(&mut player.inventory.items, &mut player.credits);
                // executed_orders.extend(executed);
                
                // Store orders that need updating
                let mut orders_to_update = Vec::new();
                
                // First, check for orders that meet price conditions
                for (index, order) in market.trade_orders.iter().enumerate() {
                    if order.status != OrderStatus::Active {
                        continue;
                    }
                    
                    // Check if market has this item
                    if let Some(market_item) = market.items.get(&order.item_name) {
                        let current_price = market_item.current_price;
                        let current_time = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or(Duration::from_secs(0))
                            .as_secs();
                        
                        // Check price conditions
                        match order.order_type {
                            OrderType::Buy => {
                                if current_price <= order.target_price {
                                    // In a real implementation with player context:
                                    // 1. Check if player has enough credits
                                    // 2. Execute the buy transaction
                                    
                                    // For now, we'll just mark it as a completed order
                                    orders_to_update.push((index, OrderStatus::Completed, Some(current_time)));
                                    
                                    // Track the completed order
                                    executed_orders.push(order.clone());
                                }
                            },
                            OrderType::Sell => {
                                if current_price >= order.target_price {
                                    // In a real implementation with player context:
                                    // 1. Check if player still has the items
                                    // 2. Execute the sell transaction
                                    
                                    // For now, we'll just mark it as a completed order
                                    orders_to_update.push((index, OrderStatus::Completed, Some(current_time)));
                                    
                                    // Track the completed order
                                    executed_orders.push(order.clone());
                                }
                            }
                        }
                    }
                }
                
                // Now update the order statuses
                for (index, status, executed_at) in orders_to_update {
                    if let Some(order) = market.trade_orders.get_mut(index) {
                        order.status = status;
                        order.executed_at = executed_at;
                    }
                }
            }
        }
        
        // Log info about executed orders for debugging
        if !executed_orders.is_empty() {
            println!("\n===== TRADE ORDER EXECUTION REPORT =====");
            for order in &executed_orders {
                match order.order_type {
                    OrderType::Buy => {
                        println!("✅ AUTO-EXECUTED: Buy order for {} {} at price {} has been completed", 
                            order.quantity, order.item_name, order.target_price);
                    },
                    OrderType::Sell => {
                        println!("✅ AUTO-EXECUTED: Sell order for {} {} at price {} has been completed", 
                            order.quantity, order.item_name, order.target_price);
                    }
                }
            }
            println!("=======================================\n");
        }
        
        // Return executed orders so the game can display notifications
        executed_orders
    }
    
    // Get price trend information about a specific item
    pub fn get_price_trend_info(&self, item_name: &str) -> Option<(f32, String)> {
        // Forward to the universe's market system
        // Note: In a real-world implementation, this would access the market
        // associated with the current system from the universe
        
        // This is a placeholder implementation
        match item_name {
            "Iron" => Some((5.2, "Rising".to_string())),
            "Copper" => Some((-3.1, "Decreasing".to_string())),
            "Silver" => Some((12.8, "Skyrocketing".to_string())),
            "Gold" => Some((-8.5, "Falling".to_string())),
            "Titanium" => Some((0.2, "Stable".to_string())),
            "Water" => Some((-15.3, "Plummeting".to_string())),
            "Hydrogen" => Some((2.1, "Increasing".to_string())),
            "Oxygen" => Some((1.8, "Increasing".to_string())),
            "Power Cell" => Some((7.4, "Rising".to_string())),
            "Computer Chip" => Some((9.2, "Rising".to_string())),
            "Fusion Core" => Some((-2.5, "Decreasing".to_string())),
            "Shield Generator" => Some((14.3, "Skyrocketing".to_string())),
            "Warp Drive" => Some((-7.1, "Falling".to_string())),
            "Medical Supplies" => Some((3.9, "Increasing".to_string())),
            "Luxury Goods" => Some((6.3, "Rising".to_string())),
            "Military Equipment" => Some((-4.2, "Decreasing".to_string())),
            "Ship Parts" => Some((0.8, "Stable".to_string())),
            _ => None,
        }
    }
    
    // === Order Management Methods ===
    
    // Order tab UI management
    pub fn get_order_tab_index(&self) -> usize {
        self.current_order_tab
    }
    
    pub fn next_order_tab(&mut self) {
        self.current_order_tab = (self.current_order_tab + 1) % 3;
        self.selected_order_index = None; // Reset selection when changing tabs
    }
    
    pub fn previous_order_tab(&mut self) {
        self.current_order_tab = if self.current_order_tab == 0 { 2 } else { self.current_order_tab - 1 };
        self.selected_order_index = None; // Reset selection when changing tabs
    }
    
    pub fn is_order_buy_mode(&self) -> bool {
        self.order_buy_mode
    }
    
    pub fn toggle_order_type(&mut self) {
        self.order_buy_mode = !self.order_buy_mode;
    }
    
    // Order selection
    pub fn get_selected_order_index(&self) -> Option<usize> {
        self.selected_order_index
    }
    
    pub fn select_order(&mut self, index: usize) {
        self.selected_order_index = Some(index);
    }
    
    pub fn deselect_order(&mut self) {
        self.selected_order_index = None;
    }
    
    pub fn select_next_order(&mut self) {
        // If there's a current selection, increment it
        if let Some(index) = self.selected_order_index {
            let next_index = index + 1;
            // Here we don't check for upper bounds - this should be done when rendering
            // orders, since the available orders can change between updates
            self.selected_order_index = Some(next_index);
        } else {
            // If nothing is selected, select the first order
            self.selected_order_index = Some(0);
        }
    }
    
    pub fn select_previous_order(&mut self) {
        // If there's a current selection, decrement it unless at index 0
        if let Some(index) = self.selected_order_index {
            if index > 0 {
                self.selected_order_index = Some(index - 1);
            }
        } else {
            // If nothing is selected, select the first order
            self.selected_order_index = Some(0);
        }
    }
    
    pub fn get_selected_order(&self) -> Option<&TradeOrder> {
        // This function returns the currently selected order if one exists
        let _index = self.selected_order_index?; // Early return None if no selection
        
        // We'll need to get active orders from the current system
        // Since this is a placeholder implementation that doesn't have direct
        // access to the player and universe, we'll return None for now
        // 
        // In a real implementation, we would:
        // 1. Get all active orders from the market
        // 2. Filter for orders from the current player
        // 3. Return the order at the selected index if it exists
        None
    }
    
    pub fn cancel_selected_order(&mut self, player: &mut Player) -> Result<(), String> {
        // If there's a selected order, try to cancel it
        if let Some(index) = self.selected_order_index {
            self.cancel_order(player, index)
        } else {
            Err("No order selected".to_string())
        }
    }
    
    // Order creation
    pub fn create_buy_order(&mut self, player: &mut Player, item_name: &str, 
                           quantity: u32, target_price: u32, notes: &str) -> Result<Uuid, String> {
        // Verify player is docked
        if !player.is_docked {
            return Err("You must be docked at a station to create orders".to_string());
        }
        
        // Get system market from player's current location
        let system_id = player.current_system.id.clone();
        let mut universe = Universe::new(); // This may need to change depending on your architecture
        
        // Find the market for this system
        let mut market = universe.get_market(&system_id)
            .ok_or("Cannot find market for current system")?
            .clone(); // Clone to avoid borrow issues
            
        // Check if item exists in market
        if !market.items.contains_key(item_name) {
            return Err("Item not available in this market".to_string());
        }
        
        // Create the order
        let player_id = player.id.to_string();
        let order_id = market.create_buy_order(
            &player_id, 
            item_name, 
            quantity, 
            target_price, 
            None,  // No expiration date for now
            notes
        ).ok_or("Failed to create buy order")?;
        
        // Update the market back to the universe
        universe.update_market(market);
        
        Ok(order_id)
    }
    
    pub fn create_sell_order(&mut self, player: &mut Player, item_name: &str, 
                            quantity: u32, target_price: u32, notes: &str) -> Result<Uuid, String> {
        // Verify player is docked
        if !player.is_docked {
            return Err("You must be docked at a station to create orders".to_string());
        }
        
        // Verify player has the item and enough quantity
        let has_enough = player.inventory.items.iter()
            .any(|(item, qty)| item.name == item_name && *qty >= quantity);
            
        if !has_enough {
            return Err("You don't have enough of this item".to_string());
        }
        
        // Get system market from player's current location
        let system_id = player.current_system.id.clone();
        let mut universe = Universe::new(); // This may need to change depending on your architecture
        
        // Find the market for this system
        let mut market = universe.get_market(&system_id)
            .ok_or("Cannot find market for current system")?
            .clone(); // Clone to avoid borrow issues
            
        // Create the order
        let player_id = player.id.to_string();
        let order_id = market.create_sell_order(
            &player_id, 
            item_name, 
            quantity, 
            target_price, 
            None,  // No expiration date for now
            notes
        ).ok_or("Failed to create sell order")?;
        
        // Update the market back to the universe
        universe.update_market(market);
        
        Ok(order_id)
    }
    
    // Order cancellation
    pub fn cancel_order(&mut self, player: &mut Player, order_index: usize) -> Result<(), String> {
        // Verify player is docked
        if !player.is_docked {
            return Err("You must be docked at a station to manage orders".to_string());
        }
        
        // Get active orders for the player
        let system_id = player.current_system.id.clone();
        let mut universe = Universe::new();
        
        // Get the market for this system
        let mut market = universe.get_market(&system_id)
            .ok_or("Cannot find market for current system")?
            .clone();
            
        // Get player's active orders
        let player_id = player.id.to_string();
        let active_orders: Vec<&TradeOrder> = market.trade_orders.iter()
            .filter(|order| order.player_id == player_id && order.status == OrderStatus::Active)
            .collect();
            
        // Validate order index
        if order_index >= active_orders.len() {
            return Err("Invalid order selection".to_string());
        }
        
        // Get the order ID to cancel
        let order_id = active_orders[order_index].id;
        
        // Cancel the order
        if market.cancel_order(&order_id, &player_id) {
            // Update the market back to the universe
            universe.update_market(market);
            Ok(())
        } else {
            Err("Failed to cancel order".to_string())
        }
    }
    
    // Get active orders for display
    pub fn get_active_orders(&self, player: &Player) -> Vec<TradeOrder> {
        // Get the player's system
        let system_id = player.current_system.id.clone();
        let universe = Universe::new();
        
        if let Some(market) = universe.get_market(&system_id) {
            let player_id = player.id.to_string();
            market.trade_orders.iter()
                .filter(|order| order.player_id == player_id && order.status == OrderStatus::Active)
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }
    
    // Get completed orders for display
    pub fn get_completed_orders(&self, player: &Player) -> Vec<TradeOrder> {
        // Get the player's system
        let system_id = player.current_system.id.clone();
        let universe = Universe::new();
        
        if let Some(market) = universe.get_market(&system_id) {
            let player_id = player.id.to_string();
            market.trade_orders.iter()
                .filter(|order| order.player_id == player_id && 
                      (order.status == OrderStatus::Completed || 
                       order.status == OrderStatus::Cancelled || 
                       order.status == OrderStatus::Failed ||
                       order.status == OrderStatus::Expired))
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }
}
