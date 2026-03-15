// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::date_time_base::{calculate_30_360_days, calculate_30e_360_days};
use crate::financial::helpers::{get_next_coupon_date_eom, get_previous_coupon_date};
use chrono::{DateTime, Utc};
use std::error::Error;

/// Calculates the number of days from the settlement date to the next coupon date.
///
/// # Arguments
/// * `settlement` - The settlement date.
/// * `maturity` - The maturity date of the bond.
/// * `frequency` - The number of coupon payments per year (1 for annual, 2 for semi-annual, 4 for quarterly).
/// * `basis` - The basis for day count calculation (0 = 30/360, 1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = 30E/360).
///
/// # Returns
/// The number of days from the settlement date to the next coupon date.
pub fn codcel_coup_days_nc(
    settlement: DateTime<Utc>,
    maturity: DateTime<Utc>,
    frequency: i32,
    basis: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Convert UTC DateTime to NaiveDate for calculations
    let settlement_date = settlement.date_naive();
    let maturity_date = maturity.date_naive();

    // Validate inputs
    if settlement_date >= maturity_date {
        return Err("COUPDAYSNC: Settlement date must be before maturity date".into());
    }

    if ![1, 2, 4].contains(&frequency) {
        return Err(
            "COUPDAYSNC: Frequency must be 1 (annual), 2 (semi-annual), or 4 (quarterly)".into(),
        );
    }

    let basis = basis.unwrap_or(0);
    if !(0..=4).contains(&basis) {
        return Err("COUPDAYSNC: Basis must be between 0 and 4".into());
    }

    // Find the previous and next coupon dates using shared helpers
    let previous_coupon = get_previous_coupon_date(settlement_date, maturity_date, frequency)?;
    let next_coupon = get_next_coupon_date_eom(settlement_date, maturity_date, frequency)?;

    // Calculate days based on the day count basis.
    // For basis 0 and 4, compute as complement (COUPDAYS - COUPDAYBS) to match
    // Excel's asymmetric 30/360 adjustment rules.
    match basis {
        0 => {
            // 30/360 US: complement method
            let coupdaybs = calculate_30_360_days(previous_coupon, settlement_date) as f64;
            Ok(360.0 / frequency as f64 - coupdaybs)
        }
        1..=3 => {
            // Actual day count using correctly-derived next coupon date
            Ok((next_coupon - settlement_date).num_days() as f64)
        }
        4 => {
            // European 30/360: complement method
            let coupdaybs = calculate_30e_360_days(previous_coupon, settlement_date) as f64;
            Ok(360.0 / frequency as f64 - coupdaybs)
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_coup_days_nc_basic() {
        let settlement = Utc.with_ymd_and_hms(2021, 2, 15, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2023, 5, 15, 0, 0, 0).unwrap();
        let frequency = 2; // Semi-annual

        // With basis 0 (30/360)
        let result = codcel_coup_days_nc(settlement, maturity, frequency, Some(0)).unwrap();

        // Next coupon date would be 2021-05-15
        // Days between 2021-02-15 and 2021-05-15 using 30/360 = 90 days
        assert_eq!(result, 90.0);
    }
}
