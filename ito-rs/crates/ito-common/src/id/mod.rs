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
    input
        .trim()
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .count()
        > 0
        && input.contains('-')
        && input.contains('_')
}

/// Quick heuristic used by CLI prompts to detect a likely module id.
pub fn looks_like_module_id(input: &str) -> bool {
    let t = input.trim();
    !t.is_empty() && t.chars().next().is_some_and(|c| c.is_ascii_digit())
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
