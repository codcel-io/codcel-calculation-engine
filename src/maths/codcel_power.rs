// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::maths::check_values::check_value_f64;
use std::error::Error;

/// Excel-compatible `POWER` that returns the result of a number raised to a power.
/// - `base`: the base number.
/// - `exponent`: the exponent to which the base is raised.
///
/// Returns base^exponent or an error for invalid inputs (e.g., negative base with fractional exponent).
pub fn codcel_power(base: f64, exponent: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    check_value_f64("POWER", base)?;
    check_value_f64("EXPONENT", exponent)?;

    // Special case: 0^0 is undefined mathematically, Excel returns 1
    if base == 0.0 && exponent == 0.0 {
        return Ok(1.0);
    }

    // Special case: negative base with a fractional exponent is invalid
    if base < 0.0 && exponent.fract() != 0.0 {
        return Err("POWER: Negative base with a fractional exponent is not allowed".into());
    }

    // Calculate the power
    let result = base.powf(exponent);

    Ok(result)
}

/// Convenience wrapper for `POWER` that accepts a two-element vector `[base, exponent]`.
///
/// # Errors
/// Returns an error if the slice is not exactly two numbers long or if `codcel_power`
/// rejects the parsed input.
pub fn codcel_power_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 2 {
        return Err("POWER: Must have 2 parameters".into());
    }

    codcel_power(inputs[0], inputs[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_positive_base_positive_exponent() {
        // =POWER(2,3) in US format
        // =POWER(2;3) in German format
        let result = codcel_power(2.0, 3.0).unwrap();
        assert_eq!(result, 8.0); // 2^3 = 8
    }

    #[test]
    fn test_power_negative_base_integer_exponent() {
        // =POWER(-2,3) in US format
        // =POWER(-2;3) in German format
        let result = codcel_power(-2.0, 3.0).unwrap();
        assert_eq!(result, -8.0); // (-2)^3 = -8
    }

    #[test]
    fn test_power_negative_base_even_exponent() {
        // =POWER(-2,4) in US format
        // =POWER(-2;4) in German format
        let result = codcel_power(-2.0, 4.0).unwrap();
        assert_eq!(result, 16.0); // (-2)^4 = 16
    }

    #[test]
    fn test_power_zero_base_positive_exponent() {
        // =POWER(0,5) in US format
        // =POWER(0;5) in German format
        let result = codcel_power(0.0, 5.0).unwrap();
        assert_eq!(result, 0.0); // 0^5 = 0
    }

    #[test]
    fn test_power_positive_base_zero_exponent() {
        // =POWER(5,0) in US format
        // =POWER(5;0) in German format
        let result = codcel_power(5.0, 0.0).unwrap();
        assert_eq!(result, 1.0); // 5^0 = 1
    }

    #[test]
    fn test_power_zero_base_zero_exponent() {
        // =POWER(0,0) in US format
        // =POWER(0;0) in German format
        let result = codcel_power(0.0, 0.0).unwrap();
        assert_eq!(result, 1.0); // Excel defines 0^0 = 1
    }

    #[test]
    fn test_power_decimal_exponent() {
        // =POWER(4,0.5) in US format
        // =POWER(4;0,5) in German format
        let result = codcel_power(4.0, 0.5).unwrap();
        assert_eq!(result, 2.0); // 4^0.5 = 2
    }

    #[test]
    fn test_power_negative_exponent() {
        // =POWER(2,-2) in US format
        // =POWER(2;-2) in German format
        let result = codcel_power(2.0, -2.0).unwrap();
        assert_eq!(result, 0.25); // 2^(-2) = 1/4 = 0.25
    }

    #[test]
    fn test_power_large_numbers() {
        // =POWER(10,10) in US format
        // =POWER(10;10) in German format
        let result = codcel_power(10.0, 10.0).unwrap();
        assert_eq!(result, 10000000000.0); // 10^10 = 10,000,000,000
    }

    #[test]
    fn test_power_negative_base_fractional_exponent() {
        // =POWER(-4,0.5) in US format (returns #NUM! error)
        // =POWER(-4;0,5) in German format (returns #NUM! error)
        let result = codcel_power(-4.0, 0.5);
        assert!(result.is_err());
    }

    #[test]
    fn test_power_vec_valid() {
        // =POWER(2,3) in US format
        // =POWER(2;3) in German format
        let result = codcel_power_vec(vec![2.0, 3.0]).unwrap();
        assert_eq!(result, 8.0); // 2^3 = 8
    }

    #[test]
    fn test_power_vec_invalid_length() {
        // =POWER(2) in US format (returns #VALUE! error)
        // =POWER(2) in German format (returns #VALUE! error)
        let result = codcel_power_vec(vec![2.0]);
        assert!(result.is_err());
    }
}
