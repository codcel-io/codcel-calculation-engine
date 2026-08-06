// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `CSCH` that returns the hyperbolic cosecant of a number.
/// - `x`: any real number except 0.
///
/// Returns the hyperbolic cosecant (1/sinh) or an error when x is zero.
pub fn codcel_csch(x: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Check if sinh(x) is zero (CSCH is undefined for x = 0)
    let sinh = crate::portable_math::sinh(x);
    if sinh == 0.0 {
        return Err("CSCH is undefined for x = 0".into());
    }

    // Calculate CSCH as 1 / sinh(x)
    Ok(1.0 / sinh)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csch_positive() {
        // =CSCH(1) in US format
        // =CSCH(1) in German format
        let result = codcel_csch(1.0).unwrap();
        let expected = 0.8509181282393216;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_csch_negative() {
        // =CSCH(-1) in US format
        // =CSCH(-1) in German format
        let result = codcel_csch(-1.0).unwrap();
        let expected = -0.8509181282393216;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_csch_large_value() {
        // =CSCH(5) in US format
        // =CSCH(5) in German format
        let result = codcel_csch(5.0).unwrap();
        let expected = 0.013476505830589089;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_csch_small_value() {
        // =CSCH(0.1) in US format
        // =CSCH(0,1) in German format
        let result = codcel_csch(0.1).unwrap();
        let expected = 9.98335275729611;
        let epsilon = 1e-13;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_csch_zero() {
        // =CSCH(0) in US format - should return an error
        // =CSCH(0) in German format - should return an error
        let result = codcel_csch(0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_csch_odd_function() {
        // =CSCH(2) in US format
        // =CSCH(2) in German format
        let result_positive = codcel_csch(2.0).unwrap();

        // =CSCH(-2) in US format
        // =CSCH(-2) in German format
        let result_negative = codcel_csch(-2.0).unwrap();

        // CSCH is an odd function, so CSCH(-x) = -CSCH(x)
        let epsilon = 1e-14;
        assert!((result_positive + result_negative).abs() < epsilon);
    }
}
