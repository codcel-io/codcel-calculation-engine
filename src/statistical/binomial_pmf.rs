// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::statistical::ln_binomial_coefficient::ln_binomial_coefficient;
use std::error::Error;

// Calculates the binomial probability mass function.
pub(crate) fn binomial_pmf(
    successes: i32,
    trials: i32,
    probability: f64,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if successes < 0 || successes > trials {
        return Ok(0.0);
    }

    // Handle edge cases explicitly
    if probability == 0.0 {
        return Ok(if successes == 0 { 1.0 } else { 0.0 });
    }
    if probability == 1.0 {
        return Ok(if successes == trials { 1.0 } else { 0.0 });
    }

    // Log-domain computation for numerical stability
    let ln_coef = ln_binomial_coefficient(trials, successes)?;
    let ln_prob = (successes as f64) * crate::portable_math::ln(probability)
        + ((trials - successes) as f64) * crate::portable_math::ln(1.0 - probability);

    Ok(crate::portable_math::exp(ln_coef + ln_prob))
}
