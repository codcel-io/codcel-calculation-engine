// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::engineering::complex::{format_complex_with_precision, number_to_string, parse_complex};
use std::error::Error;

/// Excel-compatible `IMSQRT` that returns the square root of a complex number.
/// - `complex`: a complex number string in the form `x+yi` or `x+yj`.
///   Returns the principal square root as a complex number string, or an error for invalid formats.
pub fn codcel_im_sqrt(
    complex: String,
    decimal_separator: &str,
    use_excel_rounding: bool,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    // Remove all whitespace from the input
    let complex = complex.replace(" ", "");

    // Determine which imaginary symbol is used (default to 'i' if no imaginary part)
    let im_symbol = if complex.contains('j') { 'j' } else { 'i' };

    // For a non-negative purely real number, compute directly
    if !complex.contains('i') && !complex.contains('j') {
        if let Ok(real) = complex.parse::<f64>() {
            if real >= 0.0 {
                return Ok(number_to_string(
                    crate::portable_math::sqrt(real),
                    decimal_separator,
                    use_excel_rounding,
                ));
            }
            // Negative reals fall through to the complex formula below
        } else {
            return Err("IMSQRT: Invalid real number format".into());
        }
    }

    // Parse out the real and imaginary parts
    let (real, imag) = parse_complex(&complex)?;

    // Calculate the magnitude (r) and argument (θ) of the complex number
    let magnitude = crate::portable_math::sqrt(real * real + imag * imag);
    let argument = crate::portable_math::atan2(imag, real);

    // Calculate square root components:
    // r' = √r
    // θ' = θ/2
    let sqrt_magnitude = crate::portable_math::sqrt(magnitude);
    let half_argument = argument / 2.0;

    // Convert back to rectangular form:
    // x = r'*cos(θ')
    // y = r'*sin(θ')
    let real_part = sqrt_magnitude * crate::portable_math::cos(half_argument);
    let imag_part = sqrt_magnitude * crate::portable_math::sin(half_argument);

    // Format the result. Excel preserves floating-point noise in IMSQRT results
    // (e.g. sqrt(-1) = 6.12e-17+i), so use epsilon=0.
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
    fn test_im_sqrt_windows_difference() {
        // =IMSQRT("0+2i") in US format
        // =IMSQRT("0+2i") in German format
        let complex = "0+2i".to_string();
        println!("Input: {complex}");

        // Parse the complex number
        let (real, imag) = crate::engineering::complex::parse_complex(&complex).unwrap();
        println!("Parsed: real={real}, imag={imag}");

        // Calculate magnitude and argument
        let magnitude = (real * real + imag * imag).sqrt();
        let argument = imag.atan2(real);
        println!("Magnitude: {magnitude}, Argument: {argument}");

        // Calculate square root components
        let sqrt_magnitude = magnitude.sqrt();
        let half_argument = argument / 2.0;
        println!("Sqrt Magnitude: {sqrt_magnitude}, Half Argument: {half_argument}");

        // Convert back to rectangular form
        let real_part = sqrt_magnitude * half_argument.cos();
        let imag_part = sqrt_magnitude * half_argument.sin();
        println!("Result components: real_part={real_part}, imag_part={imag_part}");

        // Check if the imaginary part is very close to 1.0
        println!(
            "Is imag_part close to 1.0? {}",
            (imag_part - 1.0).abs() < 1e-10
        );

        // Format the complex number directly to see the result
        let formatted =
            crate::engineering::complex::format_complex(real_part, imag_part, 'i', ".", true)
                .unwrap();
        println!("Directly formatted: {formatted}");

        let result = codcel_im_sqrt(complex, ".", true).unwrap();
        println!("Final result: {result}");
        assert!(result.contains("1+i"));
    }

    #[test]
    fn test_im_sqrt_complex_number() {
        // =IMSQRT("3+4i") in US format
        // =IMSQRT("3+4i") in German format
        let result = codcel_im_sqrt("3+4i".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("2+i"));
    }

    #[test]
    fn test_im_sqrt_positive_real() {
        // =IMSQRT("9") in US format
        // =IMSQRT("9") in German format
        let result = codcel_im_sqrt("9".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!((result.parse::<f64>().unwrap() - 3.0).abs() < 0.0001);
    }

    #[test]
    fn test_im_sqrt_negative_real() {
        // =IMSQRT("-9") in US format
        // =IMSQRT("-9") in German format
        let result = codcel_im_sqrt("-9".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("3i"));
    }

    #[test]
    fn test_im_sqrt_purely_imaginary() {
        // =IMSQRT("4i") in US format
        // =IMSQRT("4i") in German format
        let result = codcel_im_sqrt("4i".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("1.4142135623731") && result.contains("i"));
    }

    #[test]
    fn test_im_sqrt_j_symbol() {
        // =IMSQRT("3+4j") in US format
        // =IMSQRT("3+4j") in German format
        let result = codcel_im_sqrt("3+4j".to_string(), ".", true).unwrap();
        println!("{result}");
        assert!(result.contains("2+j"));
    }

    #[test]
    fn test_im_sqrt_invalid_input() {
        // =IMSQRT("not_a_complex_number") in US format
        // =IMSQRT("not_a_complex_number") in German format
        let result = codcel_im_sqrt("not_a_complex_number".to_string(), ".", true);
        assert!(result.is_err());
    }
}
