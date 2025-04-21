use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::models::item::{Item, ItemType};
use crate::models::market::OrderStatus;

/// Represents a single market listing created by a player
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerMarketListing {
    pub id: String,                     // Unique identifier for the listing
    pub seller_id: String,              // ID of the player selling the item
    pub seller_name: String,            // Display name of the seller
    pub item: Item,                     // The item being sold
    pub quantity: u32,                  // Quantity available
    pub price_per_unit: u32,            // Price per single unit
    pub total_price: u32,               // Total price for all units
    pub system_id: String,              // Star system where the listing is available
    pub location_id: String,            // Specific location within the system
    pub created_at: u64,                // When the listing was created
    pub expires_at: Option<u64>,        // When the listing expires (if applicable)
    pub minimum_reputation: i32,        // Minimum reputation needed to purchase
    pub visibility: ListingVisibility,  // Who can see this listing
    pub negotiable: bool,               // Whether price is negotiable
    pub description: String,            // Optional description
    pub tags: Vec<String>,              // Tags for filtering/searching
    pub quantity_sold: u32,             // Track how many units have been sold so far
}

/// Determines who can see a listing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ListingVisibility {
    Public,             // Anyone can see it
    FactionOnly(String), // Only visible to members of a specific faction
    PlayerList(Vec<String>), // Only visible to specific players
}

/// Bid placed on a negotiable listing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MarketBid {
    pub id: String,               // Unique ID for this bid
    pub listing_id: String,       // The listing being bid on
    pub bidder_id: String,        // ID of the player making the bid
    pub bidder_name: String,      // Name of the player making the bid
    pub bid_amount: u32,          // Amount bid per unit
    pub quantity: u32,            // Quantity requested
    pub total_amount: u32,        // Total bid (bid_amount * quantity)
    pub message: String,          // Optional message to seller
    pub status: BidStatus,        // Current status of this bid
    pub created_at: u64,          // When bid was placed
    pub expires_at: Option<u64>,  // When bid expires
}

/// Status of a market bid
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BidStatus {
    Pending,    // Waiting for seller response
    Accepted,   // Bid accepted
    Rejected,   // Bid rejected
    Expired,    // Bid expired
    Canceled,   // Bid canceled by bidder
}

/// Purchase record for tracking sales and purchase history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPurchase {
    pub id: String,               // Unique transaction ID
    pub listing_id: String,       // Original listing ID
    pub buyer_id: String,         // ID of buyer
    pub seller_id: String,        // ID of seller
    pub item_name: String,        // Name of item purchased
    pub quantity: u32,            // Quantity purchased
    pub price_per_unit: u32,      // Price per unit paid
    pub total_price: u32,         // Total transaction amount
    pub timestamp: u64,           // When purchase occurred
    pub system_id: String,        // Where transaction occurred
    pub was_negotiated: bool,     // Whether price was negotiated
}

/// Contract between players for large orders, collaborations, etc
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketContract {
    pub id: String,                      // Unique contract ID
    pub issuer_id: String,               // Player creating the contract
    pub assignee_ids: Vec<String>,       // Players assigned to fulfill contract
    pub title: String,                   // Contract title
    pub description: String,             // Detailed description
    pub items_required: Vec<(Item, u32)>, // Items and quantities needed
    pub reward_credits: u32,             // Credit reward upon completion
    pub reward_items: Vec<(Item, u32)>,  // Item rewards upon completion
    pub deadline: Option<u64>,           // When contract expires
    pub created_at: u64,                 // When contract was created
    pub status: ContractStatus,          // Current status
    pub progress: Vec<ContractProgress>, // Progress updates
    pub terms: Vec<String>,              // Additional contract terms
    pub is_public: bool,                 // Whether contract is publicly visible
}

/// Status of a market contract
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContractStatus {
    Open,               // Available for acceptance
    InProgress,         // Currently being worked on
    Completed,          // Successfully completed
    Failed,             // Failed to complete
    Canceled,           // Canceled by issuer
    Disputed,           // Under dispute
}

/// Records progress updates on contracts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractProgress {
    pub timestamp: u64,
    pub update_by: String,    // Player ID who updated
    pub message: String,
    pub items_delivered: Vec<(String, u32)>, // Item name and quantity
}

