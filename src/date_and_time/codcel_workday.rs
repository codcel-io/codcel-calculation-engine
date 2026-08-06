// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc};
use std::collections::HashSet;
use std::error::Error;

/// Excel-compatible `WORKDAY` that returns the date that is a specified number
/// of working days before or after a start date.
/// - `start_date`: the starting date (not counted as a work day).
/// - `days`: number of working days to advance (positive) or go back (negative).
/// - `holidays`: optional list of dates to exclude from working days.
///   Returns the resulting date as `DateTime<Utc>`.
pub fn codcel_workday(
    start_date: DateTime<Utc>,
    days: i32,
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
        let weekday = current.weekday();
        if weekday != chrono::Weekday::Sat
            && weekday != chrono::Weekday::Sun
            && !holiday_set.contains(&current.date_naive())
        {
            remaining -= 1;
        }
    }

    Ok(current)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn create_date(year: i32, month: u32, day: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap()
    }

    #[test]
    fn test_workday_basic_forward() {
        // =WORKDAY(DATE(2024, 1, 1), 1)
        // Jan 1 (Mon) + 1 working day = Jan 2 (Tue)
        let result = codcel_workday(create_date(2024, 1, 1), 1, None).unwrap();
        assert_eq!(result, create_date(2024, 1, 2));
    }

    #[test]
    fn test_workday_skip_weekend() {
        // =WORKDAY(DATE(2024, 1, 5), 1)
        // Jan 5 (Fri) + 1 working day = Jan 8 (Mon)
        let result = codcel_workday(create_date(2024, 1, 5), 1, None).unwrap();
        assert_eq!(result, create_date(2024, 1, 8));
    }

    #[test]
    fn test_workday_multiple_days() {
        // =WORKDAY(DATE(2024, 1, 1), 5)
        // Jan 1 (Mon) + 5 working days = Jan 8 (Mon)
        let result = codcel_workday(create_date(2024, 1, 1), 5, None).unwrap();
        assert_eq!(result, create_date(2024, 1, 8));
    }

    #[test]
    fn test_workday_negative() {
        // =WORKDAY(DATE(2024, 1, 8), -1)
        // Jan 8 (Mon) - 1 working day = Jan 5 (Fri)
        let result = codcel_workday(create_date(2024, 1, 8), -1, None).unwrap();
        assert_eq!(result, create_date(2024, 1, 5));
    }

    #[test]
    fn test_workday_negative_skip_weekend() {
        // =WORKDAY(DATE(2024, 1, 8), -5)
        // Jan 8 (Mon) - 5 working days = Jan 1 (Mon)
        let result = codcel_workday(create_date(2024, 1, 8), -5, None).unwrap();
        assert_eq!(result, create_date(2024, 1, 1));
    }

    #[test]
    fn test_workday_zero() {
        // =WORKDAY(DATE(2024, 1, 1), 0)
        // Returns start_date unchanged
        let result = codcel_workday(create_date(2024, 1, 1), 0, None).unwrap();
        assert_eq!(result, create_date(2024, 1, 1));
    }

    #[test]
    fn test_workday_with_holidays() {
        // =WORKDAY(DATE(2024, 1, 1), 1, {DATE(2024, 1, 2)})
        // Jan 1 (Mon) + 1 working day, but Jan 2 is holiday = Jan 3 (Wed)
        let result = codcel_workday(
            create_date(2024, 1, 1),
            1,
            Some(vec![create_date(2024, 1, 2)]),
        )
        .unwrap();
        assert_eq!(result, create_date(2024, 1, 3));
    }

    #[test]
    fn test_workday_holiday_on_weekend() {
        // Holiday on a Saturday should not affect the result
        // =WORKDAY(DATE(2024, 1, 5), 1, {DATE(2024, 1, 6)})
        // Jan 5 (Fri) + 1 working day = Jan 8 (Mon), Jan 6 (Sat) is already skipped
        let result = codcel_workday(
            create_date(2024, 1, 5),
            1,
            Some(vec![create_date(2024, 1, 6)]),
        )
        .unwrap();
        assert_eq!(result, create_date(2024, 1, 8));
    }

    #[test]
    fn test_workday_start_on_weekend() {
        // =WORKDAY(DATE(2024, 1, 6), 1)
        // Jan 6 (Sat) + 1 working day = Jan 8 (Mon)
        let result = codcel_workday(create_date(2024, 1, 6), 1, None).unwrap();
        assert_eq!(result, create_date(2024, 1, 8));
    }

    #[test]
    fn test_workday_large_days() {
        // =WORKDAY(DATE(2024, 1, 1), 10)
        // Jan 1 (Mon) + 10 working days = Jan 15 (Mon)
        let result = codcel_workday(create_date(2024, 1, 1), 10, None).unwrap();
        assert_eq!(result, create_date(2024, 1, 15));
    }

    #[test]
    fn test_workday_multiple_holidays() {
        // =WORKDAY(DATE(2024, 1, 1), 5, {DATE(2024, 1, 2), DATE(2024, 1, 3)})
        // Jan 1 (Mon) + 5 working days, with Jan 2 (Tue) and Jan 3 (Wed) as holidays
        // Working days: Jan 4 (Thu), Jan 5 (Fri), Jan 8 (Mon), Jan 9 (Tue), Jan 10 (Wed)
        let result = codcel_workday(
            create_date(2024, 1, 1),
            5,
            Some(vec![create_date(2024, 1, 2), create_date(2024, 1, 3)]),
        )
        .unwrap();
        assert_eq!(result, create_date(2024, 1, 10));
    }

    #[test]
    fn test_workday_negative_with_holidays() {
        // =WORKDAY(DATE(2024, 1, 8), -1, {DATE(2024, 1, 5)})
        // Jan 8 (Mon) - 1 working day, with Jan 5 (Fri) as holiday = Jan 4 (Thu)
        let result = codcel_workday(
            create_date(2024, 1, 8),
            -1,
            Some(vec![create_date(2024, 1, 5)]),
        )
        .unwrap();
        assert_eq!(result, create_date(2024, 1, 4));
    }
}
