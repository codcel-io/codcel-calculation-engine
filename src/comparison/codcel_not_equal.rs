// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::comparison::codcel_equals::codcel_equals;
use std::any::Any;
use std::error::Error;

/// Excel-style not-equal (`<>`) that wraps [`codcel_equals`] and negates it.
///
/// Supports the same `f64`, `i32`, `bool`, and `&str` inputs as
/// [`codcel_equals`]. If the underlying equality check cannot compare the
/// supplied types, this function returns `Ok(true)` only when a valid equality
/// comparison yields `false`; otherwise it mirrors the equality result.
pub fn codcel_not_equal<T: 'static + Any, U: 'static + Any>(
    input1: T,
    input2: U,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    Ok(!codcel_equals(input1, input2)?)
}

#[test]
fn test_not_equal() {
    let input1: &str = "test";
    let input2: &str = "test";
    assert!(!codcel_not_equal(input1, input2).unwrap());

    let input2: &str = "test1";
    assert!(codcel_not_equal(input1, input2).unwrap());

    let input1: f64 = 42.0;
    let input2: i32 = 42;
    assert!(!codcel_not_equal(input1, input2).unwrap());

    let input1: f64 = 42.1;
    assert!(codcel_not_equal(input1, input2).unwrap());

    let input1: bool = true;
    let input2: bool = false;
    assert!(codcel_not_equal(input1, input2).unwrap());
}

#[test]
fn test_f64_inequality() {
    // Basic inequality
    assert!(!codcel_not_equal(10.0, 10.0).unwrap());
    assert!(codcel_not_equal(10.0, 10.1).unwrap());

    // Zero values
    assert!(!codcel_not_equal(0.0, 0.0).unwrap());
    assert!(codcel_not_equal(0.0, -0.1).unwrap());

    // Negative values
    assert!(!codcel_not_equal(-5.5, -5.5).unwrap());
    assert!(codcel_not_equal(-5.5, -5.6).unwrap());

    // Very small difference
    assert!(codcel_not_equal(0.00000001, 0.00000002).unwrap());

    // Very large numbers
    assert!(!codcel_not_equal(1e15, 1e15).unwrap());
    assert!(codcel_not_equal(1e15, 1e15 + 1.0).unwrap());
}

#[test]
fn test_i32_inequality() {
    // Basic inequality
    assert!(!codcel_not_equal(10, 10).unwrap());
    assert!(codcel_not_equal(10, 11).unwrap());

    // Zero values
    assert!(!codcel_not_equal(0, 0).unwrap());

    // Negative values
    assert!(!codcel_not_equal(-5, -5).unwrap());
    assert!(codcel_not_equal(-5, -6).unwrap());

    // Min/Max values
    assert!(!codcel_not_equal(i32::MAX, i32::MAX).unwrap());
    assert!(!codcel_not_equal(i32::MIN, i32::MIN).unwrap());
    assert!(codcel_not_equal(i32::MIN, i32::MAX).unwrap());
}

#[test]
fn test_bool_inequality() {
    // All possible boolean combinations
    assert!(!codcel_not_equal(true, true).unwrap());
    assert!(!codcel_not_equal(false, false).unwrap());
    assert!(codcel_not_equal(true, false).unwrap());
    assert!(codcel_not_equal(false, true).unwrap());
}

#[test]
fn test_string_inequality() {
    // Basic inequality
    assert!(!codcel_not_equal("hello", "hello").unwrap());
    assert!(codcel_not_equal("hello", "world").unwrap());

    // Empty strings
    assert!(!codcel_not_equal("", "").unwrap());
    assert!(codcel_not_equal("", "not empty").unwrap());

    // Case sensitivity
    assert!(codcel_not_equal("Hello", "hello").unwrap());

    // Special characters
    assert!(!codcel_not_equal("!@#$%^&*()", "!@#$%^&*()").unwrap());
    assert!(codcel_not_equal("!@#$%^&*()", "!@#$%^&*").unwrap());

    // Unicode characters
    assert!(!codcel_not_equal("こんにちは", "こんにちは").unwrap());
    assert!(codcel_not_equal("こんにちは", "さようなら").unwrap());
}

#[test]
fn test_cross_type_number_inequality() {
    // f64 to i32
    assert!(!codcel_not_equal(42.0, 42).unwrap());
    assert!(codcel_not_equal(42.1, 42).unwrap());
    assert!(!codcel_not_equal(0.0, 0).unwrap());
    assert!(!codcel_not_equal(-10.0, -10).unwrap());
    assert!(codcel_not_equal(-10.1, -10).unwrap());

    // i32 to f64
    assert!(!codcel_not_equal(42, 42.0).unwrap());
    assert!(codcel_not_equal(42, 42.1).unwrap());
    assert!(!codcel_not_equal(0, 0.0).unwrap());
    assert!(!codcel_not_equal(-10, -10.0).unwrap());
    assert!(codcel_not_equal(-10, -10.1).unwrap());

    // Edge cases
    assert!(!codcel_not_equal(i32::MAX, i32::MAX as f64).unwrap());
    assert!(!codcel_not_equal(i32::MIN, i32::MIN as f64).unwrap());
}

#[test]
fn test_unsupported_type_combinations() {
    // Boolean to number
    assert!(codcel_not_equal(true, 1).unwrap());
    assert!(codcel_not_equal(false, 0).unwrap());
    assert!(codcel_not_equal(1, true).unwrap());
    assert!(codcel_not_equal(0, false).unwrap());

    // String to number
    assert!(codcel_not_equal("42", 42).unwrap());
    assert!(codcel_not_equal(42, "42").unwrap());
    assert!(codcel_not_equal("42.0", 42.0).unwrap());
    assert!(codcel_not_equal(42.0, "42.0").unwrap());

    // String to boolean
    assert!(codcel_not_equal("true", true).unwrap());
    assert!(codcel_not_equal(true, "true").unwrap());
    assert!(codcel_not_equal("false", false).unwrap());
    assert!(codcel_not_equal(false, "false").unwrap());
}

#[test]
fn test_edge_cases() {
    // Different types that might be considered equal in some contexts
    assert!(codcel_not_equal("", 0).unwrap());
    assert!(codcel_not_equal(0, "").unwrap());
    assert!(codcel_not_equal("", false).unwrap());
    assert!(codcel_not_equal(false, "").unwrap());

    // Empty string vs non-empty string
    assert!(codcel_not_equal("", "not empty").unwrap());

    // Zero vs non-zero
    assert!(codcel_not_equal(0, 1).unwrap());
    assert!(codcel_not_equal(0.0, 1.0).unwrap());

    // Positive vs negative zero
    assert!(!codcel_not_equal(0.0, -0.0).unwrap());
}
