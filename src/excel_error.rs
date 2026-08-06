// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::value::Value;
use serde::{Deserialize, Serialize};
use std::error::Error;

/// Typed Excel error value. Excel's `ERROR.TYPE` returns a number per variant.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExcelError {
    Null, // #NULL!  -> 1
    Div0, // #DIV/0! -> 2
    Value, // #VALUE! -> 3
    Ref,  // #REF!   -> 4
    Name, // #NAME?  -> 5
    Num,  // #NUM!   -> 6
    Na,   // #N/A    -> 7
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

pub fn is_error(value: &Value) -> bool {
    error_type(value).is_some()
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
