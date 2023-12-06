// Wrapper around chrono::NaiveDateTime, but make it serializable/deserializable

use chrono::{
    format::{DelayedFormat, StrftimeItems},
    Duration, NaiveDateTime, ParseError,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt::Display, fmt::Formatter, ops::Sub};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
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
            time: chrono::Utc::now().naive_utc(),
        }
    }

    #[inline]
    #[must_use]
    pub fn timestamp(&self) -> i64 {
        self.time.timestamp()
    }
}

impl Sub for ResaltTime {
    type Output = Duration;

    #[inline]
    fn sub(self, rhs: ResaltTime) -> Duration {
        self.time - rhs.time
    }
}

impl Display for ResaltTime {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.time.format("%Y-%m-%d %H:%M:%S"))
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

impl From<ResaltTime> for NaiveDateTime {
    fn from(val: ResaltTime) -> Self {
        val.time
    }
}

impl From<NaiveDateTime> for ResaltTime {
    fn from(val: NaiveDateTime) -> Self {
        ResaltTime { time: val }
    }
}
