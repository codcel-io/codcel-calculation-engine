// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::maths::check_values::check_value_f64;
use std::error::Error;

/// Excel-compatible `ODD` that rounds a number up to the nearest odd integer.
/// - `num`: the value to round.
///
/// Returns the nearest odd integer away from zero.
pub fn codcel_odd(num: f64) -> Result<i32, Box<dyn Error + Send + Sync>> {
    check_value_f64("ODD", num)?;

    // Check if the number is zero; zero cannot be odd
    if num == 0.0 {
        return Ok(1);
    }

    // Round the number away from zero
    let mut result = if num > 0.0 {
        num.ceil() as i32
    } else {
        num.floor() as i32
    };

    // If the result is even, adjust to make it odd
    if result % 2 == 0 {
        result += if num > 0.0 { 1 } else { -1 };
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_odd_positive_integer_already_odd() {
        // =ODD(3) in US format
        // =ODD(3) in German format
        let result = codcel_odd(3.0).unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_odd_positive_integer_even() {
        // =ODD(4) in US format
        // =ODD(4) in German format
        let result = codcel_odd(4.0).unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn test_odd_negative_integer_already_odd() {
        // =ODD(-3) in US format
        // =ODD(-3) in German format
        let result = codcel_odd(-3.0).unwrap();
        assert_eq!(result, -3);
    }

    #[test]
    fn test_odd_negative_integer_even() {
        // =ODD(-4) in US format
        // =ODD(-4) in German format
        let result = codcel_odd(-4.0).unwrap();
        assert_eq!(result, -5);
    }

    #[test]
    fn test_odd_positive_decimal_rounds_up() {
        // =ODD(2.5) in US format
        // =ODD(2,5) in German format
        let result = codcel_odd(2.5).unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_odd_negative_decimal_rounds_down() {
        // =ODD(-2.5) in US format
        // =ODD(-2,5) in German format
        let result = codcel_odd(-2.5).unwrap();
        assert_eq!(result, -3);
    }

    #[test]
    fn test_odd_zero() {
        // =ODD(0) in US format
        // =ODD(0) in German format
        let result = codcel_odd(0.0).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_odd_small_positive() {
        // =ODD(0.1) in US format
        // =ODD(0,1) in German format
        let result = codcel_odd(0.1).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_odd_small_negative() {
        // =ODD(-0.1) in US format
        // =ODD(-0,1) in German format
        let result = codcel_odd(-0.1).unwrap();
        assert_eq!(result, -1);
    }
}
