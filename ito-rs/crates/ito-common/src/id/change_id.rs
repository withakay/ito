use std::fmt;

use super::IdParseError;
use super::ModuleId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChangeId(String);

impl ChangeId {
    pub(crate) fn new(inner: String) -> Self {
        Self(inner)
    }

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
pub struct ParsedChangeId {
    pub module_id: ModuleId,
    pub change_num: String,
    pub name: String,
    pub canonical: ChangeId,
}

pub fn parse_change_id(input: &str) -> Result<ParsedChangeId, IdParseError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(IdParseError::new(
            "Change ID cannot be empty",
            Some("Provide a change ID like \"1-2_my-change\" or \"001-02_my-change\""),
        ));
    }

    // Match TS hint for the common mistake: using '_' between module and change number.
    // Example: "001_02_name" (should be "001-02_name").
    if trimmed.contains('_') && !trimmed.contains('-') {
        let mut parts = trimmed.split('_');
        let a = parts.next().unwrap_or("");
        let b = parts.next().unwrap_or("");
        let c = parts.next().unwrap_or("");
        if !a.is_empty()
            && !b.is_empty()
            && !c.is_empty()
            && a.chars().all(|ch| ch.is_ascii_digit())
            && b.chars().all(|ch| ch.is_ascii_digit())
        {
            return Err(IdParseError::new(
                format!("Invalid change ID format: \"{input}\""),
                Some(
                    "Change IDs use \"-\" between module and change number (e.g., \"001-02_name\" not \"001_02_name\")",
                ),
            ));
        }
    }

    // TS: const FLEXIBLE_CHANGE_PATTERN = /^(\d+)-(\d+)_([a-z][a-z0-9-]*)$/i;
    let Some((left, name_part)) = trimmed.split_once('_') else {
        if trimmed.split_once('-').is_some_and(|(a, b)| {
            !a.is_empty()
                && !b.is_empty()
                && a.chars().all(|c| c.is_ascii_digit())
                && b.chars().all(|c| c.is_ascii_digit())
        }) {
            return Err(IdParseError::new(
                format!("Change ID missing name: \"{input}\""),
                Some("Change IDs require a name suffix (e.g., \"001-02_my-change\")"),
            ));
        }
        return Err(IdParseError::new(
            format!("Invalid change ID format: \"{input}\""),
            Some(
                "Expected format: \"NNN-NN_name\" (e.g., \"1-2_my-change\", \"001-02_my-change\")",
            ),
        ));
    };

    let Some((module_part, change_part)) = left.split_once('-') else {
        return Err(IdParseError::new(
            format!("Invalid change ID format: \"{input}\""),
            Some(
                "Expected format: \"NNN-NN_name\" (e.g., \"1-2_my-change\", \"001-02_my-change\")",
            ),
        ));
    };

    if module_part.is_empty()
        || change_part.is_empty()
        || !module_part.chars().all(|c| c.is_ascii_digit())
        || !change_part.chars().all(|c| c.is_ascii_digit())
    {
        return Err(IdParseError::new(
            format!("Invalid change ID format: \"{input}\""),
            Some(
                "Expected format: \"NNN-NN_name\" (e.g., \"1-2_my-change\", \"001-02_my-change\")",
            ),
        ));
    }

    let module_num: u32 = module_part.parse().map_err(|_| {
        IdParseError::new(
            "Change ID is required",
            Some("Provide a change ID like \"1-2_my-change\" or \"001-02_my-change\""),
        )
    })?;
    let change_num: u32 = change_part.parse().map_err(|_| {
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
    let canonical = ChangeId::new(format!("{module_id}-{change_num_str}_{name}"));

    Ok(ParsedChangeId {
        module_id,
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
    }

    #[test]
    fn parse_change_id_supports_extra_leading_zeros_for_change_num() {
        let parsed = parse_change_id("1-00003_bar").unwrap();
        assert_eq!(parsed.canonical.as_str(), "001-03_bar");
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
}
