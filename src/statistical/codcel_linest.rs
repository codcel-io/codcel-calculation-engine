// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use nalgebra::DMatrix;
use std::error::Error;

/// Excel-compatible `LINEST` that returns statistics for a linear regression line.
/// Supports simple and multiple regression.
/// - `known_ys`: the dependent array of known y-values.
/// - `known_xs`: optional 2D array of known x-values. Each row is one observation,
///   each column is one independent variable. Defaults to {1, 2, 3, ...}.
/// - `constant`: if `true` or omitted, calculates the y-intercept normally;
///   if `false`, forces the y-intercept to 0.
/// - `stats`: if `true`, returns additional regression statistics; if `false` or omitted, returns only coefficients.
///
/// Returns an array containing slope(s) and intercept (in reverse order: m_k, ..., m_1, b),
/// and optionally additional regression statistics.
pub fn codcel_linest(
    known_ys: Vec<f64>,
    known_xs: Option<Vec<Vec<f64>>>,
    constant: Option<bool>,
    stats: Option<bool>,
) -> Result<Vec<Vec<f64>>, Box<dyn Error + Send + Sync>> {
    let constant = constant.unwrap_or(true);
    let stats = stats.unwrap_or(false);

    let n = known_ys.len();
    if n == 0 {
        return Err("LINEST: known_ys must not be empty.".into());
    }

    // Build the X matrix: each row is one observation
    // If known_xs is None, use {1, 2, 3, ...} as a single independent variable
    let x_cols: Vec<Vec<f64>> = match known_xs {
        Some(mut xs_2d) => {
            // xs_2d is row-major: xs_2d[row][col]
            if xs_2d.is_empty() {
                return Err("LINEST: known_xs must not be empty.".into());
            }

            // If X is a single row but Y has multiple elements, transpose X
            // (Excel treats a 1×N X array as N observations when Y has N elements)
            if xs_2d.len() == 1 && xs_2d[0].len() == n && n > 1 {
                xs_2d = xs_2d[0].iter().map(|&v| vec![v]).collect();
            }

            let n_rows = xs_2d.len();
            if n_rows != n {
                return Err("LINEST: known_ys and known_xs must have the same number of rows.".into());
            }
            let k = xs_2d[0].len();
            if k == 0 {
                return Err("LINEST: known_xs must have at least one column.".into());
            }
            // Transpose to column-major for easier processing
            let mut cols = vec![vec![0.0; n]; k];
            for i in 0..n {
                if xs_2d[i].len() != k {
                    return Err("LINEST: All rows in known_xs must have the same number of columns.".into());
                }
                for j in 0..k {
                    cols[j][i] = xs_2d[i][j];
                }
            }
            cols
        }
        None => {
            vec![(1..=n).map(|i| i as f64).collect()]
        }
    };

    let k = x_cols.len(); // number of independent variables
    let p = if constant { k + 1 } else { k }; // total parameters

    // Build the design matrix
    let n_cols = p;
    let mut x_data = vec![0.0; n * n_cols];
    for i in 0..n {
        for j in 0..k {
            x_data[i * n_cols + j] = x_cols[j][i];
        }
        if constant {
            x_data[i * n_cols + k] = 1.0; // intercept column
        }
    }

    let x = DMatrix::from_row_slice(n, n_cols, &x_data);
    let y = DMatrix::from_column_slice(n, 1, &known_ys);

    // Solve via normal equations: (X^T X) beta = X^T y
    let xtx = x.transpose() * &x;
    let xty = x.transpose() * &y;

    let beta = xtx.clone().lu().solve(&xty)
        .ok_or("LINEST: Cannot solve the linear system (singular matrix).")?;

    // Extract coefficients
    let mut slopes: Vec<f64> = (0..k).map(|j| beta[(j, 0)]).collect();
    let intercept = if constant { beta[(k, 0)] } else { 0.0 };

    // Excel returns coefficients in reverse order: m_k, m_{k-1}, ..., m_1, b
    slopes.reverse();

    if !stats {
        let mut row = slopes;
        if constant {
            row.push(intercept);
        }
        return Ok(vec![row]);
    }

    // Calculate statistics
    let n_f = n as f64;
    let df_residual = if constant { n_f - k as f64 - 1.0 } else { n_f - k as f64 };

    if df_residual <= 0.0 {
        return Err("LINEST: Not enough data points for regression statistics.".into());
    }

    // Fitted values and residuals
    let y_hat = &x * &beta;
    let residuals = &y - &y_hat;

    // SSE (sum of squared errors)
    let sse: f64 = residuals.iter().map(|r| r * r).sum();

    // SST (total sum of squares)
    let mean_y = known_ys.iter().sum::<f64>() / n_f;
    let sst = if constant {
        known_ys.iter().map(|&yi| (yi - mean_y).powi(2)).sum::<f64>()
    } else {
        known_ys.iter().map(|&yi| yi.powi(2)).sum::<f64>()
    };

    // SSR (regression sum of squares)
    let ssr = sst - sse;

    // R-squared
    let r_squared = if sst == 0.0 { 0.0 } else { 1.0 - (sse / sst) };

    // Standard error of regression
    let se_regression = crate::portable_math::sqrt(sse / df_residual);

    // Standard errors of coefficients: sqrt(diag((X^T X)^{-1}) * MSE)
    let mse = sse / df_residual;
    let xtx_inv = xtx.lu().solve(&DMatrix::identity(p, p));
    let se_coeffs: Vec<f64> = if let Some(inv) = xtx_inv {
        (0..p).map(|j| crate::portable_math::sqrt(inv[(j, j)] * mse)).collect()
    } else {
        vec![f64::NAN; p]
    };

    // Standard errors in reverse order to match coefficients
    let mut se_slopes: Vec<f64> = (0..k).map(|j| se_coeffs[j]).collect();
    se_slopes.reverse();
    let se_intercept = if constant { se_coeffs[k] } else { f64::NAN };

    // F-statistic
    let df_regression = k as f64;
    let f_statistic = if constant && sst != 0.0 && df_residual > 0.0 {
        (ssr / df_regression) / (sse / df_residual)
    } else {
        f64::NAN
    };

    // Build result array
    // Row 1: coefficients (m_k, ..., m_1, b)
    let mut row1 = slopes;
    if constant {
        row1.push(intercept);
    }

    // Row 2: standard errors
    let mut row2 = se_slopes;
    if constant {
        row2.push(se_intercept);
    }

    // Row 3: R-squared, standard error of regression
    let width = row1.len();
    let mut row3 = vec![0.0; width];
    row3[0] = r_squared;
    row3[1] = se_regression;

    // Row 4: F-statistic, degrees of freedom
    let mut row4 = vec![0.0; width];
    row4[0] = f_statistic;
    row4[1] = df_residual;

    // Row 5: Regression SS, Residual SS
    let mut row5 = vec![0.0; width];
    row5[0] = ssr;
    row5[1] = sse;

    Ok(vec![row1, row2, row3, row4, row5])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn xs_1d(xs: Vec<f64>) -> Option<Vec<Vec<f64>>> {
        Some(xs.into_iter().map(|x| vec![x]).collect())
    }

    #[test]
    fn test_linest_basic() {
        // y = 2x + 3
        let known_ys = vec![3.0, 5.0, 7.0, 9.0];
        let known_xs = xs_1d(vec![0.0, 1.0, 2.0, 3.0]);
        let result = codcel_linest(known_ys, known_xs, Some(true), Some(false)).unwrap();
        assert!((result[0][0] - 2.0).abs() < 0.0001);
        assert!((result[0][1] - 3.0).abs() < 0.0001);
    }

    #[test]
    fn test_linest_no_constant() {
        // y = 2x (b = 0)
        let known_ys = vec![0.0, 2.0, 4.0, 6.0];
        let known_xs = xs_1d(vec![0.0, 1.0, 2.0, 3.0]);
        let result = codcel_linest(known_ys, known_xs, Some(false), Some(false)).unwrap();
        assert!((result[0][0] - 2.0).abs() < 0.0001);
    }

    #[test]
    fn test_linest_with_stats() {
        let known_ys = vec![3.0, 5.0, 7.0, 9.0];
        let known_xs = xs_1d(vec![0.0, 1.0, 2.0, 3.0]);
        let result = codcel_linest(known_ys, known_xs, Some(true), Some(true)).unwrap();
        assert_eq!(result.len(), 5);
        assert_eq!(result[0].len(), 2);
        assert!((result[0][0] - 2.0).abs() < 0.0001);
        assert!((result[0][1] - 3.0).abs() < 0.0001);
        assert!(result[2][0] > 0.99); // R-squared
    }

    #[test]
    fn test_linest_no_xs() {
        // x = 1, 2, 3, 4 => y = 2x + 1
        let known_ys = vec![3.0, 5.0, 7.0, 9.0];
        let result = codcel_linest(known_ys, None, Some(true), Some(false)).unwrap();
        assert!((result[0][0] - 2.0).abs() < 0.0001);
        assert!((result[0][1] - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_linest_multiple_regression() {
        // y = 2*x1 + 3*x2 + 1
        let known_ys = vec![
            2.0 * 1.0 + 3.0 * 0.0 + 1.0,   // 3.0
            2.0 * 0.0 + 3.0 * 1.0 + 1.0,   // 4.0
            2.0 * 1.0 + 3.0 * 1.0 + 1.0,   // 6.0
            2.0 * 2.0 + 3.0 * 1.0 + 1.0,   // 8.0
            2.0 * 1.0 + 3.0 * 2.0 + 1.0,   // 9.0
        ];
        let known_xs = Some(vec![
            vec![1.0, 0.0],
            vec![0.0, 1.0],
            vec![1.0, 1.0],
            vec![2.0, 1.0],
            vec![1.0, 2.0],
        ]);
        let result = codcel_linest(known_ys, known_xs, Some(true), Some(false)).unwrap();
        // Excel returns [m2, m1, b] (reverse order)
        assert!((result[0][0] - 3.0).abs() < 0.0001); // m2
        assert!((result[0][1] - 2.0).abs() < 0.0001); // m1
        assert!((result[0][2] - 1.0).abs() < 0.0001); // b
    }

    #[test]
    fn test_linest_test_data() {
        // Test data matching the failing generated test
        // Y: [1, 3, 7, 10, 15], X: [[1,2],[3,1],[2,4],[4,3],[5,2]]
        let known_ys = vec![1.0, 3.0, 7.0, 10.0, 15.0];
        let known_xs = Some(vec![
            vec![1.0, 2.0],
            vec![3.0, 1.0],
            vec![2.0, 4.0],
            vec![4.0, 3.0],
            vec![5.0, 2.0],
        ]);
        let result = codcel_linest(known_ys, known_xs, None, None).unwrap();
        // Expected: result[0][0] ≈ 1.901960784313725 (Excel's value)
        assert!((result[0][0] - 1.901960784313725).abs() < 0.0001);
    }

    #[test]
    fn test_linest_different_length_arrays() {
        let known_ys = vec![3.0, 5.0, 7.0];
        let known_xs = xs_1d(vec![0.0, 1.0, 2.0, 3.0]);
        let result = codcel_linest(known_ys, known_xs, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_linest_empty_arrays() {
        let known_ys: Vec<f64> = vec![];
        let result = codcel_linest(known_ys, Some(vec![]), None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_linest_zero_variance() {
        let known_ys = vec![3.0, 5.0, 7.0, 9.0];
        let known_xs = xs_1d(vec![1.0, 1.0, 1.0, 1.0]);
        let result = codcel_linest(known_ys, known_xs, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_linest_negative_slope() {
        // y = -3x + 10
        let known_ys = vec![10.0, 7.0, 4.0, 1.0];
        let known_xs = xs_1d(vec![0.0, 1.0, 2.0, 3.0]);
        let result = codcel_linest(known_ys, known_xs, Some(true), Some(false)).unwrap();
        assert!((result[0][0] + 3.0).abs() < 0.0001);
        assert!((result[0][1] - 10.0).abs() < 0.0001);
    }

    #[test]
    fn test_linest_horizontal_line() {
        // y = 5
        let known_ys = vec![5.0, 5.0, 5.0, 5.0];
        let known_xs = xs_1d(vec![1.0, 2.0, 3.0, 4.0]);
        let result = codcel_linest(known_ys, known_xs, Some(true), Some(false)).unwrap();
        assert!((result[0][0]).abs() < 0.0001); // slope ≈ 0
        assert!((result[0][1] - 5.0).abs() < 0.0001);
    }

    #[test]
    fn test_linest_stats_detailed() {
        // y = 2x + 3 with perfect fit
        let known_ys = vec![3.0, 5.0, 7.0, 9.0];
        let known_xs = xs_1d(vec![0.0, 1.0, 2.0, 3.0]);
        let result = codcel_linest(known_ys, known_xs, Some(true), Some(true)).unwrap();

        assert!((result[0][0] - 2.0).abs() < 0.0001); // m
        assert!((result[0][1] - 3.0).abs() < 0.0001); // b
        assert!(result[1][0] < 0.0001); // se(m)
        assert!(result[1][1] < 0.0001); // se(b)
        assert!(result[2][0] > 0.99); // R-squared
        assert!(result[2][1] < 0.0001); // se(y)
        assert!(result[3][0] > 1000.0); // F
        assert!((result[3][1] - 2.0).abs() < 0.0001); // df
    }

    #[test]
    fn test_linest_not_enough_data_for_stats() {
        let known_ys = vec![3.0, 5.0];
        let known_xs = xs_1d(vec![0.0, 1.0]);
        let result = codcel_linest(known_ys, known_xs, Some(true), Some(true));
        assert!(result.is_err());
    }
}
