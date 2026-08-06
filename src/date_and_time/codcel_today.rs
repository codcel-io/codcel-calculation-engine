// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use chrono::{DateTime, Datelike, NaiveDate, TimeZone, Utc};
use std::error::Error;

/// Excel-compatible `TODAY` that returns the current date.
///   Returns today's date as a UTC datetime with the time set to midnight (00:00:00).
pub fn codcel_today() -> Result<DateTime<Utc>, Box<dyn Error + Send + Sync>> {
    let now = Utc::now();
    let naive_date = NaiveDate::from_ymd_opt(now.year(), now.month(), now.day())
        .ok_or("Failed to construct TODAY")?;

    // Assuming `midnight` is a NaiveDateTime
    let midnight = naive_date
        .and_hms_opt(0, 0, 0)
        .ok_or("Failed to construct DateTime")?;
    let datetime = Utc.from_utc_datetime(&midnight);
    Ok(datetime)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Timelike;

    #[test]
    fn test_today() {
        // =TODAY() in US format
        // =TODAY() in German format
        let now = Utc::now();
        let result = codcel_today().unwrap();
        println!("{result}");

        // Check that the result has the same date as now
        assert_eq!(result.year(), now.year());
        assert_eq!(result.month(), now.month());
        assert_eq!(result.day(), now.day());

        // Check that the time is set to midnight
        assert_eq!(result.hour(), 0);
        assert_eq!(result.minute(), 0);
        assert_eq!(result.second(), 0);
        assert_eq!(result.nanosecond(), 0);
    }

    #[test]
    fn test_today_is_date_only() {
        // =TODAY() in US format
        // =TODAY() in German format
        let result = codcel_today().unwrap();
        println!("{result}");

        // Check that the time is set to midnight
        assert_eq!(result.hour(), 0);
        assert_eq!(result.minute(), 0);
        assert_eq!(result.second(), 0);
        assert_eq!(result.nanosecond(), 0);
    }
}
