//! # Jalali-rs
//!
//! A simple crate for converting dates between the Gregorian and Jalali (Persian) calendars.
//!
//! This crate provides functions for date conversions, including support for Unix timestamps and string formatting.
//! All functions assume valid inputs where possible, but return `Option` for cases with potential invalid data to avoid panics.
//!
//! ## Core Features
//! - Convert Gregorian to Jalali dates and vice versa.
//! - Convert Unix timestamps to Jalali dates and vice versa (assuming UTC midnight; negative timestamps return `None`).
//! - Parse and format date strings with custom separators, handling Persian/Arabic digits automatically.
//! - Convert between Latin, Persian, and Arabic digits for flexible user input.
//!
//! ## Usage
//!
//! ```rust
//! use jalali_rs::{gregorian_to_jalali, jalali_to_gregorian,unix_to_jalali,jalali_to_unix,persian_or_arabic_digits_to_latin,latin_digits_to_persian,parse_gregorian_string_to_jalali_string,parse_jalali_string_to_gregorian_string};
//!
//! // Basic date conversion
//! let (jy, jm, jd) = gregorian_to_jalali(2025, 12, 27);
//! assert_eq!((jy, jm, jd), (1404, 10, 6));
//!
//! let (gy, gm, gd) = jalali_to_gregorian(1404, 10, 6);
//! assert_eq!((gy, gm, gd), (2025, 12, 27));
//!
//! // Unix timestamp conversion (0 -> 1970-01-01 Gregorian -> 1348-10-11 Jalali)
//! if let Some((jy, jm, jd)) = unix_to_jalali(0) {
//!     println!("Jalali from Unix 0: {}-{}-{}", jy, jm, jd);
//! }
//!
//! if let Some(timestamp) = jalali_to_unix(1348, 10, 11) {
//!     println!("Unix from Jalali 1348-10-11: {}", timestamp);
//! }
//!
//! // String parsing with separator (handles Persian/Arabic digits)
//! if let Some(jalali_str) = parse_gregorian_string_to_jalali_string("۲۰۲۵-۱۲-۲۷", '-') {
//!     println!("Converted: {}", jalali_str); // Outputs: 1404-10-06
//! }
//!
//! if let Some(gregorian_str) = parse_jalali_string_to_gregorian_string("۱۴۰۴-۱۰-۰۶", '-') {
//!     println!("Converted: {}", gregorian_str); // Outputs: 2025-12-27
//! }
//!
//! // Digit conversions
//! let persian = latin_digits_to_persian("2025-12-27");
//! println!("Persian digits: {}", persian); // ۲۰۲۵-۱۲-۲۷
//!
//! let latin = persian_or_arabic_digits_to_latin("۱۴۰۴-۱۰-۰۶");
//! println!("Latin digits: {}", latin); // 1404-10-06
//! ```

