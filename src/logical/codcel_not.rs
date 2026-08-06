// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `NOT` that reverses the logical value of its argument.
/// - `value`: a boolean value to negate.
///
/// Returns `true` if the input is `false`, and `false` if the input is `true`.
pub fn codcel_not(value: bool) -> Result<bool, Box<dyn Error + Send + Sync>> {
    Ok(!value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_basic_functionality() {
        // Test that true becomes false
        assert!(!codcel_not(true).unwrap());

        // Test that false becomes true
        assert!(codcel_not(false).unwrap());
    }

    #[test]
    fn test_not_result_wrapping() {
        // Test that the result is properly wrapped in a Result type
        let result_true = codcel_not(true);
        let result_false = codcel_not(false);

        // Check that the results are Ok variants
        assert!(result_true.is_ok());
        assert!(result_false.is_ok());

        // Check the unwrapped values
        assert!(!result_true.unwrap());
        assert!(result_false.unwrap());
    }

    #[test]
    fn test_not_double_negation() {
        // Test that applying NOT twice returns the original value
        let original_true = true;
        let original_false = false;

        // Apply NOT twice
        let double_negated_true = codcel_not(codcel_not(original_true).unwrap()).unwrap();
        let double_negated_false = codcel_not(codcel_not(original_false).unwrap()).unwrap();

        // Check that double negation returns the original value
        assert_eq!(double_negated_true, original_true);
        assert_eq!(double_negated_false, original_false);
    }

    #[test]
    fn test_not_with_expressions() {
        // Test NOT with boolean expressions
        assert!(!codcel_not(true && true).unwrap());
        assert!(!codcel_not(true || false).unwrap());
        assert!(codcel_not(false && true).unwrap());
        assert!(codcel_not(false || false).unwrap());
    }

    #[test]
    fn test_not_with_variables() {
        // Test NOT with boolean variables
        let var_true = true;
        let var_false = false;

        assert!(!codcel_not(var_true).unwrap());
        assert!(codcel_not(var_false).unwrap());
    }

    #[test]
    fn test_not_with_complex_expressions() {
        // Test NOT with more complex boolean expressions
        let a = true;
        let b = false;
        let c = true;

        // NOT(a AND b OR c) = NOT(true AND false OR true) = NOT(false OR true) = NOT(true) = false
        let expr_result = codcel_not((a && b) || c).unwrap();
        assert!(!expr_result);

        // NOT(a OR b AND c) = NOT(true OR false AND true) = NOT(true OR false) = NOT(true) = false
        let expr_result2 = codcel_not(a || (b && c)).unwrap();
        assert!(!expr_result2);
    }
}
