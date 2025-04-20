use std::fs::File;
use std::io::{self, Read, Write};
use std::error::Error;
use std::path::Path;
use serde::{Serialize, Deserialize};

const SAVE_FILE: &str = "savegame.json";

pub fn save_game<T: Serialize>(game: &T) -> Result<(), Box<dyn Error>> {
    // Serialize game state to JSON
    let serialized = serde_json::to_string_pretty(game)?;
    
    // Write to file
    let mut file = File::create(SAVE_FILE)?;
    file.write_all(serialized.as_bytes())?;
    
    Ok(())
}

pub fn load_game<T>() -> Result<T, Box<dyn Error>> 
where
    T: for<'de> Deserialize<'de>,
{
    // Check if save file exists
    if !Path::new(SAVE_FILE).exists() {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::NotFound,
            "Save file not found"
        )));
    }
    
    // Read file content
    let mut file = File::open(SAVE_FILE)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    
    // Deserialize from JSON
    let deserialized = serde_json::from_str(&content)?;
    
    Ok(deserialized)
}