/// Represents the entire player-driven marketplace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerMarket {
    pub listings: HashMap<String, PlayerMarketListing>,
    pub bids: HashMap<String, MarketBid>,
    pub purchase_history: Vec<MarketPurchase>,
    pub contracts: HashMap<String, MarketContract>,
    pub market_fee: f32,         // Transaction fee percentage (e.g., 0.05 for 5%)
    pub price_trends: HashMap<String, Vec<PriceTrend>>, // Track price trends by item
    pub reputation_requirements: bool, // Whether to enforce reputation requirements
}

/// Records price trend data for market analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceTrend {
    pub item_name: String,
    pub average_price: u32,
    pub lowest_price: u32,
    pub highest_price: u32,
    pub volume_traded: u32,
    pub timestamp: u64,
}

impl PlayerMarket {
    pub fn new() -> Self {
        PlayerMarket {
            listings: HashMap::new(),
            bids: HashMap::new(),
            purchase_history: Vec::new(),
            contracts: HashMap::new(),
            market_fee: 0.05,  // 5% default fee
            price_trends: HashMap::new(),
            reputation_requirements: true,
        }
    }

    /// Create a new listing in the player market
    pub fn create_listing(
        &mut self,
        seller_id: &str,
        seller_name: &str,
        item: Item,
        quantity: u32,
        price_per_unit: u32,
        system_id: &str,
        location_id: &str,
        expiration: Option<u64>,
        min_reputation: i32,
        visibility: ListingVisibility,
        negotiable: bool,
        description: &str,
        tags: Vec<String>,
    ) -> String {
        let id = Uuid::new_v4().to_string();
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();

        let listing = PlayerMarketListing {
            id: id.clone(),
            seller_id: seller_id.to_string(),
            seller_name: seller_name.to_string(),
            item,
            quantity,
            price_per_unit,
            total_price: price_per_unit * quantity,
            system_id: system_id.to_string(),
            location_id: location_id.to_string(),
            created_at: current_time,
            expires_at: expiration,
            minimum_reputation: min_reputation,
            visibility,
            negotiable,
            description: description.to_string(),
            tags,
            quantity_sold: 0,
        };

        self.listings.insert(id.clone(), listing);
        
        // Return the listing ID
        id
    }

    /// Find listings matching search criteria
    pub fn search_listings(
        &self,
        player_id: &str,
        player_faction: Option<&str>,
        item_name: Option<&str>,
        item_type: Option<&ItemType>,
        system_id: Option<&str>,
        tags: Option<&[String]>,
        min_price: Option<u32>,
        max_price: Option<u32>,
        sort_by: ListingSortOption,
    ) -> Vec<&PlayerMarketListing> {
        let mut results: Vec<&PlayerMarketListing> = self.listings.values()
            .filter(|listing| {
                // Filter by visibility first
                match &listing.visibility {
                    ListingVisibility::Public => true,
                    ListingVisibility::FactionOnly(faction) => {
                        if let Some(player_faction) = player_faction {
                            player_faction == faction
                        } else {
                            false
                        }
                    },
                    ListingVisibility::PlayerList(players) => {
                        players.contains(&player_id.to_string())
                    }
                } &&
                // Then apply other filters
                (item_name.is_none() || listing.item.name.contains(item_name.unwrap())) &&
                (item_type.is_none() || &listing.item.item_type == item_type.unwrap()) &&
                (system_id.is_none() || listing.system_id == system_id.unwrap()) &&
                (min_price.is_none() || listing.price_per_unit >= min_price.unwrap()) &&
                (max_price.is_none() || listing.price_per_unit <= max_price.unwrap()) &&
                (tags.is_none() || tags.unwrap().iter().any(|tag| listing.tags.contains(tag)))
            })
            .collect();

        // Sort results based on sort option
        match sort_by {
            ListingSortOption::PriceAscending => {
                results.sort_by(|a, b| a.price_per_unit.cmp(&b.price_per_unit));
            },
            ListingSortOption::PriceDescending => {
                results.sort_by(|a, b| b.price_per_unit.cmp(&a.price_per_unit));
            },
            ListingSortOption::NewestFirst => {
                results.sort_by(|a, b| b.created_at.cmp(&a.created_at));
            },
            ListingSortOption::OldestFirst => {
                results.sort_by(|a, b| a.created_at.cmp(&b.created_at));
            },
            ListingSortOption::QuantityAscending => {
                results.sort_by(|a, b| a.quantity.cmp(&b.quantity));
            },
            ListingSortOption::QuantityDescending => {
                results.sort_by(|a, b| b.quantity.cmp(&a.quantity));
            },
        }

        results
    }

