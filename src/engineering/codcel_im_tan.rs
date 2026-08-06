// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::engineering::complex::{format_complex, number_to_string, parse_complex};
use std::error::Error;

/// Excel-compatible `IMTAN` that returns the tangent of a complex number.
/// - `complex`: a complex number string in the form `x+yi` or `x+yj`.
///   Returns tan(z) as a complex number string, or an error for invalid formats or division by zero.
pub fn codcel_im_tan(
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
                crate::portable_math::tan(real),
                decimal_separator,
                use_excel_rounding,
            ));
        }
        return Err("IMTAN: Invalid real number format".into());
    }

    // Parse out the real and imaginary parts
    let (real, imag) = parse_complex(&complex)?;

    // Handle special cases
    if real == 0.0 && imag == 0.0 {
        return Ok("0".to_string());
    }

    // Calculate intermediate values for a more stable computation
    let two_x = 2.0 * real;
    let two_y = 2.0 * imag;

    let sin_2x = crate::portable_math::sin(two_x);
    let sinh_2y = crate::portable_math::sinh(two_y);
    let cos_2x = crate::portable_math::cos(two_x);
    let cosh_2y = crate::portable_math::cosh(two_y);

    // Calculate denominator
    let denominator = cos_2x + cosh_2y;

    // Check for division by zero
    if denominator.abs() < 1e-14 {
        return Err("IMTAN: Division by zero".into());
    }

    // Calculate the result using the formula
    let real_part = sin_2x / denominator;
    let imag_part = sinh_2y / denominator;

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
    fn test_im_tan_complex_number() {
        // =IMTAN("3+4i") in US format
        // =IMTAN("3+4i") in German format
        let result = codcel_im_tan("3+4i".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("-0.000187346204629478+0.999355987381473i"));
    }

    #[test]
    fn test_im_tan_purely_real() {
        // =IMTAN("2") in US format
        // =IMTAN("2") in German format
        let result = codcel_im_tan("2".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!((result.parse::<f64>().unwrap() - 2.0_f64.tan()).abs() < 0.0001);
    }

    #[test]
    fn test_im_tan_purely_imaginary() {
        // =IMTAN("4i") in US format
        // =IMTAN("4i") in German format
        let result = codcel_im_tan("4i".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("0.999329299739067i"));
    }

    #[test]
    fn test_im_tan_negative_real() {
        // =IMTAN("-3+4i") in US format
        // =IMTAN("-3+4i") in German format
        let result = codcel_im_tan("-3+4i".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("0.000187346204629478+0.999355987381473i"));
    }

    #[test]
    fn test_im_tan_j_symbol() {
        // =IMTAN("3+4j") in US format
        // =IMTAN("3+4j") in German format
        let result = codcel_im_tan("3+4j".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("-0.000187346204629478+0.999355987381473j"));
    }

    #[test]
    fn test_im_tan_zero() {
        // =IMTAN("0") in US format
        // =IMTAN("0") in German format
        let result = codcel_im_tan("0".to_string(), ".", true).unwrap();
        println!("{result}");
        assert_eq!(result, "0");
    }

    #[test]
    fn test_im_tan_invalid_input() {
        // =IMTAN("not_a_complex_number") in US format
        // =IMTAN("not_a_complex_number") in German format
        let result = codcel_im_tan("not_a_complex_number".to_string(), ".", true);
        assert!(result.is_err());
    }
}
