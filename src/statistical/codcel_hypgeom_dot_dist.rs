// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::statistical::hypergeom_prob::hypergeom_prob;
use std::error::Error;

/// Excel-compatible `HYPGEOM.DIST` that returns the hypergeometric distribution.
/// - `x`: the number of successes in the sample.
/// - `n`: the sample size.
/// - `m`: the number of successes in the population.
/// - `k`: the population size.
/// - `cumulative`: if `true`, returns the cumulative distribution function;
///   if `false`, returns the probability mass function.
///
/// Returns the probability or an error when inputs are outside the allowed range.
pub fn codcel_hypgeom_dot_dist(
    x: f64,
    n: f64,
    m: f64,
    k: f64,
    cumulative: bool,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Truncate to integers (Excel behavior)
    let x = x.floor();
    let n = n.floor();
    let m = m.floor();
    let k = k.floor();

    if k <= 0.0 {
        return Err("HYPGEOM.DIST: k (population size) must be greater than 0.".into());
    }
    if x < 0.0 {
        return Err("HYPGEOM.DIST: x must be non-negative.".into());
    }
    if n < 0.0 || n > k {
        return Err(
            "HYPGEOM.DIST: n (sample size) must be between 0 and k (population size).".into(),
        );
    }
    if m < 0.0 || m > k {
        return Err(
            "HYPGEOM.DIST: m (population successes) must be between 0 and k (population size)."
                .into(),
        );
    }
    if x > n.min(m) {
        return Err("HYPGEOM.DIST: x must be less than or equal to min(n, m).".into());
    }
    let lower_bound = (n + m - k).max(0.0);
    if x < lower_bound {
        return Err("HYPGEOM.DIST: x is too small given n, m, and k.".into());
    }

    let x_int = x as u64;
    let n_int = n as u64;
    let m_int = m as u64;
    let k_int = k as u64;

    if cumulative {
        let min_x = lower_bound as u64;
        let mut cumulative_prob = 0.0;
        for i in min_x..=x_int {
            match hypergeom_prob(i, k_int, m_int, n_int) {
                Ok(prob) => cumulative_prob += prob,
                Err(e) => return Err(e),
            }
        }
        Ok(cumulative_prob)
    } else {
        hypergeom_prob(x_int, k_int, m_int, n_int)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hypgeom_dot_dist_pmf_basic() {
        // =HYPGEOM.DIST(1, 4, 8, 20, FALSE) = C(8,1)*C(12,3)/C(20,4)
        let result = codcel_hypgeom_dot_dist(1.0, 4.0, 8.0, 20.0, false).unwrap();
        assert!((result - 0.36326109391124872).abs() < 1e-10);
    }

    #[test]
    fn test_hypgeom_dot_dist_cdf_basic() {
        // =HYPGEOM.DIST(1, 4, 8, 20, TRUE)
        let result = codcel_hypgeom_dot_dist(1.0, 4.0, 8.0, 20.0, true).unwrap();
        assert!((result - 0.46542827657378744).abs() < 1e-10);
    }

    #[test]
    fn test_hypgeom_dot_dist_pmf_zero_successes() {
        // =HYPGEOM.DIST(0, 4, 5, 20, FALSE)
        let result = codcel_hypgeom_dot_dist(0.0, 4.0, 5.0, 20.0, false).unwrap();
        assert!((result - 0.28173374613003094).abs() < 1e-10);
    }

    #[test]
    fn test_hypgeom_dot_dist_sample_larger_than_population() {
        // n=10 > k=5 → #NUM! in Excel
        let result = codcel_hypgeom_dot_dist(2.0, 10.0, 50.0, 5.0, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_hypgeom_dot_dist_negative_x() {
        let result = codcel_hypgeom_dot_dist(-1.0, 10.0, 50.0, 5.0, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_hypgeom_dot_dist_n_greater_than_k() {
        // n=60 > k=5 → sample larger than population
        let result = codcel_hypgeom_dot_dist(2.0, 60.0, 50.0, 5.0, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_hypgeom_dot_dist_x_greater_than_n() {
        // x=15 > n=10 → more successes than sample size
        let result = codcel_hypgeom_dot_dist(15.0, 10.0, 50.0, 20.0, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_hypgeom_dot_dist_m_zero() {
        // m=0 (no successes in population), x must be 0
        let result = codcel_hypgeom_dot_dist(0.0, 5.0, 0.0, 20.0, false).unwrap();
        assert!((result - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_hypgeom_dot_dist_m_greater_than_k() {
        // m=50 > k=20 → more successes than population
        let result = codcel_hypgeom_dot_dist(2.0, 5.0, 50.0, 20.0, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_hypgeom_dot_dist_truncation() {
        // Excel truncates: HYPGEOM.DIST(1.9, 4.7, 8.3, 20.1, FALSE) → (1, 4, 8, 20)
        let result = codcel_hypgeom_dot_dist(1.9, 4.7, 8.3, 20.1, false).unwrap();
        let expected = codcel_hypgeom_dot_dist(1.0, 4.0, 8.0, 20.0, false).unwrap();
        assert!((result - expected).abs() < 1e-10);
    }
}
