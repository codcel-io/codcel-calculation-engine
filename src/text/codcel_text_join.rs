// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `TEXTJOIN` that concatenates a list of text strings using a delimiter.
/// - `delimiter`: the text to insert between each value.
/// - `ignore_empty`: if `true`, empty strings are omitted from the result.
/// - `values`: a vector of text strings to join.
///   Returns all values concatenated with the delimiter between them.
///   Unlike CONCAT/CONCATENATE, this function allows specifying a delimiter.
pub fn codcel_text_join<X: AsRef<str>, S: AsRef<str>>(
    delimiter: X,
    ignore_empty: bool,
    values: Vec<S>,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let result = values
        .into_iter()
        .filter(|s| {
            let str_ref = s.as_ref();
            !(ignore_empty && str_ref.is_empty())
        })
        .map(|s| s.as_ref().to_string())
        .collect::<Vec<String>>()
        .join(delimiter.as_ref());
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_join_basic() {
        // =TEXTJOIN(", ", FALSE, "apple", "banana", "cherry") in US format
        // =TEXTJOIN("; "; FALSE; "apple"; "banana"; "cherry") in German format
        let result = codcel_text_join(", ", false, vec!["apple", "banana", "cherry"]).unwrap();
        println!("{result}");
        assert_eq!(result, "apple, banana, cherry");
    }

    #[test]
    fn test_text_join_with_empty_values_not_ignored() {
        // =TEXTJOIN(", ", FALSE, "apple", "", "cherry") in US format
        // =TEXTJOIN("; "; FALSE; "apple"; ""; "cherry") in German format
        let result = codcel_text_join(", ", false, vec!["apple", "", "cherry"]).unwrap();
        println!("{result}");
        assert_eq!(result, "apple, , cherry");
    }

    #[test]
    fn test_text_join_with_empty_values_ignored() {
        // =TEXTJOIN(", ", TRUE, "apple", "", "cherry") in US format
        // =TEXTJOIN("; "; TRUE; "apple"; ""; "cherry") in German format
        let result = codcel_text_join(", ", true, vec!["apple", "", "cherry"]).unwrap();
        println!("{result}");
        assert_eq!(result, "apple, cherry");
    }

    #[test]
    fn test_text_join_with_empty_delimiter() {
        // =TEXTJOIN("", FALSE, "apple", "banana", "cherry") in US format
        // =TEXTJOIN(""; FALSE; "apple"; "banana"; "cherry") in German format
        let result = codcel_text_join("", false, vec!["apple", "banana", "cherry"]).unwrap();
        println!("{result}");
        assert_eq!(result, "applebananacherry");
    }

    #[test]
    fn test_text_join_single_value() {
        // =TEXTJOIN(", ", FALSE, "apple") in US format
        // =TEXTJOIN("; "; FALSE; "apple") in German format
        let result = codcel_text_join(", ", false, vec!["apple"]).unwrap();
        println!("{result}");
        assert_eq!(result, "apple");
    }

    #[test]
    fn test_text_join_no_values() {
        // =TEXTJOIN(", ", FALSE) in US format
        // =TEXTJOIN("; "; FALSE) in German format
        let result: String = codcel_text_join(", ", false, Vec::<String>::new()).unwrap();
        println!("{result}");
        assert_eq!(result, "");
    }

    #[test]
    fn test_text_join_all_empty_values_ignored() {
        // =TEXTJOIN(", ", TRUE, "", "", "") in US format
        // =TEXTJOIN("; "; TRUE; ""; ""; "") in German format
        let result = codcel_text_join(", ", true, vec!["", "", ""]).unwrap();
        println!("{result}");
        assert_eq!(result, "");
    }

    #[test]
    fn test_text_join_all_empty_values_not_ignored() {
        // =TEXTJOIN(", ", FALSE, "", "", "") in US format
        // =TEXTJOIN("; "; FALSE; ""; ""; "") in German format
        let result = codcel_text_join(", ", false, vec!["", "", ""]).unwrap();
        println!("{result}");
        assert_eq!(result, ", , ");
    }
}
