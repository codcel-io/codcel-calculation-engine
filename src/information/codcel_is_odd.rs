// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::maths::check_values::check_value_f64;
use std::error::Error;

/// Excel-compatible `ISODD` that checks whether a number is odd.
/// - `value`: the numeric value to test; decimals are truncated before the check.
///
/// Returns `true` if the integer portion of the value is odd, `false` otherwise.
/// Returns an error if the value fails numeric validation.
pub fn codcel_is_odd(value: f64) -> Result<bool, Box<dyn Error + Send + Sync>> {
    check_value_f64("ISODD", value)?;

    // Check if the number is zero (which is even)
    if value == 0.0 {
        return Ok(false);
    }

    // Convert to integer and check if it's odd
    let int_value = value.trunc() as i64;

    // A number is odd if it has a remainder of 1 or -1 when divided by 2
    Ok(int_value % 2 != 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_odd_positive_integer_odd() {
        // =ISODD(3) in Excel
        let result = codcel_is_odd(3.0).unwrap();
        assert!(result);
    }

    #[test]
    fn test_is_odd_positive_integer_even() {
        // =ISODD(4) in Excel
        let result = codcel_is_odd(4.0).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_is_odd_negative_integer_odd() {
        // =ISODD(-3) in Excel
        let result = codcel_is_odd(-3.0).unwrap();
        assert!(result);
    }

    #[test]
    fn test_is_odd_negative_integer_even() {
        // =ISODD(-4) in Excel
        let result = codcel_is_odd(-4.0).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_is_odd_positive_decimal() {
        // =ISODD(2.5) in Excel
        // Excel ISODD only considers the integer part
        let result = codcel_is_odd(2.5).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_is_odd_negative_decimal() {
        // =ISODD(-2.5) in Excel
        // Excel ISODD only considers the integer part
        let result = codcel_is_odd(-2.5).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_is_odd_zero() {
        // =ISODD(0) in Excel
        let result = codcel_is_odd(0.0).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_is_odd_small_positive() {
        // =ISODD(0.1) in Excel
        // Excel ISODD only considers the integer part
        let result = codcel_is_odd(0.1).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_is_odd_small_negative() {
        // =ISODD(-0.1) in Excel
        // Excel ISODD only considers the integer part
        let result = codcel_is_odd(-0.1).unwrap();
        assert!(!result);
    }
}
