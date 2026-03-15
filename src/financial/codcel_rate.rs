// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Calculate the interest rate per period of an annuity.
///
/// # Arguments
/// * `nper` - The total number of payment periods.
/// * `pmt` - The payment made each period.
/// * `pv` - The present value.
/// * `fv` - The future value (optional, defaults to 0).
/// * `type_` - When payments are due: 0 for end of period, 1 for beginning of period (optional, defaults to 0).
/// * `guess` - Your guess for what the rate will be (optional, defaults to 0.1).
///
/// # Returns
/// The interest rate per period.
pub fn codcel_rate(
    nper: f64,
    pmt: f64,
    pv: f64,
    fv: Option<f64>,
    type_: Option<i32>,
    guess: Option<f64>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Set default values for optional parameters
    let fv = fv.unwrap_or(0.0);
    let type_ = type_.unwrap_or(0);
    let guess = guess.unwrap_or(0.1); // 10% initial guess

    if nper <= 0.0 {
        return Err("RATE: Number of periods must be positive".into());
    }

    // Validate type parameter
    if type_ != 0 && type_ != 1 {
        return Err("RATE: Type must be 0 or 1".into());
    }

    const TOL: f64 = 1e-10;
    const MAX_ITERATIONS: usize = 128;

    let w = type_ as f64;

    // f(rate) = fv + pv*(1+rate)^nper + pmt*(1+rate*w)*((1+rate)^nper - 1)/rate
    let f_val = |r: f64| -> f64 {
        if r.abs() < 1e-12 {
            fv + pv * (1.0 + nper * r) + pmt * (1.0 + r * w) * nper
        } else {
            let tmp = (1.0 + r).powf(nper);
            fv + pv * tmp + pmt * (1.0 + r * w) * (tmp - 1.0) / r
        }
    };

    // Exact derivative of f with respect to rate
    let f_deriv = |r: f64| -> f64 {
        if r.abs() < 1e-12 {
            pv * nper + pmt * w * nper + pmt * nper * (nper - 1.0) / 2.0
        } else {
            let tmp = (1.0 + r).powf(nper);
            let dtmp = nper * (1.0 + r).powf(nper - 1.0);
            let d_pv = pv * dtmp;
            // Product rule: d/dr [pmt * (1+r*w) * ((1+r)^n - 1) / r]
            let u = 1.0 + r * w;
            let v = (tmp - 1.0) / r;
            let v_prime = (dtmp * r - (tmp - 1.0)) / (r * r);
            let d_pmt = pmt * (w * v + u * v_prime);
            d_pv + d_pmt
        }
    };

    let mut rate = guess;

    for _ in 0..MAX_ITERATIONS {
        let y = f_val(rate);
        if y.abs() < TOL {
            return Ok(rate);
        }

        let dy = f_deriv(rate);
        if dy.abs() < 1e-14 {
            return Err("RATE: Failed to converge to a solution.".into());
        }

        let mut new_rate = rate - y / dy;

        // Prevent overshooting past the singularity at rate = -1
        if new_rate < -1.0 {
            new_rate = (rate - 1.0) / 2.0;
        }

        // Damp excessively large steps to improve stability
        let step = new_rate - rate;
        if step.abs() > 1.0 {
            new_rate = rate + step.signum() * step.abs().min(1.0);
        }

        if (new_rate - rate).abs() < TOL {
            return Ok(new_rate);
        }

        rate = new_rate;
    }

    Err("RATE: Failed to converge to a solution.".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_basic() {
        let result = codcel_rate(10.0, -100.0, 800.0, None, None, None).unwrap();
        assert!((result - 0.05).abs() < 0.01);
    }

    #[test]
    fn test_rate_with_future_value() {
        let result = codcel_rate(10.0, -100.0, 800.0, Some(100.0), None, None).unwrap();
        assert!(result > 0.0);
    }

    #[test]
    fn test_rate_quick_payoff() {
        // RATE(3, -5000, 12000)
        let result = codcel_rate(3.0, -5000.0, 12000.0, None, None, None).unwrap();
        assert!((result - 0.1204439829770753).abs() < 1e-6);
    }

    #[test]
    fn test_rate_short_term_fv() {
        // RATE(4, -3000, 10000, -1000)
        let result = codcel_rate(4.0, -3000.0, 10000.0, Some(-1000.0), None, None).unwrap();
        assert!((result - 0.10847887169328899).abs() < 1e-6);
    }

    #[test]
    fn test_rate_all_six_args() {
        // RATE(5, -2500, 10000, 0, 1, 0.05)
        let result = codcel_rate(5.0, -2500.0, 10000.0, Some(0.0), Some(1), Some(0.05)).unwrap();
        assert!((result - 0.12589832496242429).abs() < 1e-6);
    }

    #[test]
    fn test_rate_two_periods_high_guess() {
        // RATE(2, -600, 1000, 0, 0, 0.5)
        let result = codcel_rate(2.0, -600.0, 1000.0, Some(0.0), Some(0), Some(0.5)).unwrap();
        assert!((result - 0.1306623862918077).abs() < 1e-6);
    }

    #[test]
    fn test_rate_error_cases() {
        let result = codcel_rate(-10.0, -100.0, 800.0, None, None, None);
        assert!(result.is_err());

        let result = codcel_rate(10.0, -100.0, 800.0, None, Some(2), None);
        assert!(result.is_err());
    }
}
