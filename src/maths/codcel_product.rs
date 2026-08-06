// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `PRODUCT` that multiplies all the numbers given as arguments.
/// - `values`: a list of numbers to multiply.
///
/// Returns the product of all values (1 if empty).
pub fn codcel_product(values: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    Ok(values.iter().product())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_positive_numbers() {
        // =PRODUCT(2,3,4) in US format
        // =PRODUCT(2;3;4) in German format
        let result = codcel_product(vec![2.0, 3.0, 4.0]).unwrap();
        assert_eq!(result, 24.0); // 2 * 3 * 4 = 24
    }

    #[test]
    fn test_product_negative_numbers() {
        // =PRODUCT(-2,-3,-4) in US format
        // =PRODUCT(-2;-3;-4) in German format
        let result = codcel_product(vec![-2.0, -3.0, -4.0]).unwrap();
        assert_eq!(result, -24.0); // -2 * -3 * -4 = -24
    }

    #[test]
    fn test_product_mixed_numbers() {
        // =PRODUCT(2,-3,4) in US format
        // =PRODUCT(2;-3;4) in German format
        let result = codcel_product(vec![2.0, -3.0, 4.0]).unwrap();
        assert_eq!(result, -24.0); // 2 * -3 * 4 = -24
    }

    #[test]
    fn test_product_with_zero() {
        // =PRODUCT(2,0,4) in US format
        // =PRODUCT(2;0;4) in German format
        let result = codcel_product(vec![2.0, 0.0, 4.0]).unwrap();
        assert_eq!(result, 0.0); // 2 * 0 * 4 = 0
    }

    #[test]
    fn test_product_single_number() {
        // =PRODUCT(5) in US format
        // =PRODUCT(5) in German format
        let result = codcel_product(vec![5.0]).unwrap();
        assert_eq!(result, 5.0); // 5 = 5
    }

    #[test]
    fn test_product_empty() {
        // =PRODUCT() in US format
        // =PRODUCT() in German format
        let result = codcel_product(vec![]).unwrap();
        assert_eq!(result, 1.0); // Empty product is 1 (multiplicative identity)
    }

    #[test]
    fn test_product_decimals() {
        // =PRODUCT(1.5,2.5) in US format
        // =PRODUCT(1,5;2,5) in German format
        let result = codcel_product(vec![1.5, 2.5]).unwrap();
        assert_eq!(result, 3.75); // 1.5 * 2.5 = 3.75
    }

    #[test]
    fn test_product_large_numbers() {
        // =PRODUCT(1000,2000) in US format
        // =PRODUCT(1000;2000) in German format
        let result = codcel_product(vec![1000.0, 2000.0]).unwrap();
        assert_eq!(result, 2000000.0); // 1000 * 2000 = 2,000,000
    }

    #[test]
    fn test_product_small_decimals() {
        // =PRODUCT(0.1,0.2,0.3) in US format
        // =PRODUCT(0,1;0,2;0,3) in German format
        let result = codcel_product(vec![0.1, 0.2, 0.3]).unwrap();
        assert!((result - 0.006).abs() < 1e-10); // 0.1 * 0.2 * 0.3 = 0.006
    }
}
