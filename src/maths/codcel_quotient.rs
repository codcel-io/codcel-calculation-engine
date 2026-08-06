// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::maths::check_values::check_value_f64;
use std::error::Error;

/// Excel-compatible `QUOTIENT` that returns the integer portion of a division.
/// - `numerator`: the dividend.
/// - `denominator`: the divisor.
///
/// Returns the integer quotient or an error for division by zero.
pub fn codcel_quotient(
    numerator: f64,
    denominator: f64,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    check_value_f64("QUOTIENT", numerator)?;
    check_value_f64("QUOTIENT", denominator)?;

    // Division by zero check
    if denominator == 0.0 {
        return Err("QUOTIENT: Division by zero is not allowed".into());
    }

    // Perform integer division
    let result = (numerator / denominator).trunc() as i32;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quotient_positive_numbers() {
        // =QUOTIENT(10, 3) in US format
        // =QUOTIENT(10; 3) in German format
        let result = codcel_quotient(10.0, 3.0).unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_quotient_negative_numerator() {
        // =QUOTIENT(-10, 3) in US format
        // =QUOTIENT(-10; 3) in German format
        let result = codcel_quotient(-10.0, 3.0).unwrap();
        assert_eq!(result, -3);
    }

    #[test]
    fn test_quotient_negative_denominator() {
        // =QUOTIENT(10, -3) in US format
        // =QUOTIENT(10; -3) in German format
        let result = codcel_quotient(10.0, -3.0).unwrap();
        assert_eq!(result, -3);
    }

    #[test]
    fn test_quotient_both_negative() {
        // =QUOTIENT(-10, -3) in US format
        // =QUOTIENT(-10; -3) in German format
        let result = codcel_quotient(-10.0, -3.0).unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_quotient_decimal_numbers() {
        // =QUOTIENT(5.5, 2.5) in US format
        // =QUOTIENT(5,5; 2,5) in German format
        let result = codcel_quotient(5.5, 2.5).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_quotient_zero_numerator() {
        // =QUOTIENT(0, 5) in US format
        // =QUOTIENT(0; 5) in German format
        let result = codcel_quotient(0.0, 5.0).unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_quotient_large_numbers() {
        // =QUOTIENT(1000000, 3) in US format
        // =QUOTIENT(1000000; 3) in German format
        let result = codcel_quotient(1000000.0, 3.0).unwrap();
        assert_eq!(result, 333333);
    }

    #[test]
    fn test_quotient_division_by_zero() {
        // =QUOTIENT(10, 0) in US format - should return an error
        // =QUOTIENT(10; 0) in German format - should return an error
        let result = codcel_quotient(10.0, 0.0);
        assert!(result.is_err());
    }
}