    /// Purchase from a listing directly
    pub fn purchase_listing(
        &mut self,
        listing_id: &str, 
        buyer_id: &str,
        quantity: u32,
        buyer_reputation: i32,
    ) -> Result<MarketPurchase, String> {
        // Get the listing
        let listing = match self.listings.get_mut(listing_id) {
            Some(l) => l,
            None => return Err("Listing not found".to_string()),
        };

        // Check quantity
        if listing.quantity < quantity {
            return Err(format!("Not enough quantity available. Requested: {}, Available: {}", 
                              quantity, listing.quantity));
        }

        // Check reputation requirement
        if self.reputation_requirements && buyer_reputation < listing.minimum_reputation {
            return Err(format!("Insufficient reputation. Required: {}, You have: {}", 
                              listing.minimum_reputation, buyer_reputation));
        }

        // Check if negotiable - if yes, direct purchase may not be allowed
        if listing.negotiable {
            // For this implementation, we'll allow direct purchase even for negotiable items
            // In a full implementation, you might want to enforce bidding for negotiable items
        }

        // Calculate total price
        let price_per_unit = listing.price_per_unit;
        let total_price = price_per_unit * quantity;
        
        // Create purchase record
        let purchase_id = Uuid::new_v4().to_string();
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
            
        let purchase = MarketPurchase {
            id: purchase_id,
            listing_id: listing_id.to_string(),
            buyer_id: buyer_id.to_string(),
            seller_id: listing.seller_id.clone(),
            item_name: listing.item.name.clone(),
            quantity,
            price_per_unit,
            total_price,
            timestamp: current_time,
            system_id: listing.system_id.clone(),
            was_negotiated: false,
        };
        
        // Update the listing quantity
        listing.quantity -= quantity;
        listing.quantity_sold += quantity;
        
        // Remove listing if sold out
        if listing.quantity == 0 {
            self.listings.remove(listing_id);
        }
        
        // Update price trend data
        self.update_price_trend(&listing.item.name, price_per_unit, quantity);
        
        // Add to purchase history
        self.purchase_history.push(purchase.clone());
        
        Ok(purchase)
    }

    /// Place a bid on a negotiable listing
    pub fn place_bid(
        &mut self,
        listing_id: &str,
        bidder_id: &str,
        bidder_name: &str,
        bid_amount: u32,
        quantity: u32,
        message: &str,
        expires_in_hours: Option<u64>,
    ) -> Result<String, String> {
        // Check if listing exists and is negotiable
        let listing = match self.listings.get(listing_id) {
            Some(l) => {
                if !l.negotiable {
                    return Err("This listing does not accept bids".to_string());
                }
                if l.quantity < quantity {
                    return Err("Requested quantity exceeds available quantity".to_string());
                }
                l
            },
            None => return Err("Listing not found".to_string()),
        };
        
        // Create the bid
        let bid_id = Uuid::new_v4().to_string();
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
            
        // Calculate expiration time
        let expires_at = expires_in_hours.map(|hours| {
            current_time + (hours * 3600) // hours to seconds
        });
        
        let bid = MarketBid {
            id: bid_id.clone(),
            listing_id: listing_id.to_string(),
            bidder_id: bidder_id.to_string(),
            bidder_name: bidder_name.to_string(),
            bid_amount,
            quantity,
            total_amount: bid_amount * quantity,
            message: message.to_string(),
            status: BidStatus::Pending,
            created_at: current_time,
            expires_at,
        };
        
        // Store the bid
        self.bids.insert(bid_id.clone(), bid);
        
        Ok(bid_id)
    }

