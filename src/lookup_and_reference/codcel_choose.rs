// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

// Returns a value or a list of values from a given list based on 1-based indices.

use std::error::Error;
use std::fmt::Debug;

/// Selects items from `values` using 1-based positions from `indices`, mirroring Excel's `CHOOSE`.
///
/// Each entry in `indices` is evaluated independently, so duplicates or out-of-order indices are
/// allowed. The requested elements are cloned from `values` and returned in the same order the
/// indices appear.
///
/// # Errors
/// Returns an error when an index is less than 1 or greater than the length of `values`.
pub fn codcel_choose<T: Clone + Debug>(
    indices: Vec<i32>,
    values: &[T],
) -> Result<Vec<T>, Box<dyn Error + Send + Sync>> {
    println!("indices {:?}", &indices);
    println!("values {:?}", &values);
    indices
        .iter()
        .map(|&index| choose_result(index, values))
        .collect()
}

fn choose_result<T: Clone>(index: i32, values: &[T]) -> Result<T, Box<dyn Error + Send + Sync>> {
    if index <= 0 {
        Err("CHOOSE: Index cannot be less or equal to 0.".into())
    } else if index > values.len() as i32 {
        Err("CHOOSE: Index cannot be greater than the values length.".into())
    } else {
        Ok(values[index as usize - 1].clone())
    }
}
