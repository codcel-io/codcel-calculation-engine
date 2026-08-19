// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

//! Shared matching machinery for the lookup functions.
//!
//! [`Value`]'s own `PartialEq` is type-strict and case-sensitive, which is not how Excel
//! compares a lookup value against a lookup column. This module provides the Excel rules
//! once so that `VLOOKUP`, `HLOOKUP` and (in future) the rest of the lookup family share a
//! single definition.
//!
//! Known limitation: the wildcard path folds case through the regex engine's simple case
//! folding while [`excel_equals`] uses full Unicode lowercase mapping. The two can disagree
//! on exotic characters such as `İ`. No realistic spreadsheet is affected, and unifying them
//! would mean reimplementing case folding by hand.

use crate::excel_error::{err_to_box, ExcelError};
use crate::text::codcel_search::build_wildcard_pattern;
use crate::value::Value;
use regex::Regex;
use std::error::Error;

/// Compares two values the way Excel's lookup functions compare them.
///
/// Excel's rules, each of which differs from [`Value`]'s derived equality:
/// - Text compares case-insensitively, so `"apple"` equals `"APPLE"`.
/// - Numbers compare by value across storage types, so `2` equals `2.0`.
/// - `Option`-wrapped values compare as their contents, so `Some("a")` equals `"a"`.
/// - Booleans are their own type: `TRUE` does **not** equal `1` or `"TRUE"`.
/// - Numbers and text are never equal: `2` does **not** equal `"2"`.
/// - An error value equals only the identical error value, so an error cell in the lookup
///   column never matches an ordinary lookup value.
///
/// Anything not covered above (blanks, vectors, areas) falls through to `Value`'s own
/// type-strict equality.
pub fn excel_equals(lookup_value: &Value, candidate: &Value) -> bool {
    if let (Some(a), Some(b)) = (lookup_number(lookup_value), lookup_number(candidate)) {
        // The NaN arm mirrors `Value`'s existing PartialEq. Excel has no NaN, so this is
        // only reachable through directly constructed values.
        return a == b || (a.is_nan() && b.is_nan());
    }

    if let (Some(a), Some(b)) = (lookup_text(lookup_value), lookup_text(candidate)) {
        return text_eq_ignore_case(a, b);
    }

    match (lookup_value, candidate) {
        (
            Value::Bool(a) | Value::OptionBool(Some(a)),
            Value::Bool(b) | Value::OptionBool(Some(b)),
        ) => a == b,
        (
            Value::ChronoDateTime(a) | Value::OptionChronoDateTime(Some(a)),
            Value::ChronoDateTime(b) | Value::OptionChronoDateTime(Some(b)),
        ) => a == b,
        (
            Value::Time(a) | Value::OptionTime(Some(a)),
            Value::Time(b) | Value::OptionTime(Some(b)),
        ) => a == b,
        _ => lookup_value == candidate,
    }
}

/// The numeric payload of a value, for the values Excel treats as numbers in a lookup.
/// Booleans and dates are deliberately excluded: Excel does not coerce them to numbers when
/// matching.
fn lookup_number(value: &Value) -> Option<f64> {
    match value {
        Value::F64(v) | Value::OptionF64(Some(v)) => Some(*v),
        Value::I32(v) | Value::OptionI32(Some(v)) => Some(f64::from(*v)),
        _ => None,
    }
}

/// The text payload of a value, for the values Excel treats as text in a lookup. Numbers are
/// deliberately excluded: Excel does not coerce them to text when matching.
fn lookup_text(value: &Value) -> Option<&str> {
    match value {
        Value::String(s) | Value::OptionString(Some(s)) => Some(s.as_str()),
        _ => None,
    }
}

/// Case-insensitive text equality using Unicode lowercase mapping, without allocating.
fn text_eq_ignore_case(a: &str, b: &str) -> bool {
    a.chars()
        .flat_map(char::to_lowercase)
        .eq(b.chars().flat_map(char::to_lowercase))
}

/// Whether `pattern` contains a character Excel treats specially in a wildcard pattern:
/// `*`, `?`, or the escape `~`.
///
/// The tilde counts even when no live wildcard is present. `VLOOKUP("~*", …)` looks for a
/// cell holding a literal `*`, so `"~*"` must take the pattern path — only that path strips
/// the escape.
pub fn needs_wildcard_matching(pattern: &str) -> bool {
    pattern.contains(['*', '?', '~'])
}

/// A comparison prepared against one lookup value.
///
/// Build it once per lookup call, then call [`LookupMatcher::matches`] per candidate. The
/// wildcard variant compiles its regex during construction, so a large table pays the
/// compilation cost once rather than once per row.
pub enum LookupMatcher<'a> {
    /// Plain Excel equality; see [`excel_equals`].
    Plain(&'a Value),
    /// Anchored, case-insensitive wildcard match, applied to text cells only.
    Wildcard(Box<Regex>),
}

