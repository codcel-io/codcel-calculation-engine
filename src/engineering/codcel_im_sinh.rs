// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::engineering::complex::{format_complex, number_to_string, parse_complex};
use std::error::Error;

/// Excel-compatible `IMSINH` that returns the hyperbolic sine of a complex number.
/// - `complex`: a complex number string in the form `x+yi` or `x+yj`.
///   Returns sinh(z) as a complex number string, or an error for invalid formats.
pub fn codcel_im_sinh(
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
                crate::portable_math::sinh(real),
                decimal_separator,
                use_excel_rounding,
            ));
        }
        return Err("IMSINH: Invalid real number format".into());
    }

    // Parse out the real and imaginary parts
    let (real, imag) = parse_complex(&complex)?;

    // Calculate using the formula: sinh(x + yi) = sinh(x)cos(y) + i*cosh(x)sin(y)
    let real_part = crate::portable_math::sinh(real) * crate::portable_math::cos(imag);
    let imag_part = crate::portable_math::cosh(real) * crate::portable_math::sin(imag);

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
    fn test_im_sinh_complex_number() {
        // =IMSINH("3+4i") in US format
        // =IMSINH("3+4i") in German format
        let result = codcel_im_sinh("3+4i".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("-6.548120040911-7.61923172032141i"));
    }

    #[test]
    fn test_im_sinh_purely_real() {
        // =IMSINH("2") in US format
        // =IMSINH("2") in German format
        let result = codcel_im_sinh("2".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!((result.parse::<f64>().unwrap() - 2.0_f64.sinh()).abs() < 0.0001);
    }

    #[test]
    fn test_im_sinh_purely_imaginary() {
        // =IMSINH("4i") in US format
        // =IMSINH("4i") in German format
        let result = codcel_im_sinh("4i".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("-0.756802495307928i"));
    }

    #[test]
    fn test_im_sinh_negative_real() {
        // =IMSINH("-3+4i") in US format
        // =IMSINH("-3+4i") in German format
        let result = codcel_im_sinh("-3+4i".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("6.548120040911-7.61923172032141i"));
    }

    #[test]
    fn test_im_sinh_j_symbol() {
        // =IMSINH("3+4j") in US format
        // =IMSINH("3+4j") in German format
        let result = codcel_im_sinh("3+4j".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("-6.548120040911-7.61923172032141j"));
    }

    #[test]
    fn test_im_sinh_invalid_input() {
        // =IMSINH("not_a_complex_number") in US format
        // =IMSINH("not_a_complex_number") in German format
        let result = codcel_im_sinh("not_a_complex_number".to_string(), ".", true);
        assert!(result.is_err());
    }
}
