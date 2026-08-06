// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::engineering::complex::{format_complex, parse_complex};
use std::error::Error;
use std::f64::consts::LN_10;

/// Excel-compatible `IMLOG10` that returns the base-10 logarithm of a complex number.
/// - `complex`: a complex number string in the form `x+yi` or `x+yj`.
///   Returns log₁₀(z) as a complex number string, or an error for invalid formats or zero input.
pub fn codcel_im_log10(
    complex: String,
    decimal_separator: &str,
    use_excel_rounding: bool,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    // Remove all whitespace from the input
    let complex = complex.replace(" ", "");

    // Determine which imaginary symbol is used (default to 'i' if no imaginary part)
    let im_symbol = if complex.contains('j') { 'j' } else { 'i' };

    // Parse out the real and imaginary parts
    let (real, imag) = parse_complex(&complex)?;

    // Calculate the magnitude (sqrt(x² + y²))
    let magnitude = crate::portable_math::sqrt(real * real + imag * imag);

    // Check for zero magnitude
    if magnitude == 0.0 {
        return Err("IMLOG10: Cannot compute logarithm of zero".into());
    }

    // Calculate using the formula: log10(x + yi) = ln(x + yi)/ln(10)
    let real_part = crate::portable_math::ln(magnitude) / LN_10;
    let imag_part = crate::portable_math::atan2(imag, real) / LN_10;

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
    fn test_im_log10_real_positive() {
        // =IMLOG10(100) in US format
        // =IMLOG10(100) in German format
        let result = codcel_im_log10("100".to_string(), ".", true).unwrap();
        println!("{result}");
        assert_eq!(result, "2");
    }

    #[test]
    fn test_im_log10_complex() {
        // =IMLOG10("3+4i") in US format
        // =IMLOG10("3+4i") in German format
        let result = codcel_im_log10("3+4i".to_string(), ".", true).unwrap();
        println!("{result}");
        assert_eq!(result, "0.698970004336019+0.402719196273373i");
    }

    #[test]
    fn test_im_log10_imaginary_only() {
        // =IMLOG10("4i") in US format
        // =IMLOG10("4i") in German format
        let result = codcel_im_log10("4i".to_string(), ".", true).unwrap();
        println!("{result}");
        assert_eq!(result, "0.602059991327962+0.682188176920921i");
    }

    #[test]
    fn test_im_log10_j_symbol() {
        // =IMLOG10("3+4j") in US format
        // =IMLOG10("3+4j") in German format
        let result = codcel_im_log10("3+4j".to_string(), ".", true).unwrap();
        println!("{result}");
        assert_eq!(result, "0.698970004336019+0.402719196273373j");
    }

    #[test]
    fn test_im_log10_negative_real() {
        // =IMLOG10(-4) in US format - Excel returns log10(4)+πi/ln(10) for negative reals
        let result = codcel_im_log10("-4".to_string(), ".", true).unwrap();
        assert!(result.contains("0.60205999132796") && result.contains("i"));
    }

    #[test]
    fn test_im_log10_zero() {
        // =IMLOG10(0) in US format
        // =IMLOG10(0) in German format
        let result = codcel_im_log10("0".to_string(), ".", true);
        assert!(result.is_err());
    }

    #[test]
    fn test_im_log10_decimal_separator() {
        // =IMLOG10(100) in US format
        // =IMLOG10(100) in German format
        let result = codcel_im_log10("100".to_string(), ",", true).unwrap();
        println!("{result}");
        assert_eq!(result, "2");
    }
}
