// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `TREND` that returns values along a linear trend.
/// - `known_y`: the set of known y-values.
/// - `known_x`: optional set of known x-values (defaults to 1, 2, 3, ...).
/// - `new_x`: optional set of new x-values for which to calculate y-values (defaults to known_x).
/// - `const_flag`: if `true` or omitted, the constant b is calculated normally;
///   if `false`, the y-intercept is forced to 0.
///
/// Returns an array of y-values along the linear regression line (y = mx + b).
pub fn codcel_trend(
    known_y: Vec<f64>,
    known_x: Option<Vec<f64>>,
    new_x: Option<Vec<f64>>,
    const_flag: Option<bool>,
) -> Result<Vec<f64>, Box<dyn Error + Send + Sync>> {
    if known_y.is_empty() {
        return Err("TREND: known_y cannot be empty.".into());
    }

    let const_flag = const_flag.unwrap_or(true);

    let known_x = match known_x {
        Some(x) => {
            if x.len() != known_y.len() {
                return Err("TREND: known_x and known_y must have the same length.".into());
            }
            x
        }
        None => (1..=known_y.len()).map(|i| i as f64).collect(), // Default to 1, 2, 3, ...
    };

    let new_x = new_x.unwrap_or_else(|| known_x.clone());

    if new_x.is_empty() {
        return Err("TREND: new_x cannot be empty.".into());
    }

    let mean_x = known_x.iter().sum::<f64>() / known_x.len() as f64;
    let mean_y = known_y.iter().sum::<f64>() / known_y.len() as f64;

    let covariance = known_x
        .iter()
        .zip(known_y.iter())
        .map(|(&x, &y)| (x - mean_x) * (y - mean_y))
        .sum::<f64>();

    let variance_x = known_x.iter().map(|&x| (x - mean_x).powi(2)).sum::<f64>();

    if variance_x == 0.0 {
        return Err("TREND: Variance of x is zero. Cannot compute trend.".into());
    }

    let slope = covariance / variance_x;
    let intercept = if const_flag {
        mean_y - slope * mean_x
    } else {
        0.0
    };

    let trend = new_x
        .iter()
        .map(|&x| slope * x + intercept)
        .collect::<Vec<f64>>();

    Ok(trend)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trend_basic() {
        // =TREND({1,2,3},{1,2,3},{4}) in US format
        // =TREND({1;2;3};{1;2;3};{4}) in German format
        let known_y = vec![1.0, 2.0, 3.0];
        let known_x = Some(vec![1.0, 2.0, 3.0]);
        let new_x = Some(vec![4.0]);
        let result = codcel_trend(known_y, known_x, new_x, None).unwrap();
        println!("{result:?}");
        assert!((result[0] - 4.0).abs() < 0.0001);
    }

    #[test]
    fn test_trend_multiple_new_x() {
        // =TREND({1,2,3},{1,2,3},{4,5,6}) in US format
        // =TREND({1;2;3};{1;2;3};{4;5;6}) in German format
        let known_y = vec![1.0, 2.0, 3.0];
        let known_x = Some(vec![1.0, 2.0, 3.0]);
        let new_x = Some(vec![4.0, 5.0, 6.0]);
        let result = codcel_trend(known_y, known_x, new_x, None).unwrap();
        println!("{result:?}");
        assert!((result[0] - 4.0).abs() < 0.0001);
        assert!((result[1] - 5.0).abs() < 0.0001);
        assert!((result[2] - 6.0).abs() < 0.0001);
    }

    #[test]
    fn test_trend_default_known_x() {
        // =TREND({1,2,3},,,TRUE) in US format
        // =TREND({1;2;3};;;TRUE) in German format
        let known_y = vec![1.0, 2.0, 3.0];
        let result = codcel_trend(known_y, None, None, Some(true)).unwrap();
        println!("{result:?}");
        assert!(result.len() == 3);
        assert!((result[0] - 1.0).abs() < 0.0001);
        assert!((result[1] - 2.0).abs() < 0.0001);
        assert!((result[2] - 3.0).abs() < 0.0001);
    }

    #[test]
    fn test_trend_default_new_x() {
        // =TREND({1,2,3},{1,2,3}) in US format
        // =TREND({1;2;3};{1;2;3}) in German format
        let known_y = vec![1.0, 2.0, 3.0];
        let known_x = Some(vec![1.0, 2.0, 3.0]);
        let result = codcel_trend(known_y, known_x, None, None).unwrap();
        println!("{result:?}");
        assert!(result.len() == 3);
        assert!((result[0] - 1.0).abs() < 0.0001);
        assert!((result[1] - 2.0).abs() < 0.0001);
        assert!((result[2] - 3.0).abs() < 0.0001);
    }

    #[test]
    fn test_trend_const_flag_false() {
        // =TREND({1,2,3},{1,2,3},{4},FALSE) in US format
        // =TREND({1;2;3};{1;2;3};{4};FALSE) in German format
        let known_y = vec![1.0, 2.0, 3.0];
        let known_x = Some(vec![1.0, 2.0, 3.0]);
        let new_x = Some(vec![4.0]);
        let result = codcel_trend(known_y, known_x, new_x, Some(false)).unwrap();
        println!("{result:?}");
        assert!((result[0] - 4.0).abs() < 0.0001);
    }

    #[test]
    fn test_trend_non_linear_data() {
        // =TREND({1,4,9},{1,2,3},{4}) in US format
        // =TREND({1;4;9};{1;2;3};{4}) in German format
        let known_y = vec![1.0, 4.0, 9.0];
        let known_x = Some(vec![1.0, 2.0, 3.0]);
        let new_x = Some(vec![4.0]);
        let result = codcel_trend(known_y, known_x, new_x, None).unwrap();
        println!("{result:?}");
        assert!((result[0] - 12.666666666666668).abs() < 0.0001);
    }

    #[test]
    fn test_trend_empty_known_y() {
        // Empty known_y should return an error
        let known_y: Vec<f64> = vec![];
        let known_x = Some(vec![1.0, 2.0, 3.0]);
        let result = codcel_trend(known_y, known_x, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_trend_mismatched_lengths() {
        // Mismatched lengths should return an error
        let known_y = vec![1.0, 2.0, 3.0];
        let known_x = Some(vec![1.0, 2.0]);
        let result = codcel_trend(known_y, known_x, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_trend_zero_variance() {
        // Zero variance in x should return an error
        let known_y = vec![1.0, 2.0, 3.0];
        let known_x = Some(vec![2.0, 2.0, 2.0]);
        let result = codcel_trend(known_y, known_x, None, None);
        assert!(result.is_err());
    }
}
