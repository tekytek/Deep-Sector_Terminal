use crate::models::ship::ShipType;
use std::time::Instant;

pub fn get_ship_art(ship_type: &ShipType) -> String {
    match ship_type {
        ShipType::Scout => r#"
         .-==-.
      .-'     `-.
     /___________\
    (  _       _  )
    | (o)     (o) |
    \___________/
      / `-----' \
     /           \
    /_.-'|___|`-._\
      ""--._.--""
        "#.to_string(),
        
        ShipType::Freighter => r#"
                 /\
                /  \
      /=========|  |============\
    //          |  |           ||
   // __________/  \__________ ||
  || |  _______    _______   | ||
  || | |       |  |       |  | ||
  || | |_______|  |_______|  | ||
  ||/_____________________ \  \||
   \_______________________ \__\
                |__|
        "#.to_string(),
        
        ShipType::Miner => r#"
            __/\__
        ___/      \___
       /             /\
      /             /  \
     /_____________/    \
    <___|_________|_____>
       |   \___/  |
       |  /| ||\  |
      _|_/_|_||_\_|_
     |_|_|_|==|_|_|_|
        "#.to_string(),
        
        ShipType::Fighter => r#"
               ^
              / \
             /___\
            |=   =|
         ___/|   |\___
        /    |___|    \
       /|    |___|    |\
      / |  ,/"""""'\. | \
     /__|/'         `\|__\
        /             \
       {_______________}
        "#.to_string(),
    }
}

pub fn get_station_art() -> String {
    r#"
           /\
          /  \
         / __ \
        / /  \ \
       / / /\ \ \
      /_/_/__\_\_\
     |  __  __  __|
     | |  ||  || |
     | |__||__|| |
     |____________|
     |   |    |   |
     |___[]__[]___|
      ||        ||
      ||[]    []||
     /||        ||\
    / ||[]    []|| \
   /__|          |__\
  /   |__________|   \
    "#.to_string()
}

pub fn get_title_art() -> String {
    r#"
 █████╗ ███████╗████████╗██████╗  █████╗     ████████╗██████╗  █████╗ ██████╗ ███████╗██████╗ 
██╔══██╗██╔════╝╚══██╔══╝██╔══██╗██╔══██╗    ╚══██╔══╝██╔══██╗██╔══██╗██╔══██╗██╔════╝██╔══██╗
███████║███████╗   ██║   ██████╔╝███████║       ██║   ██████╔╝███████║██║  ██║█████╗  ██████╔╝
██╔══██║╚════██║   ██║   ██╔══██╗██╔══██║       ██║   ██╔══██╗██╔══██║██║  ██║██╔══╝  ██╔══██╗
██║  ██║███████║   ██║   ██║  ██║██║  ██║       ██║   ██║  ██║██║  ██║██████╔╝███████╗██║  ██║
╚═╝  ╚═╝╚══════╝   ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═╝       ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═╝╚═════╝ ╚══════╝╚═╝  ╚═╝
"#.to_string()
}

// Animated stars for background
pub fn get_star_field(current_time: &Instant, frame_num: u64) -> Vec<(u16, u16, char)> {
    let mut stars = Vec::new();
    
    // Use time to create animation frames by shifting stars
    let elapsed = current_time.elapsed().as_millis() as u64;
    let shift = (elapsed / 200) % 3; // Move stars every 200ms
    
    // Different star types
    let star_types = ['·', '★', '☆', '.', '*', '+'];
    
    // Create star field based on deterministic pattern but with animation
    for i in 0..100 {
        let seed = (i * 13 + frame_num) % 997; // Prime number for pseudo-randomness
        let x = ((seed * 7) % 80) as u16;
        let y = ((seed * 11) % 24) as u16;
        
        // Skip stars in the center where the menu will be
        if (10..70).contains(&x) && (5..20).contains(&y) {
            continue;
        }
        
        // Choose star type based on position and time
        let star_type = star_types[((x as u64 + y as u64 + shift) % star_types.len() as u64) as usize];
        
        stars.push((x, y, star_type));
    }
    
    stars
}

// Dynamic engine animation for ships
pub fn get_engine_particles(current_time: &Instant) -> Vec<(u16, u16, char)> {
    let mut particles = Vec::new();
    let elapsed = current_time.elapsed().as_millis() as u64;
    
    // Generate particles based on time
    for i in 0..5 {
        let seed = (i * 17 + elapsed) % 1009;
        // Cast to i16 first to safely handle the subtraction, then cast to u16
        let x_offset = (seed % 5) as i16 - 2;
        let x = 40u16.saturating_add_signed(x_offset);
        let y = 22 + (seed % 2) as u16;
        
        // Different particle characters for engine trail
        let particles_chars = ['░', '▒', '▓', '█', '▄', '▀'];
        let particle_char = particles_chars[(elapsed / 100 + i) as usize % particles_chars.len()];
        
        particles.push((x, y, particle_char));
    }
    
    particles
}
