// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::financial::codcel_odd_l_price::codcel_odd_l_price;
use chrono::{DateTime, Utc};
use std::error::Error;

/// Calculates the yield of a security with an odd last coupon period.
///
/// Mirrors Excel's `ODDLYIELD`, solving for the annual yield that prices the
/// security given its odd final coupon, redemption amount, frequency, and
/// day-count basis.
///
/// Uses Newton-Raphson with numerical derivatives via the working ODDLPRICE function.
///
/// # Arguments
/// * `settlement` - Settlement date of the security.
/// * `maturity` - Maturity date of the security.
/// * `last_interest` - Date of the last coupon payment before settlement.
/// * `rate` - Annual coupon rate.
/// * `price` - Price per $100 face value.
/// * `redemption` - Redemption value per $100 face value.
/// * `frequency` - Number of coupon payments per year (1, 2, or 4).
/// * `basis` - Optional day-count basis (0-4).
///
/// # Errors
/// Returns an error when inputs are invalid or dates are out of order.
#[allow(clippy::too_many_arguments)]
pub fn codcel_odd_l_yield(
    settlement: DateTime<Utc>,
    maturity: DateTime<Utc>,
    last_interest: DateTime<Utc>,
    rate: f64,
    price: f64,
    redemption: f64,
    frequency: i32,
    basis: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Validate inputs
    if settlement >= maturity {
        return Err("ODDLYIELD: Settlement date must be before maturity date".into());
    }
    if last_interest > settlement {
        return Err(
            "ODDLYIELD: Last interest date must be before or equal to settlement date".into(),
        );
    }
    if rate < 0.0 {
        return Err("ODDLYIELD: Rate cannot be negative".into());
    }
    if price <= 0.0 {
        return Err("ODDLYIELD: Price must be positive".into());
    }
    if redemption <= 0.0 {
        return Err("ODDLYIELD: Redemption value must be positive".into());
    }
    if ![1, 2, 4].contains(&frequency) {
        return Err("ODDLYIELD: Frequency must be 1, 2, or 4".into());
    }

    let basis_val = basis.unwrap_or(0);
    if !(0..=4).contains(&basis_val) {
        return Err("ODDLYIELD: Basis must be between 0 and 4".into());
    }

    // Helper closure: compute ODDLPRICE at a given yield
    let price_at_yield = |y: f64| -> Result<f64, Box<dyn Error + Send + Sync>> {
        codcel_odd_l_price(
            settlement,
            maturity,
            last_interest,
            rate,
            y,
            redemption,
            frequency,
            basis,
        )
    };

    // Special case: when rate=0 and price=redemption, yield is 0
    if rate == 0.0 && (price - redemption).abs() < 1e-12 {
        return Ok(0.0);
    }

    // Newton-Raphson with numerical derivative.
    // Excel allows negative yields for deeply premium bonds (price > redemption).
    // Start with rate as initial guess, or a small positive value for zero-rate bonds.
    // If price > redemption, also try a negative starting point and pick the closer root.
    let mut yield_guess = if rate > 0.0 { rate } else { 0.05 };

    let max_iterations = 200;
    let tolerance = 1e-12;
    let dy = 1e-7;

    let run_nr = |mut y: f64| -> f64 {
        for _ in 0..max_iterations {
            let p = match price_at_yield(y) {
                Ok(v) => v,
                Err(_) => return y,
            };
            let diff = p - price;

            if diff.abs() < tolerance {
                break;
            }

            // Central difference numerical derivative: dp/dy ≈ (p(y+dy) - p(y-dy)) / (2*dy)
            let p_up = match price_at_yield(y + dy) {
                Ok(v) => v,
                Err(_) => break,
            };
            let p_down = match price_at_yield(y - dy) {
                Ok(v) => v,
                Err(_) => break,
            };
            let derivative = (p_up - p_down) / (2.0 * dy);

            if derivative.abs() < 1e-15 {
                break;
            }

            y -= diff / derivative;

            // Only clamp the upper bound; negative yields are valid for premium bonds
            if y > 10.0 {
                y = 10.0;
            }
        }
        y
    };

    yield_guess = run_nr(yield_guess);

    // If price > redemption, the true yield may be negative. Try a negative starting
    // point and use whichever guess converges closer to the target price.
    if price > redemption {
        let neg_guess = run_nr(-0.1);
        let p_pos = price_at_yield(yield_guess).unwrap_or(f64::MAX);
        let p_neg = price_at_yield(neg_guess).unwrap_or(f64::MAX);
        if (p_neg - price).abs() < (p_pos - price).abs() {
            yield_guess = neg_guess;
        }
    }

    Ok(yield_guess)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn excel_date(serial: f64) -> DateTime<Utc> {
        let base = Utc.with_ymd_and_hms(1899, 12, 30, 0, 0, 0).unwrap();
        base + chrono::Duration::days(serial as i64)
    }

    #[test]
    fn test_odd_l_yield_ann_b_0() {
        // rate=0.075, price=101.68, freq=1, basis=0, redemption=100
        // Expected: 0.0600000144352254
        let result = codcel_odd_l_yield(
            excel_date(44986.0),
            excel_date(45519.0),
            excel_date(44788.0),
            0.075,
            101.68,
            100.0,
            1,
            Some(0),
        )
        .unwrap();
        println!("ann_b_0: {result}");
        assert!(
            (result - 0.0600000144352254).abs() < 0.000001,
            "Expected 0.0600000144352254, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_basis_0_semi() {
        // rate=0.05, price=99.62, freq=2, basis=0, redemption=100
        // Expected: 0.059836289291541694
        let result = codcel_odd_l_yield(
            excel_date(43136.0),
            excel_date(43266.0),
            excel_date(43023.0),
            0.05,
            99.62,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("basis_0_semi: {result}");
        assert!(
            (result - 0.059836289291541694).abs() < 0.000001,
            "Expected 0.059836289291541694, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_both_zero() {
        // rate=0.0, price=100.0, freq=2, basis=0, redemption=100
        // Expected: 0.0
        let result = codcel_odd_l_yield(
            excel_date(44317.0),
            excel_date(44592.0),
            excel_date(44043.0),
            0.0,
            100.0,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("both_zero: {result}");
        assert!(
            result.abs() < 0.000001,
            "Expected 0.0, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_error_cases() {
        let settlement = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
        let last_interest = Utc.with_ymd_and_hms(2021, 7, 1, 0, 0, 0).unwrap();

        assert!(codcel_odd_l_yield(
            settlement,
            maturity,
            last_interest,
            0.05,
            95.0,
            100.0,
            2,
            Some(0)
        )
        .is_err());
    }
}
