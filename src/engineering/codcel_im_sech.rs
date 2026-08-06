// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::engineering::complex::{format_complex, number_to_string, parse_complex};
use std::error::Error;

/// Excel-compatible `IMSECH` that returns the hyperbolic secant of a complex number.
/// - `complex`: a complex number string in the form `x+yi` or `x+yj`.
///   Returns sech(z) as a complex number string, or an error for invalid formats or division by zero.
pub fn codcel_im_sech(
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
                1.0 / crate::portable_math::cosh(real),
                decimal_separator,
                use_excel_rounding,
            ));
        }
        return Err("IMSECH: Invalid real number format".into());
    }

    // Parse out the real and imaginary parts
    let (real, imag) = parse_complex(&complex)?;

    // Calculate intermediate values
    let cosh_x = crate::portable_math::cosh(real);
    let sinh_x = crate::portable_math::sinh(real);
    let cos_y = crate::portable_math::cos(imag);
    let sin_y = crate::portable_math::sin(imag);

    // Calculate the denominator
    let denominator = crate::portable_math::cosh(2.0 * real) + crate::portable_math::cos(2.0 * imag);

    // Check for division by zero
    if denominator.abs() < 1e-14 {
        return Err("IMSECH: Division by zero".into());
    }

    // Calculate real and imaginary parts
    let real_part = 2.0 * cosh_x * cos_y / denominator;
    let imag_part = -2.0 * sinh_x * sin_y / denominator;

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
    fn test_im_sech_complex_number() {
        // =IMSECH("3+4i") in US format
        // =IMSECH("3+4i") in German format
        let result = codcel_im_sech("3+4i".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("-0.065294"));
    }

    #[test]
    fn test_im_sech_purely_real() {
        // =IMSECH("2") in US format
        // =IMSECH("2") in German format
        let result = codcel_im_sech("2".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!((result.parse::<f64>().unwrap() - (1.0 / 2.0_f64.cosh())).abs() < 0.0001);
    }

    #[test]
    fn test_im_sech_purely_imaginary() {
        // =IMSECH("4i") in US format
        // =IMSECH("4i") in German format
        let result = codcel_im_sech("4i".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("-1.5298856564664"));
    }

    #[test]
    fn test_im_sech_negative_real() {
        // =IMSECH("-3+4i") in US format
        // =IMSECH("-3+4i") in German format
        let result = codcel_im_sech("-3+4i".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("-0.0652940278579471-0.07522496"));
    }

    #[test]
    fn test_im_sech_j_symbol() {
        // =IMSECH("3+4j") in US format
        // =IMSECH("3+4j") in German format
        let result = codcel_im_sech("3+4j".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("-0.0652940278579471+0.075224"));
    }

    #[test]
    fn test_im_sech_invalid_input() {
        // =IMSECH("not_a_complex_number") in US format
        // =IMSECH("not_a_complex_number") in German format
        let result = codcel_im_sech("not_a_complex_number".to_string(), ".", true);
        assert!(result.is_err());
    }
}
