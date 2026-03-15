// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use chrono::{NaiveTime, Timelike};
use std::error::Error;

/// Excel-compatible `SECOND` that extracts the second from a time value.
/// - `time`: a time value.
///   Returns the second component as an integer (0–59).
pub fn codcel_second(time: NaiveTime) -> Result<i32, Box<dyn Error + Send + Sync>> {
    Ok(time.second() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_time(hour: u32, minute: u32, second: u32) -> NaiveTime {
        NaiveTime::from_hms_opt(hour, minute, second).unwrap()
    }

    #[test]
    fn test_second_zero() {
        // =SECOND("12:30:00") in US format
        // =SECOND("12:30:00") in German format
        let time = create_time(12, 30, 0);
        let result = codcel_second(time).unwrap();
        println!("{result}");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_second_single_digit() {
        // =SECOND("09:05:05") in US format
        // =SECOND("09:05:05") in German format
        let time = create_time(9, 5, 5);
        let result = codcel_second(time).unwrap();
        println!("{result}");
        assert_eq!(result, 5);
    }

    #[test]
    fn test_second_double_digit() {
        // =SECOND("15:45:30") in US format
        // =SECOND("15:45:30") in German format
        let time = create_time(15, 45, 30);
        let result = codcel_second(time).unwrap();
        println!("{result}");
        assert_eq!(result, 30);
    }

    #[test]
    fn test_second_max() {
        // =SECOND("23:59:59") in US format
        // =SECOND("23:59:59") in German format
        let time = create_time(23, 59, 59);
        let result = codcel_second(time).unwrap();
        println!("{result}");
        assert_eq!(result, 59);
    }

    #[test]
    fn test_second_midnight() {
        // =SECOND("00:00:00") in US format
        // =SECOND("00:00:00") in German format
        let time = create_time(0, 0, 0);
        let result = codcel_second(time).unwrap();
        println!("{result}");
        assert_eq!(result, 0);
    }
}
