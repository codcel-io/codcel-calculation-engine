// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use super::codcel_price::codcel_price;
use chrono::{DateTime, Utc};
use std::error::Error;

/// Calculates the yield on a security that pays periodic interest.
///
/// Uses Newton-Raphson iteration to find the yield where
/// PRICE(settlement, maturity, rate, yield, redemption, frequency, basis) == price.
///
/// # Arguments
/// * `settlement` - The settlement date of the security.
/// * `maturity` - The maturity date of the security.
/// * `rate` - The annual coupon rate of the security.
/// * `price` - The price per $100 face value of the security.
/// * `redemption` - The redemption value per $100 face value of the security.
/// * `frequency` - The number of coupon payments per year (1, 2, or 4).
/// * `basis` - The day count basis to use (0-4, optional, defaults to 0).
///
/// # Returns
/// The annual yield of the security.
#[allow(clippy::too_many_arguments)]
pub fn codcel_yield(
    settlement: DateTime<Utc>,
    maturity: DateTime<Utc>,
    rate: f64,
    price: f64,
    redemption: f64,
    frequency: i32,
    basis: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if frequency != 1 && frequency != 2 && frequency != 4 {
        return Err(
            "YIELD: Frequency must be 1 (annual), 2 (semiannual), or 4 (quarterly).".into(),
        );
    }

    let basis_val = basis.unwrap_or(0);
    if basis_val > 4 {
        return Err("YIELD: Basis must be between 0 and 4.".into());
    }

    if settlement >= maturity {
        return Err("YIELD: Maturity date must be after settlement date.".into());
    }

    // Initial guess: simple current yield approximation
    let coupon = 100.0 * rate;
    let mut yield_guess = (coupon + (redemption - price) / 5.0) / ((price + redemption) / 2.0);
    if yield_guess.is_nan() || yield_guess <= 0.0 {
        yield_guess = 0.1;
    }

    const MAX_ITERATIONS: usize = 200;
    const TOLERANCE: f64 = 1e-10;
    const DY: f64 = 1e-7; // Small delta for numerical derivative

    for _ in 0..MAX_ITERATIONS {
        let price_at_yield = codcel_price(
            settlement,
            maturity,
            rate,
            yield_guess,
            redemption,
            frequency,
            basis,
        )?;
        let delta = price_at_yield - price;

        if delta.abs() < TOLERANCE {
            return Ok(yield_guess);
        }

        // Numerical derivative: dPrice/dYield
        let price_at_yield_plus = codcel_price(
            settlement,
            maturity,
            rate,
            yield_guess + DY,
            redemption,
            frequency,
            basis,
        )?;
        let derivative = (price_at_yield_plus - price_at_yield) / DY;

        if derivative.abs() < 1e-15 {
            return Err("YIELD: Derivative too small, cannot converge.".into());
        }

        let correction = delta / derivative;
        if correction.abs() < TOLERANCE {
            return Ok(yield_guess);
        }

        yield_guess -= correction;
        yield_guess = yield_guess.clamp(-0.99, 100.0);
    }

    Err("YIELD: Failed to converge to a solution.".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_yield_basic() {
        let settlement = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2027, 1, 1, 0, 0, 0).unwrap();
        let result = codcel_yield(settlement, maturity, 0.05, 95.0, 100.0, 2, Some(0)).unwrap();
        assert!(result > 0.0);
    }

    #[test]
    fn test_yield_error_cases() {
        let settlement = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();

        // Maturity date must be after settlement date
        assert!(codcel_yield(settlement, maturity, 0.05, 95.0, 100.0, 2, Some(0)).is_err());

        // Frequency must be 1, 2, or 4
        let maturity = Utc.with_ymd_and_hms(2027, 1, 1, 0, 0, 0).unwrap();
        assert!(codcel_yield(settlement, maturity, 0.05, 95.0, 100.0, 3, Some(0)).is_err());

        // Basis must be between 0 and 4
        assert!(codcel_yield(settlement, maturity, 0.05, 95.0, 100.0, 2, Some(5)).is_err());
    }
}
