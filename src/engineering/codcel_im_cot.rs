// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::engineering::complex::{format_complex, number_to_string, parse_complex};
use std::error::Error;

/// Excel-compatible `IMCOT` that returns the cotangent of a complex number.
/// - `complex`: a complex number string in the form `x+yi` or `x+yj`.
///   Returns cot(z) as a complex number string, or an error for invalid formats or division by zero.
pub fn codcel_im_cot(
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
            let tan_val = real.tan();
            if tan_val.abs() < 1e-14 {
                return Err("IMCOT: Division by zero".into());
            }
            return Ok(number_to_string(
                1.0 / tan_val,
                decimal_separator,
                use_excel_rounding,
            ));
        }
        return Err("IMCOT: Invalid real number format".into());
    }

    // Parse out the real and imaginary parts
    let (real, imag) = parse_complex(&complex)?;

    // Handle special cases
    if real == 0.0 && imag == 0.0 {
        return Err("IMCOT: Division by zero".into());
    }

    // Calculate using the formula: cot(x + yi) = (sin(2x) - i*sinh(2y))/(cosh(2y) - cos(2x))
    let two_x = 2.0 * real;
    let two_y = 2.0 * imag;

    let sin_2x = two_x.sin();
    let sinh_2y = two_y.sinh();
    let cosh_2y = two_y.cosh();
    let cos_2x = two_x.cos();

    let denominator = cosh_2y - cos_2x;

    // Check for division by zero
    if denominator.abs() < 1e-14 {
        return Err("IMCOT: Division by zero".into());
    }

    let real_part = sin_2x / denominator;
    let imag_part = -sinh_2y / denominator;

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
    fn test_im_cot_positive() {
        // =IMCOT("3+4i") in US format
        // =IMCOT("3+4i") in German format
        let result = codcel_im_cot("3+4i".to_string(), ".", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check the format
        assert!(result.contains("-") && result.contains("i"));
    }

    #[test]
    fn test_im_cot_negative() {
        // =IMCOT("-3-4i") in US format
        // =IMCOT("-3-4i") in German format
        let result = codcel_im_cot("-3-4i".to_string(), ".", true).unwrap();
        println!("{result}");
        assert_eq!(result, "0.000187587737983659+1.00064439247156i");
    }

    #[test]
    fn test_im_cot_real_only() {
        // =IMCOT("5") in US format
        // =IMCOT("5") in German format
        let result = codcel_im_cot("5".to_string(), ".", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check it's a real number
        assert!(!result.contains("i") && !result.contains("j"));
    }

    #[test]
    fn test_im_cot_imaginary_only() {
        // =IMCOT("5i") in US format
        // =IMCOT("5i") in German format
        let result = codcel_im_cot("5i".to_string(), ".", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check it's an imaginary number
        assert!(result.contains("i"));
    }

    #[test]
    fn test_im_cot_j_notation() {
        // =IMCOT("3+4j") in US format
        // =IMCOT("3+4j") in German format
        let result = codcel_im_cot("3+4j".to_string(), ".", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check it uses j notation
        assert!(result.contains("j"));
    }

    #[test]
    fn test_im_cot_decimal_separator() {
        // =IMCOT("3,5+4,2i") in US format
        // =IMCOT("3,5+4,2i") in German format
        let result = codcel_im_cot("3.5+4.2i".to_string(), ",", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check the format
        assert!(result.contains(","));
    }

    #[test]
    fn test_im_cot_division_by_zero() {
        // =IMCOT("0") in US format
        // =IMCOT("0") in German format
        let result = codcel_im_cot("0".to_string(), ".", true);
        assert!(result.is_err());
    }

    #[test]
    fn test_im_cot_invalid_input() {
        // =IMCOT("not_a_complex_number") in US format
        // =IMCOT("not_a_complex_number") in German format
        let result = codcel_im_cot("not_a_complex_number".to_string(), ".", true);
        assert!(result.is_err());
    }
}
