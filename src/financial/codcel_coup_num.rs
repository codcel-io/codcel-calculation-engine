// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::date_time_base::get_days_in_month;
use crate::financial::helpers::get_next_coupon_date_coup_num;
use chrono::{DateTime, Datelike, NaiveDate, Utc};
use std::error::Error;

/// Calculates the number of coupon payments between the settlement date and maturity date.
///
/// # Arguments
/// * `settlement` - The settlement date.
/// * `maturity` - The maturity date of the bond.
/// * `frequency` - The number of coupon payments per year (1 for annual, 2 for semi-annual, 4 for quarterly).
/// * `basis` - The basis for day count calculation (0 = 30/360, 1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = 30E/360).
///
/// # Returns
/// The number of coupon payments between the settlement date and maturity date.
pub fn codcel_coup_num(
    settlement: DateTime<Utc>,
    maturity: DateTime<Utc>,
    frequency: i32,
    basis: Option<i32>,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    let settlement_date = settlement.date_naive();
    let maturity_date = maturity.date_naive();

    // Validation checks remain the same
    if settlement_date >= maturity_date {
        return Err("COUPNUM: Settlement date must be before maturity date".into());
    }
    if ![1, 2, 4].contains(&frequency) {
        return Err(
            "COUPNUM: Frequency must be 1 (annual), 2 (semi-annual), or 4 (quarterly)".into(),
        );
    }
    let basis = basis.unwrap_or(0);
    if !(0..=4).contains(&basis) {
        return Err("COUPNUM: Basis must be between 0 and 4".into());
    }

    let months_between = 12 / frequency;

    // Find the first coupon date after settlement
    let first_coupon = get_next_coupon_date_coup_num(settlement_date, maturity_date, frequency)?;

    // Calculate number of full periods between first coupon and maturity
    let mut num_coupons = 1; // Include the final payment at maturity
    let mut current = first_coupon;

    while current < maturity_date {
        // Move forward by one period
        let new_month = ((current.month() as i32 + months_between - 1) % 12 + 1) as u32;
        let new_year = current.year() + (current.month() as i32 + months_between - 1) / 12;

        if let Some(new_date) = NaiveDate::from_ymd_opt(
            new_year,
            new_month,
            std::cmp::min(current.day(), get_days_in_month(new_year, new_month)),
        ) {
            current = new_date;
            if current <= maturity_date {
                num_coupons += 1;
            }
        } else {
            return Err("COUPNUM: Invalid date calculation".into());
        }
    }

    Ok(num_coupons)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_coup_num_basic() {
        let settlement = Utc.with_ymd_and_hms(2021, 2, 15, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2023, 5, 15, 0, 0, 0).unwrap();
        let frequency = 2; // Semi-annual

        // With basis 0 (30/360)
        let result = codcel_coup_num(settlement, maturity, frequency, Some(0)).unwrap();

        // Coupons on: 2021-05-15, 2021-11-15, 2022-05-15, 2022-11-15, 2023-05-15
        // Total: 5 coupons
        assert_eq!(result, 5);
    }
}
