//! Identifier parsing and lightweight ID heuristics.

mod change_id;
mod error;
mod module_id;
mod spec_id;
pub(crate) mod sub_module_id;

pub use change_id::parse_change_id;
pub use change_id::{ChangeId, ParsedChangeId};
pub use error::IdParseError;
pub use module_id::parse_module_id;
pub use module_id::{ModuleId, ParsedModuleId};
pub use spec_id::parse_spec_id;
pub use spec_id::{ParsedSpecId, SpecId};
pub use sub_module_id::parse_sub_module_id;
pub use sub_module_id::{ParsedSubModuleId, SubModuleId};

/// The kind of an Ito identifier, as determined by [`classify_id`].
///
/// Use this when you need to route an opaque user-supplied string to the
/// correct parser without attempting a full parse first.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItoIdKind {
    /// A module identifier: `NNN` or `NNN_name` (e.g., `"005"`, `"005_dev-tooling"`).
    ModuleId,
    /// A sub-module identifier: `NNN.SS` or `NNN.SS_name` (e.g., `"005.01"`).
    SubModuleId,
    /// A change identifier in the legacy module format: `NNN-NN_name` (e.g., `"005-01_my-change"`).
    ModuleChangeId,
    /// A change identifier in the sub-module format: `NNN.SS-NN_name` (e.g., `"005.01-03_my-change"`).
    SubModuleChangeId,
}

/// Classify an opaque identifier string into one of the four [`ItoIdKind`] variants.
///
/// This is a lightweight structural heuristic. It does **not** validate the
/// identifier; use the appropriate `parse_*` function for full validation.
///
/// The classification inspects the portion of the string **before** the first
/// `_` (the name separator), so hyphens inside name suffixes (e.g.,
/// `005_dev-tooling`) do not affect the result.
///
/// | Prefix structure         | Kind                |
/// |--------------------------|---------------------|
/// | `NNN.SS-NN`              | `SubModuleChangeId` |
/// | `NNN.SS`                 | `SubModuleId`       |
/// | `NNN-NN`                 | `ModuleChangeId`    |
/// | `NNN`                    | `ModuleId`          |
pub fn classify_id(input: &str) -> ItoIdKind {
    // Inspect only the prefix before the first `_` so that hyphens inside
    // name suffixes (e.g., "005_dev-tooling") do not affect classification.
    let prefix = match input.split_once('_') {
        Some((left, _)) => left,
        None => input,
    };

    let has_dot = prefix.contains('.');
    let has_hyphen = prefix.contains('-');

    if has_dot && has_hyphen {
        ItoIdKind::SubModuleChangeId
    } else if has_dot {
        ItoIdKind::SubModuleId
    } else if has_hyphen {
        ItoIdKind::ModuleChangeId
    } else {
        ItoIdKind::ModuleId
    }
}

/// Quick heuristic used by CLI prompts to detect a likely change id.
///
/// Returns `true` for both legacy `NNN-NN_name` and sub-module `NNN.SS-NN_name`
/// formats.
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
            '.' => {}
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
    fn looks_like_change_id_recognizes_sub_module_format() {
        assert!(looks_like_change_id("005.01-03_my-change"));
        assert!(looks_like_change_id("5.1-2_foo"));
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

    #[test]
    fn classify_id_module_change_id() {
        assert_eq!(classify_id("005-01_my-change"), ItoIdKind::ModuleChangeId);
        assert_eq!(classify_id("1-2_foo"), ItoIdKind::ModuleChangeId);
    }

    #[test]
    fn classify_id_sub_module_change_id() {
        assert_eq!(
            classify_id("005.01-03_my-change"),
            ItoIdKind::SubModuleChangeId
        );
        assert_eq!(classify_id("5.1-2_foo"), ItoIdKind::SubModuleChangeId);
    }

    #[test]
    fn classify_id_sub_module_id() {
        assert_eq!(classify_id("005.01"), ItoIdKind::SubModuleId);
        assert_eq!(classify_id("005.01_core-api"), ItoIdKind::SubModuleId);
    }

    #[test]
    fn classify_id_module_id() {
        assert_eq!(classify_id("005"), ItoIdKind::ModuleId);
        assert_eq!(classify_id("005_dev-tooling"), ItoIdKind::ModuleId);
        assert_eq!(classify_id("1"), ItoIdKind::ModuleId);
    }

    #[test]
    fn classify_id_hyphen_without_underscore_is_module_change_id() {
        // "005-01" has a hyphen in the prefix → classified as ModuleChangeId.
        // It is not a *valid* change id (missing name), but structurally it
        // looks like one. Full validation is left to parse_change_id.
        assert_eq!(classify_id("005-01"), ItoIdKind::ModuleChangeId);
    }
}
