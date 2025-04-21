use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::time::Duration;
use crate::utils::serde::SerializableInstant;
use crate::models::ship::Ship;
use crate::models::universe::Universe;
use crate::models::account::UserAccount;

/// Market type just for network protocol
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Market {
    pub system_id: String,
    pub items: Vec<MarketItem>,
}

/// Market item for network protocol
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarketItem {
    pub name: String, 
    pub quantity: u32,
    pub current_price: u32,
}

// Constants
pub const DEFAULT_SERVER_PORT: u16 = 7890;
pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
#[allow(dead_code)]
pub const CONNECTION_TIMEOUT: Duration = Duration::from_secs(10);

/// Protocol messages between client and server
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Message {
    // Authentication and connection messages
    Connect {
        client_id: Uuid,
        username: String,
        password: Option<String>,
    },
    ConnectResponse {
        success: bool,
        message: String,
        universe: Option<Universe>,
        player_ship: Option<Ship>,
    },
    Disconnect {
        client_id: Uuid,
    },
    Heartbeat {
        client_id: Uuid,
        timestamp: SerializableInstant,
    },
    
    // Account management messages
    RegisterAccount {
        username: String,
        password: String,
        email: Option<String>,
    },
    RegisterAccountResponse {
        success: bool,
        message: String,
        account: Option<UserAccount>,
    },
    LoginAccount {
        username: String,
        password: String,
    },
    LoginAccountResponse {
        success: bool,
        message: String,
        account: Option<UserAccount>,
        characters: Vec<String>, // Character IDs
    },
    ChangePassword {
        client_id: Uuid,
        username: String,
        current_password: String,
        new_password: String,
    },
    ChangePasswordResponse {
        success: bool,
        message: String,
    },
    DeleteAccount {
        client_id: Uuid,
        username: String,
        password: String,
    },
    DeleteAccountResponse {
        success: bool,
        message: String,
    },
    
    // Character management
    CreateCharacter {
        client_id: Uuid,
        account_username: String,
        character_name: String,
        faction_type: u8,
        storyline_id: String,
    },
    CreateCharacterResponse {
        success: bool,
        message: String,
        character_id: Option<String>,
    },
    ListCharacters {
        client_id: Uuid,
        account_username: String,
    },
    ListCharactersResponse {
        success: bool,
        message: String,
        characters: Vec<(String, String)>, // (ID, Name) pairs
    },
    SelectCharacter {
        client_id: Uuid,
        account_username: String,
        character_id: String,
    },
    SelectCharacterResponse {
        success: bool,
        message: String,
        universe: Option<Universe>,
        player_ship: Option<Ship>,
    },
    
    // Game state messages
    RequestGameState {
        client_id: Uuid,
    },
    GameStateUpdate {
        universe: Universe,
        player_ship: Ship,
    },
    
    // Player action messages
    NavigationAction {
        client_id: Uuid,
        destination_system: String,
    },
    MiningAction {
        client_id: Uuid,
        resource: String,
        quantity: u32,
    },
    MarketAction {
        client_id: Uuid,
        action_type: MarketActionType,
        item_name: String,
        quantity: u32,
    },
    
    // Action responses
    ActionResponse {
        success: bool,
        message: String,
        updated_ship: Option<Ship>,
        updated_market: Option<Market>,
    },
    
    // Error messages
    Error {
        code: u32,
        message: String,
    },
}

/// Types of market actions
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MarketActionType {
    Buy,
    Sell,
}

/// Game configuration that can be changed on the server
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameConfig {
    pub time_scale: f32,
    pub starting_credits: u32,
    pub universe_seed: u64,
    pub market_volatility: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            time_scale: 1.0,
            starting_credits: 1000,
            universe_seed: 42,
            market_volatility: 0.2,
        }
    }
}