// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use chrono::{NaiveTime, Timelike};
use std::error::Error;

/// Excel-compatible `MINUTE` that extracts the minute from a time value.
/// - `time`: a time value.
///   Returns the minute component as an integer (0–59).
pub fn codcel_minute(time: NaiveTime) -> Result<i32, Box<dyn Error + Send + Sync>> {
    Ok(time.minute() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_time(hour: u32, minute: u32, second: u32) -> NaiveTime {
        NaiveTime::from_hms_opt(hour, minute, second).unwrap()
    }

    #[test]
    fn test_minute_zero() {
        // =MINUTE("12:00:30") in US format
        // =MINUTE("12:00:30") in German format
        let time = create_time(12, 0, 30);
        let result = codcel_minute(time).unwrap();
        println!("{result}");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_minute_single_digit() {
        // =MINUTE("09:05:00") in US format
        // =MINUTE("09:05:00") in German format
        let time = create_time(9, 5, 0);
        let result = codcel_minute(time).unwrap();
        println!("{result}");
        assert_eq!(result, 5);
    }

    #[test]
    fn test_minute_double_digit() {
        // =MINUTE("15:45:30") in US format
        // =MINUTE("15:45:30") in German format
        let time = create_time(15, 45, 30);
        let result = codcel_minute(time).unwrap();
        println!("{result}");
        assert_eq!(result, 45);
    }

    #[test]
    fn test_minute_max() {
        // =MINUTE("23:59:59") in US format
        // =MINUTE("23:59:59") in German format
        let time = create_time(23, 59, 59);
        let result = codcel_minute(time).unwrap();
        println!("{result}");
        assert_eq!(result, 59);
    }

    #[test]
    fn test_minute_midnight() {
        // =MINUTE("00:00:00") in US format
        // =MINUTE("00:00:00") in German format
        let time = create_time(0, 0, 0);
        let result = codcel_minute(time).unwrap();
        println!("{result}");
        assert_eq!(result, 0);
    }
}
