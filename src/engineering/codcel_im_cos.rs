// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::engineering::complex::{format_complex, number_to_string, parse_complex};
use std::error::Error;

/// Excel-compatible `IMCOS` that returns the cosine of a complex number.
/// - `complex`: a complex number string in the form `x+yi` or `x+yj`.
///   Returns cos(z) as a complex number string, or an error for invalid formats.
pub fn codcel_im_cos(
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
                crate::portable_math::cos(real),
                decimal_separator,
                use_excel_rounding,
            ));
        }
        return Err("IMCOS: Invalid real number format".into());
    }

    // Parse out the real and imaginary parts
    let (real, imag) = parse_complex(&complex)?;

    // Calculate the complex cosine using the formula:
    // cos(x + yi) = cos(x)cosh(y) - i*sin(x)sinh(y)
    let real_part = crate::portable_math::cos(real) * crate::portable_math::cosh(imag);
    let imag_part = -(crate::portable_math::sin(real) * crate::portable_math::sinh(imag));

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
    fn test_im_cos_positive() {
        // =IMCOS("3+4i") in US format
        // =IMCOS("3+4i") in German format
        let result = codcel_im_cos("3+4i".to_string(), ".", true).unwrap();
        println!("{result}");
        assert_eq!(result, "-27.0349456030742-3.85115333481178i");
    }

    #[test]
    fn test_im_cos_negative() {
        // =IMCOS("-3-4i") in US format
        // =IMCOS("-3-4i") in German format
        let result = codcel_im_cos("-3-4i".to_string(), ".", true).unwrap();
        println!("{result}");
        assert_eq!(result, "-27.0349456030742-3.85115333481178i");
    }

    #[test]
    fn test_im_cos_real_only() {
        // =IMCOS("5") in US format
        // =IMCOS("5") in German format
        let result = codcel_im_cos("5".to_string(), ".", true).unwrap();
        println!("{result}");
        assert_eq!(result, "0.283662185463226");
    }

    #[test]
    fn test_im_cos_imaginary_only() {
        // =IMCOS("5i") in US format
        // =IMCOS("5i") in German format
        let result = codcel_im_cos("5i".to_string(), ".", true).unwrap();
        println!("{result}");
        assert_eq!(result, "74.2099485247878");
    }

    #[test]
    fn test_im_cos_j_notation() {
        // =IMCOS("3+4j") in US format
        // =IMCOS("3+4j") in German format
        let result = codcel_im_cos("3+4j".to_string(), ".", true).unwrap();
        println!("{result}");
        assert_eq!(result, "-27.0349456030742-3.85115333481178j");
    }

    #[test]
    fn test_im_cos_decimal_separator() {
        // =IMCOS("3,5+4,2i") in US format
        // =IMCOS("3,5+4,2i") in German format
        let result = codcel_im_cos("3.5+4.2i".to_string(), ",", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check the format
        assert!(result.contains(","));
    }

    #[test]
    fn test_im_cos_invalid_input() {
        // =IMCOS("not_a_complex_number") in US format
        // =IMCOS("not_a_complex_number") in German format
        let result = codcel_im_cos("not_a_complex_number".to_string(), ".", true);
        assert!(result.is_err());
    }
}
