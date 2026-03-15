// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use statrs::distribution::{ContinuousCDF, Normal};
use std::error::Error;

/// Excel-compatible `Z.TEST` that returns the one-tailed p-value of a z-test.
/// - `data`: an array of sample values.
/// - `hyp_mean`: the hypothesized population mean to test against.
/// - `sigma`: optional known population standard deviation (defaults to sample standard deviation).
///
/// Returns the one-tailed probability that the sample mean differs from the hypothesized mean,
/// or an error when the data is empty.
pub fn codcel_z_dot_test(
    data: Vec<f64>,
    hyp_mean: f64,
    sigma: Option<f64>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let n = data.len();
    if n == 0 {
        return Err("Z.TEST: Data vector cannot be empty".into());
    }

    let mean = data.iter().sum::<f64>() / n as f64;

    let std_dev = if let Some(sigma) = sigma {
        sigma
    } else {
        // Compute sample standard deviation (unbiased)
        let variance = data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / (n - 1) as f64;
        variance.sqrt()
    };

    if std_dev == 0.0 {
        return Err("Z.TEST: Standard deviation cannot be zero".into());
    }

    let z_score = (mean - hyp_mean) / (std_dev / (n as f64).sqrt());

    // Compute the p-value using the standard normal distribution
    let normal = Normal::new(0.0, 1.0).map_err(|_| "Failed to create normal distribution")?;
    let p_value = 1.0 - normal.cdf(z_score);

    Ok(p_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_z_dot_test_with_sigma() {
        // =Z.TEST({3,4,5,2,3,4,5,6,4,7}, 3.5, 1.5) in US format
        // =Z.TEST({3;4;5;2;3;4;5;6;4;7}; 3,5; 1,5) in German format
        let data = vec![3.0, 4.0, 5.0, 2.0, 3.0, 4.0, 5.0, 6.0, 4.0, 7.0];
        let result = codcel_z_dot_test(data, 3.5, Some(1.5)).unwrap();
        println!("{result}");
        assert!((result - 0.04584514077663859).abs() < 0.0001);
    }

    #[test]
    fn test_z_dot_test_without_sigma() {
        // =Z.TEST({3,4,5,2,3,4,5,6,4,7}, 3.5) in US format
        // =Z.TEST({3;4;5;2;3;4;5;6;4;7}; 3,5) in German format
        let data = vec![3.0, 4.0, 5.0, 2.0, 3.0, 4.0, 5.0, 6.0, 4.0, 7.0];
        let result = codcel_z_dot_test(data, 3.5, None).unwrap();
        println!("{result}");
        assert!((result - 0.04524396496753047).abs() < 0.0001);
    }

    #[test]
    fn test_z_dot_test_different_hyp_mean() {
        // =Z.TEST({3,4,5,2,3,4,5,6,4,7}, 4, 1.5) in US format
        // =Z.TEST({3;4;5;2;3;4;5;6;4;7}; 4; 1,5) in German format
        let data = vec![3.0, 4.0, 5.0, 2.0, 3.0, 4.0, 5.0, 6.0, 4.0, 7.0];
        let result = codcel_z_dot_test(data, 4.0, Some(1.5)).unwrap();
        println!("{result}");
        assert!((result - 0.26354462843276916).abs() < 0.0001);
    }

    #[test]
    fn test_z_dot_test_different_data() {
        // =Z.TEST({5,6,7,8,9}, 6, 1.2) in US format
        // =Z.TEST({5;6;7;8;9}; 6; 1,2) in German format
        let data = vec![5.0, 6.0, 7.0, 8.0, 9.0];
        let result = codcel_z_dot_test(data, 6.0, Some(1.2)).unwrap();
        println!("{result}");
        assert!((result - 0.031203709282932257).abs() < 0.0001);
    }

    #[test]
    fn test_z_dot_test_same_values() {
        // =Z.TEST({5,5,5,5,5}, 4.5, 0.5) in US format
        // =Z.TEST({5;5;5;5;5}; 4,5; 0,5) in German format
        let data = vec![5.0, 5.0, 5.0, 5.0, 5.0];
        let result = codcel_z_dot_test(data, 4.5, Some(0.5)).unwrap();
        println!("{result}");
        assert!((result - 0.012673659338182652).abs() < 0.0001);
    }

    #[test]
    fn test_z_dot_test_negative_values() {
        // =Z.TEST({-3,-2,-1,0,1}, -2, 1.5) in US format
        // =Z.TEST({-3;-2;-1;0;1}; -2; 1,5) in German format
        let data = vec![-3.0, -2.0, -1.0, 0.0, 1.0];
        let result = codcel_z_dot_test(data, -2.0, Some(1.5)).unwrap();
        println!("{result}");
        assert!((result - 0.06801856406004303).abs() < 0.0001);
    }

    #[test]
    fn test_z_dot_test_empty_data() {
        // Empty data should return an error
        let data: Vec<f64> = vec![];
        let result = codcel_z_dot_test(data, 3.5, Some(1.5));
        assert!(result.is_err());
    }

    #[test]
    fn test_z_dot_test_zero_sigma() {
        // Zero sigma should return an error
        let data = vec![3.0, 4.0, 5.0, 2.0, 3.0, 4.0, 5.0, 6.0, 4.0, 7.0];
        let result = codcel_z_dot_test(data, 3.5, Some(0.0));
        assert!(result.is_err());
    }

    #[test]
    fn test_z_dot_test_same_values_no_sigma() {
        // Same values with no sigma should return an error (calculated std_dev would be 0)
        let data = vec![5.0, 5.0, 5.0, 5.0, 5.0];
        let result = codcel_z_dot_test(data, 4.5, None);
        assert!(result.is_err());
    }
}
