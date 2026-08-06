// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `ROUNDDOWN` that rounds a number down toward zero.
/// - `value`: the number to round.
/// - `decimal_places`: the number of digits to round to.
///
/// Returns the rounded value (always rounds toward zero).
pub fn codcel_round_down(
    value: f64,
    decimal_places: i32,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let multiplier = 10_f64.powi(decimal_places);
    let rounded = (value * multiplier).trunc() / multiplier;
    Ok(rounded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_down_positive() {
        // =ROUNDDOWN(3.14159,2) in US format
        // =ROUNDDOWN(3,14159;2) in German format
        let result = codcel_round_down(3.14159, 2).unwrap();
        assert_eq!(result, 3.14); // 3.14159 rounds down to 3.14 with 2 decimal places
    }

    #[test]
    fn test_round_down_negative() {
        // =ROUNDDOWN(-3.14159,2) in US format
        // =ROUNDDOWN(-3,14159;2) in German format
        let result = codcel_round_down(-3.14159, 2).unwrap();
        assert_eq!(result, -3.14); // -3.14159 rounds toward zero to -3.14 with 2 decimal places
    }

    #[test]
    fn test_round_down_zero_decimal_places() {
        // =ROUNDDOWN(3.99,0) in US format
        // =ROUNDDOWN(3,99;0) in German format
        let result = codcel_round_down(3.99, 0).unwrap();
        assert_eq!(result, 3.0); // 3.99 rounds down to 3 with 0 decimal places
    }

    #[test]
    fn test_round_down_negative_decimal_places() {
        // =ROUNDDOWN(1234.5678,-2) in US format
        // =ROUNDDOWN(1234,5678;-2) in German format
        let result = codcel_round_down(1234.5678, -2).unwrap();
        assert_eq!(result, 1200.0); // 1234.5678 rounds down to 1200 with -2 decimal places
    }

    #[test]
    fn test_round_down_negative_value_negative_places() {
        // =ROUNDDOWN(-1234.5678,-2) in US format
        // =ROUNDDOWN(-1234,5678;-2) in German format
        let result = codcel_round_down(-1234.5678, -2).unwrap();
        assert_eq!(result, -1200.0); // -1234.5678 rounds toward zero to -1200 with -2 decimal places
    }

    #[test]
    fn test_round_down_multiple_decimal_places() {
        // =ROUNDDOWN(3.14159265359,8) in US format
        // =ROUNDDOWN(3,14159265359;8) in German format
        let result = codcel_round_down(3.14159265359, 8).unwrap();
        assert_eq!(result, 3.14159265); // 3.14159265359 rounds down to 3.14159265 with 8 decimal places
    }

    #[test]
    fn test_round_down_exactly_integer() {
        // =ROUNDDOWN(5.0,2) in US format
        // =ROUNDDOWN(5,0;2) in German format
        let result = codcel_round_down(5.0, 2).unwrap();
        assert_eq!(result, 5.0); // 5.0 rounds down to 5.0 with 2 decimal places
    }

    #[test]
    fn test_round_down_large_number() {
        // =ROUNDDOWN(1234567.89,0) in US format
        // =ROUNDDOWN(1234567,89;0) in German format
        let result = codcel_round_down(1234567.89, 0).unwrap();
        assert_eq!(result, 1234567.0); // 1234567.89 rounds down to 1234567 with 0 decimal places
    }

    #[test]
    fn test_round_down_small_decimal() {
        // =ROUNDDOWN(0.0000123456,7) in US format
        // =ROUNDDOWN(0,0000123456;7) in German format
        let result = codcel_round_down(0.0000123456, 7).unwrap();
        assert_eq!(result, 0.0000123); // 0.0000123456 rounds down to 0.0000123 with 7 decimal places
    }
}
