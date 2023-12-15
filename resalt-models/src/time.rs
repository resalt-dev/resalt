// Wrapper around chrono::DateTime<Utc>, but make it serializable/deserializable

use chrono::{
    format::{DelayedFormat, StrftimeItems},
    DateTime, Duration, NaiveDateTime, ParseError, Utc,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{
    fmt::Display,
    fmt::Formatter,
    ops::{Add, Sub},
};

#[derive(Clone, Copy, Eq, Default)]
pub struct ResaltTime {
    time: DateTime<Utc>,
}

const TIME_FMT: &str = "%Y-%m-%dT%H:%M:%S%.6fZ";

impl ResaltTime {
    #[inline]
    #[must_use]
    pub fn format<'a>(&self, fmt: &'a str) -> DelayedFormat<StrftimeItems<'a>> {
        self.time.format(fmt)
    }

    #[inline]
    pub fn parse_from_rfc3339(s: &str) -> Result<ResaltTime, ParseError> {
        // Trim
        let s = s.trim();
        // Replace space with T
        let s = s.replace(' ', "T");
        // Append Z if missing
        let s = if s.ends_with('Z') {
            s.to_string()
        } else {
            format!("{}Z", s)
        };
        match DateTime::parse_from_rfc3339(&s) {
            Ok(time) => Ok(ResaltTime { time: time.into() }),
            Err(e) => Err(e),
        }
    }

    #[inline]
    #[must_use]
    pub fn now() -> ResaltTime {
        ResaltTime {
            time: chrono::Utc::now(),
        }
    }

    #[inline]
    #[must_use]
    pub fn timestamp(&self) -> i64 {
        self.time.timestamp()
    }
}

impl Add<Duration> for ResaltTime {
    type Output = ResaltTime;

    #[inline]
    fn add(self, rhs: Duration) -> ResaltTime {
        ResaltTime {
            time: self.time + rhs,
        }
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
        write!(f, "{}", self.time.format(TIME_FMT))
    }
}

impl std::fmt::Debug for ResaltTime {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.time.format(TIME_FMT))
    }
}

impl PartialEq for ResaltTime {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Ord for ResaltTime {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Drop too many nanoseconds by converting to string and back
        let other = ResaltTime::parse_from_rfc3339(&other.to_string()).unwrap();
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for ResaltTime {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }

    #[inline]
    fn lt(&self, other: &Self) -> bool {
        let other = ResaltTime::parse_from_rfc3339(&other.to_string()).unwrap();
        self.time.lt(&other.time)
    }

    #[inline]
    fn le(&self, other: &Self) -> bool {
        let other = ResaltTime::parse_from_rfc3339(&other.to_string()).unwrap();
        self.time.le(&other.time)
    }

    #[inline]
    fn gt(&self, other: &Self) -> bool {
        let other = ResaltTime::parse_from_rfc3339(&other.to_string()).unwrap();
        self.time.gt(&other.time)
    }

    #[inline]
    fn ge(&self, other: &Self) -> bool {
        let other = ResaltTime::parse_from_rfc3339(&other.to_string()).unwrap();
        self.time.ge(&other.time)
    }
}

impl Serialize for ResaltTime {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.time.format(TIME_FMT).to_string())
    }
}

impl<'de> Deserialize<'de> for ResaltTime {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let time = String::deserialize(deserializer)?;
        match ResaltTime::parse_from_rfc3339(&time) {
            Ok(time) => Ok(time),
            Err(_) => Err(serde::de::Error::custom(format!(
                "Failed to parse time: {}",
                time
            ))),
        }
    }
}

impl From<ResaltTime> for DateTime<Utc> {
    fn from(val: ResaltTime) -> Self {
        val.time
    }
}

impl From<DateTime<Utc>> for ResaltTime {
    fn from(val: DateTime<Utc>) -> Self {
        ResaltTime { time: val }
    }
}

impl From<ResaltTime> for NaiveDateTime {
    fn from(val: ResaltTime) -> Self {
        val.time.naive_utc()
    }
}

impl From<NaiveDateTime> for ResaltTime {
    fn from(val: NaiveDateTime) -> Self {
        ResaltTime {
            time: DateTime::from_naive_utc_and_offset(val, Utc),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time() {
        let time = ResaltTime::parse_from_rfc3339("2020-01-01T00:00:00.000000Z").unwrap();
        assert_eq!(time.timestamp(), 1577836800);
        assert_eq!(time.to_string(), "2020-01-01T00:00:00.000000Z");
    }

    #[test]
    fn test_time_add() {
        let time = ResaltTime::parse_from_rfc3339("2020-01-01T00:00:00.000000Z").unwrap();
        let time2 = time + Duration::seconds(1);
        assert_eq!(time2.timestamp(), 1577836801);
    }

    #[test]
    fn test_time_sub() {
        let time = ResaltTime::parse_from_rfc3339("2020-01-01T00:00:00.000000Z").unwrap();
        let time2 = ResaltTime::parse_from_rfc3339("2020-01-01T00:00:01.000000Z").unwrap();
        assert_eq!(time2 - time, Duration::seconds(1));
    }

    #[test]
    fn test_time_serde() {
        let time = ResaltTime::parse_from_rfc3339("2020-01-01T00:00:00.000000Z").unwrap();
        let json = serde_json::to_string(&time).unwrap();
        assert_eq!(json, "\"2020-01-01T00:00:00.000000Z\"");
        let time2: ResaltTime = serde_json::from_str(&json).unwrap();
        assert_eq!(time, time2);
    }
}
