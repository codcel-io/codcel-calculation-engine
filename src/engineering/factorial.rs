// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Computes factorial of a number
pub(crate) fn factorial(n: u64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if n > 170 {
        return Err("Factorial would overflow f64".into());
    }

    let mut result: f64 = 1.0;
    for i in 2..=n {
        result *= i as f64;
    }
    Ok(result)
}
