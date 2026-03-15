// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::date_time_base::get_days_between;
use chrono::{DateTime, Datelike, Utc};
use std::error::Error;

/// Calculates the annual yield of a security that pays interest at maturity.
///
/// # Arguments
/// * `settlement` - The settlement date of the security.
/// * `maturity` - The maturity date of the security.
/// * `issue` - The issue date of the security.
/// * `rate` - The interest rate of the security at the issue date.
/// * `price` - The price per $100 face value of the security.
/// * `basis` - The day count basis to use (0-4, optional, defaults to 0).
///
/// # Returns
/// The annual yield of the security.
pub fn codcel_yield_mat(
    settlement: DateTime<Utc>,
    maturity: DateTime<Utc>,
    issue: DateTime<Utc>,
    rate: f64,
    price: f64,
    basis: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Validate inputs
    if price <= 0.0 {
        return Err("YIELDMAT: Price must be greater than 0.".into());
    }
    if rate < 0.0 {
        return Err("YIELDMAT: Rate must be greater than or equal to 0.".into());
    }
    if maturity <= settlement {
        return Err("YIELDMAT: Maturity must be later than settlement.".into());
    }
    if settlement <= issue {
        return Err("YIELDMAT: Settlement must be later than issue date.".into());
    }

    let basis = basis.unwrap_or(0);
    if basis > 4 {
        return Err("YIELDMAT: Basis must be between 0 and 4.".into());
    }

    let days_in_year = match basis {
        0 => 360.0, // 30/360
        1 => {
            // Actual/Actual: use actual days in the issue year
            let year = issue.year();
            if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                366.0
            } else {
                365.0
            }
        }
        2 => 360.0, // Actual/360
        3 => 365.0, // Actual/365
        4 => 360.0, // European 30/360
        _ => unreachable!(),
    };

    let dsm = get_days_between(&settlement, &maturity, basis) as f64;
    let dis = get_days_between(&issue, &settlement, basis) as f64;
    let dim = get_days_between(&issue, &maturity, basis) as f64;

    let par = 100.0;

    // New implementation following Excel's exact formula
    let annual_rate = rate; // Annual interest rate
    let redemption = par; // Redemption value

    // Calculate accrued interest from issue to settlement
    let accrued_interest = redemption * annual_rate * dis / days_in_year;

    // Calculate the future value
    let future_value = redemption * (1.0 + annual_rate * dim / days_in_year);

    // Calculate the price with accrued interest
    let full_price = price + accrued_interest;

    // Calculate the yield
    let yield_mat = ((future_value - full_price) / full_price) * (days_in_year / dsm);

    Ok(yield_mat)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_yield_mat_basic() {
        let issue = Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0).unwrap();
        let settlement = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let result = codcel_yield_mat(settlement, maturity, issue, 0.05, 95.0, Some(0)).unwrap();
        assert!(result > 0.0);
    }

    #[test]
    fn test_yield_mat_error_cases() {
        let issue = Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0).unwrap();
        let settlement = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();

        // Maturity date must be after settlement date
        assert!(codcel_yield_mat(settlement, maturity, issue, 0.05, 95.0, Some(0)).is_err());

        // Settlement must be after issue date
        let settlement = Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        assert!(codcel_yield_mat(settlement, maturity, issue, 0.05, 95.0, Some(0)).is_err());

        // Price must be greater than 0
        let settlement = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
        assert!(codcel_yield_mat(settlement, maturity, issue, 0.05, 0.0, Some(0)).is_err());

        // Rate must be greater than or equal to 0
        assert!(codcel_yield_mat(settlement, maturity, issue, -0.01, 95.0, Some(0)).is_err());

        // Basis must be between 0 and 4
        assert!(codcel_yield_mat(settlement, maturity, issue, 0.05, 95.0, Some(5)).is_err());
    }
}
