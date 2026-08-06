// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

// Function that uses banker's rounding (round to even) for midpoint values
pub fn codcel_excel_bankers_round(value: f64, decimal_places: i32) -> f64 {
    if !value.is_finite() {
        return value; // Return inf or nan as is
    }

    // Scale factor based on decimal places
    let scale = 10.0_f64.powi(decimal_places);

    // Multiply by scale
    let scaled = value * scale;

    // Get the fractional part
    let fraction = scaled.fract().abs();

    // Check if exactly at midpoint (0.5)
    if (fraction - 0.5).abs() < f64::EPSILON {
        // Round to nearest even number
        let floor = scaled.floor();
        if floor % 2.0 == 0.0 {
            // If floor is even, round down
            floor / scale
        } else {
            // If floor is odd, round up
            (floor + 1.0) / scale
        }
    } else {
        // Regular rounding for non-midpoint values
        (scaled.round()) / scale
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
    fn test_regular_rounding() {
        // Test regular rounding (non-midpoint values)
        assert_approx_eq(codcel_excel_bankers_round(1.234, 2), 1.23);
        assert_approx_eq(codcel_excel_bankers_round(1.236, 2), 1.24);
        assert_approx_eq(codcel_excel_bankers_round(-1.234, 2), -1.23);
        assert_approx_eq(codcel_excel_bankers_round(-1.236, 2), -1.24);

        // Test with different decimal places
        assert_approx_eq(codcel_excel_bankers_round(1.234, 0), 1.0);
        assert_approx_eq(codcel_excel_bankers_round(1.234, 1), 1.2);
        assert_approx_eq(codcel_excel_bankers_round(1.234, 3), 1.234);
        assert_approx_eq(codcel_excel_bankers_round(1.234, 4), 1.234);
    }

    #[test]
    fn test_midpoint_rounding_to_even() {
        // Test midpoint rounding to even (down)
        assert_approx_eq(codcel_excel_bankers_round(1.25, 1), 1.2); // 1.25 -> 1.2 (round down to even)
        assert_approx_eq(codcel_excel_bankers_round(2.25, 1), 2.2); // 2.25 -> 2.2 (round down to even)
        assert_approx_eq(codcel_excel_bankers_round(-1.25, 1), -1.2); // -1.25 -> -1.2 (round down to even)
        assert_approx_eq(codcel_excel_bankers_round(-2.25, 1), -2.2); // -2.25 -> -2.2 (round down to even)

        // Test midpoint rounding to even (up)
        assert_approx_eq(codcel_excel_bankers_round(1.35, 1), 1.4); // 1.35 -> 1.4 (round up to even)
        assert_approx_eq(codcel_excel_bankers_round(2.35, 1), 2.4); // 2.35 -> 2.4 (round up to even)
        assert_approx_eq(codcel_excel_bankers_round(-1.35, 1), -1.4); // -1.35 -> -1.4 (round up to even)
        assert_approx_eq(codcel_excel_bankers_round(-2.35, 1), -2.4); // -2.35 -> -2.4 (round up to even)

        // Test with different decimal places
        assert_approx_eq(codcel_excel_bankers_round(1.5, 0), 2.0); // 1.5 -> 2 (round up to even)
        assert_approx_eq(codcel_excel_bankers_round(2.5, 0), 2.0); // 2.5 -> 2 (round down to even)
        assert_approx_eq(codcel_excel_bankers_round(3.5, 0), 4.0); // 3.5 -> 4 (round up to even)
        assert_approx_eq(codcel_excel_bankers_round(4.5, 0), 4.0); // 4.5 -> 4 (round down to even)

        // Test with negative numbers
        assert_approx_eq(codcel_excel_bankers_round(-1.5, 0), -2.0); // -1.5 -> -2 (round up to even)
        assert_approx_eq(codcel_excel_bankers_round(-2.5, 0), -2.0); // -2.5 -> -2 (round down to even)
        assert_approx_eq(codcel_excel_bankers_round(-3.5, 0), -4.0); // -3.5 -> -4 (round up to even)
        assert_approx_eq(codcel_excel_bankers_round(-4.5, 0), -4.0); // -4.5 -> -4 (round down to even)
    }

    #[test]
    fn test_edge_cases() {
        // Test with infinity
        assert!(codcel_excel_bankers_round(f64::INFINITY, 2).is_infinite());
        assert!(codcel_excel_bankers_round(f64::NEG_INFINITY, 2).is_infinite());

        // Test with NaN
        assert!(codcel_excel_bankers_round(f64::NAN, 2).is_nan());

        // Test with zero
        assert_approx_eq(codcel_excel_bankers_round(0.0, 2), 0.0);
        assert_approx_eq(codcel_excel_bankers_round(-0.0, 2), 0.0);

        // Test with very large and very small numbers
        assert_approx_eq(codcel_excel_bankers_round(1e15, 2), 1e15);
        assert_approx_eq(codcel_excel_bankers_round(1e-15, 2), 0.0);
    }

    #[test]
    fn test_boundary_conditions() {
        // Test values just below and above midpoints
        assert_approx_eq(codcel_excel_bankers_round(1.249999999, 1), 1.2);
        assert_approx_eq(codcel_excel_bankers_round(1.250000001, 1), 1.3);

        // Test with extreme decimal place values
        assert_approx_eq(codcel_excel_bankers_round(1.23456789, 8), 1.23456789);
        assert_approx_eq(codcel_excel_bankers_round(1.23456789, -2), 0.0);
        assert_approx_eq(codcel_excel_bankers_round(123.456789, -2), 100.0);
        assert_approx_eq(codcel_excel_bankers_round(150.0, -2), 200.0); // 1.5 hundreds -> 2 hundreds (round up to even)
        assert_approx_eq(codcel_excel_bankers_round(250.0, -2), 200.0); // 2.5 hundreds -> 2 hundreds (round down to even)
    }

    #[test]
    fn test_precision_issues() {
        // Test cases that might have floating-point precision issues
        assert_approx_eq(codcel_excel_bankers_round(0.1 + 0.2, 1), 0.3);
        assert_approx_eq(codcel_excel_bankers_round(0.3 - 0.1, 1), 0.2);

        // Test with numbers that don't have exact binary representations
        assert_approx_eq(codcel_excel_bankers_round(0.1, 15), 0.1);
        assert_approx_eq(codcel_excel_bankers_round(0.3, 15), 0.3);
    }

    #[test]
    fn test_specific_excel_examples() {
        // Examples that match Excel's ROUND function behavior
        assert_approx_eq(codcel_excel_bankers_round(2.15, 1), 2.2); // In Excel: ROUND(2.15, 1) = 2.2
        assert_approx_eq(codcel_excel_bankers_round(2.149, 1), 2.1); // In Excel: ROUND(2.149, 1) = 2.1
        assert_approx_eq(codcel_excel_bankers_round(2.151, 1), 2.2); // In Excel: ROUND(2.151, 1) = 2.2

        // More complex Excel examples
        assert_approx_eq(codcel_excel_bankers_round(21.5, -1), 20.0); // In Excel: ROUND(21.5, -1) = 20
        assert_approx_eq(codcel_excel_bankers_round(25.5, -1), 30.0); // In Excel: ROUND(25.5, -1) = 30 (banker's rounding)
        assert_approx_eq(codcel_excel_bankers_round(626.3, -3), 1000.0); // In Excel: ROUND(626.3, -3) = 1000
        assert_approx_eq(codcel_excel_bankers_round(1.98, 1), 2.0); // In Excel: ROUND(1.98, 1) = 2.0
        assert_approx_eq(codcel_excel_bankers_round(1.98, 0), 2.0); // In Excel: ROUND(1.98, 0) = 2
        assert_approx_eq(codcel_excel_bankers_round(-1.98, 1), -2.0); // In Excel: ROUND(-1.98, 1) = -2.0
        assert_approx_eq(codcel_excel_bankers_round(-1.98, 0), -2.0); // In Excel: ROUND(-1.98, 0) = -2
    }
}
