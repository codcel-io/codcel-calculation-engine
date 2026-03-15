// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::engineering::complex::{format_complex, number_to_string, parse_complex};
use std::error::Error;

/// Excel-compatible `IMCONJUGATE` that returns the complex conjugate of a complex number.
/// - `complex`: a complex number string in the form `x+yi` or `x+yj`.
///   Returns the conjugate (e.g., `"3-4i"` for `"3+4i"`), or an error for invalid formats.
pub fn codcel_im_conjugate(
    complex: String,
    decimal_separator: &str,
    use_excel_rounding: bool,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    // Remove all whitespace from the input
    let complex = complex.replace(" ", "");

    // For a purely real number, return the same number
    if !complex.contains('i') && !complex.contains('j') {
        if complex.parse::<f64>().is_ok() {
            let complex = complex.parse::<f64>()?;
            return Ok(number_to_string(
                complex,
                decimal_separator,
                use_excel_rounding,
            ));
        }
        return Err("IMCONJUGATE: Invalid real number format".into());
    }

    // Determine which imaginary symbol is used
    let im_symbol = if complex.contains('j') { 'j' } else { 'i' };

    // Parse the complex number
    let (real, imag) = parse_complex(&complex)?;

    // Format with negated imaginary part
    format_complex(
        real,
        -imag,
        im_symbol,
        decimal_separator,
        use_excel_rounding,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_im_conjugate_positive() {
        // =IMCONJUGATE("3+4i") in US format
        // =IMCONJUGATE("3+4i") in German format
        let result = codcel_im_conjugate("3+4i".to_string(), ".", true).unwrap();
        println!("{result}");
        assert_eq!(result, "3-4i");
    }

    #[test]
    fn test_im_conjugate_negative() {
        // =IMCONJUGATE("-3-4i") in US format
        // =IMCONJUGATE("-3-4i") in German format
        let result = codcel_im_conjugate("-3-4i".to_string(), ".", true).unwrap();
        println!("{result}");
        assert_eq!(result, "-3+4i");
    }

    #[test]
    fn test_im_conjugate_real_only() {
        // =IMCONJUGATE("5") in US format
        // =IMCONJUGATE("5") in German format
        let result = codcel_im_conjugate("5".to_string(), ".", true).unwrap();
        println!("{result}");
        assert_eq!(result, "5");
    }

    #[test]
    fn test_im_conjugate_imaginary_only() {
        // =IMCONJUGATE("5i") in US format
        // =IMCONJUGATE("5i") in German format
        let result = codcel_im_conjugate("5i".to_string(), ".", true).unwrap();
        println!("{result}");
        assert_eq!(result, "-5i");
    }

    #[test]
    fn test_im_conjugate_j_notation() {
        // =IMCONJUGATE("3+4j") in US format
        // =IMCONJUGATE("3+4j") in German format
        let result = codcel_im_conjugate("3+4j".to_string(), ".", true).unwrap();
        println!("{result}");
        assert_eq!(result, "3-4j");
    }

    #[test]
    fn test_im_conjugate_decimal_separator() {
        // =IMCONJUGATE("3,5+4,2i") in US format
        // =IMCONJUGATE("3,5+4,2i") in German format
        let result = codcel_im_conjugate("3.5+4.2i".to_string(), ",", true).unwrap();
        println!("{result}");
        assert_eq!(result, "3,5-4,2i");
    }

    #[test]
    fn test_im_conjugate_invalid_input() {
        // =IMCONJUGATE("not_a_complex_number") in US format
        // =IMCONJUGATE("not_a_complex_number") in German format
        let result = codcel_im_conjugate("not_a_complex_number".to_string(), ".", true);
        assert!(result.is_err());
    }
}
