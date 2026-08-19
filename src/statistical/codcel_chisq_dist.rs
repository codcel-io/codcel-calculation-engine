// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use statrs::distribution::{Continuous, ContinuousCDF};
use std::error::Error;

/// Excel-compatible `CHISQ.DIST` that returns the chi-squared distribution.
/// - `x`: the value at which to evaluate the distribution (must be >= 0).
/// - `df`: degrees of freedom (must be > 0).
/// - `cumulative`: if `true`, returns the cumulative distribution function (CDF);
///   if `false`, returns the probability density function (PDF).
///
/// Returns the distribution value or an error when inputs are outside the allowed range.
pub fn codcel_chisq_dist(
    x: f64,
    df: f64,
    cumulative: bool,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if x < 0.0 {
        return Err("CHISQ.DIST: x must be >= 0.".into());
    }
    if df <= 0.0 {
        return Err("CHISQ.DIST: degrees_of_freedom must be > 0.".into());
    }
    // The density diverges at the origin for fewer than two degrees of freedom, where Excel
    // gives #NUM! rather than an infinity.
    if !cumulative && x == 0.0 && df < 2.0 {
        return Err("CHISQ.DIST: density is undefined at x = 0 for df < 2.".into());
    }

    // Shared with CHISQ.DIST.RT, CHISQ.INV, CHISQ.INV.RT and CHISQ.TEST, so the five agree.
    let distribution = statrs::distribution::ChiSquared::new(df)
        .map_err(|_| "CHISQ.DIST: Error creating chi-squared distribution.")?;

    if cumulative {
        Ok(distribution.cdf(x))
    } else {
        Ok(distribution.pdf(x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chisq_dist_pdf() {
        // =CHISQ.DIST(2,3,FALSE) in US format
        // =CHISQ.DIST(2;3;FALSE) in German format
        let result = codcel_chisq_dist(2.0, 3.0, false).unwrap();
        assert!((result - 0.20755374871029736).abs() < 1e-15); // PDF of Chi-squared(3) at x=2
    }

    #[test]
    fn test_chisq_dist_cdf() {
        // =CHISQ.DIST(2,3,TRUE) in US format
        // =CHISQ.DIST(2;3;TRUE) in German format
        let result = codcel_chisq_dist(2.0, 3.0, true).unwrap();
        println!("{result:?}");
        assert!((result - 0.4275932955291202).abs() < 1e-15); // CDF of Chi-squared(3) at x=2
    }

    #[test]
    fn test_chisq_dist_zero_x() {
        // =CHISQ.DIST(0,3,TRUE) in US format
        // =CHISQ.DIST(0;3;TRUE) in German format
        let result = codcel_chisq_dist(0.0, 3.0, true).unwrap();
        assert_eq!(result, 0.0); // CDF of Chi-squared(3) at x=0 is 0
    }

    #[test]
    fn test_chisq_dist_large_x() {
        // =CHISQ.DIST(10,3,TRUE) in US format
        // =CHISQ.DIST(10;3;TRUE) in German format
        let result = codcel_chisq_dist(10.0, 3.0, true).unwrap();
        println!("{result:?}");
        assert!((result - 0.9814338645369568).abs() < 1e-15); // CDF of Chi-squared(3) at x=10
    }

    #[test]
    fn test_chisq_dist_low_df() {
        // =CHISQ.DIST(2,1,TRUE) in US format
        // =CHISQ.DIST(2;1;TRUE) in German format
        let result = codcel_chisq_dist(2.0, 1.0, true).unwrap();
        assert!((result - 0.8427007929497149).abs() < 1e-15); // CDF of Chi-squared(1) at x=2
    }

    #[test]
    fn test_chisq_dist_high_df() {
        // =CHISQ.DIST(20,10,TRUE) in US format
        // =CHISQ.DIST(20;10;TRUE) in German format
        let result = codcel_chisq_dist(20.0, 10.0, true).unwrap();
        assert!((result - 0.970747311923039).abs() < 1e-15); // CDF of Chi-squared(10) at x=20
    }

    #[test]
    fn test_chisq_dist_pdf_zero_x() {
        // =CHISQ.DIST(0,3,FALSE) in US format
        // =CHISQ.DIST(0;3;FALSE) in German format
        let result = codcel_chisq_dist(0.0, 3.0, false).unwrap();
        assert_eq!(result, 0.0); // PDF of Chi-squared(3) at x=0 is 0
    }

    #[test]
    fn test_chisq_dist_pdf_large_x() {
        // =CHISQ.DIST(20,3,FALSE) in US format
        // =CHISQ.DIST(20;3;FALSE) in German format
        let result = codcel_chisq_dist(20.0, 3.0, false).unwrap();
        println!("{result:?}");
        assert!((result - 8.099910956089118e-5).abs() < 1e-18); // PDF of Chi-squared(3) at x=20
    }

    #[test]
    fn test_chisq_dist_invalid_x() {
        // =CHISQ.DIST(-1,3,TRUE) in US format (returns #NUM! error)
        // =CHISQ.DIST(-1;3;TRUE) in German format (returns #NUM! error)
        let result = codcel_chisq_dist(-1.0, 3.0, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_chisq_dist_invalid_df_zero() {
        // =CHISQ.DIST(2,0,TRUE) in US format (returns #NUM! error)
        // =CHISQ.DIST(2;0;TRUE) in German format (returns #NUM! error)
        let result = codcel_chisq_dist(2.0, 0.0, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_chisq_dist_invalid_df_negative() {
        // =CHISQ.DIST(2,-1,TRUE) in US format (returns #NUM! error)
        // =CHISQ.DIST(2;-1;TRUE) in German format (returns #NUM! error)
        let result = codcel_chisq_dist(2.0, -1.0, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_chisq_dist_density_undefined_at_origin_for_low_df() {
        // =CHISQ.DIST(0,1,FALSE) in US format
        // =CHISQ.DIST(0;1;FALSE) in German format
        // The density diverges here; Excel gives #NUM!.
        assert!(codcel_chisq_dist(0.0, 1.0, false).is_err());
        // Two degrees of freedom is the boundary case, where the density is finite.
        assert!((codcel_chisq_dist(0.0, 2.0, false).unwrap() - 0.5).abs() < 1e-16);
    }

    #[test]
    fn test_chisq_dist_complements_chisq_dist_rt() {
        // CHISQ.DIST(x, df, TRUE) and CHISQ.DIST.RT(x, df) are the two tails of one distribution.
        // They used to be computed by unrelated code: a bespoke incomplete gamma here and statrs
        // there.
        use crate::statistical::codcel_chisq_dist_rt::codcel_chisq_dist_rt;
        for df in [1.0, 2.0, 3.0, 10.0, 50.0] {
            for x in [0.5, 2.0, 10.0, 20.0] {
                let left = codcel_chisq_dist(x, df, true).unwrap();
                let right = codcel_chisq_dist_rt(x, df).unwrap();
                assert!(
                    (left + right - 1.0).abs() < 1e-14,
                    "tails of ({x}, {df}) sum to {}",
                    left + right
                );
            }
        }
    }

    #[test]
    fn test_chisq_dist_inverts_chisq_inv() {
        use crate::statistical::codcel_chisq_inv::codcel_chisq_inv;
        for df in [1.0, 3.0, 10.0] {
            for p in [0.01, 0.25, 0.5, 0.75, 0.99] {
                let x = codcel_chisq_inv(p, df).unwrap();
                let round_tripped = codcel_chisq_dist(x, df, true).unwrap();
                assert!(
                    (round_tripped - p).abs() < 1e-12,
                    "round trip of p = {p} at df = {df} gave {round_tripped}"
                );
            }
        }
    }
}
