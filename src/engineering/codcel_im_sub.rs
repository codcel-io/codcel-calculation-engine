// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::engineering::complex::{format_complex, parse_complex};
use std::error::Error;

/// Excel-compatible `IMSUB` that returns the difference of two complex numbers.
/// - `number1`: the minuend complex number string.
/// - `number2`: the subtrahend complex number string.
///   Returns (number1 - number2) as a complex number string, or an error for invalid formats.
pub fn codcel_im_sub(
    number1: String,
    number2: String,
    decimal_separator: &str,
    use_excel_rounding: bool,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    // Remove all whitespace from inputs
    let number1 = number1.replace(" ", "");
    let number2 = number2.replace(" ", "");

    // Determine which imaginary symbol is used (default to 'i' if no imaginary part)
    let im_symbol = if number1.contains('j') || number2.contains('j') {
        'j'
    } else {
        'i'
    };

    // Parse the complex numbers
    let (real1, imag1) = parse_complex(&number1)?;
    let (real2, imag2) = parse_complex(&number2)?;

    // Subtract the components
    let real_part = real1 - real2;
    let imag_part = imag1 - imag2;

    // Format the result
    format_complex(
        real_part,
        imag_part,
        im_symbol,
        decimal_separator,
        use_excel_rounding,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_im_sub_complex_numbers() {
        // =IMSUB("3+4i", "1+2i") in US format
        // =IMSUB("3+4i"; "1+2i") in German format
        let result = codcel_im_sub("3+4i".to_string(), "1+2i".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("2+2i"));
    }

    #[test]
    fn test_im_sub_real_from_complex() {
        // =IMSUB("3+4i", "2") in US format
        // =IMSUB("3+4i"; "2") in German format
        let result = codcel_im_sub("3+4i".to_string(), "2".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("1+4i"));
    }

    #[test]
    fn test_im_sub_complex_from_real() {
        // =IMSUB("5", "2+3i") in US format
        // =IMSUB("5"; "2+3i") in German format
        let result = codcel_im_sub("5".to_string(), "2+3i".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("3-3i"));
    }

    #[test]
    fn test_im_sub_real_numbers() {
        // =IMSUB("5", "2") in US format
        // =IMSUB("5"; "2") in German format
        let result = codcel_im_sub("5".to_string(), "2".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("3"));
    }

    #[test]
    fn test_im_sub_j_symbol() {
        // =IMSUB("3+4j", "1+2j") in US format
        // =IMSUB("3+4j"; "1+2j") in German format
        let result = codcel_im_sub("3+4j".to_string(), "1+2j".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("2+2j"));
    }

    #[test]
    fn test_im_sub_mixed_symbols() {
        // =IMSUB("3+4i", "1+2j") in US format
        // =IMSUB("3+4i"; "1+2j") in German format
        let result = codcel_im_sub("3+4i".to_string(), "1+2j".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("2+2j"));
    }

    #[test]
    fn test_im_sub_invalid_input() {
        // =IMSUB("not_a_complex_number", "1+2i") in US format
        // =IMSUB("not_a_complex_number"; "1+2i") in German format
        let result = codcel_im_sub(
            "not_a_complex_number".to_string(),
            "1+2i".to_string(),
            ".",
            true,
        );
        assert!(result.is_err());
    }
}
