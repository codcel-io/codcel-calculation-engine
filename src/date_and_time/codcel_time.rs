// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use chrono::NaiveTime;
use std::error::Error;

/// Excel-compatible `TIME` that constructs a time from hour, minute, and second components.
/// - `hour`: the hour component (values outside 0–23 wrap modulo 24).
/// - `minute`: the minute component (values outside 0–59 roll over into hours).
/// - `second`: the second component (values outside 0–59 roll over into minutes).
///   Returns a time value normalized to a 24-hour day.
pub fn codcel_time(
    hour: i32,
    minute: i32,
    second: i32,
) -> Result<NaiveTime, Box<dyn Error + Send + Sync>> {
    let mut total_seconds = hour * 3600 + minute * 60 + second;

    // Normalize the total seconds to fit within a single day
    total_seconds = ((total_seconds % 86400) + 86400) % 86400; // Handle underflows (negative time)

    // Extract normalized hours, minutes, and seconds
    let normalized_hour = total_seconds / 3600;
    let remaining_seconds = total_seconds % 3600;
    let normalized_minute = remaining_seconds / 60;
    let normalized_second = remaining_seconds % 60;

    // Construct and return the NaiveTime
    NaiveTime::from_hms_opt(
        normalized_hour as u32,
        normalized_minute as u32,
        normalized_second as u32,
    )
    .ok_or_else(|| format!("Error: Invalid time components after {hour}:{minute}:{second}").into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Timelike;

    #[test]
    fn test_time_normal() {
        // =TIME(12, 30, 45) in US format
        // =TIME(12; 30; 45) in German format
        let result = codcel_time(12, 30, 45).unwrap();
        println!("{result}");
        assert_eq!(result.hour(), 12);
        assert_eq!(result.minute(), 30);
        assert_eq!(result.second(), 45);
    }

    #[test]
    fn test_time_midnight() {
        // =TIME(0, 0, 0) in US format
        // =TIME(0; 0; 0) in German format
        let result = codcel_time(0, 0, 0).unwrap();
        println!("{result}");
        assert_eq!(result.hour(), 0);
        assert_eq!(result.minute(), 0);
        assert_eq!(result.second(), 0);
    }

    #[test]
    fn test_time_hour_overflow() {
        // =TIME(25, 15, 30) in US format
        // =TIME(25; 15; 30) in German format
        let result = codcel_time(25, 15, 30).unwrap();
        println!("{result}");
        assert_eq!(result.hour(), 1);
        assert_eq!(result.minute(), 15);
        assert_eq!(result.second(), 30);
    }

    #[test]
    fn test_time_minute_overflow() {
        // =TIME(12, 75, 30) in US format
        // =TIME(12; 75; 30) in German format
        let result = codcel_time(12, 75, 30).unwrap();
        println!("{result}");
        assert_eq!(result.hour(), 13);
        assert_eq!(result.minute(), 15);
        assert_eq!(result.second(), 30);
    }

    #[test]
    fn test_time_second_overflow() {
        // =TIME(12, 30, 90) in US format
        // =TIME(12; 30; 90) in German format
        let result = codcel_time(12, 30, 90).unwrap();
        println!("{result}");
        assert_eq!(result.hour(), 12);
        assert_eq!(result.minute(), 31);
        assert_eq!(result.second(), 30);
    }

    #[test]
    fn test_time_negative_hour() {
        // =TIME(-12, 30, 45) in US format
        // =TIME(-12; 30; 45) in German format
        let result = codcel_time(-12, 30, 45).unwrap();
        println!("{result}");
        assert_eq!(result.hour(), 12);
        assert_eq!(result.minute(), 30);
        assert_eq!(result.second(), 45);
    }

    #[test]
    fn test_time_negative_minute() {
        // =TIME(12, -30, 45) in US format
        // =TIME(12; -30; 45) in German format
        let result = codcel_time(12, -30, 45).unwrap();
        println!("{result}");
        assert_eq!(result.hour(), 11);
        assert_eq!(result.minute(), 30);
        assert_eq!(result.second(), 45);
    }

    #[test]
    fn test_time_negative_second() {
        // =TIME(12, 30, -45) in US format
        // =TIME(12; 30; -45) in German format
        let result = codcel_time(12, 30, -45).unwrap();
        println!("{result}");
        assert_eq!(result.hour(), 12);
        assert_eq!(result.minute(), 29);
        assert_eq!(result.second(), 15);
    }

    #[test]
    fn test_time_all_negative() {
        // =TIME(-12, -30, -45) in US format
        // =TIME(-12; -30; -45) in German format
        let result = codcel_time(-12, -30, -45).unwrap();
        println!("{result}");
        assert_eq!(result.hour(), 11);
        assert_eq!(result.minute(), 29);
        assert_eq!(result.second(), 15);
    }
}
