use std::time::Duration;
use chrono::{DateTime, Utc, TimeZone};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct TimeSystem {
    game_epoch: DateTime<Utc>,
    game_time_multiplier: f32,
    elapsed_real_time: Duration,
}

impl TimeSystem {
    pub fn new() -> Self {
        // Start the game time at 2300-01-01
        let game_epoch = Utc.with_ymd_and_hms(2300, 1, 1, 0, 0, 0).unwrap();
        
        TimeSystem {
            game_epoch,
            game_time_multiplier: 60.0,  // 1 real second = 1 minute of game time
            elapsed_real_time: Duration::from_secs(0),
        }
    }

    pub fn update(&mut self, delta_time: Duration) {
        self.elapsed_real_time += delta_time;
    }

    pub fn get_current_game_time(&self) -> DateTime<Utc> {
        let game_seconds = (self.elapsed_real_time.as_secs_f32() * self.game_time_multiplier) as i64;
        self.game_epoch + chrono::Duration::seconds(game_seconds)
    }

    pub fn get_formatted_time(&self) -> String {
        let time = self.get_current_game_time();
        time.format("%Y-%m-%d %H:%M").to_string()
    }

    pub fn set_time_multiplier(&mut self, multiplier: f32) {
        self.game_time_multiplier = multiplier;
    }

    pub fn get_time_multiplier(&self) -> f32 {
        self.game_time_multiplier
    }
}
