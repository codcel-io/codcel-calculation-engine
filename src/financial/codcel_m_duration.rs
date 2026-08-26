// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::financial::codcel_duration::codcel_duration;
use chrono::{DateTime, Utc};
use std::error::Error;

/// Calculates the modified Macaulay duration for a security with periodic interest payments.
///
/// # Arguments
/// * `settlement` - The settlement date of the security.
/// * `maturity` - The maturity date of the security.
/// * `coupon` - The annual coupon rate of the security.
/// * `yield_rate` - The annual yield of the security.
/// * `frequency` - The number of coupon payments per year (1, 2, or 4).
/// * `basis` - The day count basis to use (0-4, optional, defaults to 0).
///
/// # Returns
/// The modified Macaulay duration for the security.
pub fn codcel_m_duration(
    settlement: DateTime<Utc>,
    maturity: DateTime<Utc>,
    coupon: f64,
    yield_rate: f64,
    frequency: i32,
    basis: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let duration = codcel_duration(settlement, maturity, coupon, yield_rate, frequency, basis)?;
    Ok(duration / (1.0 + yield_rate / frequency as f64))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_m_duration_basic() {
        let settlement = Utc
            .with_ymd_and_hms(2022, 1, 1, 0, 0, 0)
            .single()
            .expect("valid test date");
        let maturity = Utc
            .with_ymd_and_hms(2027, 1, 1, 0, 0, 0)
            .single()
            .expect("valid test date");
        let result = codcel_m_duration(settlement, maturity, 0.05, 0.06, 2, Some(0)).unwrap();
        assert!(result > 0.0);
    }

    #[test]
    fn test_m_duration_error_cases() {
        // Since m_duration relies on duration, we'll test that errors from duration are propagated
        let settlement = Utc
            .with_ymd_and_hms(2022, 1, 1, 0, 0, 0)
            .single()
            .expect("valid test date");
        let maturity = Utc
            .with_ymd_and_hms(2022, 1, 1, 0, 0, 0)
            .single()
            .expect("valid test date");

        // Settlement must be before maturity
        assert!(codcel_m_duration(settlement, maturity, 0.05, 0.06, 2, Some(0)).is_err());
    }
}
