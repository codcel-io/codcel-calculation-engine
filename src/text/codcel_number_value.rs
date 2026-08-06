// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `NUMBERVALUE` that converts text to a number in a locale-independent way.
/// - `text`: the text representing a number (may include currency symbols, which are stripped).
/// - `decimal_separator`: the character used as the decimal separator in the text.
/// - `group_separator`: the character used as the thousands/grouping separator in the text.
///   Returns the numeric value parsed from the text.
///   Returns an error if the decimal and group separators are the same or if the text
///   cannot be converted to a valid number.
pub fn codcel_number_value<S: AsRef<str>>(
    text: S,
    decimal_separator: S,
    group_separator: S,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let text = text.as_ref().trim();
    let decimal_separator = decimal_separator.as_ref();
    let group_separator = group_separator.as_ref();

    // Remove the first character if it is not a "-" or a number
    // as it could be a currency symbol
    let text = if let Some(first_char) = text.chars().next() {
        if !first_char.is_numeric() && first_char != '-' {
            // Safely skip the first character
            text[first_char.len_utf8()..].trim()
        } else {
            text
        }
    } else {
        text
    };

    // Ensure the separators are not the same
    if decimal_separator == group_separator {
        return Err("NUMBERVALUE: Decimal and group separators must be different.".into());
    }

    // Replace group separators with an empty string
    let mut processed_text = text.replace(group_separator, "");

    // Replace the decimal separator with a standard dot (.)
    if let Some(pos) = processed_text.find(decimal_separator) {
        processed_text.replace_range(pos..pos + decimal_separator.len(), ".");
    }

    // Attempt to parse the resulting string as a floating-point number
    processed_text.parse::<f64>().map_err(|_| {
        format!("NUMBERVALUE: Unable to convert '{text}' to a number with the given separators.")
            .into()
    })
}

/// Vector variant of `NUMBERVALUE` that accepts a vector of exactly 3 inputs.
/// - `inputs`: a vector containing [text, decimal_separator, group_separator].
///   Returns the numeric value or an error if not exactly 3 inputs are provided.
pub fn codcel_number_value_vec<S: AsRef<str>>(
    inputs: Vec<S>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 3 {
        return Err("NUMBERVALUE: Expected 3 inputs".into());
    }
    codcel_number_value(inputs[0].as_ref(), inputs[1].as_ref(), inputs[2].as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_value_basic() {
        // =NUMBERVALUE("1,234.56", ".", ",") in US format
        // =NUMBERVALUE("1,234.56"; "."; ",") in German format
        let result = codcel_number_value("1,234.56", ".", ",").unwrap();
        println!("{result}");
        assert!((result - 1234.56).abs() < 0.0001);
    }

    #[test]
    fn test_number_value_different_separators() {
        // =NUMBERVALUE("1.234,56", ",", ".") in US format
        // =NUMBERVALUE("1.234,56"; ","; ".") in German format
        let result = codcel_number_value("1.234,56", ",", ".").unwrap();
        println!("{result}");
        assert!((result - 1234.56).abs() < 0.0001);
    }

    #[test]
    fn test_number_value_with_currency_symbol() {
        // =NUMBERVALUE("$1,234.56", ".", ",") in US format
        // =NUMBERVALUE("$1,234.56"; "."; ",") in German format
        let result = codcel_number_value("$1,234.56", ".", ",").unwrap();
        println!("{result}");
        assert!((result - 1234.56).abs() < 0.0001);
    }

    #[test]
    fn test_number_value_negative() {
        // =NUMBERVALUE("-1,234.56", ".", ",") in US format
        // =NUMBERVALUE("-1,234.56"; "."; ",") in German format
        let result = codcel_number_value("-1,234.56", ".", ",").unwrap();
        println!("{result}");
        assert!((result + 1234.56).abs() < 0.0001);
    }

    #[test]
    fn test_number_value_no_separators() {
        // =NUMBERVALUE("1234.56", ".", ",") in US format
        // =NUMBERVALUE("1234.56"; "."; ",") in German format
        let result = codcel_number_value("1234.56", ".", ",").unwrap();
        println!("{result}");
        assert!((result - 1234.56).abs() < 0.0001);
    }

    #[test]
    fn test_number_value_same_separators() {
        // This should return an error
        let result = codcel_number_value("1,234.56", ",", ",");
        assert!(result.is_err());
    }

    #[test]
    fn test_number_value_invalid_number() {
        // This should return an error
        let result = codcel_number_value("not a number", ".", ",");
        assert!(result.is_err());
    }
}
