// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::engineering::complex::{format_complex, parse_complex};
use std::error::Error;

/// Excel-compatible `IMSUM` that returns the sum of one or more complex numbers.
/// - `numbers`: a vector of complex number strings in the form `x+yi` or `x+yj`.
///   Returns the sum as a complex number string, or an error when the list is empty or formats are invalid.
pub fn codcel_im_sum(
    numbers: Vec<String>,
    decimal_separator: &str,
    use_excel_rounding: bool,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    if numbers.is_empty() {
        return Err("IMSUM: requires at least 1 number".into());
    }

    // Determine which imaginary symbol is used (default to 'i' if no imaginary part)
    let im_symbol = if numbers.iter().any(|n| n.contains('j')) {
        'j'
    } else {
        'i'
    };

    // Initialize accumulators for real and imaginary parts
    let mut real_sum = 0.0;
    let mut imag_sum = 0.0;

    // Sum the parts of each number
    for number in numbers {
        let number = number.replace(" ", "");
        let (real, imag) = parse_complex(&number)?;
        real_sum += real;
        imag_sum += imag;
    }

    // Format the result
    format_complex(
        real_sum,
        imag_sum,
        im_symbol,
        decimal_separator,
        use_excel_rounding,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_im_sum_multiple_complex() {
        // =IMSUM("3+4i", "1+2i", "5+6i") in US format
        // =IMSUM("3+4i"; "1+2i"; "5+6i") in German format
        let result = codcel_im_sum(
            vec!["3+4i".to_string(), "1+2i".to_string(), "5+6i".to_string()],
            ".",
            true,
        )
        .unwrap();
        println!("{result}");
        assert!(result.contains("9+12i"));
    }

    #[test]
    fn test_im_sum_mixed_complex_and_real() {
        // =IMSUM("3+4i", "2", "5+6i") in US format
        // =IMSUM("3+4i"; "2"; "5+6i") in German format
        let result = codcel_im_sum(
            vec!["3+4i".to_string(), "2".to_string(), "5+6i".to_string()],
            ".",
            true,
        )
        .unwrap();
        println!("{result}");
        assert!(result.contains("10+10i"));
    }

    #[test]
    fn test_im_sum_only_real() {
        // =IMSUM("3", "2", "5") in US format
        // =IMSUM("3"; "2"; "5") in German format
        let result = codcel_im_sum(
            vec!["3".to_string(), "2".to_string(), "5".to_string()],
            ".",
            true,
        )
        .unwrap();
        println!("{result}");
        assert!(result.contains("10"));
    }

    #[test]
    fn test_im_sum_single_complex() {
        // =IMSUM("3+4i") in US format
        // =IMSUM("3+4i") in German format
        let result = codcel_im_sum(vec!["3+4i".to_string()], ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("3+4i"));
    }

    #[test]
    fn test_im_sum_j_symbol() {
        // =IMSUM("3+4j", "1+2j") in US format
        // =IMSUM("3+4j"; "1+2j") in German format
        let result =
            codcel_im_sum(vec!["3+4j".to_string(), "1+2j".to_string()], ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("4+6j"));
    }

    #[test]
    fn test_im_sum_mixed_symbols() {
        // =IMSUM("3+4i", "1+2j") in US format
        // =IMSUM("3+4i"; "1+2j") in German format
        let result =
            codcel_im_sum(vec!["3+4i".to_string(), "1+2j".to_string()], ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("4+6j"));
    }

    #[test]
    fn test_im_sum_empty_input() {
        // =IMSUM() in US format
        // =IMSUM() in German format
        let result = codcel_im_sum(vec![], ".", true);
        assert!(result.is_err());
    }

    #[test]
    fn test_im_sum_invalid_input() {
        // =IMSUM("not_a_complex_number", "1+2i") in US format
        // =IMSUM("not_a_complex_number"; "1+2i") in German format
        let result = codcel_im_sum(
            vec!["not_a_complex_number".to_string(), "1+2i".to_string()],
            ".",
            true,
        );
        assert!(result.is_err());
    }
}
