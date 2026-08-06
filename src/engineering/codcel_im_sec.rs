// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::engineering::complex::{format_complex, number_to_string, parse_complex};
use std::error::Error;

/// Excel-compatible `IMSEC` that returns the secant of a complex number.
/// - `complex`: a complex number string in the form `x+yi` or `x+yj`.
///   Returns sec(z) as a complex number string, or an error for invalid formats or division by zero.
pub fn codcel_im_sec(
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
                1.0 / crate::portable_math::cos(real),
                decimal_separator,
                use_excel_rounding,
            ));
        }
        return Err("IMSEC: Invalid real number format".into());
    }

    // Parse out the real and imaginary parts
    let (real, imag) = parse_complex(&complex)?;

    // Calculate intermediate values
    let cos_x = crate::portable_math::cos(real);
    let sin_x = crate::portable_math::sin(real);
    let cosh_y = crate::portable_math::cosh(imag);
    let sinh_y = crate::portable_math::sinh(imag);

    // Calculate the denominator
    let denominator = crate::portable_math::cos(2.0 * real) + crate::portable_math::cosh(2.0 * imag);

    // Check for division by zero
    if denominator.abs() < 1e-14 {
        return Err("IMSEC: Division by zero".into());
    }

    // Calculate real and imaginary parts
    let real_part = 2.0 * cos_x * cosh_y / denominator;
    let imag_part = 2.0 * sin_x * sinh_y / denominator;

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
    fn test_im_sec_complex_number() {
        // =IMSEC("3+4i") in US format
        // =IMSEC("3+4i") in German format
        let result = codcel_im_sec("3+4i".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("-0.0362534969158689+0.00516434460775318i"));
    }

    #[test]
    fn test_im_sec_purely_real() {
        // =IMSEC("2") in US format
        // =IMSEC("2") in German format
        let result = codcel_im_sec("2".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!((result.parse::<f64>().unwrap() - (1.0 / 2.0_f64.cos())).abs() < 0.0001);
    }

    #[test]
    fn test_im_sec_purely_imaginary() {
        // =IMSEC("4i") in US format
        // =IMSEC("4i") in German format
        let result = codcel_im_sec("4i".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("0.0366189934736865"));
    }

    #[test]
    fn test_im_sec_negative_real() {
        // =IMSEC("-3+4i") in US format
        // =IMSEC("-3+4i") in German format
        let result = codcel_im_sec("-3+4i".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("-0.0362534969158689-0.00516434460775318i"));
    }

    #[test]
    fn test_im_sec_j_symbol() {
        // =IMSEC("3+4j") in US format
        // =IMSEC("3+4j") in German format
        let result = codcel_im_sec("3+4j".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("-0.0362534969158689+0.00516434460775318j"));
    }

    #[test]
    fn test_im_sec_invalid_input() {
        // =IMSEC("not_a_complex_number") in US format
        // =IMSEC("not_a_complex_number") in German format
        let result = codcel_im_sec("not_a_complex_number".to_string(), ".", true);
        assert!(result.is_err());
    }
}
