// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `MAXA` that returns the largest value in a set of values,
/// including text and logical values.
/// - `values`: an array of numeric values (text coerced to 0, TRUE to 1, FALSE to 0).
///
/// Returns the maximum value, or 0.0 if the input is empty.
pub fn codcel_maxa(values: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Ok(0.0);
    }

    let result = values.into_iter().fold(None, |max, num| match max {
        None => Some(num),
        Some(current_max) => Some(current_max.max(num)),
    });
    Ok(result.unwrap_or(0.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maxa_basic() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_maxa(values).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_maxa_negative_numbers() {
        let values = vec![-10.0, -5.0, -3.0, -1.0];
        let result = codcel_maxa(values).unwrap();
        assert_eq!(result, -1.0);
    }

    #[test]
    fn test_maxa_mixed_numbers() {
        let values = vec![-10.0, 0.0, 10.0, 5.0];
        let result = codcel_maxa(values).unwrap();
        assert_eq!(result, 10.0);
    }

    #[test]
    fn test_maxa_with_boolean_true_as_1() {
        // TRUE is coerced to 1.0 by the wrapper layer
        let values = vec![0.0, 1.0, 0.5];
        let result = codcel_maxa(values).unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_maxa_with_text_as_zero() {
        // Text values are coerced to 0.0 by the wrapper layer
        let values = vec![0.0, 0.0, -5.0];
        let result = codcel_maxa(values).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_maxa_single_value() {
        let values = vec![42.0];
        let result = codcel_maxa(values).unwrap();
        assert_eq!(result, 42.0);
    }

    #[test]
    fn test_maxa_empty() {
        let values: Vec<f64> = vec![];
        let result = codcel_maxa(values).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_maxa_decimals() {
        let values = vec![1.5, 2.7, 3.1, 4.9];
        let result = codcel_maxa(values).unwrap();
        assert_eq!(result, 4.9);
    }
}
