// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

pub fn codcel_excel_standard_rounding(value: f64) -> f64 {
    if value == 0.0 || !value.is_finite() {
        return value; // Return 0, inf, or nan as is
    }

    // Get the exponent (power of 10) for the value
    let abs_value = value.abs();
    let exponent = crate::portable_math::log10(abs_value).floor() as i32;

    // Calculate the scale factor to normalize the number
    let scale_to_15_digits = 10.0_f64.powi(14 - exponent);

    // Scale the number, round it, and scale back
    let scaled = (abs_value * scale_to_15_digits).round() / scale_to_15_digits;

    // Preserve the original sign
    if value < 0.0 {
        -scaled
    } else {
        scaled
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64;

    // Helper function to assert floating point equality with a small epsilon
    fn assert_approx_eq(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-10, "Expected {}, got {}", b, a);
    }

    #[test]
    fn test_basic_rounding() {
        // Test basic rounding of positive numbers
        assert_approx_eq(codcel_excel_standard_rounding(1.4), 1.4);
        assert_approx_eq(codcel_excel_standard_rounding(1.5), 1.5);
        assert_approx_eq(codcel_excel_standard_rounding(1.6), 1.6);

        // Test basic rounding of negative numbers
        assert_approx_eq(codcel_excel_standard_rounding(-1.4), -1.4);
        assert_approx_eq(codcel_excel_standard_rounding(-1.5), -1.5);
        assert_approx_eq(codcel_excel_standard_rounding(-1.6), -1.6);
    }

    #[test]
    fn test_edge_cases() {
        // Test with zero
        assert_approx_eq(codcel_excel_standard_rounding(0.0), 0.0);
        assert_approx_eq(codcel_excel_standard_rounding(-0.0), 0.0);

        // Test with infinity
        assert!(codcel_excel_standard_rounding(f64::INFINITY).is_infinite());
        assert!(codcel_excel_standard_rounding(f64::NEG_INFINITY).is_infinite());

        // Test with NaN
        assert!(codcel_excel_standard_rounding(f64::NAN).is_nan());
    }

    #[test]
    fn test_different_exponents() {
        // Test with very large numbers
        assert_approx_eq(codcel_excel_standard_rounding(1e15), 1e15);
        assert_approx_eq(codcel_excel_standard_rounding(1.23456789e10), 1.23456789e10);

        // Test with very small numbers
        assert_approx_eq(codcel_excel_standard_rounding(1e-10), 1e-10);
        assert_approx_eq(
            codcel_excel_standard_rounding(1.23456789e-10),
            1.23456789e-10,
        );

        // Test with numbers having different exponents
        assert_approx_eq(codcel_excel_standard_rounding(123.456), 123.456);
        assert_approx_eq(codcel_excel_standard_rounding(1.23456), 1.23456);
        assert_approx_eq(codcel_excel_standard_rounding(0.0123456), 0.0123456);
    }

    #[test]
    fn test_rounding_boundaries() {
        // Test values that are close to rounding boundaries
        // The function should preserve 15 significant digits

        // Numbers with 15 significant digits should be preserved
        assert_approx_eq(
            codcel_excel_standard_rounding(1.234567890123456),
            1.23456789012346,
        );
        assert_approx_eq(
            codcel_excel_standard_rounding(1234567890.123456),
            1234567890.12346,
        );

        // Numbers with more than 15 significant digits should be rounded
        assert_approx_eq(
            codcel_excel_standard_rounding(1.2345678901234567),
            1.23456789012346,
        );
        assert_approx_eq(
            codcel_excel_standard_rounding(1.2345678901234563),
            1.23456789012346,
        );
        assert_approx_eq(
            codcel_excel_standard_rounding(1.2345678901234565),
            1.23456789012346,
        );
        assert_approx_eq(
            codcel_excel_standard_rounding(1.2345678901234566),
            1.23456789012346,
        );
    }

    #[test]
    fn test_precision_handling() {
        // Test the precision handling with exactly 15 significant digits

        // Test with numbers that have exactly 15 significant digits
        assert_approx_eq(
            codcel_excel_standard_rounding(123456789012345.0),
            123456789012345.0,
        );
        assert_approx_eq(
            codcel_excel_standard_rounding(0.000000000000123456789012345),
            0.000000000000123456789012345,
        );

        // Test with numbers that have 16 significant digits (should round the 16th digit)
        assert_approx_eq(
            codcel_excel_standard_rounding(1234567890123456.0),
            1234567890123460.0,
        );
        assert_approx_eq(
            codcel_excel_standard_rounding(0.0000000000001234567890123456),
            0.000000000000123456789012346,
        );
    }

    #[test]
    fn test_rounding_behavior() {
        // Test specific rounding behavior

        // Test rounding up
        assert_approx_eq(
            codcel_excel_standard_rounding(1.2345678901234565),
            1.23456789012346,
        );
        assert_approx_eq(
            codcel_excel_standard_rounding(123456789012345.5),
            123456789012346.0,
        );

        // Test rounding down
        assert_approx_eq(
            codcel_excel_standard_rounding(1.2345678901234564),
            1.23456789012346,
        );
        assert_approx_eq(
            codcel_excel_standard_rounding(123456789012345.4),
            123456789012345.0,
        );

        // Test with negative numbers
        assert_approx_eq(
            codcel_excel_standard_rounding(-1.2345678901234565),
            -1.23456789012346,
        );
        assert_approx_eq(
            codcel_excel_standard_rounding(-123456789012345.5),
            -123456789012346.0,
        );
        assert_approx_eq(
            codcel_excel_standard_rounding(-1.2345678901234564),
            -1.23456789012346,
        );
        assert_approx_eq(
            codcel_excel_standard_rounding(-123456789012345.4),
            -123456789012345.0,
        );
    }

    #[test]
    fn test_specific_examples() {
        // Test with specific examples that demonstrate Excel's standard rounding behavior

        // Examples with different number of significant digits
        assert_approx_eq(codcel_excel_standard_rounding(0.5), 0.5);
        assert_approx_eq(codcel_excel_standard_rounding(1.5), 1.5);
        assert_approx_eq(codcel_excel_standard_rounding(2.5), 2.5);

        // Examples with negative numbers
        assert_approx_eq(codcel_excel_standard_rounding(-0.5), -0.5);
        assert_approx_eq(codcel_excel_standard_rounding(-1.5), -1.5);
        assert_approx_eq(codcel_excel_standard_rounding(-2.5), -2.5);

        // Examples with numbers that might have floating-point precision issues
        assert_approx_eq(codcel_excel_standard_rounding(0.1 + 0.2), 0.3);
        assert_approx_eq(codcel_excel_standard_rounding(0.3 - 0.1), 0.2);
    }

    #[test]
    fn test_excel_compatibility() {
        // Test cases that specifically match Excel's behavior

        // Excel rounds to 15 significant digits
        assert_approx_eq(
            codcel_excel_standard_rounding(0.999999999999999),
            0.999999999999999,
        );
        assert_approx_eq(codcel_excel_standard_rounding(0.9999999999999999), 1.0); // 16th digit gets rounded

        // Examples from Excel documentation or known Excel behavior
        assert_approx_eq(
            codcel_excel_standard_rounding(12345678901234.56),
            12345678901234.6,
        );
        assert_approx_eq(
            codcel_excel_standard_rounding(0.12345678901234567),
            0.123456789012346,
        );
        assert_approx_eq(codcel_excel_standard_rounding(9.9999999999999999), 10.0);
    }
}
