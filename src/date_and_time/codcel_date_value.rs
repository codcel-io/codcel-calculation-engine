// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::date_and_time::try_parse_excel_formats::try_parse_excel_formats;
use crate::date_and_time::try_parse_month_day_formats::try_parse_month_day_formats;
use crate::date_and_time::try_parse_standard_formats::try_parse_standard_formats;
use crate::date_system::DateSemantics;
use crate::date_time_base::date_time_to_excel;
use std::error::Error;

/// Excel-compatible `DATEVALUE` that converts a date string to an Excel serial number.
/// - `date_text`: a string representing a date in various formats (ISO, US, Excel, etc.).
/// - `dates`: the serial convention to encode into.
///
/// Returns the Excel serial number or an error if the string cannot be parsed.
pub fn codcel_date_value<S: AsRef<str>>(
    date_text: S,
    dates: DateSemantics,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let date_text = date_text.as_ref().trim();

    if date_text.is_empty() {
        return Err("DATEVALUE: Date text is empty".into());
    }

    // Try different date formats to get a DateTime first
    let dt = if let Some(dt) = try_parse_standard_formats(date_text) {
        dt
    } else if let Some(dt) = try_parse_month_day_formats(date_text) {
        dt
    } else if let Some(dt) = try_parse_excel_formats(date_text) {
        dt
    } else {
        return Err(format!("DATEVALUE: Could not parse date string: {date_text}").into());
    };

    // Convert DateTime to Excel date serial number
    date_time_to_excel(&dt, dates)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The existing expectations all pin Excel's own serials, so the tests read
    /// better with the convention bound once rather than repeated per call.
    fn codcel_date_value_excel_1900(date_text: &str) -> Result<f64, Box<dyn Error + Send + Sync>> {
        codcel_date_value(date_text, DateSemantics::EXCEL_1900)
    }

    #[test]
    fn test_date_value_iso_format() {
        // =DATEVALUE("2023-05-20") in US format
        // =DATEVALUE("2023-05-20") in German format
        let result = codcel_date_value_excel_1900("2023-05-20").unwrap();
        println!("{result}");
        assert!((result - 45066.0).abs() < 0.0001);
    }

    #[test]
    fn test_date_value_us_format() {
        // =DATEVALUE("5/20/2023") in US format
        // =DATEVALUE("5/20/2023") in German format
        let result = codcel_date_value_excel_1900("5/20/2023").unwrap();
        println!("{result}");
        assert!((result - 45066.0).abs() < 0.0001);
    }

    #[test]
    fn test_date_value_month_day_format() {
        // =DATEVALUE("May 20, 2023") in US format
        // =DATEVALUE("May 20, 2023") in German format
        let result = codcel_date_value_excel_1900("May 20, 2023").unwrap();
        println!("{result}");
        assert!((result - 45066.0).abs() < 0.0001);
    }

    #[test]
    fn test_date_value_excel_format() {
        // =DATEVALUE("20-May-2023") in US format
        // =DATEVALUE("20-May-2023") in German format
        let result = codcel_date_value_excel_1900("20-May-2023").unwrap();
        println!("{result}");
        assert!((result - 45066.0).abs() < 0.0001);
    }

    #[test]
    fn test_date_value_empty_string() {
        // =DATEVALUE("") in US format
        // =DATEVALUE("") in German format
        let result = codcel_date_value_excel_1900("");
        assert!(result.is_err());
    }

    #[test]
    fn test_date_value_invalid_format() {
        // =DATEVALUE("not a date") in US format
        // =DATEVALUE("not a date") in German format
        let result = codcel_date_value_excel_1900("not a date");
        assert!(result.is_err());
    }
}
