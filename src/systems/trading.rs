use std::time::Duration;
use serde::{Serialize, Deserialize};

use crate::models::player::Player;
use crate::models::universe::Universe;
use crate::models::item::Item;

#[derive(Serialize, Deserialize)]
pub struct TradingSystem {
    buy_mode: bool,
    transaction_cooldown: Duration,
    last_transaction: Duration,
}

impl TradingSystem {
    pub fn new() -> Self {
        TradingSystem {
            buy_mode: true,
            transaction_cooldown: Duration::from_millis(500),
            last_transaction: Duration::from_secs(0),
        }
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

    pub fn update(&mut self, universe: &mut Universe, delta_time: Duration) {
        // Update market prices periodically
        self.last_transaction += delta_time;
        
        if self.last_transaction > Duration::from_secs(60) {
            universe.update_market_prices();
            self.last_transaction = Duration::from_secs(0);
        }
    }
}
