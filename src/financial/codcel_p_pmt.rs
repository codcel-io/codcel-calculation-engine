// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::financial::codcel_i_pmt::codcel_i_pmt;
use crate::financial::codcel_pmt::codcel_pmt;
use std::error::Error;

/// Calculate the principal payment for a given period of an annuity.
///
/// # Arguments
/// - `rate`: The interest rate per period.
/// - `per`: The period for which to calculate the principal payment.
/// - `nper`: The total number of payment periods.
/// - `pv`: The present value (the amount of money today).
/// - `fv`: The future value (optional, default is 0.0, final balance after last payment).
/// - `type_`: Payment type (0 = end of period, 1 = beginning of period, default is 0).
///
/// # Returns
/// - Returns the principal payment for the specified period as `f64` or an error if invalid arguments are provided.
pub fn codcel_p_pmt(
    rate: f64,
    per: i32,
    nper: f64,
    pv: f64,
    fv: Option<f64>,
    type_: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let pmt = codcel_pmt(rate, nper, pv, fv, type_)?;
    let i_pmt = codcel_i_pmt(rate, per, nper as i32, pv, fv, type_)?;

    Ok(pmt - i_pmt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p_pmt_basic() {
        // Test with rate = 0.01, per = 1, nper = 12, pv = 1000
        let result = codcel_p_pmt(0.01, 1, 12.0, 1000.0, None, None).unwrap();

        // First payment should have less principal than later payments
        assert!(result < 0.0); // Principal payment should be negative

        // Test with different period
        let result1 = codcel_p_pmt(0.01, 1, 12.0, 1000.0, None, None).unwrap();
        let result12 = codcel_p_pmt(0.01, 12, 12.0, 1000.0, None, None).unwrap();

        // Last payment should have more principal than first payment
        assert!(result12.abs() > result1.abs());
    }

    #[test]
    fn test_p_pmt_with_options() {
        // Test with future value
        let result = codcel_p_pmt(0.01, 1, 12.0, 1000.0, Some(500.0), None).unwrap();
        assert!(result < 0.0);

        // Test with beginning of period payment
        let result = codcel_p_pmt(0.01, 1, 12.0, 1000.0, None, Some(1)).unwrap();
        assert!(result < 0.0);
    }
}
