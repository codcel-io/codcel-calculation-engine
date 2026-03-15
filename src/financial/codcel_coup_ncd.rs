// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::financial::helpers::get_next_coupon_date_eom;
use chrono::{DateTime, Utc};
use std::error::Error;

/// Calculates the next coupon date after the settlement date.
///
/// # Arguments
/// * `settlement` - The settlement date.
/// * `maturity` - The maturity date of the bond.
/// * `frequency` - The number of coupon payments per year (1 for annual, 2 for semi-annual, 4 for quarterly).
/// * `basis` - The basis for day count calculation (0 = 30/360, 1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = 30E/360).
///
/// # Returns
/// The next coupon date after the settlement date.
pub fn codcel_coup_ncd(
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
        return Err("COUPNCD: Settlement date must be before maturity date".into());
    }

    if ![1, 2, 4].contains(&frequency) {
        return Err(
            "COUPNCD: Frequency must be 1 (annual), 2 (semi-annual), or 4 (quarterly)".into(),
        );
    }

    let basis = basis.unwrap_or(0);
    if !(0..=4).contains(&basis) {
        return Err("COUPNCD: Basis must be between 0 and 4".into());
    }

    // Find the next coupon date after settlement using EOM-aware helper
    let next_coupon = get_next_coupon_date_eom(settlement_date, maturity_date, frequency)?;

    // Convert back to UTC DateTime
    let next_coupon_time = next_coupon
        .and_hms_opt(0, 0, 0)
        .ok_or("COUPNCD: Invalid time conversion")?;
    Ok(DateTime::<Utc>::from_naive_utc_and_offset(
        next_coupon_time,
        Utc,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_coup_ncd_basic() {
        let settlement = Utc.with_ymd_and_hms(2021, 2, 15, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2023, 5, 15, 0, 0, 0).unwrap();
        let frequency = 2; // Semi-annual

        // With basis 0 (30/360)
        let result = codcel_coup_ncd(settlement, maturity, frequency, Some(0)).unwrap();

        // Next coupon date would be 2021-05-15
        let expected = Utc.with_ymd_and_hms(2021, 5, 15, 0, 0, 0).unwrap();
        assert_eq!(result, expected);
    }
}
