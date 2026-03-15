// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::any::Any;
use std::error::Error;

/// Excel-style equality (`=`) that works across primitive types.
///
/// Supported inputs are `f64`, `i32`, `bool`, and `&str`. Numbers are compared
/// after converting `i32` values to `f64`; other mixed-type combinations
/// return `false` rather than failing. The function mirrors Excel's notion of
/// equality by only comparing like-for-like values and succeeds with
/// `Ok(false)` when no comparison is possible.
pub fn codcel_equals<T: 'static + Any, U: 'static + Any>(
    input1: T,
    input2: U,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    let input1 = &input1 as &dyn Any;
    let input2 = &input2 as &dyn Any;

    if let Some(val) = input1.downcast_ref::<f64>() {
        if let Some(val2) = input2.downcast_ref::<f64>() {
            return Ok(val == val2);
        } else if let Some(val2) = input2.downcast_ref::<i32>() {
            return Ok(*val == *val2 as f64);
        }
    } else if let Some(val) = input1.downcast_ref::<i32>() {
        if let Some(val2) = input2.downcast_ref::<i32>() {
            return Ok(val == val2);
        } else if let Some(val2) = input2.downcast_ref::<f64>() {
            return Ok(*val as f64 == *val2);
        }
    } else if let Some(val) = input1.downcast_ref::<bool>() {
        if let Some(val2) = input2.downcast_ref::<bool>() {
            return Ok(val == val2);
        }
    } else if let Some(val) = input1.downcast_ref::<&str>() {
        if let Some(val2) = input2.downcast_ref::<&str>() {
            return Ok(val == val2);
        }
    }

    // To think about, I guess if the types don´t match then obviously they cannot be equal
    Ok(false)
}

#[test]
fn test_equals() {
    let input1: &str = "test";
    let input2: &str = "test";
    assert!(codcel_equals(input1, input2).unwrap());

    let input2: &str = "test1";
    assert!(!codcel_equals(input1, input2).unwrap());

    let input1: f64 = 42.0;
    let input2: i32 = 42;
    assert!(codcel_equals(input1, input2).unwrap());

    let input1: f64 = 42.1;
    assert!(!codcel_equals(input1, input2).unwrap());

    let input1: bool = true;
    let input2: bool = false;
    assert!(!codcel_equals(input1, input2).unwrap());
}

#[test]
fn test_f64_equality() {
    // Basic equality
    assert!(codcel_equals(10.0, 10.0).unwrap());
    assert!(!codcel_equals(10.0, 10.1).unwrap());

    // Zero values
    assert!(codcel_equals(0.0, 0.0).unwrap());
    assert!(!codcel_equals(0.0, -0.1).unwrap());

    // Negative values
    assert!(codcel_equals(-5.5, -5.5).unwrap());
    assert!(!codcel_equals(-5.5, -5.6).unwrap());

    // Very small difference
    assert!(!codcel_equals(0.00000001, 0.00000002).unwrap());

    // Very large numbers
    assert!(codcel_equals(1e15, 1e15).unwrap());
    assert!(!codcel_equals(1e15, 1e15 + 1.0).unwrap());
}

#[test]
fn test_i32_equality() {
    // Basic equality
    assert!(codcel_equals(10, 10).unwrap());
    assert!(!codcel_equals(10, 11).unwrap());

    // Zero values
    assert!(codcel_equals(0, 0).unwrap());

    // Negative values
    assert!(codcel_equals(-5, -5).unwrap());
    assert!(!codcel_equals(-5, -6).unwrap());

    // Min/Max values
    assert!(codcel_equals(i32::MAX, i32::MAX).unwrap());
    assert!(codcel_equals(i32::MIN, i32::MIN).unwrap());
    assert!(!codcel_equals(i32::MIN, i32::MAX).unwrap());
}

#[test]
fn test_bool_equality() {
    // All possible boolean combinations
    assert!(codcel_equals(true, true).unwrap());
    assert!(codcel_equals(false, false).unwrap());
    assert!(!codcel_equals(true, false).unwrap());
    assert!(!codcel_equals(false, true).unwrap());
}

#[test]
fn test_string_equality() {
    // Basic equality
    assert!(codcel_equals("hello", "hello").unwrap());
    assert!(!codcel_equals("hello", "world").unwrap());

    // Empty strings
    assert!(codcel_equals("", "").unwrap());
    assert!(!codcel_equals("", "not empty").unwrap());

    // Case sensitivity
    assert!(!codcel_equals("Hello", "hello").unwrap());

    // Special characters
    assert!(codcel_equals("!@#$%^&*()", "!@#$%^&*()").unwrap());
    assert!(!codcel_equals("!@#$%^&*()", "!@#$%^&*").unwrap());

    // Unicode characters
    assert!(codcel_equals("こんにちは", "こんにちは").unwrap());
    assert!(!codcel_equals("こんにちは", "さようなら").unwrap());
}

#[test]
fn test_cross_type_number_equality() {
    // f64 to i32
    assert!(codcel_equals(42.0, 42).unwrap());
    assert!(!codcel_equals(42.1, 42).unwrap());
    assert!(codcel_equals(0.0, 0).unwrap());
    assert!(codcel_equals(-10.0, -10).unwrap());
    assert!(!codcel_equals(-10.1, -10).unwrap());

    // i32 to f64
    assert!(codcel_equals(42, 42.0).unwrap());
    assert!(!codcel_equals(42, 42.1).unwrap());
    assert!(codcel_equals(0, 0.0).unwrap());
    assert!(codcel_equals(-10, -10.0).unwrap());
    assert!(!codcel_equals(-10, -10.1).unwrap());

    // Edge cases
    assert!(codcel_equals(i32::MAX, i32::MAX as f64).unwrap());
    assert!(codcel_equals(i32::MIN, i32::MIN as f64).unwrap());
}

#[test]
fn test_unsupported_type_combinations() {
    // Boolean to number
    assert!(!codcel_equals(true, 1).unwrap());
    assert!(!codcel_equals(false, 0).unwrap());
    assert!(!codcel_equals(1, true).unwrap());
    assert!(!codcel_equals(0, false).unwrap());

    // String to number
    assert!(!codcel_equals("42", 42).unwrap());
    assert!(!codcel_equals(42, "42").unwrap());
    assert!(!codcel_equals("42.0", 42.0).unwrap());
    assert!(!codcel_equals(42.0, "42.0").unwrap());

    // String to boolean
    assert!(!codcel_equals("true", true).unwrap());
    assert!(!codcel_equals(true, "true").unwrap());
    assert!(!codcel_equals("false", false).unwrap());
    assert!(!codcel_equals(false, "false").unwrap());
}

#[test]
fn test_edge_cases() {
    // Different types that might be considered equal in some contexts
    assert!(!codcel_equals("", 0).unwrap());
    assert!(!codcel_equals(0, "").unwrap());
    assert!(!codcel_equals("", false).unwrap());
    assert!(!codcel_equals(false, "").unwrap());

    // Empty string vs non-empty string
    assert!(!codcel_equals("", "not empty").unwrap());

    // Zero vs non-zero
    assert!(!codcel_equals(0, 1).unwrap());
    assert!(!codcel_equals(0.0, 1.0).unwrap());

    // Positive vs negative zero
    assert!(codcel_equals(0.0, -0.0).unwrap());
}
