// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::any::Any;
use std::error::Error;

/// Excel-style greater-than (`>`) comparison across common primitive types.
///
/// Accepts `f64`, `i32`, `bool`, and `&str` values. Integers are promoted to
/// `f64` for numeric comparisons; booleans and strings use their natural
/// ordering. Mismatched or unsupported type pairs return `Ok(false)` instead
/// of erroring, aligning with Excel comparison behavior.
pub fn codcel_greater_than<T: 'static + Any, U: 'static + Any>(
    input1: T,
    input2: U,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    let input1 = &input1 as &dyn Any;
    let input2 = &input2 as &dyn Any;

    if let Some(val) = input1.downcast_ref::<f64>() {
        if let Some(val2) = input2.downcast_ref::<f64>() {
            return greater_than_internal(*val, *val2);
        } else if let Some(val2) = input2.downcast_ref::<i32>() {
            return greater_than_internal(*val, *val2 as f64);
        }
    } else if let Some(val) = input1.downcast_ref::<i32>() {
        if let Some(val2) = input2.downcast_ref::<i32>() {
            return greater_than_internal(*val, *val2);
        } else if let Some(val2) = input2.downcast_ref::<f64>() {
            return greater_than_internal(*val as f64, *val2);
        }
    } else if let Some(val) = input1.downcast_ref::<bool>() {
        if let Some(val2) = input2.downcast_ref::<bool>() {
            return greater_than_internal(val, val2);
        }
    } else if let Some(val) = input1.downcast_ref::<&str>() {
        if let Some(val2) = input2.downcast_ref::<&str>() {
            return greater_than_internal(val, val2);
        }
    }

    // To think about, I guess if the types don´t match then obviously they cannot be equal
    Ok(false)
}

// For greater than i32 must be converted into float/number
fn greater_than_internal<T, U>(input1: T, input2: U) -> Result<bool, Box<dyn Error + Send + Sync>>
where
    T: PartialOrd<U>,
{
    match input1
        .partial_cmp(&input2)
        .map(|ord| ord == std::cmp::Ordering::Greater)
    {
        None => {
            // TODO, Perhaps should throw an error here.  To think
            Ok(false)
        }
        Some(result) => Ok(result),
    }
}

#[test]
fn test_greater_than() {
    use crate::arithmetic_base::float;

    assert!(codcel_greater_than(float(42.0, ".").unwrap(), float(36, ".").unwrap()).unwrap());
    assert!(!codcel_greater_than("apple", "banana").unwrap());
    assert!(!codcel_greater_than(5, 5).unwrap());
}

#[test]
fn test_f64_greater_than() {
    // Basic comparison
    assert!(codcel_greater_than(10.1, 10.0).unwrap());
    assert!(!codcel_greater_than(10.0, 10.0).unwrap());
    assert!(!codcel_greater_than(9.9, 10.0).unwrap());

    // Zero values
    assert!(codcel_greater_than(0.1, 0.0).unwrap());
    assert!(!codcel_greater_than(0.0, 0.0).unwrap());
    assert!(!codcel_greater_than(-0.1, 0.0).unwrap());

    // Negative values
    assert!(codcel_greater_than(-5.4, -5.5).unwrap());
    assert!(!codcel_greater_than(-5.5, -5.5).unwrap());
    assert!(!codcel_greater_than(-5.6, -5.5).unwrap());

    // Very small difference
    assert!(codcel_greater_than(0.00000002, 0.00000001).unwrap());
    assert!(!codcel_greater_than(0.00000001, 0.00000001).unwrap());
    assert!(!codcel_greater_than(0.00000001, 0.00000002).unwrap());

    // Very large numbers
    assert!(codcel_greater_than(1e15 + 1.0, 1e15).unwrap());
    assert!(!codcel_greater_than(1e15, 1e15).unwrap());
    assert!(!codcel_greater_than(1e15, 1e15 + 1.0).unwrap());

    // Positive vs negative
    assert!(codcel_greater_than(1.0, -1.0).unwrap());
    assert!(!codcel_greater_than(-1.0, 1.0).unwrap());
}

#[test]
fn test_i32_greater_than() {
    // Basic comparison
    assert!(codcel_greater_than(11, 10).unwrap());
    assert!(!codcel_greater_than(10, 10).unwrap());
    assert!(!codcel_greater_than(9, 10).unwrap());

    // Zero values
    assert!(codcel_greater_than(1, 0).unwrap());
    assert!(!codcel_greater_than(0, 0).unwrap());
    assert!(!codcel_greater_than(-1, 0).unwrap());

    // Negative values
    assert!(codcel_greater_than(-4, -5).unwrap());
    assert!(!codcel_greater_than(-5, -5).unwrap());
    assert!(!codcel_greater_than(-6, -5).unwrap());

    // Min/Max values
    assert!(codcel_greater_than(i32::MAX, i32::MIN).unwrap());
    assert!(codcel_greater_than(i32::MAX, 0).unwrap());
    assert!(codcel_greater_than(0, i32::MIN).unwrap());
    assert!(!codcel_greater_than(i32::MIN, i32::MAX).unwrap());
}

