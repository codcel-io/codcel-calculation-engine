// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `RAND` that returns a random number between 0 and 1.
///
/// Returns a pseudo-random number greater than or equal to 0 and less than 1.
pub fn codcel_rand() -> Result<f64, Box<dyn Error + Send + Sync>> {
    Ok(fastrand::f64())
}
