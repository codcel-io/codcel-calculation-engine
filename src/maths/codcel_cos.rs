// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::maths::check_values::check_value_f64;
use std::error::Error;

/// Excel-compatible `COS` that returns the cosine of an angle.
/// - `value`: the angle in radians.
///
/// Returns the cosine of the angle or an error when input is NaN or infinite.
pub fn codcel_cos(value: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // TODO: PERHAPS WE NEED THIS CHECK ON ALL FUNCTIONS
    // PERHAPS IF IT HAPPENS WE SHOULD RETURN 0.0?????
    check_value_f64("COS", value)?;

    Ok(crate::portable_math::cos(value))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_cos_zero() {
        // =COS(0) in US format
        // =COS(0) in German format
        let result = codcel_cos(0.0).unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_cos_pi() {
        // =COS(PI()) in US format
        // =COS(PI()) in German format
        let result = codcel_cos(PI).unwrap();
        let expected = -1.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_cos_half_pi() {
        // =COS(PI()/2) in US format
        // =COS(PI()/2) in German format
        let result = codcel_cos(PI / 2.0).unwrap();
        let expected = 0.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_cos_quarter_pi() {
        // =COS(PI()/4) in US format
        // =COS(PI()/4) in German format
        let result = codcel_cos(PI / 4.0).unwrap();
        let expected = 0.7071067811865475;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_cos_negative_pi() {
        // =COS(-PI()) in US format
        // =COS(-PI()) in German format
        let result = codcel_cos(-PI).unwrap();
        let expected = -1.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_cos_two_pi() {
        // =COS(2*PI()) in US format
        // =COS(2*PI()) in German format
        let result = codcel_cos(2.0 * PI).unwrap();
        let expected = 1.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_cos_arbitrary_value() {
        // =COS(1) in US format
        // =COS(1) in German format
        let result = codcel_cos(1.0).unwrap();
        let expected = 0.5403023058681398;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }
}
