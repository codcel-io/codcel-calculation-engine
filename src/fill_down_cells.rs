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

use crate::arithmetic_base::{sum, SUM_NOT_NUMBERS};
use crate::compensated_sum::CompensatedSum;
use crate::input::{FastMap, Input};
use crate::value::Value;
use crate::value_format::ValueFormat;
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
    cells: FastMap<(u64, i32), Vec<Value>>,
    first_rows: FastMap<u64, usize>,
    /// Running totals of the kept cells, by (column, first row, strict), for [`fill_down_sum`].
    sums: FastMap<(u64, i32, bool), RunningSums>,
}

/// The state of `sum` after each of a range's first cells: entry `k` after cells `0..=k`.
#[derive(Debug, Default)]
struct RunningSums {
    /// `None` from the first cell `sum` rejects (with `strict`) on.
    states: Vec<Option<CompensatedSum>>,
    /// Set when a cell is a range other than a vector or area of values (one that may fail to
    /// read): `sum` is then run over the cells themselves.
    not_running: bool,
}

/// Whether a range's cells were kept, or computed for this read only.
enum Computed {
    Kept,
    Uncached(Vec<Value>),
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
    match compute(input, column, first, last, cell).await? {
        Computed::Uncached(cells) => Ok(cells),
        Computed::Kept => {
            let needed = (last - first + 1) as usize;
            let cache = input.fill_down_cache();
            Ok(cache.cells[&(column, first)][..needed].to_vec())
        }
    }
}

