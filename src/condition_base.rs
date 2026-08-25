// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::compensated_sum::{CompensatedSum, CompensatedSumExt};
use crate::condition::{parse_condition, Condition};
use crate::logical::codcel_and::codcel_and;
use crate::logical::codcel_or::codcel_or;
use crate::logical::codcel_xor::codcel_xor;
use crate::match_type_and_compare_macro::compare;
use crate::value::{flatten_value_to_partial_vec, vec_value_to_vec_boolean, Value};
use crate::value_format::ValueFormat;
use std::error::Error;

pub fn equals(
    input_lhs: Value,
    input_rhs: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    compare(&input_lhs, &input_rhs, &Condition::Equal, value_format)
}

pub fn not_equal(
    input_lhs: Value,
    input_rhs: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    compare(&input_lhs, &input_rhs, &Condition::NotEqual, value_format)
}

pub fn greater_than(
    input_lhs: Value,
    input_rhs: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    compare(
        &input_lhs,
        &input_rhs,
        &Condition::GreaterThan,
        value_format,
    )
}

pub fn less_than(
    input_lhs: Value,
    input_rhs: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    compare(&input_lhs, &input_rhs, &Condition::LessThan, value_format)
}

pub fn less_than_or_equal(
    input_lhs: Value,
    input_rhs: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    compare(
        &input_lhs,
        &input_rhs,
        &Condition::LessThanOrEqual,
        value_format,
    )
}

pub fn greater_than_or_equal(
    input_lhs: Value,
    input_rhs: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    compare(
        &input_lhs,
        &input_rhs,
        &Condition::GreaterThanOrEqual,
        value_format,
    )
}

pub fn logical_and(
    input_lhs: Value,
    input_rhs: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let input1 = input_lhs.to_single_value();
    let input2 = input_rhs.to_single_value();
    Ok(Value::Bool(
        input1.bool(value_format)? && input2.bool(value_format)?,
    ))
}

pub fn logical_or(
    input_lhs: Value,
    input_rhs: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let input1 = input_lhs.to_single_value();
    let input2 = input_rhs.to_single_value();
    Ok(Value::Bool(
        input1.bool(value_format)? || input2.bool(value_format)?,
    ))
}

pub fn xor(
    values: Vec<Value>,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Ok(Value::Bool(false));
    }

    let all_values = vec_value_to_vec_boolean(values, strict_type_conversion, value_format)?;
    Ok(Value::Bool(codcel_xor(all_values)?))
}

pub fn and(
    values: Vec<Value>,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Ok(Value::Bool(false));
    }

    let all_values = vec_value_to_vec_boolean(values, strict_type_conversion, value_format)?;
    Ok(Value::Bool(codcel_and(all_values)?))
}

pub fn or(
    values: Vec<Value>,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Ok(Value::Bool(false));
    }

    let all_values = vec_value_to_vec_boolean(values, strict_type_conversion, value_format)?;
    Ok(Value::Bool(codcel_or(all_values)?))
}

pub fn count_if(
    values: Value,
    condition: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    // Extract the area of values and the condition string
    let values_area = values.area_of_value()?;
    let condition_string = condition.string(value_format)?;

    // Parse the condition and the value to compare against
    // Excel conditional functions (AVERAGEIF, SUMIF, COUNTIF) are case-insensitive
    let (condition_operator, comparison_value) = parse_condition(&condition_string);
    let comparison_value = Value::String(comparison_value.to_lowercase());

    // Count the cells matching the condition
    let count = values_area
        .iter()
        .flat_map(|row| row.iter())
        .filter(|cell| {
            let cell_value = case_insensitive_cell(cell, value_format);
            compare(
                &cell_value,
                &comparison_value,
                &condition_operator,
                value_format,
            )
            .map(|result| result.bool(value_format).unwrap_or(false))
            .unwrap_or(false)
        })
        .count() as i32;

    Ok(Value::I32(count))
}

