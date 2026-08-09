// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use libm::{lgamma, tgamma};
use std::error::Error;

/// Regularized lower incomplete gamma function using series expansion
fn regularized_gamma_lower(a: f64, x: f64) -> f64 {
    const EPS: f64 = 1e-14;
    const MAX_ITER: usize = 1000;

    if x == 0.0 {
        return 0.0;
    }

    if x < a + 1.0 {
        // Series expansion
        let mut sum = 1.0 / a;
        let mut term = sum;
        for n in 1..MAX_ITER {
            term *= x / (a + n as f64);
            sum += term;
            if term.abs() < EPS * sum {
                break;
            }
        }

        sum * crate::portable_math::exp(-x) * crate::portable_math::powf(x, a) / tgamma(a)
    } else {
        // Continued fraction
        let mut b = x + 1.0 - a;
        let mut c = 1.0 / f64::MIN_POSITIVE;
        let mut d = 1.0 / b;
        let mut h = d;
        for i in 1..MAX_ITER {
            let an = -(i as f64) * (i as f64 - a);
            b += 2.0;
            d = an * d + b;
            if d.abs() < f64::MIN_POSITIVE {
                d = f64::MIN_POSITIVE;
            }
            c = b + an / c;
            if c.abs() < f64::MIN_POSITIVE {
                c = f64::MIN_POSITIVE;
            }
            d = 1.0 / d;
            let delta = d * c;
            h *= delta;
            if (delta - 1.0).abs() < EPS {
                break;
            }
        }
        1.0 - crate::portable_math::exp(-x + a * crate::portable_math::ln(x) - lgamma(a)) * h
    }
}

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

    let a = df / 2.0;
    let x_scaled = x / 2.0;

    if cumulative {
        Ok(regularized_gamma_lower(a, x_scaled))
    } else {
        let numerator =
            crate::portable_math::powf(x, a - 1.0) * crate::portable_math::exp(-x_scaled);
        let denominator = crate::portable_math::powf(2f64, a) * tgamma(a);
        Ok(numerator / denominator)
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
        assert!((result - 0.2075537487).abs() < 1e-10); // PDF of Chi-squared(3) at x=2
    }

    #[test]
    fn test_chisq_dist_cdf() {
        // =CHISQ.DIST(2,3,TRUE) in US format
        // =CHISQ.DIST(2;3;TRUE) in German format
        let result = codcel_chisq_dist(2.0, 3.0, true).unwrap();
        println!("{result:?}");
        assert!((result - 0.42759329552911934).abs() < 1e-10); // CDF of Chi-squared(3) at x=2
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
        assert!((result - 0.9814338645369567).abs() < 1e-10); // CDF of Chi-squared(3) at x=10
    }

    #[test]
    fn test_chisq_dist_low_df() {
        // =CHISQ.DIST(2,1,TRUE) in US format
        // =CHISQ.DIST(2;1;TRUE) in German format
        let result = codcel_chisq_dist(2.0, 1.0, true).unwrap();
        assert!((result - 0.8427007929).abs() < 1e-10); // CDF of Chi-squared(1) at x=2
    }

    #[test]
    fn test_chisq_dist_high_df() {
        // =CHISQ.DIST(20,10,TRUE) in US format
        // =CHISQ.DIST(20;10;TRUE) in German format
        let result = codcel_chisq_dist(20.0, 10.0, true).unwrap();
        assert!((result - 0.970747312).abs() < 1e-10); // CDF of Chi-squared(10) at x=20
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
        assert!((result - 8.099910956089115e-5).abs() < 1e-10); // PDF of Chi-squared(3) at x=20
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
}
