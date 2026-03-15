// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `GESTEP` that tests whether a number is greater than or equal to a threshold.
/// - `number`: the value to test.
/// - `step`: the threshold value (defaults to `0`).
///   Returns `1` if `number >= step`, `0` otherwise.
pub fn codcel_ge_step(number: f64, step: Option<f64>) -> Result<i32, Box<dyn Error + Send + Sync>> {
    // Default step value is 0 when not provided
    let step = step.unwrap_or(0.0);

    // If number >= step, return 1. Otherwise, return 0.
    if number >= step {
        Ok(1)
    } else {
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ge_step_greater() {
        // =GESTEP(5, 4) in US format
        // =GESTEP(5; 4) in German format
        let result = codcel_ge_step(5.0, Some(4.0)).unwrap();
        println!("{result}");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_ge_step_equal() {
        // =GESTEP(5, 5) in US format
        // =GESTEP(5; 5) in German format
        let result = codcel_ge_step(5.0, Some(5.0)).unwrap();
        println!("{result}");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_ge_step_less() {
        // =GESTEP(4, 5) in US format
        // =GESTEP(4; 5) in German format
        let result = codcel_ge_step(4.0, Some(5.0)).unwrap();
        println!("{result}");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_ge_step_with_default() {
        // =GESTEP(5) in US format
        // =GESTEP(5) in German format
        let result = codcel_ge_step(5.0, None).unwrap();
        println!("{result}");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_ge_step_negative_with_default() {
        // =GESTEP(-5) in US format
        // =GESTEP(-5) in German format
        let result = codcel_ge_step(-5.0, None).unwrap();
        println!("{result}");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_ge_step_zero_with_default() {
        // =GESTEP(0) in US format
        // =GESTEP(0) in German format
        let result = codcel_ge_step(0.0, None).unwrap();
        println!("{result}");
        assert_eq!(result, 1);
    }
}
