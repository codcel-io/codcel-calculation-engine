// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::engineering::complex::{format_complex, number_to_string, parse_complex};
use std::error::Error;

/// Excel-compatible `IMCOSH` that returns the hyperbolic cosine of a complex number.
/// - `complex`: a complex number string in the form `x+yi` or `x+yj`.
///   Returns cosh(z) as a complex number string, or an error for invalid formats.
pub fn codcel_im_cosh(
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
                crate::portable_math::cosh(real),
                decimal_separator,
                use_excel_rounding,
            ));
        }
        return Err("IMCOSH: Invalid real number format".into());
    }

    // Parse out the real and imaginary parts
    let (real, imag) = parse_complex(&complex)?;

    // Calculate the complex hyperbolic cosine using the formula:
    // cosh(x + yi) = cosh(x)cos(y) + i*sinh(x)sin(y)
    let real_part = crate::portable_math::cosh(real) * crate::portable_math::cos(imag);
    let imag_part = crate::portable_math::sinh(real) * crate::portable_math::sin(imag);

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
    fn test_im_cosh_positive() {
        // =IMCOSH("3+4i") in US format
        // =IMCOSH("3+4i") in German format
        let result = codcel_im_cosh("3+4i".to_string(), ".", true).unwrap();
        println!("{result}");
        // Last digit may differ by 1 ULP across platforms/math backends (macOS: ...4655, libm: ...4654)
        assert!(
            result == "-6.58066304055116-7.58155274274655i"
                || result == "-6.58066304055116-7.58155274274654i"
        );
    }

    #[test]
    fn test_im_cosh_negative() {
        // =IMCOSH("-3-4i") in US format
        // =IMCOSH("-3-4i") in German format
        let result = codcel_im_cosh("-3-4i".to_string(), ".", true).unwrap();
        println!("{result}");
        // Last digit may differ by 1 ULP across platforms/math backends (macOS: ...4655, libm: ...4654)
        assert!(
            result == "-6.58066304055116-7.58155274274655i"
                || result == "-6.58066304055116-7.58155274274654i"
        );
    }

    #[test]
    fn test_im_cosh_real_only() {
        // =IMCOSH("5") in US format
        // =IMCOSH("5") in German format
        let result = codcel_im_cosh("5".to_string(), ".", true).unwrap();
        println!("{result}");
        assert_eq!(result, "74.2099485247878");
    }

    #[test]
    fn test_im_cosh_imaginary_only() {
        // =IMCOSH("5i") in US format
        // =IMCOSH("5i") in German format
        let result = codcel_im_cosh("5i".to_string(), ".", true).unwrap();
        println!("{result}");
        assert_eq!(result, "0.283662185463226");
    }

    #[test]
    fn test_im_cosh_j_notation() {
        // =IMCOSH("3+4j") in US format
        // =IMCOSH("3+4j") in German format
        let result = codcel_im_cosh("3+4j".to_string(), ".", true).unwrap();
        println!("{result}");
        // Last digit may differ by 1 ULP across platforms/math backends (macOS: ...4655, libm: ...4654)
        assert!(
            result == "-6.58066304055116-7.58155274274655j"
                || result == "-6.58066304055116-7.58155274274654j"
        );
    }

    #[test]
    fn test_im_cosh_decimal_separator() {
        // =IMCOSH("3,5+4,2i") in US format
        // =IMCOSH("3,5+4,2i") in German format
        let result = codcel_im_cosh("3.5+4.2i".to_string(), ",", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check the format
        assert!(result.contains(","));
    }

    #[test]
    fn test_im_cosh_invalid_input() {
        // =IMCOSH("not_a_complex_number") in US format
        // =IMCOSH("not_a_complex_number") in German format
        let result = codcel_im_cosh("not_a_complex_number".to_string(), ".", true);
        assert!(result.is_err());
    }
}