pub fn average_if(
    values: Value,
    condition: Value,
    average_range_values: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    // Extract the area of values and the condition string
    let values_area = values.area_of_value()?;
    let condition_string = condition.string(value_format)?;

    // Parse the condition and the value to compare against
    // Excel conditional functions (AVERAGEIF, SUMIF, COUNTIF) are case-insensitive
    let (condition_operator, comparison_value) = parse_condition(&condition_string);
    let comparison_value = Value::String(comparison_value.to_lowercase());

    // Determine the averaging area: use the optional average_range if provided, otherwise use values_area
    let average_area =
        if let Some(average_range_value) = average_range_values.option_area_of_value()? {
            average_range_value
        } else {
            values_area.clone()
        };

    // Ensure the dimensions of the values_area and average_area match
    if average_area.len() != values_area.len()
        || average_area
            .iter()
            .zip(values_area.iter())
            .any(|(average_row, values_row)| average_row.len() != values_row.len())
    {
        return Err(
            "AVERAGEIF: The dimensions of the average range do not match the values range.".into(),
        );
    }

    // Sum the values of cells in the average_area matching the condition in values_area
    let mut sum = CompensatedSum::new();
    let mut count = 0;

    for (values_row, average_row) in values_area.iter().zip(average_area.iter()) {
        for (value_cell, average_cell) in values_row.iter().zip(average_row.iter()) {
            // Lowercase string values for case-insensitive matching
            let cell_value = case_insensitive_cell(value_cell, value_format);
            if compare(
                &cell_value,
                &comparison_value,
                &condition_operator,
                value_format,
            )
            .map(|result| result.bool(value_format).unwrap_or(false))
            .unwrap_or(false)
            {
                // Extract the numeric value of the average_cell
                if let Ok(avg_value) = average_cell.f64(value_format) {
                    sum.add(avg_value);
                    count += 1;
                }
            }
        }
    }

    if count > 0 {
        Ok(Value::F64(sum.total() / count as f64))
    } else {
        // Return a default Value, e.g., 0.0, if no matching cells are found
        Ok(Value::F64(0.0))
    }
}

/// Lowercase a cell value for case-insensitive conditional matching.
/// Non-string values are returned as-is.
fn case_insensitive_cell(value: &Value, value_format: &ValueFormat) -> Value {
    if value.is_string() {
        if let Ok(s) = value.string(value_format) {
            return Value::String(s.to_lowercase());
        }
    }
    value.clone()
}

pub fn sum_ifs(
    sum_range_values: Value,
    criteria: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let sum_range_values = flatten_value_to_partial_vec(sum_range_values);
    let criteria = criteria.vec_value()?;

    // Process criteria and collect matching indexes
    let matching_indexes = process_criteria("SUMIFS", &criteria, value_format)?;

    // Find indexes that match all criteria
    let common_indexes = if matching_indexes.is_empty() {
        Vec::new()
    } else {
        matching_indexes
            .iter()
            .skip(1)
            .fold(matching_indexes[0].clone(), |acc, indexes| {
                acc.into_iter()
                    .filter(|idx| indexes.contains(idx))
                    .collect()
            })
    };

    // Sum values at matching indexes, with compensation so long ranges do not drift
    let mut sum = CompensatedSum::new();
    for &idx in &common_indexes {
        sum.add(sum_range_values[idx].f64(value_format)?);
    }

    Ok(Value::F64(sum.total()))
}

