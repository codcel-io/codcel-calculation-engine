// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

//! The cells of a range inside a filled-down formula, such as `SUM($D$9:D9)` copied down a
//! column, computed incrementally.
//!
//! Row `n` of such a formula reads rows 9 to `n`, so a column of 1,200 rows reads 720,000 cells
//! in all. Each read is a per-request cache hit, but those still cost far more than the
//! arithmetic. Here the cells read from a given first row are kept, in row order, for the rest of
//! the request: the next row's formula reuses them and computes only the cells it adds.
//!
//! The cells are computed in the same order, and the same error is returned, as reading them one
//! by one: a call computes the missing cells from the lowest row up and stops at the first error.
//! No lock is held while a cell is computed, so a cell may itself read the same range.

use crate::input::Input;
use crate::value::Value;
use std::collections::HashMap;
use std::error::Error;
use std::future::Future;
use std::pin::Pin;

/// The value of a cell, as a row dispatcher returns it.
pub type CellFuture<'a> =
    Pin<Box<dyn Future<Output = Result<Value, Box<dyn Error + Send + Sync>>> + Send + 'a>>;

/// A column's row dispatcher: the value of the column's cell on a row.
pub type CellFn = for<'a> fn(&'a Input, Value) -> CellFuture<'a>;

/// Ranges kept per column. A column read from more first rows than this (a sliding window, or a
/// range up to the column's end) reads its further ranges cell by cell, so memory stays bounded.
const MAX_FIRST_ROWS_PER_COLUMN: usize = 64;

/// Per-request cells read so far, by (column, first row).
#[derive(Debug, Default)]
pub struct FillDownCache {
    cells: HashMap<(u64, i32), Vec<Value>>,
    first_rows: HashMap<u64, usize>,
}

/// The values of rows `first` to `last` of the column with dispatcher `cell`, in row order.
/// `column` identifies the column (the generator uses a hash of the dispatcher's name).
pub async fn fill_down_cells(
    input: &Input,
    column: u64,
    first: i32,
    last: i32,
    cell: CellFn,
) -> Result<Vec<Value>, Box<dyn Error + Send + Sync>> {
    if last < first {
        return Ok(Vec::new());
    }
    let needed = (last - first + 1) as usize;
    let kept = {
        let mut cache = input.fill_down_cache();
        let cache = &mut *cache;
        if cache.cells.contains_key(&(column, first)) {
            true
        } else {
            let count = cache.first_rows.entry(column).or_insert(0);
            if *count < MAX_FIRST_ROWS_PER_COLUMN {
                *count += 1;
                cache
                    .cells
                    .insert((column, first), Vec::with_capacity(needed));
                true
            } else {
                false
            }
        }
    };
    if !kept {
        crate::profile::count(&crate::profile::FILL_DOWN_UNCACHED);
        let mut cells = Vec::with_capacity(needed);
        for row in first..=last {
            cells.push(cell(input, Value::I32(row)).await?);
        }
        return Ok(cells);
    }
    loop {
        let have = {
            let cache = input.fill_down_cache();
            let cells = &cache.cells[&(column, first)];
            if cells.len() >= needed {
                crate::profile::count(&crate::profile::FILL_DOWN_HITS);
                return Ok(cells[..needed].to_vec());
            }
            cells.len()
        };
        crate::profile::count(&crate::profile::FILL_DOWN_COMPUTED);
        let value = cell(input, Value::I32(first + have as i32)).await?;
        let mut cache = input.fill_down_cache();
        if let Some(cells) = cache.cells.get_mut(&(column, first)) {
            // A cell computed meanwhile may have read this range further already.
            if cells.len() == have {
                cells.push(value);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value_format::ValueFormat;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static CALLS: AtomicUsize = AtomicUsize::new(0);

    fn counted_square(input: &Input, row: Value) -> CellFuture<'_> {
        CALLS.fetch_add(1, Ordering::SeqCst);
        square(input, row)
    }

    fn square(_input: &Input, row: Value) -> CellFuture<'_> {
        Box::pin(async move {
            let Value::I32(row) = row else {
                return Err("row".into());
            };
            if row == 50 {
                return Err("row 50 fails".into());
            }
            Ok(Value::F64(f64::from(row * row)))
        })
    }

    /// Runs a future that never waits, as these don't.
    fn run<F: Future>(future: F) -> F::Output {
        let mut future = std::pin::pin!(future);
        let mut context = std::task::Context::from_waker(std::task::Waker::noop());
        match future.as_mut().poll(&mut context) {
            std::task::Poll::Ready(output) => output,
            std::task::Poll::Pending => panic!("the future waited"),
        }
    }

    #[test]
    fn growing_ranges_compute_each_cell_once_and_match_reading_cell_by_cell() {
        let input = Input::new("Test", ValueFormat::default());
        CALLS.store(0, Ordering::SeqCst);
        run(async {
            for last in 9..=40 {
                let cells = fill_down_cells(&input, 1, 9, last, counted_square)
                    .await
                    .expect("cells");
                let expected: Vec<Value> =
                    (9..=last).map(|r| Value::F64(f64::from(r * r))).collect();
                assert_eq!(cells, expected);
            }
            // Shorter than what is kept.
            let cells = fill_down_cells(&input, 1, 9, 12, counted_square)
                .await
                .expect("cells");
            assert_eq!(cells.len(), 4);
        });
        assert_eq!(CALLS.load(Ordering::SeqCst), 32);
    }

    #[test]
    fn the_first_error_is_returned_and_not_kept() {
        let input = Input::new("Test", ValueFormat::default());
        run(async {
            assert!(fill_down_cells(&input, 2, 45, 49, square).await.is_ok());
            let error = fill_down_cells(&input, 2, 45, 60, square)
                .await
                .expect_err("row 50");
            assert_eq!(error.to_string(), "row 50 fails");
            assert!(fill_down_cells(&input, 2, 45, 55, square).await.is_err());
            assert_eq!(
                fill_down_cells(&input, 2, 45, 48, square)
                    .await
                    .expect("cells")
                    .len(),
                4
            );
            assert!(fill_down_cells(&input, 2, 7, 3, square)
                .await
                .expect("empty")
                .is_empty());
        });
    }

    #[test]
    fn many_first_rows_fall_back_to_reading_cell_by_cell() {
        let input = Input::new("Test", ValueFormat::default());
        run(async {
            for first in 100..300 {
                let cells = fill_down_cells(&input, 3, first, first + 3, square)
                    .await
                    .expect("cells");
                assert_eq!(cells[3], Value::F64(f64::from((first + 3) * (first + 3))));
            }
        });
        assert_eq!(
            input.fill_down_cache().first_rows[&3],
            MAX_FIRST_ROWS_PER_COLUMN
        );
    }
}
