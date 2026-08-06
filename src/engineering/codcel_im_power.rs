// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::engineering::complex::{
    format_complex, format_complex_with_precision, number_to_string, parse_complex,
};
use std::error::Error;

/// Excel-compatible `IMPOWER` that returns a complex number raised to a power.
/// - `complex`: a complex number string in the form `x+yi` or `x+yj`.
/// - `power`: the exponent (a real number).
///   Returns z^n as a complex number string, or an error for invalid formats.
pub fn codcel_im_power(
    complex: String,
    power: String,
    decimal_separator: &str,
    use_excel_rounding: bool,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    // Remove all whitespace from inputs
    let complex = complex.replace(" ", "");

    // Parse the power
    let n = power
        .trim()
        .parse::<f64>()
        .map_err(|_| "IMPOWER: Invalid power format")?;

    // Determine which imaginary symbol is used (default to 'i' if no imaginary part)
    let im_symbol = if complex.contains('j') { 'j' } else { 'i' };

    // For a purely real number
    if !complex.contains('i') && !complex.contains('j') {
        if let Ok(real) = complex.parse::<f64>() {
            return if real < 0.0 && n.fract() == 0.0 {
                // Handle negative real numbers with integer powers
                return Ok(number_to_string(
                    crate::portable_math::powf(real, n),
                    decimal_separator,
                    use_excel_rounding,
                ));
            } else if real < 0.0 {
                // For negative real with non-integer power, treat as complex number
                let (mag, arg) = (real.abs(), std::f64::consts::PI);
                let new_mag = crate::portable_math::powf(mag, n);
                let new_arg = n * arg;
                let real_part = new_mag * crate::portable_math::cos(new_arg);
                let imag_part = new_mag * crate::portable_math::sin(new_arg);
                format_complex(
                    real_part,
                    imag_part,
                    im_symbol,
                    decimal_separator,
                    use_excel_rounding,
                )
            } else {
                return Ok(number_to_string(
                    crate::portable_math::powf(real, n),
                    decimal_separator,
                    use_excel_rounding,
                ));
            };
        }
        return Err("IMPOWER: Invalid real number format".into());
    }

    // Parse out the real and imaginary parts
    let (real, imag) = parse_complex(&complex)?;

    // Calculate magnitude and argument of the original complex number
    let magnitude = crate::portable_math::sqrt(real * real + imag * imag);

    // For zero magnitude
    if magnitude == 0.0 && n > 0.0 {
        return Ok("0".to_string());
    }

    // Calculate argument, being careful with edge cases
    let argument = if real == 0.0 && imag > 0.0 {
        std::f64::consts::PI / 2.0
    } else if real == 0.0 && imag < 0.0 {
        -std::f64::consts::PI / 2.0
    } else {
        crate::portable_math::atan2(imag, real)
    };

    // Calculate new magnitude and argument
    let new_magnitude = crate::portable_math::powf(magnitude, n);
    let new_argument = n * argument;

    // Convert back to real and imaginary parts using the new angle
    let real_part = new_magnitude * crate::portable_math::cos(new_argument);
    let imag_part = new_magnitude * crate::portable_math::sin(new_argument);

    // Excel does not zero out very small floating-point noise in IMPOWER results,
    // so use epsilon=0 to preserve all computed values.
    format_complex_with_precision(
        real_part,
        imag_part,
        im_symbol,
        0.0,
        decimal_separator,
        use_excel_rounding,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_im_power_real_positive() {
        // =IMPOWER(2, 3) in US format
        // =IMPOWER(2; 3) in German format
        let result = codcel_im_power("2".to_string(), "3".to_string(), ".", true).unwrap();
        println!("{result}");
        assert_eq!(result, "8");
    }

    #[test]
    fn test_im_power_real_negative_integer() {
        // =IMPOWER(-2, 3) in US format
        // =IMPOWER(-2; 3) in German format
        let result = codcel_im_power("-2".to_string(), "3".to_string(), ".", true).unwrap();
        println!("{result}");
        assert_eq!(result, "-8");
    }

    #[test]
    fn test_im_power_real_negative_non_integer() {
        // =IMPOWER(-2, 0.5) in US format
        // =IMPOWER(-2; 0,5) in German format
        let result = codcel_im_power("-2".to_string(), "0.5".to_string(), ".", true).unwrap();
        println!("{result}");
        // The result will be a complex number
        assert!(result.contains("i") || result.contains("j"));
    }

    #[test]
    fn test_im_power_complex() {
        // =IMPOWER("3+4i", 2) in US format
        // =IMPOWER("3+4i"; 2) in German format
        let result = codcel_im_power("3+4i".to_string(), "2".to_string(), ".", true).unwrap();
        println!("{result}");
        // The result should be -7+24i
        assert!(result.contains("-7") && result.contains("24i"));
    }

    #[test]
    fn test_im_power_i_to_4() {
        // =IMPOWER("i", 4) in US format
        // =IMPOWER("i"; 4) in German format
        let result = codcel_im_power("i".to_string(), "4".to_string(), ".", true).unwrap();
        println!("{result}");
        // The result should be 1
        assert!(result.contains("1"));
    }

    #[test]
    fn test_im_power_1_plus_i_squared() {
        // =IMPOWER("1+i", 2) in US format
        // =IMPOWER("1+i"; 2) in German format
        let result = codcel_im_power("1+i".to_string(), "2".to_string(), ".", true).unwrap();
        println!("{result}");
        // The result should be 2i
        assert!(result.contains("2i"));
    }

    #[test]
    fn test_im_power_zero() {
        // =IMPOWER(0, 2) in US format
        // =IMPOWER(0; 2) in German format
        let result = codcel_im_power("0".to_string(), "2".to_string(), ".", true).unwrap();
        println!("{result}");
        assert_eq!(result, "0");
    }

    #[test]
    fn test_im_power_j_symbol() {
        // =IMPOWER("3+4j", 2) in US format
        // =IMPOWER("3+4j"; 2) in German format
        let result = codcel_im_power("3+4j".to_string(), "2".to_string(), ".", true).unwrap();
        println!("{result}");
        // The result should be -7+24j
        assert!(result.contains("-7") && result.contains("24j"));
    }

    #[test]
    fn test_im_power_invalid_power() {
        // =IMPOWER("3+4i", "invalid") in US format
        // =IMPOWER("3+4i"; "invalid") in German format
        let result = codcel_im_power("3+4i".to_string(), "invalid".to_string(), ".", true);
        assert!(result.is_err());
    }
}
