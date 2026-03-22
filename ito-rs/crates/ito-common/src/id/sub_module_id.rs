//! Sub-module ID parsing and normalization.

use std::fmt;

use super::{IdParseError, ModuleId, is_all_ascii_digits};

/// A sub-module identifier in canonical `NNN.SS` form.
///
/// Sub-modules partition a parent module into named sections, each with their
/// own change sequence. The canonical form is always `NNN.SS` (e.g., `005.01`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SubModuleId(String);

impl SubModuleId {
    /// Construct a `SubModuleId` from a pre-validated canonical string.
    pub(crate) fn new(inner: String) -> Self {
        Self(inner)
    }

    /// Borrow the canonical `NNN.SS` sub-module id string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SubModuleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Parsed representation of a sub-module identifier.
///
/// Produced by [`parse_sub_module_id`]; carries the canonical id, the parent
/// module id, the zero-padded sub-module number, and an optional name suffix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedSubModuleId {
    /// Canonical sub-module id (e.g., `"005.01"`).
    pub sub_module_id: SubModuleId,

    /// Parent module id (e.g., `"005"`).
    pub parent_module_id: ModuleId,

    /// Zero-padded sub-module number (e.g., `"01"`).
    pub sub_num: String,

    /// Optional name suffix (lowercased), e.g., `"core-api"` from `"005.01_core-api"`.
    pub sub_name: Option<String>,
}

