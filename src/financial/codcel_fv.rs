// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Calculates the future value of an investment based on periodic, constant payments and a constant interest rate.
///
/// # Arguments
/// * `rate` - The interest rate per period.
/// * `nper` - The total number of payment periods.
/// * `pmt` - The payment made each period.
/// * `pv` - The present value (optional, defaults to 0).
/// * `type_` - When payments are due: 0 = end of period, 1 = beginning of period (optional, defaults to 0).
///
/// # Returns
/// The future value of the investment.
pub fn codcel_fv(
    rate: f64,
    nper: f64,
    pmt: f64,
    pv: Option<f64>,
    type_: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let pv = pv.unwrap_or(0.0);
    let type_ = type_.unwrap_or(0);

    if type_ != 0 && type_ != 1 {
        return Err("FV: Type must be 0 (end of period) or 1 (beginning of period)".into());
    }

    let fv = if rate == 0.0 {
        -(pv + pmt * nper)
    } else {
        let factor = crate::portable_math::powf(1.0 + rate, nper);
        if type_ == 0 {
            -(pv * factor + pmt * (factor - 1.0) / rate)
        } else {
            -(pv * factor + pmt * (factor - 1.0) / rate * (1.0 + rate))
        }
    };

    Ok(fv)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fv_basic() {
        // Example: $100 payment at 5% for 10 periods
        let result = codcel_fv(0.05, 10.0, -100.0, None, None).unwrap();
        assert!((result - 1257.79).abs() < 0.01); // Should be approximately $1,257.79
    }

    #[test]
    fn test_fv_beginning_of_period() {
        // Example: $100 payment at 5% for 10 periods, beginning of period
        let result = codcel_fv(0.05, 10.0, -100.0, None, Some(1)).unwrap();
        assert!((result - 1320.68).abs() < 0.01); // Should be approximately $1,320.68
    }

    #[test]
    fn test_fv_zero_rate() {
        // Example: $100 payment at 0% for 10 periods
        let result = codcel_fv(0.0, 10.0, -100.0, None, None).unwrap();
        assert!((result - 1000.0).abs() < 0.01); // Should be exactly $1,000
    }

    #[test]
    fn test_fv_fractional_nper() {
        // Example: $100 payment at 5% for 6.5 periods
        let result = codcel_fv(0.05, 6.5, -100.0, None, None).unwrap();
        assert!((result - 746.3788102322933).abs() < 0.000001);
    }

    #[test]
    fn test_fv_error_cases() {
        // Type must be 0 or 1
        assert!(codcel_fv(0.05, 10.0, 100.0, None, Some(2)).is_err());
    }
}
