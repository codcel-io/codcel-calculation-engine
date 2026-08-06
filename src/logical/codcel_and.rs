// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `AND` that evaluates whether all arguments are TRUE.
/// - `values`: a vector of boolean values to evaluate.
///
/// Returns `true` only when every value is `true`; returns `false` for any `false` value
///
/// or when the input is empty (matching Excel's `AND()` behavior).
pub fn codcel_and(values: Vec<bool>) -> Result<bool, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Ok(false);
    }

    Ok(values.iter().all(|&b| b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and_empty_vector() {
        // =AND() in Excel
        let result = codcel_and(vec![]).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_and_all_true() {
        // =AND(TRUE, TRUE, TRUE) in Excel
        let result = codcel_and(vec![true, true, true]).unwrap();
        assert!(result);
    }

    #[test]
    fn test_and_all_false() {
        // =AND(FALSE, FALSE, FALSE) in Excel
        let result = codcel_and(vec![false, false, false]).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_and_mixed_values_with_false() {
        // =AND(TRUE, FALSE, TRUE) in Excel
        let result = codcel_and(vec![true, false, true]).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_and_single_true() {
        // =AND(TRUE) in Excel
        let result = codcel_and(vec![true]).unwrap();
        assert!(result);
    }

    #[test]
    fn test_and_single_false() {
        // =AND(FALSE) in Excel
        let result = codcel_and(vec![false]).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_and_first_false() {
        // =AND(FALSE, TRUE, TRUE) in Excel
        let result = codcel_and(vec![false, true, true]).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_and_last_false() {
        // =AND(TRUE, TRUE, FALSE) in Excel
        let result = codcel_and(vec![true, true, false]).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_and_many_true_values() {
        // =AND(TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE) in Excel
        let result = codcel_and(vec![
            true, true, true, true, true, true, true, true, true, true,
        ])
        .unwrap();
        assert!(result);
    }
}