    /// Accept a bid on a listing
    pub fn accept_bid(&mut self, bid_id: &str) -> Result<MarketPurchase, String> {
        // Get the bid
        let bid = match self.bids.get_mut(bid_id) {
            Some(b) => {
                // Check if bid is still pending
                if b.status != BidStatus::Pending {
                    return Err(format!("Bid cannot be accepted: current status is {:?}", b.status));
                }
                
                // Mark bid as accepted
                b.status = BidStatus::Accepted;
                b.clone()
            },
            None => return Err("Bid not found".to_string()),
        };
        
        // Get the listing
        let listing = match self.listings.get_mut(&bid.listing_id) {
            Some(l) => {
                // Check if listing still has enough quantity
                if l.quantity < bid.quantity {
                    // Revert bid status to pending
                    if let Some(b) = self.bids.get_mut(bid_id) {
                        b.status = BidStatus::Pending;
                    }
                    return Err("Listing no longer has sufficient quantity available".to_string());
                }
                l
            },
            None => {
                // If listing is gone, mark bid as failed
                if let Some(b) = self.bids.get_mut(bid_id) {
                    b.status = BidStatus::Rejected;
                }
                return Err("Listing no longer exists".to_string());
            },
        };
        
        // Process the purchase
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
            
        // Create the purchase record
        let purchase_id = Uuid::new_v4().to_string();
        let purchase = MarketPurchase {
            id: purchase_id,
            listing_id: bid.listing_id.clone(),
            buyer_id: bid.bidder_id.clone(),
            seller_id: listing.seller_id.clone(),
            item_name: listing.item.name.clone(),
            quantity: bid.quantity,
            price_per_unit: bid.bid_amount,
            total_price: bid.total_amount,
            timestamp: current_time,
            system_id: listing.system_id.clone(),
            was_negotiated: true,
        };
        
        // Update the listing quantity
        listing.quantity -= bid.quantity;
        listing.quantity_sold += bid.quantity;
        
        // Remove listing if sold out
        if listing.quantity == 0 {
            self.listings.remove(&bid.listing_id);
        }
        
        // Update price trend data
        self.update_price_trend(&listing.item.name, bid.bid_amount, bid.quantity);
        
        // Add to purchase history
        self.purchase_history.push(purchase.clone());
        
        Ok(purchase)
    }

    /// Reject a bid
    pub fn reject_bid(&mut self, bid_id: &str) -> Result<(), String> {
        // Get the bid and update status
        match self.bids.get_mut(bid_id) {
            Some(bid) => {
                if bid.status != BidStatus::Pending {
                    return Err(format!("Cannot reject bid with status: {:?}", bid.status));
                }
                bid.status = BidStatus::Rejected;
                Ok(())
            },
            None => Err("Bid not found".to_string()),
        }
    }

    /// Cancel own bid (as a bidder)
    pub fn cancel_bid(&mut self, bid_id: &str, bidder_id: &str) -> Result<(), String> {
        // Get the bid and update status
        match self.bids.get_mut(bid_id) {
            Some(bid) => {
                // Check if bidder owns this bid
                if bid.bidder_id != bidder_id {
                    return Err("You can only cancel your own bids".to_string());
                }
                
                if bid.status != BidStatus::Pending {
                    return Err(format!("Cannot cancel bid with status: {:?}", bid.status));
                }
                
                bid.status = BidStatus::Canceled;
                Ok(())
            },
            None => Err("Bid not found".to_string()),
        }
    }

    /// Create a new contract
    pub fn create_contract(
        &mut self,
        issuer_id: &str,
        title: &str,
        description: &str,
        items_required: Vec<(Item, u32)>,
        reward_credits: u32,
        reward_items: Vec<(Item, u32)>,
        deadline: Option<u64>,
        terms: Vec<String>,
        is_public: bool,
    ) -> String {
        let contract_id = Uuid::new_v4().to_string();
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
            
        let contract = MarketContract {
            id: contract_id.clone(),
            issuer_id: issuer_id.to_string(),
            assignee_ids: Vec::new(), // No assignees initially
            title: title.to_string(),
            description: description.to_string(),
            items_required,
            reward_credits,
            reward_items,
            deadline,
            created_at: current_time,
            status: ContractStatus::Open,
            progress: Vec::new(),
            terms,
            is_public,
        };
        
        self.contracts.insert(contract_id.clone(), contract);
        
        contract_id
    }

