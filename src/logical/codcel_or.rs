// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `OR` that evaluates whether any argument is TRUE.
/// - `values`: a vector of boolean values to evaluate.
///
/// Returns `true` when any element is `true`; returns `false` when all are `false`
///
/// or when the input is empty (matching Excel's `OR()` behavior).
pub fn codcel_or(values: Vec<bool>) -> Result<bool, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Ok(false); // OR of an empty set is false
    }

    Ok(values.iter().any(|&b| b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_or_basic_functionality() {
        // Test with all true values
        assert!(codcel_or(vec![true, true, true]).unwrap());

        // Test with all false values
        assert!(!codcel_or(vec![false, false, false]).unwrap());

        // Test with mixed values
        assert!(codcel_or(vec![false, true, false]).unwrap());
        assert!(codcel_or(vec![true, false, false]).unwrap());
        assert!(codcel_or(vec![false, false, true]).unwrap());

        // Test with single value
        assert!(codcel_or(vec![true]).unwrap());
        assert!(!codcel_or(vec![false]).unwrap());
    }

    #[test]
    fn test_or_empty_vector() {
        // Test with empty vector (should return false)
        assert!(!codcel_or(vec![]).unwrap());
    }

    #[test]
    fn test_or_result_wrapping() {
        // Test that the result is properly wrapped in a Result type
        let result_all_true = codcel_or(vec![true, true]);
        let result_all_false = codcel_or(vec![false, false]);
        let result_mixed = codcel_or(vec![false, true]);
        let result_empty = codcel_or(vec![]);

        // Check that the results are Ok variants
        assert!(result_all_true.is_ok());
        assert!(result_all_false.is_ok());
        assert!(result_mixed.is_ok());
        assert!(result_empty.is_ok());

        // Check the unwrapped values
        assert!(result_all_true.unwrap());
        assert!(!result_all_false.unwrap());
        assert!(result_mixed.unwrap());
        assert!(!result_empty.unwrap());
    }

    #[test]
    fn test_or_with_expressions() {
        // Test OR with boolean expressions
        assert!(codcel_or(vec![true && true, false]).unwrap());
        assert!(codcel_or(vec![false, true || false]).unwrap());
        assert!(!codcel_or(vec![false && true, false || false]).unwrap());
        assert!(codcel_or(vec![true && false, true || false]).unwrap());
    }

    #[test]
    fn test_or_with_variables() {
        // Test OR with boolean variables
        let var_true = true;
        let var_false = false;

        assert!(codcel_or(vec![var_true, var_false]).unwrap());
        assert!(codcel_or(vec![var_false, var_true]).unwrap());
        assert!(codcel_or(vec![var_true, var_true]).unwrap());
        assert!(!codcel_or(vec![var_false, var_false]).unwrap());
    }

    #[test]
    fn test_or_with_complex_expressions() {
        // Test OR with more complex boolean expressions
        let a = true;
        let b = false;
        let c = true;

        // OR([a AND b], [b OR c]) = OR([true AND false], [false OR true]) = OR(false, true) = true
        let expr_result = codcel_or(vec![a && b, b || c]).unwrap();
        assert!(expr_result);

        // OR([a OR b], [b AND c]) = OR([true OR false], [false AND true]) = OR(true, false) = true
        let expr_result2 = codcel_or(vec![a || b, b && c]).unwrap();
        assert!(expr_result2);

        // OR([a AND b], [b AND c]) = OR([true AND false], [false AND true]) = OR(false, false) = false
        let expr_result3 = codcel_or(vec![a && b, b && c]).unwrap();
        assert!(!expr_result3);
    }

    #[test]
    fn test_or_with_nested_calls() {
        // Test OR with nested OR calls
        let result1 = codcel_or(vec![false, false]).unwrap();
        let result2 = codcel_or(vec![true, false]).unwrap();

        // OR(OR(false, false), OR(true, false)) = OR(false, true) = true
        assert!(codcel_or(vec![result1, result2]).unwrap());

        // OR(OR(false, false), false) = OR(false, false) = false
        assert!(!codcel_or(vec![result1, false]).unwrap());
    }

    #[test]
    fn test_or_with_many_values() {
        // Test OR with many values
        let many_false = vec![false; 100];
        assert!(!codcel_or(many_false).unwrap());

        // Create a vector with 99 false values and 1 true value
        let mut mostly_false = vec![false; 99];
        mostly_false.push(true);
        assert!(codcel_or(mostly_false).unwrap());

        // Test with alternating true/false values
        let mut alternating = Vec::new();
        for i in 0..100 {
            alternating.push(i % 2 == 0);
        }
        assert!(codcel_or(alternating).unwrap());
    }
}
