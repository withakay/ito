use miette::{Result, miette};
use std::time::Duration;

/// Parse a human-readable duration string into a Duration.
///
/// Supported formats:
/// - `30s` - 30 seconds
/// - `5m` - 5 minutes
/// - `1m30s` - 1 minute 30 seconds
/// - `2h` - 2 hours
/// - `1h30m` - 1 hour 30 minutes
/// - `1h30m45s` - 1 hour 30 minutes 45 seconds
/// - `90` - 90 seconds (bare number defaults to seconds)
///
/// # Examples
/// ```
/// use ito_core::ralph::duration::parse_duration;
/// use std::time::Duration;
///
/// assert_eq!(parse_duration("30s").unwrap(), Duration::from_secs(30));
/// assert_eq!(parse_duration("5m").unwrap(), Duration::from_secs(300));
/// assert_eq!(parse_duration("1m30s").unwrap(), Duration::from_secs(90));
/// assert_eq!(parse_duration("90").unwrap(), Duration::from_secs(90));
/// ```
pub fn parse_duration(s: &str) -> Result<Duration> {
    let s = s.trim();
    if s.is_empty() {
        return Err(miette!("Duration string cannot be empty"));
    }

    // Try parsing as bare number (seconds)
    if let Ok(secs) = s.parse::<u64>() {
        return Ok(Duration::from_secs(secs));
    }

    let mut total_secs: u64 = 0;
    let mut current_num = String::new();

    for c in s.chars() {
        if c.is_ascii_digit() {
            current_num.push(c);
        } else {
            let unit = c.to_ascii_lowercase();
            if current_num.is_empty() {
                return Err(miette!(
                    "Invalid duration format: missing number before '{unit}'"
                ));
            }
            let num: u64 = current_num
                .parse()
                .map_err(|_| miette!("Invalid number in duration: {current_num}"))?;
            current_num.clear();

            let multiplier = match unit {
                's' => 1,
                'm' => 60,
                'h' => 3600,
                _ => {
                    return Err(miette!(
                        "Invalid duration unit '{unit}'. Use 's', 'm', or 'h'"
                    ));
                }
            };

            total_secs = total_secs
                .checked_add(num.saturating_mul(multiplier))
                .ok_or_else(|| miette!("Duration overflow"))?;
        }
    }

    // Handle trailing number without unit (treat as seconds)
    if !current_num.is_empty() {
        let num: u64 = current_num
            .parse()
            .map_err(|_| miette!("Invalid number in duration: {current_num}"))?;
        total_secs = total_secs
            .checked_add(num)
            .ok_or_else(|| miette!("Duration overflow"))?;
    }

    if total_secs == 0 {
        return Err(miette!("Duration must be greater than 0"));
    }

    Ok(Duration::from_secs(total_secs))
}

/// Format a Duration as a human-readable string.
pub fn format_duration(d: Duration) -> String {
    let total_secs = d.as_secs();
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;

    let mut parts = Vec::new();
    if hours > 0 {
        parts.push(format!("{hours}h"));
    }
    if minutes > 0 {
        parts.push(format!("{minutes}m"));
    }
    if seconds > 0 || parts.is_empty() {
        parts.push(format!("{seconds}s"));
    }
    parts.join("")
}

#[cfg(test)]
mod tests {
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
}
