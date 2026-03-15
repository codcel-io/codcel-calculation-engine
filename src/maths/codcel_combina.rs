// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `COMBINA` that returns the number of combinations with repetitions.
/// - `number`: the total number of items (n).
/// - `number_chosen`: the number of items to choose (k).
///
/// Returns (n+k-1)! / (k! × (n-1)!) or an error for invalid inputs.
pub fn codcel_combina(
    number: i32,
    number_chosen: i32,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    let number = number as u32;
    let number_chosen = number_chosen as u32;

    // Excel returns 1 when both values are 0
    if number == 0 && number_chosen == 0 {
        return Ok(1);
    }

    if number == 0 && number_chosen > 0 {
        return Err(
            "COMBINA: For combinations with repetition, n must be greater than 0 if k > 0".into(),
        );
    }

    // Helper function to calculate factorial
    fn factorial(num: u32) -> u64 {
        (1..=num as u64).product()
    }

    // Use the formula: (n + k - 1)! / (k! * (n - 1)!)
    let numerator = factorial(number + number_chosen - 1);
    let denominator = factorial(number_chosen) * factorial(number - 1);

    Ok((numerator / denominator) as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combina_basic() {
        // =COMBINA(5, 2) in US format
        // =COMBINA(5; 2) in German format
        let result = codcel_combina(5, 2).unwrap();
        assert_eq!(result, 15);
    }

    #[test]
    fn test_combina_all_items() {
        // =COMBINA(5, 5) in US format
        // =COMBINA(5; 5) in German format
        let result = codcel_combina(5, 5).unwrap();
        assert_eq!(result, 126);
    }

    #[test]
    fn test_combina_no_items() {
        // =COMBINA(5, 0) in US format
        // =COMBINA(5; 0) in German format
        let result = codcel_combina(5, 0).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_combina_one_item() {
        // =COMBINA(5, 1) in US format
        // =COMBINA(5; 1) in German format
        let result = codcel_combina(5, 1).unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn test_combina_larger_numbers() {
        // =COMBINA(10, 3) in US format
        // =COMBINA(10; 3) in German format
        let result = codcel_combina(10, 3).unwrap();
        assert_eq!(result, 220);
    }

    #[test]
    fn test_combina_zero_number() {
        // =COMBINA(0, 0) in US format
        // =COMBINA(0; 0) in German format
        let result = codcel_combina(0, 0).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_combina_invalid_input() {
        // =COMBINA(0, 5) in US format - should return an error
        // =COMBINA(0; 5) in German format - should return an error
        let result = codcel_combina(0, 5);
        assert!(result.is_err());
    }

    #[test]
    fn test_combina_special_case() {
        // =COMBINA(3, 2) in US format
        // =COMBINA(3; 2) in German format
        let result = codcel_combina(3, 2).unwrap();
        assert_eq!(result, 6);
    }
}
