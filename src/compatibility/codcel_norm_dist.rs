// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::statistical::standard_normal::{std_normal_cdf, std_normal_pdf};
use std::error::Error;

/// Excel-compatible `NORMDIST`/`NORM.DIST` function.
/// Evaluates the normal distribution.
/// - `x`: value at which to evaluate the distribution.
/// - `mean`: arithmetic mean of the distribution.
/// - `std_dev`: standard deviation of the distribution (must be greater than 0).
/// - `cumulative`: `true` for cumulative distribution (CDF), `false` for probability density (PDF).
///
/// Returns an error if `std_dev` is not positive.
pub fn codcel_norm_dist(
    x: f64,
    mean: f64,
    std_dev: f64,
    cumulative: bool,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if std_dev <= 0.0 {
        return Err("NORMDIST: Standard deviation must be greater than 0.".into());
    }

    let z = (x - mean) / std_dev;

    if cumulative {
        Ok(std_normal_cdf(z))
    } else {
        // The density is scaled by 1/std_dev to keep unit area after the change of variable.
        Ok(std_normal_pdf(z) / std_dev)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Expected values were computed with mpmath at 60 decimal digits and rounded to the nearest
    // f64.

    #[test]
    fn test_norm_dist_pdf_basic() {
        // =NORMDIST(1, 0, 1, FALSE) in US format
        // =NORMDIST(1; 0; 1; FALSE) in German format
        let result = codcel_norm_dist(1.0, 0.0, 1.0, false).unwrap();
        assert!((result - 0.24197072451914334).abs() < 1e-16);
    }

    #[test]
    fn test_norm_dist_cdf_basic() {
        // =NORMDIST(1, 0, 1, TRUE) in US format
        // =NORMDIST(1; 0; 1; TRUE) in German format
        let result = codcel_norm_dist(1.0, 0.0, 1.0, true).unwrap();
        assert!((result - 0.8413447460685429).abs() < 1e-15);
    }

    #[test]
    fn test_norm_dist_at_mean() {
        // =NORMDIST(5, 5, 2, TRUE) in US format
        // =NORMDIST(5; 5; 2; TRUE) in German format
        let result = codcel_norm_dist(5.0, 5.0, 2.0, true).unwrap();
        assert_eq!(result, 0.5);
    }

    #[test]
    fn test_norm_dist_pdf_at_mean() {
        // =NORMDIST(5, 5, 2, FALSE) in US format
        // =NORMDIST(5; 5; 2; FALSE) in German format
        let result = codcel_norm_dist(5.0, 5.0, 2.0, false).unwrap();
        assert!((result - 0.19947114020071635).abs() < 1e-16);
    }

    #[test]
    fn test_norm_dist_different_std_dev() {
        // =NORMDIST(1, 0, 0.5, TRUE) in US format
        // =NORMDIST(1; 0; 0,5; TRUE) in German format
        let result = codcel_norm_dist(1.0, 0.0, 0.5, true).unwrap();
        assert!((result - 0.9772498680518208).abs() < 1e-15);
    }

    #[test]
    fn test_norm_dist_negative_x() {
        // =NORMDIST(-1, 0, 1, TRUE) in US format
        // =NORMDIST(-1; 0; 1; TRUE) in German format
        let result = codcel_norm_dist(-1.0, 0.0, 1.0, true).unwrap();
        assert!((result - 0.15865525393145705).abs() < 1e-16);
    }

    #[test]
    fn test_norm_dist_non_standard() {
        // =NORMDIST(3, 2, 1.5, TRUE) in US format
        // =NORMDIST(3; 2; 1,5; TRUE) in German format
        let result = codcel_norm_dist(3.0, 2.0, 1.5, true).unwrap();
        assert!((result - 0.7475074624530771).abs() < 1e-15);
        // =NORMDIST(3, 2, 1.5, FALSE)
        let result = codcel_norm_dist(3.0, 2.0, 1.5, false).unwrap();
        assert!((result - 0.21296533701490147).abs() < 1e-16);
    }

    #[test]
    fn test_norm_dist_far_left_tail() {
        // =NORMDIST(-10, 0, 1, TRUE) in US format
        // =NORMDIST(-10; 0; 1; TRUE) in German format
        //
        // The `0.5 * (1 + erf(z / sqrt(2)))` form this used to carry cancels to exactly 0.0 here.
        let result = codcel_norm_dist(-10.0, 0.0, 1.0, true).unwrap();
        let expected = 7.619853024160525e-24;
        assert!(
            ((result - expected) / expected).abs() < 1e-13,
            "got {result}"
        );
    }

    #[test]
    fn test_norm_dist_agrees_with_the_standard_normal() {
        // NORM.DIST(z, 0, 1, ..) and NORM.S.DIST(z, ..) are the same Excel function; they used to
        // disagree around the eighth decimal because each carried its own approximation.
        use crate::statistical::codcel_norm_dot_s_dot_dist::codcel_norm_dot_s_dot_dist;
        for z in [-5.0, -2.0, -1.0, -0.5, 0.0, 0.5, 1.0, 2.0, 5.0] {
            for cumulative in [true, false] {
                assert_eq!(
                    codcel_norm_dist(z, 0.0, 1.0, cumulative).unwrap(),
                    codcel_norm_dot_s_dot_dist(z, cumulative).unwrap(),
                    "disagreement at z = {z}, cumulative = {cumulative}"
                );
            }
        }
    }

    #[test]
    fn test_norm_dist_zero_std_dev() {
        // =NORMDIST(1, 0, 0, TRUE) in US format
        // =NORMDIST(1; 0; 0; TRUE) in German format
        let result = codcel_norm_dist(1.0, 0.0, 0.0, true);
        assert!(result.is_err());
    }
}
