// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::statistical::codcel_beta_dist::codcel_beta_dist;
use std::error::Error;

/// Excel-compatible `BETADIST` function.
/// Evaluates the cumulative beta distribution.
/// - `x`: value at which to evaluate the distribution (between `a` and `b`, defaults to 0–1).
/// - `alpha` / `beta`: shape parameters that must be positive.
/// - `a` / `b`: optional lower/upper bounds for scaling the distribution.
///
/// Returns the cumulative probability or an error when inputs are outside the allowed range.
pub fn codcel_betadist(
    x: f64,
    alpha: f64,
    beta: f64,
    a: Option<f64>,
    b: Option<f64>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    codcel_beta_dist(x, alpha, beta, true, a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_betadist_basic() {
        // =BETADIST(0.5, 2, 3) in US format
        // =BETADIST(0,5; 2; 3) in German format
        let result = codcel_betadist(0.5, 2.0, 3.0, None, None).unwrap();
        println!("{result}");
        assert!((result - 0.6875).abs() < 0.0001);
    }

    #[test]
    fn test_betadist_with_bounds() {
        // =BETADIST(3, 2, 3, 1, 5) in US format
        // =BETADIST(3; 2; 3; 1; 5) in German format
        let result = codcel_betadist(3.0, 2.0, 3.0, Some(1.0), Some(5.0)).unwrap();
        println!("{result}");
        assert!((result - 0.6875).abs() < 0.0001);
    }

    #[test]
    fn test_betadist_symmetric() {
        // =BETADIST(0.5, 2, 2) in US format
        // =BETADIST(0,5; 2; 2) in German format
        let result = codcel_betadist(0.5, 2.0, 2.0, None, None).unwrap();
        println!("{result}");
        assert!((result - 0.5).abs() < 0.0001);
    }

    #[test]
    fn test_betadist_alpha_greater() {
        // =BETADIST(0.7, 5, 2) in US format
        // =BETADIST(0,7; 5; 2) in German format
        let result = codcel_betadist(0.7, 5.0, 2.0, None, None).unwrap();
        println!("{result}");
        assert!((result - 0.420175).abs() < 0.0001);
    }

    #[test]
    fn test_betadist_beta_greater() {
        // =BETADIST(0.3, 2, 5) in US format
        // =BETADIST(0,3; 2; 5) in German format
        let result = codcel_betadist(0.3, 2.0, 5.0, None, None).unwrap();
        println!("{result}");
        assert!((result - 0.579825).abs() < 0.0001);
    }

    #[test]
    fn test_betadist_boundary_x_zero() {
        // =BETADIST(0, 2, 3) in US format
        // =BETADIST(0; 2; 3) in German format
        let result = codcel_betadist(0.0, 2.0, 3.0, None, None).unwrap();
        println!("{result}");
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_betadist_boundary_x_one() {
        // =BETADIST(1, 2, 3) in US format
        // =BETADIST(1; 2; 3) in German format
        let result = codcel_betadist(1.0, 2.0, 3.0, None, None).unwrap();
        println!("{result}");
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_betadist_invalid_x() {
        // =BETADIST(1.5, 2, 3) in US format (returns #NUM! error)
        // =BETADIST(1,5; 2; 3) in German format (returns #NUM! error)
        let result = codcel_betadist(1.5, 2.0, 3.0, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_betadist_invalid_alpha() {
        // =BETADIST(0.5, 0, 3) in US format (returns #NUM! error)
        // =BETADIST(0,5; 0; 3) in German format (returns #NUM! error)
        let result = codcel_betadist(0.5, 0.0, 3.0, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_betadist_invalid_beta() {
        // =BETADIST(0.5, 2, -1) in US format (returns #NUM! error)
        // =BETADIST(0,5; 2; -1) in German format (returns #NUM! error)
        let result = codcel_betadist(0.5, 2.0, -1.0, None, None);
        assert!(result.is_err());
    }
}