/// Converts a Gregorian date to a Jalali (Persian) date.
///
/// # Arguments
///
/// * `gregorian_year` - The Gregorian year (e.g., 2025).
/// * `gregorian_month` - The Gregorian month (1-12).
/// * `gregorian_day` - The Gregorian day (1-31).
///
/// # Returns
///
/// A tuple containing (jalali_year, jalali_month, jalali_day).
///
/// # Examples
///
/// ```
/// let (jy, jm, jd) = jalali_rs::gregorian_to_jalali(2025, 12, 27);
/// assert_eq!((jy, jm, jd), (1404, 10, 6));
/// ```
pub fn gregorian_to_jalali(
    gregorian_year: i32,
    gregorian_month: usize,
    gregorian_day: i32,
) -> (i32, u32, u32) {
    // cumulative days at the end of each Gregorian month (non-leap year, adjusted later)
    let gregorian_cumulative_days: [i64; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

    // adjust year for leap year calculation if month is after February
    let adjusted_year: i64 = if gregorian_month > 2 {
        gregorian_year as i64 + 1
    } else {
        gregorian_year as i64
    };

    // calculate total days from a fixed epoch, including leap year adjustments
    let mut total_days: i64 = 355666
        + (365 * gregorian_year as i64)
        + ((adjusted_year + 3) / 4)
        - ((adjusted_year + 99) / 100)
        + ((adjusted_year + 399) / 400)
        + gregorian_day as i64
        + gregorian_cumulative_days[gregorian_month - 1];

    // compute Jalali year using divisions based on Jalali cycle lengths
    let mut jalali_year: i64 = -1595 + (33 * (total_days / 12053));
    total_days %= 12053;
    jalali_year += 4 * (total_days / 1461);
    total_days %= 1461;

    // handle extra days beyond a standard year
    if total_days > 365 {
        jalali_year += (total_days - 1) / 365;
        total_days = (total_days - 1) % 365;
    }

    let (jalali_month, jalali_day);
    if total_days < 186 {
        // first half of Jalali year (months 1-6, 31 days each)
        jalali_month = 1 + (total_days / 31) as u32;
        jalali_day = 1 + (total_days % 31) as u32;
    } else {
        // second half of Jalali year (months 7-12, 30 days each except possibly last)
        jalali_month = 7 + ((total_days - 186) / 30) as u32;
        jalali_day = 1 + ((total_days - 186) % 30) as u32;
    }

    (jalali_year as i32, jalali_month, jalali_day)
}

/// Converts a Jalali (Persian) date to a Gregorian date.
///
/// # Arguments
///
/// * `jalali_year` - The Jalali year (e.g., 1404).
/// * `jalali_month` - The Jalali month (1-12).
/// * `jalali_day` - The Jalali day (1-31).
///
/// # Returns
///
/// A tuple containing (gregorian_year, gregorian_month, gregorian_day).
///
/// # Examples
///
/// ```
/// let (gy, gm, gd) = jalali_rs::jalali_to_gregorian(1404, 10, 6);
/// assert_eq!((gy, gm, gd), (2025, 12, 27));
/// ```
pub fn jalali_to_gregorian(
    jalali_year: i32,
    jalali_month: usize,
    jalali_day: i32,
) -> (i32, u32, u32) {
    let  jalali_year_i64: i64 = jalali_year as i64 + 1595;

    // calculate total days from a fixed epoch, including Jalali leap adjustments
    let mut total_days: i64 = -355668
        + (365 * jalali_year_i64)
        + ((jalali_year_i64 / 33) * 8)
        + (((jalali_year_i64 % 33) + 3) / 4)
        + jalali_day as i64
        + if jalali_month < 7 {
        (jalali_month as i64 - 1) * 31
    } else {
        ((jalali_month as i64 - 7) * 30) + 186
    };

    // compute Gregorian year using divisions based on Gregorian cycle lengths
    let mut gregorian_year: i64 = 400 * (total_days / 146097);
    total_days %= 146097;
    if total_days > 36524 {
        total_days -= 1;
        gregorian_year += 100 * (total_days / 36524);
        total_days %= 36524;
        if total_days >= 365 {
            total_days += 1;
        }
    }
    gregorian_year += 4 * (total_days / 1461);
    total_days %= 1461;
    if total_days > 365 {
        gregorian_year += (total_days - 1) / 365;
        total_days = (total_days - 1) % 365;
    }

    // determine Gregorian day and advance through months
    let mut gregorian_day: i64 = total_days + 1;

    // array of days in each Gregorian month, adjusting February for leap year
    let is_leap_year = (gregorian_year % 4 == 0 && gregorian_year % 100 != 0)
        || (gregorian_year % 400 == 0);
    let gregorian_days_in_month: [i64; 13] = [
        0,
        31,
        if is_leap_year { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];

    let mut gregorian_month: usize = 0;
    while gregorian_month < 13 && gregorian_day > gregorian_days_in_month[gregorian_month] {
        gregorian_day -= gregorian_days_in_month[gregorian_month];
        gregorian_month += 1;
    }

    (gregorian_year as i32, gregorian_month as u32, gregorian_day as u32)
}

/// Converts a Unix timestamp (seconds since 1970-01-01 UTC) to a Jalali date.
///
/// Returns `None` for negative timestamps or invalid calculations.
///
/// # Arguments
///
/// * `timestamp` - Unix timestamp in seconds.
///
/// # Returns
///
/// An `Option` containing (jalali_year, jalali_month, jalali_day) or `None`.
///
/// # Examples
///
/// ```
/// if let Some((jy, jm, jd)) = jalali_rs::unix_to_jalali(0) {
///     assert_eq!((jy, jm, jd), (1348, 10, 11));
/// }
/// if let Some((jy, jm, jd)) = jalali_rs::unix_to_jalali(1766806014) {
///     assert_eq!((jy, jm, jd), (1404, 10, 6));
/// }
/// ```
pub fn unix_to_jalali(timestamp: i64) -> Option<(i32, u32, u32)> {
    unix_to_gregorian(timestamp).map(|(gy, gm, gd)| {
        gregorian_to_jalali(gy, gm as usize, gd as i32)
    })
}

/// Converts a Jalali date to a Unix timestamp (seconds since 1970-01-01 UTC at midnight).
///
/// Returns `None` if the date is before 1970-01-01 or invalid.
///
/// # Arguments
///
/// * `jalali_year` - The Jalali year.
/// * `jalali_month` - The Jalali month (1-12).
/// * `jalali_day` - The Jalali day (1-31).
///
/// # Returns
///
/// An `Option` containing the Unix timestamp or `None`.
///
/// # Examples
///
/// ```
/// if let Some(ts) = jalali_rs::jalali_to_unix(1348, 10, 11) {
///     assert_eq!(ts, 0);
/// }
/// ```
pub fn jalali_to_unix(jalali_year: i32, jalali_month: u32, jalali_day: u32) -> Option<i64> {
    let (gy, gm, gd) = jalali_to_gregorian(jalali_year, jalali_month as usize, jalali_day as i32);
    gregorian_to_unix(gy, gm, gd)
}

/// Parses a Gregorian date string (e.g., "2025-12-27") and converts to Jalali string format.
///
/// Handles Persian/Arabic digits in input. Returns `None` for invalid formats.
///
/// # Arguments
///
/// * `date_str` - The date string.
/// * `separator` - The separator character (e.g., '-').
///
/// # Returns
///
/// An `Option` containing the Jalali date string (e.g., "1404-10-06") or `None`.
///
/// # Examples
///
/// ```
/// let result = jalali_rs::parse_gregorian_string_to_jalali_string("2025-12-27", '-');
/// assert_eq!(result, Some("1404-10-06".to_string()));
///
/// let result_persian = jalali_rs::parse_gregorian_string_to_jalali_string("۲۰۲۵-۱۲-۲۷", '-');
/// assert_eq!(result_persian, Some("1404-10-06".to_string()));
/// ```
pub fn parse_gregorian_string_to_jalali_string(date_str: &str, separator: char) -> Option<String> {
    let normalized = persian_or_arabic_digits_to_latin(date_str);
    let parts: Vec<&str> = normalized.split(separator).collect();
    if parts.len() != 3 {
        return None;
    }
    let gy = parts[0].parse::<i32>().ok()?;
    let gm = parts[1].parse::<usize>().ok()?;
    let gd = parts[2].parse::<i32>().ok()?;
    if gm < 1 || gm > 12 || gd < 1 || gd > 31 {
        return None; // basic validation
    }
    let (jy, jm, jd) = gregorian_to_jalali(gy, gm, gd);
    Some(format!("{:04}-{:02}-{:02}", jy, jm, jd))
}

/// Parses a Jalali date string (e.g., "1404-10-06") and converts to Gregorian string format.
///
/// Handles Persian/Arabic digits in input. Returns `None` for invalid formats.
///
/// # Arguments
///
/// * `date_str` - The date string.
/// * `separator` - The separator character (e.g., '-').
///
/// # Returns
///
/// An `Option` containing the Gregorian date string (e.g., "2025-12-27") or `None`.
///
/// # Examples
///
/// ```
/// let result = jalali_rs::parse_jalali_string_to_gregorian_string("1404-10-06", '-');
/// assert_eq!(result, Some("2025-12-27".to_string()));
///
/// let result_persian = jalali_rs::parse_jalali_string_to_gregorian_string("۱۴۰۴-۱۰-۰۶", '-');
/// assert_eq!(result_persian, Some("2025-12-27".to_string()));
/// ```
pub fn parse_jalali_string_to_gregorian_string(date_str: &str, separator: char) -> Option<String> {
    let normalized = persian_or_arabic_digits_to_latin(date_str);
    let parts: Vec<&str> = normalized.split(separator).collect();
    if parts.len() != 3 {
        return None;
    }
    let jy = parts[0].parse::<i32>().ok()?;
    let jm = parts[1].parse::<usize>().ok()?;
    let jd = parts[2].parse::<i32>().ok()?;
    if jm < 1 || jm > 12 || jd < 1 || jd > 31 {
        return None; // basic validation
    }
    let (gy, gm, gd) = jalali_to_gregorian(jy, jm, jd);
    Some(format!("{:04}-{:02}-{:02}", gy, gm, gd))
}

/// Converts Latin digits in a string to Persian digits.
///
/// Non-digit characters remain unchanged.
///
/// # Arguments
///
/// * `s` - The input string.
///
/// # Returns
///
/// A new string with Latin digits replaced by Persian equivalents.
///
/// # Examples
///
/// ```
/// let result = jalali_rs::latin_digits_to_persian("1400-12-10");
/// assert_eq!(result, "۱۴۰۰-۱۲-۱۰");
/// ```
pub fn latin_digits_to_persian(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_digit() {
                char::from_u32('۰' as u32 + (c as u32 - '0' as u32)).unwrap()
            } else {
                c
            }
        })
        .collect()
}

