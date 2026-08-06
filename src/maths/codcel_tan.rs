// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::maths::check_values::check_value_f64;
use std::error::Error;

/// Excel-compatible `TAN` that returns the tangent of an angle.
/// - `x`: the angle in radians.
///
/// Returns the tangent or an error when cos(x) is zero.
pub fn codcel_tan(x: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    check_value_f64("TAN", x)?;

    let cos_x = crate::portable_math::cos(x);
    if cos_x.abs() < f64::EPSILON {
        Err("TAN: Undefined value at this angle due to division by zero".into())
    } else {
        Ok(crate::portable_math::sin(x) / cos_x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_tan_zero() {
        // =TAN(0) in US format
        // =TAN(0) in German format
        let result = codcel_tan(0.0).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_tan_pi() {
        // =TAN(PI()) in US format
        // =TAN(PI()) in German format
        let result = codcel_tan(PI).unwrap();
        let expected = 0.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_tan_quarter_pi() {
        // =TAN(PI()/4) in US format
        // =TAN(PI()/4) in German format
        let result = codcel_tan(PI / 4.0).unwrap();
        let expected = 1.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_tan_third_pi() {
        // =TAN(PI()/3) in US format
        // =TAN(PI()/3) in German format
        let result = codcel_tan(PI / 3.0).unwrap();
        let expected = 1.7320508075688772;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_tan_negative_value() {
        // =TAN(-PI()/4) in US format
        // =TAN(-PI()/4) in German format
        let result = codcel_tan(-PI / 4.0).unwrap();
        let expected = -1.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_tan_arbitrary_value() {
        // =TAN(1) in US format
        // =TAN(1) in German format
        let result = codcel_tan(1.0).unwrap();
        let expected = 1.5574077246549023;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_tan_half_pi() {
        // =TAN(PI()/2) in US format - should return an error
        // =TAN(PI()/2) in German format - should return an error
        let result = codcel_tan(PI / 2.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_tan_three_half_pi() {
        // =TAN(3*PI()/2) in US format - should return an error
        // =TAN(3*PI()/2) in German format - should return an error
        let result = codcel_tan(3.0 * PI / 2.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_tan_periodicity() {
        // =TAN(PI()/4) in US format
        // =TAN(PI()/4) in German format
        let result1 = codcel_tan(PI / 4.0).unwrap();

        // =TAN(PI()/4 + PI()) in US format
        // =TAN(PI()/4 + PI()) in German format
        let result2 = codcel_tan(PI / 4.0 + PI).unwrap();

        // TAN has a period of π, so TAN(x) = TAN(x + π)
        let epsilon = 1e-14;
        assert!((result1 - result2).abs() < epsilon);
    }
}
