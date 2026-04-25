/// Tests for `stamp_version` — the idempotent version-stamp injector.
use ito_templates::{
    ITO_END_MARKER, ITO_START_MARKER, ITO_VERSION_MARKER_PREFIX, ITO_VERSION_MARKER_SUFFIX,
    get_skill_file, stamp_version,
};

fn canonical_stamp(version: &str) -> String {
    format!(
        "{}{}{}",
        ITO_VERSION_MARKER_PREFIX, version, ITO_VERSION_MARKER_SUFFIX
    )
}

// ── 1. No managed block → no-op ──────────────────────────────────────────────

#[test]
fn stamp_no_op_when_no_managed_block() {
    let input = "# Just a plain file\n\nNo markers here.\n";
    let result = stamp_version(input, "1.0.0");
    assert_eq!(
        result, input,
        "content without markers must be returned unchanged"
    );
}

// ── 2. Insert stamp when markers present but no stamp ────────────────────────

#[test]
fn stamp_inserts_when_no_existing_stamp() {
    let input = format!("{}\nhello\n{}\n", ITO_START_MARKER, ITO_END_MARKER);
    let result = stamp_version(&input, "1.2.3");

    let expected_stamp = canonical_stamp("1.2.3");
    assert!(
        result.contains(&expected_stamp),
        "stamped output must contain the canonical stamp; got:\n{result}"
    );

    // Stamp must appear on the line immediately after the start marker.
    let mut lines = result.lines();
    let start_line = lines
        .position(|l| l.trim() == ITO_START_MARKER)
        .expect("ITO:START must be present");
    let result_lines: Vec<&str> = result.lines().collect();
    assert_eq!(
        result_lines[start_line + 1],
        expected_stamp,
        "stamp must be on the line directly after ITO:START"
    );
}

// ── 3. Idempotent: same canonical stamp → byte-identical ─────────────────────

#[test]
fn stamp_idempotent_when_same_version() {
    let stamp = canonical_stamp("2.0.0");
    let input = format!(
        "{}\n{}\nhello\n{}\n",
        ITO_START_MARKER, stamp, ITO_END_MARKER
    );
    let result = stamp_version(&input, "2.0.0");
    assert_eq!(
        result, input,
        "re-stamping with the same version must be byte-identical"
    );
}

// ── 4. Spaced stamp → rewritten to canonical form ────────────────────────────

#[test]
fn stamp_rewrites_spaced_stamp_to_canonical() {
    // Spaced form: `<!-- ITO:VERSION: 1.2.3 -->`
    let spaced = "<!-- ITO:VERSION: 1.2.3 -->";
    let input = format!(
        "{}\n{}\nhello\n{}\n",
        ITO_START_MARKER, spaced, ITO_END_MARKER
    );
    let result = stamp_version(&input, "1.2.3");

    let canonical = canonical_stamp("1.2.3");
    assert!(
        result.contains(&canonical),
        "spaced stamp must be replaced with canonical form; got:\n{result}"
    );
    assert!(
        !result.contains(spaced),
        "spaced stamp must not remain in output; got:\n{result}"
    );
}

// ── 5. Older-version canonical stamp → rewritten to current version ───────────

#[test]
fn stamp_rewrites_older_version_stamp() {
    let old_stamp = canonical_stamp("0.9.0");
    let input = format!(
        "{}\n{}\nhello\n{}\n",
        ITO_START_MARKER, old_stamp, ITO_END_MARKER
    );
    let result = stamp_version(&input, "1.0.0");

    let new_stamp = canonical_stamp("1.0.0");
    assert!(
        result.contains(&new_stamp),
        "old stamp must be replaced with new version; got:\n{result}"
    );
    assert!(
        !result.contains(&old_stamp),
        "old stamp must not remain in output; got:\n{result}"
    );
}

// ── 6. Rest of file preserved byte-for-byte ───────────────────────────────────

#[test]
fn stamp_preserves_rest_of_file() {
    let preamble = "# Title\n\nSome intro text.\n\n";
    let postamble = "\n## After\n\nTrailing content.\n";
    let inner = "managed content\n";
    let input = format!(
        "{preamble}{}\n{inner}{}\n{postamble}",
        ITO_START_MARKER, ITO_END_MARKER
    );

    let result = stamp_version(&input, "3.1.4");

    // Preamble must be intact.
    assert!(
        result.starts_with(preamble),
        "preamble must be preserved; got:\n{result}"
    );
    // Postamble must be intact.
    assert!(
        result.ends_with(postamble),
        "postamble must be preserved; got:\n{result}"
    );
    // Inner content must still be present.
    assert!(
        result.contains(inner),
        "inner managed content must be preserved; got:\n{result}"
    );
    // Trailing newline preserved.
    assert!(result.ends_with('\n'), "trailing newline must be preserved");
}

// ── 7. Frontmatter before the marker ─────────────────────────────────────────

#[test]
fn stamp_works_with_frontmatter_before_marker() {
    let input = format!(
        "---\nname: test\n---\n\n{}\nhello\n{}\n",
        ITO_START_MARKER, ITO_END_MARKER
    );
    let result = stamp_version(&input, "1.0.0");

    // Frontmatter must be untouched.
    assert!(
        result.starts_with("---\nname: test\n---\n"),
        "frontmatter must be preserved; got:\n{result}"
    );
    // Stamp must be present.
    assert!(
        result.contains(&canonical_stamp("1.0.0")),
        "stamp must be injected; got:\n{result}"
    );
}

// ── 8. Round-trip: render a real embedded skill and assert stamp ──────────────

#[test]
fn stamp_round_trip_on_real_skill() {
    let bytes =
        get_skill_file("ito-feature/SKILL.md").expect("ito-feature/SKILL.md must be embedded");
    let text = std::str::from_utf8(bytes).expect("skill must be valid UTF-8");

    let version = "9.9.9";
    let stamped = stamp_version(text, version);

    let expected_stamp = canonical_stamp(version);

    // Stamp must appear exactly once.
    let occurrences = stamped.matches(&expected_stamp).count();
    assert_eq!(
        occurrences, 1,
        "stamp must appear exactly once; found {occurrences} occurrences"
    );

    // Stamp must be on the line directly after ITO:START.
    let lines: Vec<&str> = stamped.lines().collect();
    let start_pos = lines
        .iter()
        .position(|l| l.trim() == ITO_START_MARKER)
        .expect("ITO:START must be present in the skill");
    assert_eq!(
        lines[start_pos + 1],
        expected_stamp,
        "stamp must be on the line directly after ITO:START"
    );

    // A second stamp_version call with the same version must be byte-identical.
    let re_stamped = stamp_version(&stamped, version);
    assert_eq!(
        re_stamped, stamped,
        "re-stamping with the same version must be idempotent"
    );
}
