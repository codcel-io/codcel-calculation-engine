// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `FACTDOUBLE` that returns the double factorial of a number.
/// - `n`: a non-negative integer.
///
/// Returns n!! (n × (n-2) × (n-4) × ... × 2 or 1).
pub fn codcel_fact_double(n: i32) -> Result<i32, Box<dyn Error + Send + Sync>> {
    if n <= 1 {
        Ok(1)
    } else {
        Ok(n * codcel_fact_double(n - 2)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fact_double_zero() {
        // =FACTDOUBLE(0) in US format
        // =FACTDOUBLE(0) in German format
        let result = codcel_fact_double(0).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fact_double_one() {
        // =FACTDOUBLE(1) in US format
        // =FACTDOUBLE(1) in German format
        let result = codcel_fact_double(1).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fact_double_odd() {
        // =FACTDOUBLE(5) in US format
        // =FACTDOUBLE(5) in German format
        let result = codcel_fact_double(5).unwrap();
        assert_eq!(result, 15); // 5!! = 5 * 3 * 1 = 15
    }

    #[test]
    fn test_fact_double_even() {
        // =FACTDOUBLE(6) in US format
        // =FACTDOUBLE(6) in German format
        let result = codcel_fact_double(6).unwrap();
        assert_eq!(result, 48); // 6!! = 6 * 4 * 2 = 48
    }

    #[test]
    fn test_fact_double_larger_odd() {
        // =FACTDOUBLE(9) in US format
        // =FACTDOUBLE(9) in German format
        let result = codcel_fact_double(9).unwrap();
        assert_eq!(result, 945); // 9!! = 9 * 7 * 5 * 3 * 1 = 945
    }

    #[test]
    fn test_fact_double_larger_even() {
        // =FACTDOUBLE(10) in US format
        // =FACTDOUBLE(10) in German format
        let result = codcel_fact_double(10).unwrap();
        assert_eq!(result, 3840); // 10!! = 10 * 8 * 6 * 4 * 2 = 3840
    }
}
