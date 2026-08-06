// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::date_and_time::codcel_year_frac::codcel_year_frac;
use chrono::{DateTime, Utc};
use std::error::Error;

/// Calculates the price per $100 face value of a discount security.
///
/// Mirrors Excel's `PRICEDISC`, returning the present price of a security sold
/// at a discount using the settlement/maturity dates, discount rate, redemption
/// value, and day-count basis.
///
/// # Arguments
/// * `settlement` - Settlement date of the security.
/// * `maturity` - Maturity date of the security.
/// * `discount` - Annual discount rate.
/// * `redemption` - Redemption value per $100 face value.
/// * `basis` - Optional day-count basis (0-4).
///
/// # Errors
/// Returns an error when inputs are invalid or dates are out of order.
pub fn codcel_price_disc(
    settlement: DateTime<Utc>,
    maturity: DateTime<Utc>,
    discount: f64,
    redemption: f64,
    basis: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Validate inputs
    if discount <= 0.0 {
        return Err("PRICEDISC: Discount must be greater than 0.".into());
    }
    if redemption <= 0.0 {
        return Err("PRICEDISC: Redemption must be greater than 0.".into());
    }
    if maturity <= settlement {
        return Err("PRICEDISC: Maturity must be later than settlement.".into());
    }

    let basis_val = basis.unwrap_or(0);
    if !(0..=4).contains(&basis_val) {
        return Err("PRICEDISC: Basis must be between 0 and 4.".into());
    }

    let year_frac = codcel_year_frac(settlement, maturity, Some(basis_val))?;

    // Calculate the price using Excel's formula:
    // PRICEDISC = redemption - discount * redemption * YEARFRAC(settlement, maturity, basis)
    let price_disc = redemption - (discount * redemption * year_frac);

    Ok(price_disc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_price_disc_basic() {
        let settlement = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();

        let result = codcel_price_disc(settlement, maturity, 0.05, 100.0, Some(0));

        assert!(result.is_ok());
        let price = result.unwrap();
        assert!(price > 0.0 && price < 100.0);
    }

    #[test]
    fn test_price_disc_error_cases() {
        let settlement = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();

        // Maturity must be later than settlement
        assert!(codcel_price_disc(settlement, maturity, 0.05, 100.0, Some(0)).is_err());

        // Discount must be greater than 0
        let maturity = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        assert!(codcel_price_disc(settlement, maturity, 0.0, 100.0, Some(0)).is_err());

        // Redemption must be greater than 0
        assert!(codcel_price_disc(settlement, maturity, 0.05, 0.0, Some(0)).is_err());

        // Basis must be between 0 and 4
        assert!(codcel_price_disc(settlement, maturity, 0.05, 100.0, Some(5)).is_err());
    }
}
