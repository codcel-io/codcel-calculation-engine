// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `MAX` that returns the largest value in a set of values.
/// - `values`: an array of numeric values.
///
/// Returns the maximum value, or 0.0 if the input is empty.
pub fn codcel_max(values: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Ok(0.0);
    }

    let result = values.into_iter().fold(None, |max, num| match max {
        None => Some(num),
        Some(current_max) => Some(current_max.max(num)),
    });
    Ok(result.unwrap_or(0.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_basic() {
        // =MAX(1, 2, 3, 4, 5) in US format
        // =MAX(1; 2; 3; 4; 5) in German format
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_max(values).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_max_negative_numbers() {
        // =MAX(-10, -5, -3, -1) in US format
        // =MAX(-10; -5; -3; -1) in German format
        let values = vec![-10.0, -5.0, -3.0, -1.0];
        let result = codcel_max(values).unwrap();
        assert_eq!(result, -1.0);
    }

    #[test]
    fn test_max_mixed_numbers() {
        // =MAX(-10, 0, 10, 5) in US format
        // =MAX(-10; 0; 10; 5) in German format
        let values = vec![-10.0, 0.0, 10.0, 5.0];
        let result = codcel_max(values).unwrap();
        assert_eq!(result, 10.0);
    }

    #[test]
    fn test_max_decimals() {
        // =MAX(1.5, 2.7, 3.1, 4.9) in US format
        // =MAX(1,5; 2,7; 3,1; 4,9) in German format
        let values = vec![1.5, 2.7, 3.1, 4.9];
        let result = codcel_max(values).unwrap();
        assert_eq!(result, 4.9);
    }

    #[test]
    fn test_max_single_value() {
        // =MAX(42) in US format
        // =MAX(42) in German format
        let values = vec![42.0];
        let result = codcel_max(values).unwrap();
        assert_eq!(result, 42.0);
    }

    #[test]
    fn test_max_empty() {
        // =MAX() in US format
        // =MAX() in German format
        let values: Vec<f64> = vec![];
        let result = codcel_max(values).unwrap();
        assert_eq!(result, 0.0);
    }
}
