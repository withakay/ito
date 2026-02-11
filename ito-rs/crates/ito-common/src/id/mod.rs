//! Identifier parsing and lightweight ID heuristics.

mod change_id;
mod error;
mod module_id;
mod spec_id;

pub use change_id::parse_change_id;
pub use change_id::{ChangeId, ParsedChangeId};
pub use error::IdParseError;
pub use module_id::parse_module_id;
pub use module_id::{ModuleId, ParsedModuleId};
pub use spec_id::parse_spec_id;
pub use spec_id::{ParsedSpecId, SpecId};

/// Quick heuristic used by CLI prompts to detect a likely change id.
pub fn looks_like_change_id(input: &str) -> bool {
    let input = input.trim();
    if input.is_empty() {
        return false;
    }

    let mut digit_prefix_len = 0usize;
    let mut has_hyphen = false;
    let mut has_underscore = false;

    for ch in input.chars() {
        if ch.is_ascii_digit() && digit_prefix_len == 0 {
            digit_prefix_len = 1;
            continue;
        }

        if ch.is_ascii_digit() && digit_prefix_len > 0 {
            digit_prefix_len += 1;
            continue;
        }

        if digit_prefix_len == 0 {
            break;
        }

        match ch {
            '-' => has_hyphen = true,
            '_' => has_underscore = true,
            _ => {}
        }
    }

    digit_prefix_len > 0 && has_hyphen && has_underscore
}

/// Quick heuristic used by CLI prompts to detect a likely module id.
pub fn looks_like_module_id(input: &str) -> bool {
    let input = input.trim();
    let Some(first) = input.chars().next() else {
        return false;
    };
    first.is_ascii_digit()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn looks_like_change_id_requires_digits_hyphen_and_underscore() {
        assert!(looks_like_change_id("001-02_hello"));
        assert!(!looks_like_change_id("-02_hello"));
        assert!(!looks_like_change_id("001_hello"));
        assert!(!looks_like_change_id("001-02hello"));
        assert!(!looks_like_change_id("abc-02_hello"));
    }

    #[test]
    fn looks_like_module_id_is_digit_prefixed() {
        assert!(looks_like_module_id("001"));
        assert!(looks_like_module_id("001_demo"));
        assert!(looks_like_module_id(" 001_demo "));
        assert!(!looks_like_module_id(""));
        assert!(!looks_like_module_id("demo"));
        assert!(!looks_like_module_id("_001_demo"));
    }
}
