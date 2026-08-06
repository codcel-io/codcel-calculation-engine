// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::engineering::complex::{format_complex, parse_complex};
use std::error::Error;

/// Excel-compatible `IMPRODUCT` that returns the product of one or more complex numbers.
/// - `numbers`: a vector of complex number strings in the form `x+yi` or `x+yj`.
///   Returns the product as a complex number string, or an error when the list is empty or formats are invalid.
pub fn codcel_im_product(
    numbers: Vec<String>,
    decimal_separator: &str,
    use_excel_rounding: bool,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    if numbers.is_empty() {
        return Err("IMPRODUCT requires at least 1 number".into());
    }

    // Determine which imaginary symbol is used (default to 'i' if no imaginary part)
    let im_symbol = if numbers.iter().any(|n| n.contains('j')) {
        'j'
    } else {
        'i'
    };

    // Parse first number
    let (mut real_acc, mut imag_acc) = parse_complex(&numbers[0].replace(" ", ""))?;

    // Multiply by each subsequent number
    for number in numbers.iter().skip(1) {
        let number = number.replace(" ", "");

        // Parse the current number
        let (real, imag) = parse_complex(&number)?;

        // Multiply complex numbers: (a + bi)(c + di) = (ac - bd) + (ad + bc)i
        let new_real = real_acc * real - imag_acc * imag;
        let new_imag = real_acc * imag + imag_acc * real;

        real_acc = new_real;
        imag_acc = new_imag;
    }

    // Format the result
    format_complex(
        real_acc,
        imag_acc,
        im_symbol,
        decimal_separator,
        use_excel_rounding,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_im_product_two_complex() {
        // =IMPRODUCT("3+4i", "2+3i") in US format
        // =IMPRODUCT("3+4i"; "2+3i") in German format
        let result =
            codcel_im_product(vec!["3+4i".to_string(), "2+3i".to_string()], ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("-6+17i"));
    }

    #[test]
    fn test_im_product_complex_and_real() {
        // =IMPRODUCT("3+4i", "2") in US format
        // =IMPRODUCT("3+4i"; "2") in German format
        let result =
            codcel_im_product(vec!["3+4i".to_string(), "2".to_string()], ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("6+8i"));
    }

    #[test]
    fn test_im_product_multiple_complex() {
        // =IMPRODUCT("3+4i", "2+3i", "1+2i") in US format
        // =IMPRODUCT("3+4i"; "2+3i"; "1+2i") in German format
        let result = codcel_im_product(
            vec!["3+4i".to_string(), "2+3i".to_string(), "1+2i".to_string()],
            ".",
            true,
        )
        .unwrap();
        println!("{result}");
        assert!(result.contains("-40+5i"));
    }

    #[test]
    fn test_im_product_j_symbol() {
        // =IMPRODUCT("3+4j", "2+3j") in US format
        // =IMPRODUCT("3+4j"; "2+3j") in German format
        let result =
            codcel_im_product(vec!["3+4j".to_string(), "2+3j".to_string()], ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("-6+17j"));
    }

    #[test]
    fn test_im_product_empty_input() {
        // =IMPRODUCT() in US format
        // =IMPRODUCT() in German format
        let result = codcel_im_product(vec![], ".", true);
        assert!(result.is_err());
    }

    #[test]
    fn test_im_product_invalid_input() {
        // =IMPRODUCT("not_a_complex_number") in US format
        // =IMPRODUCT("not_a_complex_number") in German format
        let result = codcel_im_product(vec!["not_a_complex_number".to_string()], ".", true);
        assert!(result.is_err());
    }
}
