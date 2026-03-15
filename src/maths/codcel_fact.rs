// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `FACT` that returns the factorial of a number.
/// - `value`: a non-negative integer.
///
/// Returns value! or an error when value is negative.
pub fn codcel_fact(value: i32) -> Result<i32, Box<dyn Error + Send + Sync>> {
    if value < 0 {
        Err(
            format!("FACT: Negative values are not allowed for factorial calculations {value:?}")
                .into(),
        )
    } else if value <= 1 {
        Ok(1)
    } else {
        Ok(value * codcel_fact(value - 1)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fact_zero() {
        // =FACT(0) in US format
        // =FACT(0) in German format
        let result = codcel_fact(0).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fact_one() {
        // =FACT(1) in US format
        // =FACT(1) in German format
        let result = codcel_fact(1).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fact_five() {
        // =FACT(5) in US format
        // =FACT(5) in German format
        let result = codcel_fact(5).unwrap();
        assert_eq!(result, 120); // 5! = 5 * 4 * 3 * 2 * 1 = 120
    }

    #[test]
    fn test_fact_ten() {
        // =FACT(10) in US format
        // =FACT(10) in German format
        let result = codcel_fact(10).unwrap();
        assert_eq!(result, 3628800); // 10! = 10 * 9 * ... * 1 = 3,628,800
    }

    #[test]
    fn test_fact_negative() {
        // =FACT(-1) in US format - should return an error
        // =FACT(-1) in German format - should return an error
        let result = codcel_fact(-1);
        assert!(result.is_err());
    }

    #[test]
    fn test_fact_large() {
        // Note: This test might fail for very large values due to integer overflow
        // =FACT(12) in US format
        // =FACT(12) in German format
        let result = codcel_fact(12).unwrap();
        assert_eq!(result, 479001600); // 12! = 479,001,600
    }
}
