// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::date_time_base::get_days_between;
use chrono::{DateTime, Datelike, Utc};
use std::error::Error;

/// Calculates the price per $100 face value of a security that pays interest at maturity.
///
/// Equivalent to Excel's `PRICEMAT`, this discounts the redemption value using the
/// coupon rate, yield, and day-count basis between issue, settlement, and maturity.
///
/// # Arguments
/// * `settlement` - Settlement date of the security.
/// * `maturity` - Maturity date of the security.
/// * `issue` - Original issue date.
/// * `rate` - Annual coupon rate.
/// * `yield_rate` - Annual yield expected by the investor.
/// * `basis` - Optional day-count basis (0-4).
///
/// # Errors
/// Returns an error when dates are inconsistent or rates/basis are invalid.
pub fn codcel_price_mat(
    settlement: DateTime<Utc>,
    maturity: DateTime<Utc>,
    issue: DateTime<Utc>,
    rate: f64,
    yield_rate: f64,
    basis: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Validate inputs
    if yield_rate < 0.0 {
        return Err("PRICEMAT: Yield rate must be greater than or equal to 0.".into());
    }
    if rate < 0.0 {
        return Err("PRICEMAT: Rate must be greater than or equal to 0.".into());
    }
    if maturity <= settlement {
        return Err("PRICEMAT: Maturity must be later than settlement.".into());
    }
    if settlement <= issue {
        return Err("PRICEMAT: Settlement must be later than issue date.".into());
    }

    let basis = basis.unwrap_or(0);
    if basis > 4 {
        return Err("PRICEMAT: Basis must be between 0 and 4.".into());
    }

    let dsm = get_days_between(&settlement, &maturity, basis) as f64;
    let dim = get_days_between(&issue, &maturity, basis) as f64;
    let dis = get_days_between(&issue, &settlement, basis) as f64;

    let days_in_year = match basis {
        0 => 360.0,
        1 => {
            let year = settlement.year();
            if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                366.0
            } else {
                365.0
            }
        }
        2 => 360.0,
        3 => 365.0,
        4 => 360.0,
        _ => unreachable!(),
    };

    let b = 1.0 + (dim / days_in_year) * rate;
    let d = 1.0 + (dsm / days_in_year) * yield_rate;

    let price_mat = (b / d) - (dis / days_in_year) * rate;

    Ok(price_mat * 100.0)
}


#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_price_mat_basic() {
        let settlement = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let issue = Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0).unwrap();

        let result = codcel_price_mat(settlement, maturity, issue, 0.05, 0.06, Some(0));

        assert!(result.is_ok());
        let price = result.unwrap();
        assert!(price > 0.0);
    }

    #[test]
    fn test_price_mat_error_cases() {
        let settlement = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
        let issue = Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0).unwrap();

        // Maturity must be later than settlement
        assert!(codcel_price_mat(settlement, maturity, issue, 0.05, 0.06, Some(0)).is_err());

        // Settlement must be later than issue
        let settlement = Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        assert!(codcel_price_mat(settlement, maturity, settlement, 0.05, 0.06, Some(0)).is_err());

        // Rate must be non-negative
        let settlement = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
        assert!(codcel_price_mat(settlement, maturity, issue, -0.05, 0.06, Some(0)).is_err());

        // Yield must be non-negative
        assert!(codcel_price_mat(settlement, maturity, issue, 0.05, -0.06, Some(0)).is_err());

        // Basis must be between 0 and 4
        assert!(codcel_price_mat(settlement, maturity, issue, 0.05, 0.06, Some(5)).is_err());
    }
}
