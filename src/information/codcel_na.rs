// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `NA` that returns the `#N/A` error value.
///
/// Takes no arguments. Returns `f64::NAN` which represents `#N/A` in the calculation engine.
pub fn codcel_na() -> Result<f64, Box<dyn Error + Send + Sync>> {
    Ok(f64::NAN)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_na_returns_nan() {
        // =NA() in US format
        // =NA() in German format
        let result = codcel_na().unwrap();
        assert!(result.is_nan());
    }

    #[test]
    fn test_na_is_not_finite() {
        // =NA() is not a finite number
        let result = codcel_na().unwrap();
        assert!(!result.is_finite());
    }

    #[test]
    fn test_na_propagates_through_arithmetic() {
        // =NA()+1 should still be NaN
        let result = codcel_na().unwrap() + 1.0;
        assert!(result.is_nan());
    }
}
