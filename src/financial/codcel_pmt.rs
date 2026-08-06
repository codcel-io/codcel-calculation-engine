// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Calculate the payment for an annuity based on constant payments and a constant interest rate.
///
/// # Arguments
/// - `rate`: The interest rate per period.
/// - `nper`: The number of payment periods.
/// - `pv`: The present value (the amount of money today).
/// - `fv`: The future value (optional, default is 0.0, final balance after last payment).
/// - `type_`: Payment type (0 = end of period, 1 = beginning of period, default is 0).
///
/// # Returns
/// - Returns the payment amount as `f64` or an error if invalid arguments are provided.
///
/// # Errors
/// - Returns an error if `nper` is less than or equal to 0.
///
/// This function replicates the behavior of the Excel `PMT` function.
pub fn codcel_pmt(
    rate: f64,
    nper: f64,
    pv: f64,
    fv: Option<f64>,
    type_: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if nper <= 0.0 {
        return Err("PMT: Number of periods (nper) must be greater than 0.".into());
    }

    let fv = fv.unwrap_or(0.0);
    let type_ = type_.unwrap_or(0);

    if type_ != 0 && type_ != 1 {
        return Err("PMT: Type must be 0 (end of period) or 1 (beginning of period).".into());
    }

    if rate == 0.0 {
        // If the interest rate is 0, the payment is simply the principal + future value divided by periods.
        return Ok(-(pv + fv) / nper);
    }

    let rate_per_period = rate;
    let discount_factor = crate::portable_math::powf(1.0 + rate_per_period, nper);

    // PMT formula derived from the financial formula for present value of an annuity
    let payment = -rate_per_period * (pv * discount_factor + fv)
        / ((1.0 + rate_per_period * type_ as f64) * (discount_factor - 1.0));

    Ok(payment)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pmt_errors() {
        // Test with nper <= 0
        assert!(codcel_pmt(0.01, 0.0, 1000.0, None, None).is_err());
        assert!(codcel_pmt(0.01, -12.0, 1000.0, None, None).is_err());

        // Test with invalid type
        assert!(codcel_pmt(0.01, 12.0, 1000.0, None, Some(2)).is_err());
        assert!(codcel_pmt(0.01, 12.0, 1000.0, None, Some(-1)).is_err());
    }
}
