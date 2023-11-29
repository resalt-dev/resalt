use rand::distributions::{Alphanumeric, DistString};

#[inline]
#[must_use]
pub fn strip_quotes(s: &str) -> String {
    #[allow(clippy::if_same_then_else)]
    if s.starts_with('"') && s.ends_with('"') {
        s[1..s.len() - 1].to_string()
    } else if s.starts_with('\'') && s.ends_with('\'') {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

#[inline]
#[must_use]
pub(crate) fn generate_random_token() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 512)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_quotes() {
        assert_eq!(strip_quotes("\"hello\""), "hello");
        assert_eq!(strip_quotes("'world'"), "world");
        assert_eq!(strip_quotes("no quotes"), "no quotes");
    }

    #[test]
    fn test_generate_random_token() {
        let result = generate_random_token();
        assert_eq!(result.len(), 512);
    }
}
