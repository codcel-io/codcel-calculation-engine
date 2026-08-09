// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc};
use std::collections::HashSet;
use std::error::Error;

use super::codcel_networkdays_intl::WeekendMask;

/// Excel-compatible `WORKDAY.INTL` that returns the date that is a specified number
/// of working days before or after a start date, with customizable weekend days.
/// - `start_date`: the starting date (not counted as a work day).
/// - `days`: number of working days to advance (positive) or go back (negative).
/// - `weekend_mask`: a 7-element array (Mon=0..Sun=6) where `true` = weekend day.
/// - `holidays`: optional list of dates to exclude from working days.
///   Returns the resulting date as `DateTime<Utc>`.
pub fn codcel_workday_intl(
    start_date: DateTime<Utc>,
    days: i32,
    weekend_mask: WeekendMask,
    holidays: Option<Vec<DateTime<Utc>>>,
) -> Result<DateTime<Utc>, Box<dyn Error + Send + Sync>> {
    if days == 0 {
        return Ok(start_date);
    }

    let holiday_set: HashSet<NaiveDate> = holidays
        .unwrap_or_default()
        .into_iter()
        .map(|d| d.date_naive())
        .collect();

    let direction: i64 = if days > 0 { 1 } else { -1 };
    let mut remaining = days.unsigned_abs();
    let mut current = start_date;

    while remaining > 0 {
        current += Duration::days(direction);
        let weekday_index = current.weekday().num_days_from_monday() as usize;
        if !weekend_mask[weekday_index] && !holiday_set.contains(&current.date_naive()) {
            remaining -= 1;
        }
    }

    Ok(current)
}

#[cfg(test)]
mod tests {
    use super::super::codcel_networkdays_intl::{parse_weekend_mask, parse_weekend_string};
    use super::*;
    use chrono::TimeZone;

    fn create_date(year: i32, month: u32, day: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap()
    }

    #[test]
    fn test_default_weekend_basic_forward() {
        // =WORKDAY.INTL(DATE(2024, 1, 1), 1, 1)
        // Jan 1 (Mon) + 1 working day = Jan 2 (Tue)
        let mask = parse_weekend_mask(1).unwrap();
        let result = codcel_workday_intl(create_date(2024, 1, 1), 1, mask, None).unwrap();
        assert_eq!(result, create_date(2024, 1, 2));
    }

    #[test]
    fn test_default_weekend_skip_weekend() {
        // =WORKDAY.INTL(DATE(2024, 1, 5), 1, 1)
        // Jan 5 (Fri) + 1 working day = Jan 8 (Mon)
        let mask = parse_weekend_mask(1).unwrap();
        let result = codcel_workday_intl(create_date(2024, 1, 5), 1, mask, None).unwrap();
        assert_eq!(result, create_date(2024, 1, 8));
    }

    #[test]
    fn test_default_weekend_multiple_days() {
        // =WORKDAY.INTL(DATE(2024, 1, 1), 5, 1)
        // Jan 1 (Mon) + 5 working days = Jan 8 (Mon)
        let mask = parse_weekend_mask(1).unwrap();
        let result = codcel_workday_intl(create_date(2024, 1, 1), 5, mask, None).unwrap();
        assert_eq!(result, create_date(2024, 1, 8));
    }

    #[test]
    fn test_zero_days() {
        let mask = parse_weekend_mask(1).unwrap();
        let result = codcel_workday_intl(create_date(2024, 1, 1), 0, mask, None).unwrap();
        assert_eq!(result, create_date(2024, 1, 1));
    }

    #[test]
    fn test_negative_days() {
        // =WORKDAY.INTL(DATE(2024, 1, 8), -1, 1)
        // Jan 8 (Mon) - 1 working day = Jan 5 (Fri)
        let mask = parse_weekend_mask(1).unwrap();
        let result = codcel_workday_intl(create_date(2024, 1, 8), -1, mask, None).unwrap();
        assert_eq!(result, create_date(2024, 1, 5));
    }

