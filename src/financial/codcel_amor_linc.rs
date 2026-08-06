// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::date_time_base::{calculate_360_days, calculate_actual_days, is_leap_year};
use chrono::{DateTime, Datelike, Utc};
use std::error::Error;

/// Calculates the depreciation for each accounting period using the linear depreciation method
///
/// # Arguments
///
/// * `cost` - The cost of the asset
/// * `date_purchased` - The date the asset was purchased
/// * `first_period_end` - The date of the end of the first period
/// * `salvage` - The salvage value of the asset
/// * `period` - The period for which to calculate depreciation
/// * `rate` - The rate of depreciation
/// * `basis` - The day count basis to use (0-4)
#[allow(clippy::too_many_arguments)]
pub fn codcel_amor_linc(
    cost: f64,
    date_purchased: DateTime<Utc>,
    first_period_end: DateTime<Utc>,
    salvage: f64,
    period: i32,
    rate: f64,
    basis: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Validate inputs
    if cost <= 0.0 {
        return Err("AMORLINC: Cost must be positive".into());
    }
    if salvage < 0.0 {
        return Err("AMORLINC: Salvage value cannot be negative".into());
    }
    if rate <= 0.0 {
        return Err("AMORLINC: Rate must be positive".into());
    }
    if period < 0 {
        return Err("AMORLINC: Period must be non-negative".into());
    }
    if date_purchased > first_period_end {
        return Err(
            "AMORLINC: Purchase date must be before or equal to first period end date".into(),
        );
    }

    // Handle basis (default is 0 for 30/360 basis)
    let basis = basis.unwrap_or(0);
    if !matches!(basis, 0..=4) {
        return Err("AMORLINC: Invalid basis. Must be 0, 1, 2, 3, or 4".into());
    }

    // Calculate days in first period
    let days_in_first_period = match basis {
        0 => calculate_360_days(date_purchased, first_period_end),
        1 => (first_period_end - date_purchased).num_days() as i32,
        3 => 365, // Always use 365 days
        4 => 360, // Always use 360 days
        _ => calculate_actual_days(date_purchased, first_period_end),
    };

    // Calculate days in year based on basis
    let days_in_year = match basis {
        0 | 4 => 360,
        1 => {
            if is_leap_year(first_period_end.year()) {
                366
            } else {
                365
            }
        }
        2 => 360,
        3 => 365,
        _ => unreachable!(),
    };

    // Calculate depreciation
    let total_depreciation = cost - salvage;
    let life_in_years = (1.0 / rate) as i32;

    if period == 0 {
        // First period depreciation (partial year)
        let first_year_depreciation =
            total_depreciation * rate * (days_in_first_period as f64 / days_in_year as f64);
        Ok(first_year_depreciation)
    } else if period > life_in_years {
        // No depreciation after end of life
        Ok(0.0)
    } else {
        // Regular period depreciation
        // Fixed: Use cost instead of total_depreciation when calculating the annual depreciation
        let annual_depreciation = cost * rate;
        Ok(annual_depreciation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_amor_linc_first_period() {
        let cost = 2400.0;
        let date_purchased = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
        let first_period_end = Utc.with_ymd_and_hms(2020, 12, 31, 0, 0, 0).unwrap();
        let salvage = 300.0;
        let rate = 0.2;

        // Period 0 (first period)
        let result = codcel_amor_linc(
            cost,
            date_purchased,
            first_period_end,
            salvage,
            0,
            rate,
            Some(0),
        )
        .unwrap();

        // Total depreciation: 2400 - 300 = 2100
        // First period depreciation: 2100 * 0.2 * (360/360) = 420
        assert!((result - 420.0).abs() < 0.01);
    }

    #[test]
    fn test_amor_linc_regular_period() {
        let cost = 2400.0;
        let date_purchased = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
        let first_period_end = Utc.with_ymd_and_hms(2020, 12, 31, 0, 0, 0).unwrap();
        let salvage = 300.0;
        let rate = 0.2;

        // Period 1 (second period)
        let result = codcel_amor_linc(
            cost,
            date_purchased,
            first_period_end,
            salvage,
            1,
            rate,
            Some(0),
        )
        .unwrap();

        // Regular period depreciation: 2400 * 0.2 = 480
        assert!((result - 480.0).abs() < 0.01);
    }

    #[test]
    fn test_amor_linc_after_life() {
        let cost = 2400.0;
        let date_purchased = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
        let first_period_end = Utc.with_ymd_and_hms(2020, 12, 31, 0, 0, 0).unwrap();
        let salvage = 300.0;
        let rate = 0.2;

        // Life in years: 1/0.2 = 5
        // Period 6 (after life)
        let result = codcel_amor_linc(
            cost,
            date_purchased,
            first_period_end,
            salvage,
            6,
            rate,
            Some(0),
        )
        .unwrap();

        // No depreciation after end of life
        assert!((result - 0.0).abs() < 0.01);
    }
}