#[test]
fn test_bool_greater_than() {
    // All possible boolean combinations
    assert!(codcel_greater_than(true, false).unwrap());
    assert!(!codcel_greater_than(true, true).unwrap());
    assert!(!codcel_greater_than(false, false).unwrap());
    assert!(!codcel_greater_than(false, true).unwrap());
}

#[test]
fn test_string_greater_than() {
    // Basic comparison
    assert!(codcel_greater_than("world", "hello").unwrap());
    assert!(!codcel_greater_than("hello", "hello").unwrap());
    assert!(!codcel_greater_than("hello", "world").unwrap());

    // Empty strings
    assert!(codcel_greater_than("a", "").unwrap());
    assert!(!codcel_greater_than("", "").unwrap());
    assert!(!codcel_greater_than("", "a").unwrap());

    // Case sensitivity
    assert!(codcel_greater_than("hello", "Hello").unwrap()); // lowercase is greater than uppercase in ASCII
    assert!(!codcel_greater_than("Hello", "hello").unwrap());

    // Special characters
    assert!(codcel_greater_than("z", "!").unwrap());
    assert!(!codcel_greater_than("!", "z").unwrap());

    // Unicode characters
    assert!(codcel_greater_than("さようなら", "こんにちは").unwrap());
    assert!(!codcel_greater_than("こんにちは", "さようなら").unwrap());

    // Lexicographical comparison
    assert!(codcel_greater_than("abc123", "abc122").unwrap());
    assert!(!codcel_greater_than("abc122", "abc123").unwrap());
}

#[test]
fn test_cross_type_number_greater_than() {
    // f64 to i32
    assert!(codcel_greater_than(42.1, 42).unwrap());
    assert!(!codcel_greater_than(42.0, 42).unwrap());
    assert!(!codcel_greater_than(41.9, 42).unwrap());
    assert!(codcel_greater_than(0.1, 0).unwrap());
    assert!(!codcel_greater_than(-10.0, -10).unwrap());
    assert!(codcel_greater_than(-9.9, -10).unwrap());

    // i32 to f64
    assert!(codcel_greater_than(43, 42.0).unwrap());
    assert!(!codcel_greater_than(42, 42.0).unwrap());
    assert!(!codcel_greater_than(41, 42.0).unwrap());
    assert!(codcel_greater_than(1, 0.0).unwrap());
    assert!(!codcel_greater_than(-10, -10.0).unwrap());
    assert!(codcel_greater_than(-9, -10.0).unwrap());

    // Edge cases
    assert!(codcel_greater_than(i32::MAX, (i32::MAX - 1) as f64).unwrap());
    assert!(codcel_greater_than((i32::MIN + 1) as f64, i32::MIN).unwrap());
}

#[test]
fn test_unsupported_type_combinations() {
    // Boolean to number
    assert!(!codcel_greater_than(true, 1).unwrap());
    assert!(!codcel_greater_than(false, 0).unwrap());
    assert!(!codcel_greater_than(1, true).unwrap());
    assert!(!codcel_greater_than(0, false).unwrap());

    // String to number
    assert!(!codcel_greater_than("42", 42).unwrap());
    assert!(!codcel_greater_than(42, "42").unwrap());
    assert!(!codcel_greater_than("42.0", 42.0).unwrap());
    assert!(!codcel_greater_than(42.0, "42.0").unwrap());

    // String to boolean
    assert!(!codcel_greater_than("true", true).unwrap());
    assert!(!codcel_greater_than(true, "true").unwrap());
    assert!(!codcel_greater_than("false", false).unwrap());
    assert!(!codcel_greater_than(false, "false").unwrap());
}

#[test]
fn test_edge_cases() {
    // Different types that might be considered comparable in some contexts
    assert!(!codcel_greater_than("", 0).unwrap());
    assert!(!codcel_greater_than(0, "").unwrap());
    assert!(!codcel_greater_than("", false).unwrap());
    assert!(!codcel_greater_than(false, "").unwrap());

    // Positive vs negative zero
    assert!(!codcel_greater_than(0.0, -0.0).unwrap());
    assert!(!codcel_greater_than(-0.0, 0.0).unwrap());

    // NaN handling (should return false for any comparison)
    assert!(!codcel_greater_than(f64::NAN, 0.0).unwrap());
    assert!(!codcel_greater_than(0.0, f64::NAN).unwrap());
    assert!(!codcel_greater_than(f64::NAN, f64::NAN).unwrap());

    // Infinity handling
    assert!(codcel_greater_than(f64::INFINITY, 0.0).unwrap());
    assert!(codcel_greater_than(f64::INFINITY, f64::MAX).unwrap());
    assert!(!codcel_greater_than(0.0, f64::INFINITY).unwrap());
    assert!(!codcel_greater_than(f64::NEG_INFINITY, 0.0).unwrap());
    assert!(codcel_greater_than(0.0, f64::NEG_INFINITY).unwrap());
    assert!(codcel_greater_than(f64::INFINITY, f64::NEG_INFINITY).unwrap());
    assert!(!codcel_greater_than(f64::NEG_INFINITY, f64::INFINITY).unwrap());
    assert!(!codcel_greater_than(f64::INFINITY, f64::INFINITY).unwrap());
    assert!(!codcel_greater_than(f64::NEG_INFINITY, f64::NEG_INFINITY).unwrap());
}
