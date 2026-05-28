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
