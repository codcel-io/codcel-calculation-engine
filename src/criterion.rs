// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

//! The criteria of COUNTIF, SUMIF, AVERAGEIF, the *IFS functions and the D* functions, parsed
//! once per call rather than once per cell.
//!
//! Each of those functions tests every cell of a range against a criteria string such as
//! `">2"` or `"a*"`. [`Criterion::matches`] gives the same answer as lowercasing a text cell and
//! calling [`compare`] with the lowercased criteria, which is what the functions did before,
//! but the criteria's number, whole-number and boolean readings are worked out once, and a
//! number or text cell is compared without allocating. Cells of any other kind still go
//! through [`compare`].

use std::cmp::Ordering;

use crate::condition::{parse_condition, Condition};
use crate::match_type_and_compare_macro::compare;
use crate::value::Value;
use crate::value_format::ValueFormat;

/// A parsed criteria string.
pub(crate) struct Criterion {
    operator: Condition,
    /// The value to compare against, lowercased, as `compare` takes it.
    rhs: Value,
    /// The value as text: what `rhs.string()` gives.
    text: String,
    /// The pattern chars, when the operator is `=` and the text has a `*` or `?`.
    wildcard: Option<Vec<char>>,
    /// `rhs.f64()`, `rhs.i32()` and `rhs.bool()`, or `None` where they fail.
    number: Option<f64>,
    whole_number: Option<i32>,
    boolean: Option<bool>,
}

impl Criterion {
    /// Parses `criteria`, a string such as `">=2"`, `"<>abc"` or `"a*"`.
    pub(crate) fn new(criteria: &str, value_format: &ValueFormat) -> Self {
        let (operator, comparison_value) = parse_condition(criteria);
        let text = comparison_value.to_lowercase();
        let rhs = Value::String(text.clone());
        let wildcard = (operator == Condition::Equal && (text.contains('*') || text.contains('?')))
            .then(|| text.chars().collect());
        Criterion {
            number: rhs.f64(value_format).ok(),
            whole_number: rhs.i32(value_format).ok(),
            boolean: rhs.bool(value_format).ok(),
            operator,
            rhs,
            text,
            wildcard,
        }
    }

    /// Whether `cell` meets the criterion. Text is compared without regard to case.
    pub(crate) fn matches(&self, cell: &Value, value_format: &ValueFormat) -> bool {
        match cell {
            Value::F64(lhs) => self.with_text_fallback(cell, value_format, |c| {
                c.number.map(|rhs| c.holds(lhs.partial_cmp(&rhs)))
            }),
            Value::I32(lhs) => self.with_text_fallback(cell, value_format, |c| {
                c.whole_number.map(|rhs| c.holds(lhs.partial_cmp(&rhs)))
            }),
            Value::Bool(lhs) => self.with_text_fallback(cell, value_format, |c| {
                c.boolean.map(|rhs| c.holds(lhs.partial_cmp(&rhs)))
            }),
            Value::String(text) => {
                if let Some(pattern) = &self.wildcard {
                    return wildcard_matches(pattern, &text.to_lowercase());
                }
                if text.is_ascii() {
                    let lowered = text.bytes().map(|b| b.to_ascii_lowercase());
                    self.holds(Some(lowered.cmp(self.text.bytes())))
                } else {
                    self.holds(Some(text.to_lowercase().as_str().cmp(self.text.as_str())))
                }
            }
            other => self.compare_generic(other, value_format),
        }
    }

    /// The comparison `compare` makes for a number, whole number or boolean cell: by value when
    /// the criteria reads as the cell's type (`typed`), otherwise as text.
    fn with_text_fallback(
        &self,
        cell: &Value,
        value_format: &ValueFormat,
        typed: impl Fn(&Self) -> Option<bool>,
    ) -> bool {
        if let Some(pattern) = &self.wildcard {
            return cell
                .string(value_format)
                .is_ok_and(|text| wildcard_matches(pattern, &text));
        }
        if let Some(result) = typed(self) {
            return result;
        }
        cell.string(value_format)
            .is_ok_and(|text| self.holds(Some(text.as_str().cmp(self.text.as_str()))))
    }

    /// Whether the operator holds for `ordering` (`None`: the values are unordered, as NaN is).
    fn holds(&self, ordering: Option<Ordering>) -> bool {
        match (&self.operator, ordering) {
            (Condition::Equal, Some(o)) => o == Ordering::Equal,
            (Condition::NotEqual, Some(o)) => o != Ordering::Equal,
            (Condition::LessThan, Some(o)) => o == Ordering::Less,
            (Condition::GreaterThan, Some(o)) => o == Ordering::Greater,
            (Condition::LessThanOrEqual, Some(o)) => o != Ordering::Greater,
            (Condition::GreaterThanOrEqual, Some(o)) => o != Ordering::Less,
            (Condition::NotEqual, None) => true,
            (_, None) => false,
        }
    }

    /// The earlier comparison, for cells of any other kind.
    fn compare_generic(&self, cell: &Value, value_format: &ValueFormat) -> bool {
        let lhs = case_insensitive_cell(cell, value_format);
        compare(&lhs, &self.rhs, &self.operator, value_format)
            .map(|result| result.bool(value_format).unwrap_or(false))
            .unwrap_or(false)
    }
}

