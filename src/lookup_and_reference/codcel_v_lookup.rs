// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::excel_error::{err_to_box, ExcelError};
use crate::lookup_and_reference::lookup_match::{excel_equals, LookupMatcher};
use crate::value::Value;
use std::cmp::Ordering;
use std::error::Error;

/// Performs a vertical lookup against the first column of `table_array`, like Excel's `VLOOKUP`.
///
/// `col_index_num` is 1-based and identifies which column's value to return from the matching
/// row.
///
/// When `range_lookup` is `Some(false)` an exact match is required, and the lookup value may
/// contain wildcards: `*` matches any sequence of characters, `?` matches any single character,
/// and `~` escapes them (`~*`, `~?`, `~~`). Matching is case-insensitive and `2` matches `2.0`,
/// but numbers never match text and booleans never match numbers. The first matching row wins.
///
/// When `range_lookup` is `None` or `Some(true)` the first column is treated as sorted ascending
/// and a binary search returns the last row whose value is less than or equal to `lookup_value`.
/// Wildcards are not applied in this mode, matching Excel. If the column turns out not to be
/// cleanly ordered — it holds error values, mixed incomparable types, or empty rows — the search
/// degrades to a full linear scan rather than returning a binary-search artefact.
///
/// `table_array` is expected to be rectangular, as an Excel range always is. Its width is taken
/// from the first row; a later row that is shorter yields a blank for the missing cell instead
/// of invalidating the whole table.
///
/// # Errors
/// - `#VALUE!` when `col_index_num` is less than 1.
/// - `#REF!` when `col_index_num` exceeds the table's width, including when the table is empty.
/// - `#N/A` when no row matches.
pub fn codcel_v_lookup(
    lookup_value: Value,
    table_array: Vec<Vec<Value>>,
    col_index_num: i32,
    range_lookup: Option<bool>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    // Excel validates the column index before it looks at any data, so an out-of-range index
    // wins over a missing match. Checked before the cast, otherwise a negative index wraps to
    // a huge `usize`.
    if col_index_num < 1 {
        return Err(err_to_box(ExcelError::Value));
    }
    let col_index_num = col_index_num as usize;

    // An empty table has width 0, so every column index is out of range and falls into the
    // same `#REF!` branch.
    let table_width = table_array.first().map_or(0, Vec::len);
    if col_index_num > table_width {
        return Err(err_to_box(ExcelError::Ref));
    }

    if range_lookup.unwrap_or(true) {
        approximate_v_lookup(&lookup_value, &table_array, col_index_num)
    } else {
        exact_v_lookup(&lookup_value, &table_array, col_index_num)
    }
}

fn exact_v_lookup(
    lookup_value: &Value,
    table_array: &[Vec<Value>],
    col_index_num: usize,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    // Built once, not once per row: a wildcard lookup compiles its regex here.
    let matcher = LookupMatcher::new(lookup_value)?;

    for row in table_array {
        let Some(key) = row.first() else { continue };
        if matcher.matches(key) {
            return Ok(cell_or_blank(row, col_index_num));
        }
    }

    Err(err_to_box(ExcelError::Na))
}

fn approximate_v_lookup(
    lookup_value: &Value,
    table_array: &[Vec<Value>],
    col_index_num: usize,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    // Partition point: `low` ends as the number of leading rows whose key is <= the lookup
    // value, so `low - 1` is the last row of a run of duplicates. That is Excel's tie-break
    // for approximate matches.
    let mut low = 0usize;
    let mut high = table_array.len();

    while low < high {
        let mid = low + (high - low) / 2;
        match key_cmp(&table_array[mid], lookup_value) {
            Some(Ordering::Greater) => high = mid,
            Some(Ordering::Less | Ordering::Equal) => low = mid + 1,
            // The key is an error value, an empty row, or a type that cannot be ordered
            // against the lookup value. The column is therefore not the ascending ladder a
            // binary search needs, and halving on an unordered probe would silently return an
            // arbitrary row. Restart as a linear scan, which stays correct for a sorted column
            // that merely has a few unusable cells sprinkled through it.
            None => return linear_approximate_v_lookup(lookup_value, table_array, col_index_num),
        }
    }

    if low == 0 {
        // Every key is greater than the lookup value.
        return Err(err_to_box(ExcelError::Na));
    }

    Ok(cell_or_blank(&table_array[low - 1], col_index_num))
}

