// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `ROUND` that rounds a number to a specified number of digits.
/// - `value`: the number to round.
/// - `decimal_places`: the number of digits to round to.
///
/// Returns the rounded value (rounds away from zero at midpoint).
pub fn codcel_round(value: f64, decimal_places: i32) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let multiplier = 10_f64.powi(decimal_places);
    let rounded = (value * multiplier).round() / multiplier;
    Ok(rounded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_positive_up() {
        // =ROUND(2.15,1) in US format
        // =ROUND(2,15;1) in German format
        let result = codcel_round(2.15, 1).unwrap();
        assert_eq!(result, 2.2); // 2.15 rounds up to 2.2 with 1 decimal place
    }

    #[test]
    fn test_round_positive_down() {
        // =ROUND(2.14,1) in US format
        // =ROUND(2,14;1) in German format
        let result = codcel_round(2.14, 1).unwrap();
        assert_eq!(result, 2.1); // 2.14 rounds down to 2.1 with 1 decimal place
    }

    #[test]
    fn test_round_negative_up() {
        // =ROUND(-2.15,1) in US format
        // =ROUND(-2,15;1) in German format
        let result = codcel_round(-2.15, 1).unwrap();
        assert_eq!(result, -2.2); // -2.15 rounds up to -2.2 with 1 decimal place
    }

    #[test]
    fn test_round_negative_down() {
        // =ROUND(-2.14,1) in US format
        // =ROUND(-2,14;1) in German format
        let result = codcel_round(-2.14, 1).unwrap();
        assert_eq!(result, -2.1); // -2.14 rounds down to -2.1 with 1 decimal place
    }

    #[test]
    fn test_round_zero_decimal_places() {
        // =ROUND(2.5,0) in US format
        // =ROUND(2,5;0) in German format
        let result = codcel_round(2.5, 0).unwrap();
        assert_eq!(result, 3.0); // 2.5 rounds up to 3 with 0 decimal places
    }

    #[test]
    fn test_round_negative_decimal_places() {
        // =ROUND(1234.5678,-2) in US format
        // =ROUND(1234,5678;-2) in German format
        let result = codcel_round(1234.5678, -2).unwrap();
        assert_eq!(result, 1200.0); // 1234.5678 rounds to 1200 with -2 decimal places
    }

    #[test]
    fn test_round_multiple_decimal_places() {
        // =ROUND(2.1234567,5) in US format
        // =ROUND(2,1234567;5) in German format
        let result = codcel_round(2.1234567, 5).unwrap();
        assert_eq!(result, 2.12346); // 2.1234567 rounds to 2.12346 with 5 decimal places
    }

    #[test]
    fn test_round_exactly_half() {
        // =ROUND(2.5,0) in US format
        // =ROUND(2,5;0) in German format
        let result = codcel_round(2.5, 0).unwrap();
        assert_eq!(result, 3.0); // 2.5 rounds up to 3 (Excel rounds away from zero for .5)

        // =ROUND(-2.5,0) in US format
        // =ROUND(-2,5;0) in German format
        let result = codcel_round(-2.5, 0).unwrap();
        assert_eq!(result, -3.0); // -2.5 rounds to -3 (Excel rounds away from zero for .5)
    }

    #[test]
    fn test_round_large_number() {
        // =ROUND(1234567.89,0) in US format
        // =ROUND(1234567,89;0) in German format
        let result = codcel_round(1234567.89, 0).unwrap();
        assert_eq!(result, 1234568.0); // 1234567.89 rounds to 1234568 with 0 decimal places
    }

    #[test]
    fn test_round_small_decimal() {
        // =ROUND(0.0000123456,7) in US format
        // =ROUND(0,0000123456;7) in German format
        let result = codcel_round(0.0000123456, 7).unwrap();
        assert_eq!(result, 0.0000123); // 0.0000123456 rounds to 0.0000123 with 7 decimal places
    }
}
