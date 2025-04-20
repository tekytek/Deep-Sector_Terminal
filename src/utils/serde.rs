use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::time::{Duration, Instant};

// A wrapper for serializing and deserializing Instant
#[derive(Clone, Copy, Debug)]
pub struct SerializableInstant(Instant);

impl From<Instant> for SerializableInstant {
    fn from(instant: Instant) -> Self {
        SerializableInstant(instant)
    }
}

impl From<SerializableInstant> for Instant {
    fn from(serializable: SerializableInstant) -> Self {
        serializable.0
    }
}

impl Serialize for SerializableInstant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Convert to duration since UNIX_EPOCH
        let duration_since_epoch = self.0.elapsed();
        let secs = duration_since_epoch.as_secs();
        let nanos = duration_since_epoch.subsec_nanos();
        (secs, nanos).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SerializableInstant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (secs, nanos): (u64, u32) = Deserialize::deserialize(deserializer)?;
        let duration = Duration::new(secs, nanos);
        let now = Instant::now();
        Ok(SerializableInstant(now - duration))
    }
}

// Serialization modules for game.rs

/// Module for serializing/deserializing Instant
pub mod instant_serde {
    use super::*;
    use serde::{Serializer, Deserializer};
    use std::time::Instant;

    pub fn serialize<S>(instant: &Instant, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let serializable = SerializableInstant::from(*instant);
        serializable.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Instant, D::Error>
    where
        D: Deserializer<'de>,
    {
        let serializable = SerializableInstant::deserialize(deserializer)?;
        Ok(serializable.into())
    }
}

/// Module for serializing/deserializing Option<Instant>
pub mod option_instant_serde {
    use super::*;
    use serde::{Serializer, Deserializer};
    use std::time::Instant;

    pub fn serialize<S>(opt_instant: &Option<Instant>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match opt_instant {
            Some(instant) => {
                let serializable = SerializableInstant::from(*instant);
                Some(serializable).serialize(serializer)
            }
            None => None::<SerializableInstant>.serialize(serializer),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Instant>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt_serializable: Option<SerializableInstant> = Option::deserialize(deserializer)?;
        Ok(opt_serializable.map(|s| s.into()))
    }
}