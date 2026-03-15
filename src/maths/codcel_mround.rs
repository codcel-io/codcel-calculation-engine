// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::maths::check_values::check_value_f64;
use std::error::Error;

/// Excel-compatible `MROUND` that rounds a number to the nearest multiple.
/// - `number`: the value to round.
/// - `multiple`: the multiple to which you want to round.
///
/// Returns the rounded value or an error when multiple is zero.
pub fn codcel_mround(number: f64, multiple: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    check_value_f64("MROUND", number)?;
    check_value_f64("MULTIPLE", multiple)?;

    if multiple == 0.0 {
        return Err("MROUND: Multiple cannot be zero".into()); // Cannot round to a multiple of zero.
    }

    // Calculate the rounded result.
    Ok((number / multiple).round() * multiple)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mround_positive_number_positive_multiple() {
        // =MROUND(10, 3) in US format
        // =MROUND(10; 3) in German format
        let result = codcel_mround(10.0, 3.0).unwrap();
        assert_eq!(result, 9.0); // Closest multiple of 3 to 10 is 9
    }

    #[test]
    fn test_mround_positive_number_positive_multiple_round_up() {
        // =MROUND(11, 3) in US format
        // =MROUND(11; 3) in German format
        let result = codcel_mround(11.0, 3.0).unwrap();
        assert_eq!(result, 12.0); // Closest multiple of 3 to 11 is 12
    }

    #[test]
    fn test_mround_negative_number_positive_multiple() {
        // =MROUND(-10, 3) in US format
        // =MROUND(-10; 3) in German format
        let result = codcel_mround(-10.0, 3.0).unwrap();
        assert_eq!(result, -9.0); // Closest multiple of 3 to -10 is -9
    }

    #[test]
    fn test_mround_negative_number_negative_multiple() {
        // =MROUND(-10, -3) in US format
        // =MROUND(-10; -3) in German format
        let result = codcel_mround(-10.0, -3.0).unwrap();
        assert_eq!(result, -9.0); // Closest multiple of -3 to -10 is -9
    }

    #[test]
    fn test_mround_positive_number_negative_multiple() {
        // =MROUND(10, -3) in US format
        // =MROUND(10; -3) in German format
        let result = codcel_mround(10.0, -3.0).unwrap();
        assert_eq!(result, 9.0); // Closest multiple of -3 to 10 is 9
    }

    #[test]
    fn test_mround_decimal_number() {
        // =MROUND(2.5, 0.5) in US format
        // =MROUND(2,5; 0,5) in German format
        let result = codcel_mround(2.5, 0.5).unwrap();
        assert_eq!(result, 2.5); // 2.5 is already a multiple of 0.5
    }

    #[test]
    fn test_mround_decimal_number_round_up() {
        // =MROUND(2.7, 0.5) in US format
        // =MROUND(2,7; 0,5) in German format
        let result = codcel_mround(2.7, 0.5).unwrap();
        assert_eq!(result, 2.5); // Closest multiple of 0.5 to 2.7 is 2.5
    }

    #[test]
    fn test_mround_decimal_number_round_down() {
        // =MROUND(2.3, 0.5) in US format
        // =MROUND(2,3; 0,5) in German format
        let result = codcel_mround(2.3, 0.5).unwrap();
        assert_eq!(result, 2.5); // Closest multiple of 0.5 to 2.3 is 2.5
    }

    #[test]
    fn test_mround_zero() {
        // =MROUND(0, 3) in US format
        // =MROUND(0; 3) in German format
        let result = codcel_mround(0.0, 3.0).unwrap();
        assert_eq!(result, 0.0); // 0 is a multiple of any number
    }

    #[test]
    fn test_mround_zero_multiple() {
        // =MROUND(10, 0) in US format - should return an error
        // =MROUND(10; 0) in German format - should return an error
        let result = codcel_mround(10.0, 0.0);
        assert!(result.is_err());
    }
}
