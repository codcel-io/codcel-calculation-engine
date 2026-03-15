// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `FIXED` that formats a number as text with a fixed number of decimals.
/// - `number`: the number to format.
/// - `decimal_places`: optional number of decimal places (default 2). Negative values treated as 0.
/// - `no_commas`: optional flag to omit thousands separators (default `false`).
/// - `thousands_separator`: the character to use as thousands separator (e.g., `,`).
/// - `decimal_separator`: the character to use as decimal separator (e.g., `.`).
///   Returns the formatted number as a text string with the specified decimal places
///   and optional thousands separators.
pub fn codcel_fixed(
    number: f64,
    decimal_places: Option<i32>,
    no_commas: Option<bool>,
    thousands_separator: &str,
    decimal_separator: &str,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let mut decimal_places = decimal_places.unwrap_or(2);
    if decimal_places < 0 {
        decimal_places = 0;
    }
    let no_commas = no_commas.unwrap_or(false);
    let thousands_separator = if thousands_separator.is_empty() {
        ","
    } else {
        thousands_separator
    };

    let decimal_separator = if decimal_separator.is_empty() {
        "."
    } else {
        decimal_separator
    };

    // Format the number with the specified number of decimal places
    let mut formatted_number = format!(
        "{:.precision$}",
        number,
        precision = decimal_places as usize
    );
    if decimal_separator != "." {
        formatted_number = formatted_number.replace(".", decimal_separator);
    }

    if no_commas {
        // Return the number without commas
        Ok(formatted_number)
    } else {
        // Add commas as thousand separators
        let parts: Vec<&str> = formatted_number.split(decimal_separator).collect();
        let integer_part = parts[0];
        let fractional_part = if parts.len() > 1 { parts[1] } else { "" };

        let mut with_commas = String::new();
        let mut count = 0;

        // Iterate through the integer part in reverse order to insert commas
        for c in integer_part.chars().rev() {
            if count == 3 {
                with_commas.push_str(thousands_separator);
                count = 0;
            }
            with_commas.push(c);
            count += 1;
        }

        // Reverse the string to get the correct order
        let with_commas: String = with_commas.chars().rev().collect();

        // Add the fractional part if it exists
        if !fractional_part.is_empty() {
            Ok(format!("{with_commas}{decimal_separator}{fractional_part}"))
        } else {
            Ok(with_commas)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_default_parameters() {
        // =FIXED(1234.567) in US format
        // =FIXED(1234,567) in German format
        let result = codcel_fixed(1234.567, None, None, ",", ".").unwrap();
        println!("{result}");
        assert_eq!(result, "1,234.57");
    }

    #[test]
    fn test_fixed_with_decimal_places() {
        // =FIXED(1234.567, 1) in US format
        // =FIXED(1234,567; 1) in German format
        let result = codcel_fixed(1234.567, Some(1), None, ",", ".").unwrap();
        println!("{result}");
        assert_eq!(result, "1,234.6");
    }

    #[test]
    fn test_fixed_with_zero_decimal_places() {
        // =FIXED(1234.567, 0) in US format
        // =FIXED(1234,567; 0) in German format
        let result = codcel_fixed(1234.567, Some(0), None, ",", ".").unwrap();
        println!("{result}");
        assert_eq!(result, "1,235");
    }

    #[test]
    fn test_fixed_with_negative_decimal_places() {
        // =FIXED(1234.567, -1) in US format
        // =FIXED(1234,567; -1) in German format
        let result = codcel_fixed(1234.567, Some(-1), None, ",", ".").unwrap();
        println!("{result}");
        assert_eq!(result, "1,235");
    }

    #[test]
    fn test_fixed_no_commas() {
        // =FIXED(1234.567, 2, TRUE) in US format
        // =FIXED(1234,567; 2; TRUE) in German format
        let result = codcel_fixed(1234.567, Some(2), Some(true), ",", ".").unwrap();
        println!("{result}");
        assert_eq!(result, "1234.57");
    }

    #[test]
    fn test_fixed_negative_number() {
        // =FIXED(-1234.567, 2) in US format
        // =FIXED(-1234,567; 2) in German format
        let result = codcel_fixed(-1234.567, Some(2), None, ",", ".").unwrap();
        println!("{result}");
        assert_eq!(result, "-1,234.57");
    }

    #[test]
    fn test_fixed_german_format() {
        // =FIXED(1234.567, 2) in US format
        // =FIXED(1234,567; 2) in German format
        let result = codcel_fixed(1234.567, Some(2), None, ".", ",").unwrap();
        println!("{result}");
        assert_eq!(result, "1.234,57");
    }

    #[test]
    fn test_fixed_large_number() {
        // =FIXED(1234567.89, 2) in US format
        // =FIXED(1234567,89; 2) in German format
        let result = codcel_fixed(1234567.89, Some(2), None, ",", ".").unwrap();
        println!("{result}");
        assert_eq!(result, "1,234,567.89");
    }
}
