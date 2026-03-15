// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

pub(crate) fn binomial_coefficient(n: u32, k: u32) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if k > n {
        return Err("BINOMIAL_COEFFICIENT: k cannot be greater than n.".into());
    }
    // Use symmetry to reduce computation: C(n,k) = C(n,n-k)
    let k = if k > n / 2 { n - k } else { k };
    if k == 0 {
        return Ok(1.0);
    }
    // Log-space computation to avoid integer overflow for large n
    let mut ln_result = 0.0;
    for i in (n - k + 1)..=n {
        ln_result += (i as f64).ln();
    }
    for i in 2..=k {
        ln_result -= (i as f64).ln();
    }
    Ok(ln_result.exp())
}
