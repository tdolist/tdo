//! Simple time expression parser for `--due` and `--remind` arguments.
//!
//! Accepts:
//! - ISO-8601 datetime: `2025-06-15T09:00:00Z` or `2025-06-15T09:00:00+02:00`
//! - ISO-8601 date (assumes midnight UTC): `2025-06-15`
//! - Relative offsets: `+1h`, `+2h`, `+1d`, `+7d`, `+1w`, `+30m`
//! - Keywords: `tomorrow`, `next week`

use anyhow::{Result, bail};
use chrono::{DateTime, Duration, NaiveDate, TimeZone, Utc};

pub fn parse_time_expr(s: &str) -> Result<DateTime<Utc>> {
    let s = s.trim();

    // Keywords
    match s.to_lowercase().as_str() {
        "tomorrow" => return Ok(Utc::now() + Duration::days(1)),
        "next week" => return Ok(Utc::now() + Duration::weeks(1)),
        _ => {}
    }

    // Relative offsets: +<n><unit>
    if let Some(rest) = s.strip_prefix('+') {
        let (num_str, unit) = rest
            .find(|c: char| c.is_alphabetic())
            .map(|i| (&rest[..i], &rest[i..]))
            .ok_or_else(|| anyhow::anyhow!("invalid relative time: {s:?}"))?;
        let n: i64 = num_str
            .parse()
            .map_err(|_| anyhow::anyhow!("invalid number in relative time: {num_str:?}"))?;
        let duration = match unit.to_lowercase().as_str() {
            "m" | "min" | "mins" => Duration::minutes(n),
            "h" | "hr" | "hrs" => Duration::hours(n),
            "d" | "day" | "days" => Duration::days(n),
            "w" | "week" | "weeks" => Duration::weeks(n),
            other => bail!("unknown time unit {other:?} — use m/h/d/w"),
        };
        return Ok(Utc::now() + duration);
    }

    // ISO-8601 datetime (with timezone)
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Ok(dt.with_timezone(&Utc));
    }

    // ISO-8601 date only (midnight UTC)
    if let Ok(date) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        let midnight = date
            .and_hms_opt(0, 0, 0)
            .ok_or_else(|| anyhow::anyhow!("invalid date: {s:?}"))?;
        return Ok(Utc.from_utc_datetime(&midnight));
    }

    bail!(
        "could not parse time expression {s:?}\n\
         Expected: ISO-8601 date/datetime, relative offset (+1h/+2d/+1w), \
         or keyword (tomorrow, next week)"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_relative_hours() {
        let t = parse_time_expr("+2h").unwrap();
        let diff = (t - Utc::now()).num_seconds();
        // Allow ±5 s for test execution time; target is 7200 s.
        assert!(
            (7195..=7205).contains(&diff),
            "expected ~7200s, got {diff}s"
        );
    }

    #[test]
    fn parses_relative_days() {
        let t = parse_time_expr("+1d").unwrap();
        let diff = (t - Utc::now()).num_seconds();
        // Allow ±5 s for test execution time; target is 86400 s.
        assert!(
            (86395..=86405).contains(&diff),
            "expected ~86400s, got {diff}s"
        );
    }

    #[test]
    fn parses_iso_date() {
        let t = parse_time_expr("2099-01-01").unwrap();
        assert_eq!(t.format("%Y-%m-%d").to_string(), "2099-01-01");
    }

    #[test]
    fn parses_iso_datetime() {
        let t = parse_time_expr("2099-01-01T12:00:00Z").unwrap();
        assert_eq!(
            t.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
            "2099-01-01T12:00:00Z"
        );
    }

    #[test]
    fn unknown_unit_returns_error() {
        assert!(parse_time_expr("+5x").is_err());
    }

    #[test]
    fn garbage_returns_error() {
        assert!(parse_time_expr("next tuesday").is_err());
    }
}
