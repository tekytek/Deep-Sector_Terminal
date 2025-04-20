use crate::models::ship::ShipType;

pub fn get_ship_art(ship_type: &ShipType) -> String {
    match ship_type {
        ShipType::Scout => r#"
      .--.
     /____\
     |    |
   __|____|__
  /____/\____\
       ||
       ||
       ||
    "mwwwwmwmw"m
        "#.to_string(),
        
        ShipType::Freighter => r#"
        __
        \ \_____
     ###[==_____>
        /_/
        "#.to_string(),
        
        ShipType::Miner => r#"
         ___
        / _ \
      .=\_()/=.
     /___||___\
        //\\
       //  \\
      ||    ||
      ||    ||
     //      \\
    "#.to_string(),
        
        ShipType::Fighter => r#"
         __
        /  \
       /    \
      /      \
     |--------|
     \___/\___/
        ||
      __||__
     /______\
        "#.to_string(),
    }
}

pub fn get_station_art() -> String {
    r#"
       /\
      /  \
     |    |
    /|    |\
   / |----| \
  /__|____|__\
 |____________|
 |            |
 |   []  []   |
 |   []  []   |
 |____________|
    "#.to_string()
}
