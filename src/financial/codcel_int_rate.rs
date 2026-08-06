// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::date_and_time::codcel_year_frac::codcel_year_frac;
use chrono::{DateTime, Utc};
use std::error::Error;

/// Calculates the interest rate for a fully invested security.
///
/// # Arguments
/// * `settlement` - The settlement date of the security.
/// * `maturity` - The maturity date of the security.
/// * `investment` - The amount invested in the security.
/// * `redemption` - The amount to be received at maturity.
/// * `basis` - The day count basis to use (0-4, optional, defaults to 0).
///
/// # Returns
/// The interest rate for the security.
pub fn codcel_int_rate(
    settlement: DateTime<Utc>,
    maturity: DateTime<Utc>,
    investment: f64,
    redemption: f64,
    basis: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if settlement >= maturity {
        return Err("INTRATE: Settlement date must be before the maturity date".into());
    }
    if investment <= 0.0 {
        return Err("INTRATE: Investment must be greater than 0".into());
    }
    if redemption <= 0.0 {
        return Err("INTRATE: Redemption must be greater than 0".into());
    }

    let basis = basis.unwrap_or(0);
    if ![0, 1, 2, 3, 4].contains(&basis) {
        return Err("INTRATE: Basis must be 0, 1, 2, 3, or 4".into());
    }

    let years = codcel_year_frac(settlement, maturity, Some(basis))?;

    if years <= 0.0 {
        return Err("INTRATE: Maturity date and settlement date do not result in a positive year difference".into());
    }

    // Calculate the interest rate
    let interest_rate = (redemption - investment) / (investment * years);

    Ok(interest_rate)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_int_rate_basic() {
        let settlement = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let result = codcel_int_rate(settlement, maturity, 1000.0, 1100.0, Some(0)).unwrap();
        assert!((result - 0.1).abs() < 0.001);
    }

    #[test]
    fn test_int_rate_error_cases() {
        let settlement = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();

        // Settlement must be before maturity
        assert!(codcel_int_rate(settlement, maturity, 1000.0, 1100.0, Some(0)).is_err());

        // Investment must be greater than 0
        let maturity = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        assert!(codcel_int_rate(settlement, maturity, 0.0, 1100.0, Some(0)).is_err());

        // Redemption must be greater than 0
        assert!(codcel_int_rate(settlement, maturity, 1000.0, 0.0, Some(0)).is_err());

        // Basis must be 0, 1, 2, 3, or 4
        assert!(codcel_int_rate(settlement, maturity, 1000.0, 1100.0, Some(5)).is_err());
    }
}
