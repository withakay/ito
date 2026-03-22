//! Change ID parsing and normalization.

use std::fmt;

use super::sub_module_id::SubModuleId;
use super::IdParseError;
use super::ModuleId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// A change identifier.
///
/// Changes are tracked as `NNN-NN_name` (e.g. `014-01_add-rust-crate-documentation`).
pub struct ChangeId(String);

impl ChangeId {
    pub(crate) fn new(inner: String) -> Self {
        Self(inner)
    }

    /// Borrow the underlying string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ChangeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Parsed representation of a change identifier.
pub struct ParsedChangeId {
    /// Canonical module id.
    pub module_id: ModuleId,

    /// Sub-module id in canonical `NNN.SS` form, present only for sub-module changes.
    ///
    /// `None` for legacy `NNN-NN_name` format changes without a sub-module component.
    pub sub_module_id: Option<SubModuleId>,

    /// Canonical change number (at least 2 digits).
    pub change_num: String,

    /// Canonicalized change name (lowercase).
    pub name: String,

    /// Canonical `NNN-NN_name` or `NNN.SS-NN_name` string.
    pub canonical: ChangeId,
}

/// Parse a change identifier.
///
/// Accepts both the legacy `NNN-NN_name` format and the sub-module
/// `NNN.SS-NN_name` format with flexible zero-padding; always returns a
/// canonical representation.
///
/// When the input contains a sub-module component (`NNN.SS-NN_name`), the
/// returned [`ParsedChangeId`] has `sub_module_id` set to `Some(...)`.
/// Legacy `NNN-NN_name` inputs produce `sub_module_id = None`.
pub fn parse_change_id(input: &str) -> Result<ParsedChangeId, IdParseError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(IdParseError::new(
            "Change ID cannot be empty",
            Some("Provide a change ID like \"1-2_my-change\" or \"001-02_my-change\""),
        ));
    }

    if trimmed.len() > 256 {
        return Err(IdParseError::new(
            format!("Change ID is too long: {} bytes (max 256)", trimmed.len()),
            Some("Provide a shorter change ID in the form \"NNN-NN_name\""),
        ));
    }

    // Match TS hint for the common mistake: using '_' between module and change number.
    // Example: "001_02_name" (should be "001-02_name").
    if trimmed.contains('_') && !trimmed.contains('-') {
        let mut parts = trimmed.split('_');
        let a = parts.next().unwrap_or("");
        let b = parts.next().unwrap_or("");
        let c = parts.next().unwrap_or("");
        let mut a_all_digits = true;
        for ch in a.chars() {
            if !ch.is_ascii_digit() {
                a_all_digits = false;
                break;
            }
        }

        let mut b_all_digits = true;
        for ch in b.chars() {
            if !ch.is_ascii_digit() {
                b_all_digits = false;
                break;
            }
        }

        if !a.is_empty() && !b.is_empty() && !c.is_empty() && a_all_digits && b_all_digits {
            return Err(IdParseError::new(
                format!("Invalid change ID format: \"{input}\""),
                Some(
                    "Change IDs use \"-\" between module and change number (e.g., \"001-02_name\" not \"001_02_name\")",
                ),
            ));
        }
    }

    // Split off the name suffix (everything after the first `_`).
    let Some((left, name_part)) = trimmed.split_once('_') else {
        if let Some((a, b)) = trimmed.split_once('-') {
            let mut a_all_digits = true;
            for c in a.chars() {
                if !c.is_ascii_digit() {
                    a_all_digits = false;
                    break;
                }
            }

            let mut b_all_digits = true;
            for c in b.chars() {
                if !c.is_ascii_digit() {
                    b_all_digits = false;
                    break;
                }
            }

            if !a.is_empty() && !b.is_empty() && a_all_digits && b_all_digits {
                return Err(IdParseError::new(
                    format!("Change ID missing name: \"{input}\""),
                    Some("Change IDs require a name suffix (e.g., \"001-02_my-change\")"),
                ));
            }
        }
        return Err(IdParseError::new(
            format!("Invalid change ID format: \"{input}\""),
            Some(
                "Expected format: \"NNN-NN_name\" (e.g., \"1-2_my-change\", \"001-02_my-change\")",
            ),
        ));
    };

    // Determine whether this is a sub-module change (`NNN.SS-NN_name`) or a
    // legacy change (`NNN-NN_name`) by checking for a `.` in the left part.
    let (module_num, sub_module_id, change_part) = if left.contains('.') {
        // Sub-module format: NNN.SS-NN
        let Some((module_sub_part, change_str)) = left.split_once('-') else {
            return Err(IdParseError::new(
                format!("Invalid change ID format: \"{input}\""),
                Some(
                    "Expected format: \"NNN.SS-NN_name\" (e.g., \"005.01-02_my-change\")",
                ),
            ));
        };

        let Some((module_str, sub_str)) = module_sub_part.split_once('.') else {
            return Err(IdParseError::new(
                format!("Invalid change ID format: \"{input}\""),
                Some(
                    "Expected format: \"NNN.SS-NN_name\" (e.g., \"005.01-02_my-change\")",
                ),
            ));
        };

        // Validate all three numeric parts.
        let mut module_all_digits = true;
        for c in module_str.chars() {
            if !c.is_ascii_digit() {
                module_all_digits = false;
                break;
            }
        }

        let mut sub_all_digits = true;
        for c in sub_str.chars() {
            if !c.is_ascii_digit() {
                sub_all_digits = false;
                break;
            }
        }

        let mut change_all_digits = true;
        for c in change_str.chars() {
            if !c.is_ascii_digit() {
                change_all_digits = false;
                break;
            }
        }

        if module_str.is_empty()
            || !module_all_digits
            || sub_str.is_empty()
            || !sub_all_digits
            || change_str.is_empty()
            || !change_all_digits
        {
            return Err(IdParseError::new(
                format!("Invalid change ID format: \"{input}\""),
                Some(
                    "Expected format: \"NNN.SS-NN_name\" (e.g., \"005.01-02_my-change\")",
                ),
            ));
        }

        let module_num: u32 = module_str.parse().map_err(|_| {
            IdParseError::new(
                "Change ID is required",
                Some("Provide a change ID like \"005.01-02_my-change\""),
            )
        })?;

        let sub_num: u32 = sub_str.parse().map_err(|_| {
            IdParseError::new(
                "Change ID is required",
                Some("Provide a change ID like \"005.01-02_my-change\""),
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

        let sub_id = SubModuleId::new(format!("{module_num:03}.{sub_num:02}"));
        (module_num, Some(sub_id), change_str)
    } else {
        // Legacy format: NNN-NN
        let Some((module_str, change_str)) = left.split_once('-') else {
            return Err(IdParseError::new(
                format!("Invalid change ID format: \"{input}\""),
                Some(
                    "Expected format: \"NNN-NN_name\" (e.g., \"1-2_my-change\", \"001-02_my-change\")",
                ),
            ));
        };

        let mut module_all_digits = true;
        for c in module_str.chars() {
            if !c.is_ascii_digit() {
                module_all_digits = false;
                break;
            }
        }

        let mut change_all_digits = true;
        for c in change_str.chars() {
            if !c.is_ascii_digit() {
                change_all_digits = false;
                break;
            }
        }

        if module_str.is_empty()
            || change_str.is_empty()
            || !module_all_digits
            || !change_all_digits
        {
            return Err(IdParseError::new(
                format!("Invalid change ID format: \"{input}\""),
                Some(
                    "Expected format: \"NNN-NN_name\" (e.g., \"1-2_my-change\", \"001-02_my-change\")",
                ),
            ));
        }

        let module_num: u32 = module_str.parse().map_err(|_| {
            IdParseError::new(
                "Change ID is required",
                Some("Provide a change ID like \"1-2_my-change\" or \"001-02_my-change\""),
            )
        })?;

        if module_num > 999 {
            return Err(IdParseError::new(
                format!("Module number {module_num} exceeds maximum (999)"),
                Some("Module numbers must be between 0 and 999"),
            ));
        }

        (module_num, None, change_str)
    };

    let change_num: u32 = change_part.parse().map_err(|_| {
        IdParseError::new(
            "Change ID is required",
            Some("Provide a change ID like \"1-2_my-change\" or \"001-02_my-change\""),
        )
    })?;

    // NOTE: Do not enforce an upper bound for change numbers.
    // Padding is for readability/sorting only; functionality is more important.

    // Validate name
    let mut chars = name_part.chars();
    let first = chars.next().unwrap_or('\0');
    if !first.is_ascii_alphabetic() {
        return Err(IdParseError::new(
            format!("Invalid change ID format: \"{input}\""),
            Some(
                "Expected format: \"NNN-NN_name\" (e.g., \"1-2_my-change\", \"001-02_my-change\")",
            ),
        ));
    }
    for c in chars {
        if !(c.is_ascii_alphanumeric() || c == '-') {
            return Err(IdParseError::new(
                format!("Invalid change ID format: \"{input}\""),
                Some(
                    "Expected format: \"NNN-NN_name\" (e.g., \"1-2_my-change\", \"001-02_my-change\")",
                ),
            ));
        }
    }

    let module_id = ModuleId::new(format!("{module_num:03}"));
    let change_num_str = format!("{change_num:02}");
    let name = name_part.to_ascii_lowercase();

    let canonical = match &sub_module_id {
        Some(sub_id) => ChangeId::new(format!("{sub_id}-{change_num_str}_{name}")),
        None => ChangeId::new(format!("{module_id}-{change_num_str}_{name}")),
    };

    Ok(ParsedChangeId {
        module_id,
        sub_module_id,
        change_num: change_num_str,
        name,
        canonical,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_change_id_pads_both_parts() {
        let parsed = parse_change_id("1-2_Bar").unwrap();
        assert_eq!(parsed.canonical.as_str(), "001-02_bar");
        assert_eq!(parsed.module_id.as_str(), "001");
        assert_eq!(parsed.change_num, "02");
        assert_eq!(parsed.name, "bar");
        assert_eq!(parsed.sub_module_id, None);
    }

    #[test]
    fn parse_change_id_supports_extra_leading_zeros_for_change_num() {
        let parsed = parse_change_id("1-00003_bar").unwrap();
        assert_eq!(parsed.canonical.as_str(), "001-03_bar");
        assert_eq!(parsed.sub_module_id, None);
    }

    #[test]
    fn parse_change_id_allows_three_digit_change_numbers() {
        let parsed = parse_change_id("1-100_Bar").unwrap();
        assert_eq!(parsed.canonical.as_str(), "001-100_bar");
        assert_eq!(parsed.change_num, "100");
    }

    #[test]
    fn parse_change_id_normalizes_excessive_padding_for_large_change_numbers() {
        let parsed = parse_change_id("1-000100_bar").unwrap();
        assert_eq!(parsed.canonical.as_str(), "001-100_bar");
        assert_eq!(parsed.change_num, "100");
    }

    #[test]
    fn parse_change_id_allows_large_change_numbers() {
        let parsed = parse_change_id("1-1234_example").unwrap();
        assert_eq!(parsed.canonical.as_str(), "001-1234_example");
        assert_eq!(parsed.change_num, "1234");
    }

    #[test]
    fn parse_change_id_missing_name_has_specific_error() {
        let err = parse_change_id("1-2").unwrap_err();
        assert_eq!(err.error, "Change ID missing name: \"1-2\"");
    }

    #[test]
    fn parse_change_id_uses_specific_hint_for_wrong_separator() {
        let err = parse_change_id("001_02_name").unwrap_err();
        assert_eq!(err.error, "Invalid change ID format: \"001_02_name\"");
        assert_eq!(
            err.hint.as_deref(),
            Some(
                "Change IDs use \"-\" between module and change number (e.g., \"001-02_name\" not \"001_02_name\")"
            )
        );
    }

    #[test]
    fn parse_change_id_rejects_overlong_input() {
        let input = format!("001-01_{}", "a".repeat(300));
        let err = parse_change_id(&input).expect_err("overlong change id should fail");
        assert!(err.error.contains("too long"));
    }

    // Sub-module format tests

    #[test]
    fn parse_change_id_sub_module_format_canonical() {
        let parsed = parse_change_id("005.01-03_my-change").unwrap();
        assert_eq!(parsed.canonical.as_str(), "005.01-03_my-change");
        assert_eq!(parsed.module_id.as_str(), "005");
        assert_eq!(parsed.change_num, "03");
        assert_eq!(parsed.name, "my-change");
        let sub_id = parsed.sub_module_id.as_ref().unwrap();
        assert_eq!(sub_id.as_str(), "005.01");
    }

    #[test]
    fn parse_change_id_sub_module_format_pads_all_parts() {
        let parsed = parse_change_id("5.1-3_foo").unwrap();
        assert_eq!(parsed.canonical.as_str(), "005.01-03_foo");
        assert_eq!(parsed.module_id.as_str(), "005");
        let sub_id = parsed.sub_module_id.as_ref().unwrap();
        assert_eq!(sub_id.as_str(), "005.01");
        assert_eq!(parsed.change_num, "03");
    }

    #[test]
    fn parse_change_id_sub_module_format_lowercases_name() {
        let parsed = parse_change_id("005.01-03_My-Change").unwrap();
        assert_eq!(parsed.name, "my-change");
        assert_eq!(parsed.canonical.as_str(), "005.01-03_my-change");
    }

    #[test]
    fn parse_change_id_sub_module_rejects_sub_overflow() {
        let err = parse_change_id("005.100-01_foo").unwrap_err();
        assert!(err.error.contains("exceeds maximum (99)"));
    }

    #[test]
    fn parse_change_id_sub_module_rejects_module_overflow() {
        let err = parse_change_id("1000.01-01_foo").unwrap_err();
        assert!(err.error.contains("exceeds maximum (999)"));
    }

    #[test]
    fn parse_change_id_sub_module_missing_name_is_error() {
        let err = parse_change_id("005.01-03").unwrap_err();
        assert!(err.error.contains("Invalid change ID format") || err.error.contains("missing name"));
    }
}
