// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Calculate the present value of an investment.
///
/// # Arguments
/// * `rate` - The interest rate per period.
/// * `nper` - The total number of payment periods.
/// * `pmt` - The payment made each period.
/// * `fv` - The future value, or a cash balance you want to attain after the last payment is made (optional, defaults to 0).
/// * `type_` - When payments are due: 0 for end of period, 1 for beginning of period (optional, defaults to 0).
///
/// # Returns
/// The present value of the investment.
pub fn codcel_pv(
    rate: f64,
    nper: f64,
    pmt: f64,
    fv: Option<f64>,
    type_: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let fv = fv.unwrap_or(0.0);
    let type_ = type_.unwrap_or(0);

    if type_ != 0 && type_ != 1 {
        return Err("PV: Type must be 0 (end of period) or 1 (beginning of period).".into());
    }

    if rate == 0.0 {
        // When the interest rate is 0, the present value is the sum of the payments and the future value.
        return Ok(-(pmt * nper + fv));
    }

    let rate_per_period = rate;
    let discount_factor = crate::portable_math::powf(1.0 + rate_per_period, -nper);

    // PV formula derived from financial equations
    let present_value = -(pmt * (1.0 + rate_per_period * type_ as f64) * (1.0 - discount_factor)
        / rate_per_period
        + fv * discount_factor);

    Ok(present_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pv_basic() {
        // Test with rate = 5%, nper = 10, pmt = -100, fv = 0, type = 0
        let result = codcel_pv(0.05, 10.0, -100.0, None, None).unwrap();
        assert!(result > 0.0); // Present value should be positive when payment is negative

        // Test with rate = 0%, nper = 10, pmt = -100, fv = 0, type = 0
        let result = codcel_pv(0.0, 10.0, -100.0, None, None).unwrap();
        assert_eq!(result, 1000.0); // With 0% rate, PV is just the sum of payments
    }

    #[test]
    fn test_pv_with_future_value() {
        // Test with rate = 5%, nper = 10, pmt = -100, fv = 1000, type = 0
        let result = codcel_pv(0.05, 10.0, -100.0, Some(1000.0), None).unwrap();
        assert!(result > 0.0);
    }

    #[test]
    fn test_pv_beginning_of_period() {
        // Test with rate = 5%, nper = 10, pmt = -100, fv = 0, type = 1 (beginning of period)
        let result = codcel_pv(0.05, 10.0, -100.0, None, Some(1)).unwrap();
        let result_end = codcel_pv(0.05, 10.0, -100.0, None, Some(0)).unwrap();
        assert!(result > result_end); // PV should be higher when payments are at beginning of period
    }

    #[test]
    fn test_pv_error_cases() {
        // Test with invalid type
        let result = codcel_pv(0.05, 10.0, -100.0, None, Some(2));
        assert!(result.is_err());
    }
}
