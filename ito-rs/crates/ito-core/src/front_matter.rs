//! YAML front matter parsing, writing, and metadata utilities.
//!
//! Ito module and change markdown artifacts support an optional YAML front
//! matter header delimited by `---` lines at the beginning of the file.
//!
//! Front matter stores stable metadata (timestamps, identifiers, integrity
//! checksums) that is independent of filesystem attributes and survives
//! copies across hosts.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

use crate::errors::CoreError;

/// Parsed YAML front matter metadata for an Ito artifact.
///
/// Timestamps are stored as RFC 3339 strings to avoid requiring the `serde`
/// feature on `chrono`. Use [`FrontMatter::created_at_dt`] and
/// [`FrontMatter::updated_at_dt`] to parse them into `DateTime<Utc>`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FrontMatter {
    /// Schema version for forward compatibility.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,

    /// When the artifact was first created (RFC 3339 UTC string).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    /// When the artifact was last updated (RFC 3339 UTC string).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,

    /// Identity of the creator (optional).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,

    /// Identity of the last updater (optional).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,

    /// Change identifier for integrity validation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change_id: Option<String>,

    /// Module identifier for integrity validation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub module_id: Option<String>,

    /// Integrity metadata for corruption detection.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integrity: Option<IntegrityMetadata>,

    /// Additional fields not captured by the typed struct.
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_yaml::Value>,
}

impl FrontMatter {
    /// Parse `created_at` into a `DateTime<Utc>`, if present and valid.
    pub fn created_at_dt(&self) -> Option<DateTime<Utc>> {
        self.created_at
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
    }

    /// Parse `updated_at` into a `DateTime<Utc>`, if present and valid.
    pub fn updated_at_dt(&self) -> Option<DateTime<Utc>> {
        self.updated_at
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
    }
}

/// Integrity metadata for checksum-based corruption detection.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IntegrityMetadata {
    /// SHA-256 hex digest of the markdown body (content after front matter).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_sha256: Option<String>,
}

/// Result of parsing front matter from a markdown document.
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedDocument {
    /// Parsed front matter, if present.
    pub front_matter: Option<FrontMatter>,
    /// Markdown body (everything after the closing `---`).
    pub body: String,
}

/// Front matter delimiter.
const DELIMITER: &str = "---";

/// Parse a markdown document that may start with YAML front matter.
///
/// Front matter is delimited by `---` on a line by itself at the start
/// of the document. The opening `---` must be the very first line.
/// The closing `---` terminates the YAML block.
///
/// Returns the parsed metadata (if present) and the remaining markdown body.
pub fn parse(content: &str) -> Result<ParsedDocument, CoreError> {
    let Some(rest) = content.strip_prefix(DELIMITER) else {
        return Ok(ParsedDocument {
            front_matter: None,
            body: content.to_string(),
        });
    };

    // The delimiter must be followed by a newline (or be the entire first line)
    let Some(rest) = rest
        .strip_prefix('\n')
        .or_else(|| rest.strip_prefix("\r\n"))
    else {
        // The line has content after `---`, so this is not front matter
        return Ok(ParsedDocument {
            front_matter: None,
            body: content.to_string(),
        });
    };

    // Find the closing delimiter
    let Some(end_pos) = find_closing_delimiter(rest) else {
        // No closing delimiter found — treat entire content as body
        return Ok(ParsedDocument {
            front_matter: None,
            body: content.to_string(),
        });
    };

    let yaml_block = &rest[..end_pos];
    let body_start = end_pos + DELIMITER.len();
    let remaining = &rest[body_start..];

    // Strip exactly one leading newline from the body
    let body = remaining
        .strip_prefix('\n')
        .or_else(|| remaining.strip_prefix("\r\n"))
        .unwrap_or(remaining);

    let front_matter: FrontMatter = serde_yaml::from_str(yaml_block)
        .map_err(|e| CoreError::Parse(format!("invalid YAML front matter: {e}")))?;

    Ok(ParsedDocument {
        front_matter: Some(front_matter),
        body: body.to_string(),
    })
}