/// `sum` of rows `first` to `last` of the column with dispatcher `cell`: the same as
/// `sum(fill_down_cells(input, column, first, last, cell).await?, strict_type_conversion,
/// value_format)`, value for value and error for error, without copying the cells.
///
/// The outer `Result` is the cells' (an error computing a cell, as from [`fill_down_cells`]);
/// the inner one is the sum's. A range filled down (`SUM($D$9:D9)`) grows by a cell per row, and
/// `sum` adds its cells in order, so each row's total continues from the previous row's
/// accumulator, compensation included: the totals are those of summing each range afresh.
pub async fn fill_down_sum(
    input: &Input,
    column: u64,
    first: i32,
    last: i32,
    cell: CellFn,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Result<Value, Box<dyn Error + Send + Sync>>, Box<dyn Error + Send + Sync>> {
    if last < first {
        return Ok(sum(Vec::new(), strict_type_conversion, value_format));
    }
    let needed = (last - first + 1) as usize;
    match compute(input, column, first, last, cell).await? {
        Computed::Uncached(cells) => Ok(sum(cells, strict_type_conversion, value_format)),
        Computed::Kept => {
            let mut cache = input.fill_down_cache();
            let FillDownCache { cells, sums, .. } = &mut *cache;
            let cells = &cells[&(column, first)][..needed];
            let running = sums
                .entry((column, first, strict_type_conversion))
                .or_default();
            while !running.not_running && running.states.len() < needed {
                let value = &cells[running.states.len()];
                let state = running
                    .states
                    .last()
                    .copied()
                    .unwrap_or(Some(CompensatedSum::new()));
                let next = match value {
                    // As `sum` reads a range's cells, which cannot fail for these.
                    Value::VecValue(_)
                    | Value::OptionVecValue(Some(_))
                    | Value::AreaValue(_)
                    | Value::OptionAreaValue(Some(_)) => state.map(|mut total| {
                        let _ = value.for_each_cell(|cell| {
                            if let Some(number) = cell.try_f64(value_format) {
                                total.add(number);
                            }
                        });
                        total
                    }),
                    _ if value.is_array() || value.is_area() => {
                        // Other ranges may fail to read: left to `sum` itself.
                        running.not_running = true;
                        break;
                    }
                    _ => state.and_then(|mut total| match value.f64(value_format) {
                        Ok(number) => {
                            total.add(number);
                            Some(total)
                        }
                        Err(_) if strict_type_conversion => None,
                        Err(_) => Some(total),
                    }),
                };
                running.states.push(next);
            }
            if running.not_running {
                return Ok(sum(cells.to_vec(), strict_type_conversion, value_format));
            }
            Ok(match running.states[needed - 1] {
                Some(total) => Ok(Value::F64(total.total())),
                None => Err(SUM_NOT_NUMBERS.into()),
            })
        }
    }
}

/// Computes the cells of rows `first` to `last` (`first <= last`) not computed yet: kept for the
/// rest of the request, or, past [`MAX_FIRST_ROWS_PER_COLUMN`] first rows, returned instead.
async fn compute(
    input: &Input,
    column: u64,
    first: i32,
    last: i32,
    cell: CellFn,
) -> Result<Computed, Box<dyn Error + Send + Sync>> {
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
        return Ok(Computed::Uncached(cells));
    }
    loop {
        let have = {
            let cache = input.fill_down_cache();
            let cells = &cache.cells[&(column, first)];
            if cells.len() >= needed {
                crate::profile::count(&crate::profile::FILL_DOWN_HITS);
                return Ok(Computed::Kept);
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

    /// Cells chosen to stress `sum`: cancellation that compensation recovers, text (rejected
    /// with strict conversion, skipped without), numeric text, a boolean, and an error at 70.
    fn mixed(_input: &Input, row: Value) -> CellFuture<'_> {
        Box::pin(async move {
            let Value::I32(row) = row else {
                return Err("row".into());
            };
            Ok(match row % 9 {
                _ if row == 70 => return Err("row 70 fails".into()),
                _ if row == 40 => Value::String("text".to_string()),
                _ if row == 41 => Value::String("2.5".to_string()),
                _ if row == 42 => Value::Bool(true),
                0 => Value::F64(1e16),
                1 => Value::F64(1.0),
                2 => Value::F64(-1e16),
                3 => Value::F64(0.1),
                4 => Value::I32(row),
                5 => Value::F64(-0.3),
                6 => Value::None,
                7 => Value::F64(1e-9),
                _ => Value::F64(f64::from(row) / 7.0),
            })
        })
    }

    fn with_array(_input: &Input, row: Value) -> CellFuture<'_> {
        Box::pin(async move {
            let Value::I32(row) = row else {
                return Err("row".into());
            };
            Ok(if row == 15 {
                Value::VecValue(vec![Value::F64(2.0), Value::String("x".to_string())])
            } else if row % 4 == 0 {
                Value::AreaValue(vec![
                    vec![Value::F64(1e16)],
                    vec![Value::Bool(true), Value::F64(-1e16)],
                ])
            } else if row % 5 == 0 {
                Value::OptionVecValue(Some(vec![Value::String("1.5".to_string()), Value::None]))
            } else if row == 27 {
                Value::OptionVecValue(None)
            } else {
                Value::F64(f64::from(row) * 0.1)
            })
        })
    }

    /// `fill_down_sum` equals `sum` over `fill_down_cells`, bit for bit and error for error,
    /// whatever order the ranges are read in.
    #[test]
    fn fill_down_sum_matches_sum_of_the_cells() {
        let format = ValueFormat::default();
        let render = |r: Result<Value, Box<dyn Error + Send + Sync>>| match r {
            Ok(Value::F64(v)) => format!("{:?}", v.to_bits()),
            Ok(other) => format!("{other:?}"),
            Err(e) => format!("error {e}"),
        };
        for (dispatcher, first, rows) in [
            (mixed as CellFn, 9, 9..=80),
            (mixed as CellFn, 43, 43..=69),
            (with_array as CellFn, 10, 10..=30),
        ] {
            for strict in [true, false] {
                // Rows read in a scrambled order, as warming and dependencies may read them.
                let mut lasts: Vec<i32> = rows.clone().collect();
                lasts.sort_by_key(|r| (r * 7919) % 97);
                lasts.push(first - 1);
                let fast_input = Input::new("Test", ValueFormat::default());
                let plain_input = Input::new("Test", ValueFormat::default());
                run(async {
                    for last in lasts {
                        let fast =
                            fill_down_sum(&fast_input, 7, first, last, dispatcher, strict, &format)
                                .await;
                        let plain = fill_down_cells(&plain_input, 7, first, last, dispatcher).await;
                        match (fast, plain) {
                            (Ok(fast), Ok(cells)) => assert_eq!(
                                render(fast),
                                render(sum(cells, strict, &format)),
                                "rows {first}..={last} strict {strict}"
                            ),
                            (Err(fast), Err(plain)) => {
                                assert_eq!(fast.to_string(), plain.to_string())
                            }
                            (fast, plain) => panic!(
                                "{first}..={last}: {:?} vs {:?}",
                                fast.map(render),
                                plain.map(|c| c.len())
                            ),
                        }
                    }
                });
            }
        }
    }

    /// Past the first rows kept per column, the sum is of cells read for that call only.
    #[test]
    fn fill_down_sum_of_uncached_ranges() {
        let input = Input::new("Test", ValueFormat::default());
        let format = ValueFormat::default();
        run(async {
            for first in 100..200 {
                let total = fill_down_sum(&input, 4, first, first + 2, square, true, &format)
                    .await
                    .expect("cells")
                    .expect("sum");
                let expected: f64 = (first..=first + 2).map(|r| f64::from(r * r)).sum();
                assert_eq!(total, Value::F64(expected));
            }
        });
    }
}
