// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `XOR` that returns a logical exclusive OR of all arguments.
/// - `values`: a vector of boolean values to evaluate.
///
/// Returns `true` when an odd number of values are `true`; returns `false` for an even
///
/// count of `true` values or when the input is empty.
pub fn codcel_xor(values: Vec<bool>) -> Result<bool, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Ok(false);
    }

    Ok(values.iter().filter(|&&b| b).count() % 2 == 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_empty_vector() {
        // =XOR() in Excel
        let result = codcel_xor(vec![]).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_xor_single_true() {
        // =XOR(TRUE) in Excel
        let result = codcel_xor(vec![true]).unwrap();
        assert!(result);
    }

    #[test]
    fn test_xor_single_false() {
        // =XOR(FALSE) in Excel
        let result = codcel_xor(vec![false]).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_xor_two_true() {
        // =XOR(TRUE, TRUE) in Excel
        let result = codcel_xor(vec![true, true]).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_xor_two_false() {
        // =XOR(FALSE, FALSE) in Excel
        let result = codcel_xor(vec![false, false]).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_xor_true_false() {
        // =XOR(TRUE, FALSE) in Excel
        let result = codcel_xor(vec![true, false]).unwrap();
        assert!(result);
    }

    #[test]
    fn test_xor_false_true() {
        // =XOR(FALSE, TRUE) in Excel
        let result = codcel_xor(vec![false, true]).unwrap();
        assert!(result);
    }

    #[test]
    fn test_xor_three_true() {
        // =XOR(TRUE, TRUE, TRUE) in Excel
        let result = codcel_xor(vec![true, true, true]).unwrap();
        assert!(result);
    }

    #[test]
    fn test_xor_three_false() {
        // =XOR(FALSE, FALSE, FALSE) in Excel
        let result = codcel_xor(vec![false, false, false]).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_xor_mixed_values_odd_true() {
        // =XOR(TRUE, FALSE, TRUE) in Excel
        let result = codcel_xor(vec![true, false, true]).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_xor_mixed_values_even_true() {
        // =XOR(TRUE, FALSE, TRUE, FALSE) in Excel
        let result = codcel_xor(vec![true, false, true, false]).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_xor_mixed_values_odd_true_2() {
        // =XOR(TRUE, FALSE, FALSE, TRUE) in Excel
        let result = codcel_xor(vec![true, false, false, true]).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_xor_mixed_values_single_true() {
        // =XOR(TRUE, FALSE, FALSE) in Excel
        let result = codcel_xor(vec![true, false, false]).unwrap();
        assert!(result);
    }

    #[test]
    fn test_xor_mixed_values_three_true() {
        // =XOR(TRUE, TRUE, TRUE, FALSE) in Excel
        let result = codcel_xor(vec![true, true, true, false]).unwrap();
        assert!(result);
    }

    #[test]
    fn test_xor_many_values_even_true() {
        // =XOR(TRUE, TRUE, FALSE, FALSE, TRUE, TRUE, FALSE, FALSE) in Excel
        let result = codcel_xor(vec![true, true, false, false, true, true, false, false]).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_xor_many_values_odd_true() {
        // =XOR(TRUE, TRUE, FALSE, FALSE, TRUE, FALSE, FALSE) in Excel
        let result = codcel_xor(vec![true, true, false, false, true, false, false]).unwrap();
        assert!(result);
    }

    #[test]
    fn test_xor_large_vector_even_true() {
        // Create a vector with 100 true values
        let values = vec![true; 100];
        let result = codcel_xor(values).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_xor_large_vector_odd_true() {
        // Create a vector with 101 true values
        let values = vec![true; 101];
        let result = codcel_xor(values).unwrap();
        assert!(result);
    }

    #[test]
    fn test_xor_result_wrapping() {
        // Test that the result is properly wrapped in a Result type
        let result = codcel_xor(vec![true, false]);

        // Check that the result is an Ok variant
        assert!(result.is_ok());

        // Check the unwrapped value
        assert!(result.unwrap());
    }

    #[test]
    fn test_xor_with_expressions() {
        // Test XOR with boolean expressions
        let a = true;
        let b = false;

        assert!(codcel_xor(vec![a && b, a || b]).unwrap());
        assert!(codcel_xor(vec![a && a, b || b]).unwrap());
        assert!(!codcel_xor(vec![a && a, a || b, b || b]).unwrap());
    }

    #[test]
    fn test_xor_with_variables() {
        // Test XOR with boolean variables
        let var_true = true;
        let var_false = false;

        assert!(codcel_xor(vec![var_true]).unwrap());
        assert!(!codcel_xor(vec![var_false]).unwrap());
        assert!(codcel_xor(vec![var_true, var_false]).unwrap());
        assert!(!codcel_xor(vec![var_true, var_true]).unwrap());
        assert!(!codcel_xor(vec![var_false, var_false]).unwrap());
    }

    #[test]
    fn test_xor_mathematical_properties() {
        // Test XOR's mathematical properties

        // Identity: A XOR false = A
        assert!(codcel_xor(vec![true, false]).unwrap());
        assert!(!codcel_xor(vec![false, false]).unwrap());

        // Self-inverse: A XOR A = false
        assert!(!codcel_xor(vec![true, true]).unwrap());
        assert!(!codcel_xor(vec![false, false]).unwrap());

        // Associativity: (A XOR B) XOR C = A XOR (B XOR C)
        let a = true;
        let b = false;
        let c = true;

        // (A XOR B) XOR C
        let ab_xor = codcel_xor(vec![a, b]).unwrap();
        let ab_xor_c = codcel_xor(vec![ab_xor, c]).unwrap();

        // A XOR (B XOR C)
        let bc_xor = codcel_xor(vec![b, c]).unwrap();
        let a_xor_bc = codcel_xor(vec![a, bc_xor]).unwrap();

        assert_eq!(ab_xor_c, a_xor_bc);
    }
}
