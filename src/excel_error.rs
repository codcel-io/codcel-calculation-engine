// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::locale::Locale;
use crate::value::Value;
use serde::{Deserialize, Serialize};
use std::error::Error;

/// Typed Excel error value. Excel's `ERROR.TYPE` returns a number per variant.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExcelError {
    Null,  // #NULL!  -> 1
    Div0,  // #DIV/0! -> 2
    Value, // #VALUE! -> 3
    Ref,   // #REF!   -> 4
    Name,  // #NAME?  -> 5
    Num,   // #NUM!   -> 6
    Na,    // #N/A    -> 7
}

impl ExcelError {
    pub fn to_type_code(self) -> f64 {
        match self {
            ExcelError::Null => 1.0,
            ExcelError::Div0 => 2.0,
            ExcelError::Value => 3.0,
            ExcelError::Ref => 4.0,
            ExcelError::Name => 5.0,
            ExcelError::Num => 6.0,
            ExcelError::Na => 7.0,
        }
    }

    /// The error value in English.
    ///
    /// Deliberately not localized. This is the form that goes on the wire, that
    /// [`ExcelError::from_legacy_string`] round-trips, and that generated code
    /// compares against; changing it with the caller's language would break all
    /// three. Use [`ExcelError::display_localized`] to show an error to a
    /// person.
    pub fn display(&self) -> &'static str {
        match self {
            ExcelError::Null => "#NULL!",
            ExcelError::Div0 => "#DIV/0!",
            ExcelError::Value => "#VALUE!",
            ExcelError::Ref => "#REF!",
            ExcelError::Name => "#NAME?",
            ExcelError::Num => "#NUM!",
            ExcelError::Na => "#N/A",
        }
    }

    /// The error value as the given locale's Excel writes it — `#WERT!` in
    /// German, `#VALEUR!` in French.
    ///
    /// Excel translates its error values along with its interface, so a German
    /// user reading `#VALUE!` is reading something their spreadsheet would
    /// never show them. This is a presentation concern only: the wire format
    /// stays the typed [`ExcelError`], and localization happens at the display
    /// boundary.
    ///
    /// Languages Codcel has no table for fall back to the English values.
    pub fn display_localized(&self, locale: &'static Locale) -> &'static str {
        let e = &locale.errors;
        match self {
            ExcelError::Null => e.null,
            ExcelError::Div0 => e.div0,
            ExcelError::Value => e.value,
            ExcelError::Ref => e.r#ref,
            ExcelError::Name => e.name,
            ExcelError::Num => e.num,
            ExcelError::Na => e.na,
        }
    }

    pub fn from_legacy_string(s: &str) -> Option<Self> {
        match s {
            "#NULL!" => Some(ExcelError::Null),
            "#DIV/0!" => Some(ExcelError::Div0),
            "#VALUE!" => Some(ExcelError::Value),
            "#REF!" => Some(ExcelError::Ref),
            "#NAME?" => Some(ExcelError::Name),
            "#NUM!" => Some(ExcelError::Num),
            "#N/A" => Some(ExcelError::Na),
            _ => None,
        }
    }
}

pub fn err_to_box(e: ExcelError) -> Box<dyn Error + Send + Sync> {
    format!("{} (Excel error)", e.display()).into()
}

/// [`err_to_box`] with the error value and the suffix in the caller's language.
///
/// For a transport that already knows the reader's locale — a server handling
/// an `Accept-Language` header, say — so the message it returns is one the
/// reader's own Excel would have shown them.
pub fn err_to_box_localized(
    e: ExcelError,
    locale: &'static Locale,
) -> Box<dyn Error + Send + Sync> {
    format!("{} ({})", e.display_localized(locale), locale.errors.suffix).into()
}

/// Returns the specific Excel error kind for `value`, if any.
///
/// Recognizes three legacy error representations to keep the engine
/// migrating cleanly:
/// - `Value::Error(e)` — the typed variant going forward.
/// - `Value::F64`/`OptionF64` carrying `NaN` — generic legacy error, treated as `#N/A`.
/// - `Value::String("#…!")` — string sentinel used by some database functions.
pub fn error_type(value: &Value) -> Option<ExcelError> {
    match value {
        Value::Error(e) => Some(*e),
        Value::F64(v) if v.is_nan() => Some(ExcelError::Na),
        Value::OptionF64(Some(v)) if v.is_nan() => Some(ExcelError::Na),
        Value::String(s) => ExcelError::from_legacy_string(s),
        Value::OptionString(Some(s)) => ExcelError::from_legacy_string(s),
        _ => None,
    }
}

/// Maps a failed coercion of `value` onto the Excel error Excel itself would raise.
///
/// An in-band `Value::Error` propagates unchanged; anything else that will not coerce
/// becomes `#VALUE!`. Use this instead of `.expect(...)` when a coercion sits somewhere
/// `?` cannot reach, so a bad cell yields an Excel error rather than aborting the process.
pub fn coercion_error(value: &Value) -> Box<dyn Error + Send + Sync> {
    err_to_box(error_type(value).unwrap_or(ExcelError::Value))
}

