// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc};
use std::collections::HashSet;
use std::error::Error;

/// Weekday mask indexed Mon=0 through Sun=6. `true` means the day is a weekend (non-working) day.
pub type WeekendMask = [bool; 7];

/// Maps an Excel NETWORKDAYS.INTL integer weekend code (1-7, 11-17) to a weekday mask.
pub fn parse_weekend_mask(code: i32) -> Result<WeekendMask, Box<dyn Error + Send + Sync>> {
    //                                Mon    Tue    Wed    Thu    Fri    Sat    Sun
    match code {
        1 => Ok([false, false, false, false, false, true, true]), // Sat, Sun
        2 => Ok([true, false, false, false, false, false, true]), // Sun, Mon
        3 => Ok([true, true, false, false, false, false, false]), // Mon, Tue
        4 => Ok([false, true, true, false, false, false, false]), // Tue, Wed
        5 => Ok([false, false, true, true, false, false, false]), // Wed, Thu
        6 => Ok([false, false, false, true, true, false, false]), // Thu, Fri
        7 => Ok([false, false, false, false, true, true, false]), // Fri, Sat
        11 => Ok([false, false, false, false, false, false, true]), // Sun only
        12 => Ok([true, false, false, false, false, false, false]), // Mon only
        13 => Ok([false, true, false, false, false, false, false]), // Tue only
        14 => Ok([false, false, true, false, false, false, false]), // Wed only
        15 => Ok([false, false, false, true, false, false, false]), // Thu only
        16 => Ok([false, false, false, false, true, false, false]), // Fri only
        17 => Ok([false, false, false, false, false, true, false]), // Sat only
        _ => Err(format!("NETWORKDAYS.INTL: Invalid weekend argument {code}").into()),
    }
}

/// Parses a 7-character weekend string where each character is '0' (workday) or '1' (weekend),
/// indexed from Monday (position 0) to Sunday (position 6).
pub fn parse_weekend_string(
    weekend_str: &str,
) -> Result<WeekendMask, Box<dyn Error + Send + Sync>> {
    if weekend_str.len() != 7 {
        return Err("NETWORKDAYS.INTL: Weekend string must be exactly 7 characters".into());
    }
    let mut mask = [false; 7];
    let mut all_ones = true;
    for (i, ch) in weekend_str.chars().enumerate() {
        match ch {
            '1' => mask[i] = true,
            '0' => {
                all_ones = false;
            }
            _ => return Err("NETWORKDAYS.INTL: Weekend string must contain only 0 and 1".into()),
        }
    }
    if all_ones {
        return Err("NETWORKDAYS.INTL: Weekend string cannot be all 1s (no workdays)".into());
    }
    Ok(mask)
}

