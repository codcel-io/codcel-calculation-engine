// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::date_and_time::codcel_year_frac::codcel_year_frac;
use chrono::{DateTime, Utc};
use std::error::Error;

/// Calculates the annual yield for a discounted security.
///
/// # Arguments
/// * `settlement` - The settlement date of the security.
/// * `maturity` - The maturity date of the security.
/// * `price` - The price per $100 face value of the security.
/// * `redemption` - The redemption value per $100 face value of the security.
/// * `basis` - The day count basis to use (0-4, optional, defaults to 0).
///
/// # Returns
/// The annual yield of the discounted security.
pub fn codcel_yield_disc(
    settlement: DateTime<Utc>,
    maturity: DateTime<Utc>,
    price: f64,
    redemption: f64,
    basis: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Validate inputs
    if price <= 0.0 {
        return Err("YIELDDISC: Price must be greater than 0.".into());
    }
    if redemption <= 0.0 {
        return Err("YIELDDISC: Redemption must be greater than 0.".into());
    }
    if maturity <= settlement {
        return Err("YIELDDISC: Maturity must be later than settlement.".into());
    }

    let basis_val = basis.unwrap_or(0);
    if !(0..=4).contains(&basis_val) {
        return Err("YIELDDISC: Basis must be between 0 and 4.".into());
    }

    let year_frac = codcel_year_frac(settlement, maturity, Some(basis_val))?;
    if year_frac <= 0.0 {
        return Err("YIELDDISC: Invalid date range".into());
    }

    // Calculate the discount yield using Excel's formula:
    // YIELDDISC = ((redemption - price) / price) / YEARFRAC(settlement, maturity, basis)
    let yield_disc = ((redemption - price) / price) / year_frac;

    Ok(yield_disc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_yield_disc_basic() {
        let settlement = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let result = codcel_yield_disc(settlement, maturity, 95.0, 100.0, Some(0)).unwrap();
        assert!(result > 0.0);
    }

    #[test]
    fn test_yield_disc_error_cases() {
        let settlement = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();

        // Maturity date must be after settlement date
        assert!(codcel_yield_disc(settlement, maturity, 95.0, 100.0, Some(0)).is_err());

        // Price must be greater than 0
        let maturity = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        assert!(codcel_yield_disc(settlement, maturity, 0.0, 100.0, Some(0)).is_err());

        // Redemption must be greater than 0
        assert!(codcel_yield_disc(settlement, maturity, 95.0, 0.0, Some(0)).is_err());

        // Basis must be between 0 and 4
        assert!(codcel_yield_disc(settlement, maturity, 95.0, 100.0, Some(5)).is_err());
    }
}
