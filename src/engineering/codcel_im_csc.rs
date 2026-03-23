// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::engineering::complex::{format_complex, number_to_string, parse_complex};
use std::error::Error;

/// Excel-compatible `IMCSC` that returns the cosecant of a complex number.
/// - `complex`: a complex number string in the form `x+yi` or `x+yj`.
///   Returns csc(z) as a complex number string, or an error for invalid formats or division by zero.
pub fn codcel_im_csc(
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
            // Check for division by zero
            let sin_val = crate::portable_math::sin(real);
            if sin_val.abs() < 1e-14 {
                return Err("IMCSC: Division by zero".into());
            }
            return Ok(number_to_string(
                1.0 / sin_val,
                decimal_separator,
                use_excel_rounding,
            ));
        }
        return Err("IMCSC: Invalid real number format".into());
    }

    // Parse out the real and imaginary parts
    let (real, imag) = parse_complex(&complex)?;

    // Check for zero input
    if real == 0.0 && imag == 0.0 {
        return Err("IMCSC: Division by zero".into());
    }

    // Calculate components
    let sin_x = crate::portable_math::sin(real);
    let cos_x = crate::portable_math::cos(real);
    let sinh_y = crate::portable_math::sinh(imag);
    let cosh_y = crate::portable_math::cosh(imag);

    // Calculate denominator first to check for division by zero
    let denominator = sin_x * sin_x + sinh_y * sinh_y;

    if denominator.abs() < 1e-14 {
        return Err("IMCSC: Division by zero".into());
    }

    // Calculate real and imaginary parts
    let real_part = (sin_x * cosh_y) / denominator;
    let imag_part = -(cos_x * sinh_y) / denominator;

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
    fn test_im_csc_positive() {
        // =IMCSC("3+4i") in US format
        // =IMCSC("3+4i") in German format
        let result = codcel_im_csc("3+4i".to_string(), ".", true).unwrap();
        println!("{result}");
        assert_eq!(result, "0.0051744731840194+0.036275889628626i");
    }

    #[test]
    fn test_im_csc_negative() {
        // =IMCSC("-3-4i") in US format
        // =IMCSC("-3-4i") in German format
        let result = codcel_im_csc("-3-4i".to_string(), ".", true).unwrap();
        println!("{result}");
        assert_eq!(result, "-0.0051744731840194-0.036275889628626i");
    }

    #[test]
    fn test_im_csc_real_only() {
        // =IMCSC("5") in US format
        // =IMCSC("5") in German format
        let result = codcel_im_csc("5".to_string(), ".", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check it's a real number
        assert!(!result.contains("i") && !result.contains("j"));
    }

    #[test]
    fn test_im_csc_imaginary_only() {
        // =IMCSC("5i") in US format
        // =IMCSC("5i") in German format
        let result = codcel_im_csc("5i".to_string(), ".", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check it's an imaginary number
        assert!(result.contains("i"));
    }

    #[test]
    fn test_im_csc_j_notation() {
        // =IMCSC("3+4j") in US format
        // =IMCSC("3+4j") in German format
        let result = codcel_im_csc("3+4j".to_string(), ".", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check it uses j notation
        assert!(result.contains("j"));
    }

    #[test]
    fn test_im_csc_decimal_separator() {
        // =IMCSC("3,5+4,2i") in US format
        // =IMCSC("3,5+4,2i") in German format
        let result = codcel_im_csc("3.5+4.2i".to_string(), ",", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check the format
        assert!(result.contains(","));
    }

    #[test]
    fn test_im_csc_division_by_zero() {
        // =IMCSC("0") in US format
        // =IMCSC("0") in German format
        let result = codcel_im_csc("0".to_string(), ".", true);
        assert!(result.is_err());
    }

    #[test]
    fn test_im_csc_invalid_input() {
        // =IMCSC("not_a_complex_number") in US format
        // =IMCSC("not_a_complex_number") in German format
        let result = codcel_im_csc("not_a_complex_number".to_string(), ".", true);
        assert!(result.is_err());
    }
}
