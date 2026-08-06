// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `MIN` that returns the smallest value in a set of values.
/// - `values`: an array of numeric values.
///
/// Returns the minimum value, or 0.0 if the input is empty.
pub fn codcel_min(values: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Ok(0.0);
    }

    let min_value_option = values.iter().cloned().fold(None, |min, x| match min {
        None => Some(x),
        Some(min) => Some(if x < min { x } else { min }),
    });

    Ok(min_value_option.unwrap_or(0.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_positive_values() {
        // =MIN(1,2,3,4,5) in US format
        // =MIN(1;2;3;4;5) in German format
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_min(values).unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_min_negative_values() {
        // =MIN(-5,-4,-3,-2,-1) in US format
        // =MIN(-5;-4;-3;-2;-1) in German format
        let values = vec![-5.0, -4.0, -3.0, -2.0, -1.0];
        let result = codcel_min(values).unwrap();
        assert_eq!(result, -5.0);
    }

    #[test]
    fn test_min_mixed_values() {
        // =MIN(-3,-1,0,2,4) in US format
        // =MIN(-3;-1;0;2;4) in German format
        let values = vec![-3.0, -1.0, 0.0, 2.0, 4.0];
        let result = codcel_min(values).unwrap();
        assert_eq!(result, -3.0);
    }

    #[test]
    fn test_min_decimal_values() {
        // =MIN(1.1,2.2,3.3,4.4,5.5) in US format
        // =MIN(1,1;2,2;3,3;4,4;5,5) in German format
        let values = vec![1.1, 2.2, 3.3, 4.4, 5.5];
        let result = codcel_min(values).unwrap();
        assert_eq!(result, 1.1);
    }

    #[test]
    fn test_min_single_value() {
        // =MIN(42) in US format
        // =MIN(42) in German format
        let values = vec![42.0];
        let result = codcel_min(values).unwrap();
        assert_eq!(result, 42.0);
    }

    #[test]
    fn test_min_empty_dataset() {
        // =MIN() in US format
        // =MIN() in German format
        // Excel returns 0 for empty MIN function
        let values: Vec<f64> = vec![];
        let result = codcel_min(values).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_min_same_values() {
        // =MIN(5,5,5,5,5) in US format
        // =MIN(5;5;5;5;5) in German format
        let values = vec![5.0, 5.0, 5.0, 5.0, 5.0];
        let result = codcel_min(values).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_min_large_values() {
        // =MIN(1000000,2000000,3000000) in US format
        // =MIN(1000000;2000000;3000000) in German format
        let values = vec![1000000.0, 2000000.0, 3000000.0];
        let result = codcel_min(values).unwrap();
        assert_eq!(result, 1000000.0);
    }

    #[test]
    fn test_min_small_values() {
        // =MIN(0.001,0.002,0.003) in US format
        // =MIN(0,001;0,002;0,003) in German format
        let values = vec![0.001, 0.002, 0.003];
        let result = codcel_min(values).unwrap();
        assert_eq!(result, 0.001);
    }

    #[test]
    fn test_min_unsorted_data() {
        // =MIN(5,3,1,4,2) in US format
        // =MIN(5;3;1;4;2) in German format
        let values = vec![5.0, 3.0, 1.0, 4.0, 2.0];
        let result = codcel_min(values).unwrap();
        assert_eq!(result, 1.0);
    }
}
