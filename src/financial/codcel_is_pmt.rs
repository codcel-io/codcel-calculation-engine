// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Calculates the interest payment for a given period of an investment with equal principal payments.
///
/// # Arguments
/// * `rate` - The interest rate per period.
/// * `per` - The period for which to calculate the interest.
/// * `nper` - The total number of payment periods.
/// * `pv` - The present value of the investment.
///
/// # Returns
/// The interest payment for the specified period.
pub fn codcel_is_pmt(
    rate: f64,
    per: f64,
    nper: f64,
    pv: f64,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if nper == 0.0 {
        return Err("Number of periods cannot be zero".into());
    }

    let payment = -pv * rate * (nper - per) / nper;
    Ok(payment)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_pmt_basic() {
        // Test with a simple case
        let result = codcel_is_pmt(0.1, 1.0, 3.0, 1000.0).unwrap();
        assert!((result - (-66.67)).abs() < 0.01);

        // Test with a different period
        let result = codcel_is_pmt(0.1, 2.0, 3.0, 1000.0).unwrap();
        assert!((result - (-33.33)).abs() < 0.01);
    }

    #[test]
    fn test_is_pmt_fractional() {
        // Fractional per: ISPMT(0.05, 1.5, 12, 10000) = -437.5
        let result = codcel_is_pmt(0.05, 1.5, 12.0, 10000.0).unwrap();
        assert!((result - (-437.5)).abs() < 0.000001);

        // Fractional nper: ISPMT(0.05, 1, 12.5, 10000) = -460.0
        let result = codcel_is_pmt(0.05, 1.0, 12.5, 10000.0).unwrap();
        assert!((result - (-460.0)).abs() < 0.000001);

        // Both fractional: ISPMT(0.05, 1.5, 12.5, 10000) = -440.0
        let result = codcel_is_pmt(0.05, 1.5, 12.5, 10000.0).unwrap();
        assert!((result - (-440.0)).abs() < 0.000001);
    }

    #[test]
    fn test_is_pmt_error_cases() {
        // Number of periods cannot be zero
        assert!(codcel_is_pmt(0.1, 1.0, 0.0, 1000.0).is_err());
    }
}
