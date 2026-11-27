// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::statistical::codcel_linest::codcel_linest;
use std::error::Error;

/// Excel-compatible `LOGEST` that returns statistics for an exponential regression curve.
/// - `known_ys`: the dependent array of known y-values (must be positive).
/// - `known_xs`: optional independent array of known x-values (defaults to 1, 2, 3, ...).
/// - `constant`: if `true` or omitted, calculates the constant b normally;
///   if `false`, forces b to 1 in the equation y = b * m^x.
/// - `stats`: if `true`, returns additional regression statistics; if `false` or omitted, returns only coefficients.
///
/// Returns an array containing the coefficient m and constant b for the exponential curve y = b * m^x
/// (b is 1 when `constant` is `false`, and still returned, as in Excel).
///
/// As Excel defines it, this is LINEST of ln(y): the coefficients are exponentiated
/// (m = e^slope, b = e^intercept), and every statistic, standard errors included, is
/// the one for the fit of ln(y), not exponentiated.
pub fn codcel_logest(
    known_ys: Vec<f64>,
    known_xs: Option<Vec<f64>>,
    constant: Option<bool>,
    stats: Option<bool>,
) -> Result<Vec<Vec<f64>>, Box<dyn Error + Send + Sync>> {
    if known_ys.is_empty() {
        return Err("LOGEST: known_ys must not be empty.".into());
    }

    if let Some(xs) = &known_xs {
        if xs.is_empty() {
            return Err("LOGEST: known_xs must not be empty.".into());
        }
        if known_ys.len() != xs.len() {
            return Err("LOGEST: known_ys and known_xs must have the same size.".into());
        }
    }

    // Can't take the logarithm of a non-positive y
    if known_ys.iter().any(|&y| y <= 0.0) {
        return Err("LOGEST: All y values must be positive for exponential regression.".into());
    }

    let ln_ys: Vec<f64> = known_ys
        .iter()
        .map(|&y| crate::portable_math::ln(y))
        .collect();
    // One observation per row, as LINEST takes them
    let xs = known_xs.map(|xs| xs.into_iter().map(|x| vec![x]).collect());

    let mut result = codcel_linest(ln_ys, xs, constant, stats)
        .map_err(|e| e.to_string().replacen("LINEST", "LOGEST", 1))?;

    // Only the coefficients go back to exponential form; b = e^0 = 1 when there is no constant
    for coefficient in result[0].iter_mut() {
        *coefficient = crate::portable_math::exp(*coefficient);
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

        // Should return [m, 1] where y = m^x: Excel still returns b
        assert!((result[0][0] - 3.0).abs() < 0.0001); // m should be approximately 3
        assert_eq!(result[0][1], 1.0);
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

        // Second row: standard errors of ln(m) and ln(b), as Excel reports them (not
        // exponentiated). The fit is perfect, so they are 0 up to rounding.
        assert!(result[1][0].abs() < 1e-9);
        assert!(result[1][1].abs() < 1e-9);

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

        // Second row: standard error of ln(m), as Excel reports it (not exponentiated).
        // The fit is perfect, so it is 0 up to rounding.
        assert!(result[1][0].abs() < 1e-9);

        // Third row: R-squared and standard error of regression
        assert!(result[2][0] > 0.99); // R-squared close to 1
                                      // Standard error might be zero, NaN, or a positive value in the no-constant case
                                      // Just verify it's not negative or infinite
        assert!(!result[2][1].is_sign_negative() && !result[2][1].is_infinite());

        // Fourth row: F-statistic and degrees of freedom
        // Excel reports F for a no-constant model too, from the uncentred sums of squares
        assert!(result[3][0] > 1000.0);
        assert!(result[1][1].is_nan()); // se(b) is #N/A
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

    #[test]
    fn test_logest_no_constant_matches_excel() {
        // =LOGEST(B2:B11,C2:C11, FALSE, FALSE) in the statistical regression workbook
        let known_ys = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let known_xs = vec![2.0, 4.0, 7.0, 11.0, 17.0, 25.0, 35.0, 47.0, 62.0, 80.0];
        let result = codcel_logest(known_ys, Some(known_xs), Some(false), Some(false)).unwrap();
        assert_eq!(result.len(), 1);
        assert!((result[0][0] - 1.0403082957930034).abs() < 1e-9);
        assert_eq!(result[0][1], 1.0);
    }

    #[test]
    fn test_logest_stats_match_excel() {
        // =LOGEST(B2:B11,C2:C11, TRUE, TRUE) in the statistical regression workbook
        let known_ys = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let known_xs = vec![2.0, 4.0, 7.0, 11.0, 17.0, 25.0, 35.0, 47.0, 62.0, 80.0];
        let result = codcel_logest(known_ys, Some(known_xs), Some(true), Some(true)).unwrap();
        let excel = [
            [1.0231955489991669, 2.3290331926283376],
            [0.005398133910020889, 0.2075440335015363],
            [0.6928331648390377, 0.43090500386444447],
            [18.04447838845564, 8.0],
            [3.3504829105297307, 1.4854329788433351],
        ];
        assert_eq!(result.len(), 5);
        for (row, expected) in result.iter().zip(excel) {
            for (got, want) in row.iter().zip(expected) {
                assert!((got - want).abs() < 1e-9, "got {got}, Excel {want}");
            }
        }
    }
}
