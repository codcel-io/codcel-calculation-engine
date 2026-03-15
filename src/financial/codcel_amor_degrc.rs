// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::financial::helpers::calculate_days_between;
use chrono::{DateTime, Utc};
use std::error::Error;

/// Excel-compatible rounding: pre-rounds to 13 decimal places to handle
/// floating-point precision issues, then rounds to nearest integer.
/// This matches the F# ExcelFinancialFunctions `round` behavior.
fn excel_round(x: f64) -> f64 {
    // Pre-round to 13 significant decimal places to fix floating-point drift
    // e.g., 247.4999999999999715... becomes 247.5 before final rounding
    let factor = 10_f64.powi(13);
    let pre_rounded = (x * factor).round() / factor;
    pre_rounded.round()
}

/// Calculates the depreciation for each accounting period using the French accounting system
///
/// Uses the accelerated declining balance method with depreciation coefficients.
/// Algorithm matches Excel's AMORDEGRC behavior including the 50%/100% rule
/// for the last two periods of asset life.
///
/// # Arguments
///
/// * `cost` - The cost of the asset
/// * `date_purchased` - The date the asset was purchased
/// * `first_period` - The date of the end of the first period
/// * `salvage_value` - The salvage value of the asset
/// * `period` - The period for which to calculate depreciation
/// * `rate` - The rate of depreciation
/// * `basis` - The day count basis to use (0-4)
pub fn codcel_amor_degrc(
    cost: f64,
    date_purchased: DateTime<Utc>,
    first_period: DateTime<Utc>,
    salvage_value: f64,
    period: i32,
    rate: f64,
    basis: Option<i32>,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    // Validate inputs
    if cost < 0.0 {
        return Err("AMORDEGRC: Cost cannot be negative".into());
    }
    if salvage_value < 0.0 {
        return Err("AMORDEGRC: Salvage value cannot be negative".into());
    }
    if period < 0 {
        return Err("AMORDEGRC: Period cannot be negative".into());
    }
    if rate <= 0.0 {
        return Err("AMORDEGRC: Rate must be positive".into());
    }
    if cost < salvage_value {
        return Err("AMORDEGRC: Cost must be greater than or equal to salvage value".into());
    }

    // Validate and set default basis
    let basis = basis.unwrap_or(0);
    if !(0..=4).contains(&basis) {
        return Err("AMORDEGRC: Basis must be between 0 and 4".into());
    }

    // Calculate the asset lifetime using ceiling (matches Excel)
    let ass_life = (1.0_f64 / rate).ceil();

    // Early exit: no depreciation if cost equals salvage or period is past asset life
    if (cost - salvage_value).abs() < f64::EPSILON {
        return Ok(0);
    }
    if period as f64 > ass_life {
        return Ok(0);
    }

    // Determine the depreciation coefficient based on ceiling'd asset life
    let depreciation_coefficient = if (3.0..=4.0).contains(&ass_life) {
        1.5
    } else if (5.0..=6.0).contains(&ass_life) {
        2.0
    } else if ass_life > 6.0 {
        2.5
    } else {
        1.0
    };

    let adjusted_rate = rate * depreciation_coefficient;

    // Calculate days in year based on basis
    let days_in_year: f64 = match basis {
        1 | 3 => 365.0,
        _ => 360.0, // 0, 2, 4 all use 360 days
    };

    // Calculate the first period depreciation
    let days_in_first_period = calculate_days_between(&date_purchased, &first_period, basis)?;
    let first_depr_temp = (days_in_first_period / days_in_year) * adjusted_rate * cost;

    // If first period has zero depreciation (purchase date == first period date),
    // use a full period's depreciation and don't extend asset life.
    // Otherwise, round immediately and extend asset life by 1 for the partial first period.
    let (first_depr, asset_life) = if first_depr_temp == 0.0 {
        (cost * adjusted_rate, ass_life)
    } else {
        (excel_round(first_depr_temp), ass_life + 1.0)
    };

    // Cap first depreciation at available depreciable amount
    let available_depr = cost - salvage_value;
    let first_depr = if first_depr > available_depr {
        available_depr
    } else {
        first_depr
    };

    // Period 0: return the first period depreciation
    if period == 0 {
        return Ok(first_depr as i32);
    }

    // Iterative computation for periods 1..=period
    let mut remain_cost = cost - first_depr; // Uses rounded first depreciation
    let mut depr_rate = adjusted_rate;
    let mut depr = 0.0_f64;

    for counted_period in 1..=period {
        let calc_t = asset_life - (counted_period as f64 + 1.0);

        // Second-to-last period: depreciate 50% of remaining cost
        // and set rate to 1.0 so the last period takes the other 50%
        let depr_temp = if (calc_t - 2.0).abs() < 0.0001 {
            depr_rate = 1.0;
            remain_cost * 0.5
        } else {
            depr_rate * remain_cost
        };

        depr = if remain_cost < salvage_value {
            if remain_cost - salvage_value < 0.0 {
                0.0
            } else {
                remain_cost - salvage_value
            }
        } else {
            depr_temp
        };

        remain_cost -= depr;
    }

    Ok(excel_round(depr) as i32)
}
