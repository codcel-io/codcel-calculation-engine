//! Timing of common functions over large ranges, to measure performance changes.
//!
//! Ignored by default. Run with:
//! `cargo test --release --test perf_functions -- --ignored --nocapture`

use std::hint::black_box;
use std::time::Instant;

use codcel_calculation_engine::arithmetic_base::{
    count, counta, countblank, max, min, sum, sum_product,
};
use codcel_calculation_engine::compatibility_base::rank;
use codcel_calculation_engine::condition_base::{average_if, count_if, count_ifs, sum_if, sum_ifs};
use codcel_calculation_engine::lookup_reference_base::{
    h_lookup_array, index_array, match_array, v_lookup_array, x_lookup_array,
};
use codcel_calculation_engine::statistical_base::{average, large, median, rank_eq, small};
use codcel_calculation_engine::text_base::{concatenate, mid, search};
use codcel_calculation_engine::value::Value;
use codcel_calculation_engine::value_format::ValueFormat;

/// A column of `n` cells: mostly numbers, with some text and blanks.
fn mixed_column(n: usize) -> Vec<Value> {
    (0..n)
        .map(|i| match i % 10 {
            7 => Value::String(format!("item {i}")),
            9 => Value::None,
            _ => Value::F64(i as f64 * 1.5),
        })
        .collect()
}

fn numbers(n: usize) -> Vec<Value> {
    (0..n).map(|i| Value::F64(i as f64)).collect()
}

/// The numbers 0..n in a scrambled order, so sorting them is real work.
fn shuffled(n: usize) -> Vec<Value> {
    (0..n)
        .map(|i| Value::F64(((i * 7919) % n) as f64))
        .collect()
}

fn texts(n: usize) -> Vec<Value> {
    (0..n)
        .map(|i| Value::String(format!("Name {}", i % 50)))
        .collect()
}

fn column_area(cells: Vec<Value>) -> Value {
    Value::AreaValue(cells.into_iter().map(|c| vec![c]).collect())
}

/// Runs `f` until about 0.3 s have passed and prints the time per call.
fn time(name: &str, n: usize, mut f: impl FnMut()) {
    f();
    let start = Instant::now();
    let mut calls = 0u32;
    while start.elapsed().as_secs_f64() < 0.3 {
        f();
        calls += 1;
    }
    let per_call = start.elapsed().as_secs_f64() / f64::from(calls) * 1e6;
    println!("PERF {name:<28} n={n:<6} {per_call:>12.2} us/call");
}

