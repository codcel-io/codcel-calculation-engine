// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Helper function to calculate future value for IPMT
fn ipmt_future_value(rate: f64, nper: i32, pmt: f64, pv: f64, type_: i32) -> f64 {
    if rate == 0.0 {
        return -pv - pmt * nper as f64;
    }
    let term = (1.0 + rate).powi(nper);
    pmt * (if type_ == 1 { 1.0 + rate } else { 1.0 }) * (1.0 - term) / rate - pv * term
}

/// Calculates the interest payment for a given period of an investment based on periodic, constant payments and a constant interest rate.
///
/// # Arguments
/// * `rate` - The interest rate per period.
/// * `per` - The period for which to calculate the interest, must be between 1 and nper.
/// * `nper` - The total number of payment periods.
/// * `pv` - The present value of the investment.
/// * `fv` - The future value of the investment (optional, defaults to 0).
/// * `type_` - When payments are due (0 = end of period, 1 = beginning of period, optional, defaults to 0).
///
/// # Returns
/// The interest payment for the specified period.
pub fn codcel_i_pmt(
    rate: f64,
    per: i32,
    nper: i32,
    pv: f64,
    fv: Option<f64>,
    type_: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Validate input parameters
    if rate <= -1.0 {
        return Err("IPMT: Rate must be greater than -1".into());
    }
    if per < 1 || per > nper {
        return Err("IPMT: Period must be between 1 and total number of periods".into());
    }
    if nper <= 0 {
        return Err("IPMT: Number of periods must be positive".into());
    }

    // Set default values for optional parameters
    let fv = fv.unwrap_or(0.0);
    let type_ = type_.unwrap_or(0);

    if type_ != 0 && type_ != 1 {
        return Err("IPMT: Type must be 0 or 1".into());
    }

    // Get the payment amount first
    let pmt = if rate == 0.0 {
        (-pv - fv) / nper as f64
    } else {
        let term = (1.0 + rate).powi(nper);
        (fv * rate + pv * rate * term) * (if type_ == 1 { 1.0 / (1.0 + rate) } else { 1.0 })
            / (1.0 - term)
    };

    // Special case for first period
    if per == 1 {
        return Ok(if type_ == 1 { 0.0 } else { rate * -pv });
    }

    // Calculate the balance using future value function and then apply rate
    let balance = if type_ == 1 {
        ipmt_future_value(rate, per - 2, pmt, pv, type_) - pmt
    } else {
        ipmt_future_value(rate, per - 1, pmt, pv, type_)
    };

    Ok(rate * balance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i_pmt_basic() {
        // Test with end-of-period payments (type = 0)
        let result = codcel_i_pmt(0.1, 2, 3, 1000.0, None, Some(0)).unwrap();
        assert!(result < 0.0); // Interest payment should be negative

        // Test with beginning-of-period payments (type = 1)
        let result = codcel_i_pmt(0.1, 2, 3, 1000.0, None, Some(1)).unwrap();
        assert!(result < 0.0); // Interest payment should be negative
    }

    #[test]
    fn test_i_pmt_first_period() {
        // For first period with type = 1, interest should be 0
        let result = codcel_i_pmt(0.1, 1, 3, 1000.0, None, Some(1)).unwrap();
        assert_eq!(result, 0.0);

        // For first period with type = 0, interest should be rate * -pv
        let result = codcel_i_pmt(0.1, 1, 3, 1000.0, None, Some(0)).unwrap();
        assert_eq!(result, 0.1 * -1000.0);
    }

    #[test]
    fn test_i_pmt_error_cases() {
        // Rate must be greater than -1
        assert!(codcel_i_pmt(-1.0, 1, 3, 1000.0, None, None).is_err());

        // Period must be between 1 and nper
        assert!(codcel_i_pmt(0.1, 0, 3, 1000.0, None, None).is_err());
        assert!(codcel_i_pmt(0.1, 4, 3, 1000.0, None, None).is_err());

        // Number of periods must be positive
        assert!(codcel_i_pmt(0.1, 1, 0, 1000.0, None, None).is_err());

        // Type must be 0 or 1
        assert!(codcel_i_pmt(0.1, 1, 3, 1000.0, None, Some(2)).is_err());
    }
}
