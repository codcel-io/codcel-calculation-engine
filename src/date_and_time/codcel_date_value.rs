// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::date_and_time::datetime_to_excel_serial::datetime_to_excel_serial;
use crate::date_and_time::try_parse_excel_formats::try_parse_excel_formats;
use crate::date_and_time::try_parse_month_day_formats::try_parse_month_day_formats;
use crate::date_and_time::try_parse_standard_formats::try_parse_standard_formats;
use std::error::Error;

/// Excel-compatible `DATEVALUE` that converts a date string to an Excel serial number.
/// - `date_text`: a string representing a date in various formats (ISO, US, Excel, etc.).
///   Returns the Excel serial number (days since 1900-01-01) or an error if the string cannot be parsed.
pub fn codcel_date_value<S: AsRef<str>>(date_text: S) -> Result<f64, Box<dyn Error + Send + Sync>> {
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
    Ok(datetime_to_excel_serial(&dt))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_value_iso_format() {
        // =DATEVALUE("2023-05-20") in US format
        // =DATEVALUE("2023-05-20") in German format
        let result = codcel_date_value("2023-05-20").unwrap();
        println!("{result}");
        assert!((result - 45066.0).abs() < 0.0001);
    }

    #[test]
    fn test_date_value_us_format() {
        // =DATEVALUE("5/20/2023") in US format
        // =DATEVALUE("5/20/2023") in German format
        let result = codcel_date_value("5/20/2023").unwrap();
        println!("{result}");
        assert!((result - 45066.0).abs() < 0.0001);
    }

    #[test]
    fn test_date_value_month_day_format() {
        // =DATEVALUE("May 20, 2023") in US format
        // =DATEVALUE("May 20, 2023") in German format
        let result = codcel_date_value("May 20, 2023").unwrap();
        println!("{result}");
        assert!((result - 45066.0).abs() < 0.0001);
    }

    #[test]
    fn test_date_value_excel_format() {
        // =DATEVALUE("20-May-2023") in US format
        // =DATEVALUE("20-May-2023") in German format
        let result = codcel_date_value("20-May-2023").unwrap();
        println!("{result}");
        assert!((result - 45066.0).abs() < 0.0001);
    }

    #[test]
    fn test_date_value_empty_string() {
        // =DATEVALUE("") in US format
        // =DATEVALUE("") in German format
        let result = codcel_date_value("");
        assert!(result.is_err());
    }

    #[test]
    fn test_date_value_invalid_format() {
        // =DATEVALUE("not a date") in US format
        // =DATEVALUE("not a date") in German format
        let result = codcel_date_value("not a date");
        assert!(result.is_err());
    }
}
