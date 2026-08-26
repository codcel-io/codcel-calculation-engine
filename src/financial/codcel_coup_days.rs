// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use chrono::{DateTime, Utc};
use std::error::Error;

/// Calculates the number of days in a coupon period.
///
/// # Arguments
/// * `settlement` - The settlement date.
/// * `maturity` - The maturity date of the bond.
/// * `frequency` - The number of coupon payments per year (1 for annual, 2 for semi-annual, 4 for quarterly).
/// * `basis` - The basis for day count calculation (0 = 30/360, 1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = 30E/360).
///
/// # Returns
/// The number of days in a coupon period.
pub fn codcel_coup_days(
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
        return Err("COUPDAYS: Settlement date must be before maturity date".into());
    }

    if ![1, 2, 4].contains(&frequency) {
        return Err(
            "COUPDAYS: Frequency must be 1 (annual), 2 (semi-annual), or 4 (quarterly)".into(),
        );
    }

    let basis = basis.unwrap_or(0);
    if !(0..=4).contains(&basis) {
        return Err("COUPDAYS: Basis must be between 0 and 4".into());
    }

    // Find the next and previous coupon dates to get actual period for basis 1.
    // Use non-EOM mode (eom=false) so that day-of-month clamping propagates naturally,
    // matching Excel's day count for COUPDAYS.
    let next_coupon =
        crate::financial::helpers::get_next_coupon_date(settlement_date, maturity_date, frequency)?;
    let previous_coupon = crate::financial::helpers::get_previous_coupon_date_eom(
        settlement_date,
        maturity_date,
        frequency,
        false,
    )?;

    match basis {
        0 => Ok(360.0 / frequency as f64), // 30/360 basis
        1 => {
            // Actual/Actual basis - use actual days between coupon dates
            let days = (next_coupon - previous_coupon).num_days();
            Ok(days as f64)
        }
        2 => Ok(360.0 / frequency as f64), // Actual/360 basis
        3 => Ok(365.0 / frequency as f64), // Actual/365 basis
        4 => Ok(360.0 / frequency as f64), // European 30/360 basis
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_coup_days_basic() {
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
        let result = codcel_coup_days(settlement, maturity, frequency, Some(0)).unwrap();

        // 30/360 basis with semi-annual frequency: 360/2 = 180 days
        assert_eq!(result, 180.0);
    }
}
