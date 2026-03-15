// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::any::Any;
use std::cmp::Ordering;
use std::error::Error;

/// Excel-style less-than (`<`) comparison across common primitive types.
///
/// Supports `f64`, `i32`, `bool`, and `&str`. Numeric inputs convert `i32` to
/// `f64` to allow cross-type comparisons, while booleans and strings use their
/// inherent ordering. Unsupported or mismatched type pairs return `Ok(false)`
/// rather than producing an error.
pub fn codcel_less_than<T: 'static + Any, U: 'static + Any>(
    input1: T,
    input2: U,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    let input1 = &input1 as &dyn Any;
    let input2 = &input2 as &dyn Any;

    if let Some(val) = input1.downcast_ref::<f64>() {
        if let Some(val2) = input2.downcast_ref::<f64>() {
            return less_than_internal(*val, *val2);
        } else if let Some(val2) = input2.downcast_ref::<i32>() {
            return less_than_internal(*val, *val2 as f64);
        }
    } else if let Some(val) = input1.downcast_ref::<i32>() {
        if let Some(val2) = input2.downcast_ref::<i32>() {
            return less_than_internal(*val, *val2);
        } else if let Some(val2) = input2.downcast_ref::<f64>() {
            return less_than_internal(*val as f64, *val2);
        }
    } else if let Some(val) = input1.downcast_ref::<bool>() {
        if let Some(val2) = input2.downcast_ref::<bool>() {
            return less_than_internal(val, val2);
        }
    } else if let Some(val) = input1.downcast_ref::<&str>() {
        if let Some(val2) = input2.downcast_ref::<&str>() {
            return less_than_internal(val, val2);
        }
    }

    // To think about, I guess if the types don´t match then obviously they cannot be equal
    Ok(false)
}

// For less than i32 must be converted into float/number
fn less_than_internal<T, U>(input1: T, input2: U) -> Result<bool, Box<dyn Error + Send + Sync>>
where
    T: PartialOrd<U>,
{
    match input1.partial_cmp(&input2).map(|ord| ord == Ordering::Less) {
        None => Ok(false),
        Some(result) => Ok(result),
    }
}

#[test]
fn test_less_than() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Numerical comparisons
    assert!(codcel_less_than(5, 10)?);
    assert!(!(codcel_less_than(10, 5)?));
    assert!(!(codcel_less_than(10, 10)?));

    // String comparisons
    assert!(codcel_less_than("apple", "banana")?);
    assert!(!(codcel_less_than("banana", "apple")?));
    assert!(!(codcel_less_than("apple", "apple")?));

    Ok(())
}

#[test]
fn test_f64_less_than() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Basic comparison
    assert!(codcel_less_than(10.0, 10.1)?);
    assert!(!(codcel_less_than(10.0, 10.0)?));
    assert!(!(codcel_less_than(10.0, 9.9)?));

    // Zero values
    assert!(codcel_less_than(0.0, 0.1)?);
    assert!(!(codcel_less_than(0.0, 0.0)?));
    assert!(!(codcel_less_than(0.0, -0.1)?));

    // Negative values
    assert!(codcel_less_than(-5.5, -5.4)?);
    assert!(!(codcel_less_than(-5.5, -5.5)?));
    assert!(!(codcel_less_than(-5.5, -5.6)?));

    // Very small difference
    assert!(codcel_less_than(0.00000001, 0.00000002)?);
    assert!(!(codcel_less_than(0.00000001, 0.00000001)?));
    assert!(!(codcel_less_than(0.00000002, 0.00000001)?));

    // Very large numbers
    assert!(codcel_less_than(1e15, 1e15 + 1.0)?);
    assert!(!(codcel_less_than(1e15, 1e15)?));
    assert!(!(codcel_less_than(1e15 + 1.0, 1e15)?));

    // Positive vs negative
    assert!(codcel_less_than(-1.0, 1.0)?);
    assert!(!(codcel_less_than(1.0, -1.0)?));

    Ok(())
}

#[test]
fn test_i32_less_than() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Basic comparison
    assert!(codcel_less_than(10, 11)?);
    assert!(!(codcel_less_than(10, 10)?));
    assert!(!(codcel_less_than(10, 9)?));

    // Zero values
    assert!(codcel_less_than(0, 1)?);
    assert!(!(codcel_less_than(0, 0)?));
    assert!(!(codcel_less_than(0, -1)?));

    // Negative values
    assert!(codcel_less_than(-5, -4)?);
    assert!(!(codcel_less_than(-5, -5)?));
    assert!(!(codcel_less_than(-5, -6)?));

    // Min/Max values
    assert!(codcel_less_than(i32::MIN, i32::MAX)?);
    assert!(codcel_less_than(i32::MIN, 0)?);
    assert!(codcel_less_than(0, i32::MAX)?);
    assert!(!(codcel_less_than(i32::MAX, i32::MIN)?));

    Ok(())
}