/// Excel-compatible `NETWORKDAYS.INTL` that returns the number of whole working days
/// between two dates, excluding specified weekend days and optionally specified holidays.
/// - `start_date`: the starting date (inclusive).
/// - `end_date`: the ending date (inclusive).
/// - `weekend_mask`: a 7-element array (Mon=0..Sun=6) where `true` = weekend day.
/// - `holidays`: optional list of dates to exclude from the working day count.
///   Returns the count of working days (positive if end_date >= start_date, negative otherwise).
pub fn codcel_networkdays_intl(
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    weekend_mask: WeekendMask,
    holidays: Option<Vec<DateTime<Utc>>>,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    let start = start_date.date_naive();
    let end = end_date.date_naive();

    let (from, to, direction) = if start <= end {
        (start, end, 1i32)
    } else {
        (end, start, -1i32)
    };

    let holiday_set: HashSet<NaiveDate> = holidays
        .unwrap_or_default()
        .into_iter()
        .map(|d| d.date_naive())
        .collect();

    let mut count = 0i32;
    let mut current = from;
    while current <= to {
        let weekday_index = current.weekday().num_days_from_monday() as usize;
        if !weekend_mask[weekday_index] && !holiday_set.contains(&current) {
            count += 1;
        }
        current += Duration::days(1);
    }

    Ok(count * direction)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn create_date(year: i32, month: u32, day: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(year, month, day, 0, 0, 0)
            .single()
            .expect("valid test date")
    }

    /// Default weekend (code 1 = Sat/Sun) should match NETWORKDAYS.
    #[test]
    fn test_default_weekend_matches_networkdays() {
        // Jan 1 (Mon) to Jan 15 (Mon) 2024: 11 working days
        let mask = parse_weekend_mask(1).unwrap();
        let result = codcel_networkdays_intl(
            create_date(2024, 1, 1),
            create_date(2024, 1, 15),
            mask,
            None,
        )
        .unwrap();
        assert_eq!(result, 11);
    }

    #[test]
    fn test_weekend_code_2_sun_mon() {
        // Code 2: Sun + Mon are weekends
        // Jan 1 (Mon) to Jan 15 (Mon) 2024
        // Week 1: Mon=off, Tue-Sat=5 work. Week 2: Sun=off, Mon=off, Tue-Sat=5 work. Mon 15=off.
        // Days: 1(Mon-off), 2(Tue), 3(Wed), 4(Thu), 5(Fri), 6(Sat), 7(Sun-off),
        //        8(Mon-off), 9(Tue), 10(Wed), 11(Thu), 12(Fri), 13(Sat), 14(Sun-off), 15(Mon-off)
        // Working: 2,3,4,5,6, 9,10,11,12,13 = 10
        let mask = parse_weekend_mask(2).unwrap();
        let result = codcel_networkdays_intl(
            create_date(2024, 1, 1),
            create_date(2024, 1, 15),
            mask,
            None,
        )
        .unwrap();
        assert_eq!(result, 10);
    }

    #[test]
    fn test_weekend_code_11_sun_only() {
        // Code 11: Only Sunday is weekend
        // Jan 1 (Mon) to Jan 15 (Mon) 2024
        // Non-working: Jan 7 (Sun), Jan 14 (Sun) = 2 days off
        // 15 days - 2 = 13 working days
        let mask = parse_weekend_mask(11).unwrap();
        let result = codcel_networkdays_intl(
            create_date(2024, 1, 1),
            create_date(2024, 1, 15),
            mask,
            None,
        )
        .unwrap();
        assert_eq!(result, 13);
    }

    #[test]
    fn test_weekend_code_17_sat_only() {
        // Code 17: Only Saturday is weekend
        // Jan 1 (Mon) to Jan 15 (Mon) 2024
        // Non-working: Jan 6 (Sat), Jan 13 (Sat) = 2 days off
        // 15 days - 2 = 13 working days
        let mask = parse_weekend_mask(17).unwrap();
        let result = codcel_networkdays_intl(
            create_date(2024, 1, 1),
            create_date(2024, 1, 15),
            mask,
            None,
        )
        .unwrap();
        assert_eq!(result, 13);
    }

    #[test]
    fn test_weekend_string_sat_sun() {
        // "0000011" = Sat+Sun weekend, same as code 1
        let mask = parse_weekend_string("0000011").unwrap();
        let result = codcel_networkdays_intl(
            create_date(2024, 1, 1),
            create_date(2024, 1, 15),
            mask,
            None,
        )
        .unwrap();
        assert_eq!(result, 11);
    }

    #[test]
    fn test_weekend_string_fri_sat() {
        // "0000110" = Fri+Sat weekend (same as code 7)
        let mask = parse_weekend_string("0000110").unwrap();
        let result = codcel_networkdays_intl(
            create_date(2024, 1, 1),
            create_date(2024, 1, 15),
            mask,
            None,
        )
        .unwrap();
        // Non-working: Jan 5(Fri), 6(Sat), 12(Fri), 13(Sat) = 4 days off
        // 15 - 4 = 11
        assert_eq!(result, 11);
    }

    #[test]
    fn test_weekend_string_no_weekends() {
        // "0000000" = no weekend days (all days are workdays)
        let mask = parse_weekend_string("0000000").unwrap();
        let result = codcel_networkdays_intl(
            create_date(2024, 1, 1),
            create_date(2024, 1, 15),
            mask,
            None,
        )
        .unwrap();
        assert_eq!(result, 15);
    }

    #[test]
    fn test_with_holidays() {
        let mask = parse_weekend_mask(1).unwrap(); // Sat/Sun
        let result = codcel_networkdays_intl(
            create_date(2024, 1, 1),
            create_date(2024, 1, 15),
            mask,
            Some(vec![create_date(2024, 1, 1), create_date(2024, 1, 2)]),
        )
        .unwrap();
        // 11 working days - 2 holidays = 9
        assert_eq!(result, 9);
    }

    #[test]
    fn test_with_holidays_on_weekend() {
        let mask = parse_weekend_mask(1).unwrap();
        let result = codcel_networkdays_intl(
            create_date(2024, 1, 1),
            create_date(2024, 1, 15),
            mask,
            Some(vec![create_date(2024, 1, 6)]), // Jan 6 is Saturday
        )
        .unwrap();
        // Holiday on weekend doesn't reduce count
        assert_eq!(result, 11);
    }

    #[test]
    fn test_negative_direction() {
        let mask = parse_weekend_mask(1).unwrap();
        let result = codcel_networkdays_intl(
            create_date(2024, 1, 15),
            create_date(2024, 1, 1),
            mask,
            None,
        )
        .unwrap();
        assert_eq!(result, -11);
    }

    #[test]
    fn test_same_day_weekday() {
        let mask = parse_weekend_mask(1).unwrap();
        let result = codcel_networkdays_intl(
            create_date(2024, 1, 1), // Monday
            create_date(2024, 1, 1),
            mask,
            None,
        )
        .unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_same_day_weekend() {
        let mask = parse_weekend_mask(1).unwrap();
        let result = codcel_networkdays_intl(
            create_date(2024, 1, 6), // Saturday
            create_date(2024, 1, 6),
            mask,
            None,
        )
        .unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_invalid_weekend_code() {
        assert!(parse_weekend_mask(0).is_err());
        assert!(parse_weekend_mask(8).is_err());
        assert!(parse_weekend_mask(9).is_err());
        assert!(parse_weekend_mask(10).is_err());
        assert!(parse_weekend_mask(18).is_err());
    }

    #[test]
    fn test_invalid_weekend_string_wrong_length() {
        assert!(parse_weekend_string("000001").is_err());
        assert!(parse_weekend_string("00000111").is_err());
    }

    #[test]
    fn test_invalid_weekend_string_bad_chars() {
        assert!(parse_weekend_string("000002a").is_err());
    }

    #[test]
    fn test_invalid_weekend_string_all_ones() {
        assert!(parse_weekend_string("1111111").is_err());
    }
}