    #[test]
    fn test_negative_skip_weekend() {
        // =WORKDAY.INTL(DATE(2024, 1, 8), -5, 1)
        // Jan 8 (Mon) - 5 working days = Jan 1 (Mon)
        let mask = parse_weekend_mask(1).unwrap();
        let result = codcel_workday_intl(create_date(2024, 1, 8), -5, mask, None).unwrap();
        assert_eq!(result, create_date(2024, 1, 1));
    }

    #[test]
    fn test_weekend_code_2_sun_mon() {
        // Code 2: Sun + Mon are weekends
        // =WORKDAY.INTL(DATE(2024, 1, 5), 1, 2)
        // Jan 5 (Fri) + 1 working day = Jan 6 (Sat) — Saturday is a workday with code 2
        let mask = parse_weekend_mask(2).unwrap();
        let result = codcel_workday_intl(create_date(2024, 1, 5), 1, mask, None).unwrap();
        assert_eq!(result, create_date(2024, 1, 6));
    }

    #[test]
    fn test_weekend_code_2_skip_sun_mon() {
        // Code 2: Sun + Mon are weekends
        // =WORKDAY.INTL(DATE(2024, 1, 6), 1, 2)
        // Jan 6 (Sat) + 1 working day: skip Jan 7 (Sun), skip Jan 8 (Mon) = Jan 9 (Tue)
        let mask = parse_weekend_mask(2).unwrap();
        let result = codcel_workday_intl(create_date(2024, 1, 6), 1, mask, None).unwrap();
        assert_eq!(result, create_date(2024, 1, 9));
    }

    #[test]
    fn test_weekend_code_7_fri_sat() {
        // Code 7: Fri + Sat are weekends
        // =WORKDAY.INTL(DATE(2024, 1, 4), 1, 7)
        // Jan 4 (Thu) + 1 working day: skip Jan 5 (Fri), skip Jan 6 (Sat) = Jan 7 (Sun)
        let mask = parse_weekend_mask(7).unwrap();
        let result = codcel_workday_intl(create_date(2024, 1, 4), 1, mask, None).unwrap();
        assert_eq!(result, create_date(2024, 1, 7));
    }

    #[test]
    fn test_weekend_code_11_sun_only() {
        // Code 11: Only Sunday is weekend
        // =WORKDAY.INTL(DATE(2024, 1, 6), 1, 11)
        // Jan 6 (Sat) + 1 working day: skip Jan 7 (Sun) = Jan 8 (Mon)
        let mask = parse_weekend_mask(11).unwrap();
        let result = codcel_workday_intl(create_date(2024, 1, 6), 1, mask, None).unwrap();
        assert_eq!(result, create_date(2024, 1, 8));
    }

    #[test]
    fn test_weekend_code_17_sat_only() {
        // Code 17: Only Saturday is weekend
        // =WORKDAY.INTL(DATE(2024, 1, 5), 1, 17)
        // Jan 5 (Fri) + 1 working day: skip Jan 6 (Sat) = Jan 7 (Sun)
        let mask = parse_weekend_mask(17).unwrap();
        let result = codcel_workday_intl(create_date(2024, 1, 5), 1, mask, None).unwrap();
        assert_eq!(result, create_date(2024, 1, 7));
    }

    #[test]
    fn test_weekend_string_sat_sun() {
        // "0000011" = Sat+Sun weekend, same as code 1
        let mask = parse_weekend_string("0000011").unwrap();
        let result = codcel_workday_intl(create_date(2024, 1, 5), 1, mask, None).unwrap();
        assert_eq!(result, create_date(2024, 1, 8));
    }

    #[test]
    fn test_weekend_string_no_weekends() {
        // "0000000" = no weekend days (all days are workdays)
        let mask = parse_weekend_string("0000000").unwrap();
        let result = codcel_workday_intl(create_date(2024, 1, 5), 1, mask, None).unwrap();
        assert_eq!(result, create_date(2024, 1, 6));
    }