    /// Accept a contract
    pub fn accept_contract(&mut self, contract_id: &str, player_id: &str) -> Result<(), String> {
        match self.contracts.get_mut(contract_id) {
            Some(contract) => {
                if contract.status != ContractStatus::Open {
                    return Err(format!("Contract is not open for acceptance: {:?}", contract.status));
                }
                
                // Add player to assignees
                if !contract.assignee_ids.contains(&player_id.to_string()) {
                    contract.assignee_ids.push(player_id.to_string());
                }
                
                // Update status
                contract.status = ContractStatus::InProgress;
                
                // Record progress
                let current_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or(Duration::from_secs(0))
                    .as_secs();
                    
                contract.progress.push(ContractProgress {
                    timestamp: current_time,
                    update_by: player_id.to_string(),
                    message: "Contract accepted".to_string(),
                    items_delivered: Vec::new(),
                });
                
                Ok(())
            },
            None => Err("Contract not found".to_string()),
        }
    }

    /// Update contract progress
    pub fn update_contract_progress(
        &mut self,
        contract_id: &str,
        player_id: &str,
        message: &str,
        items_delivered: Vec<(String, u32)>,
    ) -> Result<(), String> {
        match self.contracts.get_mut(contract_id) {
            Some(contract) => {
                // Check if player is an assignee
                if !contract.assignee_ids.contains(&player_id.to_string()) && 
                   contract.issuer_id != player_id {
                    return Err("You are not authorized to update this contract".to_string());
                }
                
                if contract.status != ContractStatus::InProgress {
                    return Err(format!("Cannot update contract with status: {:?}", contract.status));
                }
                
                // Record progress
                let current_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or(Duration::from_secs(0))
                    .as_secs();
                    
                contract.progress.push(ContractProgress {
                    timestamp: current_time,
                    update_by: player_id.to_string(),
                    message: message.to_string(),
                    items_delivered: items_delivered.clone(),
                });
                
                Ok(())
            },
            None => Err("Contract not found".to_string()),
        }
    }

    /// Complete a contract
    pub fn complete_contract(&mut self, contract_id: &str, issuer_id: &str) -> Result<(), String> {
        match self.contracts.get_mut(contract_id) {
            Some(contract) => {
                // Verify issuer
                if contract.issuer_id != issuer_id {
                    return Err("Only the contract issuer can mark it as complete".to_string());
                }
                
                if contract.status != ContractStatus::InProgress {
                    return Err(format!("Cannot complete contract with status: {:?}", contract.status));
                }
                
                // Update status
                contract.status = ContractStatus::Completed;
                
                // Record completion
                let current_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or(Duration::from_secs(0))
                    .as_secs();
                    
                contract.progress.push(ContractProgress {
                    timestamp: current_time,
                    update_by: issuer_id.to_string(),
                    message: "Contract marked as completed by issuer".to_string(),
                    items_delivered: Vec::new(),
                });
                
                Ok(())
            },
            None => Err("Contract not found".to_string()),
        }
    }

    /// Update price trends
    fn update_price_trend(&mut self, item_name: &str, price: u32, quantity: u32) {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
            
        // Get current trend data or create new entry
        let trends = self.price_trends.entry(item_name.to_string())
            .or_insert_with(Vec::new);
            
        // If we have existing data for this hour, update it
        let hour_timestamp = current_time - (current_time % 3600); // Round to hour
        
        if let Some(last_trend) = trends.last_mut() {
            if last_trend.timestamp == hour_timestamp {
                // Update existing trend
                let total_volume = last_trend.volume_traded + quantity;
                let weighted_avg = (last_trend.average_price as u64 * last_trend.volume_traded as u64 + 
                                   price as u64 * quantity as u64) / total_volume as u64;
                                   
                last_trend.average_price = weighted_avg as u32;
                last_trend.lowest_price = price.min(last_trend.lowest_price);
                last_trend.highest_price = price.max(last_trend.highest_price);
                last_trend.volume_traded = total_volume;
                return;
            }
        }
        
        // Create new trend data point
        let trend = PriceTrend {
            item_name: item_name.to_string(),
            average_price: price,
            lowest_price: price,
            highest_price: price,
            volume_traded: quantity,
            timestamp: hour_timestamp,
        };
        
        trends.push(trend);
        
        // Keep only the last 24 hours of trend data
        if trends.len() > 24 {
            *trends = trends.drain(trends.len() - 24..).collect();
        }
    }

