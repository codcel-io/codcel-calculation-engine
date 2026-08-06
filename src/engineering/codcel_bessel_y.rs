// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use libm::yn;
use std::error::Error;

/// Excel-compatible `BESSELY` that returns the Bessel function of the second kind Y_n(x).
/// - `x`: the value at which to evaluate the function (must be non-negative).
/// - `n`: the order of the Bessel function (must be non-negative).
///   Returns Y_n(x), or an error when `x` or `n` is negative or computation results in NaN.
pub fn codcel_bessel_y(x: f64, n: i32) -> Result<f64, Box<dyn Error + Send + Sync>> {
    println!("x = {x}, n = {n}");

    if x < 0.0 {
        return Err(
            "BESSELY: Bessel function of the second kind is undefined for negative x".into(),
        );
    }
    if n < 0 {
        return Err("BESSELY: Order n must be non-negative".into());
    }

    let result = yn(n, x);
    if result.is_nan() {
        return Err("BESSELY: Computation resulted in NaN".into());
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bessel_y_basic() {
        // =BESSELY(1, 0) in US format
        // =BESSELY(1; 0) in German format
        let result = codcel_bessel_y(1.0, 0).unwrap();
        println!("{result}");
        assert!((result - 0.08825696421567698).abs() < 0.0001);
    }

    #[test]
    fn test_bessel_y_order_1() {
        // =BESSELY(1, 1) in US format
        // =BESSELY(1; 1) in German format
        let result = codcel_bessel_y(1.0, 1).unwrap();
        println!("{result}");
        assert!((result - (-0.7812128213002887)).abs() < 0.0001);
    }

    #[test]
    fn test_bessel_y_order_2() {
        // =BESSELY(1, 2) in US format
        // =BESSELY(1; 2) in German format
        let result = codcel_bessel_y(1.0, 2).unwrap();
        println!("{result}");
        assert!((result - (-1.6506826068162546)).abs() < 0.0001);
    }

    #[test]
    fn test_bessel_y_large_x() {
        // =BESSELY(5, 0) in US format
        // =BESSELY(5; 0) in German format
        let result = codcel_bessel_y(5.0, 0).unwrap();
        println!("{result}");
        assert!((result - (-0.308_517_625_249_034_1)).abs() < 0.0001);
    }

    #[test]
    fn test_bessel_y_error_negative_x() {
        // =BESSELY(-1, 0) in US format
        // =BESSELY(-1; 0) in German format
        let result = codcel_bessel_y(-1.0, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_bessel_y_error_negative_n() {
        // =BESSELY(1, -1) in US format
        // =BESSELY(1; -1) in German format
        let result = codcel_bessel_y(1.0, -1);
        assert!(result.is_err());
    }
}