/// Converts Persian or Arabic digits in a string to Latin digits.
///
/// Non-digit characters remain unchanged. Handles both Persian (U+06F0-U+06F9) and Arabic (U+0660-U+0669) digits.
///
/// # Arguments
///
/// * `s` - The input string.
///
/// # Returns
///
/// A new string with Persian/Arabic digits replaced by Latin equivalents.
///
/// # Examples
///
/// ```
/// let result_persian = jalali_rs::persian_or_arabic_digits_to_latin("۱۴۰۰-۱۲-۱۰");
/// assert_eq!(result_persian, "1400-12-10");
///
/// let result_arabic = jalali_rs::persian_or_arabic_digits_to_latin("١٤٠٠-١٢-١٠");
/// assert_eq!(result_arabic, "1400-12-10");
/// ```
pub fn persian_or_arabic_digits_to_latin(s: &str) -> String {
    s.chars()
        .map(|c| {
            let u = c as u32;
            if (0x0660..=0x0669).contains(&u) {
                char::from_u32('0' as u32 + (u - 0x0660)).unwrap()
            } else if (0x06F0..=0x06F9).contains(&u) {
                char::from_u32('0' as u32 + (u - 0x06F0)).unwrap()
            } else {
                c
            }
        })
        .collect()
}

// Helper function to convert Gregorian date to Julian Day Number (JDN).
fn gregorian_to_jdn(year: i32, month: i32, day: i32) -> i64 {
    let a = (14 - month) / 12;
    let y = year as i64 + 4800 - a as i64;
    let m = month as i64 + 12 * a as i64 - 3;
    day as i64 + ((153 * m + 2) / 5) + 365 * y + (y / 4) - (y / 100) + (y / 400) - 32045
}

