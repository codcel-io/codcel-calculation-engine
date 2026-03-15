// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::to_bool::ToBool;
use std::error::Error;

/// Excel-compatible logical `AND` operator that converts inputs to booleans and returns their conjunction.
/// - `input1`: first value to evaluate (any type implementing `ToBool`).
/// - `input2`: second value to evaluate (any type implementing `ToBool`).
///
/// Uses Excel-style truthiness: zero/empty is `false`, non-zero is `true`.
///
/// Returns `true` if both values are truthy, or an error if either value cannot be converted to a boolean.
pub fn codcel_logical_and<T: ToBool, S: ToBool>(
    input1: T,
    input2: S,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    Ok(input1.to_bool()? && input2.to_bool()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logical_and_bool_values() {
        // Test all combinations of boolean values
        assert!(codcel_logical_and(true, true).unwrap());
        assert!(!codcel_logical_and(true, false).unwrap());
        assert!(!codcel_logical_and(false, true).unwrap());
        assert!(!codcel_logical_and(false, false).unwrap());
    }

    #[test]
    fn test_logical_and_integers() {
        // Test with integer values (0 is false, non-zero is true)
        assert!(codcel_logical_and(1, 2).unwrap());
        assert!(!codcel_logical_and(0, 1).unwrap());
        assert!(!codcel_logical_and(1, 0).unwrap());
        assert!(!codcel_logical_and(0, 0).unwrap());

        // Test with negative integers (still considered true)
        assert!(codcel_logical_and(-1, 2).unwrap());
        assert!(codcel_logical_and(-5, -10).unwrap());
    }

    #[test]
    fn test_logical_and_floats() {
        // Test with floating point values (0.0 is false, non-zero is true)
        assert!(codcel_logical_and(1.5, 2.5).unwrap());
        assert!(!codcel_logical_and(0.0, 1.5).unwrap());
        assert!(!codcel_logical_and(1.5, 0.0).unwrap());
        assert!(!codcel_logical_and(0.0, 0.0).unwrap());

        // Test with negative floats (still considered true)
        assert!(codcel_logical_and(-1.5, 2.5).unwrap());
        assert!(codcel_logical_and(-1.5, -2.5).unwrap());
    }

    #[test]
    fn test_logical_and_strings() {
        // Test with string values ("true", "false", "1", "1.0" are valid)
        assert!(codcel_logical_and("true", "true").unwrap());
        assert!(!codcel_logical_and("true", "false").unwrap());
        assert!(!codcel_logical_and("false", "true").unwrap());
        assert!(!codcel_logical_and("false", "false").unwrap());

        // Test with "1" and "1.0" (both considered true)
        assert!(codcel_logical_and("1", "true").unwrap());
        assert!(codcel_logical_and("1.0", "true").unwrap());
        assert!(codcel_logical_and("1", "1.0").unwrap());

        // Test case insensitivity
        assert!(codcel_logical_and("TRUE", "true").unwrap());
        assert!(codcel_logical_and("True", "TRUE").unwrap());
    }

    #[test]
    fn test_logical_and_mixed_types() {
        // Test combinations of different types
        assert!(codcel_logical_and(true, 1).unwrap());
        assert!(codcel_logical_and(1, true).unwrap());
        assert!(codcel_logical_and("true", 1).unwrap());
        assert!(codcel_logical_and(1.0, "true").unwrap());
        assert!(codcel_logical_and(1, 1.0).unwrap());

        // Mixed types with false values
        assert!(!codcel_logical_and(false, 1).unwrap());
        assert!(!codcel_logical_and(0, "true").unwrap());
        assert!(!codcel_logical_and("false", 1.0).unwrap());
        assert!(!codcel_logical_and(0.0, true).unwrap());
    }

    #[test]
    fn test_logical_and_with_string_slices() {
        // Test with string slices
        let true_str: &str = "true";
        let false_str: &str = "false";

        assert!(codcel_logical_and(true_str, true_str).unwrap());
        assert!(!codcel_logical_and(true_str, false_str).unwrap());
        assert!(!codcel_logical_and(false_str, true_str).unwrap());
        assert!(!codcel_logical_and(false_str, false_str).unwrap());
    }

    #[test]
    fn test_logical_and_with_owned_strings() {
        // Test with owned String values
        let true_string = String::from("true");
        let false_string = String::from("false");

        assert!(codcel_logical_and(true_string.clone(), true_string.clone()).unwrap());
        assert!(!codcel_logical_and(true_string.clone(), false_string.clone()).unwrap());
        assert!(!codcel_logical_and(false_string.clone(), true_string.clone()).unwrap());
        assert!(!codcel_logical_and(false_string.clone(), false_string.clone()).unwrap());

        // Mix String and &str
        assert!(codcel_logical_and(String::from("true"), "true").unwrap());
        assert!(!codcel_logical_and("false", String::from("true")).unwrap());
    }

    #[test]
    #[should_panic]
    fn test_logical_and_invalid_string() {
        // Test with invalid string that can't be converted to boolean
        // This should cause a panic when unwrapped
        codcel_logical_and("invalid", "true").unwrap();
    }

    #[test]
    fn test_logical_and_error_handling() {
        // Test error handling without unwrapping
        assert!(codcel_logical_and("invalid", "true").is_err());
        assert!(codcel_logical_and("true", "invalid").is_err());
        assert!(codcel_logical_and("invalid1", "invalid2").is_err());
    }
}
