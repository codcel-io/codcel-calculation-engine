// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `LOGEST` that returns statistics for an exponential regression curve.
/// - `known_ys`: the dependent array of known y-values (must be positive).
/// - `known_xs`: optional independent array of known x-values (defaults to 1, 2, 3, ...).
/// - `constant`: if `true` or omitted, calculates the constant b normally;
///   if `false`, forces b to 1 in the equation y = b * m^x.
/// - `stats`: if `true`, returns additional regression statistics; if `false` or omitted, returns only coefficients.
///
/// Returns an array containing the coefficient(s) m and constant b for the exponential curve y = b * m^x.
pub fn codcel_logest(
    known_ys: Vec<f64>,
    known_xs: Option<Vec<f64>>,
    constant: Option<bool>,
    stats: Option<bool>,
) -> Result<Vec<Vec<f64>>, Box<dyn Error + Send + Sync>> {
    // Default values for optional parameters
    let constant = constant.unwrap_or(true);
    let stats = stats.unwrap_or(false);

    // Input validation
    if known_ys.is_empty() {
        return Err("LOGEST: known_ys must not be empty.".into());
    }

    // If known_xs is None, generate sequential x values (1, 2, 3, ...)
    let known_xs = match known_xs {
        Some(xs) => {
            // Validate that known_xs has the same length as known_ys
            if known_ys.len() != xs.len() {
                return Err("LOGEST: known_ys and known_xs must have the same size.".into());
            }
            if xs.is_empty() {
                return Err("LOGEST: known_xs must not be empty.".into());
            }
            xs
        }
        None => {
            // Generate sequential x values starting from 1
            (1..=known_ys.len()).map(|i| i as f64).collect()
        }
    };

    // Check for non-positive y values (can't take logarithm)
    if known_ys.iter().any(|&y| y <= 0.0) {
        return Err("LOGEST: All y values must be positive for exponential regression.".into());
    }

    // Take logarithm of y values for linear regression
    let ln_ys: Vec<f64> = known_ys.iter().map(|&y| y.ln()).collect();

    // Calculate means
    let mean_x = known_xs.iter().sum::<f64>() / known_xs.len() as f64;
    let mean_ln_y = ln_ys.iter().sum::<f64>() / ln_ys.len() as f64;

    // Calculate variance_x (needed for both cases and for statistics)
    let variance_x = known_xs
        .iter()
        .map(|&xi| (xi - mean_x).powi(2))
        .sum::<f64>();

    if variance_x == 0.0 {
        return Err("LOGEST: Division by zero due to zero variance in known_xs.".into());
    }

    // Calculate slope and intercept
    let (slope, intercept) = if constant {
        // With constant term: y = b * m^x
        let covariance = known_xs
            .iter()
            .zip(&ln_ys)
            .map(|(&xi, &ln_yi)| (xi - mean_x) * (ln_yi - mean_ln_y))
            .sum::<f64>();

        let slope = covariance / variance_x;
        let intercept = mean_ln_y - slope * mean_x;

        (slope, intercept)
    } else {
        // Without constant term: y = m^x (or ln(y) = x * ln(m))
        let sum_x_ln_y = known_xs
            .iter()
            .zip(&ln_ys)
            .map(|(&xi, &ln_yi)| xi * ln_yi)
            .sum::<f64>();
        let sum_x_squared = known_xs.iter().map(|&xi| xi.powi(2)).sum::<f64>();

        if sum_x_squared == 0.0 {
            return Err("LOGEST: Division by zero due to zero sum of squares in known_xs.".into());
        }

        let slope = sum_x_ln_y / sum_x_squared;

        // No intercept in this model
        (slope, 0.0)
    };

    // Convert back to exponential form
    let m = slope.exp(); // m in y = b * m^x
    let b = if constant { intercept.exp() } else { 1.0 }; // b in y = b * m^x

    if !stats {
        // Return just the coefficients
        if constant {
            // Return [m, b] as a 1x2 array
            return Ok(vec![vec![m, b]]);
        } else {
            // Return [m] as a 1x1 array
            return Ok(vec![vec![m]]);
        }
    }

    // Calculate additional statistics for the regression
    let n = known_ys.len() as f64;
    let df = if constant { n - 2.0 } else { n - 1.0 };

    if df <= 0.0 {
        return Err("LOGEST: Not enough data points for regression statistics.".into());
    }

    // Calculate fitted values and residuals
    let fitted_ln_ys: Vec<f64> = known_xs
        .iter()
        .map(|&x| {
            if constant {
                slope * x + intercept
            } else {
                slope * x
            }
        })
        .collect();

    let residuals: Vec<f64> = ln_ys
        .iter()
        .zip(&fitted_ln_ys)
        .map(|(&actual, &fitted)| actual - fitted)
        .collect();

    // Sum of squared residuals
    let sse = residuals.iter().map(|&r| r.powi(2)).sum::<f64>();

    // Total sum of squares
    let sst = if constant {
        ln_ys.iter().map(|&y| (y - mean_ln_y).powi(2)).sum::<f64>()
    } else {
        ln_ys.iter().map(|&y| y.powi(2)).sum::<f64>()
    };

    // R-squared
    let r_squared = if sst == 0.0 { 0.0 } else { 1.0 - (sse / sst) };

    // Standard error of the regression
    let se_regression = (sse / df).sqrt();

    // Standard errors of coefficients
    let se_slope = if variance_x == 0.0 {
        f64::NAN
    } else {
        se_regression / variance_x.sqrt()
    };

    let se_intercept = if constant {
        se_regression * ((1.0 / n) + (mean_x.powi(2) / variance_x)).sqrt()
    } else {
        f64::NAN
    };

    // F statistic
    let f_statistic = if constant && sst != 0.0 && df > 0.0 {
        (sst - sse) / (sse / df)
    } else {
        f64::NAN
    };

    // Degrees of freedom
    let df_regression = 1.0;
    let df_residual = n - df_regression - (if constant { 1.0 } else { 0.0 });

    // Regression sum of squares
    let ssr = sst - sse;

    // Prepare the result array
    let mut result = Vec::new();

    if constant {
        // First row: coefficients
        result.push(vec![m, b]);
        // Second row: standard errors
        result.push(vec![se_slope.exp(), se_intercept.exp()]);
        // Third row: R-squared and standard error of the regression
        result.push(vec![r_squared, se_regression]);
        // Fourth row: F-statistic and degrees of freedom
        result.push(vec![f_statistic, df_residual]);
        // Fifth row: Regression SS and Residual SS
        result.push(vec![ssr, sse]);
    } else {
        // First row: coefficient
        result.push(vec![m]);
        // Second row: standard error
        result.push(vec![se_slope.exp()]);
        // Third row: R-squared and standard error of the regression
        result.push(vec![r_squared, se_regression]);
        // Fourth row: F-statistic and degrees of freedom
        result.push(vec![f_statistic, df_residual]);
        // Fifth row: Regression SS and Residual SS
        result.push(vec![ssr, sse]);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logest_basic() {
        // Test with a simple exponential dataset
        // y = 2 * 3^x
        let known_ys = vec![2.0, 6.0, 18.0, 54.0];
        let known_xs = vec![0.0, 1.0, 2.0, 3.0];

        let result = codcel_logest(known_ys, Some(known_xs), Some(true), Some(false)).unwrap();

        // Should return [m, b] where y = b * m^x
        assert!((result[0][0] - 3.0).abs() < 0.0001); // m should be approximately 3
        assert!((result[0][1] - 2.0).abs() < 0.0001); // b should be approximately 2
    }

    #[test]
    fn test_logest_no_constant() {
        // Test without constant term
        // y = 3^x (b = 1)
        let known_ys = vec![1.0, 3.0, 9.0, 27.0];
        let known_xs = vec![0.0, 1.0, 2.0, 3.0];

        let result = codcel_logest(known_ys, Some(known_xs), Some(false), Some(false)).unwrap();

        // Should return [m] where y = m^x
        assert!((result[0][0] - 3.0).abs() < 0.0001); // m should be approximately 3
    }

    #[test]
    fn test_logest_with_stats() {
        // Test with stats
        let known_ys = vec![2.0, 6.0, 18.0, 54.0];
        let known_xs = vec![0.0, 1.0, 2.0, 3.0];

        let result = codcel_logest(known_ys, Some(known_xs), Some(true), Some(true)).unwrap();

        // Should return a 5x2 array with regression statistics
        assert_eq!(result.len(), 5);
        assert_eq!(result[0].len(), 2);

        // First row: coefficients
        assert!((result[0][0] - 3.0).abs() < 0.0001); // m
        assert!((result[0][1] - 2.0).abs() < 0.0001); // b

        // Check that R-squared is close to 1 for this perfect fit
        assert!(result[2][0] > 0.99);
    }

    #[test]
    fn test_logest_no_xs() {
        // Test with no x values provided (should use 1, 2, 3, 4 as x values)
        // y = 2 * 3^x with x = 1, 2, 3, 4
        let known_ys = vec![6.0, 18.0, 54.0, 162.0];

        let result = codcel_logest(known_ys, None, Some(true), Some(false)).unwrap();

        // Should return [m, b] where y = b * m^x
        assert!((result[0][0] - 3.0).abs() < 0.0001); // m should be approximately 3
        assert!((result[0][1] - 2.0).abs() < 0.0001); // b should be approximately 2
    }

    #[test]
    fn test_logest_negative_y() {
        // Negative y values should return an error
        let known_ys = vec![-2.0, 6.0, 18.0, 54.0];
        let known_xs = vec![0.0, 1.0, 2.0, 3.0];

        let result = codcel_logest(known_ys, Some(known_xs), None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_logest_zero_y() {
        // Zero y values should return an error
        let known_ys = vec![0.0, 6.0, 18.0, 54.0];
        let known_xs = vec![0.0, 1.0, 2.0, 3.0];

        let result = codcel_logest(known_ys, Some(known_xs), None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_logest_different_length_arrays() {
        // Different length arrays should return an error
        let known_ys = vec![2.0, 6.0, 18.0];
        let known_xs = vec![0.0, 1.0, 2.0, 3.0];

        let result = codcel_logest(known_ys, Some(known_xs), None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_logest_empty_arrays() {
        // Empty arrays should return an error
        let known_ys: Vec<f64> = vec![];
        let known_xs: Vec<f64> = vec![];

        let result = codcel_logest(known_ys, Some(known_xs), None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_logest_zero_variance() {
        // Zero variance in known_xs should return an error
        let known_ys = vec![2.0, 6.0, 18.0, 54.0];
        let known_xs = vec![1.0, 1.0, 1.0, 1.0];

        let result = codcel_logest(known_ys, Some(known_xs), None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_logest_real_data() {
        // Test with some real-world data
        let known_ys = vec![8.0, 18.0, 33.0, 74.0, 148.0];
        let known_xs = vec![1.0, 2.0, 3.0, 4.0, 5.0];

        let result = codcel_logest(known_ys, Some(known_xs), Some(true), Some(false)).unwrap();

        // The growth factor should be approximately 2
        assert!((result[0][0] - 2.0).abs() < 0.1);
    }

    #[test]
    fn test_logest_large_values() {
        // Test with very large values
        // y = 2 * 3^x with large values
        let known_ys = vec![2.0, 6.0, 18.0, 54.0, 162.0, 486.0, 1458.0, 4374.0];
        let known_xs = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];

        let result = codcel_logest(known_ys, Some(known_xs), Some(true), Some(false)).unwrap();

        // Should return [m, b] where y = b * m^x
        assert!((result[0][0] - 3.0).abs() < 0.0001); // m should be approximately 3
        assert!((result[0][1] - 2.0).abs() < 0.0001); // b should be approximately 2
    }

    #[test]
    fn test_logest_small_values() {
        // Test with very small values
        // y = 0.5 * 0.7^x
        let known_ys = vec![0.5, 0.35, 0.245, 0.1715, 0.12005];
        let known_xs = vec![0.0, 1.0, 2.0, 3.0, 4.0];

        let result = codcel_logest(known_ys, Some(known_xs), Some(true), Some(false)).unwrap();

        // Should return [m, b] where y = b * m^x
        assert!((result[0][0] - 0.7).abs() < 0.0001); // m should be approximately 0.7
        assert!((result[0][1] - 0.5).abs() < 0.0001); // b should be approximately 0.5
    }

    #[test]
    fn test_logest_non_integer_x() {
        // Test with non-integer x values
        // y = 2 * 3^x
        let known_ys = vec![2.0, 3.464, 6.0, 10.392, 18.0];
        let known_xs = vec![0.0, 0.5, 1.0, 1.5, 2.0];

        let result = codcel_logest(known_ys, Some(known_xs), Some(true), Some(false)).unwrap();

        // Should return [m, b] where y = b * m^x
        assert!((result[0][0] - 3.0).abs() < 0.0001); // m should be approximately 3
        assert!((result[0][1] - 2.0).abs() < 0.0001); // b should be approximately 2
    }

    #[test]
    fn test_logest_non_sequential_x() {
        // Test with non-sequential x values
        // y = 2 * 3^x
        let known_ys = vec![2.0, 18.0, 162.0, 1458.0];
        let known_xs = vec![0.0, 2.0, 4.0, 6.0];

        let result = codcel_logest(known_ys, Some(known_xs), Some(true), Some(false)).unwrap();

        // Should return [m, b] where y = b * m^x
        assert!((result[0][0] - 3.0).abs() < 0.0001); // m should be approximately 3
        assert!((result[0][1] - 2.0).abs() < 0.0001); // b should be approximately 2
    }

    #[test]
    fn test_logest_minimal_data_with_constant() {
        // Test with minimal data points (2) for constant=true
        // y = 2 * 3^x
        let known_ys = vec![2.0, 6.0];
        let known_xs = vec![0.0, 1.0];

        let result = codcel_logest(known_ys, Some(known_xs), Some(true), Some(false)).unwrap();

        // Should return [m, b] where y = b * m^x
        assert!((result[0][0] - 3.0).abs() < 0.0001); // m should be approximately 3
        assert!((result[0][1] - 2.0).abs() < 0.0001); // b should be approximately 2
    }

    #[test]
    fn test_logest_minimal_data_no_constant() {
        // Test with minimal data points (1) for constant=false
        // y = 3^x
        let known_ys = vec![1.0, 3.0];
        let known_xs = vec![0.0, 1.0];

        let result = codcel_logest(known_ys, Some(known_xs), Some(false), Some(false)).unwrap();

        // Should return [m] where y = m^x
        assert!((result[0][0] - 3.0).abs() < 0.0001); // m should be approximately 3
    }

    #[test]
    fn test_logest_imperfect_fit() {
        // Test with imperfect fit
        // Points don't lie exactly on an exponential curve
        let known_ys = vec![2.1, 5.8, 18.3, 53.7, 163.2];
        let known_xs = vec![0.0, 1.0, 2.0, 3.0, 4.0];

        let result = codcel_logest(known_ys, Some(known_xs), Some(true), Some(true)).unwrap();

        // Check that R-squared is less than 1 but still high for this good fit
        assert!(result[2][0] < 1.0);
        assert!(result[2][0] > 0.99);

        // The growth factor should be approximately 3
        assert!((result[0][0] - 3.0).abs() < 0.1);
    }

    #[test]
    fn test_logest_stats_detailed() {
        // Test detailed statistics output
        // y = 2 * 3^x with perfect fit
        let known_ys = vec![2.0, 6.0, 18.0, 54.0, 162.0];
        let known_xs = vec![0.0, 1.0, 2.0, 3.0, 4.0];

        let result = codcel_logest(known_ys, Some(known_xs), Some(true), Some(true)).unwrap();

        // First row: coefficients [m, b]
        assert!((result[0][0] - 3.0).abs() < 0.0001); // m
        assert!((result[0][1] - 2.0).abs() < 0.0001); // b

        // Second row: standard errors
        // For exponential regression, standard errors can be larger
        // Just verify they are positive and finite
        assert!(result[1][0] > 0.0 && !result[1][0].is_infinite());
        assert!(result[1][1] > 0.0 && !result[1][1].is_infinite());

        // Third row: R-squared and standard error of regression
        assert!(result[2][0] > 0.99); // R-squared close to 1
        assert!(result[2][1] > 0.0 && !result[2][1].is_infinite()); // Standard error should be positive and finite

        // Fourth row: F-statistic and degrees of freedom
        // F-statistic should be very large for perfect fit
        assert!(result[3][0] > 1000.0);
        assert!((result[3][1] - 3.0).abs() < 0.0001); // df = n - 2 = 5 - 2 = 3

        // Fifth row: Regression SS and Residual SS
        assert!(result[4][0] > 0.0); // Regression SS should be positive
        assert!(result[4][1] >= 0.0); // Residual SS should be non-negative
    }

    #[test]
    fn test_logest_stats_no_constant() {
        // Test statistics without constant term
        // y = 3^x
        let known_ys = vec![1.0, 3.0, 9.0, 27.0, 81.0];
        let known_xs = vec![0.0, 1.0, 2.0, 3.0, 4.0];

        let result = codcel_logest(known_ys, Some(known_xs), Some(false), Some(true)).unwrap();

        // First row: coefficient [m]
        assert!((result[0][0] - 3.0).abs() < 0.0001); // m

        // Second row: standard error
        // For exponential regression without constant, standard errors can be larger
        // Just verify it is positive and finite
        assert!(result[1][0] > 0.0 && !result[1][0].is_infinite());

        // Third row: R-squared and standard error of regression
        assert!(result[2][0] > 0.99); // R-squared close to 1
                                      // Standard error might be zero, NaN, or a positive value in the no-constant case
                                      // Just verify it's not negative or infinite
        assert!(!result[2][1].is_sign_negative() && !result[2][1].is_infinite());

        // Fourth row: F-statistic and degrees of freedom
        // For no-constant model, F-statistic is set to NaN in the implementation
        assert!(result[3][0].is_nan()); // F-statistic should be NaN
        assert!((result[3][1] - 4.0).abs() < 0.0001); // df = n - 1 = 5 - 1 = 4

        // Fifth row: Regression SS and Residual SS
        assert!(result[4][0] > 0.0); // Regression SS should be positive
        assert!(result[4][1] >= 0.0); // Residual SS should be non-negative
    }

    #[test]
    fn test_logest_not_enough_data_for_stats() {
        // Test with not enough data points for regression statistics
        // Need at least 3 points for constant=true to calculate statistics
        let known_ys = vec![2.0, 6.0];
        let known_xs = vec![0.0, 1.0];

        let result = codcel_logest(known_ys, Some(known_xs), Some(true), Some(true));
        assert!(result.is_err());
    }

    #[test]
    fn test_logest_decreasing_exponential() {
        // Test with decreasing exponential function
        // y = 100 * 0.5^x
        let known_ys = vec![100.0, 50.0, 25.0, 12.5, 6.25];
        let known_xs = vec![0.0, 1.0, 2.0, 3.0, 4.0];

        let result = codcel_logest(known_ys, Some(known_xs), Some(true), Some(false)).unwrap();

        // Should return [m, b] where y = b * m^x
        assert!((result[0][0] - 0.5).abs() < 0.0001); // m should be approximately 0.5
        assert!((result[0][1] - 100.0).abs() < 0.0001); // b should be approximately 100
    }

    #[test]
    fn test_logest_flat_curve() {
        // Test with flat exponential curve (growth factor close to 1)
        // y = 10 * 1.01^x
        let known_ys = vec![10.0, 10.1, 10.201, 10.303, 10.406];
        let known_xs = vec![0.0, 1.0, 2.0, 3.0, 4.0];

        let result = codcel_logest(known_ys, Some(known_xs), Some(true), Some(false)).unwrap();

        // Should return [m, b] where y = b * m^x
        assert!((result[0][0] - 1.01).abs() < 0.0001); // m should be approximately 1.01
        assert!((result[0][1] - 10.0).abs() < 0.0001); // b should be approximately 10
    }

    #[test]
    fn test_logest_steep_curve() {
        // Test with steep exponential curve (high growth factor)
        // y = 1 * 10^x
        let known_ys = vec![1.0, 10.0, 100.0, 1000.0, 10000.0];
        let known_xs = vec![0.0, 1.0, 2.0, 3.0, 4.0];

        let result = codcel_logest(known_ys, Some(known_xs), Some(true), Some(false)).unwrap();

        // Should return [m, b] where y = b * m^x
        assert!((result[0][0] - 10.0).abs() < 0.0001); // m should be approximately 10
        assert!((result[0][1] - 1.0).abs() < 0.0001); // b should be approximately 1
    }

    #[test]
    fn test_logest_default_parameters() {
        // Test with default parameters (constant=true, stats=false)
        // y = 2 * 3^x
        let known_ys = vec![2.0, 6.0, 18.0, 54.0];
        let known_xs = vec![0.0, 1.0, 2.0, 3.0];

        let result = codcel_logest(known_ys, Some(known_xs), None, None).unwrap();

        // Should return [m, b] where y = b * m^x
        assert!((result[0][0] - 3.0).abs() < 0.0001); // m should be approximately 3
        assert!((result[0][1] - 2.0).abs() < 0.0001); // b should be approximately 2
    }

    #[test]
    fn test_logest_fractional_growth() {
        // Test with fractional growth rate
        // y = 5 * (4/3)^x
        let known_ys = vec![5.0, 6.667, 8.889, 11.852, 15.802];
        let known_xs = vec![0.0, 1.0, 2.0, 3.0, 4.0];

        let result = codcel_logest(known_ys, Some(known_xs), Some(true), Some(false)).unwrap();

        // Should return [m, b] where y = b * m^x
        assert!((result[0][0] - 4.0 / 3.0).abs() < 0.0001); // m should be approximately 4/3
                                                            // Allow a bit more tolerance for b since the input values are approximations
        assert!((result[0][1] - 5.0).abs() < 0.01); // b should be approximately 5
    }

    #[test]
    fn test_logest_very_close_to_zero() {
        // Test with y values very close to zero (but still positive)
        // y = 1e-10 * 2^x
        let known_ys = vec![1e-10, 2e-10, 4e-10, 8e-10, 16e-10];
        let known_xs = vec![0.0, 1.0, 2.0, 3.0, 4.0];

        let result = codcel_logest(known_ys, Some(known_xs), Some(true), Some(false)).unwrap();

        // Should return [m, b] where y = b * m^x
        assert!((result[0][0] - 2.0).abs() < 0.0001); // m should be approximately 2
        assert!((result[0][1] - 1e-10).abs() < 1e-12); // b should be approximately 1e-10
    }

    #[test]
    fn test_logest_negative_x_values() {
        // Test with negative x values
        // y = 2 * 3^x with x = -2, -1, 0, 1, 2
        let known_ys = vec![2.0 / 9.0, 2.0 / 3.0, 2.0, 6.0, 18.0];
        let known_xs = vec![-2.0, -1.0, 0.0, 1.0, 2.0];

        let result = codcel_logest(known_ys, Some(known_xs), Some(true), Some(false)).unwrap();

        // Should return [m, b] where y = b * m^x
        assert!((result[0][0] - 3.0).abs() < 0.0001); // m should be approximately 3
        assert!((result[0][1] - 2.0).abs() < 0.0001); // b should be approximately 2
    }
}