// Helper function to convert Julian Day Number (JDN) to Gregorian date.
fn jdn_to_gregorian(jdn: i64) -> (i32, u32, u32) {
    let a = jdn + 32044;
    let b = (4 * a + 3) / 146097;
    let c = a - (146097 * b) / 4;
    let d = (4 * c + 3) / 1461;
    let e = c - (1461 * d) / 4;
    let m = (5 * e + 2) / 153;
    let day = (e - (153 * m + 2) / 5 + 1) as u32;
    let month = (m + 3 - 12 * (m / 10)) as u32;
    let year = (100 * b + d - 4800 + (m / 10)) as i32;
    (year, month, day)
}

// Helper function to convert Unix timestamp to Gregorian date.
fn unix_to_gregorian(timestamp: i64) -> Option<(i32, u32, u32)> {
    if timestamp < 0 {
        return None;
    }
    let days = timestamp / 86_400;
    let jdn = 2_440_588 + days;
    let (year, month, day) = jdn_to_gregorian(jdn);
    Some((year, month, day))
}

// Helper function to convert Gregorian date to Unix timestamp.
fn gregorian_to_unix(year: i32, month: u32, day: u32) -> Option<i64> {
    let jdn = gregorian_to_jdn(year, month as i32, day as i32);
    let days = jdn - 2_440_588;
    if days < 0 {
        return None;
    }
    Some(days * 86_400)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gregorian_to_jalali() {
        let (jy, jm, jd) = gregorian_to_jalali(2023, 12, 27);
        assert_eq!(jy, 1402);
        assert_eq!(jm, 10);
        assert_eq!(jd, 6);

        let (jy, jm, jd) = gregorian_to_jalali(2025, 12, 27);
        assert_eq!(jy, 1404);
        assert_eq!(jm, 10);
        assert_eq!(jd, 6);
    }

    #[test]
    fn test_jalali_to_gregorian() {
        let (gy, gm, gd) = jalali_to_gregorian(1402, 10, 6);
        assert_eq!(gy, 2023);
        assert_eq!(gm, 12);
        assert_eq!(gd, 27);

        let (gy, gm, gd) = jalali_to_gregorian(1404, 10, 6);
        assert_eq!(gy, 2025);
        assert_eq!(gm, 12);
        assert_eq!(gd, 27);
    }

    #[test]
    fn test_unix_to_jalali() {
        let result = unix_to_jalali(0);
        assert_eq!(result, Some((1348, 10, 11)));
    }

    #[test]
    fn test_jalali_to_unix() {
        let result = jalali_to_unix(1348, 10, 11);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_parse_gregorian_string_to_jalali_string() {
        let result = parse_gregorian_string_to_jalali_string("2025-12-27", '-');
        assert_eq!(result, Some("1404-10-06".to_string()));

        let result_persian = parse_gregorian_string_to_jalali_string("۲۰۲۵-۱۲-۲۷", '-');
        assert_eq!(result_persian, Some("1404-10-06".to_string()));

        let invalid = parse_gregorian_string_to_jalali_string("invalid", '-');
        assert_eq!(invalid, None);
    }

    #[test]
    fn test_parse_jalali_string_to_gregorian_string() {
        let result = parse_jalali_string_to_gregorian_string("1404-10-06", '-');
        assert_eq!(result, Some("2025-12-27".to_string()));

        let result_persian = parse_jalali_string_to_gregorian_string("۱۴۰۴-۱۰-۰۶", '-');
        assert_eq!(result_persian, Some("2025-12-27".to_string()));

        let invalid = parse_jalali_string_to_gregorian_string("invalid", '-');
        assert_eq!(invalid, None);
    }

    #[test]
    fn test_latin_digits_to_persian() {
        let result = latin_digits_to_persian("1400-12-10");
        assert_eq!(result, "۱۴۰۰-۱۲-۱۰");
    }

    #[test]
    fn test_persian_or_arabic_digits_to_latin() {
        let result_persian = persian_or_arabic_digits_to_latin("۱۴۰۰-۱۲-۱۰");
        assert_eq!(result_persian, "1400-12-10");

        let result_arabic = persian_or_arabic_digits_to_latin("١٤٠٠-١٢-١٠");
        assert_eq!(result_arabic, "1400-12-10");

        let mixed = persian_or_arabic_digits_to_latin("۴٤۵٥۶٦"); // Persian 4, Arabic 4,5,6
        assert_eq!(mixed, "445566");
    }
}