#[test]
fn test_bool_less_than() -> Result<(), Box<dyn Error + Send + Sync>> {
    // All possible boolean combinations
    assert!(codcel_less_than(false, true)?);
    assert!(!(codcel_less_than(true, true)?));
    assert!(!(codcel_less_than(false, false)?));
    assert!(!(codcel_less_than(true, false)?));

    Ok(())
}

#[test]
fn test_string_less_than() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Basic comparison
    assert!(codcel_less_than("hello", "world")?);
    assert!(!(codcel_less_than("hello", "hello")?));
    assert!(!(codcel_less_than("world", "hello")?));

    // Empty strings
    assert!(codcel_less_than("", "a")?);
    assert!(!(codcel_less_than("", "")?));
    assert!(!(codcel_less_than("a", "")?));

    // Case sensitivity
    assert!(codcel_less_than("Hello", "hello")?); // uppercase is less than lowercase in ASCII
    assert!(!(codcel_less_than("hello", "Hello")?));

    // Special characters
    assert!(codcel_less_than("!", "z")?);
    assert!(!(codcel_less_than("z", "!")?));

    // Unicode characters
    assert!(codcel_less_than("こんにちは", "さようなら")?);
    assert!(!(codcel_less_than("さようなら", "こんにちは")?));

    // Lexicographical comparison
    assert!(codcel_less_than("abc122", "abc123")?);
    assert!(!(codcel_less_than("abc123", "abc122")?));

    Ok(())
}

#[test]
fn test_cross_type_number_less_than() -> Result<(), Box<dyn Error + Send + Sync>> {
    // f64 to i32
    assert!(codcel_less_than(41.9, 42)?);
    assert!(!(codcel_less_than(42.0, 42)?));
    assert!(!(codcel_less_than(42.1, 42)?));
    assert!(codcel_less_than(-0.1, 0)?);
    assert!(!(codcel_less_than(-10.0, -10)?));
    assert!(!(codcel_less_than(-9.9, -10)?));

    // i32 to f64
    assert!(codcel_less_than(41, 42.0)?);
    assert!(!(codcel_less_than(42, 42.0)?));
    assert!(!(codcel_less_than(43, 42.0)?));
    assert!(codcel_less_than(0, 0.1)?);
    assert!(!(codcel_less_than(-10, -10.0)?));
    assert!(!(codcel_less_than(-9, -10.0)?));

    // Edge cases
    assert!(codcel_less_than((i32::MAX - 1) as f64, i32::MAX)?);
    assert!(codcel_less_than(i32::MIN, (i32::MIN + 1) as f64)?);

    Ok(())
}

#[test]
fn test_unsupported_type_combinations() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Boolean to number
    assert!(!(codcel_less_than(true, 1)?));
    assert!(!(codcel_less_than(false, 0)?));
    assert!(!(codcel_less_than(1, true)?));
    assert!(!(codcel_less_than(0, false)?));

    // String to number
    assert!(!(codcel_less_than("42", 42)?));
    assert!(!(codcel_less_than(42, "42")?));
    assert!(!(codcel_less_than("42.0", 42.0)?));
    assert!(!(codcel_less_than(42.0, "42.0")?));

    // String to boolean
    assert!(!(codcel_less_than("true", true)?));
    assert!(!(codcel_less_than(true, "true")?));
    assert!(!(codcel_less_than("false", false)?));
    assert!(!(codcel_less_than(false, "false")?));

    Ok(())
}

#[test]
fn test_edge_cases() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Different types that might be considered comparable in some contexts
    assert!(!(codcel_less_than("", 0)?));
    assert!(!(codcel_less_than(0, "")?));
    assert!(!(codcel_less_than("", false)?));
    assert!(!(codcel_less_than(false, "")?));

    // Positive vs negative zero
    assert!(!(codcel_less_than(0.0, -0.0)?));
    assert!(!(codcel_less_than(-0.0, 0.0)?));

    // NaN handling (should return false for any comparison)
    assert!(!(codcel_less_than(f64::NAN, 0.0)?));
    assert!(!(codcel_less_than(0.0, f64::NAN)?));
    assert!(!(codcel_less_than(f64::NAN, f64::NAN)?));

    // Infinity handling
    assert!(codcel_less_than(0.0, f64::INFINITY)?);
    assert!(codcel_less_than(f64::MAX, f64::INFINITY)?);
    assert!(!(codcel_less_than(f64::INFINITY, 0.0)?));
    assert!(codcel_less_than(f64::NEG_INFINITY, 0.0)?);
    assert!(!(codcel_less_than(0.0, f64::NEG_INFINITY)?));
    assert!(codcel_less_than(f64::NEG_INFINITY, f64::INFINITY)?);
    assert!(!(codcel_less_than(f64::INFINITY, f64::NEG_INFINITY)?));
    assert!(!(codcel_less_than(f64::INFINITY, f64::INFINITY)?));
    assert!(!(codcel_less_than(f64::NEG_INFINITY, f64::NEG_INFINITY)?));

    Ok(())
}