impl<'a> LookupMatcher<'a> {
    /// Builds a matcher for `lookup_value`.
    ///
    /// A wildcard matcher is built only when the lookup value is text containing `*`, `?` or
    /// `~`. Everything else — including a number whose text form happens to contain a
    /// wildcard character — gets a plain matcher, mirroring Excel, where wildcards are a
    /// text-criteria feature.
    ///
    /// # Errors
    /// Returns `#VALUE!` if the wildcard pattern fails to compile. Unreachable in practice,
    /// because [`build_wildcard_pattern`] escapes every regex metacharacter.
    pub fn new(lookup_value: &'a Value) -> Result<Self, Box<dyn Error + Send + Sync>> {
        if let Value::String(pattern) | Value::OptionString(Some(pattern)) = lookup_value {
            if needs_wildcard_matching(pattern) {
                // `(?is)`: case-insensitive, and `.` also matches a newline so that `?`
                // behaves like Excel's "any single character".
                // `\A`/`\z`: Excel wildcards match the whole cell, unlike SEARCH, which
                // matches a substring.
                let body = build_wildcard_pattern(pattern);
                let regex = Regex::new(&format!(r"(?is)\A{body}\z"))
                    .map_err(|_| err_to_box(ExcelError::Value))?;
                return Ok(LookupMatcher::Wildcard(Box::new(regex)));
            }
        }
        Ok(LookupMatcher::Plain(lookup_value))
    }

