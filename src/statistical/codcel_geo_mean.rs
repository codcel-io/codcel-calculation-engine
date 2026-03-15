// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `GEOMEAN` that returns the geometric mean of an array of positive values.
/// - `values`: an array of positive numeric values.
///
/// Returns the n-th root of the product of all values,
/// or an error when the array is empty or contains non-positive values.
pub fn codcel_geo_mean(values: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Err("GEOMEAN: Input vector must not be empty.".into());
    }
    if values.iter().any(|&x| x <= 0.0) {
        return Err("GEOMEAN: All input values must be greater than 0.".into());
    }

    let product: f64 = values.iter().product();
    let n: f64 = values.len() as f64;

    Ok(product.powf(1.0 / n))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geo_mean_basic() {
        // =GEOMEAN(1, 2, 3, 4) in US format
        // =GEOMEAN(1; 2; 3; 4) in German format
        let values = vec![1.0, 2.0, 3.0, 4.0];
        let result = codcel_geo_mean(values).unwrap();
        assert!((result - 2.213363839400643).abs() < 1e-10);
    }

    #[test]
    fn test_geo_mean_same_values() {
        // =GEOMEAN(2, 2, 2, 2) in US format
        // =GEOMEAN(2; 2; 2; 2) in German format
        let values = vec![2.0, 2.0, 2.0, 2.0];
        let result = codcel_geo_mean(values).unwrap();
        assert!((result - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_geo_mean_single_value() {
        // =GEOMEAN(5) in US format
        // =GEOMEAN(5) in German format
        let values = vec![5.0];
        let result = codcel_geo_mean(values).unwrap();
        assert!((result - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_geo_mean_decimal_values() {
        // =GEOMEAN(1.5, 2.5, 3.5) in US format
        // =GEOMEAN(1,5; 2,5; 3,5) in German format
        let values = vec![1.5, 2.5, 3.5];
        let result = codcel_geo_mean(values).unwrap();
        println!("{result}");
        assert!((result - 2.3588469901582667).abs() < 1e-10);
    }

    #[test]
    fn test_geo_mean_large_values() {
        // =GEOMEAN(100, 1000, 10000) in US format
        // =GEOMEAN(100; 1000; 10000) in German format
        let values = vec![100.0, 1000.0, 10000.0];
        let result = codcel_geo_mean(values).unwrap();
        assert!((result - 1000.0).abs() < 1e-10);
    }

    #[test]
    fn test_geo_mean_negative_value() {
        // =GEOMEAN(1, 2, -3) in US format
        // =GEOMEAN(1; 2; -3) in German format
        let values = vec![1.0, 2.0, -3.0];
        let result = codcel_geo_mean(values);
        assert!(result.is_err());
    }

    #[test]
    fn test_geo_mean_zero_value() {
        // =GEOMEAN(1, 2, 0) in US format
        // =GEOMEAN(1; 2; 0) in German format
        let values = vec![1.0, 2.0, 0.0];
        let result = codcel_geo_mean(values);
        assert!(result.is_err());
    }

    #[test]
    fn test_geo_mean_empty() {
        // =GEOMEAN() in US format
        // =GEOMEAN() in German format
        let values: Vec<f64> = vec![];
        let result = codcel_geo_mean(values);
        assert!(result.is_err());
    }
}