/// Lowercases a text cell for case-insensitive matching. Other values are returned as they are.
pub(crate) fn case_insensitive_cell(value: &Value, value_format: &ValueFormat) -> Value {
    if value.is_string() {
        if let Ok(s) = value.string(value_format) {
            return Value::String(s.to_lowercase());
        }
    }
    value.clone()
}

/// The wildcard match `compare` makes: `?` is any one character and `*` any run of them,
/// with no escape character. A `*` followed by more pattern only tries the positions that
/// leave some text, as the earlier recursive version did.
fn wildcard_matches(pattern: &[char], text: &str) -> bool {
    let text: Vec<char> = text.chars().collect();
    matches_from(pattern, &text)
}

fn matches_from(pattern: &[char], text: &[char]) -> bool {
    let mut t = 0;
    let mut p = 0;
    while p < pattern.len() {
        match pattern[p] {
            '?' => {
                if t >= text.len() {
                    return false;
                }
                t += 1;
                p += 1;
            }
            '*' => {
                p += 1;
                if p == pattern.len() {
                    return true;
                }
                return (t..text.len()).any(|start| matches_from(&pattern[p..], &text[start..]));
            }
            c => {
                if text.get(t) != Some(&c) {
                    return false;
                }
                t += 1;
                p += 1;
            }
        }
    }
    t == text.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::excel_error::ExcelError;
    use chrono::{NaiveTime, TimeZone, Utc};

    /// What the criteria functions computed before: lowercase the cell, then `compare`.
    fn earlier(cell: &Value, criteria: &str, value_format: &ValueFormat) -> bool {
        let (operator, comparison_value) = parse_condition(criteria);
        let rhs = Value::String(comparison_value.to_lowercase());
        let lhs = case_insensitive_cell(cell, value_format);
        compare(&lhs, &rhs, &operator, value_format)
            .map(|result| result.bool(value_format).unwrap_or(false))
            .unwrap_or(false)
    }

    fn cells() -> Vec<Value> {
        let s = |v: &str| Value::String(v.to_string());
        vec![
            Value::F64(0.0),
            Value::F64(-0.0),
            Value::F64(2.0),
            Value::F64(2.5),
            Value::F64(-3.25),
            Value::F64(1e15),
            Value::F64(f64::NAN),
            Value::F64(12345.678),
            Value::I32(0),
            Value::I32(2),
            Value::I32(3),
            Value::I32(-7),
            Value::Bool(true),
            Value::Bool(false),
            s(""),
            s("abc"),
            s("ABC"),
            s("Abd"),
            s("a"),
            s("b"),
            s("2"),
            s("2.5"),
            s(" 2"),
            s("true"),
            s("TRUE"),
            s("Ünïcödé café"),
            s("ÜNÏCÖDÉ CAFÉ"),
            s("ΣΑΣ"),
            s("straße"),
            s("a*c"),
            s("x?y"),
            s("~"),
            Value::None,
            Value::OptionF64(None),
            Value::OptionF64(Some(2.0)),
            Value::OptionI32(Some(2)),
            Value::OptionString(Some("ABC".into())),
            Value::OptionString(None),
            Value::OptionBool(Some(true)),
            Value::Error(ExcelError::Na),
            Value::ChronoDateTime(
                Utc.with_ymd_and_hms(2024, 1, 5, 0, 0, 0)
                    .single()
                    .expect("a valid date"),
            ),
            Value::Time(NaiveTime::from_hms_opt(12, 0, 0).unwrap()),
            Value::VecValue(vec![Value::F64(2.0), s("abc")]),
            Value::AreaValue(vec![vec![s("ABC")]]),
        ]
    }

    fn criteria() -> Vec<String> {
        let mut out = Vec::new();
        for op in ["", "=", "<>", "<", ">", "<=", ">="] {
            for value in [
                "",
                "0",
                "-0",
                "2",
                "2.0",
                "2.5",
                "-3.25",
                "3",
                "1e15",
                "abc",
                "ABC",
                "ab",
                "abd",
                "a*",
                "*c",
                "a?c",
                "?",
                "*",
                "**",
                "a**",
                "*?",
                "true",
                "FALSE",
                "Ünïcödé café",
                "σας",
                "STRASSE",
                "straße",
                " 2 ",
                "2,5",
                "~",
                "a~*c",
                "45296",
            ] {
                out.push(format!("{op}{value}"));
            }
        }
        out
    }

    #[test]
    fn matches_the_earlier_comparison() {
        let comma_decimal = ValueFormat {
            decimal_separator: ",".to_string(),
            ..Default::default()
        };
        for value_format in [ValueFormat::default(), comma_decimal] {
            for criteria in criteria() {
                let criterion = Criterion::new(&criteria, &value_format);
                for cell in cells() {
                    assert_eq!(
                        criterion.matches(&cell, &value_format),
                        earlier(&cell, &criteria, &value_format),
                        "{cell:?} against {criteria:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn wildcard_follows_the_earlier_matcher() {
        let m = |p: &str, t: &str| wildcard_matches(&p.chars().collect::<Vec<_>>(), t);
        assert!(m("a*", "a"));
        assert!(m("a*c", "abbc"));
        assert!(
            !m("a**", "a"),
            "a * before more pattern needs some text, as before"
        );
        assert!(m("?b?", "abc"));
        assert!(!m("?", ""));
        assert!(m("*", ""));
    }
}
