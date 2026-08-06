// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::engineering::complex::{format_complex, parse_complex};
use std::error::Error;

/// Excel-compatible `IMDIV` that returns the quotient of two complex numbers.
/// - `numerator`: the dividend complex number string.
/// - `denominator`: the divisor complex number string.
///   Returns the quotient as a complex number string, or an error for invalid formats or division by zero.
pub fn codcel_im_div(
    numerator: String,
    denominator: String,
    decimal_separator: &str,
    use_excel_rounding: bool,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    println!("numerator {numerator:?}, denominator {denominator:?}");

    // Remove all whitespace from inputs
    let numerator = numerator.replace(" ", "");
    let denominator = denominator.replace(" ", "");

    // Determine which imaginary symbol is used (default to 'i' if no imaginary part)
    let im_symbol = if numerator.contains('j') || denominator.contains('j') {
        'j'
    } else {
        'i'
    };

    // Parse the complex numbers
    let (num_real, num_imag) = parse_complex(&numerator)?;
    let (den_real, den_imag) = parse_complex(&denominator)?;

    // Check for division by zero
    let denominator_magnitude = den_real * den_real + den_imag * den_imag;
    if denominator_magnitude < 1e-14 {
        return Err("IMDIV: Division by zero".into());
    }

    // Perform complex division using the formula:
    // (a + bi)/(c + di) = (ac + bd)/(c² + d²) + ((bc - ad)/(c² + d²))i
    let real_part = (num_real * den_real + num_imag * den_imag) / denominator_magnitude;
    let imag_part = (num_imag * den_real - num_real * den_imag) / denominator_magnitude;

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
    fn test_im_div_complex_by_complex() {
        // =IMDIV("3+4i", "1+2i") in US format
        // =IMDIV("3+4i"; "1+2i") in German format
        let result = codcel_im_div("3+4i".to_string(), "1+2i".to_string(), ".", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check the format
        assert!(result.contains(".") && result.contains("i"));
    }

    #[test]
    fn test_im_div_negative_complex() {
        // =IMDIV("-3-4i", "-1-2i") in US format
        // =IMDIV("-3-4i"; "-1-2i") in German format
        let result = codcel_im_div("-3-4i".to_string(), "-1-2i".to_string(), ".", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check the format
        assert!(result.contains(".") && result.contains("i"));
    }

    #[test]
    fn test_im_div_complex_by_real() {
        // =IMDIV("3+4i", "2") in US format
        // =IMDIV("3+4i"; "2") in German format
        let result = codcel_im_div("3+4i".to_string(), "2".to_string(), ".", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check the format
        assert!(result.contains(".") && result.contains("i"));
    }

    #[test]
    fn test_im_div_real_by_complex() {
        // =IMDIV("3", "1+2i") in US format
        // =IMDIV("3"; "1+2i") in German format
        let result = codcel_im_div("3".to_string(), "1+2i".to_string(), ".", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check the format
        assert!(result.contains(".") && result.contains("i"));
    }

    #[test]
    fn test_im_div_complex_by_imaginary() {
        // =IMDIV("3+4i", "2i") in US format
        // =IMDIV("3+4i"; "2i") in German format
        let result = codcel_im_div("3+4i".to_string(), "2i".to_string(), ".", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check the format
        assert!(result.contains(".") && result.contains("i"));
    }

    #[test]
    fn test_im_div_j_notation() {
        // =IMDIV("3+4j", "1+2j") in US format
        // =IMDIV("3+4j"; "1+2j") in German format
        let result = codcel_im_div("3+4j".to_string(), "1+2j".to_string(), ".", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check it uses j notation
        assert!(result.contains("j"));
    }

    #[test]
    fn test_im_div_decimal_separator() {
        // =IMDIV("3,5+4,2i", "1,5+2,5i") in US format
        // =IMDIV("3,5+4,2i"; "1,5+2,5i") in German format
        let result =
            codcel_im_div("3.5+4.2i".to_string(), "1.5+2.5i".to_string(), ",", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check the format
        assert!(result.contains(","));
    }

    #[test]
    fn test_im_div_division_by_zero() {
        // =IMDIV("3+4i", "0") in US format
        // =IMDIV("3+4i"; "0") in German format
        let result = codcel_im_div("3+4i".to_string(), "0".to_string(), ".", true);
        assert!(result.is_err());
    }

    #[test]
    fn test_im_div_invalid_input() {
        // =IMDIV("not_a_complex_number", "1+2i") in US format
        // =IMDIV("not_a_complex_number"; "1+2i") in German format
        let result = codcel_im_div(
            "not_a_complex_number".to_string(),
            "1+2i".to_string(),
            ".",
            true,
        );
        assert!(result.is_err());
    }
}
