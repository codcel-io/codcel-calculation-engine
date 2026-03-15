// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `COUNTA` that counts the number of non-empty values.
/// - `values`: a vector of numeric values (pre-converted from the range).
///
/// Returns the count of values as f64.
pub fn codcel_counta(values: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    Ok(values.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counta_basic() {
        let result = codcel_counta(vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_counta_empty() {
        let result = codcel_counta(vec![]).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_counta_single() {
        let result = codcel_counta(vec![42.0]).unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_counta_with_zeros() {
        // Zeros are non-empty values and should be counted
        let result = codcel_counta(vec![0.0, 0.0, 0.0]).unwrap();
        assert_eq!(result, 3.0);
    }
}
