// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `AVERAGEA` that calculates the average of values, including text and logical values.
/// - `values`: a vector of numeric values to average (text as 0, TRUE as 1, FALSE as 0).
///
/// Returns the arithmetic mean of all values, or 0.0 if the input is empty.
pub fn codcel_average_a(values: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Ok(0.0);
    }

    let count = values.len() as f64;
    let sum: f64 = values.iter().sum();

    Ok(sum / count)
}
