// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::financial::codcel_odd_f_price::codcel_odd_f_price;
use chrono::{DateTime, Utc};
use std::error::Error;

/// Calculates the yield of a security with an odd first coupon period.
///
/// Implements the logic of Excel's `ODDFYIELD`, solving for the annual yield
/// that produces the observed price given coupon rate, redemption value,
/// payment frequency, and day-count basis.
///
/// Uses Newton-Raphson with numerical derivatives via the working ODDFPRICE function.
///
/// # Arguments
/// * `settlement` - Settlement date of the security.
/// * `maturity` - Maturity date of the security.
/// * `issue` - Issue date of the security.
/// * `first_coupon` - Date of the first coupon payment.
/// * `rate` - Annual coupon rate.
/// * `price` - Observed price per $100 face value.
/// * `redemption` - Redemption value per $100 face value.
/// * `frequency` - Number of coupon payments per year (1, 2, or 4).
/// * `basis` - Optional day-count basis (0-4).
///
/// # Errors
/// Returns an error when inputs are out of range or date order is invalid.
#[allow(clippy::too_many_arguments)]
pub fn codcel_odd_f_yield(
    settlement: DateTime<Utc>,
    maturity: DateTime<Utc>,
    issue: DateTime<Utc>,
    first_coupon: DateTime<Utc>,
    rate: f64,
    price: f64,
    redemption: f64,
    frequency: i32,
    basis: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Validate inputs
    if settlement >= maturity {
        return Err("Settlement date must be before maturity date".into());
    }
    if rate < 0.0 || price <= 0.0 || redemption <= 0.0 {
        return Err("Rate, price and redemption must be positive".into());
    }
    if ![1, 2, 4].contains(&frequency) {
        return Err("Frequency must be 1, 2, or 4".into());
    }
    let basis_val = basis.unwrap_or(0);
    if !(0..=4).contains(&basis_val) {
        return Err("Basis must be 0, 1, 2, 3, or 4".into());
    }

    // Helper closure: compute ODDFPRICE at a given yield
    let price_at_yield = |y: f64| -> Result<f64, Box<dyn Error + Send + Sync>> {
        codcel_odd_f_price(
            settlement,
            maturity,
            issue,
            first_coupon,
            rate,
            y,
            redemption,
            frequency,
            basis,
        )
    };

    // Newton-Raphson with numerical derivative
    let mut yield_guess = rate; // Start with coupon rate as initial guess
    if yield_guess <= 0.0 {
        yield_guess = 0.05; // Default guess for zero-coupon
    }

    let max_iterations = 200;
    let tolerance = 1e-12;
    let dy = 1e-7; // Step for numerical derivative

    for _ in 0..max_iterations {
        let p = price_at_yield(yield_guess)?;
        let diff = p - price;

        if diff.abs() < tolerance {
            break;
        }

        // Numerical derivative: dp/dy ≈ (p(y+dy) - p(y-dy)) / (2*dy)
        let p_up = price_at_yield(yield_guess + dy)?;
        let p_down = price_at_yield(yield_guess - dy)?;
        let derivative = (p_up - p_down) / (2.0 * dy);

        if derivative.abs() < 1e-15 {
            break; // Avoid division by near-zero
        }

        yield_guess -= diff / derivative;

        // Clamp to reasonable range
        yield_guess = yield_guess.clamp(-1.0, 10.0);
    }

    Ok(yield_guess)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn dt(y: i32, m: u32, d: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(y, m, d, 0, 0, 0).unwrap()
    }

    #[test]
    fn test_odd_f_yield_basic() {
        let result = codcel_odd_f_yield(
            dt(2022, 1, 1),
            dt(2027, 1, 1),
            dt(2021, 7, 1),
            dt(2022, 7, 1),
            0.05,
            95.0,
            100.0,
            2,
            Some(0),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_odd_f_yield_at_par() {
        // Exact dates from failing test: serial 44635=2022-03-15, 46553=2027-06-15, 44593=2022-02-01, 44727=2022-06-15
        // Rate: 0.08, Price: 100, Redemption: 100, Freq: 2, Basis: 0
        // Expected (Excel): 0.08000057149191912
        let result = codcel_odd_f_yield(
            dt(2022, 3, 15),
            dt(2027, 6, 15),
            dt(2022, 2, 1),
            dt(2022, 6, 15),
            0.08,
            100.0,
            100.0,
            2,
            Some(0),
        )
        .unwrap();

        println!("ODDFYIELD at-par result: {result}");
        assert!(
            (result - 0.08000057149191912).abs() < 0.000001,
            "Expected yield 0.08000057149191912, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_error_cases() {
        // Settlement must be before maturity
        assert!(codcel_odd_f_yield(
            dt(2022, 1, 1),
            dt(2022, 1, 1),
            dt(2021, 7, 1),
            dt(2022, 7, 1),
            0.05,
            95.0,
            100.0,
            2,
            Some(0)
        )
        .is_err());
    }
}
