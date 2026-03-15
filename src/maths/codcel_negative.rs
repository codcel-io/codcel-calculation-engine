// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible unary minus that returns the negation of a number.
/// - `value`: the number to negate.
///
/// Returns the negated value (-value).
pub fn codcel_negative(value: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    Ok(-value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negative_positive_number() {
        // =-5 in US format
        // =-5 in German format
        let result = codcel_negative(5.0).unwrap();
        assert_eq!(result, -5.0);
    }

    #[test]
    fn test_negative_negative_number() {
        // =--3 in US format (equivalent to 3)
        // =--3 in German format (equivalent to 3)
        let result = codcel_negative(-3.0).unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_negative_zero() {
        // =-0 in US format
        // =-0 in German format
        let result = codcel_negative(0.0).unwrap();
        assert_eq!(result, -0.0);
    }

    #[test]
    fn test_negative_decimal() {
        // =-2.5 in US format
        // =-2,5 in German format
        let result = codcel_negative(2.5).unwrap();
        assert_eq!(result, -2.5);
    }

    #[test]
    fn test_negative_large_number() {
        // =-1000000 in US format
        // =-1000000 in German format
        let result = codcel_negative(1000000.0).unwrap();
        assert_eq!(result, -1000000.0);
    }

    #[test]
    fn test_negative_small_decimal() {
        // =-0.00001 in US format
        // =-0,00001 in German format
        let result = codcel_negative(0.00001).unwrap();
        assert_eq!(result, -0.00001);
    }
}
