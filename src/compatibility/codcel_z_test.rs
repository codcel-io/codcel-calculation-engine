// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use statrs::distribution::{ContinuousCDF, Normal};
use std::error::Error;

/// Excel-compatible `ZTEST`/`Z.TEST` function.
/// Computes the one-tailed probability for the sample mean relative to a hypothesized mean.
/// - `data`: array of sample values.
/// - `hyp_mean`: hypothesized population mean.
/// - `sigma`: optional population standard deviation; if `None`, sample standard deviation is used.
///
/// Returns an error on empty data or non-positive sigma.
pub fn codcel_z_test(
    data: Vec<f64>,
    hyp_mean: f64,
    sigma: Option<f64>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if data.is_empty() {
        return Err("ZTEST: Data set cannot be empty.".into());
    }

    // Calculate the mean of the data set
    let sample_mean = data.iter().sum::<f64>() / data.len() as f64;

    // Calculate the standard deviation (either provided or derived from the sample)
    let std_dev = if let Some(s) = sigma {
        if s <= 0.0 {
            return Err("ZTEST: Population standard deviation must be greater than 0.".into());
        }
        s
    } else {
        // Use sample standard deviation (n-1 in denominator) when sigma is not provided
        let variance = data.iter().map(|&x| (x - sample_mean).powi(2)).sum::<f64>()
            / (data.len() as f64 - 1.0);

        crate::portable_math::sqrt(variance)
    };

    // Check if the standard deviation is positive
    if std_dev <= 0.0 {
        return Err("ZTEST: Standard deviation cannot be zero or negative.".into());
    }

    // Calculate the z-score
    let z_score =
        (sample_mean - hyp_mean) / (std_dev / crate::portable_math::sqrt(data.len() as f64));

    // Convert the z-score to a one-tailed probability
    let normal = Normal::new(0.0, 1.0).unwrap(); // mean = 0.0, std_dev = 1.0
    let p_value = 1.0 - normal.cdf(z_score);

    Ok(p_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_z_test_basic() {
        // =ZTEST({3,6,7,8,6,5,4,2,1,9}, 4) in US format
        // =ZTEST({3;6;7;8;6;5;4;2;1;9}; 4) in German format
        let data = vec![3.0, 6.0, 7.0, 8.0, 6.0, 5.0, 4.0, 2.0, 1.0, 9.0];
        let result = codcel_z_test(data, 4.0, None).unwrap();
        println!("{result}");
        assert!((result - 0.090574197).abs() < 0.0001);
    }

    #[test]
    fn test_z_test_with_sigma() {
        // =ZTEST({3,6,7,8,6,5,4,2,1,9}, 4, 2.5) in US format
        // =ZTEST({3;6;7;8;6;5;4;2;1;9}; 4; 2,5) in German format
        let data = vec![3.0, 6.0, 7.0, 8.0, 6.0, 5.0, 4.0, 2.0, 1.0, 9.0];
        let result = codcel_z_test(data, 4.0, Some(2.5)).unwrap();
        println!("{result}");
        assert!((result - 0.08205175341656645).abs() < 0.0001);
    }

    #[test]
    fn test_z_test_different_mean() {
        // =ZTEST({3,6,7,8,6,5,4,2,1,9}, 6) in US format
        // =ZTEST({3;6;7;8;6;5;4;2;1;9}; 6) in German format
        let data = vec![3.0, 6.0, 7.0, 8.0, 6.0, 5.0, 4.0, 2.0, 1.0, 9.0];
        let result = codcel_z_test(data, 6.0, None).unwrap();
        println!("{result}");
        assert!((result - 0.863043389).abs() < 0.0001);
    }

    #[test]
    fn test_z_test_different_data() {
        // =ZTEST({10,12,15,14,16,13,11}, 12) in US format
        // =ZTEST({10;12;15;14;16;13;11}; 12) in German format
        let data = vec![10.0, 12.0, 15.0, 14.0, 16.0, 13.0, 11.0];
        let result = codcel_z_test(data, 12.0, None).unwrap();
        println!("{result}");
        assert!((result - 0.110335681).abs() < 0.0001);
    }

    #[test]
    fn test_z_test_different_data_with_sigma() {
        // =ZTEST({10,12,15,14,16,13,11}, 12, 2) in US format
        // =ZTEST({10;12;15;14;16;13;11}; 12; 2) in German format
        let data = vec![10.0, 12.0, 15.0, 14.0, 16.0, 13.0, 11.0];
        let result = codcel_z_test(data, 12.0, Some(2.0)).unwrap();
        println!("{result}");
        assert!((result - 0.09293836618717777).abs() < 0.0001);
    }

    #[test]
    fn test_z_test_higher_mean() {
        // =ZTEST({3,6,7,8,6,5,4,2,1,9}, 10) in US format
        // =ZTEST({3;6;7;8;6;5;4;2;1;9}; 10) in German format
        let data = vec![3.0, 6.0, 7.0, 8.0, 6.0, 5.0, 4.0, 2.0, 1.0, 9.0];
        let result = codcel_z_test(data, 10.0, None).unwrap();
        println!("{result}");
        assert!((result - 0.9999999998295729).abs() < 0.0001);
    }

    #[test]
    fn test_z_test_lower_mean() {
        // =ZTEST({3,6,7,8,6,5,4,2,1,9}, 0) in US format
        // =ZTEST({3;6;7;8;6;5;4;2;1;9}; 0) in German format
        let data = vec![3.0, 6.0, 7.0, 8.0, 6.0, 5.0, 4.0, 2.0, 1.0, 9.0];
        let result = codcel_z_test(data, 0.0, None).unwrap();
        println!("{result}");
        assert!((result - 0.00000000003175382179421149).abs() < 0.0001);
    }

    #[test]
    fn test_z_test_negative_sigma() {
        // Negative sigma should return an error
        let data = vec![3.0, 6.0, 7.0, 8.0, 6.0, 5.0, 4.0, 2.0, 1.0, 9.0];
        let result = codcel_z_test(data, 4.0, Some(-1.0));
        assert!(result.is_err());
    }

    #[test]
    fn test_z_test_zero_sigma() {
        // Zero sigma should return an error
        let data = vec![3.0, 6.0, 7.0, 8.0, 6.0, 5.0, 4.0, 2.0, 1.0, 9.0];
        let result = codcel_z_test(data, 4.0, Some(0.0));
        assert!(result.is_err());
    }

    #[test]
    fn test_z_test_empty_data() {
        // Empty data should return an error
        let data: Vec<f64> = vec![];
        let result = codcel_z_test(data, 4.0, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_z_test_same_values() {
        // All same values will result in zero standard deviation
        let data = vec![5.0, 5.0, 5.0, 5.0, 5.0];
        let result = codcel_z_test(data, 4.0, None);
        assert!(result.is_err());
    }
}
