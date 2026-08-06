// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::any::Any;
use std::cmp::Ordering;
use std::error::Error;

/// Excel-style less-than-or-equal (`<=`) comparison across common primitives.
///
/// Handles `f64`, `i32`, `bool`, and `&str` values. Numeric inputs promote
/// `i32` to `f64` to enable mixed comparisons. When the types cannot be
/// compared meaningfully, the function returns `Ok(false)` instead of an error,
/// matching Excel's behavior for incompatible comparisons.
pub fn codcel_less_than_or_equal<T: 'static + Any, U: 'static + Any>(
    input1: T,
    input2: U,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    let input1 = &input1 as &dyn Any;
    let input2 = &input2 as &dyn Any;

    if let Some(val) = input1.downcast_ref::<f64>() {
        if let Some(val2) = input2.downcast_ref::<f64>() {
            return less_than_or_equal_internal(*val, *val2);
        } else if let Some(val2) = input2.downcast_ref::<i32>() {
            return less_than_or_equal_internal(*val, *val2 as f64);
        }
    } else if let Some(val) = input1.downcast_ref::<i32>() {
        if let Some(val2) = input2.downcast_ref::<i32>() {
            return less_than_or_equal_internal(*val, *val2);
        } else if let Some(val2) = input2.downcast_ref::<f64>() {
            return less_than_or_equal_internal(*val as f64, *val2);
        }
    } else if let Some(val) = input1.downcast_ref::<bool>() {
        if let Some(val2) = input2.downcast_ref::<bool>() {
            return less_than_or_equal_internal(val, val2);
        }
    } else if let Some(val) = input1.downcast_ref::<&str>() {
        if let Some(val2) = input2.downcast_ref::<&str>() {
            return less_than_or_equal_internal(val, val2);
        }
    }

    // To think about, I guess if the types don´t match then obviously they cannot be equal
    Ok(false)
}

fn less_than_or_equal_internal<T, U>(
    input1: T,
    input2: U,
) -> Result<bool, Box<dyn Error + Send + Sync>>
where
    T: PartialOrd<U>,
{
    match input1.partial_cmp(&input2) {
        None => Ok(false),
        Some(Ordering::Less) | Some(Ordering::Equal) => Ok(true),
        _ => Ok(false),
    }
}

#[test]
fn test_less_than_or_equal() {
    use crate::arithmetic_base::float;

    assert!(codcel_less_than_or_equal(float(36.0, ".").unwrap(), float(42, ".").unwrap()).unwrap());
    assert!(codcel_less_than_or_equal("apple", "banana").unwrap());
    assert!(codcel_less_than_or_equal(5, 5).unwrap());
}

#[test]
fn test_f64_less_than_or_equal() {
    // Basic comparison
    assert!(codcel_less_than_or_equal(10.0, 10.1).unwrap());
    assert!(codcel_less_than_or_equal(10.0, 10.0).unwrap());
    assert!(!codcel_less_than_or_equal(10.1, 10.0).unwrap());

    // Zero values
    assert!(codcel_less_than_or_equal(0.0, 0.1).unwrap());
    assert!(codcel_less_than_or_equal(0.0, 0.0).unwrap());
    assert!(!codcel_less_than_or_equal(0.1, 0.0).unwrap());

    // Negative values
    assert!(codcel_less_than_or_equal(-5.5, -5.4).unwrap());
    assert!(codcel_less_than_or_equal(-5.5, -5.5).unwrap());
    assert!(!codcel_less_than_or_equal(-5.4, -5.5).unwrap());

    // Very small difference
    assert!(codcel_less_than_or_equal(0.00000001, 0.00000002).unwrap());
    assert!(codcel_less_than_or_equal(0.00000001, 0.00000001).unwrap());
    assert!(!codcel_less_than_or_equal(0.00000002, 0.00000001).unwrap());

    // Very large numbers
    assert!(codcel_less_than_or_equal(1e15, 1e15 + 1.0).unwrap());
    assert!(codcel_less_than_or_equal(1e15, 1e15).unwrap());
    assert!(!codcel_less_than_or_equal(1e15 + 1.0, 1e15).unwrap());

    // Positive vs negative
    assert!(codcel_less_than_or_equal(-1.0, 1.0).unwrap());
    assert!(!codcel_less_than_or_equal(1.0, -1.0).unwrap());
}

#[test]
fn test_i32_less_than_or_equal() {
    // Basic comparison
    assert!(codcel_less_than_or_equal(10, 11).unwrap());
    assert!(codcel_less_than_or_equal(10, 10).unwrap());
    assert!(!codcel_less_than_or_equal(11, 10).unwrap());

    // Zero values
    assert!(codcel_less_than_or_equal(0, 1).unwrap());
    assert!(codcel_less_than_or_equal(0, 0).unwrap());
    assert!(!codcel_less_than_or_equal(1, 0).unwrap());

    // Negative values
    assert!(codcel_less_than_or_equal(-5, -4).unwrap());
    assert!(codcel_less_than_or_equal(-5, -5).unwrap());
    assert!(!codcel_less_than_or_equal(-4, -5).unwrap());

    // Min/Max values
    assert!(codcel_less_than_or_equal(i32::MIN, i32::MAX).unwrap());
    assert!(codcel_less_than_or_equal(i32::MIN, 0).unwrap());
    assert!(codcel_less_than_or_equal(0, i32::MAX).unwrap());
    assert!(!codcel_less_than_or_equal(i32::MAX, i32::MIN).unwrap());
}

#[test]
fn test_bool_less_than_or_equal() {
    // All possible boolean combinations
    assert!(codcel_less_than_or_equal(false, true).unwrap());
    assert!(codcel_less_than_or_equal(false, false).unwrap());
    assert!(codcel_less_than_or_equal(true, true).unwrap());
    assert!(!codcel_less_than_or_equal(true, false).unwrap());
}

