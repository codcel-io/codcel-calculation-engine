// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::cmp::Ordering;
use std::error::Error;

/// Excel-compatible `RANDARRAY` that returns an array of random numbers.
/// - `rows`: the number of rows to return.
/// - `columns`: the number of columns to return.
/// - `min`: optional minimum value (defaults to 0).
/// - `max`: optional maximum value (defaults to 1).
/// - `whole_number`: optional flag for integer output (defaults to false).
///
/// Returns a 2D array of random values or an error for invalid inputs.
pub fn codcel_rand_array(
    rows: i32,
    columns: i32,
    min: Option<f64>,
    max: Option<f64>,
    whole_number: Option<bool>,
) -> Result<Vec<Vec<f64>>, Box<dyn Error + Send + Sync>> {
    // Validate input parameters
    if rows <= 0 || columns <= 0 {
        return Err("RANDARRAY: Rows and columns must be positive values".into());
    }

    // Set default values if not provided
    let min_val = min.unwrap_or(0.0);
    let max_val = max.unwrap_or(1.0);
    let whole = whole_number.unwrap_or(false);

    if min_val > max_val {
        return Err("RANDARRAY: Minimum value cannot be greater than maximum value".into());
    }

    let mut result: Vec<Vec<f64>> = Vec::with_capacity(rows as usize);

    for _ in 0..rows {
        let mut row: Vec<f64> = Vec::with_capacity(columns as usize);

        for _ in 0..columns {
            let random_val = if whole {
                // Generate whole numbers
                let min_int = min_val.ceil() as i64;
                let max_int = max_val.floor() as i64;

                // Handle case where min and max are equal after rounding
                match min_int.cmp(&max_int) {
                    Ordering::Equal => min_int as f64,
                    Ordering::Greater => {
                        // This can happen if min=5.9, max=6.1 and whole=true
                        max_int as f64
                    }
                    Ordering::Less => fastrand::i64(min_int..=max_int) as f64,
                }
            } else {
                // Generate floating-point numbers between min_val and max_val
                min_val + fastrand::f64() * (max_val - min_val)
            };

            row.push(random_val);
        }

        result.push(row);
    }

    Ok(result)
}
