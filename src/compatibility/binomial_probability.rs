// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::compatibility::binomial_coefficient::binomial_coefficient;
use std::error::Error;

pub(crate) fn binomial_probability(
    trials: u32,
    successes: u32,
    probability: f64,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let combinations = binomial_coefficient(trials, successes)?;
    let success_prob = probability.powi(successes as i32);
    let failure_prob = (1.0 - probability).powi((trials - successes) as i32);
    Ok(combinations * success_prob * failure_prob)
}
