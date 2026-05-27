use super::*;

/// Verifies that each `CoreError` helper constructor produces the expected enum variant and contains the correct data.
///
/// Constructs every public `CoreError` variant via its respective helper (e.g., `io`, `validation`, `parse`, `process`,
/// `not_found`, `serde`, `sqlite`) and asserts both the variant and the values carried by that variant.
///
/// # Examples
///
///
#[test]
fn core_error_helpers_construct_expected_variants() {
    let io_err = CoreError::io("read config", io::Error::other("boom"));
    let CoreError::Io { context, source } = io_err else {
        panic!("expected io variant");
    };
    assert_eq!(context, "read config");
    assert_eq!(source.to_string(), "boom");

    let validation_err = CoreError::validation("bad");
    let CoreError::Validation(validation_msg) = validation_err else {
        panic!("expected validation variant");
    };
    assert_eq!(validation_msg, "bad");

    let parse_err = CoreError::parse("bad");
    let CoreError::Parse(parse_msg) = parse_err else {
        panic!("expected parse variant");
    };
    assert_eq!(parse_msg, "bad");

    let process_err = CoreError::process("bad");
    let CoreError::Process(process_msg) = process_err else {
        panic!("expected process variant");
    };
    assert_eq!(process_msg, "bad");

    let not_found_err = CoreError::not_found("bad");
    let CoreError::NotFound(not_found_msg) = not_found_err else {
        panic!("expected not-found variant");
    };
    assert_eq!(not_found_msg, "bad");

    let serde_err = CoreError::serde("load", "bad");
    let CoreError::Serde { context, message } = serde_err else {
        panic!("expected serde variant");
    };
    assert_eq!(context, "load");
    assert_eq!(message, "bad");

    let sqlite_err = CoreError::sqlite("bad");
    let CoreError::Sqlite(sqlite_msg) = sqlite_err else {
        panic!("expected sqlite variant");
    };
    assert_eq!(sqlite_msg, "bad");
}
