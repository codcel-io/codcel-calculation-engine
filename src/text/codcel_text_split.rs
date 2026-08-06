// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `TEXTSPLIT` that splits text into a 2D array based on delimiters.
/// - `text`: the text to split.
/// - `col_delimiter`: the delimiter for splitting into columns.
/// - `row_delimiter`: optional delimiter for splitting into rows.
/// - `ignore_empty`: optional flag to ignore empty values (default `false`).
/// - `match_mode`: optional matching mode: 0 = trim whitespace (default), 1 = exact match.
/// - `pad_with`: optional value to use for padding uneven rows.
///   Returns a 2D vector of strings representing the split text.
///   Useful for parsing structured text data into a table format.
pub fn codcel_text_split<S: AsRef<str>>(
    text: S,
    col_delimiter: S,
    row_delimiter: Option<S>,
    ignore_empty: Option<bool>,
    match_mode: Option<i32>,
    pad_with: Option<S>,
) -> Result<Vec<Vec<String>>, Box<dyn Error + Send + Sync>> {
    let text = text.as_ref();
    let col_delimiter = col_delimiter.as_ref();
    let row_delimiter = row_delimiter.as_ref().map(|s| s.as_ref());
    let ignore_empty = ignore_empty.unwrap_or(false);
    let match_mode = match_mode.unwrap_or(0);
    let pad_with = pad_with.as_ref().map(|s| s.as_ref().to_string());

    let mut result: Vec<Vec<String>> = vec![];

    // Step 1: Split into rows if a row delimiter is specified
    let rows: Vec<&str> = if let Some(row_delimiter) = row_delimiter {
        if match_mode == 1 {
            text.split(row_delimiter).collect()
        } else {
            text.split(row_delimiter).map(|s| s.trim()).collect()
        }
    } else {
        vec![text]
    };

    // Step 2: Process each row and split into columns
    for row in rows {
        let columns: Vec<String> = row
            .split(col_delimiter)
            .filter(|s| !ignore_empty || !s.is_empty())
            .map(|s| {
                if match_mode == 1 {
                    s.to_string()
                } else {
                    s.trim().to_string()
                }
            })
            .collect();
        result.push(columns);
    }

    // Step 3: Pad uneven rows if `pad_with` is specified
    if let Some(pad_value) = pad_with {
        let max_cols = result.iter().map(|row| row.len()).max().unwrap_or(0);
        for row in result.iter_mut() {
            while row.len() < max_cols {
                row.push(pad_value.clone());
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_split_column_only() {
        // =TEXTSPLIT("apple,banana,cherry", ",") in US format
        // =TEXTSPLIT("apple;banana;cherry"; ";") in German format
        let result = codcel_text_split("apple,banana,cherry", ",", None, None, None, None).unwrap();
        println!("{result:?}");
        assert_eq!(
            result,
            vec![vec![
                "apple".to_string(),
                "banana".to_string(),
                "cherry".to_string()
            ]]
        );
    }

    #[test]
    fn test_text_split_row_and_column() {
        // =TEXTSPLIT("apple,banana,cherry;grape,orange,lemon", ",", ";") in US format
        // =TEXTSPLIT("apple;banana;cherry|grape;orange;lemon"; ";"; "|") in German format
        let result = codcel_text_split(
            "apple,banana,cherry;grape,orange,lemon",
            ",",
            Some(";"),
            None,
            None,
            None,
        )
        .unwrap();
        println!("{result:?}");
        assert_eq!(
            result,
            vec![
                vec![
                    "apple".to_string(),
                    "banana".to_string(),
                    "cherry".to_string()
                ],
                vec![
                    "grape".to_string(),
                    "orange".to_string(),
                    "lemon".to_string()
                ]
            ]
        );
    }

    #[test]
    fn test_text_split_ignore_empty() {
        // =TEXTSPLIT("apple,,cherry", ",", , TRUE) in US format
        // =TEXTSPLIT("apple;;cherry"; ";"; ; TRUE) in German format
        let result = codcel_text_split("apple,,cherry", ",", None, Some(true), None, None).unwrap();
        println!("{result:?}");
        assert_eq!(
            result,
            vec![vec!["apple".to_string(), "cherry".to_string()]]
        );
    }

    #[test]
    fn test_text_split_keep_empty() {
        // =TEXTSPLIT("apple,,cherry", ",", , FALSE) in US format
        // =TEXTSPLIT("apple;;cherry"; ";"; ; FALSE) in German format
        let result =
            codcel_text_split("apple,,cherry", ",", None, Some(false), None, None).unwrap();
        println!("{result:?}");
        assert_eq!(
            result,
            vec![vec![
                "apple".to_string(),
                "".to_string(),
                "cherry".to_string()
            ]]
        );
    }

    #[test]
    fn test_text_split_match_mode_exact() {
        // =TEXTSPLIT(" apple , banana , cherry ", ",", , , 1) in US format
        // =TEXTSPLIT(" apple ; banana ; cherry "; ";"; ; ; 1) in German format
        let result =
            codcel_text_split(" apple , banana , cherry ", ",", None, None, Some(1), None).unwrap();
        println!("{result:?}");
        assert_eq!(
            result,
            vec![vec![
                " apple ".to_string(),
                " banana ".to_string(),
                " cherry ".to_string()
            ]]
        );
    }

    #[test]
    fn test_text_split_pad_with() {
        // =TEXTSPLIT("apple,banana;grape", ",", ";", , , "N/A") in US format
        // =TEXTSPLIT("apple;banana|grape"; ";"; "|"; ; ; "N/A") in German format
        let result = codcel_text_split(
            "apple,banana;grape",
            ",",
            Some(";"),
            None,
            None,
            Some("N/A"),
        )
        .unwrap();
        println!("{result:?}");
        assert_eq!(
            result,
            vec![
                vec!["apple".to_string(), "banana".to_string()],
                vec!["grape".to_string(), "N/A".to_string()]
            ]
        );
    }

    #[test]
    fn test_text_split_empty_text() {
        // =TEXTSPLIT("", ",") in US format
        // =TEXTSPLIT(""; ";") in German format
        let result = codcel_text_split("", ",", None, None, None, None).unwrap();
        println!("{result:?}");
        assert_eq!(result, vec![vec!["".to_string()]]);
    }
}
