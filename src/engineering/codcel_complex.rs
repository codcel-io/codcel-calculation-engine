// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::engineering::complex::format_complex;
use std::error::Error;

/// Excel-compatible `COMPLEX` that creates a complex number from real and imaginary coefficients.
/// - `real`: the real coefficient.
/// - `imaginary`: the imaginary coefficient.
/// - `suffix`: optional imaginary unit suffix (`"i"` or `"j"`, defaults to `"i"`).
/// - `decimal_separator`: locale-specific decimal separator.
/// - `use_excel_rounding`: whether to apply Excel-compatible rounding.
///   Returns a string like `3+4i`, or an error when the suffix is invalid.
pub fn codcel_complex(
    real: f64,
    imaginary: f64,
    suffix: Option<String>,
    decimal_separator: &str,
    use_excel_rounding: bool,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    // Validate the suffix
    let suffix = suffix.unwrap_or("i".to_string());
    if suffix != "i" && suffix != "j" {
        return Err("COMPLEX: Invalid suffix. Must be 'i' or 'j'".into());
    }

    format_complex(
        real,
        imaginary,
        suffix.chars().next().unwrap(),
        decimal_separator,
        use_excel_rounding,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_basic() {
        // =COMPLEX(3, 4) in US format
        // =COMPLEX(3; 4) in German format
        let result = codcel_complex(3.0, 4.0, None, ".", false).unwrap();
        println!("{result}");
        assert_eq!(result, "3+4i");
    }

    #[test]
    fn test_complex_with_j_suffix() {
        // =COMPLEX(3, 4, "j") in US format
        // =COMPLEX(3; 4; "j") in German format
        let result = codcel_complex(3.0, 4.0, Some("j".to_string()), ".", false).unwrap();
        println!("{result}");
        assert_eq!(result, "3+4j");
    }

    #[test]
    fn test_complex_zero_imaginary() {
        // =COMPLEX(5, 0) in US format
        // =COMPLEX(5; 0) in German format
        let result = codcel_complex(5.0, 0.0, None, ".", false).unwrap();
        println!("{result}");
        assert_eq!(result, "5");
    }

    #[test]
    fn test_complex_zero_real() {
        // =COMPLEX(0, 7) in US format
        // =COMPLEX(0; 7) in German format
        let result = codcel_complex(0.0, 7.0, None, ".", false).unwrap();
        println!("{result}");
        assert_eq!(result, "7i");
    }

    #[test]
    fn test_complex_negative_imaginary() {
        // =COMPLEX(2, -3) in US format
        // =COMPLEX(2; -3) in German format
        let result = codcel_complex(2.0, -3.0, None, ".", false).unwrap();
        println!("{result}");
        assert_eq!(result, "2-3i");
    }

    #[test]
    fn test_complex_invalid_suffix() {
        // =COMPLEX(2, 3, "k") in US format
        // =COMPLEX(2; 3; "k") in German format
        let result = codcel_complex(2.0, 3.0, Some("k".to_string()), ".", false);
        assert!(result.is_err());
    }
}
