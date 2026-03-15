// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::collections::HashMap;
use std::error::Error;

/// Excel-compatible `ARABIC` that converts a Roman numeral text to an Arabic numeral.
/// - `input`: a text string representing a Roman numeral (e.g., "MCMXCIV").
///
/// Returns the numeric value or an error for invalid Roman numeral characters.
pub fn codcel_arabic(input: &str) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let roman_to_arabic: HashMap<char, i32> = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);

    let input = input.to_uppercase(); // Ensure case-insensitivity

    // Validate input
    if input.is_empty() || !input.chars().all(|c| roman_to_arabic.contains_key(&c)) {
        return Err(format!("ARABIC: Invalid Roman numeral: {input}").into());
    }

    let mut total = 0;
    let mut previous_value = 0;

    for c in input.chars().rev() {
        let value = *roman_to_arabic.get(&c).ok_or("Invalid Roman numeral")?;
        if value < previous_value {
            total -= value; // Subtract if a smaller numeral precedes a larger numeral
        } else {
            total += value;
        }
        previous_value = value;
    }

    if total <= 0 {
        return Err(format!("ARABIC: Invalid Roman numeral value: {input}").into());
    }

    Ok(total as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arabic_single_numeral() {
        // =ARABIC("I") in US format
        // =ARABIC("I") in German format
        let result = codcel_arabic("I").unwrap();
        assert_eq!(result, 1.0);

        // =ARABIC("V") in US format
        // =ARABIC("V") in German format
        let result = codcel_arabic("V").unwrap();
        assert_eq!(result, 5.0);

        // =ARABIC("X") in US format
        // =ARABIC("X") in German format
        let result = codcel_arabic("X").unwrap();
        assert_eq!(result, 10.0);

        // =ARABIC("L") in US format
        // =ARABIC("L") in German format
        let result = codcel_arabic("L").unwrap();
        assert_eq!(result, 50.0);

        // =ARABIC("C") in US format
        // =ARABIC("C") in German format
        let result = codcel_arabic("C").unwrap();
        assert_eq!(result, 100.0);

        // =ARABIC("D") in US format
        // =ARABIC("D") in German format
        let result = codcel_arabic("D").unwrap();
        assert_eq!(result, 500.0);

        // =ARABIC("M") in US format
        // =ARABIC("M") in German format
        let result = codcel_arabic("M").unwrap();
        assert_eq!(result, 1000.0);
    }

    #[test]
    fn test_arabic_simple_numerals() {
        // =ARABIC("III") in US format
        // =ARABIC("III") in German format
        let result = codcel_arabic("III").unwrap();
        assert_eq!(result, 3.0);

        // =ARABIC("VIII") in US format
        // =ARABIC("VIII") in German format
        let result = codcel_arabic("VIII").unwrap();
        assert_eq!(result, 8.0);

        // =ARABIC("XII") in US format
        // =ARABIC("XII") in German format
        let result = codcel_arabic("XII").unwrap();
        assert_eq!(result, 12.0);
    }

    #[test]
    fn test_arabic_subtractive_notation() {
        // =ARABIC("IV") in US format
        // =ARABIC("IV") in German format
        let result = codcel_arabic("IV").unwrap();
        assert_eq!(result, 4.0);

        // =ARABIC("IX") in US format
        // =ARABIC("IX") in German format
        let result = codcel_arabic("IX").unwrap();
        assert_eq!(result, 9.0);

        // =ARABIC("XL") in US format
        // =ARABIC("XL") in German format
        let result = codcel_arabic("XL").unwrap();
        assert_eq!(result, 40.0);

        // =ARABIC("XC") in US format
        // =ARABIC("XC") in German format
        let result = codcel_arabic("XC").unwrap();
        assert_eq!(result, 90.0);

        // =ARABIC("CD") in US format
        // =ARABIC("CD") in German format
        let result = codcel_arabic("CD").unwrap();
        assert_eq!(result, 400.0);

        // =ARABIC("CM") in US format
        // =ARABIC("CM") in German format
        let result = codcel_arabic("CM").unwrap();
        assert_eq!(result, 900.0);
    }

    #[test]
    fn test_arabic_complex_numerals() {
        // =ARABIC("MCMXCIV") in US format
        // =ARABIC("MCMXCIV") in German format
        let result = codcel_arabic("MCMXCIV").unwrap();
        assert_eq!(result, 1994.0);

        // =ARABIC("MMXXI") in US format
        // =ARABIC("MMXXI") in German format
        let result = codcel_arabic("MMXXI").unwrap();
        assert_eq!(result, 2021.0);

        // =ARABIC("MMXXIII") in US format
        // =ARABIC("MMXXIII") in German format
        let result = codcel_arabic("MMXXIII").unwrap();
        assert_eq!(result, 2023.0);
    }

    #[test]
    fn test_arabic_case_insensitivity() {
        // =ARABIC("mcmxciv") in US format
        // =ARABIC("mcmxciv") in German format
        let result = codcel_arabic("mcmxciv").unwrap();
        assert_eq!(result, 1994.0);

        // =ARABIC("mMxXi") in US format
        // =ARABIC("mMxXi") in German format
        let result = codcel_arabic("mMxXi").unwrap();
        assert_eq!(result, 2021.0);
    }

    #[test]
    fn test_arabic_invalid_input() {
        // =ARABIC("") in US format - should return an error
        // =ARABIC("") in German format - should return an error
        let result = codcel_arabic("");
        assert!(result.is_err());

        // =ARABIC("ABC") in US format - should return an error
        // =ARABIC("ABC") in German format - should return an error
        let result = codcel_arabic("ABC");
        assert!(result.is_err());

        // =ARABIC("123") in US format - should return an error
        // =ARABIC("123") in German format - should return an error
        let result = codcel_arabic("123");
        assert!(result.is_err());
    }
}
