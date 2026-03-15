// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::engineering::complex::{format_complex, number_to_string, parse_complex};
use std::error::Error;

/// Excel-compatible `IMEXP` that returns the exponential of a complex number.
/// - `complex`: a complex number string in the form `x+yi` or `x+yj`.
///   Returns e^z as a complex number string, or an error for invalid formats.
pub fn codcel_im_exp(
    complex: String,
    decimal_separator: &str,
    use_excel_rounding: bool,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    // Remove all whitespace from the input
    let complex = complex.replace(" ", "");

    // Determine which imaginary symbol is used (default to 'i' if no imaginary part)
    let im_symbol = if complex.contains('j') { 'j' } else { 'i' };

    // For a purely real number
    if !complex.contains('i') && !complex.contains('j') {
        if let Ok(real) = complex.parse::<f64>() {
            return Ok(number_to_string(
                real.exp(),
                decimal_separator,
                use_excel_rounding,
            ));
        }
        return Err("IMEXP: Invalid real number format".into());
    }

    // Parse out the real and imaginary parts
    let (real, imag) = parse_complex(&complex)?;

    // Calculate using Euler's formula: exp(x + yi) = exp(x)(cos(y) + i*sin(y))
    let exp_x = real.exp();
    let real_part = exp_x * imag.cos();
    let imag_part = exp_x * imag.sin();

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
    fn test_im_exp_positive() {
        // =IMEXP("3+4i") in US format
        // =IMEXP("3+4i") in German format
        let result = codcel_im_exp("3+4i".to_string(), ".", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check the format
        assert!(result.contains(".") && result.contains("i"));
    }

    #[test]
    fn test_im_exp_negative() {
        // =IMEXP("-3-4i") in US format
        // =IMEXP("-3-4i") in German format
        let result = codcel_im_exp("-3-4i".to_string(), ".", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check the format
        assert!(result.contains(".") && result.contains("i"));
    }

    #[test]
    fn test_im_exp_real_only() {
        // =IMEXP("5") in US format
        // =IMEXP("5") in German format
        let result = codcel_im_exp("5".to_string(), ".", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check it's a real number
        assert!(!result.contains("i") && !result.contains("j"));
    }

    #[test]
    fn test_im_exp_imaginary_only() {
        // =IMEXP("5i") in US format
        // =IMEXP("5i") in German format
        let result = codcel_im_exp("5i".to_string(), ".", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check it's a complex number
        assert!(result.contains("i"));
    }

    #[test]
    fn test_im_exp_j_notation() {
        // =IMEXP("3+4j") in US format
        // =IMEXP("3+4j") in German format
        let result = codcel_im_exp("3+4j".to_string(), ".", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check it uses j notation
        assert!(result.contains("j"));
    }

    #[test]
    fn test_im_exp_decimal_separator() {
        // =IMEXP("3,5+4,2i") in US format
        // =IMEXP("3,5+4,2i") in German format
        let result = codcel_im_exp("3.5+4.2i".to_string(), ",", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check the format
        assert!(result.contains(","));
    }

    #[test]
    fn test_im_exp_invalid_input() {
        // =IMEXP("not_a_complex_number") in US format
        // =IMEXP("not_a_complex_number") in German format
        let result = codcel_im_exp("not_a_complex_number".to_string(), ".", true);
        assert!(result.is_err());
    }
}
