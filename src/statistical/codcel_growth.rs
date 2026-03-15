// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use nalgebra::{DMatrix, DVector};
use std::error::Error;

/// Excel-compatible `GROWTH` that calculates predicted exponential growth using existing data.
/// - `known_y`: the set of known y-values (must be positive).
/// - `known_x`: optional set of known x-values (defaults to 1, 2, 3, ...).
/// - `new_x`: optional set of new x-values for which to predict y-values (defaults to known_x).
/// - `const_b`: if `true` or omitted, the constant b is calculated normally;
///   if `false`, b is set to 1 and the equation becomes y = m^x.
///
/// Returns an array of predicted y-values based on exponential regression (y = b * m^x).
pub fn codcel_growth(
    known_y: Vec<f64>,
    known_x: Option<Vec<f64>>,
    new_x: Option<Vec<f64>>,
    const_b: Option<bool>,
) -> Result<Vec<f64>, Box<dyn Error + Send + Sync>> {
    // Validate input data
    if known_y.is_empty() {
        return Err("known_y array cannot be empty".into());
    }

    // Special case: if there's only one known y value, return it for all new_x points
    if known_y.len() == 1 {
        let new_x_len = match new_x {
            Some(x) => {
                if x.is_empty() {
                    return Err("new_x array cannot be empty".into());
                }
                x.len()
            }
            None => 1,
        };
        return Ok(vec![known_y[0]; new_x_len]);
    }

    // Generate sequential x values if known_x is None
    let x_values = match known_x {
        Some(x) => {
            if x.len() != known_y.len() {
                return Err("known_x and known_y must have the same length".into());
            }
            x.to_vec()
        }
        None => (1..=known_y.len()).map(|i| i as f64).collect(),
    };

    // Use sequential values if new_x is None
    let new_x_values = match new_x {
        Some(x) => {
            if x.is_empty() {
                return Err("new_x array cannot be empty".into());
            }
            x.to_vec()
        }
        None => x_values.clone(),
    };

    // Take natural log of y values
    let ln_y: Vec<f64> = known_y
        .iter()
        .map(|&y| {
            if y <= 0.0 {
                return Err("All y values must be positive for logarithmic regression".into());
            }
            Ok(y.ln())
        })
        .collect::<Result<Vec<f64>, Box<dyn Error + Send + Sync>>>()?;

    // Create matrices for linear regression
    let n = x_values.len();
    let mut x_matrix = if const_b.unwrap_or(true) {
        DMatrix::from_element(n, 2, 1.0)
    } else {
        DMatrix::from_element(n, 1, 1.0)
    };

    // Fill in x values
    if const_b.unwrap_or(true) {
        for i in 0..n {
            x_matrix[(i, 1)] = x_values[i];
        }
    } else {
        for i in 0..n {
            x_matrix[(i, 0)] = x_values[i];
        }
    }

    let y_vector = DVector::from_vec(ln_y);

    // Calculate coefficients using least squares method
    let coefficients = (x_matrix.transpose() * &x_matrix)
        .try_inverse()
        .ok_or("Matrix inverse calculation failed")?
        * x_matrix.transpose()
        * y_vector;

    // Calculate predicted values for all new_x values
    let predicted_values = new_x_values
        .iter()
        .map(|&x| {
            let predicted = if const_b.unwrap_or(true) {
                coefficients[0] + coefficients[1] * x
            } else {
                coefficients[0] * x
            };
            predicted.exp()
        })
        .collect();

    Ok(predicted_values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_growth_basic() {
        // =GROWTH({2,4,8,16}, {1,2,3,4}, {5,6}) in US format
        // =GROWTH({2;4;8;16}; {1;2;3;4}; {5;6}) in German format
        let known_y = vec![2.0, 4.0, 8.0, 16.0];
        let known_x = Some(vec![1.0, 2.0, 3.0, 4.0]);
        let new_x = Some(vec![5.0, 6.0]);
        let result = codcel_growth(known_y, known_x, new_x, None).unwrap();

        assert!((result[0] - 32.0).abs() < 1e-10);
        assert!((result[1] - 64.0).abs() < 1e-10);
    }

    #[test]
    fn test_growth_without_known_x() {
        // =GROWTH({2,4,8,16},, {5,6}) in US format
        // =GROWTH({2;4;8;16};; {5;6}) in German format
        let known_y = vec![2.0, 4.0, 8.0, 16.0];
        let new_x = Some(vec![5.0, 6.0]);
        let result = codcel_growth(known_y, None, new_x, None).unwrap();

        assert!((result[0] - 32.0).abs() < 1e-10);
        assert!((result[1] - 64.0).abs() < 1e-10);
    }

    #[test]
    fn test_growth_without_new_x() {
        // =GROWTH({2,4,8,16}, {1,2,3,4}) in US format
        // =GROWTH({2;4;8;16}; {1;2;3;4}) in German format
        let known_y = vec![2.0, 4.0, 8.0, 16.0];
        let known_x = Some(vec![1.0, 2.0, 3.0, 4.0]);
        let result = codcel_growth(known_y, known_x, None, None).unwrap();

        assert!(result.len() == 4);
        assert!((result[0] - 2.0).abs() < 1e-10);
        assert!((result[1] - 4.0).abs() < 1e-10);
        assert!((result[2] - 8.0).abs() < 1e-10);
        assert!((result[3] - 16.0).abs() < 1e-10);
    }

    #[test]
    fn test_growth_const_b_false() {
        // =GROWTH({2,4,8,16}, {1,2,3,4}, {5,6}, FALSE) in US format
        // =GROWTH({2;4;8;16}; {1;2;3;4}; {5;6}; FALSE) in German format
        let known_y = vec![2.0, 4.0, 8.0, 16.0];
        let known_x = Some(vec![1.0, 2.0, 3.0, 4.0]);
        let new_x = Some(vec![5.0, 6.0]);
        let result = codcel_growth(known_y, known_x, new_x, Some(false)).unwrap();

        // The results will be different when const_b is false
        assert!(result.len() == 2);
    }

    #[test]
    fn test_growth_single_y_value() {
        // =GROWTH({5}, {1}, {2,3,4}) in US format
        // =GROWTH({5}; {1}; {2;3;4}) in German format
        let known_y = vec![5.0];
        let known_x = Some(vec![1.0]);
        let new_x = Some(vec![2.0, 3.0, 4.0]);
        let result = codcel_growth(known_y, known_x, new_x, None).unwrap();

        assert!(result.len() == 3);
        assert!((result[0] - 5.0).abs() < 1e-10);
        assert!((result[1] - 5.0).abs() < 1e-10);
        assert!((result[2] - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_growth_negative_y_value() {
        // =GROWTH({2,4,-8,16}, {1,2,3,4}, {5,6}) in US format
        // =GROWTH({2;4;-8;16}; {1;2;3;4}; {5;6}) in German format
        let known_y = vec![2.0, 4.0, -8.0, 16.0];
        let known_x = Some(vec![1.0, 2.0, 3.0, 4.0]);
        let new_x = Some(vec![5.0, 6.0]);
        let result = codcel_growth(known_y, known_x, new_x, None);

        assert!(result.is_err());
    }

    #[test]
    fn test_growth_empty_known_y() {
        // =GROWTH({}, {}, {1,2}) in US format
        // =GROWTH({}; {}; {1;2}) in German format
        let known_y: Vec<f64> = vec![];
        let known_x = Some(vec![]);
        let new_x = Some(vec![1.0, 2.0]);
        let result = codcel_growth(known_y, known_x, new_x, None);

        assert!(result.is_err());
    }
}
