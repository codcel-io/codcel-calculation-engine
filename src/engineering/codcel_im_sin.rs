// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::engineering::complex::{format_complex, number_to_string, parse_complex};
use std::error::Error;

/// Excel-compatible `IMSIN` that returns the sine of a complex number.
/// - `complex`: a complex number string in the form `x+yi` or `x+yj`.
///   Returns sin(z) as a complex number string, or an error for invalid formats.
pub fn codcel_im_sin(
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
                real.sin(),
                decimal_separator,
                use_excel_rounding,
            ));
        }
        return Err("IMSIN: Invalid real number format".into());
    }

    // Parse out the real and imaginary parts
    let (real, imag) = parse_complex(&complex)?;

    // Calculate using the formula: sin(x + yi) = sin(x)cosh(y) + i*cos(x)sinh(y)
    let real_part = real.sin() * imag.cosh();
    let imag_part = real.cos() * imag.sinh();

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
    fn test_im_sin_complex_number() {
        // =IMSIN("3+4i") in US format
        // =IMSIN("3+4i") in German format
        let result = codcel_im_sin("3+4i".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("3.85373803791938-27.0168132580039i"));
    }

    #[test]
    fn test_im_sin_purely_real() {
        // =IMSIN("2") in US format
        // =IMSIN("2") in German format
        let result = codcel_im_sin("2".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!((result.parse::<f64>().unwrap() - 2.0_f64.sin()).abs() < 0.0001);
    }

    #[test]
    fn test_im_sin_purely_imaginary() {
        // =IMSIN("4i") in US format
        // =IMSIN("4i") in German format
        let result = codcel_im_sin("4i".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("27.2899171971278i"));
    }

    #[test]
    fn test_im_sin_negative_real() {
        // =IMSIN("-3+4i") in US format
        // =IMSIN("-3+4i") in German format
        let result = codcel_im_sin("-3+4i".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("-3.85373803791938-27.0168132580039i"));
    }

    #[test]
    fn test_im_sin_j_symbol() {
        // =IMSIN("3+4j") in US format
        // =IMSIN("3+4j") in German format
        let result = codcel_im_sin("3+4j".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("3.85373803791938-27.0168132580039j"));
    }

    #[test]
    fn test_im_sin_invalid_input() {
        // =IMSIN("not_a_complex_number") in US format
        // =IMSIN("not_a_complex_number") in German format
        let result = codcel_im_sin("not_a_complex_number".to_string(), ".", true);
        assert!(result.is_err());
    }
}
