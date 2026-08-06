// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::value::Value;
use std::error::Error;

/// Converts a 1-based column number to an Excel column letter string.
/// 1 → "A", 26 → "Z", 27 → "AA", 702 → "ZZ", 703 → "AAA", etc.
fn column_number_to_letter(mut col: i32) -> String {
    let mut result = String::new();
    while col > 0 {
        col -= 1;
        result.insert(0, (b'A' + (col % 26) as u8) as char);
        col /= 26;
    }
    result
}

/// Excel-compatible `ADDRESS` function.
/// Creates a cell address as text, given row and column numbers.
///
/// # Parameters
/// - `row_num`: the row number (1-based, must be >= 1).
/// - `col_num`: the column number (1-based, must be >= 1).
/// - `abs_num`: the reference type (default 1):
///   - 1 = absolute row and column (`$A$1`)
///   - 2 = absolute row, relative column (`A$1`)
///   - 3 = relative row, absolute column (`$A1`)
///   - 4 = relative row and column (`A1`)
/// - `a1`: reference style (default `true`):
///   - `true` = A1 style (e.g., `$A$1`)
///   - `false` = R1C1 style (e.g., `R1C1`)
/// - `sheet_text`: optional sheet name to prepend (e.g., `"Sheet1"` → `"Sheet1!$A$1"`).
///
/// # Errors
/// Returns an error if `row_num` or `col_num` is less than 1, or `abs_num` is not 1–4.
pub fn codcel_address(
    row_num: i32,
    col_num: i32,
    abs_num: Option<i32>,
    a1: Option<bool>,
    sheet_text: Option<String>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if row_num < 1 {
        return Err("ADDRESS: Row number must be at least 1".into());
    }
    if col_num < 1 {
        return Err("ADDRESS: Column number must be at least 1".into());
    }

    let abs_num = abs_num.unwrap_or(1);
    if !(1..=4).contains(&abs_num) {
        return Err("ADDRESS: abs_num must be between 1 and 4".into());
    }

    let a1 = a1.unwrap_or(true);

    let abs_row = abs_num == 1 || abs_num == 2;
    let abs_col = abs_num == 1 || abs_num == 3;

    let cell_ref = if a1 {
        let col_str = column_number_to_letter(col_num);
        let col_part = if abs_col {
            format!("${col_str}")
        } else {
            col_str
        };
        let row_part = if abs_row {
            format!("${row_num}")
        } else {
            row_num.to_string()
        };
        format!("{col_part}{row_part}")
    } else {
        // R1C1 style
        let row_part = if abs_row {
            format!("R{row_num}")
        } else {
            format!("R[{row_num}]")
        };
        let col_part = if abs_col {
            format!("C{col_num}")
        } else {
            format!("C[{col_num}]")
        };
        format!("{row_part}{col_part}")
    };

    let result = match sheet_text {
        Some(sheet) => {
            if sheet.contains(' ') {
                format!("'{sheet}'!{cell_ref}")
            } else {
                format!("{sheet}!{cell_ref}")
            }
        }
        None => cell_ref,
    };

    Ok(Value::String(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &Value) -> &str {
        match v {
            Value::String(s) => s,
            _ => panic!("expected String"),
        }
    }

    // --- A1 style, absolute (abs_num = 1, default) ---

    #[test]
    fn address_absolute_a1() {
        let result = codcel_address(1, 1, None, None, None).unwrap();
        assert_eq!(s(&result), "$A$1");
    }

    #[test]
    fn address_absolute_a1_explicit() {
        let result = codcel_address(2, 3, Some(1), Some(true), None).unwrap();
        assert_eq!(s(&result), "$C$2");
    }

    // --- A1 style, abs_num = 2 (absolute row, relative column) ---

    #[test]
    fn address_abs_row_rel_col() {
        let result = codcel_address(2, 3, Some(2), Some(true), None).unwrap();
        assert_eq!(s(&result), "C$2");
    }

    // --- A1 style, abs_num = 3 (relative row, absolute column) ---

    #[test]
    fn address_rel_row_abs_col() {
        let result = codcel_address(2, 3, Some(3), Some(true), None).unwrap();
        assert_eq!(s(&result), "$C2");
    }

    // --- A1 style, abs_num = 4 (relative) ---

    #[test]
    fn address_relative_a1() {
        let result = codcel_address(2, 3, Some(4), Some(true), None).unwrap();
        assert_eq!(s(&result), "C2");
    }

    // --- R1C1 style ---

    #[test]
    fn address_r1c1_absolute() {
        let result = codcel_address(2, 3, Some(1), Some(false), None).unwrap();
        assert_eq!(s(&result), "R2C3");
    }

    #[test]
    fn address_r1c1_abs_row_rel_col() {
        let result = codcel_address(2, 3, Some(2), Some(false), None).unwrap();
        assert_eq!(s(&result), "R2C[3]");
    }

    #[test]
    fn address_r1c1_rel_row_abs_col() {
        let result = codcel_address(2, 3, Some(3), Some(false), None).unwrap();
        assert_eq!(s(&result), "R[2]C3");
    }

    #[test]
    fn address_r1c1_relative() {
        let result = codcel_address(2, 3, Some(4), Some(false), None).unwrap();
        assert_eq!(s(&result), "R[2]C[3]");
    }

    // --- Sheet name ---

    #[test]
    fn address_with_sheet() {
        let result = codcel_address(1, 1, Some(1), Some(true), Some("Sheet1".to_string())).unwrap();
        assert_eq!(s(&result), "Sheet1!$A$1");
    }

    #[test]
    fn address_with_sheet_spaces() {
        let result = codcel_address(1, 1, Some(1), Some(true), Some("My Sheet".to_string())).unwrap();
        assert_eq!(s(&result), "'My Sheet'!$A$1");
    }

    // --- Column letter edge cases ---

    #[test]
    fn address_column_z() {
        let result = codcel_address(1, 26, None, None, None).unwrap();
        assert_eq!(s(&result), "$Z$1");
    }

    #[test]
    fn address_column_aa() {
        let result = codcel_address(1, 27, None, None, None).unwrap();
        assert_eq!(s(&result), "$AA$1");
    }

    #[test]
    fn address_column_az() {
        let result = codcel_address(1, 52, None, None, None).unwrap();
        assert_eq!(s(&result), "$AZ$1");
    }

    #[test]
    fn address_column_ba() {
        let result = codcel_address(1, 53, None, None, None).unwrap();
        assert_eq!(s(&result), "$BA$1");
    }

    #[test]
    fn address_column_aaa() {
        let result = codcel_address(1, 703, None, None, None).unwrap();
        assert_eq!(s(&result), "$AAA$1");
    }

    // --- Error cases ---

    #[test]
    fn address_invalid_row() {
        let result = codcel_address(0, 1, None, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn address_invalid_col() {
        let result = codcel_address(1, 0, None, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn address_invalid_abs_num() {
        let result = codcel_address(1, 1, Some(5), None, None);
        assert!(result.is_err());
    }

    #[test]
    fn address_invalid_abs_num_zero() {
        let result = codcel_address(1, 1, Some(0), None, None);
        assert!(result.is_err());
    }

    // --- Column number to letter helper ---

    #[test]
    fn col_to_letter_a() {
        assert_eq!(column_number_to_letter(1), "A");
    }

    #[test]
    fn col_to_letter_z() {
        assert_eq!(column_number_to_letter(26), "Z");
    }

    #[test]
    fn col_to_letter_aa() {
        assert_eq!(column_number_to_letter(27), "AA");
    }

    #[test]
    fn col_to_letter_xfd() {
        // Excel's max column is XFD = 16384
        assert_eq!(column_number_to_letter(16384), "XFD");
    }
}
