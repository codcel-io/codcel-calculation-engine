// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::maths::check_values::check_value_f64;
use std::error::Error;

/// Excel-compatible `SIGN` that returns the sign of a number.
/// - `number`: the number whose sign you want.
///
/// Returns 1 (positive), 0 (zero), or -1 (negative).
pub fn codcel_sign(number: f64) -> Result<i32, Box<dyn Error + Send + Sync>> {
    check_value_f64("SIGN", number)?;
    if number > 0.0 {
        Ok(1)
    } else if number < 0.0 {
        Ok(-1)
    } else {
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_positive() {
        // =SIGN(2.5) in US format
        // =SIGN(2,5) in German format
        let result = codcel_sign(2.5).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_sign_negative() {
        // =SIGN(-3.7) in US format
        // =SIGN(-3,7) in German format
        let result = codcel_sign(-3.7).unwrap();
        assert_eq!(result, -1);
    }

    #[test]
    fn test_sign_zero() {
        // =SIGN(0) in US format
        // =SIGN(0) in German format
        let result = codcel_sign(0.0).unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_sign_large_positive() {
        // =SIGN(1000000.5) in US format
        // =SIGN(1000000,5) in German format
        let result = codcel_sign(1000000.5).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_sign_large_negative() {
        // =SIGN(-1000000.5) in US format
        // =SIGN(-1000000,5) in German format
        let result = codcel_sign(-1000000.5).unwrap();
        assert_eq!(result, -1);
    }

    #[test]
    fn test_sign_small_positive() {
        // =SIGN(0.00001) in US format
        // =SIGN(0,00001) in German format
        let result = codcel_sign(0.00001).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_sign_small_negative() {
        // =SIGN(-0.00001) in US format
        // =SIGN(-0,00001) in German format
        let result = codcel_sign(-0.00001).unwrap();
        assert_eq!(result, -1);
    }
}
