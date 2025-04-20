use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize, Serializer, Deserializer};

// Module for serializing and deserializing Instant
pub mod instant_serde {
    use super::*;
    
    // Store instants as durations since the UNIX epoch
    pub fn serialize<S>(instant: &Instant, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let duration = instant.elapsed();
        duration.as_secs().serialize(serializer)
    }
    
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Instant, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = u64::deserialize(deserializer)?;
        Ok(Instant::now() - Duration::from_secs(secs))
    }
}

// Module for serializing and deserializing Option<Instant>
pub mod option_instant_serde {
    use super::*;
    
    pub fn serialize<S>(opt_instant: &Option<Instant>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match opt_instant {
            Some(instant) => {
                let duration = instant.elapsed();
                Some(duration.as_secs()).serialize(serializer)
            }
            None => None::<u64>.serialize(serializer),
        }
    }
    
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Instant>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt_secs = Option::<u64>::deserialize(deserializer)?;
        match opt_secs {
            Some(secs) => Ok(Some(Instant::now() - Duration::from_secs(secs))),
            None => Ok(None),
        }
    }
}