// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Calculates the depreciation of an asset using the declining balance method.
///
/// # Arguments
/// * `cost` - The initial cost of the asset.
/// * `salvage` - The salvage value of the asset at the end of its useful life.
/// * `life` - The useful life of the asset in years.
/// * `period` - The period for which to calculate the depreciation.
/// * `month` - Optional. The number of months in the first year. If omitted, 12 is used.
///
/// # Returns
/// The depreciation of the asset for the specified period.
pub fn codcel_db(
    cost: f64,
    salvage: f64,
    life: f64,
    period: i32,
    month: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let m = month.unwrap_or(12);

    if cost <= 0.0 || salvage < 0.0 || life <= 0.0 || period < 1 {
        return Err("DB: Invalid input parameters".into());
    }
    if !(1..=12).contains(&m) {
        return Err("DB: Month must be between 1 and 12".into());
    }

    let max_period = if m < 12 { life as i32 + 1 } else { life as i32 };
    if period > max_period {
        return Err("DB: Invalid input parameters".into());
    }

    let rate =
        ((1.0 - crate::portable_math::powf(salvage / cost, 1.0 / life)) * 1000.0).round() / 1000.0;

    let mut book_value = cost;
    for p in 1..=period {
        let dep = if p == 1 {
            cost * rate * m as f64 / 12.0
        } else if p <= life as i32 {
            book_value * rate
        } else {
            book_value * rate * (12 - m) as f64 / 12.0
        };

        if p == period {
            return Ok(dep);
        }
        book_value -= dep;
    }

    Ok(0.0)
}
