// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `ACOSH` that returns the inverse hyperbolic cosine of a number.
/// - `value`: any real number ≥ 1.
///
/// Returns the inverse hyperbolic cosine or an error when `value` is less than 1.
pub fn codcel_acosh(value: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Check if the number is less than 1.0
    if value < 1.0 {
        Err(format!("ACOSH: Number {value:} supplied is less than 1").into())
    } else {
        Ok(value.acosh())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acosh_one() {
        // =ACOSH(1) in US format
        // =ACOSH(1) in German format
        let result = codcel_acosh(1.0).unwrap();
        let expected = 0.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_acosh_greater_than_one() {
        // =ACOSH(2) in US format
        // =ACOSH(2) in German format
        let result = codcel_acosh(2.0).unwrap();
        // The expected value is approximately 1.3169578969248166
        let expected = 1.3169578969248166;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_acosh_large_value() {
        // =ACOSH(10) in US format
        // =ACOSH(10) in German format
        let result = codcel_acosh(10.0).unwrap();
        // The expected value is approximately 2.993222846126381
        let expected = 2.993222846126381;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_acosh_less_than_one() {
        // =ACOSH(0.5) in US format - should return an error
        // =ACOSH(0,5) in German format - should return an error
        let result = codcel_acosh(0.5);
        assert!(result.is_err());

        // =ACOSH(0) in US format - should return an error
        // =ACOSH(0) in German format - should return an error
        let result = codcel_acosh(0.0);
        assert!(result.is_err());

        // =ACOSH(-1) in US format - should return an error
        // =ACOSH(-1) in German format - should return an error
        let result = codcel_acosh(-1.0);
        assert!(result.is_err());
    }
}