/// Full scan keeping the last row whose key is less than or equal to the lookup value. Rows
/// whose key cannot be compared are skipped rather than terminating the scan.
fn linear_approximate_v_lookup(
    lookup_value: &Value,
    table_array: &[Vec<Value>],
    col_index_num: usize,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let mut best_match_row: Option<usize> = None;

    for (index, row) in table_array.iter().enumerate() {
        if matches!(
            key_cmp(row, lookup_value),
            Some(Ordering::Less | Ordering::Equal)
        ) {
            best_match_row = Some(index);
        }
    }

    best_match_row
        .map(|index| cell_or_blank(&table_array[index], col_index_num))
        .ok_or_else(|| err_to_box(ExcelError::Na))
}

/// Orders a row's first-column key against the lookup value.
///
/// Returns `None` when the row is empty or the two values are not comparable, which for `Value`
/// includes any pairing involving `Value::Error` and any mismatched pair of types.
/// `excel_equals` is consulted first so that a case-differing text key, or a `2` against a
/// `2.0`, reports `Equal` rather than the byte-order or type-strict answer.
fn key_cmp(row: &[Value], lookup_value: &Value) -> Option<Ordering> {
    let key = row.first()?;
    if excel_equals(lookup_value, key) {
        return Some(Ordering::Equal);
    }
    key.partial_cmp(lookup_value)
}

