// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `ARRAYTOTEXT` that converts an array of values to a text string.
/// - `array`: a 2D array of string values to convert.
/// - `format`: optional format flag (default `false`).
///   - `false` (compact): returns values separated by `"; "` (e.g., `"A; B; C; D"`).
///   - `true` (strict): returns values in array literal format with rows separated by `;`
///     and columns by `\` (e.g., `"{A\B;C\D}"`).
///
///   Returns the formatted text representation of the array.
pub fn codcel_array_to_text(
    array: Vec<Vec<String>>,
    format: Option<bool>,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let format = format.unwrap_or(false);
    if format {
        // Strict format: {1\2\3;4\5\6}
        let mut rows = Vec::new();
        for row in array {
            let row_text = row.join("\\");
            rows.push(row_text);
        }
        Ok(format!("{{{}}}", rows.join(";")))
    } else {
        // Compact format: 1; 2; 3; 4; 5; 6
        let flattened = array
            .into_iter()
            .flat_map(|row| row.into_iter())
            .map(|cell| {
                if cell.contains([';', ' ', '\n', '"']) {
                    format!("\"{}\"", cell.replace('"', "\"\""))
                } else {
                    cell
                }
            })
            .collect::<Vec<_>>();

        Ok(flattened.join("; "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_to_text_compact_format() {
        // =ARRAYTOTEXT(A1:B2, FALSE) in US format
        // =ARRAYTOTEXT(A1:B2; FALSE) in German format
        let array = vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["C".to_string(), "D".to_string()],
        ];
        let result = codcel_array_to_text(array, Some(false)).unwrap();
        println!("{result}");
        assert_eq!(result, "A; B; C; D");
    }

    #[test]
    fn test_array_to_text_strict_format() {
        // =ARRAYTOTEXT(A1:B2, TRUE) in US format
        // =ARRAYTOTEXT(A1:B2; TRUE) in German format
        let array = vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["C".to_string(), "D".to_string()],
        ];
        let result = codcel_array_to_text(array, Some(true)).unwrap();
        println!("{result}");
        assert_eq!(result, "{A\\B;C\\D}");
    }

    #[test]
    fn test_array_to_text_default_format() {
        // =ARRAYTOTEXT(A1:B2) in US format
        // =ARRAYTOTEXT(A1:B2) in German format
        let array = vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["C".to_string(), "D".to_string()],
        ];
        let result = codcel_array_to_text(array, None).unwrap();
        println!("{result}");
        assert_eq!(result, "A; B; C; D");
    }

    #[test]
    fn test_array_to_text_with_spaces() {
        // =ARRAYTOTEXT(A1:B2, FALSE) in US format
        // =ARRAYTOTEXT(A1:B2; FALSE) in German format
        let array = vec![
            vec!["Hello World".to_string(), "B".to_string()],
            vec!["C".to_string(), "D".to_string()],
        ];
        let result = codcel_array_to_text(array, Some(false)).unwrap();
        println!("{result}");
        assert_eq!(result, "\"Hello World\"; B; C; D");
    }

    #[test]
    fn test_array_to_text_with_semicolons() {
        // =ARRAYTOTEXT(A1:B2, FALSE) in US format
        // =ARRAYTOTEXT(A1:B2; FALSE) in German format
        let array = vec![
            vec!["A;B".to_string(), "C".to_string()],
            vec!["D".to_string(), "E".to_string()],
        ];
        let result = codcel_array_to_text(array, Some(false)).unwrap();
        println!("{result}");
        assert_eq!(result, "\"A;B\"; C; D; E");
    }
}
