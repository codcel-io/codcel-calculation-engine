// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::value::Value;
use crate::value_format::ValueFormat;
use std::error::Error;

/// Excel-compatible `VALUETOTEXT` that converts a value to text.
/// - `value`: the value to convert to text.
/// - `format`: optional format flag (default `false`).
///   - `false`: returns the value as plain text.
///   - `true`: returns text values with surrounding quotes (e.g., `"Hello"`).
/// - `value_format`: locale settings for formatting the value.
///   Returns the text representation of the value. Useful for displaying values
///   exactly as they appear in cells.
pub fn codcel_value_to_text(
    value: Value,
    format: Option<bool>,
    value_format: &ValueFormat,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let format = format.unwrap_or(false);

    if format && value.i32(value_format).is_err() {
        return Ok(format!("\"{}\"", value.string(value_format)?));
    }

    value.string(value_format)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::{f64, i32, string};

    #[test]
    fn test_value_to_text_number() {
        // =VALUETOTEXT(123) in US format
        // =VALUETOTEXT(123) in German format
        let value_format = ValueFormat {
            decimal_separator: ".".to_string(),
            currency_symbol: "$".to_string(),
            thousands_separator: ",".to_string(),
            use_excel_rounding: true,
            language: "en".to_string(),
            allow_lotus_1_2_3_1900_date_bug: true,
        };

        let result = codcel_value_to_text(i32(123), None, &value_format).unwrap();
        println!("{result}");
        assert_eq!(result, "123");
    }

    #[test]
    fn test_value_to_text_decimal() {
        // =VALUETOTEXT(123.45) in US format
        // =VALUETOTEXT(123,45) in German format
        let value_format = ValueFormat {
            decimal_separator: ".".to_string(),
            currency_symbol: "$".to_string(),
            thousands_separator: ",".to_string(),
            use_excel_rounding: true,
            language: "en".to_string(),
            allow_lotus_1_2_3_1900_date_bug: true,
        };

        let result = codcel_value_to_text(f64(123.45), None, &value_format).unwrap();
        println!("{result}");
        assert_eq!(result, "123.45");
    }

    #[test]
    fn test_value_to_text_string() {
        // =VALUETOTEXT("Hello") in US format
        // =VALUETOTEXT("Hello") in German format
        let value_format = ValueFormat {
            decimal_separator: ".".to_string(),
            currency_symbol: "$".to_string(),
            thousands_separator: ",".to_string(),
            use_excel_rounding: true,
            language: "en".to_string(),
            allow_lotus_1_2_3_1900_date_bug: true,
        };

        let result =
            codcel_value_to_text(string("Hello".to_string()), None, &value_format).unwrap();
        println!("{result}");
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_value_to_text_with_format_string() {
        // =VALUETOTEXT("Hello", TRUE) in US format
        // =VALUETOTEXT("Hello"; TRUE) in German format
        let value_format = ValueFormat {
            decimal_separator: ".".to_string(),
            currency_symbol: "$".to_string(),
            thousands_separator: ",".to_string(),
            use_excel_rounding: true,
            language: "en".to_string(),
            allow_lotus_1_2_3_1900_date_bug: true,
        };

        let result =
            codcel_value_to_text(string("Hello".to_string()), Some(true), &value_format).unwrap();
        println!("{result}");
        assert_eq!(result, "\"Hello\"");
    }

    #[test]
    fn test_value_to_text_with_format_number() {
        // =VALUETOTEXT(123, TRUE) in US format
        // =VALUETOTEXT(123; TRUE) in German format
        let value_format = ValueFormat {
            decimal_separator: ".".to_string(),
            currency_symbol: "$".to_string(),
            thousands_separator: ",".to_string(),
            use_excel_rounding: true,
            language: "en".to_string(),
            allow_lotus_1_2_3_1900_date_bug: true,
        };

        let result = codcel_value_to_text(i32(123), Some(true), &value_format).unwrap();
        println!("{result}");
        assert_eq!(result, "123");
    }
}
