// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::engineering::complex::parse_complex;
use std::error::Error;
use std::f64::consts::PI;

/// Excel-compatible `IMARGUMENT` that returns the argument (phase angle) of a complex number in radians.
/// - `complex`: a complex number string in the form `x+yi` or `x+yj`.
///   Returns the angle θ in radians (atan2(y, x)), or an error for invalid formats.
pub fn codcel_im_argument(complex: String) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let complex = complex.replace(" ", "");
    let (real, imag) = parse_complex(&complex)?;

    // Calculate the argument (atan2 handles all quadrants correctly)
    let argument = imag.atan2(real);

    // Normalize the result to be in the range (-π, π]
    let normalized = if argument <= -PI {
        argument + 2.0 * PI
    } else if argument > PI {
        argument - 2.0 * PI
    } else {
        argument
    };

    Ok(normalized)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_im_argument_first_quadrant() {
        // =IMARGUMENT("3+4i") in US format
        // =IMARGUMENT("3+4i") in German format
        let result = codcel_im_argument("3+4i".to_string()).unwrap();
        println!("{result}");
        assert!((result - 0.9272952180016122).abs() < 0.0001);
    }

    #[test]
    fn test_im_argument_second_quadrant() {
        // =IMARGUMENT("-3+4i") in US format
        // =IMARGUMENT("-3+4i") in German format
        let result = codcel_im_argument("-3+4i".to_string()).unwrap();
        println!("{result}");
        assert!((result - 2.214297435588181).abs() < 0.0001);
    }

    #[test]
    fn test_im_argument_third_quadrant() {
        // =IMARGUMENT("-3-4i") in US format
        // =IMARGUMENT("-3-4i") in German format
        let result = codcel_im_argument("-3-4i".to_string()).unwrap();
        println!("{result}");
        assert!((result + 2.214297435588181).abs() < 0.0001);
    }

    #[test]
    fn test_im_argument_fourth_quadrant() {
        // =IMARGUMENT("3-4i") in US format
        // =IMARGUMENT("3-4i") in German format
        let result = codcel_im_argument("3-4i".to_string()).unwrap();
        println!("{result}");
        assert!((result + 0.9272952180016122).abs() < 0.0001);
    }

    #[test]
    fn test_im_argument_real_only() {
        // =IMARGUMENT("5") in US format
        // =IMARGUMENT("5") in German format
        let result = codcel_im_argument("5".to_string()).unwrap();
        println!("{result}");
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_im_argument_imaginary_only_positive() {
        // =IMARGUMENT("i") in US format
        // =IMARGUMENT("i") in German format
        let result = codcel_im_argument("i".to_string()).unwrap();
        println!("{result}");
        assert!((result - PI / 2.0).abs() < 0.0001);
    }

    #[test]
    fn test_im_argument_imaginary_only_negative() {
        // =IMARGUMENT("-i") in US format
        // =IMARGUMENT("-i") in German format
        let result = codcel_im_argument("-i".to_string()).unwrap();
        println!("{result}");
        assert!((result + PI / 2.0).abs() < 0.0001);
    }

    #[test]
    fn test_im_argument_invalid_input() {
        // =IMARGUMENT("not_a_complex_number") in US format
        // =IMARGUMENT("not_a_complex_number") in German format
        let result = codcel_im_argument("not_a_complex_number".to_string());
        assert!(result.is_err());
    }
}
