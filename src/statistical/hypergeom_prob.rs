// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

pub(crate) fn hypergeom_prob(
    x: u64,
    k: u64,
    m: u64,
    n: u64,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Calculate binomial coefficient using multiplicative formula
    fn binom(n: u64, k: u64) -> f64 {
        if k > n {
            return 0.0;
        }
        if k == 0 || k == n {
            return 1.0;
        }

        let k = k.min(n - k); // Take advantage of symmetry
        let mut c = 1.0;

        for i in 0..k {
            c = c * (n - i) as f64 / (i + 1) as f64;
        }

        c
    }

    // Calculate hypergeometric probability: C(m, x) * C(k-m, n-x) / C(k, n)
    // where m = population successes, k = population size, n = sample size, x = sample successes
    let numerator = binom(m, x) * binom(k - m, n - x);
    let denominator = binom(k, n);

    if denominator == 0.0 {
        return Err("HYPGEOM.DIST: Invalid parameters resulting in zero denominator.".into());
    }

    let prob = numerator / denominator;

    if prob.is_nan() || prob.is_infinite() || !(0.0..=1.0).contains(&prob) {
        return Err("HYPGEOM.DIST: Invalid probability calculated.".into());
    }

    Ok(prob)
}
