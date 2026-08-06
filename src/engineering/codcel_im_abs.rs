// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::engineering::complex::parse_complex;
use std::error::Error;

/// Excel-compatible `IMABS` that returns the absolute value (modulus) of a complex number.
/// - `complex`: a complex number string in the form `x+yi` or `x+yj`.
///   Returns `sqrt(x² + y²)` for complex numbers, or the absolute value for purely real inputs.
///   Returns an error when the input format is invalid.
pub fn codcel_im_abs(complex: String) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let (real, imag) = parse_complex(&complex)?;
    Ok(crate::portable_math::sqrt(real.powi(2) + imag.powi(2)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_im_abs_positive() {
        // =IMABS("3+4i") in US format
        // =IMABS("3+4i") in German format
        let result = codcel_im_abs("3+4i".to_string()).unwrap();
        println!("{result}");
        assert!((result - 5.0).abs() < 0.0001);
    }

    #[test]
    fn test_im_abs_negative() {
        // =IMABS("-3-4i") in US format
        // =IMABS("-3-4i") in German format
        let result = codcel_im_abs("-3-4i".to_string()).unwrap();
        println!("{result}");
        assert!((result - 5.0).abs() < 0.0001);
    }

    #[test]
    fn test_im_abs_real_only() {
        // =IMABS("5") in US format
        // =IMABS("5") in German format
        let result = codcel_im_abs("5".to_string()).unwrap();
        println!("{result}");
        assert!((result - 5.0).abs() < 0.0001);
    }

    #[test]
    fn test_im_abs_imaginary_only() {
        // =IMABS("5i") in US format
        // =IMABS("5i") in German format
        let result = codcel_im_abs("5i".to_string()).unwrap();
        println!("{result}");
        assert!((result - 5.0).abs() < 0.0001);
    }

    #[test]
    fn test_im_abs_j_notation() {
        // =IMABS("3+4j") in US format
        // =IMABS("3+4j") in German format
        let result = codcel_im_abs("3+4j".to_string()).unwrap();
        println!("{result}");
        assert!((result - 5.0).abs() < 0.0001);
    }

    #[test]
    fn test_im_abs_invalid_input() {
        // =IMABS("not_a_complex_number") in US format
        // =IMABS("not_a_complex_number") in German format
        let result = codcel_im_abs("not_a_complex_number".to_string());
        assert!(result.is_err());
    }
}
