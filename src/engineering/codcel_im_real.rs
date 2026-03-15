// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::engineering::complex::parse_complex;
use std::error::Error;

/// Excel-compatible `IMREAL` that returns the real coefficient of a complex number.
/// - `complex`: a complex number string in the form `x+yi` or `x+yj`.
///   Returns the real coefficient (e.g., `3` for `"3+4i"`), or an error for invalid formats.
pub fn codcel_im_real(complex: String) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Remove all whitespace from the input
    let complex = complex.replace(" ", "");

    // For a purely real number
    if !complex.contains('i') && !complex.contains('j') {
        return complex
            .parse::<f64>()
            .map_err(|_| "IMREAL: Invalid real number format".into());
    }

    // Parse out the real part using parse_complex
    let (real, _) = parse_complex(&complex)?;

    Ok(real)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_im_real_complex_number() {
        // =IMREAL("3+4i") in US format
        // =IMREAL("3+4i") in German format
        let result = codcel_im_real("3+4i".to_string()).unwrap();
        println!("{result}");
        assert!((result - 3.0).abs() < 0.0001);
    }

    #[test]
    fn test_im_real_purely_real() {
        // =IMREAL("5") in US format
        // =IMREAL("5") in German format
        let result = codcel_im_real("5".to_string()).unwrap();
        println!("{result}");
        assert!((result - 5.0).abs() < 0.0001);
    }

    #[test]
    fn test_im_real_purely_imaginary() {
        // =IMREAL("4i") in US format
        // =IMREAL("4i") in German format
        let result = codcel_im_real("4i".to_string()).unwrap();
        println!("{result}");
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_im_real_negative_real() {
        // =IMREAL("-3+4i") in US format
        // =IMREAL("-3+4i") in German format
        let result = codcel_im_real("-3+4i".to_string()).unwrap();
        println!("{result}");
        assert!((result + 3.0).abs() < 0.0001);
    }

    #[test]
    fn test_im_real_j_symbol() {
        // =IMREAL("3+4j") in US format
        // =IMREAL("3+4j") in German format
        let result = codcel_im_real("3+4j".to_string()).unwrap();
        println!("{result}");
        assert!((result - 3.0).abs() < 0.0001);
    }

    #[test]
    fn test_im_real_invalid_input() {
        // =IMREAL("not_a_complex_number") in US format
        // =IMREAL("not_a_complex_number") in German format
        let result = codcel_im_real("not_a_complex_number".to_string());
        assert!(result.is_err());
    }
}