    #[test]
    fn test_with_holidays() {
        // =WORKDAY.INTL(DATE(2024, 1, 1), 1, 1, {DATE(2024, 1, 2)})
        // Jan 1 (Mon) + 1 working day, but Jan 2 is holiday = Jan 3 (Wed)
        let mask = parse_weekend_mask(1).unwrap();
        let result = codcel_workday_intl(
            create_date(2024, 1, 1),
            1,
            mask,
            Some(vec![create_date(2024, 1, 2)]),
        )
        .unwrap();
        assert_eq!(result, create_date(2024, 1, 3));
    }

    #[test]
    fn test_holiday_on_weekend() {
        // Holiday on a Saturday should not affect the result
        let mask = parse_weekend_mask(1).unwrap();
        let result = codcel_workday_intl(
            create_date(2024, 1, 5),
            1,
            mask,
            Some(vec![create_date(2024, 1, 6)]),
        )
        .unwrap();
        assert_eq!(result, create_date(2024, 1, 8));
    }

    #[test]
    fn test_start_on_weekend() {
        // =WORKDAY.INTL(DATE(2024, 1, 6), 1, 1)
        // Jan 6 (Sat) + 1 working day = Jan 8 (Mon)
        let mask = parse_weekend_mask(1).unwrap();
        let result = codcel_workday_intl(create_date(2024, 1, 6), 1, mask, None).unwrap();
        assert_eq!(result, create_date(2024, 1, 8));
    }

    #[test]
    fn test_large_days() {
        // =WORKDAY.INTL(DATE(2024, 1, 1), 10, 1)
        // Jan 1 (Mon) + 10 working days = Jan 15 (Mon)
        let mask = parse_weekend_mask(1).unwrap();
        let result = codcel_workday_intl(create_date(2024, 1, 1), 10, mask, None).unwrap();
        assert_eq!(result, create_date(2024, 1, 15));
    }

    #[test]
    fn test_negative_with_holidays() {
        // =WORKDAY.INTL(DATE(2024, 1, 8), -1, 1, {DATE(2024, 1, 5)})
        // Jan 8 (Mon) - 1 working day, with Jan 5 (Fri) as holiday = Jan 4 (Thu)
        let mask = parse_weekend_mask(1).unwrap();
        let result = codcel_workday_intl(
            create_date(2024, 1, 8),
            -1,
            mask,
            Some(vec![create_date(2024, 1, 5)]),
        )
        .unwrap();
        assert_eq!(result, create_date(2024, 1, 4));
    }

    #[test]
    fn test_multiple_holidays() {
        // =WORKDAY.INTL(DATE(2024, 1, 1), 5, 1, {DATE(2024, 1, 2), DATE(2024, 1, 3)})
        // Working days: Jan 4 (Thu), Jan 5 (Fri), Jan 8 (Mon), Jan 9 (Tue), Jan 10 (Wed)
        let mask = parse_weekend_mask(1).unwrap();
        let result = codcel_workday_intl(
            create_date(2024, 1, 1),
            5,
            mask,
            Some(vec![create_date(2024, 1, 2), create_date(2024, 1, 3)]),
        )
        .unwrap();
        assert_eq!(result, create_date(2024, 1, 10));
    }

    #[test]
    fn test_custom_weekend_with_holidays() {
        // Code 7: Fri + Sat are weekends
        // =WORKDAY.INTL(DATE(2024, 1, 1), 5, 7, {DATE(2024, 1, 2)})
        // Jan 1 (Mon) + 5 working days with Fri/Sat weekends, Jan 2 holiday
        // Jan 2 (Tue-holiday), Jan 3 (Wed-1), Jan 4 (Thu-2), Jan 7 (Sun-3), Jan 8 (Mon-4), Jan 9 (Tue-5)
        let mask = parse_weekend_mask(7).unwrap();
        let result = codcel_workday_intl(
            create_date(2024, 1, 1),
            5,
            mask,
            Some(vec![create_date(2024, 1, 2)]),
        )
        .unwrap();
        assert_eq!(result, create_date(2024, 1, 9));
    }
}