pub fn is_error(value: &Value) -> bool {
    error_type(value).is_some()
}

#[cfg(test)]
mod tests_localized {
    use super::*;
    use crate::locale;

    /// The English form is the wire format and must stay put: generated code
    /// compares against it and `from_legacy_string` parses it back.
    #[test]
    fn display_stays_english_and_round_trips() {
        for e in [
            ExcelError::Null,
            ExcelError::Div0,
            ExcelError::Value,
            ExcelError::Ref,
            ExcelError::Name,
            ExcelError::Num,
            ExcelError::Na,
        ] {
            assert_eq!(ExcelError::from_legacy_string(e.display()), Some(e));
        }
        assert_eq!(ExcelError::Value.display(), "#VALUE!");
        assert_eq!(
            ExcelError::Value.display_localized(locale::english()),
            "#VALUE!"
        );
    }

    #[cfg(feature = "locale-data")]
    #[test]
    fn errors_are_localized_for_display() {
        assert_eq!(
            ExcelError::Value.display_localized(locale::lookup("de")),
            "#WERT!"
        );
        assert_eq!(
            ExcelError::Ref.display_localized(locale::lookup("de")),
            "#BEZUG!"
        );
        assert_eq!(
            ExcelError::Na.display_localized(locale::lookup("fr")),
            "#N/A"
        );
        assert_eq!(
            ExcelError::Num.display_localized(locale::lookup("pt")),
            "#NÚM!"
        );
    }

    /// A language with no table of its own reads the English values rather than
    /// showing a blank.
    #[cfg(feature = "locale-data")]
    #[test]
    fn an_untranslated_language_falls_back_to_english() {
        assert_eq!(
            ExcelError::Value.display_localized(locale::lookup("th")),
            "#VALUE!"
        );
    }

    #[cfg(feature = "locale-data")]
    #[test]
    fn boxed_errors_carry_the_localized_text() {
        let boxed = err_to_box_localized(ExcelError::Value, locale::lookup("de"));
        assert_eq!(boxed.to_string(), "#WERT! (Excel-Fehler)");
        assert_eq!(
            err_to_box(ExcelError::Value).to_string(),
            "#VALUE! (Excel error)"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_type_code() {
        assert_eq!(ExcelError::Null.to_type_code(), 1.0);
        assert_eq!(ExcelError::Div0.to_type_code(), 2.0);
        assert_eq!(ExcelError::Value.to_type_code(), 3.0);
        assert_eq!(ExcelError::Ref.to_type_code(), 4.0);
        assert_eq!(ExcelError::Name.to_type_code(), 5.0);
        assert_eq!(ExcelError::Num.to_type_code(), 6.0);
        assert_eq!(ExcelError::Na.to_type_code(), 7.0);
    }

    #[test]
    fn test_display_roundtrips_via_from_legacy_string() {
        for e in [
            ExcelError::Null,
            ExcelError::Div0,
            ExcelError::Value,
            ExcelError::Ref,
            ExcelError::Name,
            ExcelError::Num,
            ExcelError::Na,
        ] {
            assert_eq!(ExcelError::from_legacy_string(e.display()), Some(e));
        }
    }

    #[test]
    fn test_from_legacy_string_unknown() {
        assert_eq!(ExcelError::from_legacy_string("hello"), None);
        assert_eq!(ExcelError::from_legacy_string(""), None);
        assert_eq!(ExcelError::from_legacy_string("#WHATEVER!"), None);
    }

    #[test]
    fn test_error_type_typed_variant() {
        assert_eq!(
            error_type(&Value::Error(ExcelError::Div0)),
            Some(ExcelError::Div0)
        );
    }

    #[test]
    fn test_error_type_legacy_nan() {
        assert_eq!(error_type(&Value::F64(f64::NAN)), Some(ExcelError::Na));
        assert_eq!(
            error_type(&Value::OptionF64(Some(f64::NAN))),
            Some(ExcelError::Na)
        );
    }

    #[test]
    fn test_error_type_legacy_string() {
        assert_eq!(
            error_type(&Value::String("#NUM!".to_string())),
            Some(ExcelError::Num)
        );
        assert_eq!(
            error_type(&Value::OptionString(Some("#DIV/0!".to_string()))),
            Some(ExcelError::Div0)
        );
    }

    #[test]
    fn test_error_type_non_error_values() {
        assert_eq!(error_type(&Value::F64(1.0)), None);
        assert_eq!(error_type(&Value::I32(0)), None);
        assert_eq!(error_type(&Value::Bool(true)), None);
        assert_eq!(error_type(&Value::String("hello".to_string())), None);
        assert_eq!(error_type(&Value::None), None);
    }

    #[test]
    fn test_is_error() {
        assert!(is_error(&Value::Error(ExcelError::Ref)));
        assert!(is_error(&Value::F64(f64::NAN)));
        assert!(is_error(&Value::String("#REF!".to_string())));
        assert!(!is_error(&Value::F64(1.0)));
        assert!(!is_error(&Value::None));
    }
}
