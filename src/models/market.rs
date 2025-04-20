use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::models::item::Item;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketItem {
    pub item: Item,
    pub quantity: u32,
    pub base_price: u32,
    pub price_volatility: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    pub system_id: String,
    pub items: HashMap<String, MarketItem>,
    pub last_update: u64,
}

impl Market {
    pub fn new(system_id: &str) -> Self {
        Market {
            system_id: system_id.to_string(),
            items: HashMap::new(),
            last_update: 0,
        }
    }

    pub fn add_item(&mut self, item: Item, quantity: u32, base_price: u32, volatility: f32) {
        let market_item = MarketItem {
            item: item.clone(),
            quantity,
            base_price,
            price_volatility: volatility,
        };
        
        self.items.insert(item.name.clone(), market_item);
    }

    pub fn buy_item(&mut self, item_name: &str, quantity: u32) -> Option<(Item, u32, u32)> {
        if let Some(market_item) = self.items.get_mut(item_name) {
            if market_item.quantity >= quantity {
                market_item.quantity -= quantity;
                let cost = market_item.base_price * quantity;
                let item = market_item.item.clone();
                return Some((item, quantity, cost));
            }
        }
        None
    }

    pub fn sell_item(&mut self, item: Item, quantity: u32) -> u32 {
        let sell_price = (item.value as f32 * 0.9) as u32;  // Sell at 90% of value
        let revenue = sell_price * quantity;
        
        if let Some(market_item) = self.items.get_mut(&item.name) {
            market_item.quantity += quantity;
        } else {
            // Add new item to market if it wasn't there before
            self.add_item(item.clone(), quantity, sell_price, 0.1);
        }
        
        revenue
    }

    pub fn update_prices(&mut self, game_time: u64) {
        self.last_update = game_time;
        
        // Update each item's price based on volatility and time passed
        for market_item in self.items.values_mut() {
            let fluctuation = rand::random::<f32>() * market_item.price_volatility * 2.0 - market_item.price_volatility;
            let new_price = (market_item.base_price as f32 * (1.0 + fluctuation)) as u32;
            market_item.item.value = new_price;
        }
    }
}
