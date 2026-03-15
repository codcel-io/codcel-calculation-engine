// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `AVERAGE` that calculates the arithmetic mean of a set of numbers.
/// - `values`: a vector of numeric values to average.
///
/// Returns the arithmetic mean (sum divided by count), or 0.0 if the input is empty.
pub fn codcel_average(values: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Ok(0.0);
    }

    let sum: f64 = values.iter().sum();
    Ok(sum / values.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average_positive_numbers() {
        // =AVERAGE(1,2,3,4,5) in US format
        // =AVERAGE(1;2;3;4;5) in German format
        let result = codcel_average(vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
        assert_eq!(result, 3.0); // (1+2+3+4+5)/5 = 15/5 = 3
    }

    #[test]
    fn test_average_negative_numbers() {
        // =AVERAGE(-1,-2,-3,-4,-5) in US format
        // =AVERAGE(-1;-2;-3;-4;-5) in German format
        let result = codcel_average(vec![-1.0, -2.0, -3.0, -4.0, -5.0]).unwrap();
        assert_eq!(result, -3.0); // (-1-2-3-4-5)/5 = -15/5 = -3
    }

    #[test]
    fn test_average_mixed_numbers() {
        // =AVERAGE(-2,-1,0,1,2) in US format
        // =AVERAGE(-2;-1;0;1;2) in German format
        let result = codcel_average(vec![-2.0, -1.0, 0.0, 1.0, 2.0]).unwrap();
        assert_eq!(result, 0.0); // (-2-1+0+1+2)/5 = 0/5 = 0
    }

    #[test]
    fn test_average_single_value() {
        // =AVERAGE(5) in US format
        // =AVERAGE(5) in German format
        let result = codcel_average(vec![5.0]).unwrap();
        assert_eq!(result, 5.0); // 5/1 = 5
    }

    #[test]
    fn test_average_empty() {
        // =AVERAGE() in US format (returns 0 in Excel)
        // =AVERAGE() in German format (returns 0 in Excel)
        let result = codcel_average(vec![]).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_average_decimals() {
        // =AVERAGE(1.5,2.5,3.5) in US format
        // =AVERAGE(1,5;2,5;3,5) in German format
        let result = codcel_average(vec![1.5, 2.5, 3.5]).unwrap();
        assert_eq!(result, 2.5); // (1.5+2.5+3.5)/3 = 7.5/3 = 2.5
    }

    #[test]
    fn test_average_large_numbers() {
        // =AVERAGE(1000000,2000000,3000000) in US format
        // =AVERAGE(1000000;2000000;3000000) in German format
        let result = codcel_average(vec![1000000.0, 2000000.0, 3000000.0]).unwrap();
        assert_eq!(result, 2000000.0); // (1000000+2000000+3000000)/3 = 6000000/3 = 2000000
    }

    #[test]
    fn test_average_small_decimals() {
        // =AVERAGE(0.0001,0.0002,0.0003) in US format
        // =AVERAGE(0,0001;0,0002;0,0003) in German format
        let result = codcel_average(vec![0.0001, 0.0002, 0.0003]).unwrap();
        assert!((result - 0.0002).abs() < 1e-10); // (0.0001+0.0002+0.0003)/3 = 0.0006/3 = 0.0002
    }
}
