// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `DELTA` that tests whether two values are equal.
/// - `first_number`: the first value.
/// - `second_number`: the second value (defaults to `0`).
///   Returns `1` if the values are exactly equal, `0` otherwise.
pub fn codcel_delta(
    first_number: f64,
    second_number: Option<f64>,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    let second_number = second_number.unwrap_or(0.0);
    // Implementation of DELTA
    if first_number == second_number {
        Ok(1)
    } else {
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delta_equal() {
        // =DELTA(5, 5) in US format
        // =DELTA(5; 5) in German format
        let result = codcel_delta(5.0, Some(5.0)).unwrap();
        println!("{result}");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_delta_not_equal() {
        // =DELTA(5, 4) in US format
        // =DELTA(5; 4) in German format
        let result = codcel_delta(5.0, Some(4.0)).unwrap();
        println!("{result}");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_delta_with_default() {
        // =DELTA(0) in US format
        // =DELTA(0) in German format
        let result = codcel_delta(0.0, None).unwrap();
        println!("{result}");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_delta_not_equal_with_default() {
        // =DELTA(5) in US format
        // =DELTA(5) in German format
        let result = codcel_delta(5.0, None).unwrap();
        println!("{result}");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_delta_with_floating_point() {
        // =DELTA(3.14, 3.14) in US format
        // =DELTA(3,14; 3,14) in German format
        let result = codcel_delta(3.14, Some(3.14)).unwrap();
        println!("{result}");
        assert_eq!(result, 1);
    }
}
