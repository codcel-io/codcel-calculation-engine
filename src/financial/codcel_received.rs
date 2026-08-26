// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::date_and_time::codcel_year_frac::codcel_year_frac;
use chrono::{DateTime, Utc};
use std::error::Error;

/// Calculate the amount received at maturity for a fully invested security.
///
/// # Arguments
/// * `settlement` - The settlement date of the security.
/// * `maturity` - The maturity date of the security.
/// * `investment` - The amount invested in the security.
/// * `discount` - The discount rate of the security.
/// * `basis` - The day count basis to use (0-4, optional, defaults to 0).
///
/// # Returns
/// The amount received at maturity.
pub fn codcel_received(
    settlement: DateTime<Utc>,
    maturity: DateTime<Utc>,
    investment: f64,
    discount: f64,
    basis: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let basis = basis.unwrap_or(0);

    // Validate inputs
    if investment <= 0.0 {
        return Err("RECEIVED: Investment must be greater than zero.".into());
    }
    if discount <= 0.0 || discount >= 1.0 {
        return Err("RECEIVED: Discount must be greater than 0 and less than 1.".into());
    }
    if maturity <= settlement {
        return Err("RECEIVED: Maturity date must be after settlement date.".into());
    }
    if !(0..=4).contains(&basis) {
        return Err("RECEIVED: Basis must be between 0 and 4.".into());
    }

    let year_frac = codcel_year_frac(settlement, maturity, Some(basis))?;

    // Excel formula: RECEIVED = investment / (1 - discount * YEARFRAC(settlement, maturity, basis))
    let received = investment / (1.0 - discount * year_frac);

    Ok(received)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_received_basic() {
        // Excel: =RECEIVED(DATE(2022,1,1),DATE(2023,1,1),1000,0.05,0) = 1052.631578947368
        let settlement = Utc
            .with_ymd_and_hms(2022, 1, 1, 0, 0, 0)
            .single()
            .expect("valid test date");
        let maturity = Utc
            .with_ymd_and_hms(2023, 1, 1, 0, 0, 0)
            .single()
            .expect("valid test date");

        let result = codcel_received(settlement, maturity, 1000.0, 0.05, Some(0)).unwrap();
        assert!((result - 1052.631578947368).abs() < 1e-6);
    }

    #[test]
    fn test_received_basis_2() {
        // Basis 2 = Actual/360
        let settlement = Utc
            .with_ymd_and_hms(2022, 1, 1, 0, 0, 0)
            .single()
            .expect("valid test date");
        let maturity = Utc
            .with_ymd_and_hms(2023, 1, 1, 0, 0, 0)
            .single()
            .expect("valid test date");

        let result = codcel_received(settlement, maturity, 1000.0, 0.05, Some(2));
        assert!(result.is_ok());
        assert!(result.unwrap() > 1000.0);
    }

    #[test]
    fn test_received_basis_3() {
        // Basis 3 = Actual/365
        let settlement = Utc
            .with_ymd_and_hms(2022, 1, 1, 0, 0, 0)
            .single()
            .expect("valid test date");
        let maturity = Utc
            .with_ymd_and_hms(2023, 1, 1, 0, 0, 0)
            .single()
            .expect("valid test date");

        let result = codcel_received(settlement, maturity, 1000.0, 0.05, Some(3));
        assert!(result.is_ok());
        assert!(result.unwrap() > 1000.0);
    }

    #[test]
    fn test_received_error_cases() {
        let settlement = Utc
            .with_ymd_and_hms(2022, 1, 1, 0, 0, 0)
            .single()
            .expect("valid test date");
        let maturity = Utc
            .with_ymd_and_hms(2022, 1, 1, 0, 0, 0)
            .single()
            .expect("valid test date");

        // Maturity must be after settlement
        assert!(codcel_received(settlement, maturity, 1000.0, 0.05, Some(0)).is_err());

        // Investment must be greater than zero
        let maturity = Utc
            .with_ymd_and_hms(2023, 1, 1, 0, 0, 0)
            .single()
            .expect("valid test date");
        assert!(codcel_received(settlement, maturity, 0.0, 0.05, Some(0)).is_err());

        // Discount must be greater than 0 and less than 1
        assert!(codcel_received(settlement, maturity, 1000.0, 0.0, Some(0)).is_err());
        assert!(codcel_received(settlement, maturity, 1000.0, 1.0, Some(0)).is_err());

        // Invalid basis
        assert!(codcel_received(settlement, maturity, 1000.0, 0.05, Some(5)).is_err());
    }
}
