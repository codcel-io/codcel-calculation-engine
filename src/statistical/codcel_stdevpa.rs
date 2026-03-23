// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `STDEVPA` that returns the population standard deviation,
/// including text and logical values in the calculation.
/// - `values`: an array of numeric values (must have at least 1 value).
///
/// Text values are treated as 0, `TRUE` as 1, `FALSE` as 0 (coercion is
/// handled at the wrapper layer before calling this function).
///
/// Returns the population standard deviation (divides by n),
/// or an error when the array is empty.
pub fn codcel_stdevpa(values: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Err(
            "STDEVPA: At least one value is required to calculate standard deviation.".into(),
        );
    }

    let n = values.len() as f64;
    let mean = values.iter().sum::<f64>() / n;

    let variance = values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n;

    Ok(crate::portable_math::sqrt(variance))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stdevpa_basic() {
        // =STDEVPA({2,4,6,8,10})
        // Population stddev: mean=6, variance=8, stddev=2.8284
        let values = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let result = codcel_stdevpa(values).unwrap();
        assert!((result - 2.8284).abs() < 0.0001);
    }

    #[test]
    fn test_stdevpa_same_values() {
        // =STDEVPA({5,5,5,5,5})
        let values = vec![5.0, 5.0, 5.0, 5.0, 5.0];
        let result = codcel_stdevpa(values).unwrap();
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_stdevpa_single_value() {
        // =STDEVPA({7}) -- population stddev of one value is 0
        let values = vec![7.0];
        let result = codcel_stdevpa(values).unwrap();
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_stdevpa_two_values() {
        // =STDEVPA({2,8})
        // Population stddev: mean=5, variance=((2-5)^2+(8-5)^2)/2 = 9, stddev=3.0
        let values = vec![2.0, 8.0];
        let result = codcel_stdevpa(values).unwrap();
        assert!((result - 3.0).abs() < 0.0001);
    }

    #[test]
    fn test_stdevpa_with_zeros_from_text() {
        // Simulates STDEVPA where text values were coerced to 0
        // =STDEVPA({1, "hello", 3}) would become {1, 0, 3}
        let values = vec![1.0, 0.0, 3.0];
        let result = codcel_stdevpa(values).unwrap();
        // mean = 4/3, variance = ((1-4/3)^2 + (0-4/3)^2 + (3-4/3)^2) / 3
        // = (1/9 + 16/9 + 25/9) / 3 = 42/27 ≈ 1.5556
        // stddev = sqrt(1.5556) ≈ 1.2472
        assert!((result - 1.2472).abs() < 0.0001);
    }

    #[test]
    fn test_stdevpa_with_boolean_coercion() {
        // Simulates STDEVPA where TRUE=1, FALSE=0
        // =STDEVPA({TRUE, FALSE, 5}) would become {1, 0, 5}
        let values = vec![1.0, 0.0, 5.0];
        let result = codcel_stdevpa(values).unwrap();
        // mean = 2.0, variance = ((1-2)^2 + (0-2)^2 + (5-2)^2) / 3
        // = (1 + 4 + 9) / 3 = 14/3 ≈ 4.6667
        // stddev = sqrt(4.6667) ≈ 2.1602
        assert!((result - 2.1602).abs() < 0.0001);
    }

    #[test]
    fn test_stdevpa_negative_values() {
        // =STDEVPA({-2,-4,-6,-8,-10})
        let values = vec![-2.0, -4.0, -6.0, -8.0, -10.0];
        let result = codcel_stdevpa(values).unwrap();
        assert!((result - 2.8284).abs() < 0.0001);
    }

    #[test]
    fn test_stdevpa_empty_array() {
        let values: Vec<f64> = vec![];
        let result = codcel_stdevpa(values);
        assert!(result.is_err());
    }

    #[test]
    fn test_stdevpa_differs_from_stdeva() {
        // Population (N) gives a smaller result than sample (N-1)
        // For {2, 4, 6, 8, 10}:
        //   STDEVA  (sample, N-1): 3.1623
        //   STDEVPA (population, N): 2.8284
        let values = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let result = codcel_stdevpa(values).unwrap();
        assert!(result < 3.1623);
        assert!((result - 2.8284).abs() < 0.0001);
    }
}
