// Wrapper around chrono::NaiveDateTime, but make it serializable/deserializable

use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResaltTime {
    time: NaiveDateTime,
}

impl Default for ResaltTime {
    fn default() -> Self {
        ResaltTime {
            time: chrono::NaiveDateTime::default(),
        }
    }
}

impl Serialize for ResaltTime {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.time.format("%Y-%m-%d %H:%M:%S").to_string())
    }
}

impl<'de> Deserialize<'de> for ResaltTime {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let time = String::deserialize(deserializer)?;
        let time = NaiveDateTime::parse_from_str(&time, "%Y-%m-%d %H:%M:%S").unwrap();
        Ok(ResaltTime { time })
    }
}

impl Into<NaiveDateTime> for ResaltTime {
    fn into(self) -> NaiveDateTime {
        self.time
    }
}

impl Into<ResaltTime> for NaiveDateTime {
    fn into(self) -> ResaltTime {
        ResaltTime { time: self }
    }
}
