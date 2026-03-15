// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::condition::Condition;
use crate::value::Value;
use crate::value_format::ValueFormat;
use std::error::Error;

macro_rules! implement_comparisons {
    ($func_name:ident, $op:tt) => {
        fn $func_name<T: PartialOrd + PartialEq>(lhs: T, rhs: T) -> Result<Value, Box<dyn Error + Send + Sync>> {
            Ok(Value::Bool(lhs $op rhs))
        }
    }
}

// Implement all comparison functions with one line each
implement_comparisons!(greater_than, >);
implement_comparisons!(less_than, <);
implement_comparisons!(equals, ==);
implement_comparisons!(greater_equals, >=);
implement_comparisons!(less_equals, <=);
implement_comparisons!(not_equals, !=);

// Then for the comparison function itself
macro_rules! match_type_and_compare {
    ($input1:expr, $input2:expr, $value_format:expr, $compare_fn:ident) => {
        match $input1 {
            Value::F64(_) | Value::ChronoDateTime(_) | Value::Time(_) => {
                // Time and date are f64 in Excel for comparisons
                $compare_fn::<f64>($input1.f64($value_format)?, $input2.f64($value_format)?)
            }
            Value::I32(_) => {
                $compare_fn::<i32>($input1.i32($value_format)?, $input2.i32($value_format)?)
            }
            Value::Bool(_) => {
                $compare_fn::<bool>($input1.bool($value_format)?, $input2.bool($value_format)?)
            }
            _ => $compare_fn::<String>(
                $input1.string($value_format)?,
                $input2.string($value_format)?,
            ),
        }
    };
}

fn compare_inner(
    input_lhs: &Value,
    input_rhs: &Value,
    comparison_type: &Condition,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let input1 = input_lhs.to_single_value();
    let input2 = input_rhs.to_single_value();

    match comparison_type {
        Condition::GreaterThan => {
            match_type_and_compare!(input1, input2, value_format, greater_than)
        }
        Condition::LessThan => match_type_and_compare!(input1, input2, value_format, less_than),
        Condition::Equal => {
            if input2.is_string() {
                let input_b = input2.string(value_format)?;
                if input_b.contains('*') || input_b.contains('?') {
                    return Ok(Value::Bool(equals_wildcard_match(
                        &input_b,
                        &input1.string(value_format)?,
                    )));
                }
            }
            match_type_and_compare!(input1, input2, value_format, equals)
        }
        Condition::NotEqual => match_type_and_compare!(input1, input2, value_format, not_equals),
        Condition::LessThanOrEqual => {
            match_type_and_compare!(input1, input2, value_format, less_equals)
        }
        Condition::GreaterThanOrEqual => {
            match_type_and_compare!(input1, input2, value_format, greater_equals)
        }
    }
}

// Wildcard matches only work on strings and for equals in excel
fn equals_wildcard_match(pattern: &str, text: &str) -> bool {
    let mut pattern_iter = pattern.chars().peekable();
    let mut text_iter = text.chars().peekable();

    while let Some(&p) = pattern_iter.peek() {
        match p {
            '?' => {
                // Match any single character
                if text_iter.next().is_none() {
                    return false;
                }
                pattern_iter.next(); // Consume '?'
            }
            '*' => {
                // Match zero or more characters
                pattern_iter.next(); // Consume '*'

                if pattern_iter.peek().is_none() {
                    // If '*' is the last character, it matches everything else
                    return true;
                }

                // Try to match the rest of the pattern at each position in text
                while text_iter.peek().is_some() {
                    if equals_wildcard_match(
                        pattern_iter.clone().collect::<String>().as_str(),
                        text_iter.clone().collect::<String>().as_str(),
                    ) {
                        return true;
                    }
                    text_iter.next();
                }

                return false;
            }
            _ => {
                // Match exact characters
                if text_iter.next() != Some(p) {
                    return false;
                }
                pattern_iter.next();
            }
        }
    }

    // Ensure no leftover characters in the text
    text_iter.next().is_none()
}

pub fn compare(
    input_lhs: &Value,
    input_rhs: &Value,
    comparison_type: &Condition,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    // Handle element-wise comparison when lhs is an array
    match input_lhs {
        Value::VecValue(vec) => {
            // Compare each element of the array with the rhs value
            let mut results = Vec::with_capacity(vec.len());
            for elem in vec {
                let result = compare_single(elem, input_rhs, comparison_type, value_format)?;
                results.push(result);
            }
            return Ok(Value::VecValue(results));
        }
        Value::AreaValue(rows) => {
            // Compare each element of the 2D array with the rhs value
            let mut result_rows = Vec::with_capacity(rows.len());
            for row in rows {
                let mut result_row = Vec::with_capacity(row.len());
                for elem in row {
                    let result = compare_single(elem, input_rhs, comparison_type, value_format)?;
                    result_row.push(result);
                }
                result_rows.push(result_row);
            }
            return Ok(Value::AreaValue(result_rows));
        }
        _ => {}
    }

    // Single value comparison
    compare_single(input_lhs, input_rhs, comparison_type, value_format)
}

fn compare_single(
    input_lhs: &Value,
    input_rhs: &Value,
    comparison_type: &Condition,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if let Ok(result) = compare_inner(input_lhs, input_rhs, comparison_type, value_format) {
        Ok(result)
    } else {
        // Force a string compare to see if it can be done
        let value_lhs = input_lhs.string(value_format)?;
        let value_rhs = input_rhs.string(value_format)?;
        match comparison_type {
            Condition::Equal => {
                if value_rhs.contains('*') || value_rhs.contains('?') {
                    return Ok(Value::Bool(equals_wildcard_match(&value_rhs, &value_lhs)));
                }

                Ok(Value::Bool(value_lhs == value_rhs))
            }
            Condition::LessThan => Ok(Value::Bool(value_lhs < value_rhs)),
            Condition::GreaterThan => Ok(Value::Bool(value_lhs > value_rhs)),
            Condition::NotEqual => Ok(Value::Bool(value_lhs != value_rhs)),
            Condition::LessThanOrEqual => Ok(Value::Bool(value_lhs <= value_rhs)),
            Condition::GreaterThanOrEqual => Ok(Value::Bool(value_lhs >= value_rhs)),
        }
    }
}
