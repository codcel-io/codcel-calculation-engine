// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `ROUNDUP` that rounds a number away from zero.
/// - `value`: the number to round.
/// - `decimal_places`: the number of digits to round to.
///
/// Returns the rounded value (always rounds away from zero).
pub fn codcel_round_up(
    value: f64,
    decimal_places: i32,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let multiplier = 10_f64.powi(decimal_places);
    let rounded = if value >= 0.0 {
        (value * multiplier).ceil() / multiplier
    } else {
        -((- value * multiplier).ceil() / multiplier)
    };
    Ok(rounded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_up_positive() {
        // =ROUNDUP(3.14159,2) in US format
        // =ROUNDUP(3,14159;2) in German format
        let result = codcel_round_up(3.14159, 2).unwrap();
        assert_eq!(result, 3.15); // 3.14159 rounds up to 3.15 with 2 decimal places
    }

    #[test]
    fn test_round_up_negative() {
        // =ROUNDUP(-3.14159,2) in US format
        // =ROUNDUP(-3,14159;2) in German format
        let result = codcel_round_up(-3.14159, 2).unwrap();
        assert_eq!(result, -3.15); // -3.14159 rounds away from zero to -3.15 with 2 decimal places
    }

    #[test]
    fn test_round_up_zero_decimal_places() {
        // =ROUNDUP(3.01,0) in US format
        // =ROUNDUP(3,01;0) in German format
        let result = codcel_round_up(3.01, 0).unwrap();
        assert_eq!(result, 4.0); // 3.01 rounds up to 4 with 0 decimal places
    }

    #[test]
    fn test_round_up_negative_decimal_places() {
        // =ROUNDUP(1234.5678,-2) in US format
        // =ROUNDUP(1234,5678;-2) in German format
        let result = codcel_round_up(1234.5678, -2).unwrap();
        assert_eq!(result, 1300.0); // 1234.5678 rounds up to 1300 with -2 decimal places
    }

    #[test]
    fn test_round_up_negative_value_negative_places() {
        // =ROUNDUP(-1234.5678,-2) in US format
        // =ROUNDUP(-1234,5678;-2) in German format
        let result = codcel_round_up(-1234.5678, -2).unwrap();
        assert_eq!(result, -1300.0); // -1234.5678 rounds away from zero to -1300 with -2 decimal places
    }

    #[test]
    fn test_round_up_multiple_decimal_places() {
        // =ROUNDUP(3.14159265359,8) in US format
        // =ROUNDUP(3,14159265359;8) in German format
        let result = codcel_round_up(3.14159265359, 8).unwrap();
        assert_eq!(result, 3.14159266); // 3.14159265359 rounds up to 3.14159266 with 8 decimal places
    }

    #[test]
    fn test_round_up_exactly_integer() {
        // =ROUNDUP(5.0,2) in US format
        // =ROUNDUP(5,0;2) in German format
        let result = codcel_round_up(5.0, 2).unwrap();
        assert_eq!(result, 5.0); // 5.0 rounds up to 5.0 with 2 decimal places
    }

    #[test]
    fn test_round_up_large_number() {
        // =ROUNDUP(1234567.01,0) in US format
        // =ROUNDUP(1234567,01;0) in German format
        let result = codcel_round_up(1234567.01, 0).unwrap();
        assert_eq!(result, 1234568.0); // 1234567.01 rounds up to 1234568 with 0 decimal places
    }

    #[test]
    fn test_round_up_small_decimal() {
        // =ROUNDUP(0.0000123456,7) in US format
        // =ROUNDUP(0,0000123456;7) in German format
        let result = codcel_round_up(0.0000123456, 7).unwrap();
        assert_eq!(result, 0.0000124); // 0.0000123456 rounds up to 0.0000124 with 7 decimal places
    }

    #[test]
    fn test_round_up_zero() {
        // =ROUNDUP(0,2) in US format
        // =ROUNDUP(0;2) in German format
        let result = codcel_round_up(0.0, 2).unwrap();
        assert_eq!(result, 0.0); // 0.0 rounds up to 0.0 with 2 decimal places
    }
}