#[test]
#[ignore]
fn perf_functions() {
    let vf = ValueFormat::default();
    for n in [1_000, 10_000] {
        let mixed = mixed_column(n);
        let nums = numbers(n);
        let names = texts(n);
        let mixed_area = column_area(mixed.clone());
        let nums_area = column_area(nums.clone());
        let names_area = column_area(names.clone());
        let shuffled_area = column_area(shuffled(n));
        let table = Value::AreaValue(
            (0..n)
                .map(|i| {
                    vec![
                        Value::F64(i as f64),
                        Value::String(format!("v{i}")),
                        Value::F64(i as f64 * 2.0),
                    ]
                })
                .collect(),
        );
        let wide = Value::AreaValue(vec![
            (0..n).map(|i| Value::F64(i as f64)).collect(),
            (0..n).map(|i| Value::F64(i as f64 * 2.0)).collect(),
        ]);

        // What every timing below also pays: copying the input range.
        time("(clone of input only)", n, || {
            drop(black_box(mixed_area.clone()))
        });

        // Aggregates over a range argument (one Vec element holding the range).
        time("SUM", n, || {
            drop(black_box(sum(vec![mixed_area.clone()], false, &vf)))
        });
        time("AVERAGE", n, || {
            drop(black_box(average(vec![mixed_area.clone()], false, &vf)))
        });
        time("MAX", n, || {
            drop(black_box(max(vec![nums_area.clone()], false, &vf)))
        });
        time("MIN", n, || {
            drop(black_box(min(vec![nums_area.clone()], false, &vf)))
        });
        time("COUNT", n, || {
            drop(black_box(count(vec![mixed_area.clone()], &vf)))
        });
        time("COUNTA", n, || {
            drop(black_box(counta(vec![mixed_area.clone()], &vf)))
        });
        time("COUNTBLANK", n, || {
            drop(black_box(countblank(vec![mixed_area.clone()], &vf)))
        });
        time("MEDIAN", n, || {
            drop(black_box(median(vec![Value::VecValue(nums.clone())], &vf)))
        });
        time("SUMPRODUCT", n, || {
            drop(black_box(sum_product(
                Value::VecValue(vec![nums_area.clone(), nums_area.clone()]),
                &vf,
            )))
        });

        // Conditional functions.
        let crit = || Value::String(">100".to_string());
        let crit_text = || Value::String("name 7".to_string());
        time("COUNTIF number", n, || {
            drop(black_box(count_if(mixed_area.clone(), crit(), &vf)))
        });
        time("COUNTIF text", n, || {
            drop(black_box(count_if(names_area.clone(), crit_text(), &vf)))
        });
        time("SUMIF text", n, || {
            drop(black_box(sum_if(
                names_area.clone(),
                crit_text(),
                nums_area.clone(),
                &vf,
            )))
        });
        time("AVERAGEIF", n, || {
            drop(black_box(average_if(
                mixed_area.clone(),
                crit(),
                Value::None,
                &vf,
            )))
        });
        time("SUMIFS 2 criteria", n, || {
            drop(black_box(sum_ifs(
                nums_area.clone(),
                Value::VecValue(vec![
                    names_area.clone(),
                    crit_text(),
                    nums_area.clone(),
                    crit(),
                ]),
                &vf,
            )))
        });
        time("COUNTIFS 2 criteria", n, || {
            drop(black_box(count_ifs(
                Value::VecValue(vec![
                    names_area.clone(),
                    crit_text(),
                    nums_area.clone(),
                    crit(),
                ]),
                &vf,
            )))
        });

        // Lookups, looking for a value near the end.
        let target = || Value::F64((n - 3) as f64);
        time("VLOOKUP exact", n, || {
            drop(black_box(v_lookup_array(
                target(),
                table.clone(),
                Value::I32(2),
                Value::Bool(false),
                &vf,
            )))
        });
        time("HLOOKUP exact", n, || {
            drop(black_box(h_lookup_array(
                target(),
                wide.clone(),
                Value::I32(2),
                Value::Bool(false),
                &vf,
            )))
        });
        time("INDEX", n, || {
            drop(black_box(index_array(
                table.clone(),
                Value::I32((n - 3) as i32),
                Value::I32(2),
                &vf,
            )))
        });
        time("MATCH exact", n, || {
            drop(black_box(match_array(
                target(),
                nums_area.clone(),
                Value::I32(0),
                &vf,
            )))
        });
        time("MATCH approx", n, || {
            drop(black_box(match_array(
                target(),
                nums_area.clone(),
                Value::I32(1),
                &vf,
            )))
        });
        time("XLOOKUP", n, || {
            drop(black_box(x_lookup_array(
                target(),
                nums_area.clone(),
                nums_area.clone(),
                Value::None,
                Value::None,
                Value::None,
                &vf,
            )))
        });

        // Order statistics.
        time("RANK.EQ", n, || {
            drop(black_box(rank_eq(
                Value::F64(500.0),
                shuffled_area.clone(),
                Value::None,
                &vf,
            )))
        });
        time("RANK", n, || {
            drop(black_box(rank(
                Value::F64(500.0),
                shuffled_area.clone(),
                Value::None,
                &vf,
            )))
        });
        time("LARGE", n, || {
            drop(black_box(large(shuffled_area.clone(), Value::I32(3), &vf)))
        });
        time("SMALL", n, || {
            drop(black_box(small(shuffled_area.clone(), Value::I32(3), &vf)))
        });

        // Text, over a range and as a scalar.
        time("CONCATENATE", n, || {
            drop(black_box(concatenate(vec![names_area.clone()], &vf)))
        });
        time("SEARCH over range", n, || {
            drop(black_box(search(
                Value::String("e 4".into()),
                names_area.clone(),
                Value::None,
                &vf,
            )))
        });
        time("MID over range", n, || {
            drop(black_box(mid(
                names_area.clone(),
                Value::F64(2.0),
                Value::F64(3.0),
                false,
                &vf,
            )))
        });
    }

    let vf = ValueFormat::default();
    let scalar = Value::String("The quick brown fox".to_string());
    time("MID scalar", 1, || {
        drop(black_box(mid(
            scalar.clone(),
            Value::F64(5.0),
            Value::F64(5.0),
            false,
            &vf,
        )))
    });
}
