// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use statrs::distribution::{Beta, Continuous, ContinuousCDF};
use std::error::Error;

/// Excel-compatible `BETA.DIST` that evaluates the beta probability distribution.
/// - `x`: value at which to evaluate the distribution (between `a` and `b`, defaults to 0–1).
/// - `alpha` / `beta`: shape parameters that must be positive.
/// - `cumulative`: if `true`, returns the cumulative distribution function (CDF);
///   if `false`, returns the probability density function (PDF).
/// - `a` / `b`: optional lower/upper bounds for scaling the distribution.
///
/// Returns the cumulative probability or density, or an error when inputs are outside the allowed range.
pub fn codcel_beta_dist(
    x: f64,
    alpha: f64,
    beta: f64,
    cumulative: bool,
    a: Option<f64>,
    b: Option<f64>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let a = a.unwrap_or(0.0);
    let b = b.unwrap_or(1.0);

    if x < a || x > b || alpha <= 0.0 || beta <= 0.0 {
        return Err(format!("BETADIST: Invalid input parameters x={x:}, alpha={alpha:}, beta={beta:}, cumulative={cumulative:} a={a:}, b={b:}").into());
    }

    let scaled_x = (x - a) / (b - a); // Rescale x to [0, 1]

    let beta_dist = Beta::new(alpha, beta)?;
    if cumulative {
        Ok(beta_dist.cdf(scaled_x))
    } else {
        // Scale the PDF by 1/(b-a) to account for the change of variable from [0,1] to [a,b]
        Ok(beta_dist.pdf(scaled_x) / (b - a))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beta_dist_cumulative() {
        // =BETA.DIST(0.5,2,3,TRUE) in US format
        // =BETA.DIST(0,5;2;3;TRUE) in German format
        let result = codcel_beta_dist(0.5, 2.0, 3.0, true, None, None).unwrap();
        assert!((result - 0.6875).abs() < 1e-10); // CDF of Beta(2,3) at x=0.5
    }

    #[test]
    fn test_beta_dist_pdf() {
        // =BETA.DIST(0.5,2,3,FALSE) in US format
        // =BETA.DIST(0,5;2;3;FALSE) in German format
        let result = codcel_beta_dist(0.5, 2.0, 3.0, false, None, None).unwrap();
        assert!((result - 1.5).abs() < 1e-10); // PDF of Beta(2,3) at x=0.5
    }

    #[test]
    fn test_beta_dist_custom_bounds() {
        // =BETA.DIST(3,2,3,TRUE,1,5) in US format
        // =BETA.DIST(3;2;3;TRUE;1;5) in German format
        let result = codcel_beta_dist(3.0, 2.0, 3.0, true, Some(1.0), Some(5.0)).unwrap();
        assert!((result - 0.6875).abs() < 1e-10); // CDF of Beta(2,3) at x=0.5 (scaled from [1,5] to [0,1])
    }

    #[test]
    fn test_beta_dist_symmetric() {
        // =BETA.DIST(0.5,2,2,TRUE) in US format
        // =BETA.DIST(0,5;2;2;TRUE) in German format
        let result = codcel_beta_dist(0.5, 2.0, 2.0, true, None, None).unwrap();
        assert!((result - 0.5).abs() < 1e-10); // CDF of symmetric Beta(2,2) at x=0.5 is 0.5
    }

    #[test]
    fn test_beta_dist_alpha_greater() {
        // =BETA.DIST(0.7,5,2,TRUE) in US format
        // =BETA.DIST(0,7;5;2;TRUE) in German format
        let result = codcel_beta_dist(0.7, 5.0, 2.0, true, None, None).unwrap();
        assert!((result - 0.420175).abs() < 1e-5); // CDF of Beta(5,2) at x=0.7
    }

    #[test]
    fn test_beta_dist_beta_greater() {
        // =BETA.DIST(0.3,2,5,TRUE) in US format
        // =BETA.DIST(0,3;2;5;TRUE) in German format
        let result = codcel_beta_dist(0.3, 2.0, 5.0, true, None, None).unwrap();
        assert!((result - 0.579825).abs() < 1e-5); // CDF of Beta(2,5) at x=0.3
    }

    #[test]
    fn test_beta_dist_boundary_x_zero() {
        // =BETA.DIST(0,2,3,TRUE) in US format
        // =BETA.DIST(0;2;3;TRUE) in German format
        let result = codcel_beta_dist(0.0, 2.0, 3.0, true, None, None).unwrap();
        assert_eq!(result, 0.0); // CDF of Beta(2,3) at x=0 is 0
    }

    #[test]
    fn test_beta_dist_boundary_x_one() {
        // =BETA.DIST(1,2,3,TRUE) in US format
        // =BETA.DIST(1;2;3;TRUE) in German format
        let result = codcel_beta_dist(1.0, 2.0, 3.0, true, None, None).unwrap();
        assert_eq!(result, 1.0); // CDF of Beta(2,3) at x=1 is 1
    }

    #[test]
    fn test_beta_dist_pdf_custom_bounds() {
        // =BETA.DIST(3,2,3,FALSE,1,5) in US format → 0.375
        let result = codcel_beta_dist(3.0, 2.0, 3.0, false, Some(1.0), Some(5.0)).unwrap();
        assert!((result - 0.375).abs() < 1e-10); // PDF scaled by 1/(b-a) = 1/4
    }

    #[test]
    fn test_beta_dist_invalid_x() {
        // =BETA.DIST(1.5,2,3,TRUE) in US format (returns #NUM! error)
        // =BETA.DIST(1,5;2;3;TRUE) in German format (returns #NUM! error)
        let result = codcel_beta_dist(1.5, 2.0, 3.0, true, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_beta_dist_invalid_alpha() {
        // =BETA.DIST(0.5,0,3,TRUE) in US format (returns #NUM! error)
        // =BETA.DIST(0,5;0;3;TRUE) in German format (returns #NUM! error)
        let result = codcel_beta_dist(0.5, 0.0, 3.0, true, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_beta_dist_invalid_beta() {
        // =BETA.DIST(0.5,2,-1,TRUE) in US format (returns #NUM! error)
        // =BETA.DIST(0,5;2;-1;TRUE) in German format (returns #NUM! error)
        let result = codcel_beta_dist(0.5, 2.0, -1.0, true, None, None);
        assert!(result.is_err());
    }
}
