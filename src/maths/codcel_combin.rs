// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `COMBIN` that returns the number of combinations for a given number of items.
/// - `number`: the total number of items (n).
/// - `number_chosen`: the number of items to choose (k).
///
/// Returns n! / (k! × (n-k)!) or an error when k > n.
pub fn codcel_combin(number: i32, number_chosen: i32) -> Result<i32, Box<dyn Error + Send + Sync>> {
    let n = number as u32;
    let k = number_chosen as u32;

    if k > n {
        return Err("COMBIN: Number chosen (k) cannot be greater than number (n)".into());
    }

    // Use iterative multiplication/division with f64 to avoid overflow.
    // C(n,k) = product(i=0..k-1) of (n-i)/(i+1)
    // Optimize by using the smaller of k and n-k.
    let k = k.min(n - k);
    let mut result = 1.0_f64;
    for i in 0..k {
        result = result * (n - i) as f64 / (i + 1) as f64;
    }

    Ok(result.round() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combin_basic() {
        // =COMBIN(5, 2) in US format
        // =COMBIN(5; 2) in German format
        let result = codcel_combin(5, 2).unwrap();
        assert_eq!(result, 10);
    }

    #[test]
    fn test_combin_all_items() {
        // =COMBIN(5, 5) in US format
        // =COMBIN(5; 5) in German format
        let result = codcel_combin(5, 5).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_combin_no_items() {
        // =COMBIN(5, 0) in US format
        // =COMBIN(5; 0) in German format
        let result = codcel_combin(5, 0).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_combin_one_item() {
        // =COMBIN(5, 1) in US format
        // =COMBIN(5; 1) in German format
        let result = codcel_combin(5, 1).unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn test_combin_larger_numbers() {
        // =COMBIN(10, 3) in US format
        // =COMBIN(10; 3) in German format
        let result = codcel_combin(10, 3).unwrap();
        assert_eq!(result, 120);
    }

    #[test]
    fn test_combin_symmetry() {
        // =COMBIN(8, 3) in US format
        // =COMBIN(8; 3) in German format
        let result1 = codcel_combin(8, 3).unwrap();

        // =COMBIN(8, 5) in US format
        // =COMBIN(8; 5) in German format
        let result2 = codcel_combin(8, 5).unwrap();

        // These should be equal due to the symmetry property of combinations
        assert_eq!(result1, result2);
    }

    #[test]
    fn test_combin_invalid_input() {
        // =COMBIN(3, 5) in US format - should return an error
        // =COMBIN(3; 5) in German format - should return an error
        let result = codcel_combin(3, 5);
        assert!(result.is_err());
    }
}
