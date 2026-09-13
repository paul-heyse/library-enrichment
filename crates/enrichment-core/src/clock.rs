//! RFC 3339 timestamps for provenance, without a date-time dependency.
//!
//! Timestamps are provenance only (blueprint §6.3): they never enter a content identity, so
//! second precision in UTC is all the service needs, and a dependency is not worth it.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// The current time as `YYYY-MM-DDTHH:MM:SSZ`.
#[must_use]
pub fn now_rfc3339() -> String {
    rfc3339_utc(SystemTime::now())
}

/// Render a system time as `YYYY-MM-DDTHH:MM:SSZ`. Times before 1970 render as the epoch.
#[must_use]
pub fn rfc3339_utc(time: SystemTime) -> String {
    let secs = time
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_secs();
    let days = secs / 86_400;
    let rem = secs % 86_400;
    let (year, month, day) = civil_from_days(days as i64);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        rem / 3600,
        (rem % 3600) / 60,
        rem % 60
    )
}

/// Seconds since the epoch, now.
#[must_use]
pub fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_secs()
}

/// Parse `YYYY-MM-DDTHH:MM:SSZ` (as written by [`rfc3339_utc`], fractional seconds tolerated)
/// back to seconds since the epoch. Anything else is `None`.
#[must_use]
pub fn parse_rfc3339(text: &str) -> Option<u64> {
    let text = text.strip_suffix('Z')?;
    let (date, time) = text.split_once('T')?;
    let mut date_parts = date.split('-');
    let year: i64 = date_parts.next()?.parse().ok()?;
    let month: u32 = date_parts.next()?.parse().ok()?;
    let day: u32 = date_parts.next()?.parse().ok()?;
    let time = time.split('.').next()?;
    let mut time_parts = time.split(':');
    let hour: u64 = time_parts.next()?.parse().ok()?;
    let minute: u64 = time_parts.next()?.parse().ok()?;
    let second: u64 = time_parts.next()?.parse().ok()?;
    if !(1..=12).contains(&month)
        || !(1..=31).contains(&day)
        || hour > 23
        || minute > 59
        || second > 60
    {
        return None;
    }
    let days = days_from_civil(year, month, day);
    if days < 0 {
        return None;
    }
    Some(days as u64 * 86_400 + hour * 3600 + minute * 60 + second)
}

/// Proleptic Gregorian civil date to days since 1970-01-01 (Howard Hinnant's algorithm).
fn days_from_civil(year: i64, month: u32, day: u32) -> i64 {
    let y = if month <= 2 { year - 1 } else { year };
    let era = y.div_euclid(400);
    let yoe = y.rem_euclid(400);
    let m = i64::from(month);
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + i64::from(day) - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

/// Days since 1970-01-01 to a proleptic Gregorian civil date (Howard Hinnant's algorithm).
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_instants_render_correctly() {
        assert_eq!(rfc3339_utc(UNIX_EPOCH), "1970-01-01T00:00:00Z");
        // 2026-09-13T21:07:12Z, the docs.rs probe time recorded in the compatibility matrix.
        let t = UNIX_EPOCH + Duration::from_secs(1_789_333_632);
        assert_eq!(rfc3339_utc(t), "2026-09-13T21:07:12Z");
        // A leap day.
        let leap = UNIX_EPOCH + Duration::from_secs(951_782_400);
        assert_eq!(rfc3339_utc(leap), "2000-02-29T00:00:00Z");
    }

    #[test]
    fn rendering_and_parsing_are_inverse() {
        for secs in [0u64, 951_782_400, 1_789_333_632, 4_102_444_800] {
            let rendered = rfc3339_utc(UNIX_EPOCH + Duration::from_secs(secs));
            assert_eq!(parse_rfc3339(&rendered), Some(secs), "{rendered}");
        }
        assert_eq!(parse_rfc3339("2026-09-13T21:07:12.5Z"), Some(1_789_333_632));
        assert_eq!(parse_rfc3339("not a time"), None);
        assert_eq!(parse_rfc3339("2026-13-01T00:00:00Z"), None);
        assert!(now_secs() > 1_789_333_632);
    }

    #[test]
    fn now_has_the_expected_shape() {
        let now = now_rfc3339();
        assert_eq!(now.len(), 20);
        assert!(now.ends_with('Z'));
        assert_eq!(&now[4..5], "-");
        assert_eq!(&now[10..11], "T");
    }
}