/// Helper function to process criteria and return matching indexes
fn process_criteria(
    function_name: &str,
    criteria: &[Value],
    value_format: &ValueFormat,
) -> Result<Vec<Vec<usize>>, Box<dyn Error + Send + Sync>> {
    if criteria.is_empty() || !criteria.len().is_multiple_of(2) {
        return Err(format!("{function_name}: Criteria must be provided in pairs").into());
    }

    let mut indexes = Vec::with_capacity(criteria.len() / 2);

    for pair in criteria.chunks(2) {
        // Extract criteria range and validate its length
        let criteria_range = flatten_value_to_partial_vec(pair[0].clone());

        // Parse condition and identify matching indexes
        // Excel conditional functions are case-insensitive
        let condition_string = pair[1].string(value_format)?;
        let (condition_operator, comparison_value) = parse_condition(&condition_string);
        let comparison_value = Value::String(comparison_value.to_lowercase());

        let matched_indexes: Vec<usize> = criteria_range
            .iter()
            .enumerate()
            .filter_map(|(index, value_cell)| {
                let cell_value = case_insensitive_cell(value_cell, value_format);
                match compare(
                    &cell_value,
                    &comparison_value,
                    &condition_operator,
                    value_format,
                ) {
                    Ok(result) if result.bool(value_format).unwrap_or(false) => Some(index),
                    _ => None,
                }
            })
            .collect();

        indexes.push(matched_indexes);
    }

    Ok(indexes)
}

pub fn average_ifs(
    average_range_values: Value,
    criteria: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let average_range_values = flatten_value_to_partial_vec(average_range_values);
    let criteria = criteria.vec_value()?;

    // Process criteria and collect matching indexes
    let matching_indexes = process_criteria("AVERAGEIFS", &criteria, value_format)?;

    // Find indexes that match all criteria
    let common_indexes = if matching_indexes.is_empty() {
        Vec::new()
    } else {
        matching_indexes
            .iter()
            .skip(1)
            .fold(matching_indexes[0].clone(), |acc, indexes| {
                acc.into_iter()
                    .filter(|idx| indexes.contains(idx))
                    .collect()
            })
    };

    // Calculate the average of values at matching indexes
    if common_indexes.is_empty() {
        Ok(Value::F64(0.0)) // Return 0.0 if no matching cells are found
    } else {
        let mut sum = CompensatedSum::new();
        let mut count: usize = 0;
        for &idx in &common_indexes {
            sum.add(average_range_values[idx].f64(value_format)?);
            count += 1;
        }
        Ok(Value::F64(sum.total() / count as f64))
    }
}

pub fn count_ifs(
    criteria: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let criteria = criteria.vec_value()?;

    // Process criteria and collect matching indexes
    let matching_indexes = process_criteria("COUNTIFS", &criteria, value_format)?;

    // Find indexes that match all criteria
    let common_indexes = if matching_indexes.is_empty() {
        Vec::new()
    } else {
        matching_indexes
            .iter()
            .skip(1)
            .fold(matching_indexes[0].clone(), |acc, indexes| {
                acc.into_iter()
                    .filter(|idx| indexes.contains(idx))
                    .collect()
            })
    };

    // Count the matching indexes
    Ok(Value::I32(common_indexes.len() as i32))
}

pub fn max_ifs(
    max_range_values: Value,
    criteria: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let max_range_values = flatten_value_to_partial_vec(max_range_values);
    let criteria = criteria.vec_value()?;

    // Process criteria and collect matching indexes
    let matching_indexes = process_criteria("MAXIFS", &criteria, value_format)?;

    // Find indexes that match all criteria
    let common_indexes = if matching_indexes.is_empty() {
        Vec::new()
    } else {
        matching_indexes
            .iter()
            .skip(1)
            .fold(matching_indexes[0].clone(), |acc, indexes| {
                acc.into_iter()
                    .filter(|idx| indexes.contains(idx))
                    .collect()
            })
    };

    // Find the maximum value among the matching indexes
    if common_indexes.is_empty() {
        Ok(Value::F64(f64::NAN)) // Return NaN if no matching cells are found
    } else {
        let max_value = common_indexes.iter().try_fold(
            f64::NEG_INFINITY,
            |current_max, &idx| -> Result<f64, Box<dyn Error + Send + Sync>> {
                let value = max_range_values[idx].f64(value_format)?;
                Ok(f64::max(current_max, value))
            },
        )?;
        Ok(Value::F64(max_value))
    }
}

