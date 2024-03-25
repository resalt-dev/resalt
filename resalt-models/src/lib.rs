pub mod db;
pub mod filter;
pub mod salt;
pub mod sort;
pub mod status;
pub mod storage;
pub mod time;

use std::{fmt, str::FromStr};

pub use db::*;
pub use filter::*;
pub use salt::*;
pub use sort::*;
pub use status::*;
pub use storage::*;
pub use time::*;

use serde::{Deserialize, Deserializer};

/// Strip quotes from a string if it starts and ends with the same type of quote.
///
/// ```rust
/// use resalt_models::strip_quotes;
///
/// strip_quotes("test"); // => test
/// strip_quotes("\"test\""); // => test
/// strip_quotes("'test'"); // => test
/// strip_quotes("te\"st"); // => te"st
/// ```
///
/// This is useful for parsing command-line strings and Environment variables which may or may not be quoted.
///
/// ```rust
/// use resalt_models::strip_quotes;
///
/// // TEST="test"
///
/// let s = std::env::var("TEST").unwrap_or("\"test\"".to_string());
/// assert_eq!(s, "\"test\"");
///
/// let s = strip_quotes(s);
/// assert_eq!(s, "test");
/// ```
pub fn strip_quotes<S: AsRef<str>>(s: S) -> String {
    let s = s.as_ref();
    #[allow(clippy::if_same_then_else)]
    if s.starts_with('"') && s.ends_with('"') {
        s[1..s.len() - 1].to_string()
    } else if s.starts_with('\'') && s.ends_with('\'') {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

/// Serde deserialization decorator to map empty Strings to None, otherwise parse as T.
pub fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s)
            .map_err(serde::de::Error::custom)
            .map(Some),
    }
}

/// Serde deserialization decorator to map empty Strings to None, otherwise parse as i64.
pub fn empty_i64_as_none<'de, D>(de: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => s.parse::<i64>().map_err(serde::de::Error::custom).map(Some),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_quotes() {
        assert_eq!(strip_quotes("\"test\""), "test");
        assert_eq!(strip_quotes("'test'"), "test");
        assert_eq!(strip_quotes("test"), "test");
        assert_eq!(strip_quotes("te\"st"), "te\"st");
    }
}
