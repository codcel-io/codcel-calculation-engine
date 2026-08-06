// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Calculates the number of periods for an investment based on constant-amount periodic payments and a constant interest rate.
///
/// # Arguments
/// * `rate` - The interest rate per period.
/// * `pmt` - The payment made each period.
/// * `pv` - The present value of the investment.
/// * `fv` - The future value of the investment (optional, defaults to 0).
/// * `type_` - When payments are due (0 = end of period, 1 = beginning of period, optional, defaults to 0).
///
/// # Returns
/// The number of periods for the investment.
pub fn codcel_n_per(
    rate: f64,
    pmt: f64,
    pv: f64,
    fv: Option<f64>,
    type_: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if rate == 0.0 {
        // When rate is 0, use simple division formula
        let fv_value = fv.unwrap_or(0.0);
        return Ok(-(pv + fv_value) / pmt);
    }

    let type_value = type_.unwrap_or(0);
    if type_value != 0 && type_value != 1 {
        return Err("NPER: Type must be 0 or 1".into());
    }

    let fv_value = fv.unwrap_or(0.0);

    // Adjust payment based on payment timing
    let pmt_adjusted = if type_value == 1 {
        pmt * (1.0 + rate)
    } else {
        pmt
    };

    // Calculate NPER using the formula:
    // NPER = ln((PMT*(1+rate*type) - FV*rate)/(PMT*(1+rate*type) + PV*rate))/ln(1+rate)
    let numerator = pmt_adjusted - fv_value * rate;
    let denominator = pmt_adjusted + pv * rate;

    if denominator == 0.0 {
        return Err("NPER: Invalid parameters: denominator is zero".into());
    }

    let ratio = numerator / denominator;
    if ratio <= 0.0 {
        return Err(
            "NPER: Invalid parameters: cannot calculate logarithm of non-positive number".into(),
        );
    }

    Ok(crate::portable_math::ln(ratio) / crate::portable_math::ln(1.0 + rate))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n_per_with_future_value() {
        let result = codcel_n_per(0.08, -200.0, 1000.0, Some(10000.0), None).unwrap();
        assert!(result > 0.0);
    }

    #[test]
    fn test_n_per_with_payment_at_beginning() {
        let result = codcel_n_per(0.08, -200.0, 1000.0, None, Some(1)).unwrap();
        assert!(result > 0.0);
    }

    #[test]
    fn test_n_per_error_cases() {
        // Type must be 0 or 1
        assert!(codcel_n_per(0.08, -200.0, 1000.0, None, Some(2)).is_err());
    }
}
