// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::financial::helpers::get_previous_coupon_date;
use chrono::{DateTime, Utc};
use std::error::Error;

/// Calculates the previous coupon date before the settlement date.
///
/// # Arguments
/// * `settlement` - The settlement date.
/// * `maturity` - The maturity date of the bond.
/// * `frequency` - The number of coupon payments per year (1 for annual, 2 for semi-annual, 4 for quarterly).
/// * `basis` - The basis for day count calculation (0 = 30/360, 1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = 30E/360).
///
/// # Returns
/// The previous coupon date before the settlement date.
pub fn codcel_coup_pcd(
    settlement: DateTime<Utc>,
    maturity: DateTime<Utc>,
    frequency: i32,
    basis: Option<i32>,
) -> Result<DateTime<Utc>, Box<dyn Error + Send + Sync>> {
    // Convert UTC DateTime to NaiveDate for calculations
    let settlement_date = settlement.date_naive();
    let maturity_date = maturity.date_naive();

    // Validate inputs
    if settlement_date >= maturity_date {
        return Err("COUPPCD: Settlement date must be before maturity date".into());
    }

    if ![1, 2, 4].contains(&frequency) {
        return Err(
            "COUPPCD: Frequency must be 1 (annual), 2 (semi-annual), or 4 (quarterly)".into(),
        );
    }

    let basis = basis.unwrap_or(0);
    if !(0..=4).contains(&basis) {
        return Err("COUPPCD: Basis must be between 0 and 4".into());
    }

    // Find the previous coupon date
    let prev_coupon = get_previous_coupon_date(settlement_date, maturity_date, frequency)?;

    // Convert back to DateTime<Utc>
    let prev_coupon_time = prev_coupon
        .and_hms_opt(0, 0, 0)
        .ok_or("COUPPCD: Invalid time conversion")?;
    Ok(DateTime::<Utc>::from_naive_utc_and_offset(
        prev_coupon_time,
        Utc,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_coup_pcd_basic() {
        let settlement = Utc.with_ymd_and_hms(2021, 2, 15, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2023, 5, 15, 0, 0, 0).unwrap();
        let frequency = 2; // Semi-annual

        // With basis 0 (30/360)
        let result = codcel_coup_pcd(settlement, maturity, frequency, Some(0)).unwrap();

        // Previous coupon date would be 2020-11-15
        let expected = Utc.with_ymd_and_hms(2020, 11, 15, 0, 0, 0).unwrap();
        assert_eq!(result, expected);
    }
}
