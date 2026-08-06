// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::to_bool::ToBool;
use std::error::Error;

/// Excel-compatible logical `OR` operator that converts inputs to booleans and returns their disjunction.
/// - `input1`: first value to evaluate (any type implementing `ToBool`).
/// - `input2`: second value to evaluate (any type implementing `ToBool`).
///
/// Uses Excel-style truthiness and short-circuits when the first argument is truthy.
///
/// Returns `true` if either value is truthy, or an error if the evaluated operand cannot be converted to a boolean.
pub fn codcel_logical_or<T: ToBool, S: ToBool>(
    input1: T,
    input2: S,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    Ok(input1.to_bool()? || input2.to_bool()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logical_or_bool_values() {
        // Test all combinations of boolean values
        assert!(codcel_logical_or(true, true).unwrap());
        assert!(codcel_logical_or(true, false).unwrap());
        assert!(codcel_logical_or(false, true).unwrap());
        assert!(!codcel_logical_or(false, false).unwrap());
    }

    #[test]
    fn test_logical_or_integers() {
        // Test with integer values (0 is false, non-zero is true)
        assert!(codcel_logical_or(1, 2).unwrap());
        assert!(codcel_logical_or(0, 1).unwrap());
        assert!(codcel_logical_or(1, 0).unwrap());
        assert!(!codcel_logical_or(0, 0).unwrap());

        // Test with negative integers (still considered true)
        assert!(codcel_logical_or(-1, 2).unwrap());
        assert!(codcel_logical_or(-5, -10).unwrap());
        assert!(codcel_logical_or(0, -10).unwrap());
        assert!(codcel_logical_or(-10, 0).unwrap());
    }

    #[test]
    fn test_logical_or_floats() {
        // Test with floating point values (0.0 is false, non-zero is true)
        assert!(codcel_logical_or(1.5, 2.5).unwrap());
        assert!(codcel_logical_or(0.0, 1.5).unwrap());
        assert!(codcel_logical_or(1.5, 0.0).unwrap());
        assert!(!codcel_logical_or(0.0, 0.0).unwrap());

        // Test with negative floats (still considered true)
        assert!(codcel_logical_or(-1.5, 2.5).unwrap());
        assert!(codcel_logical_or(-1.5, -2.5).unwrap());
        assert!(codcel_logical_or(0.0, -2.5).unwrap());
        assert!(codcel_logical_or(-1.5, 0.0).unwrap());
    }

    #[test]
    fn test_logical_or_strings() {
        // Test with string values ("true", "false", "1", "1.0" are valid)
        assert!(codcel_logical_or("true", "true").unwrap());
        assert!(codcel_logical_or("true", "false").unwrap());
        assert!(codcel_logical_or("false", "true").unwrap());
        assert!(!codcel_logical_or("false", "false").unwrap());

        // Test with "1" and "1.0" (both considered true)
        assert!(codcel_logical_or("1", "true").unwrap());
        assert!(codcel_logical_or("1.0", "true").unwrap());
        assert!(codcel_logical_or("1", "1.0").unwrap());
        assert!(codcel_logical_or("false", "1").unwrap());
        assert!(codcel_logical_or("1.0", "false").unwrap());

        // Test case insensitivity
        assert!(codcel_logical_or("TRUE", "true").unwrap());
        assert!(codcel_logical_or("True", "TRUE").unwrap());
        assert!(codcel_logical_or("FALSE", "TRUE").unwrap());
        assert!(!codcel_logical_or("FALSE", "false").unwrap());
    }

    #[test]
    fn test_logical_or_mixed_types() {
        // Test combinations of different types
        assert!(codcel_logical_or(true, 1).unwrap());
        assert!(codcel_logical_or(1, true).unwrap());
        assert!(codcel_logical_or("true", 1).unwrap());
        assert!(codcel_logical_or(1.0, "true").unwrap());
        assert!(codcel_logical_or(1, 1.0).unwrap());

        // Mixed types with one false value (should still be true)
        assert!(codcel_logical_or(false, 1).unwrap());
        assert!(codcel_logical_or(0, "true").unwrap());
        assert!(codcel_logical_or("false", 1.0).unwrap());
        assert!(codcel_logical_or(0.0, true).unwrap());

        // Mixed types with both false values
        assert!(!codcel_logical_or(false, 0).unwrap());
        assert!(!codcel_logical_or(0.0, "false").unwrap());
        assert!(!codcel_logical_or("false", false).unwrap());
    }

    #[test]
    fn test_logical_or_with_string_slices() {
        // Test with string slices
        let true_str: &str = "true";
        let false_str: &str = "false";

        assert!(codcel_logical_or(true_str, true_str).unwrap());
        assert!(codcel_logical_or(true_str, false_str).unwrap());
        assert!(codcel_logical_or(false_str, true_str).unwrap());
        assert!(!codcel_logical_or(false_str, false_str).unwrap());
    }

    #[test]
    fn test_logical_or_with_owned_strings() {
        // Test with owned String values
        let true_string = String::from("true");
        let false_string = String::from("false");

        assert!(codcel_logical_or(true_string.clone(), true_string.clone()).unwrap());
        assert!(codcel_logical_or(true_string.clone(), false_string.clone()).unwrap());
        assert!(codcel_logical_or(false_string.clone(), true_string.clone()).unwrap());
        assert!(!codcel_logical_or(false_string.clone(), false_string.clone()).unwrap());

        // Mix String and &str
        assert!(codcel_logical_or(String::from("true"), "true").unwrap());
        assert!(codcel_logical_or("false", String::from("true")).unwrap());
        assert!(!codcel_logical_or("false", String::from("false")).unwrap());
    }

    #[test]
    #[should_panic]
    fn test_logical_or_invalid_string() {
        // Test with invalid string that can't be converted to boolean
        // This should cause a panic when unwrapped
        codcel_logical_or("invalid", "true").unwrap();
    }

    #[test]
    fn test_logical_or_error_handling() {
        // Test error handling without unwrapping
        // When first operand is invalid, it should error
        assert!(codcel_logical_or("invalid", "true").is_err());
        assert!(codcel_logical_or("invalid1", "invalid2").is_err());

        // When first operand is false and second is invalid, it should error
        assert!(codcel_logical_or("false", "invalid").is_err());
        assert!(codcel_logical_or(false, "invalid").is_err());
        assert!(codcel_logical_or(0, "invalid").is_err());
        assert!(codcel_logical_or(0.0, "invalid").is_err());
    }

    #[test]
    fn test_logical_or_short_circuit() {
        // In Rust, logical OR should short-circuit when the first operand is true
        // This test verifies that short-circuiting works correctly

        // Both valid, both true
        assert!(codcel_logical_or("true", "true").unwrap());

        // First valid and true, second invalid - should NOT error due to short-circuiting
        assert!(codcel_logical_or("true", "invalid").unwrap());
        assert!(codcel_logical_or(true, "invalid").unwrap());
        assert!(codcel_logical_or(1, "invalid").unwrap());
        assert!(codcel_logical_or(1.0, "invalid").unwrap());

        // First invalid, second valid - should error
        assert!(codcel_logical_or("invalid", "true").is_err());
    }
}
