// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::arithmetic_base::float_to_formatted_string_display;
use crate::value_format::ValueFormat;
use std::error::Error;

/// Excel-compatible `DOLLAR` that converts a number to text in currency format.
/// - `number`: the number to format as currency.
/// - `decimals`: optional number of decimal places (default 2). Negative values round
///   to the left of the decimal point.
/// - `value_format`: locale settings for currency symbol, decimal separator, and thousands separator.
///   Returns the number formatted as currency text with thousands separators.
pub fn codcel_dollar(
    number: f64,
    decimals: Option<i32>,
    value_format: &ValueFormat,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let decimals = decimals.unwrap_or(2); // Default to 2 decimal places if not provided
    let factor = 10f64.powi(decimals.abs());

    let rounded = if decimals >= 0 {
        (number * factor).round() / factor
    } else {
        (number / factor).round() * factor
    };

    // Format with the correct number of decimal places
    let formatted = if decimals > 0 {
        format!("{:.dec$}", rounded, dec = decimals as usize)
    } else {
        format!("{rounded:.0}")
    };

    let formatted = float_to_formatted_string_display(
        formatted,
        &value_format.decimal_separator,
        &value_format.thousands_separator,
    )?;

    let formatted = if let Some(stripped) = formatted.strip_prefix('-') {
        format!("-{}{}", value_format.currency_symbol, stripped)
    } else {
        format!("{}{}", value_format.currency_symbol, formatted)
    };
    Ok(formatted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dollar_positive_number_us_format() {
        // =DOLLAR(1234.56) in US format
        // =DOLLAR(1234,56) in German format
        let value_format = ValueFormat {
            decimal_separator: ".".to_string(),
            currency_symbol: "$".to_string(),
            thousands_separator: ",".to_string(),
            use_excel_rounding: true,
            language: "en".to_string(),
            allow_lotus_1_2_3_1900_date_bug: true,
        };

        let result = codcel_dollar(1234.56, None, &value_format).unwrap();
        println!("{result}");
        assert_eq!(result, "$1,234.56");
    }

    #[test]
    fn test_dollar_negative_number_us_format() {
        // =DOLLAR(-1234.56) in US format
        // =DOLLAR(-1234,56) in German format
        let value_format = ValueFormat {
            decimal_separator: ".".to_string(),
            currency_symbol: "$".to_string(),
            thousands_separator: ",".to_string(),
            use_excel_rounding: true,
            language: "en".to_string(),
            allow_lotus_1_2_3_1900_date_bug: true,
        };

        let result = codcel_dollar(-1234.56, None, &value_format).unwrap();
        println!("{result}");
        assert_eq!(result, "-$1,234.56");
    }

    #[test]
    fn test_dollar_with_custom_decimals_us_format() {
        // =DOLLAR(1234.56, 3) in US format
        // =DOLLAR(1234,56; 3) in German format
        let value_format = ValueFormat {
            decimal_separator: ".".to_string(),
            currency_symbol: "$".to_string(),
            thousands_separator: ",".to_string(),
            use_excel_rounding: true,
            language: "en".to_string(),
            allow_lotus_1_2_3_1900_date_bug: true,
        };

        let result = codcel_dollar(1234.56, Some(3), &value_format).unwrap();
        println!("{result}");
        assert_eq!(result, "$1,234.560");
    }

    #[test]
    fn test_dollar_with_zero_decimals_us_format() {
        // =DOLLAR(1234.56, 0) in US format
        // =DOLLAR(1234,56; 0) in German format
        let value_format = ValueFormat {
            decimal_separator: ".".to_string(),
            currency_symbol: "$".to_string(),
            thousands_separator: ",".to_string(),
            use_excel_rounding: true,
            language: "en".to_string(),
            allow_lotus_1_2_3_1900_date_bug: true,
        };

        let result = codcel_dollar(1234.56, Some(0), &value_format).unwrap();
        println!("{result}");
        assert_eq!(result, "$1,235");
    }

    #[test]
    fn test_dollar_with_negative_decimals_us_format() {
        // =DOLLAR(1234.56, -1) in US format
        // =DOLLAR(1234,56; -1) in German format
        let value_format = ValueFormat {
            decimal_separator: ".".to_string(),
            currency_symbol: "$".to_string(),
            thousands_separator: ",".to_string(),
            use_excel_rounding: true,
            language: "en".to_string(),
            allow_lotus_1_2_3_1900_date_bug: true,
        };

        let result = codcel_dollar(1234.56, Some(-1), &value_format).unwrap();
        println!("{result}");
        assert_eq!(result, "$1,230");
    }

    #[test]
    fn test_dollar_german_format() {
        // =DOLLAR(1234.56) in US format
        // =DOLLAR(1234,56) in German format
        let value_format = ValueFormat {
            decimal_separator: ",".to_string(),
            currency_symbol: "€".to_string(),
            thousands_separator: ".".to_string(),
            use_excel_rounding: true,
            language: "de".to_string(),
            allow_lotus_1_2_3_1900_date_bug: true,
        };

        let result = codcel_dollar(1234.56, None, &value_format).unwrap();
        println!("{result}");
        assert_eq!(result, "€1.234,56");
    }
}