/// The requested cell, or a blank when the matched row is shorter than the table's first row.
/// Ragged input cannot come from a real spreadsheet range; treating the missing cell as blank
/// keeps the error surface to the three Excel errors `VLOOKUP` can actually raise.
fn cell_or_blank(row: &[Value], col_index_num: usize) -> Value {
    row.get(col_index_num - 1).cloned().unwrap_or(Value::None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    fn text(value: &str) -> Value {
        Value::String(value.to_string())
    }

    /// Three fruit rows: key, category, colour.
    fn fruit_table() -> Vec<Vec<Value>> {
        vec![
            vec![text("Apple"), text("Fruit"), text("Red")],
            vec![text("Banana"), text("Fruit"), text("Yellow")],
            vec![text("Carrot"), text("Vegetable"), text("Orange")],
        ]
    }

    /// Ascending numeric keys 0/60/70/80/90 with letter grades.
    fn grade_table() -> Vec<Vec<Value>> {
        vec![
            vec![Value::I32(0), text("F")],
            vec![Value::I32(60), text("D")],
            vec![Value::I32(70), text("C")],
            vec![Value::I32(80), text("B")],
            vec![Value::I32(90), text("A")],
        ]
    }

    // --- Exact match ---

    #[test]
    fn test_vlookup_exact_string_match() {
        // =VLOOKUP("Banana", A1:C3, 2, FALSE) in US format
        // =VLOOKUP("Banana"; A1:C3; 2; FALSE) in German format
        let result = codcel_v_lookup(text("Banana"), fruit_table(), 2, Some(false));
        assert_eq!(result.unwrap(), text("Fruit"));
    }

    #[test]
    fn test_vlookup_exact_numeric_match() {
        // =VLOOKUP(70, A1:B5, 2, FALSE) in US format
        // =VLOOKUP(70; A1:B5; 2; FALSE) in German format
        let result = codcel_v_lookup(Value::I32(70), grade_table(), 2, Some(false));
        assert_eq!(result.unwrap(), text("C"));
    }

    #[test]
    fn test_vlookup_exact_is_case_insensitive() {
        // =VLOOKUP("BANANA", A1:C3, 3, FALSE) in US format
        // =VLOOKUP("BANANA"; A1:C3; 3; FALSE) in German format
        // Excel's lookup comparison ignores case.
        let result = codcel_v_lookup(text("BANANA"), fruit_table(), 3, Some(false));
        assert_eq!(result.unwrap(), text("Yellow"));

        let result = codcel_v_lookup(text("carrot"), fruit_table(), 2, Some(false));
        assert_eq!(result.unwrap(), text("Vegetable"));
    }

    #[test]
    fn test_vlookup_exact_matches_across_numeric_types() {
        // =VLOOKUP(2, A1:B2, 2, FALSE) in US format
        // =VLOOKUP(2; A1:B2; 2; FALSE) in German format
        // The key column may hold F64 or I32 depending on the source; Excel sees one number.
        let table = vec![
            vec![Value::F64(2.0), text("two")],
            vec![Value::I32(3), text("three")],
        ];
        let result = codcel_v_lookup(Value::I32(2), table.clone(), 2, Some(false));
        assert_eq!(result.unwrap(), text("two"));

        let result = codcel_v_lookup(Value::F64(3.0), table, 2, Some(false));
        assert_eq!(result.unwrap(), text("three"));
    }

    #[test]
    fn test_vlookup_exact_bool_does_not_match_number() {
        // =VLOOKUP(TRUE, A1:B2, 2, FALSE) in US format
        // =VLOOKUP(TRUE; A1:B2; 2; FALSE) in German format
        let table = vec![
            vec![Value::Bool(true), text("yes")],
            vec![Value::Bool(false), text("no")],
        ];
        let result = codcel_v_lookup(Value::Bool(true), table.clone(), 2, Some(false));
        assert_eq!(result.unwrap(), text("yes"));

        let result = codcel_v_lookup(Value::I32(1), table, 2, Some(false));
        assert!(result.unwrap_err().to_string().contains("#N/A"));
    }

    #[test]
    fn test_vlookup_exact_number_does_not_match_text() {
        // =VLOOKUP(2, A1:B1, 2, FALSE) against a text key in US format
        // =VLOOKUP(2; A1:B1; 2; FALSE) against a text key in German format
        let text_key = vec![vec![text("2"), text("text two")]];
        let result = codcel_v_lookup(Value::I32(2), text_key, 2, Some(false));
        assert!(result.unwrap_err().to_string().contains("#N/A"));

        let number_key = vec![vec![Value::I32(2), text("number two")]];
        let result = codcel_v_lookup(text("2"), number_key, 2, Some(false));
        assert!(result.unwrap_err().to_string().contains("#N/A"));
    }

    #[test]
    fn test_vlookup_exact_returns_first_of_duplicates() {
        // =VLOOKUP("A", A1:B3, 2, FALSE) in US format
        // =VLOOKUP("A"; A1:B3; 2; FALSE) in German format
        // Excel's exact match returns the first hit, the opposite of the approximate path.
        let table = vec![
            vec![text("A"), text("first")],
            vec![text("A"), text("second")],
            vec![text("B"), text("third")],
        ];
        let result = codcel_v_lookup(text("A"), table, 2, Some(false));
        assert_eq!(result.unwrap(), text("first"));
    }

    #[test]
    fn test_vlookup_exact_miss_is_not_available() {
        // =VLOOKUP("Durian", A1:C3, 2, FALSE) in US format
        // =VLOOKUP("Durian"; A1:C3; 2; FALSE) in German format
        let result = codcel_v_lookup(text("Durian"), fruit_table(), 2, Some(false));
        assert!(result.unwrap_err().to_string().contains("#N/A"));
    }

    #[test]
    fn test_vlookup_exact_does_not_require_sorting() {
        // =VLOOKUP(10, A1:B3, 2, FALSE) in US format
        // =VLOOKUP(10; A1:B3; 2; FALSE) in German format
        let table = vec![
            vec![Value::I32(30), text("thirty")],
            vec![Value::I32(10), text("ten")],
            vec![Value::I32(20), text("twenty")],
        ];
        let result = codcel_v_lookup(Value::I32(10), table, 2, Some(false));
        assert_eq!(result.unwrap(), text("ten"));
    }

    #[test]
    fn test_vlookup_exact_datetime_key() {
        // =VLOOKUP(DATE(2023,1,1), A1:B2, 2, FALSE) in US format
        // =VLOOKUP(DATE(2023;1;1); A1:B2; 2; FALSE) in German format
        let day_one = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let day_two = Utc.with_ymd_and_hms(2023, 1, 2, 0, 0, 0).unwrap();
        let table = vec![
            vec![Value::ChronoDateTime(day_one), text("Day 1")],
            vec![Value::ChronoDateTime(day_two), text("Day 2")],
        ];
        let result = codcel_v_lookup(Value::ChronoDateTime(day_two), table, 2, Some(false));
        assert_eq!(result.unwrap(), text("Day 2"));
    }

    #[test]
    fn test_vlookup_exact_matches_option_wrapped_values() {
        // =VLOOKUP("Banana", A1:B2, 2, FALSE) where the column holds optional values
        // =VLOOKUP("Banana"; A1:B2; 2; FALSE) in German format
        let table = vec![
            vec![
                Value::OptionString(Some("Banana".to_string())),
                text("Fruit"),
            ],
            vec![Value::OptionI32(Some(2)), text("two")],
        ];
        let result = codcel_v_lookup(text("Banana"), table.clone(), 2, Some(false));
        assert_eq!(result.unwrap(), text("Fruit"));

        let result = codcel_v_lookup(Value::I32(2), table, 2, Some(false));
        assert_eq!(result.unwrap(), text("two"));
    }

    #[test]
    fn test_vlookup_exact_error_cell_never_matches() {
        // An error value in the lookup column must not match an ordinary lookup value.
        let table = vec![
            vec![Value::Error(ExcelError::Na), text("error row")],
            vec![text("b"), text("b row")],
        ];
        let result = codcel_v_lookup(text("b"), table.clone(), 2, Some(false));
        assert_eq!(result.unwrap(), text("b row"));

        let result = codcel_v_lookup(text("a"), table, 2, Some(false));
        assert!(result.unwrap_err().to_string().contains("#N/A"));
    }

    // --- Wildcards (exact match only) ---

    #[test]
    fn test_vlookup_wildcard_trailing_star() {
        // =VLOOKUP("App*", A1:C3, 3, FALSE) in US format
        // =VLOOKUP("App*"; A1:C3; 3; FALSE) in German format
        let result = codcel_v_lookup(text("App*"), fruit_table(), 3, Some(false));
        assert_eq!(result.unwrap(), text("Red"));
    }

    #[test]
    fn test_vlookup_wildcard_leading_and_inner_star() {
        let result = codcel_v_lookup(text("*ple"), fruit_table(), 2, Some(false));
        assert_eq!(result.unwrap(), text("Fruit"));

        let result = codcel_v_lookup(text("C*t"), fruit_table(), 2, Some(false));
        assert_eq!(result.unwrap(), text("Vegetable"));

        let result = codcel_v_lookup(text("*z*"), fruit_table(), 2, Some(false));
        assert!(result.unwrap_err().to_string().contains("#N/A"));
    }

    #[test]
    fn test_vlookup_wildcard_question_mark() {
        // =VLOOKUP("B?nana", A1:C3, 3, FALSE) in US format
        // =VLOOKUP("B?nana"; A1:C3; 3; FALSE) in German format
        let result = codcel_v_lookup(text("B?nana"), fruit_table(), 3, Some(false));
        assert_eq!(result.unwrap(), text("Yellow"));

        // The pattern is anchored to the whole cell, so a prefix does not match.
        let result = codcel_v_lookup(text("B?n"), fruit_table(), 3, Some(false));
        assert!(result.unwrap_err().to_string().contains("#N/A"));
    }

    #[test]
    fn test_vlookup_wildcard_escaped_star_is_literal() {
        // =VLOOKUP("10~*20", A1:B2, 2, FALSE) in US format
        // =VLOOKUP("10~*20"; A1:B2; 2; FALSE) in German format
        let table = vec![
            vec![text("10*20"), text("literal star")],
            vec![text("103020"), text("digits")],
        ];
        let result = codcel_v_lookup(text("10~*20"), table.clone(), 2, Some(false));
        assert_eq!(result.unwrap(), text("literal star"));

        let result = codcel_v_lookup(text("10~*30"), table.clone(), 2, Some(false));
        assert!(result.unwrap_err().to_string().contains("#N/A"));

        // Unescaped, the `*` is a live wildcard and matches the digits row first.
        let result = codcel_v_lookup(text("10*20"), table, 2, Some(false));
        assert_eq!(result.unwrap(), text("literal star"));
    }

    #[test]
    fn test_vlookup_wildcard_escaped_question_is_literal() {
        let table = vec![
            vec![text("AB"), text("plain")],
            vec![text("A?"), text("literal question")],
        ];
        let result = codcel_v_lookup(text("A~?"), table.clone(), 2, Some(false));
        assert_eq!(result.unwrap(), text("literal question"));

        let result = codcel_v_lookup(text("A~?B"), table, 2, Some(false));
        assert!(result.unwrap_err().to_string().contains("#N/A"));
    }

    #[test]
    fn test_vlookup_wildcard_escaped_tilde_is_literal() {
        let table = vec![vec![text("a~b"), text("tilde")]];
        let result = codcel_v_lookup(text("a~~b"), table, 2, Some(false));
        assert_eq!(result.unwrap(), text("tilde"));
    }

    #[test]
    fn test_vlookup_wildcard_is_case_insensitive() {
        let result = codcel_v_lookup(text("app*"), fruit_table(), 2, Some(false));
        assert_eq!(result.unwrap(), text("Fruit"));
    }

    #[test]
    fn test_vlookup_wildcard_does_not_match_numbers() {
        // Excel applies wildcards to text criteria only.
        let table = vec![vec![Value::I32(123), text("number")]];
        let result = codcel_v_lookup(text("12*"), table, 2, Some(false));
        assert!(result.unwrap_err().to_string().contains("#N/A"));
    }

    #[test]
    fn test_vlookup_wildcard_regex_metacharacters_are_literal() {
        // `.` must not behave as a regex wildcard.
        let table = vec![
            vec![text("abc"), text("no dot")],
            vec![text("a.c"), text("dot")],
        ];
        let result = codcel_v_lookup(text("a.c"), table, 2, Some(false));
        assert_eq!(result.unwrap(), text("dot"));
    }

    #[test]
    fn test_vlookup_wildcard_not_applied_in_approximate_mode() {
        // =VLOOKUP("C*", A1:C3, 2) in US format
        // =VLOOKUP("C*"; A1:C3; 2) in German format
        // Approximate mode ignores wildcards and orders instead: "C*" sorts between "Banana"
        // and "Carrot" because `*` is below `a` in code-point order, so the answer is
        // Banana's row. A wildcard match would have returned Carrot's "Vegetable".
        let result = codcel_v_lookup(text("C*"), fruit_table(), 2, None);
        assert_eq!(result.unwrap(), text("Fruit"));
    }

    // --- Approximate match ---

    #[test]
    fn test_vlookup_approximate_exact_hit() {
        // =VLOOKUP(80, A1:B5, 2, TRUE) in US format
        // =VLOOKUP(80; A1:B5; 2; TRUE) in German format
        let result = codcel_v_lookup(Value::I32(80), grade_table(), 2, Some(true));
        assert_eq!(result.unwrap(), text("B"));
    }

    #[test]
    fn test_vlookup_approximate_between_keys() {
        // =VLOOKUP(85, A1:B5, 2, TRUE) in US format
        // =VLOOKUP(85; A1:B5; 2; TRUE) in German format
        let result = codcel_v_lookup(Value::I32(85), grade_table(), 2, Some(true));
        assert_eq!(result.unwrap(), text("B"));
    }

    #[test]
    fn test_vlookup_approximate_below_first_key_is_not_available() {
        // =VLOOKUP(0, A1:B3, 2, TRUE) in US format
        // =VLOOKUP(0; A1:B3; 2; TRUE) in German format
        let table = vec![
            vec![Value::I32(1), text("one")],
            vec![Value::I32(2), text("two")],
            vec![Value::I32(3), text("three")],
        ];
        let result = codcel_v_lookup(Value::I32(0), table, 2, Some(true));
        assert!(result.unwrap_err().to_string().contains("#N/A"));
    }

    #[test]
    fn test_vlookup_approximate_above_last_key_returns_last_row() {
        // =VLOOKUP(1000, A1:B5, 2, TRUE) in US format
        // =VLOOKUP(1000; A1:B5; 2; TRUE) in German format
        let result = codcel_v_lookup(Value::I32(1000), grade_table(), 2, Some(true));
        assert_eq!(result.unwrap(), text("A"));
    }

    #[test]
    fn test_vlookup_approximate_duplicates_return_last_row() {
        // =VLOOKUP(2, A1:B5, 2, TRUE) in US format
        // =VLOOKUP(2; A1:B5; 2; TRUE) in German format
        // Excel's approximate match lands on the last row of a run of duplicate keys.
        let table = vec![
            vec![Value::I32(1), text("a")],
            vec![Value::I32(2), text("b")],
            vec![Value::I32(2), text("c")],
            vec![Value::I32(2), text("d")],
            vec![Value::I32(3), text("e")],
        ];
        let result = codcel_v_lookup(Value::I32(2), table, 2, Some(true));
        assert_eq!(result.unwrap(), text("d"));
    }

    #[test]
    fn test_vlookup_approximate_is_the_default() {
        // =VLOOKUP(85, A1:B5, 2) in US format
        // =VLOOKUP(85; A1:B5; 2) in German format
        let omitted = codcel_v_lookup(Value::I32(85), grade_table(), 2, None).unwrap();
        let explicit = codcel_v_lookup(Value::I32(85), grade_table(), 2, Some(true)).unwrap();
        assert_eq!(omitted, explicit);
        assert_eq!(omitted, text("B"));
    }

    #[test]
    fn test_vlookup_approximate_mixed_int_and_float_column() {
        // =VLOOKUP(3, A1:B3, 2, TRUE) in US format
        // =VLOOKUP(3; A1:B3; 2; TRUE) in German format
        let table = vec![
            vec![Value::I32(1), text("one")],
            vec![Value::F64(2.5), text("two point five")],
            vec![Value::I32(4), text("four")],
        ];
        let result = codcel_v_lookup(Value::F64(3.0), table, 2, Some(true));
        assert_eq!(result.unwrap(), text("two point five"));
    }

    #[test]
    fn test_vlookup_approximate_binary_search_on_large_table() {
        // A 1000-row table keyed 0, 2, 4, … 1998. The binary search must land exactly.
        let table: Vec<Vec<Value>> = (0..1000)
            .map(|i| vec![Value::I32(i * 2), Value::I32(i)])
            .collect();

        // First key.
        let result = codcel_v_lookup(Value::I32(0), table.clone(), 2, Some(true));
        assert_eq!(result.unwrap(), Value::I32(0));

        // An odd value rounds down to the preceding even key.
        let result = codcel_v_lookup(Value::I32(1001), table.clone(), 2, Some(true));
        assert_eq!(result.unwrap(), Value::I32(500));

        // Last key, and beyond it.
        let result = codcel_v_lookup(Value::I32(1998), table.clone(), 2, Some(true));
        assert_eq!(result.unwrap(), Value::I32(999));
        let result = codcel_v_lookup(Value::I32(5000), table.clone(), 2, Some(true));
        assert_eq!(result.unwrap(), Value::I32(999));

        // Below the first key.
        let result = codcel_v_lookup(Value::I32(-1), table, 2, Some(true));
        assert!(result.unwrap_err().to_string().contains("#N/A"));
    }

    #[test]
    fn test_vlookup_approximate_error_in_key_column_falls_back_to_scan() {
        // An error value cannot be ordered, so the binary search degrades to a linear scan
        // instead of halving on an unordered probe.
        let table = vec![
            vec![Value::I32(1), text("one")],
            vec![Value::Error(ExcelError::Na), text("error")],
            vec![Value::I32(3), text("three")],
        ];
        let result = codcel_v_lookup(Value::I32(3), table.clone(), 2, Some(true));
        assert_eq!(result.unwrap(), text("three"));

        let result = codcel_v_lookup(Value::I32(2), table, 2, Some(true));
        assert_eq!(result.unwrap(), text("one"));
    }

    #[test]
    fn test_vlookup_approximate_incomparable_types_fall_back_to_scan() {
        let table = vec![
            vec![Value::I32(1), text("one")],
            vec![text("x"), text("text key")],
            vec![Value::I32(3), text("three")],
        ];
        let result = codcel_v_lookup(Value::I32(2), table, 2, Some(true));
        assert_eq!(result.unwrap(), text("one"));
    }

    #[test]
    fn test_vlookup_approximate_sorted_text_column() {
        // =VLOOKUP("Bbz", A1:B3, 2, TRUE) in US format
        // =VLOOKUP("Bbz"; A1:B3; 2; TRUE) in German format
        let table = vec![
            vec![text("Ba"), text("first")],
            vec![text("Bb"), text("second")],
            vec![text("Bc"), text("third")],
        ];
        let result = codcel_v_lookup(text("Bbz"), table, 2, Some(true));
        assert_eq!(result.unwrap(), text("second"));
    }

    #[test]
    fn test_vlookup_approximate_unsorted_column_is_deterministic() {
        // Excel leaves the result of an approximate lookup over an unsorted column
        // unspecified. This pins our own deterministic behaviour rather than an
        // Excel-verified answer, so a future change to the search is caught.
        let table = vec![
            vec![Value::I32(5), text("a")],
            vec![Value::I32(4), text("b")],
            vec![Value::I32(3), text("c")],
            vec![Value::I32(2), text("d")],
            vec![Value::I32(1), text("e")],
        ];
        let result = codcel_v_lookup(Value::I32(3), table, 2, Some(true));
        assert_eq!(result.unwrap(), text("e"));
    }

    // --- Validation and table shape ---

    #[test]
    fn test_vlookup_zero_column_index_is_value_error() {
        // =VLOOKUP("Apple", A1:C3, 0, FALSE) in US format
        // =VLOOKUP("Apple"; A1:C3; 0; FALSE) in German format
        let result = codcel_v_lookup(text("Apple"), fruit_table(), 0, Some(false));
        assert!(result.unwrap_err().to_string().contains("#VALUE!"));
    }

    #[test]
    fn test_vlookup_negative_column_index_is_value_error() {
        // A negative index used to wrap to a huge `usize` and report a nonsense column count.
        let result = codcel_v_lookup(text("Apple"), fruit_table(), -1, Some(false));
        assert!(result.unwrap_err().to_string().contains("#VALUE!"));

        let result = codcel_v_lookup(text("Apple"), fruit_table(), i32::MIN, Some(false));
        assert!(result.unwrap_err().to_string().contains("#VALUE!"));
    }

    #[test]
    fn test_vlookup_column_index_beyond_width_is_ref_error() {
        // =VLOOKUP("Apple", A1:C3, 4, FALSE) in US format
        // =VLOOKUP("Apple"; A1:C3; 4; FALSE) in German format
        let result = codcel_v_lookup(text("Apple"), fruit_table(), 4, Some(false));
        assert!(result.unwrap_err().to_string().contains("#REF!"));
    }

    #[test]
    fn test_vlookup_empty_table_is_ref_error() {
        let empty: Vec<Vec<Value>> = vec![];
        let result = codcel_v_lookup(Value::I32(1), empty, 1, None);
        assert!(result.unwrap_err().to_string().contains("#REF!"));
    }

    #[test]
    fn test_vlookup_table_with_zero_width_row_is_ref_error() {
        let result = codcel_v_lookup(Value::I32(1), vec![vec![]], 1, None);
        assert!(result.unwrap_err().to_string().contains("#REF!"));
    }

    #[test]
    fn test_vlookup_single_row_table() {
        let table = vec![vec![text("Apple"), text("Red")]];
        let result = codcel_v_lookup(text("Apple"), table.clone(), 2, Some(false));
        assert_eq!(result.unwrap(), text("Red"));

        let result = codcel_v_lookup(text("Apple"), table, 2, Some(true));
        assert_eq!(result.unwrap(), text("Red"));
    }

    #[test]
    fn test_vlookup_single_column_table() {
        // =VLOOKUP(2, A1:A3, 1, FALSE) in US format
        // =VLOOKUP(2; A1:A3; 1; FALSE) in German format
        let table = vec![
            vec![Value::I32(1)],
            vec![Value::I32(2)],
            vec![Value::I32(3)],
        ];
        let result = codcel_v_lookup(Value::I32(2), table, 1, Some(false));
        assert_eq!(result.unwrap(), Value::I32(2));
    }

    #[test]
    fn test_vlookup_ragged_table_is_not_rejected() {
        // A short row elsewhere in the table no longer invalidates the whole lookup.
        let table = vec![
            vec![text("Apple"), text("Fruit"), text("Red")],
            vec![text("Banana"), text("Fruit")],
            vec![text("Carrot"), text("Vegetable"), text("Orange")],
        ];
        let result = codcel_v_lookup(text("Carrot"), table, 3, Some(false));
        assert_eq!(result.unwrap(), text("Orange"));
    }

    #[test]
    fn test_vlookup_ragged_short_matched_row_returns_blank() {
        // The matched row has no third cell, so the result is a blank rather than an error.
        let table = vec![
            vec![text("Apple"), text("Fruit"), text("Red")],
            vec![text("Banana"), text("Fruit")],
        ];
        let result = codcel_v_lookup(text("Banana"), table, 3, Some(false));
        assert_eq!(result.unwrap(), Value::None);
    }

    #[test]
    fn test_vlookup_error_result_cell_is_returned_as_a_value() {
        // An error stored in the result column is data, not a function failure.
        let table = vec![vec![text("Apple"), Value::Error(ExcelError::Div0)]];
        let result = codcel_v_lookup(text("Apple"), table, 2, Some(false));
        assert_eq!(result.unwrap(), Value::Error(ExcelError::Div0));
    }
}
