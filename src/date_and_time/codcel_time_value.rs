// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::date_and_time::parse_time_formats::parse_time_formats;
use chrono::Timelike;
use std::error::Error;

/// Excel-compatible `TIMEVALUE` that converts a time string to a decimal number.
/// - `time_text`: a string representing a time in various formats (HH:MM:SS, HH:MM, AM/PM, etc.).
///   Returns a decimal fraction of a day (0.0–1.0) or an error if the string cannot be parsed.
pub fn codcel_time_value<S: AsRef<str>>(time_text: S) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let time_str = time_text.as_ref().trim();

    // Try several common time formats
    let time = parse_time_formats(time_str)?;

    // Calculate the fraction of the day
    let seconds_in_day = 24.0 * 60.0 * 60.0;
    let time_seconds = time.hour() as f64 * 3600.0
        + time.minute() as f64 * 60.0
        + time.second() as f64
        + time.nanosecond() as f64 / 1_000_000_000.0;

    Ok(time_seconds / seconds_in_day)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_value_standard_format() {
        // =TIMEVALUE("12:30:45") in US format
        // =TIMEVALUE("12:30:45") in German format
        let result = codcel_time_value("12:30:45").unwrap();
        println!("{result}");
        assert!((result - 0.5213541666666667).abs() < 0.0001);
    }

    #[test]
    fn test_time_value_with_am() {
        // =TIMEVALUE("9:30:00 AM") in US format
        // =TIMEVALUE("9:30:00 AM") in German format
        let result = codcel_time_value("9:30:00 AM").unwrap();
        println!("{result}");
        assert!((result - 0.3958333333333333).abs() < 0.0001);
    }

    #[test]
    fn test_time_value_with_pm() {
        // =TIMEVALUE("9:30:00 PM") in US format
        // =TIMEVALUE("9:30:00 PM") in German format
        let result = codcel_time_value("9:30:00 PM").unwrap();
        println!("{result}");
        assert!((result - 0.8958333333333334).abs() < 0.0001);
    }

    #[test]
    fn test_time_value_midnight() {
        // =TIMEVALUE("00:00:00") in US format
        // =TIMEVALUE("00:00:00") in German format
        let result = codcel_time_value("00:00:00").unwrap();
        println!("{result}");
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_time_value_noon() {
        // =TIMEVALUE("12:00:00") in US format
        // =TIMEVALUE("12:00:00") in German format
        let result = codcel_time_value("12:00:00").unwrap();
        println!("{result}");
        assert!((result - 0.5).abs() < 0.0001);
    }

    #[test]
    fn test_time_value_almost_midnight() {
        // =TIMEVALUE("23:59:59") in US format
        // =TIMEVALUE("23:59:59") in German format
        let result = codcel_time_value("23:59:59").unwrap();
        println!("{result}");
        assert!((result - 0.9999884259259259).abs() < 0.0001);
    }

    #[test]
    fn test_time_value_without_seconds() {
        // =TIMEVALUE("12:30") in US format
        // =TIMEVALUE("12:30") in German format
        let result = codcel_time_value("12:30").unwrap();
        println!("{result}");
        assert!((result - 0.5208333333333334).abs() < 0.0001);
    }

    #[test]
    fn test_time_value_invalid_format() {
        // =TIMEVALUE("not a time") in US format
        // =TIMEVALUE("not a time") in German format
        let result = codcel_time_value("not a time");
        assert!(result.is_err());
    }
}
