// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `AVEDEV` that calculates the average of absolute deviations from the mean.
/// - `values`: a vector of numeric values.
///
/// Returns the average of the absolute deviations of data points from their mean,
/// or 0.0 if the input is empty.
pub fn codcel_ave_dev(values: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Ok(0.0);
    }

    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let avedev = values.iter().map(|&x| (x - mean).abs()).sum::<f64>() / values.len() as f64;

    Ok(avedev)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ave_dev_positive_numbers() {
        // =AVEDEV(4,5,6,7,5,4) in US format
        // =AVEDEV(4;5;6;7;5;4) in German format
        let result = codcel_ave_dev(vec![4.0, 5.0, 6.0, 7.0, 5.0, 4.0]).unwrap();

        assert!((result - 0.888888888888889).abs() < 1e-10); // Average deviation from mean (5.167) is 1.0
    }

    #[test]
    fn test_ave_dev_negative_numbers() {
        // =AVEDEV(-4,-5,-6,-7,-5,-4) in US format
        // =AVEDEV(-4;-5;-6;-7;-5;-4) in German format
        let result = codcel_ave_dev(vec![-4.0, -5.0, -6.0, -7.0, -5.0, -4.0]).unwrap();
        println!("{result}");
        assert!((result - 0.888888888888889).abs() < 1e-10); // Average deviation from mean (-5.167) is 1.0
    }

    #[test]
    fn test_ave_dev_mixed_numbers() {
        // =AVEDEV(-2,-1,0,1,2) in US format
        // =AVEDEV(-2;-1;0;1;2) in German format
        let result = codcel_ave_dev(vec![-2.0, -1.0, 0.0, 1.0, 2.0]).unwrap();
        assert!((result - 1.2).abs() < 1e-10); // Average deviation from mean (0) is 1.2
    }

    #[test]
    fn test_ave_dev_single_value() {
        // =AVEDEV(5) in US format
        // =AVEDEV(5) in German format
        let result = codcel_ave_dev(vec![5.0]).unwrap();
        assert_eq!(result, 0.0); // Average deviation from mean (5) is 0
    }

    #[test]
    fn test_ave_dev_empty() {
        // =AVEDEV() in US format (returns 0 in Excel)
        // =AVEDEV() in German format (returns 0 in Excel)
        let result = codcel_ave_dev(vec![]).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_ave_dev_decimals() {
        // =AVEDEV(1.5,2.5,3.5,4.5) in US format
        // =AVEDEV(1,5;2,5;3,5;4,5) in German format
        let result = codcel_ave_dev(vec![1.5, 2.5, 3.5, 4.5]).unwrap();
        assert!((result - 1.0).abs() < 1e-10); // Average deviation from mean (3.0) is 1.0
    }

    #[test]
    fn test_ave_dev_same_values() {
        // =AVEDEV(3,3,3,3,3) in US format
        // =AVEDEV(3;3;3;3;3) in German format
        let result = codcel_ave_dev(vec![3.0, 3.0, 3.0, 3.0, 3.0]).unwrap();
        assert_eq!(result, 0.0); // Average deviation from mean (3) is 0
    }
}
