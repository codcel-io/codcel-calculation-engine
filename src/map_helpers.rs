/*!
SPDX-FileCopyrightText: Copyright (c) 2026 Codcel.
SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial

Helper functions for MAP operations over Value arrays.

These functions enable applying async transformations to arrays while preserving
their structure (1D VecValue or 2D AreaValue).
*/

use std::error::Error;
use std::future::Future;
use crate::value::Value;

/// Maps an async function over a Value, preserving structure (1D vs 2D).
///
/// - `VecValue` -> iterates elements, returns `VecValue`
/// - `AreaValue` -> iterates elements preserving row/col structure, returns `AreaValue`
/// - Single value -> applies function directly, returns single result
///
/// # Example
/// ```ignore
/// map_value(array, |elem| async move {
///     let lambda_param_x = elem;
///     Ok(map_lambda_xxx(input, lambda_param_x).await?)
/// }).await?
/// ```
pub async fn map_value<F, Fut>(
    value: Value,
    f: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(Value) -> Fut,
    Fut: Future<Output = Result<Value, Box<dyn Error + Send + Sync>>>,
{
    match value {
        Value::VecValue(vec) => {
            let mut results = Vec::with_capacity(vec.len());
            for elem in vec {
                results.push(f(elem).await?);
            }
            Ok(Value::VecValue(results))
        }
        Value::AreaValue(rows) => {
            let mut area_results = Vec::with_capacity(rows.len());
            for row in rows {
                let mut row_results = Vec::with_capacity(row.len());
                for elem in row {
                    row_results.push(f(elem).await?);
                }
                area_results.push(row_results);
            }
            Ok(Value::AreaValue(area_results))
        }
        single => f(single).await,
    }
}

/// Maps an async function over multiple arrays, zipping element-by-element.
///
/// Arrays are flattened to 1D for zipping (AreaValue becomes row-major order).
/// Returns a `VecValue` with the mapped results.
///
/// # Example
/// ```ignore
/// map_values_zipped(vec![arr1, arr2], |elems| async move {
///     let lambda_param_a = elems[0].clone();
///     let lambda_param_b = elems[1].clone();
///     Ok(map_lambda_xxx(input, lambda_param_a, lambda_param_b).await?)
/// }).await?
/// ```
pub async fn map_values_zipped<F, Fut>(
    arrays: Vec<Value>,
    f: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(Vec<Value>) -> Fut,
    Fut: Future<Output = Result<Value, Box<dyn Error + Send + Sync>>>,
{
    // Convert all arrays to Vec<Value> for zipping
    let vecs: Vec<Vec<Value>> = arrays
        .into_iter()
        .map(|arr| match arr {
            Value::VecValue(v) => v,
            Value::AreaValue(a) => a.into_iter().flatten().collect(),
            other => vec![other],
        })
        .collect();

    // Determine the length (use first array's length)
    let len = vecs.first().map(|v| v.len()).unwrap_or(0);
    let mut results = Vec::with_capacity(len);

    for idx in 0..len {
        let elements: Vec<Value> = vecs
            .iter()
            .map(|v| v.get(idx).cloned().unwrap_or(Value::None))
            .collect();
        results.push(f(elements).await?);
    }

    Ok(Value::VecValue(results))
}

/// Reduces a Value array to a single value using an async accumulator function.
///
/// - `VecValue` -> folds elements sequentially
/// - `AreaValue` -> flattens to 1D (row-major order) then folds
/// - Single value -> applies function with single element
/// - Empty array -> returns initial value
///
/// # Example
/// ```ignore
/// reduce_value(
///     initial,
///     array,
///     |acc, elem| async move {
///         let lambda_param_accumulator = acc;
///         let lambda_param_value = elem;
///         Ok(reduce_lambda_xxx(input, lambda_param_accumulator, lambda_param_value).await?)
///     },
/// ).await?
/// ```
pub async fn reduce_value<F, Fut>(
    initial: Value,
    array: Value,
    f: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(Value, Value) -> Fut,
    Fut: Future<Output = Result<Value, Box<dyn Error + Send + Sync>>>,
{
    // Convert array to Vec<Value> for iteration
    let elements: Vec<Value> = match array {
        Value::VecValue(vec) => vec,
        Value::AreaValue(rows) => rows.into_iter().flatten().collect(),
        Value::None => return Ok(initial), // Empty array returns initial
        single => vec![single], // Single value treated as one-element array
    };

    // Handle empty array case - return initial value
    if elements.is_empty() {
        return Ok(initial);
    }

    // Fold the elements, threading the accumulator through each iteration
    let mut accumulator = initial;
    for elem in elements {
        accumulator = f(accumulator, elem).await?;
    }

    Ok(accumulator)
}

/// Scans a Value array and returns all intermediate accumulator values.
///
/// Unlike `reduce_value` which returns only the final accumulator,
/// `scan_value` returns an array of ALL intermediate accumulator values
/// after applying the lambda to each element.
///
/// Preserves structure (like `map_value`):
/// - `VecValue` -> iterates elements, returns `VecValue` of intermediate results
/// - `AreaValue` -> iterates elements preserving row/col structure, returns `AreaValue`
/// - Single value -> applies function once, returns single result wrapped in `VecValue`
/// - Empty array -> returns empty array (preserving type)
///
/// # Example
/// ```ignore
/// // =SCAN(0, {1,2,3,4}, LAMBDA(a,b, a+b)) returns {1, 3, 6, 10}
/// scan_value(
///     Value::from(0),
///     Value::VecValue(vec![Value::from(1), Value::from(2), Value::from(3), Value::from(4)]),
///     |acc, elem| async move {
///         let lambda_param_a = acc;
///         let lambda_param_b = elem;
///         Ok(scan_lambda_xxx(input, lambda_param_a, lambda_param_b).await?)
///     },
/// ).await?
/// // Result: VecValue([1, 3, 6, 10])
/// ```
pub async fn scan_value<F, Fut>(
    initial: Value,
    array: Value,
    f: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(Value, Value) -> Fut,
    Fut: Future<Output = Result<Value, Box<dyn Error + Send + Sync>>>,
{
    match array {
        Value::VecValue(vec) => {
            if vec.is_empty() {
                return Ok(Value::VecValue(vec![]));
            }
            let mut results = Vec::with_capacity(vec.len());
            let mut accumulator = initial;
            for elem in vec {
                accumulator = f(accumulator, elem).await?;
                results.push(accumulator.clone());
            }
            Ok(Value::VecValue(results))
        }
        Value::AreaValue(rows) => {
            if rows.is_empty() {
                return Ok(Value::AreaValue(vec![]));
            }
            let mut area_results = Vec::with_capacity(rows.len());
            let mut accumulator = initial;
            for row in rows {
                let mut row_results = Vec::with_capacity(row.len());
                for elem in row {
                    accumulator = f(accumulator, elem).await?;
                    row_results.push(accumulator.clone());
                }
                area_results.push(row_results);
            }
            Ok(Value::AreaValue(area_results))
        }
        Value::None => Ok(Value::VecValue(vec![])),
        single => {
            // Single value treated as one-element array
            let accumulator = f(initial, single).await?;
            Ok(Value::VecValue(vec![accumulator]))
        }
    }
}

/// Creates a 2D array by applying an async function at each row/column position.
///
/// MAKEARRAY(rows, cols, LAMBDA(r, c, expr)) creates a `rows x cols` array where
/// each cell value is computed by calling the lambda with 1-based row and column indices.
///
/// # Arguments
/// - `rows` - A Value that evaluates to the number of rows (will be converted to integer)
/// - `cols` - A Value that evaluates to the number of columns (will be converted to integer)
/// - `f` - An async function that takes (row_index, col_index) as 1-based Values and returns the cell value
///
/// # Returns
/// - `Value::AreaValue` containing the computed 2D array
/// - Empty array if rows or cols <= 0
///
/// # Example
/// ```ignore
/// // =MAKEARRAY(3, 2, LAMBDA(r, c, r*c)) returns {{1, 2}, {2, 4}, {3, 6}}
/// makearray_value(
///     Value::from(3),  // 3 rows
///     Value::from(2),  // 2 cols
///     |row, col| async move {
///         let lambda_param_r = row;
///         let lambda_param_c = col;
///         Ok(makearray_lambda_xxx(input, lambda_param_r, lambda_param_c).await?)
///     },
/// ).await?
/// // Result: AreaValue([[1, 2], [2, 4], [3, 6]])
/// ```
pub async fn makearray_value<F, Fut>(
    rows: Value,
    cols: Value,
    f: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(Value, Value) -> Fut,
    Fut: Future<Output = Result<Value, Box<dyn Error + Send + Sync>>>,
{
    // Convert rows and cols to integers
    let num_rows = match &rows {
        Value::F64(v) => *v as usize,
        Value::I32(v) => *v as usize,
        _ => {
            // Try to extract from single-element arrays or other Value types
            match rows.clone() {
                Value::VecValue(vec) if vec.len() == 1 => {
                    match &vec[0] {
                        Value::F64(v) => *v as usize,
                        Value::I32(v) => *v as usize,
                        _ => return Ok(Value::AreaValue(vec![])),
                    }
                }
                Value::AreaValue(area) if area.len() == 1 && area[0].len() == 1 => {
                    match &area[0][0] {
                        Value::F64(v) => *v as usize,
                        Value::I32(v) => *v as usize,
                        _ => return Ok(Value::AreaValue(vec![])),
                    }
                }
                _ => return Ok(Value::AreaValue(vec![])),
            }
        }
    };

    let num_cols = match &cols {
        Value::F64(v) => *v as usize,
        Value::I32(v) => *v as usize,
        _ => {
            // Try to extract from single-element arrays or other Value types
            match cols.clone() {
                Value::VecValue(vec) if vec.len() == 1 => {
                    match &vec[0] {
                        Value::F64(v) => *v as usize,
                        Value::I32(v) => *v as usize,
                        _ => return Ok(Value::AreaValue(vec![])),
                    }
                }
                Value::AreaValue(area) if area.len() == 1 && area[0].len() == 1 => {
                    match &area[0][0] {
                        Value::F64(v) => *v as usize,
                        Value::I32(v) => *v as usize,
                        _ => return Ok(Value::AreaValue(vec![])),
                    }
                }
                _ => return Ok(Value::AreaValue(vec![])),
            }
        }
    };

    // Handle edge cases - zero dimensions result in empty array
    if num_rows == 0 || num_cols == 0 {
        return Ok(Value::AreaValue(vec![]));
    }

    // Build the 2D array with 1-based indices passed to lambda
    let mut area_results = Vec::with_capacity(num_rows);
    for row in 1..=num_rows {
        let mut row_results = Vec::with_capacity(num_cols);
        for col in 1..=num_cols {
            // Convert row and col to Value (1-based indices as f64)
            let row_value = Value::F64(row as f64);
            let col_value = Value::F64(col as f64);
            row_results.push(f(row_value, col_value).await?);
        }
        area_results.push(row_results);
    }

    Ok(Value::AreaValue(area_results))
}

/// Applies an async function to each row of a 2D array and returns a column vector.
///
/// BYROW(array, LAMBDA(row, expr)) applies a lambda to each row of the input array,
/// where each row is passed as a 1D VecValue, and returns a column vector (VecValue)
/// of the results.
///
/// - `AreaValue` -> iterates rows, passes each row as VecValue, returns `VecValue` with n elements
/// - `VecValue` -> treated as single row, returns `VecValue` with 1 element
/// - Single value -> treated as 1x1 array, returns `VecValue` with 1 element
///
/// # Example
/// ```ignore
/// // =BYROW({1,2,3;4,5,6;7,8,9}, LAMBDA(row, SUM(row))) returns {6, 15, 24} (column vector)
/// byrow_value(
///     Value::AreaValue(vec![
///         vec![Value::from(1), Value::from(2), Value::from(3)],
///         vec![Value::from(4), Value::from(5), Value::from(6)],
///         vec![Value::from(7), Value::from(8), Value::from(9)],
///     ]),
///     |row| async move {
///         let lambda_param_row = row;
///         Ok(byrow_lambda_xxx(input, lambda_param_row).await?)
///     },
/// ).await?
/// // Result: VecValue([6, 15, 24])
/// ```
pub async fn byrow_value<F, Fut>(
    array: Value,
    f: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(Value) -> Fut,
    Fut: Future<Output = Result<Value, Box<dyn Error + Send + Sync>>>,
{
    match array {
        Value::AreaValue(rows) => {
            if rows.is_empty() {
                return Ok(Value::VecValue(vec![]));
            }
            let mut results = Vec::with_capacity(rows.len());
            for row in rows {
                // Convert each row (Vec<Value>) to a VecValue for the lambda
                let row_as_vec = Value::VecValue(row);
                results.push(f(row_as_vec).await?);
            }
            // Return as a column vector (VecValue)
            Ok(Value::VecValue(results))
        }
        Value::VecValue(vec) => {
            // Treat a 1D array as a single row
            // The lambda receives the entire VecValue as its row argument
            if vec.is_empty() {
                return Ok(Value::VecValue(vec![]));
            }
            let row_as_vec = Value::VecValue(vec);
            let result = f(row_as_vec).await?;
            // Return as a column vector with 1 element
            Ok(Value::VecValue(vec![result]))
        }
        Value::None => Ok(Value::VecValue(vec![])),
        single => {
            // Single value treated as 1x1 array
            // Create a VecValue containing just this single value
            let row_as_vec = Value::VecValue(vec![single]);
            let result = f(row_as_vec).await?;
            Ok(Value::VecValue(vec![result]))
        }
    }
}

/// Applies an async function to each column of a 2D array and returns a row vector.
///
/// BYCOL(array, LAMBDA(col, expr)) applies a lambda to each column of the input array,
/// where each column is passed as a 1D VecValue, and returns a row vector (VecValue)
/// of the results.
///
/// - `AreaValue` -> iterates columns, passes each column as VecValue, returns `VecValue` with n elements
/// - `VecValue` -> treated as single column, returns `VecValue` with 1 element
/// - Single value -> treated as 1x1 array, returns `VecValue` with 1 element
///
/// # Example
/// ```ignore
/// // =BYCOL({1,2,3;4,5,6;7,8,9}, LAMBDA(col, SUM(col))) returns {12, 15, 18} (row vector)
/// bycol_value(
///     Value::AreaValue(vec![
///         vec![Value::from(1), Value::from(2), Value::from(3)],
///         vec![Value::from(4), Value::from(5), Value::from(6)],
///         vec![Value::from(7), Value::from(8), Value::from(9)],
///     ]),
///     |col| async move {
///         let lambda_param_col = col;
///         Ok(bycol_lambda_xxx(input, lambda_param_col).await?)
///     },
/// ).await?
/// // Result: VecValue([12, 15, 18])
/// ```
pub async fn bycol_value<F, Fut>(
    array: Value,
    f: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(Value) -> Fut,
    Fut: Future<Output = Result<Value, Box<dyn Error + Send + Sync>>>,
{
    match array {
        Value::AreaValue(rows) => {
            if rows.is_empty() {
                return Ok(Value::VecValue(vec![]));
            }
            // Determine the number of columns from the first row
            let num_cols = rows.first().map(|r| r.len()).unwrap_or(0);
            if num_cols == 0 {
                return Ok(Value::VecValue(vec![]));
            }

            let mut results = Vec::with_capacity(num_cols);

            // Iterate over each column index
            for col_idx in 0..num_cols {
                // Extract all values from this column across all rows
                let column_values: Vec<Value> = rows
                    .iter()
                    .filter_map(|row| row.get(col_idx).cloned())
                    .collect();

                // Convert column to VecValue for the lambda
                let col_as_vec = Value::VecValue(column_values);
                results.push(f(col_as_vec).await?);
            }

            // Return as a row vector (VecValue)
            Ok(Value::VecValue(results))
        }
        Value::VecValue(vec) => {
            // Treat a 1D array as a single column
            // The lambda receives the entire VecValue as its column argument
            if vec.is_empty() {
                return Ok(Value::VecValue(vec![]));
            }
            let col_as_vec = Value::VecValue(vec);
            let result = f(col_as_vec).await?;
            // Return as a row vector with 1 element
            Ok(Value::VecValue(vec![result]))
        }
        Value::None => Ok(Value::VecValue(vec![])),
        single => {
            // Single value treated as 1x1 array
            // Create a VecValue containing just this single value
            let col_as_vec = Value::VecValue(vec![single]);
            let result = f(col_as_vec).await?;
            Ok(Value::VecValue(vec![result]))
        }
    }
}

/// Applies GROUPBY: groups rows by key columns and aggregates values using a lambda.
///
/// Groups `row_fields` rows, collects corresponding `values` rows per group,
/// calls `f` on each group's values (as an AreaValue), and assembles the result
/// with optional headers, grand totals, and sorting.
///
/// # Parameters
/// - `row_fields`: The grouping key columns (Value::AreaValue or VecValue)
/// - `values`: The value columns to aggregate (Value::AreaValue or VecValue)
/// - `f`: Async aggregation lambda — receives AreaValue of group values, returns aggregated Value
/// - `field_headers`: 0=no headers, 1=first row is header, 2=generate, 3=header+generate
/// - `total_depth`: 0=none, 1=grand totals, 2=grand+subtotals, -1=only grand, -2=only subtotals
/// - `sort_order`: 0=no sort, 1=ascending, -1=descending
/// - `filter_array`: Boolean array to filter rows before grouping
///
/// # Example
/// ```ignore
/// // =GROUPBY(A1:A5, B1:B5, SUM)
/// groupby_value(
///     row_fields_value,
///     values_value,
///     |vals| async move {
///         let lambda_param_v = vals;
///         Ok(groupby_lambda_xxx(input, lambda_param_v).await?)
///     },
///     Value::None,  // field_headers
///     Value::None,  // total_depth
///     Value::None,  // sort_order
///     Value::None,  // filter_array
/// ).await?
/// ```
pub async fn groupby_value<F, Fut>(
    row_fields: Value,
    values: Value,
    f: F,
    field_headers: Value,
    total_depth: Value,
    sort_order: Value,
    filter_array: Value,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(Value) -> Fut,
    Fut: Future<Output = Result<Value, Box<dyn Error + Send + Sync>>>,
{
    use crate::lookup_and_reference::codcel_groupby::codcel_groupby;

    // Convert inputs to 2D arrays
    let rf_area = value_to_area(row_fields)?;
    let v_area = value_to_area(values)?;

    // Convert optional parameters
    let field_headers_opt = value_to_option_i32(&field_headers);
    let total_depth_opt = value_to_option_i32(&total_depth);
    let sort_order_opt = value_to_option_i32(&sort_order);
    let filter_opt = value_to_bool_vec(&filter_array);

    // Perform core grouping
    let group_data = codcel_groupby(
        rf_area,
        v_area,
        field_headers_opt,
        total_depth_opt,
        sort_order_opt,
        filter_opt,
    )?;

    let total_depth_val = group_data.total_depth.unwrap_or(0);
    let num_key_cols = if group_data.groups.is_empty() {
        0
    } else {
        group_data.groups[0].key.len()
    };

    // Build result rows
    let mut result_rows: Vec<Vec<Value>> = Vec::new();

    // For total_depth -1 or -2, we skip per-group rows and only show totals
    let show_group_rows = total_depth_val >= 0;

    if show_group_rows {
        // Aggregate each group
        for group in &group_data.groups {
            let group_values = Value::AreaValue(group.values.clone());
            let agg_result = f(group_values).await?;

            // Build result row: key columns + aggregation result(s)
            let mut row = group.key.clone();
            append_agg_result(&mut row, &agg_result, group_data.num_value_cols);
            result_rows.push(row);
        }
    }

    // Add grand total row if total_depth requires it (1, 2, -1, -2)
    if (total_depth_val == 1 || total_depth_val == 2 || total_depth_val == -1 || total_depth_val == -2)
        && !group_data.all_values.is_empty() {
            let all_values = Value::AreaValue(group_data.all_values.clone());
            let grand_total = f(all_values).await?;

            let mut total_row: Vec<Value> = Vec::new();
            // Use "Grand Total" for total_depth -2, "Total" for others
            let label = if total_depth_val == -2 { "Grand Total" } else { "Total" };
            total_row.push(Value::String(label.to_string()));
            for _ in 1..num_key_cols {
                total_row.push(Value::None);
            }
            append_agg_result(&mut total_row, &grand_total, group_data.num_value_cols);
            result_rows.push(total_row);
        }

    if result_rows.is_empty() {
        Ok(Value::AreaValue(vec![vec![Value::None]]))
    } else {
        Ok(Value::AreaValue(result_rows))
    }
}

/// Convert a Value to a 2D array (Vec<Vec<Value>>).
fn value_to_area(v: Value) -> Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> {
    match v {
        Value::AreaValue(area) => Ok(area),
        Value::VecValue(vec) => {
            // Treat 1D as a single-column 2D array
            Ok(vec.into_iter().map(|v| vec![v]).collect())
        }
        Value::None => Err("GROUPBY: expected an array, got empty value".into()),
        single => {
            // Single value = 1x1 array
            Ok(vec![vec![single]])
        }
    }
}

/// Extract an optional i32 from a Value.
fn value_to_option_i32(v: &Value) -> Option<i32> {
    match v {
        Value::I32(n) => Some(*n),
        Value::F64(n) => Some(*n as i32),
        Value::None => None,
        _ => None,
    }
}

/// Extract a boolean vector from a Value (for filter_array).
fn value_to_bool_vec(v: &Value) -> Option<Vec<bool>> {
    match v {
        Value::VecValue(vec) => {
            Some(vec.iter().map(|v| match v {
                Value::Bool(b) => *b,
                Value::I32(n) => *n != 0,
                Value::F64(n) => *n != 0.0,
                _ => true,
            }).collect())
        }
        Value::AreaValue(area) => {
            // Flatten 2D to 1D (take first column)
            Some(area.iter().map(|row| {
                if let Some(v) = row.first() {
                    match v {
                        Value::Bool(b) => *b,
                        Value::I32(n) => *n != 0,
                        Value::F64(n) => *n != 0.0,
                        _ => true,
                    }
                } else {
                    true
                }
            }).collect())
        }
        Value::None => None,
        _ => None,
    }
}

/// Append aggregation result(s) to a row.
/// The lambda may return a single value or a VecValue/AreaValue with multiple columns.
fn append_agg_result(row: &mut Vec<Value>, agg_result: &Value, num_value_cols: usize) {
    match agg_result {
        Value::VecValue(vec) => {
            for v in vec {
                row.push(v.clone());
            }
        }
        Value::AreaValue(area) => {
            // If the lambda returns a single-row area, use those values
            if let Some(first_row) = area.first() {
                for v in first_row {
                    row.push(v.clone());
                }
            } else {
                for _ in 0..num_value_cols {
                    row.push(Value::None);
                }
            }
        }
        single => {
            // Single aggregation result
            row.push(single.clone());
        }
    }
}

// Tests are performed in the generated calculation projects
