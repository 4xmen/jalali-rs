// src/lib.rs
//! # Jalali-rs
//!
//! A simple crate for converting dates between the Gregorian and Jalali (Persian) calendars.
//!
//! This crate provides two main functions:
//! - `gregorian_to_jalali`: Converts a Gregorian date to a Jalali date.
//! - `jalali_to_gregorian`: Converts a Jalali date to a Gregorian date.
//!
//! ## Usage
//!
//! ```rust
//! use jalali_rs::{gregorian_to_jalali, jalali_to_gregorian};
//!
//! let (jy, jm, jd) = gregorian_to_jalali(2023, 12, 27);
//! println!("Jalali date: {}-{}-{}", jy, jm, jd);
//!
//! let (gy, gm, gd) = jalali_to_gregorian(1402, 10, 6);
//! println!("Gregorian date: {}-{}-{}", gy, gm, gd);
//! ```
//!
//! Note: This implementation assumes valid input dates and does not perform validation.
//! You may want to add input validation in your application.

/// Converts a Gregorian date to a Jalali (Persian) date.
///
/// # Arguments
///
/// * `gregorian_year` - The Gregorian year (e.g., 2023).
/// * `gregorian_month` - The Gregorian month (1-12).
/// * `gregorian_day` - The Gregorian day (1-31).
///
/// # Returns
///
/// A tuple containing (jalali_year, jalali_month, jalali_day).
pub fn gregorian_to_jalali(
    gregorian_year: i32,
    gregorian_month: usize,
    gregorian_day: i32,
) -> (i32, u32, u32) {
    // cumulative days at the end of each Gregorian month (non-leap year, adjusted later)
    let gregorian_cumulative_days = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

    // adjust year for leap year calculation if month is after February
    let adjusted_year = if gregorian_month > 2 {
        gregorian_year + 1
    } else {
        gregorian_year
    };

    // calculate total days from a fixed epoch, including leap year adjustments
    let mut total_days = 355666
        + (365 * gregorian_year)
        + ((adjusted_year + 3) / 4) as i32
        - ((adjusted_year + 99) / 100) as i32
        + ((adjusted_year + 399) / 400) as i32
        + gregorian_day
        + gregorian_cumulative_days[gregorian_month - 1];

    // compute Jalali year using divisions based on Jalali cycle lengths
    let mut jalali_year = -1595 + (33 * (total_days / 12053));
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

    (jalali_year, jalali_month, jalali_day)
}

/// Converts a Jalali (Persian) date to a Gregorian date.
///
/// # Arguments
///
/// * `jalali_year` - The Jalali year (e.g., 1402).
/// * `jalali_month` - The Jalali month (1-12).
/// * `jalali_day` - The Jalali day (1-31).
///
/// # Returns
///
/// A tuple containing (gregorian_year, gregorian_month, gregorian_day).
pub fn jalali_to_gregorian(
    mut jalali_year: i32,
    jalali_month: usize,
    jalali_day: i32,
) -> (i32, u32, u32) {
    jalali_year += 1595;

    // calculate total days from a fixed epoch, including Jalali leap adjustments
    let mut total_days = -355668
        + (365 * jalali_year)
        + ((jalali_year / 33) * 8)
        + (((jalali_year % 33) + 3) / 4)
        + jalali_day
        + if jalali_month < 7 {
        (jalali_month as i32 - 1) * 31
    } else {
        ((jalali_month as i32 - 7) * 30) + 186
    };

    // compute Gregorian year using divisions based on Gregorian cycle lengths
    let mut gregorian_year = 400 * (total_days / 146097);
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
    let mut gregorian_day = total_days + 1;

    // array of days in each Gregorian month, adjusting February for leap year
    let is_leap_year = (gregorian_year % 4 == 0 && gregorian_year % 100 != 0)
        || (gregorian_year % 400 == 0);
    let gregorian_days_in_month = [
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

    let mut gregorian_month = 0;
    while gregorian_month < 13 && gregorian_day > gregorian_days_in_month[gregorian_month] {
        gregorian_day -= gregorian_days_in_month[gregorian_month];
        gregorian_month += 1;
    }

    (gregorian_year, gregorian_month as u32, gregorian_day as u32)
}
