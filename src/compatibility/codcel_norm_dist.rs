// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

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

    if cumulative {
        // Compute the cumulative distribution function (CDF)
        let z = (x - mean) / std_dev;
        Ok(0.5 * (1.0 + statrs::function::erf::erf(z / std::f64::consts::SQRT_2)))
    } else {
        // Compute the probability density function (PDF)
        let z = (x - mean) / std_dev;
        let pdf =
            (1.0 / (std_dev * crate::portable_math::sqrt(2.0 * std::f64::consts::PI))) * crate::portable_math::exp(-0.5 * z.powi(2));
        Ok(pdf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_norm_dist_pdf_basic() {
        // =NORMDIST(1, 0, 1, FALSE) in US format
        // =NORMDIST(1; 0; 1; FALSE) in German format
        let result = codcel_norm_dist(1.0, 0.0, 1.0, false).unwrap();
        println!("{result}");
        assert!((result - 0.2419707245).abs() < 0.0001);
    }

    #[test]
    fn test_norm_dist_cdf_basic() {
        // =NORMDIST(1, 0, 1, TRUE) in US format
        // =NORMDIST(1; 0; 1; TRUE) in German format
        let result = codcel_norm_dist(1.0, 0.0, 1.0, true).unwrap();
        println!("{result}");
        assert!((result - 0.8413447461).abs() < 0.0001);
    }

    #[test]
    fn test_norm_dist_at_mean() {
        // =NORMDIST(5, 5, 2, TRUE) in US format
        // =NORMDIST(5; 5; 2; TRUE) in German format
        let result = codcel_norm_dist(5.0, 5.0, 2.0, true).unwrap();
        println!("{result}");
        assert!((result - 0.5).abs() < 0.0001);
    }

    #[test]
    fn test_norm_dist_pdf_at_mean() {
        // =NORMDIST(5, 5, 2, FALSE) in US format
        // =NORMDIST(5; 5; 2; FALSE) in German format
        let result = codcel_norm_dist(5.0, 5.0, 2.0, false).unwrap();
        println!("{result}");
        assert!((result - 0.1994711402).abs() < 0.0001);
    }

    #[test]
    fn test_norm_dist_different_std_dev() {
        // =NORMDIST(1, 0, 0.5, TRUE) in US format
        // =NORMDIST(1; 0; 0,5; TRUE) in German format
        let result = codcel_norm_dist(1.0, 0.0, 0.5, true).unwrap();
        println!("{result}");
        assert!((result - 0.9772498681).abs() < 0.0001);
    }

    #[test]
    fn test_norm_dist_negative_x() {
        // =NORMDIST(-1, 0, 1, TRUE) in US format
        // =NORMDIST(-1; 0; 1; TRUE) in German format
        let result = codcel_norm_dist(-1.0, 0.0, 1.0, true).unwrap();
        println!("{result}");
        assert!((result - 0.1586552539).abs() < 0.0001);
    }

    #[test]
    fn test_norm_dist_zero_std_dev() {
        // =NORMDIST(1, 0, 0, TRUE) in US format
        // =NORMDIST(1; 0; 0; TRUE) in German format
        let result = codcel_norm_dist(1.0, 0.0, 0.0, true);
        assert!(result.is_err());
    }
}
