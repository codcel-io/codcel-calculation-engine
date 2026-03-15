// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::date_and_time::codcel_year_frac::codcel_year_frac;
use chrono::{DateTime, Utc};
use std::error::Error;

/// Calculates the discount rate for a security.
///
/// # Arguments
/// * `settlement` - The settlement date of the security.
/// * `maturity` - The maturity date of the security.
/// * `price` - The price per $100 face value of the security.
/// * `redemption` - The redemption value per $100 face value of the security.
/// * `basis` - Optional. The day count basis to use. If omitted, 0 (US 30/360) is used.
///   * 0 = US 30/360
///   * 1 = Actual/Actual
///   * 2 = Actual/360
///   * 3 = Actual/365
///   * 4 = European 30/360
///
/// # Returns
/// The discount rate for the security.
pub fn codcel_disc(
    settlement: DateTime<Utc>,
    maturity: DateTime<Utc>,
    price: f64,
    redemption: f64,
    basis: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if price <= 0.0 {
        return Err("DISC: Price must be greater than 0".into());
    }
    if redemption <= 0.0 {
        return Err("DISC: Redemption must be greater than 0".into());
    }
    if settlement >= maturity {
        return Err("DISC: Settlement date must be earlier than maturity date".into());
    }

    let basis = basis.unwrap_or(0);
    if !(0..=4).contains(&basis) {
        return Err("DISC: Invalid basis".into());
    }

    let year_frac = codcel_year_frac(settlement, maturity, Some(basis))?;
    if year_frac <= 0.0 {
        return Err("DISC: Invalid date range".into());
    }

    let discount_rate = (redemption - price) / redemption / year_frac;
    Ok(discount_rate)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_disc_basic() {
        let settlement = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let result = codcel_disc(settlement, maturity, 95.0, 100.0, None).unwrap();
        assert!((result - 0.05).abs() < 0.0001);
    }

    #[test]
    fn test_disc_basis_1_within_leap_year() {
        // 2024-01-01 to 2024-07-01, basis=1, YEARFRAC uses 366
        let settlement = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2024, 7, 1, 0, 0, 0).unwrap();
        let result = codcel_disc(settlement, maturity, 97.975, 100.0, Some(1)).unwrap();
        assert!((result - 0.04072252747252768).abs() < 1e-12);
    }

    #[test]
    fn test_disc_basis_1_no_feb29_in_range() {
        // 2023-01-01 to 2024-01-01, basis=1, period doesn't include Feb 29
        let settlement = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let result = codcel_disc(settlement, maturity, 97.0, 100.0, Some(1)).unwrap();
        assert!((result - 0.030000000000000027).abs() < 1e-12);
    }

    #[test]
    fn test_disc_basis_1_crossing_leap_year_with_feb29() {
        // 2023-06-10 to 2024-06-10, basis=1, includes Feb 29 2024
        let settlement = Utc.with_ymd_and_hms(2023, 6, 10, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2024, 6, 10, 0, 0, 0).unwrap();
        let result = codcel_disc(settlement, maturity, 98.5, 105.0, Some(1)).unwrap();
        assert!((result - 0.06190476190476191).abs() < 1e-12);
    }

    #[test]
    fn test_disc_basis_1_multi_year() {
        // 2024-01-01 to 2030-01-01, basis=1, 6-year span uses average year length
        let settlement = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2030, 1, 1, 0, 0, 0).unwrap();
        let result = codcel_disc(settlement, maturity, 90.0, 100.0, Some(1)).unwrap();
        assert!((result - 0.016664494264859223).abs() < 1e-12);
    }

    #[test]
    fn test_disc_basis_0_feb29_settlement() {
        // Feb 29 settlement with basis=0, YEARFRAC adjusts Feb 29 → day 30
        let settlement = Utc.with_ymd_and_hms(2024, 2, 29, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2024, 5, 31, 0, 0, 0).unwrap();
        let result = codcel_disc(settlement, maturity, 99.75, 100.0, Some(0)).unwrap();
        assert!((result - 0.00989010989010968).abs() < 1e-12);
    }
}
