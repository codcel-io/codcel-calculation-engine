// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `ROMAN` that converts an Arabic numeral to Roman numeral text.
/// - `input`: an integer between 1 and 3999.
/// - `form`: optional form type (0–4, defaults to 0 for classic).
///
/// Returns the Roman numeral string or an error for out-of-range inputs.
pub fn codcel_roman(input: i32, form: Option<i32>) -> Result<String, Box<dyn Error + Send + Sync>> {
    let form = form.unwrap_or(0);
    if !(1..=3999).contains(&input) {
        return Err(format!("ROMAN: Input out of range (1-3999): {input}").into());
    }
    if !(0..=4).contains(&form) {
        return Err(format!("ROMAN: Form parameter out of range (0-4): {form}").into());
    }

    // Standard Roman numeral conversion for other forms
    if form != 2 {
        let roman_maps: Vec<Vec<(i32, &str)>> = vec![
            // Form 0: Classic
            vec![
                (1000, "M"),
                (900, "CM"),
                (500, "D"),
                (400, "CD"),
                (100, "C"),
                (90, "XC"),
                (50, "L"),
                (40, "XL"),
                (10, "X"),
                (9, "IX"),
                (5, "V"),
                (4, "IV"),
                (1, "I"),
            ],
            // Form 1: More concise
            vec![
                (1000, "M"),
                (900, "CM"),
                (500, "D"),
                (400, "CD"),
                (100, "C"),
                (90, "XC"),
                (50, "L"),
                (40, "XL"),
                (10, "X"),
                (5, "V"),
                (1, "I"),
            ],
            // Form 3: Short
            vec![
                (1000, "M"),
                (900, "CM"),
                (500, "D"),
                (400, "CD"),
                (100, "C"),
                (50, "L"),
                (10, "X"),
                (5, "V"),
                (1, "I"),
            ],
            // Form 4: Simplified
            vec![
                (1000, "M"),
                (500, "D"),
                (100, "C"),
                (50, "L"),
                (10, "X"),
                (5, "V"),
                (1, "I"),
            ],
        ];

        let roman_map = &roman_maps[form as usize - (form > 1) as usize];
        let mut result = String::new();
        let mut remaining = input;

        for &(value, symbol) in roman_map {
            while remaining >= value {
                result.push_str(symbol);
                remaining -= value;
            }
        }

        return Ok(result);
    }

    // Excel Mode 2 specific conversion
    let symbols = [
        (1000, "M"),
        (900, "ML"),
        (500, "D"),
        (400, "LD"),
        (100, "C"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];

    let mut result = String::new();
    let mut remaining = input;

    for &(value, symbol) in &symbols {
        while remaining >= value {
            result.push_str(symbol);
            remaining -= value;
        }
    }

    println!("ROMAN input: {input:}, form: {form}, result: {result}");

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_small_numbers() {
        // =ROMAN(1) in US format
        // =ROMAN(1) in German format
        let result = codcel_roman(1, None).unwrap();
        assert_eq!(result, "I");

        // =ROMAN(4) in US format
        // =ROMAN(4) in German format
        let result = codcel_roman(4, None).unwrap();
        assert_eq!(result, "IV");

        // =ROMAN(9) in US format
        // =ROMAN(9) in German format
        let result = codcel_roman(9, None).unwrap();
        assert_eq!(result, "IX");
    }

    #[test]
    fn test_roman_medium_numbers() {
        // =ROMAN(49) in US format
        // =ROMAN(49) in German format
        let result = codcel_roman(49, None).unwrap();
        assert_eq!(result, "XLIX");

        // =ROMAN(99) in US format
        // =ROMAN(99) in German format
        let result = codcel_roman(99, None).unwrap();
        assert_eq!(result, "XCIX");

        // =ROMAN(499) in US format
        // =ROMAN(499) in German format
        let result = codcel_roman(499, None).unwrap();
        assert_eq!(result, "CDXCIX");
    }

    #[test]
    fn test_roman_large_numbers() {
        // =ROMAN(1999) in US format
        // =ROMAN(1999) in German format
        let result = codcel_roman(1999, None).unwrap();
        assert_eq!(result, "MCMXCIX");

        // =ROMAN(3999) in US format
        // =ROMAN(3999) in German format
        let result = codcel_roman(3999, None).unwrap();
        assert_eq!(result, "MMMCMXCIX");
    }

    #[test]
    fn test_roman_form_0() {
        // =ROMAN(499,0) in US format
        // =ROMAN(499;0) in German format
        let result = codcel_roman(499, Some(0)).unwrap();
        assert_eq!(result, "CDXCIX");
    }

    #[test]
    fn test_roman_form_1() {
        // =ROMAN(499,1) in US format
        // =ROMAN(499;1) in German format
        let result = codcel_roman(499, Some(1)).unwrap();
        assert_eq!(result, "CDXCVIIII"); // More concise form
    }

    #[test]
    fn test_roman_input_out_of_range_low() {
        // =ROMAN(0) in US format (returns #VALUE! error)
        // =ROMAN(0) in German format (returns #VALUE! error)
        let result = codcel_roman(0, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_roman_input_out_of_range_high() {
        // =ROMAN(4000) in US format (returns #VALUE! error)
        // =ROMAN(4000) in German format (returns #VALUE! error)
        let result = codcel_roman(4000, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_roman_form_out_of_range() {
        // =ROMAN(100,5) in US format (returns #VALUE! error)
        // =ROMAN(100;5) in German format (returns #VALUE! error)
        let result = codcel_roman(100, Some(5));
        assert!(result.is_err());
    }
}
