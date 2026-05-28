use super::*;

#[test]
fn test_parse_seconds() {
    assert_eq!(parse_duration("30s").unwrap(), Duration::from_secs(30));
    assert_eq!(parse_duration("1s").unwrap(), Duration::from_secs(1));
    assert_eq!(parse_duration("120s").unwrap(), Duration::from_secs(120));
}

#[test]
fn test_parse_minutes() {
    assert_eq!(parse_duration("5m").unwrap(), Duration::from_secs(300));
    assert_eq!(parse_duration("1m").unwrap(), Duration::from_secs(60));
}

#[test]
fn test_parse_hours() {
    assert_eq!(parse_duration("2h").unwrap(), Duration::from_secs(7200));
    assert_eq!(parse_duration("1h").unwrap(), Duration::from_secs(3600));
}

#[test]
fn test_parse_combined() {
    assert_eq!(parse_duration("1m30s").unwrap(), Duration::from_secs(90));
    assert_eq!(parse_duration("1h30m").unwrap(), Duration::from_secs(5400));
    assert_eq!(
        parse_duration("1h30m45s").unwrap(),
        Duration::from_secs(5445)
    );
}

#[test]
fn test_parse_bare_number() {
    assert_eq!(parse_duration("90").unwrap(), Duration::from_secs(90));
    assert_eq!(parse_duration("1").unwrap(), Duration::from_secs(1));
}

#[test]
fn test_parse_case_insensitive() {
    assert_eq!(parse_duration("5M").unwrap(), Duration::from_secs(300));
    assert_eq!(parse_duration("2H").unwrap(), Duration::from_secs(7200));
    assert_eq!(parse_duration("30S").unwrap(), Duration::from_secs(30));
}

#[test]
fn test_parse_with_whitespace() {
    assert_eq!(parse_duration(" 30s ").unwrap(), Duration::from_secs(30));
}

#[test]
fn test_parse_errors() {
    assert!(parse_duration("").is_err());
    assert!(parse_duration("abc").is_err());
    assert!(parse_duration("5x").is_err());
    assert!(parse_duration("m5").is_err());
}

#[test]
fn test_format_duration() {
    assert_eq!(format_duration(Duration::from_secs(30)), "30s");
    assert_eq!(format_duration(Duration::from_secs(60)), "1m");
    assert_eq!(format_duration(Duration::from_secs(90)), "1m30s");
    assert_eq!(format_duration(Duration::from_secs(3600)), "1h");
    assert_eq!(format_duration(Duration::from_secs(3660)), "1h1m");
    assert_eq!(format_duration(Duration::from_secs(3661)), "1h1m1s");
    assert_eq!(format_duration(Duration::from_secs(0)), "0s");
}
