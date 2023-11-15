// Wrapper around chrono::NaiveDateTime, but make it serializable/deserializable

use chrono::{
    format::{DelayedFormat, StrftimeItems},
    NaiveDateTime, ParseError,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ResaltTime {
    time: NaiveDateTime,
}

impl ResaltTime {
    #[inline]
    #[must_use]
    pub fn format<'a>(&self, fmt: &'a str) -> DelayedFormat<StrftimeItems<'a>> {
        self.time.format(fmt)
    }

    #[inline]
    #[must_use]
    pub fn parse_from_str(s: &str, fmt: &str) -> Result<ResaltTime, ParseError> {
        match NaiveDateTime::parse_from_str(s, fmt) {
            Ok(time) => Ok(ResaltTime { time }),
            Err(e) => Err(e),
        }
    }

    #[inline]
    #[must_use]
    pub fn now() -> ResaltTime {
        ResaltTime {
            time: chrono::Utc::now().naive_utc().into(),
        }
    }

    #[inline]
    #[must_use]
    pub fn timestamp(&self) -> i64 {
        self.time.timestamp()
    }
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
