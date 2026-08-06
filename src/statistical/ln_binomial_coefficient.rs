// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Calculates the natural logarithm of the binomial coefficient C(n,k).
/// Uses the logarithm of factorial to avoid overflow with large numbers.
pub(crate) fn ln_binomial_coefficient(n: i32, k: i32) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if k < 0 || k > n {
        return Err("Invalid binomial coefficient parameters".into());
    }

    // Use symmetry to reduce computation: C(n,k) = C(n,n-k)
    let k = if k > n / 2 { n - k } else { k };

    // ln(C(n,k)) = ln(n!) - ln(k!) - ln((n-k)!)
    // To avoid overflow with large numbers, calculate log factorials
    let mut ln_result = 0.0;

    // Calculate ln(n! / (n-k)!) = ln(n) + ln(n-1) + ... + ln(n-k+1)
    for i in (n - k + 1)..=n {
        ln_result += crate::portable_math::ln(i as f64);
    }

    // Subtract ln(k!)
    for i in 2..=k {
        ln_result -= crate::portable_math::ln(i as f64);
    }

    Ok(ln_result)
}
