// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `KURT` that returns the kurtosis of a data set.
/// - `values`: an array of numeric values (must have at least 4 values).
///
/// Returns the kurtosis, which characterizes the relative peakedness or flatness
/// of a distribution compared to the normal distribution.
/// A positive kurtosis indicates a peaked distribution; negative indicates a flat distribution.
pub fn codcel_kurt(values: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Need at least 4 values for kurtosis
    if values.len() < 4 {
        return Err("KURT: Need at least 4 values to calculate kurtosis".into());
    }

    let n = values.len() as f64;

    // Calculate mean
    let mean = values.iter().sum::<f64>() / n;

    // Sample variance and standard deviation
    let variance = values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1.0);

    if variance == 0.0 {
        return Err("KURT: Division by zero: variance is 0".into());
    }

    let std_dev = crate::portable_math::sqrt(variance);

    // Sum of standardized values raised to 4th power: Σ((xᵢ - x̄)/s)⁴
    let sum_z4: f64 = values.iter().map(|x| ((x - mean) / std_dev).powi(4)).sum();

    // Excel's KURT formula:
    // KURT = n(n+1)/((n-1)(n-2)(n-3)) × Σ((xᵢ - x̄)/s)⁴ - 3(n-1)²/((n-2)(n-3))
    let kurtosis = (n * (n + 1.0)) / ((n - 1.0) * (n - 2.0) * (n - 3.0)) * sum_z4
        - 3.0 * (n - 1.0) * (n - 1.0) / ((n - 2.0) * (n - 3.0));

    Ok(kurtosis)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kurt_normal_distribution() {
        // =KURT({1,2,3,4,5,6,7,8,9,10}) in US format
        // =KURT({1;2;3;4;5;6;7;8;9;10}) in German format
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let result = codcel_kurt(values).unwrap();
        println!("{}", result);
        assert!((result - (-1.2)).abs() < 1e-10);
    }

    #[test]
    fn test_kurt_uniform_distribution() {
        // =KURT({1,1,1,1,2,2,2,2}) in US format
        // =KURT({1;1;1;1;2;2;2;2}) in German format
        let values = vec![1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0];
        let result = codcel_kurt(values).unwrap();
        println!("{}", result);
        assert!((result - (-2.8)).abs() < 1e-10);
    }

    #[test]
    fn test_kurt_leptokurtic() {
        // =KURT({1,1,1,1,1,1,1,10,10,10}) in US format
        // =KURT({1;1;1;1;1;1;1;10;10;10}) in German format
        let values = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 10.0, 10.0, 10.0];
        let result = codcel_kurt(values).unwrap();
        println!("{}", result);
        assert!((result - (-1.2244898)).abs() < 1e-4);
    }

    #[test]
    fn test_kurt_decimal_values() {
        // =KURT({1.5,2.5,3.5,4.5,5.5}) in US format
        // =KURT({1,5;2,5;3,5;4,5;5,5}) in German format
        let values = vec![1.5, 2.5, 3.5, 4.5, 5.5];
        let result = codcel_kurt(values).unwrap();
        println!("{}", result);
        assert!((result - (-1.2)).abs() < 1e-10);
    }

    #[test]
    fn test_kurt_negative_values() {
        // =KURT({-5,-4,-3,-2,-1,0,1,2,3,4,5}) in US format
        // =KURT({-5;-4;-3;-2;-1;0;1;2;3;4;5}) in German format
        let values = vec![-5.0, -4.0, -3.0, -2.0, -1.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_kurt(values).unwrap();
        println!("{}", result);
        assert!((result - (-1.2)).abs() < 1e-10);
    }

    #[test]
    fn test_kurt_too_few_values() {
        // =KURT({1,2,3}) in US format
        // =KURT({1;2;3}) in German format
        let values = vec![1.0, 2.0, 3.0];
        let result = codcel_kurt(values);
        assert!(result.is_err());
    }

    #[test]
    fn test_kurt_zero_variance() {
        // =KURT({5,5,5,5,5}) in US format
        // =KURT({5;5;5;5;5}) in German format
        let values = vec![5.0, 5.0, 5.0, 5.0, 5.0];
        let result = codcel_kurt(values);
        assert!(result.is_err());
    }
}
