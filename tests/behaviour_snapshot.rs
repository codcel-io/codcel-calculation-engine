//! Pins the exact results of the functions touched by performance work.
//!
//! Each function is called over the same set of argument shapes (numbers, text, blanks,
//! errors, 1-D and 2-D ranges and their `Option` forms) and every result, value or error, is
//! written out with `{:?}`. The test fails if any line differs from
//! `tests/snapshots/behaviour.txt`. Run with `CODCEL_UPDATE_SNAPSHOT=1` to rewrite the file;
//! only do that when a change in results is intended.

use std::error::Error;
use std::fmt::Write as _;

use codcel_calculation_engine::arithmetic_base as ab;
use codcel_calculation_engine::array_base as arr;
use codcel_calculation_engine::codcel_information as info;
use codcel_calculation_engine::compatibility_base as compat;
use codcel_calculation_engine::condition_base as cb;
use codcel_calculation_engine::database_base as db;
use codcel_calculation_engine::date_time_base as dt;
use codcel_calculation_engine::excel_error::ExcelError;
use codcel_calculation_engine::financial_base as fin;
use codcel_calculation_engine::lookup_reference_base as lr;
use codcel_calculation_engine::statistical_base as st;
use codcel_calculation_engine::text_base as tb;
use codcel_calculation_engine::value::Value;
use codcel_calculation_engine::value_format::ValueFormat;

type R = Result<Value, Box<dyn Error + Send + Sync>>;

fn f(v: f64) -> Value {
    Value::F64(v)
}
fn s(v: &str) -> Value {
    Value::String(v.to_string())
}

fn mixed_cells() -> Vec<Value> {
    vec![
        f(3.0),
        s("abc"),
        Value::None,
        Value::I32(2),
        Value::Bool(true),
        s("4"),
        f(-1.5),
        Value::OptionF64(None),
        Value::OptionF64(Some(7.0)),
        s("Ünïcödé café"),
        f(2.0),
        s(""),
    ]
}

fn number_cells() -> Vec<Value> {
    [5.0, 1.0, 3.0, 3.0, 9.0, -2.0, 3.0, 7.5, 1.0, 0.0]
        .into_iter()
        .map(f)
        .collect()
}

fn column(cells: Vec<Value>) -> Value {
    Value::AreaValue(cells.into_iter().map(|c| vec![c]).collect())
}

fn grid(cells: Vec<Value>, width: usize) -> Value {
    Value::AreaValue(cells.chunks(width).map(<[Value]>::to_vec).collect())
}

/// The range-like shapes each range argument is tried with.
fn ranges() -> Vec<(&'static str, Value)> {
    vec![
        ("vec_mixed", Value::VecValue(mixed_cells())),
        ("col_mixed", column(mixed_cells())),
        ("grid_mixed", grid(mixed_cells(), 3)),
        ("vec_nums", Value::VecValue(number_cells())),
        ("col_nums", column(number_cells())),
        ("grid_nums", grid(number_cells(), 2)),
        ("optvec_nums", Value::OptionVecValue(Some(number_cells()))),
        (
            "optarea_nums",
            Value::OptionAreaValue(Some(vec![number_cells()])),
        ),
        ("optvec_none", Value::OptionVecValue(None)),
        ("empty_vec", Value::VecValue(vec![])),
        ("scalar_num", f(4.0)),
        ("scalar_text", s("abc")),
        ("scalar_none", Value::None),
        ("scalar_err", Value::Error(ExcelError::Div0)),
        (
            "vec_with_err",
            Value::VecValue(vec![f(1.0), Value::Error(ExcelError::Na), f(2.0)]),
        ),
    ]
}

fn criteria() -> Vec<Value> {
    [
        ">2",
        "abc",
        "a*",
        "<>abc",
        "4",
        "=",
        "",
        "TRUE",
        ">=1.5",
        "?b?",
        "<3",
        "ünïcödé café",
    ]
    .into_iter()
    .map(s)
    .collect()
}

struct Out(String);

impl Out {
    fn line(&mut self, name: &str, result: R) {
        match result {
            Ok(v) => writeln!(self.0, "{name} => {v:?}").unwrap(),
            Err(e) => writeln!(self.0, "{name} => Err({e})").unwrap(),
        }
    }
    /// As `line`, also recording a panic rather than failing the whole test.
    fn guarded(&mut self, name: &str, call: impl FnOnce() -> R + std::panic::UnwindSafe) {
        match std::panic::catch_unwind(call) {
            Ok(result) => self.line(name, result),
            Err(_) => writeln!(self.0, "{name} => PANIC").unwrap(),
        }
    }
}

