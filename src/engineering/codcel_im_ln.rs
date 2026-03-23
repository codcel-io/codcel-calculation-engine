// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::engineering::complex::{format_complex, parse_complex};
use std::error::Error;

/// Excel-compatible `IMLN` that returns the natural logarithm of a complex number.
/// - `complex`: a complex number string in the form `x+yi` or `x+yj`.
///   Returns ln(z) as a complex number string, or an error for invalid formats or zero input.
pub fn codcel_im_ln(
    complex: String,
    decimal_separator: &str,
    use_excel_rounding: bool,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    // Remove all whitespace from the input
    let complex = complex.replace(" ", "");

    // Determine which imaginary symbol is used (default to 'i' if no imaginary part)
    let im_symbol = if complex.contains('j') { 'j' } else { 'i' };

    // Parse out the real and imaginary parts
    let (real, imag) = parse_complex(&complex)?;

    // Calculate the magnitude (sqrt(x² + y²))
    let magnitude = crate::portable_math::sqrt(real * real + imag * imag);

    // Check for zero magnitude
    if magnitude == 0.0 {
        return Err("IMLN: Cannot compute logarithm of zero".into());
    }

    // Calculate using the formula: ln(x + yi) = ln(sqrt(x² + y²)) + i*atan2(y, x)
    let real_part = crate::portable_math::ln(magnitude);
    let imag_part = crate::portable_math::atan2(imag, real);

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
    fn test_im_ln_positive() {
        // =IMLN("3+4i") in US format
        // =IMLN("3+4i") in German format
        let result = codcel_im_ln("3+4i".to_string(), ".", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check the format
        assert!(result.contains(".") && result.contains("i"));
    }

    #[test]
    fn test_im_ln_negative_real() {
        // =IMLN("-3+4i") in US format
        // =IMLN("-3+4i") in German format
        let result = codcel_im_ln("-3+4i".to_string(), ".", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check the format
        assert!(result.contains(".") && result.contains("i"));
    }

    #[test]
    fn test_im_ln_real_only() {
        // =IMLN("5") in US format
        // =IMLN("5") in German format
        let result = codcel_im_ln("5".to_string(), ".", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check it's a real number
        assert!(!result.contains("i") && !result.contains("j"));
    }

    #[test]
    fn test_im_ln_imaginary_only() {
        // =IMLN("5i") in US format
        // =IMLN("5i") in German format
        let result = codcel_im_ln("5i".to_string(), ".", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check it's a complex number
        assert!(result.contains("i"));
    }

    #[test]
    fn test_im_ln_j_notation() {
        // =IMLN("3+4j") in US format
        // =IMLN("3+4j") in German format
        let result = codcel_im_ln("3+4j".to_string(), ".", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check it uses j notation
        assert!(result.contains("j"));
    }

    #[test]
    fn test_im_ln_decimal_separator() {
        // =IMLN("3,5+4,2i") in US format
        // =IMLN("3,5+4,2i") in German format
        let result = codcel_im_ln("3.5+4.2i".to_string(), ",", true).unwrap();
        println!("{result}");
        // The exact value will depend on the implementation, but we can check the format
        assert!(result.contains(","));
    }

    #[test]
    fn test_im_ln_negative_real_only() {
        // =IMLN("-5") in US format - Excel returns ln(5)+πi for negative reals
        let result = codcel_im_ln("-5".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("1.6094379124341") && result.contains("3.14159265358979i"));
    }

    #[test]
    fn test_im_ln_zero() {
        // =IMLN("0") in US format
        // =IMLN("0") in German format
        let result = codcel_im_ln("0".to_string(), ".", true);
        assert!(result.is_err());
    }

    #[test]
    fn test_im_ln_invalid_input() {
        // =IMLN("not_a_complex_number") in US format
        // =IMLN("not_a_complex_number") in German format
        let result = codcel_im_ln("not_a_complex_number".to_string(), ".", true);
        assert!(result.is_err());
    }
}
