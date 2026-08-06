// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::maths::check_values::check_value_f64;
use std::error::Error;

/// Excel-compatible `ISEVEN` that checks whether a number is even.
/// - `value`: the numeric value to test; decimals are truncated before the check.
///
/// Returns `true` if the integer portion of the value is even, `false` otherwise.
/// Returns an error if the value fails numeric validation.
pub fn codcel_is_even(value: f64) -> Result<bool, Box<dyn Error + Send + Sync>> {
    check_value_f64("ISEVEN", value)?;

    // Check if the number is zero (which is even)
    if value == 0.0 {
        return Ok(true);
    }

    // Convert to integer and check if it's even
    let int_value = value.trunc() as i64;

    // A number is even if it has a remainder of 0 when divided by 2
    Ok(int_value % 2 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_even_positive_integer_even() {
        // =ISEVEN(4) in Excel
        let result = codcel_is_even(4.0).unwrap();
        assert!(result);
    }

    #[test]
    fn test_is_even_positive_integer_odd() {
        // =ISEVEN(3) in Excel
        let result = codcel_is_even(3.0).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_is_even_negative_integer_even() {
        // =ISEVEN(-4) in Excel
        let result = codcel_is_even(-4.0).unwrap();
        assert!(result);
    }

    #[test]
    fn test_is_even_negative_integer_odd() {
        // =ISEVEN(-3) in Excel
        let result = codcel_is_even(-3.0).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_is_even_positive_decimal() {
        // =ISEVEN(2.5) in Excel
        // Excel ISEVEN only considers the integer part
        let result = codcel_is_even(2.5).unwrap();
        assert!(result);
    }

    #[test]
    fn test_is_even_negative_decimal() {
        // =ISEVEN(-2.5) in Excel
        // Excel ISEVEN only considers the integer part
        let result = codcel_is_even(-2.5).unwrap();
        assert!(result);
    }

    #[test]
    fn test_is_even_zero() {
        // =ISEVEN(0) in Excel
        let result = codcel_is_even(0.0).unwrap();
        assert!(result);
    }

    #[test]
    fn test_is_even_small_positive() {
        // =ISEVEN(0.1) in Excel
        // Excel ISEVEN only considers the integer part
        let result = codcel_is_even(0.1).unwrap();
        assert!(result);
    }

    #[test]
    fn test_is_even_small_negative() {
        // =ISEVEN(-0.1) in Excel
        // Excel ISEVEN only considers the integer part
        let result = codcel_is_even(-0.1).unwrap();
        assert!(result);
    }
}
