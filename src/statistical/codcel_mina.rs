// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `MINA` that returns the smallest value in a set of values,
/// including text and logical values.
/// - `values`: an array of numeric values (text coerced to 0, TRUE to 1, FALSE to 0).
///
/// Returns the minimum value, or 0.0 if the input is empty.
pub fn codcel_mina(values: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Ok(0.0);
    }

    let result = values.into_iter().fold(None, |min, num| match min {
        None => Some(num),
        Some(current_min) => Some(current_min.min(num)),
    });
    Ok(result.unwrap_or(0.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mina_basic() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_mina(values).unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_mina_negative_numbers() {
        let values = vec![-10.0, -5.0, -3.0, -1.0];
        let result = codcel_mina(values).unwrap();
        assert_eq!(result, -10.0);
    }

    #[test]
    fn test_mina_mixed_numbers() {
        let values = vec![-10.0, 0.0, 10.0, 5.0];
        let result = codcel_mina(values).unwrap();
        assert_eq!(result, -10.0);
    }

    #[test]
    fn test_mina_with_boolean_true_as_1() {
        // TRUE is coerced to 1.0 by the wrapper layer
        let values = vec![0.0, 1.0, 0.5];
        let result = codcel_mina(values).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_mina_with_text_as_zero() {
        // Text values are coerced to 0.0 by the wrapper layer
        let values = vec![0.0, 0.0, -5.0];
        let result = codcel_mina(values).unwrap();
        assert_eq!(result, -5.0);
    }

    #[test]
    fn test_mina_single_value() {
        let values = vec![42.0];
        let result = codcel_mina(values).unwrap();
        assert_eq!(result, 42.0);
    }

    #[test]
    fn test_mina_empty() {
        let values: Vec<f64> = vec![];
        let result = codcel_mina(values).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_mina_decimals() {
        let values = vec![1.5, 2.7, 3.1, 4.9];
        let result = codcel_mina(values).unwrap();
        assert_eq!(result, 1.5);
    }
}