/// Parse a sub-module identifier.
///
/// Accepts `NNN.SS` or `NNN.SS_name` with flexible zero-padding; always
/// returns a canonical `NNN.SS` representation.
///
/// # Errors
///
/// Returns [`IdParseError`] when the input is empty, too long, contains
/// non-numeric parts, or exceeds the allowed ranges (module ≤ 999, sub ≤ 99).
pub fn parse_sub_module_id(input: &str) -> Result<ParsedSubModuleId, IdParseError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(IdParseError::new(
            "Sub-module ID cannot be empty",
            Some("Provide a sub-module ID like \"005.01\" or \"005.01_core-api\""),
        ));
    }

    if trimmed.len() > 256 {
        return Err(IdParseError::new(
            format!(
                "Sub-module ID is too long: {} bytes (max 256)",
                trimmed.len()
            ),
            Some("Provide a shorter sub-module ID in the form \"NNN.SS\" or \"NNN.SS_name\""),
        ));
    }

    // Strip optional name suffix (everything after the first `_`).
    let (id_part, name_part) = match trimmed.split_once('_') {
        Some((left, right)) => (left, Some(right)),
        None => (trimmed, None),
    };

    // id_part must be NNN.SS
    let Some((module_str, sub_str)) = id_part.split_once('.') else {
        return Err(IdParseError::new(
            format!("Invalid sub-module ID format: \"{input}\""),
            Some("Expected format: \"NNN.SS\" or \"NNN.SS_name\" (e.g., \"005.01\", \"005.01_core-api\")"),
        ));
    };

    if !is_all_ascii_digits(module_str) || !is_all_ascii_digits(sub_str) {
        return Err(IdParseError::new(
            format!("Invalid sub-module ID format: \"{input}\""),
            Some("Expected format: \"NNN.SS\" or \"NNN.SS_name\" (e.g., \"005.01\", \"005.01_core-api\")"),
        ));
    }

    let module_num: u32 = module_str.parse().map_err(|_| {
        IdParseError::new(
            "Sub-module ID is required",
            Some("Provide a sub-module ID like \"005.01\" or \"005.01_core-api\""),
        )
    })?;

    let sub_num: u32 = sub_str.parse().map_err(|_| {
        IdParseError::new(
            "Sub-module ID is required",
            Some("Provide a sub-module ID like \"005.01\" or \"005.01_core-api\""),
        )
    })?;

    if module_num > 999 {
        return Err(IdParseError::new(
            format!("Module number {module_num} exceeds maximum (999)"),
            Some("Module numbers must be between 0 and 999"),
        ));
    }

    if sub_num > 99 {
        return Err(IdParseError::new(
            format!("Sub-module number {sub_num} exceeds maximum (99)"),
            Some("Sub-module numbers must be between 0 and 99"),
        ));
    }

    // Validate optional name suffix.
    let sub_name = match name_part {
        None => None,
        Some(name) => {
            if name.is_empty() {
                return Err(IdParseError::new(
                    format!("Invalid sub-module ID format: \"{input}\""),
                    Some("Expected format: \"NNN.SS\" or \"NNN.SS_name\" (e.g., \"005.01\", \"005.01_core-api\")"),
                ));
            }

            let mut chars = name.chars();
            let first = chars.next().unwrap_or('\0');
            if !first.is_ascii_alphabetic() {
                return Err(IdParseError::new(
                    format!("Invalid sub-module ID format: \"{input}\""),
                    Some("Expected format: \"NNN.SS\" or \"NNN.SS_name\" (e.g., \"005.01\", \"005.01_core-api\")"),
                ));
            }
            for c in chars {
                if !(c.is_ascii_alphanumeric() || c == '-') {
                    return Err(IdParseError::new(
                        format!("Invalid sub-module ID format: \"{input}\""),
                        Some("Expected format: \"NNN.SS\" or \"NNN.SS_name\" (e.g., \"005.01\", \"005.01_core-api\")"),
                    ));
                }
            }
            Some(name.to_ascii_lowercase())
        }
    };

    let parent_module_id = ModuleId::new(format!("{module_num:03}"));
    let sub_num_str = format!("{sub_num:02}");
    let sub_module_id = SubModuleId::new(format!("{parent_module_id}.{sub_num_str}"));

    Ok(ParsedSubModuleId {
        sub_module_id,
        parent_module_id,
        sub_num: sub_num_str,
        sub_name,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_sub_module_id_canonical_form() {
        let parsed = parse_sub_module_id("005.01").unwrap();
        assert_eq!(parsed.sub_module_id.as_str(), "005.01");
        assert_eq!(parsed.parent_module_id.as_str(), "005");
        assert_eq!(parsed.sub_num, "01");
        assert_eq!(parsed.sub_name, None);
    }

    #[test]
    fn parse_sub_module_id_pads_both_parts() {
        let parsed = parse_sub_module_id("5.1").unwrap();
        assert_eq!(parsed.sub_module_id.as_str(), "005.01");
        assert_eq!(parsed.parent_module_id.as_str(), "005");
        assert_eq!(parsed.sub_num, "01");
    }

    #[test]
    fn parse_sub_module_id_with_name_suffix() {
        let parsed = parse_sub_module_id("005.01_core-api").unwrap();
        assert_eq!(parsed.sub_module_id.as_str(), "005.01");
        assert_eq!(parsed.sub_name.as_deref(), Some("core-api"));
    }

    #[test]
    fn parse_sub_module_id_lowercases_name() {
        let parsed = parse_sub_module_id("005.01_Core-API").unwrap();
        assert_eq!(parsed.sub_name.as_deref(), Some("core-api"));
    }

    #[test]
    fn parse_sub_module_id_strips_extra_leading_zeros() {
        let parsed = parse_sub_module_id("005.001").unwrap();
        assert_eq!(parsed.sub_module_id.as_str(), "005.01");
        assert_eq!(parsed.sub_num, "01");
    }

    #[test]
    fn parse_sub_module_id_rejects_empty() {
        let err = parse_sub_module_id("").unwrap_err();
        assert_eq!(err.error, "Sub-module ID cannot be empty");
    }

    #[test]
    fn parse_sub_module_id_rejects_missing_dot() {
        let err = parse_sub_module_id("005-01").unwrap_err();
        assert!(err.error.contains("Invalid sub-module ID format"));
    }

    #[test]
    fn parse_sub_module_id_rejects_module_overflow() {
        let err = parse_sub_module_id("1000.01").unwrap_err();
        assert!(err.error.contains("exceeds maximum (999)"));
    }

    #[test]
    fn parse_sub_module_id_rejects_sub_overflow() {
        let err = parse_sub_module_id("005.100").unwrap_err();
        assert!(err.error.contains("exceeds maximum (99)"));
    }

    #[test]
    fn parse_sub_module_id_rejects_non_digit_module() {
        let err = parse_sub_module_id("abc.01").unwrap_err();
        assert!(err.error.contains("Invalid sub-module ID format"));
    }

    #[test]
    fn parse_sub_module_id_rejects_overlong_input() {
        let input = format!("005.01_{}", "a".repeat(300));
        let err = parse_sub_module_id(&input).expect_err("overlong sub-module id should fail");
        assert!(err.error.contains("too long"));
    }

    #[test]
    fn sub_module_id_display() {
        let id = SubModuleId::new("005.01".to_string());
        assert_eq!(id.to_string(), "005.01");
    }
}
