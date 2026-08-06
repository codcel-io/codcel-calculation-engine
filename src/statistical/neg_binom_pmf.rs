// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::compatibility::binomial_coefficient::binomial_coefficient;
use std::error::Error;

/// Helper function to compute the probability mass function (PMF) of the negative binomial distribution.
pub(crate) fn neg_binom_pmf(
    failures: u32,
    successes: u32,
    probability: f64,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Calculate combinations: (failures + successes - 1) choose (successes - 1)
    let combinations = binomial_coefficient(failures + successes - 1, successes - 1)?;

    // Compute the probability: P(X = failures)
    let prob = combinations
        * probability.powi(successes as i32)
        * (1.0 - probability).powi(failures as i32);

    Ok(prob)
}
