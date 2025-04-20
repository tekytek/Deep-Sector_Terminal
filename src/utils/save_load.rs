use std::fs::{File, create_dir_all};
use std::io::{Read, Write};
use std::path::Path;
use std::error::Error;

use serde_json;
use crate::game::Game;

const SAVE_DIR: &str = "./saves";
const SAVE_FILE: &str = "space_trader_save.json";

pub fn save_game(game: &Game) -> Result<(), Box<dyn Error>> {
    // Create save directory if it doesn't exist
    create_dir_all(SAVE_DIR)?;
    
    let save_path = Path::new(SAVE_DIR).join(SAVE_FILE);
    let mut file = File::create(save_path)?;
    
    // Serialize game state
    let serialized = serde_json::to_string(game)?;
    
    // Write to file
    file.write_all(serialized.as_bytes())?;
    
    Ok(())
}

pub fn load_game() -> Result<Game, Box<dyn Error>> {
    let save_path = Path::new(SAVE_DIR).join(SAVE_FILE);
    
    // Check if save file exists
    if !save_path.exists() {
        return Err("No save file found".into());
    }
    
    // Open and read the file
    let mut file = File::open(save_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    // Deserialize game state
    let game: Game = serde_json::from_str(&contents)?;
    
    Ok(game)
}
