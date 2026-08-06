// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::engineering::complex::{format_complex, parse_complex};
use std::error::Error;
use std::f64::consts::LN_2;

/// Excel-compatible `IMLOG2` that returns the base-2 logarithm of a complex number.
/// - `complex`: a complex number string in the form `x+yi` or `x+yj`.
///   Returns log₂(z) as a complex number string, or an error for invalid formats or zero input.
pub fn codcel_im_log2(
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
        return Err("IMLOG2: Cannot compute logarithm of zero".into());
    }

    // Calculate using the formula: log2(x + yi) = ln(x + yi)/ln(2)
    let real_part = crate::portable_math::ln(magnitude) / LN_2;
    let imag_part = crate::portable_math::atan2(imag, real) / LN_2;

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
    fn test_im_log2_real_positive() {
        // =IMLOG2(4) in US format
        // =IMLOG2(4) in German format
        let result = codcel_im_log2("4".to_string(), ".", true).unwrap();
        println!("{result}");
        assert_eq!(result, "2");
    }

    #[test]
    fn test_im_log2_complex() {
        // =IMLOG2("3+4i") in US format
        // =IMLOG2("3+4i") in German format
        let result = codcel_im_log2("3+4i".to_string(), ".", true).unwrap();
        println!("{result}");
        assert_eq!(result, "2.32192809488736+1.33780421245098i");
    }

    #[test]
    fn test_im_log2_imaginary_only() {
        // =IMLOG2("4i") in US format
        // =IMLOG2("4i") in German format
        let result = codcel_im_log2("4i".to_string(), ".", true).unwrap();
        println!("{result}");
        assert_eq!(result, "2+2.2661800709136i");
    }

    #[test]
    fn test_im_log2_j_symbol() {
        // =IMLOG2("3+4j") in US format
        // =IMLOG2("3+4j") in German format
        let result = codcel_im_log2("3+4j".to_string(), ".", true).unwrap();
        println!("{result}");
        assert_eq!(result, "2.32192809488736+1.33780421245098j");
    }

    #[test]
    fn test_im_log2_negative_real() {
        // =IMLOG2(-4) in US format - Excel returns log2(4)+πi/ln(2) for negative reals
        let result = codcel_im_log2("-4".to_string(), ".", true).unwrap();
        assert!(result.contains("2") && result.contains("i"));
    }

    #[test]
    fn test_im_log2_zero() {
        // =IMLOG2(0) in US format
        // =IMLOG2(0) in German format
        let result = codcel_im_log2("0".to_string(), ".", true);
        assert!(result.is_err());
    }

    #[test]
    fn test_im_log2_decimal_separator() {
        // =IMLOG2(4) in US format
        // =IMLOG2(4) in German format
        let result = codcel_im_log2("4".to_string(), ",", true).unwrap();
        println!("{result}");
        assert_eq!(result, "2");
    }
}
