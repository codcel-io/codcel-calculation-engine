// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use chrono::{NaiveTime, Timelike};
use std::error::Error;

/// Excel-compatible `HOUR` that extracts the hour from a time value.
/// - `time`: a time value.
///   Returns the hour component as an integer (0–23).
pub fn codcel_hour(time: NaiveTime) -> Result<i32, Box<dyn Error + Send + Sync>> {
    Ok(time.hour() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_time(hour: u32, minute: u32, second: u32) -> NaiveTime {
        NaiveTime::from_hms_opt(hour, minute, second).unwrap()
    }

    #[test]
    fn test_hour_morning() {
        // =HOUR("09:30:00") in US format
        // =HOUR("09:30:00") in German format
        let time = create_time(9, 30, 0);
        let result = codcel_hour(time).unwrap();
        println!("{result}");
        assert_eq!(result, 9);
    }

    #[test]
    fn test_hour_noon() {
        // =HOUR("12:00:00") in US format
        // =HOUR("12:00:00") in German format
        let time = create_time(12, 0, 0);
        let result = codcel_hour(time).unwrap();
        println!("{result}");
        assert_eq!(result, 12);
    }

    #[test]
    fn test_hour_afternoon() {
        // =HOUR("15:45:30") in US format
        // =HOUR("15:45:30") in German format
        let time = create_time(15, 45, 30);
        let result = codcel_hour(time).unwrap();
        println!("{result}");
        assert_eq!(result, 15);
    }

    #[test]
    fn test_hour_midnight() {
        // =HOUR("00:00:00") in US format
        // =HOUR("00:00:00") in German format
        let time = create_time(0, 0, 0);
        let result = codcel_hour(time).unwrap();
        println!("{result}");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_hour_late_night() {
        // =HOUR("23:59:59") in US format
        // =HOUR("23:59:59") in German format
        let time = create_time(23, 59, 59);
        let result = codcel_hour(time).unwrap();
        println!("{result}");
        assert_eq!(result, 23);
    }
}