pub fn min_ifs(
    min_range_values: Value,
    criteria: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let min_range_values = flatten_value_to_partial_vec(min_range_values);
    let criteria = criteria.vec_value()?;

    // Process criteria and collect matching indexes
    let matching_indexes = process_criteria("MINIFS", &criteria, value_format)?;

    // Find indexes that match all criteria
    let common_indexes = if matching_indexes.is_empty() {
        Vec::new()
    } else {
        matching_indexes
            .iter()
            .skip(1)
            .fold(matching_indexes[0].clone(), |acc, indexes| {
                acc.into_iter()
                    .filter(|idx| indexes.contains(idx))
                    .collect()
            })
    };

    // Find the minimum value among the matching indexes
    if common_indexes.is_empty() {
        Ok(Value::F64(f64::NAN)) // Return NaN if no matching cells are found
    } else {
        let min_value = common_indexes.iter().try_fold(
            f64::INFINITY,
            |current_min, &idx| -> Result<f64, Box<dyn Error + Send + Sync>> {
                let value = min_range_values[idx].f64(value_format)?;
                Ok(f64::min(current_min, value))
            },
        )?;
        Ok(Value::F64(min_value))
    }
}

pub fn sum_if(
    values: Value,
    condition: Value,
    sum_range_values: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    // Extract the area of values and the condition string
    let values_area = values.area_of_value()?;
    let condition_string = condition.string(value_format)?;

    // Parse the condition and the value to compare against
    // Excel conditional functions (AVERAGEIF, SUMIF, COUNTIF) are case-insensitive
    let (condition_operator, comparison_value) = parse_condition(&condition_string);
    let comparison_value = Value::String(comparison_value.to_lowercase());

    // Determine the summation area: use the optional sum_range if provided, otherwise use values_area
    let sum_area = if let Some(sum_range_value) = sum_range_values.option_area_of_value()? {
        sum_range_value
    } else {
        values_area.clone()
    };

    // Ensure the dimensions of the values_area and sum_area match
    if sum_area.len() != values_area.len()
        || sum_area
            .iter()
            .zip(values_area.iter())
            .any(|(sum_row, values_row)| sum_row.len() != values_row.len())
    {
        return Err("The dimensions of the sum range do not match the values range.".into());
    }

    // Sum the values of cells in the sum_area matching the condition in values_area
    let sum = values_area
        .iter()
        .zip(sum_area.iter())
        .flat_map(|(values_row, sum_row)| values_row.iter().zip(sum_row.iter()))
        .filter_map(|(value_cell, sum_cell)| {
            let cell_value = case_insensitive_cell(value_cell, value_format);
            if compare(
                &cell_value,
                &comparison_value,
                &condition_operator,
                value_format,
            )
            .map(|result| result.bool(value_format).unwrap_or(false))
            .unwrap_or(false)
            {
                // Extract the numeric value of the sum_cell
                sum_cell.f64(value_format).ok()
            } else {
                None
            }
        })
        .compensated_sum();

    Ok(Value::F64(sum))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::{area_f64 as value_area_f64, str as value_str};
    use crate::value_format::ValueFormat;

    fn create_value_format() -> ValueFormat {
        ValueFormat {
            use_excel_rounding: true,
            ..Default::default()
        }
    }

    /// A column of `count` cells each holding 0.1, as a range.
    fn tenths_column(count: usize) -> Value {
        value_area_f64((0..count).map(|_| vec![0.1]).collect())
    }

    /// A column of `count` cells each holding 1, as a range.
    fn ones_column(count: usize) -> Value {
        value_area_f64((0..count).map(|_| vec![1.0]).collect())
    }

    #[test]
    fn test_sum_if_matches_all_cells() {
        let value_format = create_value_format();

        // =SUMIF(A1:A3,">0",B1:B3) in US format
        // =SUMIF(A1:A3;">0";B1:B3) in German format
        let result = sum_if(
            value_area_f64(vec![vec![1.0], vec![2.0], vec![3.0]]),
            value_str(">0"),
            value_area_f64(vec![vec![10.0], vec![20.0], vec![30.0]]),
            &value_format,
        )
        .unwrap();
        assert_eq!(result.f64(&value_format).unwrap(), 60.0);
    }

    #[test]
    fn test_sum_if_excludes_non_matching_cells() {
        let value_format = create_value_format();

        // =SUMIF(A1:A3,">1",B1:B3) in US format
        // =SUMIF(A1:A3;">1";B1:B3) in German format
        let result = sum_if(
            value_area_f64(vec![vec![1.0], vec![2.0], vec![3.0]]),
            value_str(">1"),
            value_area_f64(vec![vec![10.0], vec![20.0], vec![30.0]]),
            &value_format,
        )
        .unwrap();
        assert_eq!(result.f64(&value_format).unwrap(), 50.0);
    }

    #[test]
    fn test_sum_if_has_no_drift_over_long_range() {
        let value_format = create_value_format();

        // =SUMIF(A1:A10000,">0",B1:B10000) in US format
        // =SUMIF(A1:A10000;">0";B1:B10000) in German format
        let result = sum_if(
            ones_column(10_000),
            value_str(">0"),
            tenths_column(10_000),
            &value_format,
        )
        .unwrap();
        assert_eq!(result.f64(&value_format).unwrap(), 1000.0);
    }

    #[test]
    fn test_average_if_matches_all_cells() {
        let value_format = create_value_format();

        // =AVERAGEIF(A1:A3,">0",B1:B3) in US format
        // =AVERAGEIF(A1:A3;">0";B1:B3) in German format
        let result = average_if(
            value_area_f64(vec![vec![1.0], vec![2.0], vec![3.0]]),
            value_str(">0"),
            value_area_f64(vec![vec![10.0], vec![20.0], vec![30.0]]),
            &value_format,
        )
        .unwrap();
        assert_eq!(result.f64(&value_format).unwrap(), 20.0);
    }

    #[test]
    fn test_average_if_has_no_drift_over_long_range() {
        let value_format = create_value_format();

        // =AVERAGEIF(A1:A10000,">0",B1:B10000) in US format
        // =AVERAGEIF(A1:A10000;">0";B1:B10000) in German format
        let result = average_if(
            ones_column(10_000),
            value_str(">0"),
            tenths_column(10_000),
            &value_format,
        )
        .unwrap();
        assert_eq!(result.f64(&value_format).unwrap(), 0.1);
    }

    #[test]
    fn test_sum_ifs_matches_all_cells() {
        let value_format = create_value_format();

        // =SUMIFS(B1:B3,A1:A3,">0") in US format
        // =SUMIFS(B1:B3;A1:A3;">0") in German format
        let result = sum_ifs(
            value_area_f64(vec![vec![10.0], vec![20.0], vec![30.0]]),
            Value::VecValue(vec![
                value_area_f64(vec![vec![1.0], vec![2.0], vec![3.0]]),
                value_str(">0"),
            ]),
            &value_format,
        )
        .unwrap();
        assert_eq!(result.f64(&value_format).unwrap(), 60.0);
    }

    #[test]
    fn test_sum_ifs_has_no_drift_over_long_range() {
        let value_format = create_value_format();

        // =SUMIFS(B1:B10000,A1:A10000,">0") in US format
        // =SUMIFS(B1:B10000;A1:A10000;">0") in German format
        let result = sum_ifs(
            tenths_column(10_000),
            Value::VecValue(vec![ones_column(10_000), value_str(">0")]),
            &value_format,
        )
        .unwrap();
        assert_eq!(result.f64(&value_format).unwrap(), 1000.0);
    }

    #[test]
    fn test_average_ifs_has_no_drift_over_long_range() {
        let value_format = create_value_format();

        // =AVERAGEIFS(B1:B10000,A1:A10000,">0") in US format
        // =AVERAGEIFS(B1:B10000;A1:A10000;">0") in German format
        let result = average_ifs(
            tenths_column(10_000),
            Value::VecValue(vec![ones_column(10_000), value_str(">0")]),
            &value_format,
        )
        .unwrap();
        assert_eq!(result.f64(&value_format).unwrap(), 0.1);
    }
}