/// Find the position of the closing `---` delimiter in the remaining text.
///
/// The closing delimiter must appear on a line by itself (possibly with
/// trailing whitespace).
fn find_closing_delimiter(text: &str) -> Option<usize> {
    let mut pos = 0;
    for line in text.lines() {
        if line.trim() == DELIMITER {
            return Some(pos);
        }
        // Advance past this line plus its newline
        pos += line.len();
        // Account for the newline character(s)
        if text[pos..].starts_with("\r\n") {
            pos += 2;
        } else if text[pos..].starts_with('\n') {
            pos += 1;
        }
    }
    None
}

/// Serialize front matter and body back into a markdown document with
/// YAML front matter.
///
/// If `front_matter` is `None`, the body is returned as-is.
pub fn write(front_matter: Option<&FrontMatter>, body: &str) -> Result<String, CoreError> {
    let Some(fm) = front_matter else {
        return Ok(body.to_string());
    };

    let yaml = serde_yaml::to_string(fm)
        .map_err(|e| CoreError::Parse(format!("failed to serialize front matter: {e}")))?;

    // serde_yaml adds a trailing newline; remove it so we control formatting
    let yaml = yaml.trim_end();

    Ok(format!("{DELIMITER}\n{yaml}\n{DELIMITER}\n{body}"))
}

