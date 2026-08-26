// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::date_time_base::{calculate_30_360_days, calculate_30e_360_days};
use crate::financial::helpers::get_previous_coupon_date;
use chrono::{DateTime, Utc};
use std::error::Error;

/// Calculates the number of days from the beginning of the coupon period to the settlement date.
///
/// # Arguments
/// * `settlement` - The settlement date.
/// * `maturity` - The maturity date of the bond.
/// * `frequency` - The number of coupon payments per year (1 for annual, 2 for semi-annual, 4 for quarterly).
/// * `basis` - The basis for day count calculation (0 = 30/360, 1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = 30E/360).
///
/// # Returns
/// The number of days from the beginning of the coupon period to the settlement date.
pub fn codcel_coup_day_bs(
    settlement: DateTime<Utc>,
    maturity: DateTime<Utc>,
    frequency: i32,
    basis: Option<i32>,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    // Convert UTC DateTime to NaiveDate for calculations
    let settlement_date = settlement.date_naive();
    let maturity_date = maturity.date_naive();

    // Validate inputs
    if settlement_date >= maturity_date {
        return Err("COUPDAYBS: Settlement date must be before maturity date".into());
    }

    if ![1, 2, 4].contains(&frequency) {
        return Err(
            "COUPDAYBS: Frequency must be 1 (annual), 2 (semi-annual), or 4 (quarterly)".into(),
        );
    }

    let basis = basis.unwrap_or(0);
    if !(0..=4).contains(&basis) {
        return Err("COUPDAYBS: Basis must be between 0 and 4".into());
    }

    // Find the previous coupon date before settlement
    let previous_coupon = get_previous_coupon_date(settlement_date, maturity_date, frequency)?;

    // Calculate days based on the specified day count basis
    let days = match basis {
        0 => {
            // 30/360
            calculate_30_360_days(previous_coupon, settlement_date)
        }
        1 => {
            // Actual/actual
            (settlement_date - previous_coupon).num_days() as i32
        }
        2 => {
            // Actual/360
            (settlement_date - previous_coupon).num_days() as i32
        }
        3 => {
            // Actual/365
            (settlement_date - previous_coupon).num_days() as i32
        }
        4 => {
            // European 30/360
            calculate_30e_360_days(previous_coupon, settlement_date)
        }
        _ => unreachable!(),
    };

    Ok(days)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_coup_day_bs_basic() {
        let settlement = Utc
            .with_ymd_and_hms(2021, 2, 15, 0, 0, 0)
            .single()
            .expect("valid test date");
        let maturity = Utc
            .with_ymd_and_hms(2023, 5, 15, 0, 0, 0)
            .single()
            .expect("valid test date");
        let frequency = 2; // Semi-annual

        // With basis 0 (30/360)
        let result = codcel_coup_day_bs(settlement, maturity, frequency, Some(0)).unwrap();

        // Previous coupon date would be 2020-11-15
        // Days between 2020-11-15 and 2021-02-15 using 30/360 = 90 days
        assert_eq!(result, 90);
    }
}
