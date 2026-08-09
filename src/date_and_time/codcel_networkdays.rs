// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc};
use std::collections::HashSet;
use std::error::Error;

/// Excel-compatible `NETWORKDAYS` that returns the number of whole working days
/// between two dates, excluding weekends (Saturday and Sunday) and optionally
/// specified holidays.
/// - `start_date`: the starting date (inclusive).
/// - `end_date`: the ending date (inclusive).
/// - `holidays`: optional list of dates to exclude from the working day count.
///   Returns the count of working days (positive if end_date >= start_date, negative otherwise).
pub fn codcel_networkdays(
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
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
        let weekday = current.weekday();
        if weekday != chrono::Weekday::Sat
            && weekday != chrono::Weekday::Sun
            && !holiday_set.contains(&current)
        {
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
        Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap()
    }

    #[test]
    fn test_networkdays_basic() {
        // =NETWORKDAYS(DATE(2024, 1, 1), DATE(2024, 1, 15))
        // Jan 1 (Mon) to Jan 15 (Mon): 11 working days
        let result =
            codcel_networkdays(create_date(2024, 1, 1), create_date(2024, 1, 15), None).unwrap();
        assert_eq!(result, 11);
    }

    #[test]
    fn test_networkdays_with_holidays() {
        // =NETWORKDAYS(DATE(2024, 1, 1), DATE(2024, 1, 15), {DATE(2024, 1, 1)})
        // 11 working days minus 1 holiday = 10
        let result = codcel_networkdays(
            create_date(2024, 1, 1),
            create_date(2024, 1, 15),
            Some(vec![create_date(2024, 1, 1)]),
        )
        .unwrap();
        assert_eq!(result, 10);
    }

    #[test]
    fn test_networkdays_january_2023() {
        // =NETWORKDAYS(DATE(2023, 1, 1), DATE(2023, 1, 31))
        // Jan 1 2023 is Sunday, Jan 31 is Tuesday: 22 working days
        let result =
            codcel_networkdays(create_date(2023, 1, 1), create_date(2023, 1, 31), None).unwrap();
        assert_eq!(result, 22);
    }

    #[test]
    fn test_networkdays_same_day_weekday() {
        // Same day, a Monday
        let result =
            codcel_networkdays(create_date(2024, 1, 1), create_date(2024, 1, 1), None).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_networkdays_same_day_weekend() {
        // Same day, a Saturday (Jan 6, 2024)
        let result =
            codcel_networkdays(create_date(2024, 1, 6), create_date(2024, 1, 6), None).unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_networkdays_negative_direction() {
        // start > end should return negative
        // =NETWORKDAYS(DATE(2024, 1, 15), DATE(2024, 1, 1))
        let result =
            codcel_networkdays(create_date(2024, 1, 15), create_date(2024, 1, 1), None).unwrap();
        assert_eq!(result, -11);
    }

    #[test]
    fn test_networkdays_holiday_on_weekend() {
        // Holiday on a Saturday should not reduce the count
        // Jan 6, 2024 is a Saturday
        let result = codcel_networkdays(
            create_date(2024, 1, 1),
            create_date(2024, 1, 15),
            Some(vec![create_date(2024, 1, 6)]),
        )
        .unwrap();
        assert_eq!(result, 11); // Same as without holiday
    }

    #[test]
    fn test_networkdays_multiple_holidays() {
        // =NETWORKDAYS(DATE(2023, 1, 1), DATE(2023, 1, 31), {DATE(2023, 1, 16), DATE(2023, 1, 26)})
        // 22 working days minus 2 holidays = 20
        let result = codcel_networkdays(
            create_date(2023, 1, 1),
            create_date(2023, 1, 31),
            Some(vec![create_date(2023, 1, 16), create_date(2023, 1, 26)]),
        )
        .unwrap();
        assert_eq!(result, 20);
    }

    #[test]
    fn test_networkdays_duplicate_holidays() {
        // Duplicate holidays should only count once
        let result = codcel_networkdays(
            create_date(2024, 1, 1),
            create_date(2024, 1, 15),
            Some(vec![create_date(2024, 1, 1), create_date(2024, 1, 1)]),
        )
        .unwrap();
        assert_eq!(result, 10);
    }

    #[test]
    fn test_networkdays_full_week() {
        // Monday to Friday = 5 working days
        // Jan 8, 2024 (Mon) to Jan 12, 2024 (Fri)
        let result =
            codcel_networkdays(create_date(2024, 1, 8), create_date(2024, 1, 12), None).unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn test_networkdays_weekend_only() {
        // Saturday to Sunday = 0 working days
        // Jan 6, 2024 (Sat) to Jan 7, 2024 (Sun)
        let result =
            codcel_networkdays(create_date(2024, 1, 6), create_date(2024, 1, 7), None).unwrap();
        assert_eq!(result, 0);
    }
}