#[test]
fn behaviour_matches_snapshot() {
    std::panic::set_hook(Box::new(|_| {}));
    let vf = ValueFormat::default();
    let mut o = Out(String::new());

    for (rn, r) in ranges() {
        let r2 = || r.clone();
        // Aggregates taking a list of values.
        o.guarded(&format!("SUM({rn})"), || {
            ab::sum(vec![r2(), f(1.0)], false, &ValueFormat::default())
        });
        o.guarded(&format!("COUNT({rn})"), || {
            ab::count(vec![r2(), s("x")], &ValueFormat::default())
        });
        o.guarded(&format!("COUNTA({rn})"), || {
            ab::counta(vec![r2(), Value::None], &ValueFormat::default())
        });
        o.guarded(&format!("COUNTBLANK({rn})"), || {
            ab::countblank(vec![r2()], &ValueFormat::default())
        });
        o.guarded(&format!("MAX({rn})"), || {
            ab::max(vec![r2()], false, &ValueFormat::default())
        });
        o.guarded(&format!("MIN({rn})"), || {
            ab::min(vec![r2()], false, &ValueFormat::default())
        });
        o.guarded(&format!("AVERAGE({rn})"), || {
            st::average(vec![r2()], false, &ValueFormat::default())
        });
        o.guarded(&format!("AVERAGEA({rn})"), || {
            st::average_a(vec![r2()], false, &ValueFormat::default())
        });
        o.guarded(&format!("AVEDEV({rn})"), || {
            st::ave_dev(vec![r2()], false, &ValueFormat::default())
        });
        o.guarded(&format!("MEDIAN({rn})"), || {
            st::median(vec![r2()], &ValueFormat::default())
        });
        o.guarded(&format!("MODE.SNGL({rn})"), || {
            st::mode_sngl(vec![r2()], &ValueFormat::default())
        });
        o.guarded(&format!("MODE.MULT({rn})"), || {
            st::mode_mult(vec![r2()], &ValueFormat::default())
        });
        o.guarded(&format!("STDEV.S({rn})"), || {
            st::st_dev_s(vec![r2()], &ValueFormat::default())
        });
        o.guarded(&format!("VAR.S({rn})"), || {
            st::var_s(vec![r2()], &ValueFormat::default())
        });
        o.guarded(&format!("SUMSQ({rn})"), || {
            ab::sum_sq(vec![r2()], false, &ValueFormat::default())
        });
        o.guarded(&format!("SUMPRODUCT({rn})"), || {
            ab::sum_product(Value::VecValue(vec![r2(), r2()]), &ValueFormat::default())
        });
        o.guarded(&format!("AND({rn})"), || {
            cb::and(vec![r2()], false, &ValueFormat::default())
        });
        o.guarded(&format!("OR({rn})"), || {
            cb::or(vec![r2()], false, &ValueFormat::default())
        });

        // Order statistics.
        for k in [f(1.0), f(2.0), f(3.5), s("2")] {
            o.guarded(&format!("LARGE({rn},{k:?})"), || {
                st::large(r2(), k.clone(), &ValueFormat::default())
            });
            o.guarded(&format!("SMALL({rn},{k:?})"), || {
                st::small(r2(), k.clone(), &ValueFormat::default())
            });
        }
        for x in [f(3.0), f(1.0), f(4.0), f(9.0)] {
            for order in [Value::None, f(1.0)] {
                o.guarded(&format!("RANK.EQ({x:?},{rn},{order:?})"), || {
                    st::rank_eq(x.clone(), r2(), order.clone(), &ValueFormat::default())
                });
                o.guarded(&format!("RANK.AVG({x:?},{rn},{order:?})"), || {
                    st::rank_avg(x.clone(), r2(), order.clone(), &ValueFormat::default())
                });
                o.guarded(&format!("RANK({x:?},{rn},{order:?})"), || {
                    compat::rank(x.clone(), r2(), order.clone(), &ValueFormat::default())
                });
            }
            o.guarded(&format!("PERCENTRANK.INC({rn},{x:?})"), || {
                st::percent_rank_inc(r2(), x.clone(), Value::None, &ValueFormat::default())
            });
        }
        for k in [f(0.0), f(0.25), f(0.5), f(1.0)] {
            o.guarded(&format!("PERCENTILE.INC({rn},{k:?})"), || {
                st::percentile_inc(r2(), k.clone(), &ValueFormat::default())
            });
            o.guarded(&format!("PERCENTILE.EXC({rn},{k:?})"), || {
                st::percentile_exc(r2(), k.clone(), &ValueFormat::default())
            });
            o.guarded(&format!("TRIMMEAN({rn},{k:?})"), || {
                st::trim_mean(r2(), k.clone(), &ValueFormat::default())
            });
        }
        o.guarded(&format!("QUARTILE.INC({rn},1)"), || {
            st::quartile_inc(r2(), f(1.0), &ValueFormat::default())
        });

        // Conditional functions.
        for c in criteria() {
            o.guarded(&format!("COUNTIF({rn},{c:?})"), || {
                cb::count_if(r2(), c.clone(), &ValueFormat::default())
            });
            o.guarded(&format!("SUMIF({rn},{c:?})"), || {
                cb::sum_if(r2(), c.clone(), Value::None, &ValueFormat::default())
            });
            o.guarded(&format!("SUMIF({rn},{c:?},self)"), || {
                cb::sum_if(r2(), c.clone(), r2(), &ValueFormat::default())
            });
            o.guarded(&format!("AVERAGEIF({rn},{c:?})"), || {
                cb::average_if(r2(), c.clone(), Value::None, &ValueFormat::default())
            });
            o.guarded(&format!("AVERAGEIF({rn},{c:?},self)"), || {
                cb::average_if(r2(), c.clone(), r2(), &ValueFormat::default())
            });
            let pairs = || Value::VecValue(vec![r2(), c.clone(), r2(), s("<>")]);
            o.guarded(&format!("SUMIFS({rn},{c:?})"), || {
                cb::sum_ifs(r2(), pairs(), &ValueFormat::default())
            });
            o.guarded(&format!("COUNTIFS({rn},{c:?})"), || {
                cb::count_ifs(pairs(), &ValueFormat::default())
            });
            o.guarded(&format!("AVERAGEIFS({rn},{c:?})"), || {
                cb::average_ifs(r2(), pairs(), &ValueFormat::default())
            });
            o.guarded(&format!("MAXIFS({rn},{c:?})"), || {
                cb::max_ifs(r2(), pairs(), &ValueFormat::default())
            });
            o.guarded(&format!("MINIFS({rn},{c:?})"), || {
                cb::min_ifs(r2(), pairs(), &ValueFormat::default())
            });
        }

        // Lookups.
        for target in [
            f(3.0),
            f(4.0),
            s("abc"),
            s("ABC"),
            s("a*"),
            f(100.0),
            f(-5.0),
            Value::I32(2),
        ] {
            for mt in [Value::None, f(0.0), f(1.0), f(-1.0)] {
                o.guarded(&format!("MATCH({target:?},{rn},{mt:?})"), || {
                    lr::match_array(target.clone(), r2(), mt.clone(), &ValueFormat::default())
                });
            }
            for mode in [Value::None, f(-1.0), f(1.0), f(2.0)] {
                o.guarded(&format!("XMATCH({target:?},{rn},{mode:?})"), || {
                    lr::x_match_array(
                        target.clone(),
                        r2(),
                        mode.clone(),
                        Value::None,
                        &ValueFormat::default(),
                    )
                });
                o.guarded(&format!("XLOOKUP({target:?},{rn},{mode:?})"), || {
                    lr::x_lookup_array(
                        target.clone(),
                        r2(),
                        r2(),
                        s("nf"),
                        mode.clone(),
                        Value::None,
                        &ValueFormat::default(),
                    )
                });
            }
            o.guarded(&format!("LOOKUP({target:?},{rn})"), || {
                lr::lookup_array(target.clone(), r2(), Value::None, &ValueFormat::default())
            });
            for exact in [Value::Bool(false), Value::Bool(true)] {
                o.guarded(&format!("VLOOKUP({target:?},{rn},1,{exact:?})"), || {
                    lr::v_lookup_array(
                        target.clone(),
                        r2(),
                        f(1.0),
                        exact.clone(),
                        &ValueFormat::default(),
                    )
                });
                o.guarded(&format!("HLOOKUP({target:?},{rn},1,{exact:?})"), || {
                    lr::h_lookup_array(
                        target.clone(),
                        r2(),
                        f(1.0),
                        exact.clone(),
                        &ValueFormat::default(),
                    )
                });
            }
        }
        for (row, col) in [
            (1.0, 1.0),
            (2.0, 1.0),
            (3.0, 2.0),
            (0.0, 1.0),
            (2.0, 0.0),
            (20.0, 1.0),
        ] {
            o.guarded(&format!("INDEX({rn},{row},{col})"), || {
                lr::index_array(r2(), f(row), f(col), &ValueFormat::default())
            });
        }
        o.guarded(&format!("INDEX({rn},2,None)"), || {
            lr::index_array(r2(), f(2.0), Value::None, &ValueFormat::default())
        });
        o.guarded(&format!("CHOOSE(2,{rn})"), || {
            lr::choose(f(2.0), vec![f(1.0), r2(), s("z")], &ValueFormat::default())
        });
        o.guarded(&format!("FILTER({rn})"), || {
            lr::filter_array(r2(), r2(), s("none"), &ValueFormat::default())
        });

        // Array functions.
        o.guarded(&format!("ROWS({rn})"), || {
            arr::rows(r2(), &ValueFormat::default())
        });
        o.guarded(&format!("COLUMNS({rn})"), || {
            arr::columns(r2(), &ValueFormat::default())
        });
        o.guarded(&format!("ROW({rn})"), || {
            arr::row(r2(), &ValueFormat::default())
        });
        o.guarded(&format!("COLUMN({rn})"), || {
            arr::column(r2(), &ValueFormat::default())
        });
        o.guarded(&format!("TRANSPOSE({rn})"), || {
            arr::transpose(r2(), &ValueFormat::default())
        });
        o.guarded(&format!("OFFSET({rn},1,0,2,1)"), || {
            arr::offset(
                r2(),
                f(1.0),
                f(0.0),
                f(2.0),
                f(1.0),
                Value::None,
                Value::None,
                &ValueFormat::default(),
            )
        });
        o.guarded(&format!("TAKE({rn},2)"), || {
            arr::take(r2(), f(2.0), Value::None, &ValueFormat::default())
        });
        o.guarded(&format!("DROP({rn},-2)"), || {
            arr::drop(r2(), f(-2.0), Value::None, &ValueFormat::default())
        });
        o.guarded(&format!("TOCOL({rn},1)"), || {
            arr::tocol(r2(), f(1.0), Value::None, &ValueFormat::default())
        });
        o.guarded(&format!("CHOOSEROWS({rn},1,-1)"), || {
            arr::chooserows(r2(), vec![f(1.0), f(-1.0)], &ValueFormat::default())
        });
        o.guarded(&format!("EXPAND({rn},14,4)"), || {
            arr::expand(r2(), f(14.0), f(4.0), s("-"), &ValueFormat::default())
        });
        o.guarded(&format!("TRIMRANGE({rn})"), || {
            arr::trimrange(r2(), Value::None, Value::None, &ValueFormat::default())
        });
        o.guarded(&format!("HSTACK({rn})"), || {
            arr::h_stack(Value::VecValue(vec![r2(), r2()]), &ValueFormat::default())
        });
        o.guarded(&format!("VSTACK({rn})"), || {
            arr::v_stack(Value::VecValue(vec![r2(), r2()]), &ValueFormat::default())
        });
        o.guarded(&format!("UNIQUE({rn})"), || {
            arr::unique(r2(), Value::None, Value::None, &ValueFormat::default())
        });
        o.guarded(&format!("UNIQUE({rn},once)"), || {
            arr::unique(
                r2(),
                Value::None,
                Value::Bool(true),
                &ValueFormat::default(),
            )
        });
        o.guarded(&format!("SORT({rn})"), || {
            arr::sort(
                r2(),
                Value::None,
                Value::None,
                Value::None,
                &ValueFormat::default(),
            )
        });
        o.guarded(&format!("SORT({rn},1,-1)"), || {
            arr::sort(r2(), f(1.0), f(-1.0), Value::None, &ValueFormat::default())
        });
        o.guarded(&format!("SORTBY({rn})"), || {
            arr::sort_by(r2(), vec![(r2(), f(-1.0))], &ValueFormat::default())
        });
        o.guarded(&format!("LEN({rn})"), || {
            arr::len(r2(), &ValueFormat::default())
        });
        o.guarded(&format!("MOD({rn},3)"), || {
            arr::excel_mod(r2(), f(3.0), &ValueFormat::default())
        });

        // Information.
        o.guarded(&format!("ISNUMBER({rn})"), || {
            info::is_number(r2(), false, &ValueFormat::default())
        });
        o.guarded(&format!("N({rn})"), || {
            info::n(r2(), false, &ValueFormat::default())
        });
        o.guarded(&format!("VALUE({rn})"), || {
            tb::value(r2(), false, &ValueFormat::default())
        });

        // Text over ranges.
        o.guarded(&format!("CONCATENATE({rn})"), || {
            tb::concatenate(vec![s("<"), r2(), s(">")], &ValueFormat::default())
        });
        o.guarded(&format!("TEXTJOIN({rn})"), || {
            tb::text_join(
                s(","),
                Value::Bool(true),
                vec![r2(), s("end")],
                &ValueFormat::default(),
            )
        });
        o.guarded(&format!("TEXTJOIN({rn},keep)"), || {
            tb::text_join(
                s("|"),
                Value::Bool(false),
                vec![r2()],
                &ValueFormat::default(),
            )
        });
        o.guarded(&format!("SEARCH(\"b\",{rn})"), || {
            tb::search(s("b"), r2(), Value::None, &ValueFormat::default())
        });
        o.guarded(&format!("SEARCH(\"c?f\",{rn})"), || {
            tb::search(s("c?f"), r2(), Value::None, &ValueFormat::default())
        });
        o.guarded(&format!("MID({rn},2,3)"), || {
            tb::mid(r2(), f(2.0), f(3.0), false, &ValueFormat::default())
        });
        o.guarded(&format!("RIGHT({rn},2)"), || {
            tb::right(r2(), f(2.0), false, &ValueFormat::default())
        });
        o.guarded(&format!("LEFT({rn},2)"), || {
            tb::left(r2(), f(2.0), false, &ValueFormat::default())
        });

        // Financial and dates.
        o.guarded(&format!("NPV(0.1,{rn})"), || {
            fin::npv(f(0.1), r2(), &ValueFormat::default())
        });
        o.guarded(&format!("IRR({rn})"), || {
            fin::irr(r2(), Value::None, &ValueFormat::default())
        });
        o.guarded(&format!("NETWORKDAYS({rn})"), || {
            dt::networkdays(f(45000.0), f(45060.0), r2(), false, &ValueFormat::default())
        });
        o.guarded(&format!("WORKDAY({rn})"), || {
            dt::workday(f(45000.0), f(30.0), r2(), false, &ValueFormat::default())
        });
    }

    // Scalar text functions, including non-ASCII text and edge positions.
    for text in [
        s("The quick brown fox"),
        s("Ünïcödé café"),
        s(""),
        f(12345.678),
        Value::None,
        Value::Bool(true),
    ] {
        for (start, len) in [
            (1.0, 3.0),
            (5.0, 5.0),
            (0.0, 2.0),
            (12.0, 4.0),
            (30.0, 1.0),
            (2.0, -1.0),
            (2.5, 2.9),
        ] {
            o.guarded(&format!("MID({text:?},{start},{len})"), || {
                tb::mid(text.clone(), f(start), f(len), false, &vf.clone())
            });
            o.guarded(&format!("REPLACE({text:?},{start},{len})"), || {
                tb::replace(text.clone(), f(start), f(len), s("XY"), false, &vf.clone())
            });
        }
        for len in [0.0, 1.0, 3.0, 50.0, -1.0, 2.7] {
            o.guarded(&format!("RIGHT({text:?},{len})"), || {
                tb::right(text.clone(), f(len), false, &vf.clone())
            });
            o.guarded(&format!("LEFT({text:?},{len})"), || {
                tb::left(text.clone(), f(len), false, &vf.clone())
            });
        }
        for pat in ["qu", "Q*K", "c?f", "~*", "é", "zz", ""] {
            o.guarded(&format!("SEARCH({pat:?},{text:?})"), || {
                tb::search(s(pat), text.clone(), Value::None, &vf.clone())
            });
            o.guarded(&format!("SEARCH({pat:?},{text:?},3)"), || {
                tb::search(s(pat), text.clone(), f(3.0), &vf.clone())
            });
        }
        for fmt in ["0.00", "#,##0", "yyyy-mm-dd", "@", "0%"] {
            o.guarded(&format!("TEXT({text:?},{fmt:?})"), || {
                tb::text(text.clone(), s(fmt), false, &vf.clone())
            });
        }
    }

    // Conversions used by every function.
    for v in [
        f(3.7),
        f(-3.7),
        s("12"),
        s("1.5"),
        s("abc"),
        Value::Bool(true),
        Value::None,
        s("TRUE"),
        Value::I32(4),
        f(1e12),
    ] {
        writeln!(
            o.0,
            "i32({v:?}) => {:?}",
            v.i32(&vf).map_err(|e| e.to_string())
        )
        .unwrap();
        writeln!(
            o.0,
            "option_i32({v:?}) => {:?}",
            v.option_i32(&vf).map_err(|e| e.to_string())
        )
        .unwrap();
        writeln!(
            o.0,
            "bool({v:?}) => {:?}",
            v.bool(&vf).map_err(|e| e.to_string())
        )
        .unwrap();
        writeln!(
            o.0,
            "option_bool({v:?}) => {:?}",
            v.option_bool(&vf).map_err(|e| e.to_string())
        )
        .unwrap();
        writeln!(
            o.0,
            "f64({v:?}) => {:?}",
            v.f64(&vf).map_err(|e| e.to_string())
        )
        .unwrap();
    }
    for (_, r) in ranges() {
        writeln!(o.0, "to_single_value({r:?}) => {:?}", r.to_single_value()).unwrap();
    }

    // Database functions.
    let database = Value::AreaValue(vec![
        vec![s("Name"), s("Age"), s("Score")],
        vec![s("Ann"), f(30.0), f(7.5)],
        vec![s("bob"), f(41.0), f(3.0)],
        vec![s("Cat"), f(30.0), Value::None],
        vec![s("Ünï"), f(25.0), s("x")],
        vec![s("Dan"), f(52.0), f(9.0)],
    ]);
    let crits = [
        Value::AreaValue(vec![vec![s("Age")], vec![s(">29")]]),
        Value::AreaValue(vec![vec![s("Name")], vec![s("b*")]]),
        Value::AreaValue(vec![vec![s("Age"), s("Name")], vec![s("30"), s("<>Ann")]]),
        Value::AreaValue(vec![vec![s("Age")], vec![s(">29")], vec![s("<26")]]),
        Value::AreaValue(vec![vec![s("Score")], vec![s("")]]),
    ];
    for (i, c) in crits.iter().enumerate() {
        for field in [s("Score"), f(2.0), s("Name")] {
            o.guarded(&format!("DSUM(c{i},{field:?})"), || {
                db::d_sum(
                    database.clone(),
                    field.clone(),
                    c.clone(),
                    &ValueFormat::default(),
                )
            });
            o.guarded(&format!("DCOUNT(c{i},{field:?})"), || {
                db::d_count(
                    database.clone(),
                    field.clone(),
                    c.clone(),
                    &ValueFormat::default(),
                )
            });
            o.guarded(&format!("DAVERAGE(c{i},{field:?})"), || {
                db::d_average(
                    database.clone(),
                    field.clone(),
                    c.clone(),
                    &ValueFormat::default(),
                )
            });
            o.guarded(&format!("DMAX(c{i},{field:?})"), || {
                db::d_max(
                    database.clone(),
                    field.clone(),
                    c.clone(),
                    &ValueFormat::default(),
                )
            });
            o.guarded(&format!("DGET(c{i},{field:?})"), || {
                db::d_get(
                    database.clone(),
                    field.clone(),
                    c.clone(),
                    &ValueFormat::default(),
                )
            });
        }
    }

    let _ = std::panic::take_hook();
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/snapshots/behaviour.txt");
    if std::env::var_os("CODCEL_UPDATE_SNAPSHOT").is_some() {
        std::fs::create_dir_all(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/snapshots")).unwrap();
        std::fs::write(path, &o.0).unwrap();
        return;
    }
    let expected =
        std::fs::read_to_string(path).expect("snapshot missing: run with CODCEL_UPDATE_SNAPSHOT=1");
    let mut differences = 0;
    for (line, (got, want)) in o.0.lines().zip(expected.lines()).enumerate() {
        if got != want {
            differences += 1;
            if differences <= 20 {
                eprintln!("line {}:\n  want: {want}\n  got:  {got}", line + 1);
            }
        }
    }
    assert_eq!(
        o.0.lines().count(),
        expected.lines().count(),
        "number of lines differs"
    );
    assert_eq!(
        differences, 0,
        "{differences} results differ from the snapshot"
    );
}
