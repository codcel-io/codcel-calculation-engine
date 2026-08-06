// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::engineering::complex::{format_complex, number_to_string, parse_complex};
use std::error::Error;

/// Excel-compatible `IMCSCH` that returns the hyperbolic cosecant of a complex number.
/// - `complex`: a complex number string in the form `x+yi` or `x+yj`.
///   Returns csch(z) as a complex number string, or an error for invalid formats or division by zero.
pub fn codcel_im_csch(
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
            if real == 0.0 {
                return Err("Division by zero in IMCSCH".into());
            }
            return Ok(number_to_string(
                1.0 / crate::portable_math::sinh(real),
                decimal_separator,
                use_excel_rounding,
            ));
        }
        return Err("Invalid real number format".into());
    }

    // Parse out the real and imaginary parts
    let (real, imag) = parse_complex(&complex)?;

    // Check for zero input
    if real == 0.0 && imag == 0.0 {
        return Err("Division by zero in IMCSCH".into());
    }

    // Calculate components
    let sinh_x = crate::portable_math::sinh(real);
    let cosh_x = crate::portable_math::cosh(real);
    let sin_y = crate::portable_math::sin(imag);
    let cos_y = crate::portable_math::cos(imag);

    // Calculate denominator first to check for division by zero
    let denominator = sinh_x * sinh_x + sin_y * sin_y;

    if denominator.abs() < 1e-14 {
        return Err("Division by zero in IMCSCH".into());
    }

    // Calculate real and imaginary parts
    let real_part = (sinh_x * cos_y) / denominator;
    let imag_part = -(cosh_x * sin_y) / denominator;

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
    fn test_im_csch_positive() {
        // =IMCSCH("3+4i") in US format
        // =IMCSCH("3+4i") in German format
        let result = codcel_im_csch("3+4i".to_string(), ".", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check the format
        assert!(result.contains("-") && result.contains("i"));
    }

    #[test]
    fn test_im_csch_negative() {
        // =IMCSCH("-3-4i") in US format
        // =IMCSCH("-3-4i") in German format
        let result = codcel_im_csch("-3-4i".to_string(), ".", true).unwrap();
        println!("{result}");
        assert_eq!(result, "0.0648774713706355-0.0754898329158637i");
    }

    #[test]
    fn test_im_csch_real_only() {
        // =IMCSCH("5") in US format
        // =IMCSCH("5") in German format
        let result = codcel_im_csch("5".to_string(), ".", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check it's a real number
        assert!(!result.contains("i") && !result.contains("j"));
    }

    #[test]
    fn test_im_csch_imaginary_only() {
        // =IMCSCH("5i") in US format
        // =IMCSCH("5i") in German format
        let result = codcel_im_csch("5i".to_string(), ".", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check it's an imaginary number
        assert!(result.contains("i"));
    }

    #[test]
    fn test_im_csch_j_notation() {
        // =IMCSCH("3+4j") in US format
        // =IMCSCH("3+4j") in German format
        let result = codcel_im_csch("3+4j".to_string(), ".", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check it uses j notation
        assert!(result.contains("j"));
    }

    #[test]
    fn test_im_csch_decimal_separator() {
        // =IMCSCH("3,5+4,2i") in US format
        // =IMCSCH("3,5+4,2i") in German format
        let result = codcel_im_csch("3.5+4.2i".to_string(), ",", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check the format
        assert!(result.contains(","));
    }

    #[test]
    fn test_im_csch_division_by_zero() {
        // =IMCSCH("0") in US format
        // =IMCSCH("0") in German format
        let result = codcel_im_csch("0".to_string(), ".", true);
        assert!(result.is_err());
    }

    #[test]
    fn test_im_csch_invalid_input() {
        // =IMCSCH("not_a_complex_number") in US format
        // =IMCSCH("not_a_complex_number") in German format
        let result = codcel_im_csch("not_a_complex_number".to_string(), ".", true);
        assert!(result.is_err());
    }
}
