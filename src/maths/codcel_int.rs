// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `INT` that rounds a number down to the nearest integer.
/// - `value`: the real number to round down.
///
/// Returns the largest integer less than or equal to value.
pub fn codcel_int(value: f64) -> Result<i32, Box<dyn Error + Send + Sync>> {
    Ok(value.floor() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_positive_integer() {
        // =INT(5) in US format
        // =INT(5) in German format
        let result = codcel_int(5.0).unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn test_int_negative_integer() {
        // =INT(-5) in US format
        // =INT(-5) in German format
        let result = codcel_int(-5.0).unwrap();
        assert_eq!(result, -5);
    }

    #[test]
    fn test_int_positive_decimal() {
        // =INT(5.7) in US format
        // =INT(5,7) in German format
        let result = codcel_int(5.7).unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn test_int_negative_decimal() {
        // =INT(-5.7) in US format
        // =INT(-5,7) in German format
        let result = codcel_int(-5.7).unwrap();
        assert_eq!(result, -6); // Rounds down to the next integer
    }

    #[test]
    fn test_int_zero() {
        // =INT(0) in US format
        // =INT(0) in German format
        let result = codcel_int(0.0).unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_int_small_positive_decimal() {
        // =INT(0.3) in US format
        // =INT(0,3) in German format
        let result = codcel_int(0.3).unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_int_small_negative_decimal() {
        // =INT(-0.3) in US format
        // =INT(-0,3) in German format
        let result = codcel_int(-0.3).unwrap();
        assert_eq!(result, -1); // Rounds down to the next integer
    }

    #[test]
    fn test_int_large_number() {
        // =INT(2147483647) in US format
        // =INT(2147483647) in German format
        let result = codcel_int(2147483647.0).unwrap();
        assert_eq!(result, 2147483647); // Maximum i32 value
    }
}