    /// Whether `candidate` matches the lookup value.
    ///
    /// A wildcard matcher only ever matches text cells: Excel does not apply wildcards to
    /// numbers, booleans, dates or blanks.
    pub fn matches(&self, candidate: &Value) -> bool {
        match self {
            LookupMatcher::Plain(lookup_value) => excel_equals(lookup_value, candidate),
            LookupMatcher::Wildcard(regex) => match candidate {
                Value::String(text) | Value::OptionString(Some(text)) => regex.is_match(text),
                _ => false,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveTime, TimeZone, Utc};

    fn text(value: &str) -> Value {
        Value::String(value.to_string())
    }

    // --- excel_equals ---

    #[test]
    fn test_excel_equals_text_is_case_insensitive() {
        assert!(excel_equals(&text("apple"), &text("APPLE")));
        assert!(excel_equals(&text("Banana"), &text("bAnAnA")));
        assert!(!excel_equals(&text("apple"), &text("apples")));
    }

    #[test]
    fn test_excel_equals_numbers_across_types() {
        assert!(excel_equals(&Value::I32(2), &Value::F64(2.0)));
        assert!(excel_equals(&Value::F64(2.0), &Value::I32(2)));
        assert!(excel_equals(&Value::OptionI32(Some(2)), &Value::F64(2.0)));
        assert!(excel_equals(&Value::I32(2), &Value::OptionF64(Some(2.0))));
        assert!(!excel_equals(&Value::I32(2), &Value::F64(2.5)));
    }

    #[test]
    fn test_excel_equals_bool_is_not_a_number() {
        assert!(excel_equals(&Value::Bool(true), &Value::Bool(true)));
        assert!(excel_equals(
            &Value::Bool(false),
            &Value::OptionBool(Some(false))
        ));
        assert!(!excel_equals(&Value::Bool(true), &Value::I32(1)));
        assert!(!excel_equals(&Value::Bool(true), &Value::F64(1.0)));
        assert!(!excel_equals(&Value::Bool(true), &text("TRUE")));
    }

    #[test]
    fn test_excel_equals_number_is_not_text() {
        assert!(!excel_equals(&Value::I32(2), &text("2")));
        assert!(!excel_equals(&text("2"), &Value::I32(2)));
        assert!(!excel_equals(&Value::F64(2.0), &text("2.0")));
    }

    #[test]
    fn test_excel_equals_unwraps_option_variants() {
        assert!(excel_equals(
            &Value::OptionString(Some("a".to_string())),
            &text("A")
        ));
        assert!(excel_equals(
            &text("a"),
            &Value::OptionString(Some("A".to_string()))
        ));
    }

    #[test]
    fn test_excel_equals_errors_and_blanks() {
        assert!(excel_equals(
            &Value::Error(ExcelError::Na),
            &Value::Error(ExcelError::Na)
        ));
        assert!(!excel_equals(
            &Value::Error(ExcelError::Na),
            &Value::Error(ExcelError::Div0)
        ));
        assert!(!excel_equals(&Value::Error(ExcelError::Na), &Value::I32(1)));
        assert!(!excel_equals(&Value::I32(1), &Value::Error(ExcelError::Na)));
        assert!(excel_equals(&Value::None, &Value::None));
        assert!(!excel_equals(&Value::None, &text("")));
    }

    #[test]
    fn test_excel_equals_datetime_and_time() {
        let date = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let other = Utc.with_ymd_and_hms(2023, 1, 2, 0, 0, 0).unwrap();
        assert!(excel_equals(
            &Value::ChronoDateTime(date),
            &Value::OptionChronoDateTime(Some(date))
        ));
        assert!(!excel_equals(
            &Value::ChronoDateTime(date),
            &Value::ChronoDateTime(other)
        ));

        let noon = NaiveTime::from_hms_opt(12, 0, 0).unwrap();
        assert!(excel_equals(
            &Value::Time(noon),
            &Value::OptionTime(Some(noon))
        ));
    }

    // --- needs_wildcard_matching ---

    #[test]
    fn test_needs_wildcard_matching() {
        assert!(needs_wildcard_matching("a*"));
        assert!(needs_wildcard_matching("a?"));
        assert!(needs_wildcard_matching("~*"));
        assert!(needs_wildcard_matching("a~b"));
        assert!(!needs_wildcard_matching("abc"));
        assert!(!needs_wildcard_matching(""));
    }

    // --- LookupMatcher ---

    #[test]
    fn test_wildcard_matcher_star_and_question() {
        let pattern = text("App*");
        let matcher = LookupMatcher::new(&pattern).unwrap();
        assert!(matcher.matches(&text("Apple")));
        assert!(matcher.matches(&text("App")));
        assert!(!matcher.matches(&text("Banana")));

        let pattern = text("B?nana");
        let matcher = LookupMatcher::new(&pattern).unwrap();
        assert!(matcher.matches(&text("Banana")));
        assert!(matcher.matches(&text("Bonana")));
        assert!(!matcher.matches(&text("Bnana")));
    }

    #[test]
    fn test_wildcard_matcher_escapes() {
        let pattern = text("10~*20");
        let matcher = LookupMatcher::new(&pattern).unwrap();
        assert!(matcher.matches(&text("10*20")));
        assert!(!matcher.matches(&text("103020")));

        let pattern = text("A~?");
        let matcher = LookupMatcher::new(&pattern).unwrap();
        assert!(matcher.matches(&text("A?")));
        assert!(!matcher.matches(&text("AB")));

        let pattern = text("a~~b");
        let matcher = LookupMatcher::new(&pattern).unwrap();
        assert!(matcher.matches(&text("a~b")));
        assert!(!matcher.matches(&text("ab")));
    }

    #[test]
    fn test_wildcard_matcher_anchors_to_the_whole_value() {
        let pattern = text("App?");
        let matcher = LookupMatcher::new(&pattern).unwrap();
        assert!(!matcher.matches(&text("Apple")));
        assert!(matcher.matches(&text("Apps")));
    }

    #[test]
    fn test_wildcard_matcher_is_case_insensitive() {
        let pattern = text("app*");
        let matcher = LookupMatcher::new(&pattern).unwrap();
        assert!(matcher.matches(&text("Apple")));
        assert!(matcher.matches(&text("APPLE")));
    }

    #[test]
    fn test_wildcard_matcher_ignores_non_text_candidates() {
        let pattern = text("1*");
        let matcher = LookupMatcher::new(&pattern).unwrap();
        assert!(!matcher.matches(&Value::I32(123)));
        assert!(!matcher.matches(&Value::F64(1.5)));
        assert!(!matcher.matches(&Value::Bool(true)));
        assert!(!matcher.matches(&Value::None));
        assert!(!matcher.matches(&Value::Error(ExcelError::Na)));
    }

    #[test]
    fn test_wildcard_matcher_escapes_regex_metacharacters() {
        let pattern = text("a.c*");
        let matcher = LookupMatcher::new(&pattern).unwrap();
        assert!(matcher.matches(&text("a.cd")));
        assert!(!matcher.matches(&text("abcd")));

        let pattern = text("a+b?");
        let matcher = LookupMatcher::new(&pattern).unwrap();
        assert!(matcher.matches(&text("a+bc")));
        assert!(!matcher.matches(&text("aab")));
    }

    #[test]
    fn test_lookup_matcher_is_plain_for_non_text_lookup_values() {
        // A number whose text form contains `*` is still a plain lookup.
        let lookup = Value::I32(2);
        let matcher = LookupMatcher::new(&lookup).unwrap();
        assert!(matcher.matches(&Value::F64(2.0)));
        assert!(!matcher.matches(&text("2")));

        // Text with no wildcard character is a plain, case-insensitive lookup.
        let lookup = text("Apple");
        let matcher = LookupMatcher::new(&lookup).unwrap();
        assert!(matcher.matches(&text("APPLE")));
        assert!(!matcher.matches(&text("Apples")));
    }
}
