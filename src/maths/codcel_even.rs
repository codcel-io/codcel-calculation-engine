// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::maths::check_values::check_value_f64;
use std::error::Error;

/// Excel-compatible `EVEN` that rounds a number up to the nearest even integer.
/// - `number`: the value to round.
///
/// Returns the nearest even integer away from zero or an error for NaN/infinite inputs.
pub fn codcel_even(number: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    check_value_f64("EVEN", number)?;

    if number == 0.0 {
        return Ok(0.0);
    }

    let abs_number = number.abs().ceil();
    let even = if abs_number % 2.0 == 0.0 {
        abs_number
    } else {
        abs_number + 1.0
    };

    Ok(if number > 0.0 { even } else { -even })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even_positive_integer_already_even() {
        // =EVEN(4) in US format
        // =EVEN(4) in German format
        let result = codcel_even(4.0).unwrap();
        assert_eq!(result, 4.0);
    }

    #[test]
    fn test_even_positive_integer_odd() {
        // =EVEN(3) in US format
        // =EVEN(3) in German format
        let result = codcel_even(3.0).unwrap();
        assert_eq!(result, 4.0);
    }

    #[test]
    fn test_even_negative_integer_already_even() {
        // =EVEN(-4) in US format
        // =EVEN(-4) in German format
        let result = codcel_even(-4.0).unwrap();
        assert_eq!(result, -4.0);
    }

    #[test]
    fn test_even_negative_integer_odd() {
        // =EVEN(-3) in US format
        // =EVEN(-3) in German format
        let result = codcel_even(-3.0).unwrap();
        assert_eq!(result, -4.0);
    }

    #[test]
    fn test_even_positive_decimal_rounds_up() {
        // =EVEN(2.5) in US format
        // =EVEN(2,5) in German format
        let result = codcel_even(2.5).unwrap();
        assert_eq!(result, 4.0);
    }

    #[test]
    fn test_even_negative_decimal_rounds_down() {
        // =EVEN(-2.5) in US format
        // =EVEN(-2,5) in German format
        let result = codcel_even(-2.5).unwrap();
        assert_eq!(result, -4.0);
    }

    #[test]
    fn test_even_zero() {
        // =EVEN(0) in US format
        // =EVEN(0) in German format
        let result = codcel_even(0.0).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_even_small_positive() {
        // =EVEN(0.1) in US format
        // =EVEN(0,1) in German format
        let result = codcel_even(0.1).unwrap();
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_even_small_negative() {
        // =EVEN(-0.1) in US format
        // =EVEN(-0,1) in German format
        let result = codcel_even(-0.1).unwrap();
        assert_eq!(result, -2.0);
    }
}