    /// Process expired listings and bids
    pub fn process_expirations(&mut self) -> (Vec<String>, Vec<String>) {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
            
        let mut expired_listings = Vec::new();
        let mut expired_bids = Vec::new();
        
        // Process expired listings
        self.listings.retain(|id, listing| {
            if let Some(expires_at) = listing.expires_at {
                if current_time > expires_at {
                    expired_listings.push(id.clone());
                    return false; // Remove from listings
                }
            }
            true // Keep listing
        });
        
        // Process expired bids
        for (id, bid) in self.bids.iter_mut() {
            if bid.status == BidStatus::Pending {
                if let Some(expires_at) = bid.expires_at {
                    if current_time > expires_at {
                        bid.status = BidStatus::Expired;
                        expired_bids.push(id.clone());
                    }
                }
            }
        }
        
        (expired_listings, expired_bids)
    }

    /// Get market statistics for an item
    pub fn get_market_statistics(&self, item_name: &str) -> Option<ItemMarketStatistics> {
        let listings: Vec<&PlayerMarketListing> = self.listings.values()
            .filter(|l| l.item.name == item_name)
            .collect();
            
        if listings.is_empty() {
            return None;
        }
        
        // Calculate current listings stats
        let current_count = listings.len();
        let current_quantity: u32 = listings.iter().map(|l| l.quantity).sum();
        let current_min_price = listings.iter().map(|l| l.price_per_unit).min().unwrap_or(0);
        let current_max_price = listings.iter().map(|l| l.price_per_unit).max().unwrap_or(0);
        
        // Calculate average price weighted by quantity
        let total_value: u64 = listings.iter()
            .map(|l| l.price_per_unit as u64 * l.quantity as u64)
            .sum();
        let avg_price = if current_quantity > 0 {
            (total_value / current_quantity as u64) as u32
        } else {
            0
        };
        
        // Get price history from trends
        let price_history = self.price_trends.get(item_name).cloned();
        
        // Get recent sales from purchase history
        let recent_sales: Vec<&MarketPurchase> = self.purchase_history.iter()
            .filter(|p| p.item_name == item_name)
            .collect();
            
        let recent_sales_count = recent_sales.len();
        let recent_sales_volume: u32 = recent_sales.iter().map(|p| p.quantity).sum();
        let recent_sales_value: u32 = recent_sales.iter().map(|p| p.total_price).sum();
        
        // Calculate price change percent (from 24h ago)
        let price_change_percent = if let Some(trends) = &price_history {
            if trends.len() >= 2 {
                let newest = &trends[trends.len() - 1];
                let oldest = &trends[0];
                
                if oldest.average_price > 0 {
                    ((newest.average_price as f32 - oldest.average_price as f32) / 
                     oldest.average_price as f32 * 100.0) as i32
                } else {
                    0
                }
            } else {
                0
            }
        } else {
            0
        };
        
        Some(ItemMarketStatistics {
            item_name: item_name.to_string(),
            current_listings_count: current_count,
            current_quantity_available: current_quantity,
            current_min_price,
            current_max_price,
            current_avg_price: avg_price,
            recent_sales_count,
            recent_sales_volume,
            recent_sales_value,
            price_change_percent,
            price_history,
        })
    }
}

/// Options for sorting market listings
#[derive(Debug, Clone, Copy)]
pub enum ListingSortOption {
    PriceAscending,
    PriceDescending,
    NewestFirst,
    OldestFirst,
    QuantityAscending,
    QuantityDescending,
}

/// Market statistics for an item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemMarketStatistics {
    pub item_name: String,
    pub current_listings_count: usize,
    pub current_quantity_available: u32,
    pub current_min_price: u32,
    pub current_max_price: u32,
    pub current_avg_price: u32,
    pub recent_sales_count: usize,
    pub recent_sales_volume: u32,
    pub recent_sales_value: u32,
    pub price_change_percent: i32,
    pub price_history: Option<Vec<PriceTrend>>,
}