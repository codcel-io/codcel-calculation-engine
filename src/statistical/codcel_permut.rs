// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `PERMUT` that returns the number of permutations for a given number of objects.
/// - `n`: the total number of objects.
/// - `k`: the number of objects in each permutation.
///
/// Returns n! / (n-k)!, or an error when k > n.
pub fn codcel_permut(n: i32, k: i32) -> Result<i32, Box<dyn Error + Send + Sync>> {
    if k > n {
        return Err("PERMUT: k must be less than or equal to n.".into());
    }

    // Calculate n! / (n-k)!
    let mut result = 1;
    for i in 0..k {
        result *= n - i;
    }

    Ok(result)
}

pub fn codcel_permut_vec(inputs: Vec<i32>) -> Result<i32, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 2 {
        return Err("PERMUT: Must have 2 parameters.".into());
    }

    codcel_permut(inputs[0], inputs[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permut_basic() {
        // =PERMUT(3, 2) in US format
        // =PERMUT(3; 2) in German format
        let result = codcel_permut(3, 2).unwrap();
        assert_eq!(result, 6);
    }

    #[test]
    fn test_permut_zero_k() {
        // =PERMUT(5, 0) in US format
        // =PERMUT(5; 0) in German format
        let result = codcel_permut(5, 0).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_permut_equal_n_k() {
        // =PERMUT(4, 4) in US format
        // =PERMUT(4; 4) in German format
        let result = codcel_permut(4, 4).unwrap();
        assert_eq!(result, 24);
    }

    #[test]
    fn test_permut_large_numbers() {
        // =PERMUT(10, 3) in US format
        // =PERMUT(10; 3) in German format
        let result = codcel_permut(10, 3).unwrap();
        assert_eq!(result, 720);
    }

    #[test]
    fn test_permut_k_greater_than_n() {
        // =PERMUT(3, 4) in US format
        // =PERMUT(3; 4) in German format
        // This should return an error
        let result = codcel_permut(3, 4);
        assert!(result.is_err());
    }

    #[test]
    fn test_permut_vec_valid() {
        // Test the vector version with valid inputs
        let inputs = vec![3, 2];
        let result = codcel_permut_vec(inputs).unwrap();
        assert_eq!(result, 6);
    }

    #[test]
    fn test_permut_vec_invalid() {
        // Test the vector version with invalid inputs (wrong number of parameters)
        let inputs = vec![3];
        let result = codcel_permut_vec(inputs);
        assert!(result.is_err());
    }
}
