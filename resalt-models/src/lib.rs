mod db;
mod filter;
mod salt;
mod sort;
mod status;
mod storage;
mod time;

pub use db::*;
pub use filter::*;
pub use salt::*;
pub use sort::*;
pub use status::*;
pub use storage::*;
pub use time::*;

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
