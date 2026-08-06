// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `PERMUTATIONA` that returns the number of permutations with repetition.
/// - `n`: the total number of objects.
/// - `k`: the number of objects in each permutation.
///
/// Returns n^k (permutations with repetition allowed),
/// or an error when n or k is negative.
pub fn codcel_permutation_a(n: i32, k: i32) -> Result<i32, Box<dyn Error + Send + Sync>> {
    if n < 0 || k < 0 {
        return Err("PERMUTATIONA: n and k must be non-negative.".into());
    }

    // n^k
    Ok(n.pow(k as u32))
}

pub fn codcel_permutation_a_vec(inputs: Vec<i32>) -> Result<i32, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 2 {
        return Err("PERMUTATIONA: Must have 2 parameters.".into());
    }

    codcel_permutation_a(inputs[0], inputs[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutation_a_basic() {
        // =PERMUTATIONA(3, 2) in US format
        // =PERMUTATIONA(3; 2) in German format
        let result = codcel_permutation_a(3, 2).unwrap();
        assert_eq!(result, 9);
    }

    #[test]
    fn test_permutation_a_zero_k() {
        // =PERMUTATIONA(5, 0) in US format
        // =PERMUTATIONA(5; 0) in German format
        let result = codcel_permutation_a(5, 0).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_permutation_a_zero_n() {
        // =PERMUTATIONA(0, 3) in US format
        // =PERMUTATIONA(0; 3) in German format
        let result = codcel_permutation_a(0, 3).unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_permutation_a_zero_n_zero_k() {
        // =PERMUTATIONA(0, 0) in US format
        // =PERMUTATIONA(0; 0) in German format
        let result = codcel_permutation_a(0, 0).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_permutation_a_large_numbers() {
        // =PERMUTATIONA(10, 3) in US format
        // =PERMUTATIONA(10; 3) in German format
        let result = codcel_permutation_a(10, 3).unwrap();
        assert_eq!(result, 1000);
    }

    #[test]
    fn test_permutation_a_negative_n() {
        // Negative n should return an error
        let result = codcel_permutation_a(-3, 2);
        assert!(result.is_err());
    }

    #[test]
    fn test_permutation_a_negative_k() {
        // Negative k should return an error
        let result = codcel_permutation_a(3, -2);
        assert!(result.is_err());
    }

    #[test]
    fn test_permutation_a_vec_valid() {
        // Test the vector version with valid inputs
        let inputs = vec![3, 2];
        let result = codcel_permutation_a_vec(inputs).unwrap();
        assert_eq!(result, 9);
    }

    #[test]
    fn test_permutation_a_vec_invalid() {
        // Test the vector version with invalid inputs (wrong number of parameters)
        let inputs = vec![3];
        let result = codcel_permutation_a_vec(inputs);
        assert!(result.is_err());
    }
}