/// Format a `DateTime<Utc>` as an RFC 3339 string for front matter.
fn format_timestamp(dt: DateTime<Utc>) -> String {
    dt.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

/// Update the `updated_at` timestamp in front matter to the current time.
///
/// If `front_matter` is `None`, creates a new `FrontMatter` with only
/// `created_at` and `updated_at` set to `now`.
pub fn touch(front_matter: Option<FrontMatter>, now: DateTime<Utc>) -> FrontMatter {
    let ts = format_timestamp(now);
    match front_matter {
        Some(mut fm) => {
            fm.updated_at = Some(ts);
            fm
        }
        None => FrontMatter {
            schema_version: Some("1".to_string()),
            created_at: Some(ts.clone()),
            updated_at: Some(ts),
            created_by: None,
            updated_by: None,
            change_id: None,
            module_id: None,
            integrity: None,
            extra: BTreeMap::new(),
        },
    }
}

/// Compute the SHA-256 hex digest of a body string.
pub fn body_sha256(body: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(body.as_bytes());
    hex::encode(hasher.finalize())
}

/// Update the integrity checksum in front matter to match the given body.
pub fn update_integrity(front_matter: &mut FrontMatter, body: &str) {
    let checksum = body_sha256(body);
    match &mut front_matter.integrity {
        Some(integrity) => {
            integrity.body_sha256 = Some(checksum);
        }
        None => {
            front_matter.integrity = Some(IntegrityMetadata {
                body_sha256: Some(checksum),
            });
        }
    }
}

/// Validate that a front matter checksum matches the body content.
///
/// Returns `Ok(())` if there is no checksum or the checksum matches.
/// Returns `Err` if the checksum is present but does not match.
pub fn validate_integrity(front_matter: &FrontMatter, body: &str) -> Result<(), CoreError> {
    let Some(integrity) = &front_matter.integrity else {
        return Ok(());
    };

    let Some(expected) = &integrity.body_sha256 else {
        return Ok(());
    };

    let actual = body_sha256(body);
    if *expected != actual {
        return Err(CoreError::Validation(format!(
            "artifact body checksum mismatch: expected {expected}, got {actual}"
        )));
    }

    Ok(())
}

/// Validate that a front matter identifier matches the expected value.
///
/// Returns `Ok(())` if the front matter field is `None` (absent).
/// Returns `Err` if the field is present and does not match.
pub fn validate_id(
    field_name: &str,
    front_matter_value: Option<&str>,
    expected: &str,
) -> Result<(), CoreError> {
    let Some(actual) = front_matter_value else {
        return Ok(());
    };

    if actual != expected {
        return Err(CoreError::Validation(format!(
            "{field_name} mismatch in front matter: expected '{expected}', found '{actual}'"
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Timelike};

    #[test]
    fn parse_no_front_matter() {
        let content = "# Hello\n\nSome content.";
        let result = parse(content).unwrap();
        assert!(result.front_matter.is_none());
        assert_eq!(result.body, content);
    }

    #[test]
    fn parse_valid_front_matter() {
        let content =
            "---\nschema_version: \"1\"\ncreated_at: \"2026-01-15T10:00:00Z\"\n---\n# Hello\n";
        let result = parse(content).unwrap();
        let fm = result.front_matter.unwrap();
        assert_eq!(fm.schema_version.as_deref(), Some("1"));
        assert_eq!(fm.created_at.as_deref(), Some("2026-01-15T10:00:00Z"));
        let dt = fm.created_at_dt().unwrap();
        assert_eq!(dt, Utc.with_ymd_and_hms(2026, 1, 15, 10, 0, 0).unwrap());
        assert_eq!(result.body, "# Hello\n");
    }

    #[test]
    fn parse_empty_front_matter() {
        let content = "---\n---\n# Body";
        let result = parse(content).unwrap();
        let fm = result.front_matter.unwrap();
        assert!(fm.schema_version.is_none());
        assert_eq!(result.body, "# Body");
    }

    #[test]
    fn parse_no_closing_delimiter() {
        let content = "---\nschema_version: 1\n# Not closed";
        let result = parse(content).unwrap();
        // No closing delimiter → treat as regular content
        assert!(result.front_matter.is_none());
        assert_eq!(result.body, content);
    }

    #[test]
    fn parse_delimiter_with_extra_text_on_first_line() {
        let content = "--- extra stuff\nschema_version: 1\n---\nbody";
        let result = parse(content).unwrap();
        // Not valid front matter start
        assert!(result.front_matter.is_none());
        assert_eq!(result.body, content);
    }

    #[test]
    fn parse_invalid_yaml() {
        let content = "---\n: : invalid:\n---\nbody";
        let result = parse(content);
        assert!(result.is_err());
    }

    #[test]
    fn parse_with_integrity() {
        let body = "# Content\n";
        let checksum = body_sha256(body);
        let content = format!("---\nintegrity:\n  body_sha256: {checksum}\n---\n{body}");
        let result = parse(&content).unwrap();
        let fm = result.front_matter.unwrap();
        assert_eq!(
            fm.integrity.as_ref().unwrap().body_sha256.as_deref(),
            Some(checksum.as_str())
        );
        assert_eq!(result.body, body);
    }

    #[test]
    fn roundtrip_write_parse() {
        let now = Utc.with_ymd_and_hms(2026, 3, 1, 12, 0, 0).unwrap();
        let fm = touch(None, now);

        let body = "# My proposal\n\nSome text.\n";
        let doc = write(Some(&fm), body).unwrap();
        let parsed = parse(&doc).unwrap();

        let parsed_fm = parsed.front_matter.as_ref().unwrap();
        assert_eq!(parsed_fm.created_at_dt(), Some(now));
        assert_eq!(parsed.body, body);
    }

    #[test]
    fn write_no_front_matter_returns_body() {
        let body = "# Just body\n";
        let result = write(None, body).unwrap();
        assert_eq!(result, body);
    }

    #[test]
    fn touch_creates_new_front_matter() {
        let now = Utc::now();
        let fm = touch(None, now);
        assert!(fm.created_at.is_some());
        assert!(fm.updated_at.is_some());
        assert_eq!(fm.created_at, fm.updated_at);
        assert_eq!(fm.schema_version.as_deref(), Some("1"));
        // Verify roundtrip through DateTime
        assert_eq!(fm.created_at_dt(), Some(now.with_nanosecond(0).unwrap()));
    }

    #[test]
    fn touch_updates_existing() {
        let t1 = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
        let t2 = Utc.with_ymd_and_hms(2026, 3, 1, 0, 0, 0).unwrap();
        let fm = touch(None, t1);
        let updated = touch(Some(fm), t2);
        // created_at should be unchanged
        assert_eq!(updated.created_at_dt(), Some(t1));
        // updated_at should be the new time
        assert_eq!(updated.updated_at_dt(), Some(t2));
    }

    #[test]
    fn body_sha256_is_deterministic() {
        let body = "# Hello world\n";
        let h1 = body_sha256(body);
        let h2 = body_sha256(body);
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 64);
    }

    #[test]
    fn update_integrity_sets_checksum() {
        let mut fm = touch(None, Utc::now());
        let body = "Some content\n";
        update_integrity(&mut fm, body);
        let expected = body_sha256(body);
        assert_eq!(
            fm.integrity.as_ref().unwrap().body_sha256.as_deref(),
            Some(expected.as_str())
        );
    }

    #[test]
    fn validate_integrity_passes_when_matching() {
        let body = "# Good content\n";
        let mut fm = touch(None, Utc::now());
        update_integrity(&mut fm, body);
        assert!(validate_integrity(&fm, body).is_ok());
    }

    #[test]
    fn validate_integrity_fails_on_mismatch() {
        let body = "# Good content\n";
        let mut fm = touch(None, Utc::now());
        update_integrity(&mut fm, body);
        let result = validate_integrity(&fm, "# Tampered content\n");
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("checksum mismatch"));
    }

    #[test]
    fn validate_integrity_passes_when_no_checksum() {
        let fm = touch(None, Utc::now());
        assert!(validate_integrity(&fm, "anything").is_ok());
    }

    #[test]
    fn validate_id_passes_when_absent() {
        assert!(validate_id("change_id", None, "024-10").is_ok());
    }

    #[test]
    fn validate_id_passes_when_matching() {
        assert!(validate_id("change_id", Some("024-10"), "024-10").is_ok());
    }

    #[test]
    fn validate_id_fails_on_mismatch() {
        let result = validate_id("change_id", Some("999-99_bad"), "024-10");
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("change_id"));
        assert!(msg.contains("mismatch"));
    }

    #[test]
    fn parse_preserves_extra_fields() {
        let content = "---\ncustom_field: hello\n---\nbody";
        let result = parse(content).unwrap();
        let fm = result.front_matter.unwrap();
        assert_eq!(
            fm.extra.get("custom_field"),
            Some(&serde_yaml::Value::String("hello".to_string()))
        );
    }

    #[test]
    fn format_timestamp_produces_rfc3339() {
        let dt = Utc.with_ymd_and_hms(2026, 3, 1, 12, 30, 45).unwrap();
        let ts = format_timestamp(dt);
        assert_eq!(ts, "2026-03-01T12:30:45Z");
    }

    #[test]
    fn created_at_dt_returns_none_when_absent() {
        let fm = FrontMatter {
            schema_version: None,
            created_at: None,
            updated_at: None,
            created_by: None,
            updated_by: None,
            change_id: None,
            module_id: None,
            integrity: None,
            extra: BTreeMap::new(),
        };
        assert!(fm.created_at_dt().is_none());
    }

    #[test]
    fn created_at_dt_returns_none_for_invalid_timestamp() {
        let fm = FrontMatter {
            schema_version: None,
            created_at: Some("not-a-date".to_string()),
            updated_at: None,
            created_by: None,
            updated_by: None,
            change_id: None,
            module_id: None,
            integrity: None,
            extra: BTreeMap::new(),
        };
        assert!(fm.created_at_dt().is_none());
    }
}