#[test]
fn test_string_less_than_or_equal() {
    // Basic comparison
    assert!(codcel_less_than_or_equal("hello", "world").unwrap());
    assert!(codcel_less_than_or_equal("hello", "hello").unwrap());
    assert!(!codcel_less_than_or_equal("world", "hello").unwrap());

    // Empty strings
    assert!(codcel_less_than_or_equal("", "a").unwrap());
    assert!(codcel_less_than_or_equal("", "").unwrap());
    assert!(!codcel_less_than_or_equal("a", "").unwrap());

    // Case sensitivity
    assert!(codcel_less_than_or_equal("Hello", "hello").unwrap()); // uppercase is less than lowercase in ASCII
    assert!(!codcel_less_than_or_equal("hello", "Hello").unwrap());

    // Special characters
    assert!(codcel_less_than_or_equal("!", "z").unwrap());
    assert!(!codcel_less_than_or_equal("z", "!").unwrap());

    // Unicode characters
    assert!(codcel_less_than_or_equal("こんにちは", "さようなら").unwrap());
    assert!(!codcel_less_than_or_equal("さようなら", "こんにちは").unwrap());

    // Lexicographical comparison
    assert!(codcel_less_than_or_equal("abc122", "abc123").unwrap());
    assert!(!codcel_less_than_or_equal("abc123", "abc122").unwrap());
}

#[test]
fn test_cross_type_number_less_than_or_equal() {
    // f64 to i32
    assert!(codcel_less_than_or_equal(41.9, 42).unwrap());
    assert!(codcel_less_than_or_equal(42.0, 42).unwrap());
    assert!(!codcel_less_than_or_equal(42.1, 42).unwrap());
    assert!(codcel_less_than_or_equal(-0.1, 0).unwrap());
    assert!(codcel_less_than_or_equal(-10.0, -10).unwrap());
    assert!(!codcel_less_than_or_equal(-9.9, -10).unwrap());

    // i32 to f64
    assert!(codcel_less_than_or_equal(41, 42.0).unwrap());
    assert!(codcel_less_than_or_equal(42, 42.0).unwrap());
    assert!(!codcel_less_than_or_equal(43, 42.0).unwrap());
    assert!(codcel_less_than_or_equal(0, 0.1).unwrap());
    assert!(codcel_less_than_or_equal(-10, -10.0).unwrap());
    assert!(!codcel_less_than_or_equal(-9, -10.0).unwrap());

    // Edge cases
    assert!(codcel_less_than_or_equal((i32::MAX - 1) as f64, i32::MAX).unwrap());
    assert!(codcel_less_than_or_equal(i32::MIN, (i32::MIN + 1) as f64).unwrap());
}

#[test]
fn test_unsupported_type_combinations() {
    // Boolean to number
    assert!(!codcel_less_than_or_equal(true, 1).unwrap());
    assert!(!codcel_less_than_or_equal(false, 0).unwrap());
    assert!(!codcel_less_than_or_equal(1, true).unwrap());
    assert!(!codcel_less_than_or_equal(0, false).unwrap());

    // String to number
    assert!(!codcel_less_than_or_equal("42", 42).unwrap());
    assert!(!codcel_less_than_or_equal(42, "42").unwrap());
    assert!(!codcel_less_than_or_equal("42.0", 42.0).unwrap());
    assert!(!codcel_less_than_or_equal(42.0, "42.0").unwrap());

    // String to boolean
    assert!(!codcel_less_than_or_equal("true", true).unwrap());
    assert!(!codcel_less_than_or_equal(true, "true").unwrap());
    assert!(!codcel_less_than_or_equal("false", false).unwrap());
    assert!(!codcel_less_than_or_equal(false, "false").unwrap());
}

#[test]
fn test_edge_cases() {
    // Different types that might be considered comparable in some contexts
    assert!(!codcel_less_than_or_equal("", 0).unwrap());
    assert!(!codcel_less_than_or_equal(0, "").unwrap());
    assert!(!codcel_less_than_or_equal("", false).unwrap());
    assert!(!codcel_less_than_or_equal(false, "").unwrap());

    // Positive vs negative zero
    assert!(codcel_less_than_or_equal(0.0, -0.0).unwrap());
    assert!(codcel_less_than_or_equal(-0.0, 0.0).unwrap());

    // NaN handling (should return false for any comparison)
    assert!(!codcel_less_than_or_equal(f64::NAN, 0.0).unwrap());
    assert!(!codcel_less_than_or_equal(0.0, f64::NAN).unwrap());
    assert!(!codcel_less_than_or_equal(f64::NAN, f64::NAN).unwrap());

    // Infinity handling
    assert!(codcel_less_than_or_equal(0.0, f64::INFINITY).unwrap());
    assert!(codcel_less_than_or_equal(f64::MAX, f64::INFINITY).unwrap());
    assert!(!codcel_less_than_or_equal(f64::INFINITY, 0.0).unwrap());
    assert!(codcel_less_than_or_equal(f64::NEG_INFINITY, 0.0).unwrap());
    assert!(!codcel_less_than_or_equal(0.0, f64::NEG_INFINITY).unwrap());
    assert!(codcel_less_than_or_equal(f64::NEG_INFINITY, f64::INFINITY).unwrap());
    assert!(!codcel_less_than_or_equal(f64::INFINITY, f64::NEG_INFINITY).unwrap());
    assert!(codcel_less_than_or_equal(f64::INFINITY, f64::INFINITY).unwrap());
    assert!(codcel_less_than_or_equal(f64::NEG_INFINITY, f64::NEG_INFINITY).unwrap());
}
