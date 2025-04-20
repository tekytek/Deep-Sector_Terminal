use tui::style::Color;

// Enhanced sci-fi color scheme using RGB colors for more vibrant appearance
pub const PRIMARY: Color = Color::Rgb(0, 255, 136);       // #00FF88 (neon green)
pub const SECONDARY: Color = Color::Rgb(0, 128, 255);     // #0080FF (bright blue)
pub const WARNING: Color = Color::Rgb(255, 204, 0);       // #FFCC00 (bright yellow)
pub const DANGER: Color = Color::Rgb(255, 51, 51);        // #FF3333 (bright red)
pub const INFO: Color = Color::Rgb(0, 240, 255);          // #00F0FF (bright cyan)
pub const SUCCESS: Color = Color::Rgb(0, 255, 0);         // #00FF00 (green)
pub const NORMAL: Color = Color::Rgb(220, 220, 240);      // #DCDCF0 (slightly blue-tinted white)
pub const DEFAULT_TEXT: Color = Color::Rgb(220, 220, 240); // Same as NORMAL, for text
pub const DIM: Color = Color::Rgb(100, 110, 130);         // #646E82 (slate gray)

// Animation colors
pub const HIGHLIGHT: Color = Color::Rgb(255, 255, 255);   // #FFFFFF (pure white) 
#[allow(dead_code)]
pub const ENERGY: Color = Color::Rgb(130, 60, 255);       // #823CFF (purple)
#[allow(dead_code)]
pub const SHIELD: Color = Color::Rgb(60, 170, 255);       // #3CAAFF (shield blue)
#[allow(dead_code)]
pub const HULL: Color = Color::Rgb(180, 180, 180);        // #B4B4B4 (hull gray)
#[allow(dead_code)]
pub const STARS_BG: Color = Color::Rgb(5, 10, 25);        // #050A19 (deep space blue)
