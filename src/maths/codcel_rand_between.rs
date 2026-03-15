// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `RANDBETWEEN` that returns a random integer between two values.
/// - `min`: the smallest integer to return.
/// - `max`: the largest integer to return.
///
/// Returns a random integer in [min, max] or an error when min > max.
pub fn codcel_rand_between(min: i32, max: i32) -> Result<i32, Box<dyn Error + Send + Sync>> {
    if min > max {
        return Err("RANDBETWEEN: Minimum value cannot be greater than maximum value".into());
    }

    Ok(fastrand::i32(min..=max))
}
