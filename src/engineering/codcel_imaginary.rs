// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `IMAGINARY` that returns the imaginary coefficient of a complex number.
/// - `complex`: a complex number string in the form `x+yi` or `x+yj`.
///   Returns the imaginary coefficient (e.g., `4` for `"3+4i"`), `0` for purely real inputs, or an error for invalid formats.
pub fn codcel_imaginary(complex: String) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Remove all whitespace from the input
    let complex = complex.replace(" ", "");

    // For a purely real number, return 0
    if !complex.contains('i') && !complex.contains('j') {
        if complex.parse::<f64>().is_ok() {
            return Ok(0.0);
        }
        return Err("IMAGINARY: Invalid real number format".into());
    }

    // Handle case where the input is just "i" or "-i"
    if complex.trim() == "i" || complex.trim() == "j" {
        return Ok(1.0);
    } else if complex.trim() == "-i" || complex.trim() == "-j" {
        return Ok(-1.0);
    }

    // Handle purely imaginary numbers with a coefficient (like "4i" or "-4i")
    if !complex.contains('+') && !complex.contains('-') {
        // If it's just a number followed by i/j (e.g., "4i")
        let num_part = complex.replace("i", "").replace("j", "");
        if let Ok(num) = num_part.parse::<f64>() {
            return Ok(num);
        }
    } else if complex.starts_with('-') && !complex[1..].contains('+') && !complex[1..].contains('-')
    {
        // If it's a negative number followed by i/j (e.g., "-4i")
        let num_part = complex.replace("i", "").replace("j", "");
        if let Ok(num) = num_part.parse::<f64>() {
            return Ok(num);
        }
    }

    // Remove 'i' or 'j' for parsing
    let complex = complex.replace("i", "").replace("j", "");

    // Handle the imaginary part
    if complex.contains('+') {
        // Split by plus if it exists
        let parts: Vec<&str> = complex.split('+').collect();
        if parts.len() != 2 {
            return Err("IMAGINARY: Invalid complex number format".into());
        }
        // Return the imaginary part
        Ok(if parts[1].is_empty() {
            1.0
        } else {
            parts[1].parse::<f64>()?
        })
    } else {
        // Handle cases with negative numbers
        let parts: Vec<&str> = complex.split('-').collect();
        match parts.len() {
            // Single negative number
            2 => {
                if parts[0].is_empty() {
                    // Case: -a-bi
                    if parts[1].is_empty() {
                        Ok(-1.0)
                    } else {
                        Ok(0.0) // -a+0i
                    }
                } else {
                    // Case: a-bi
                    Ok(if parts[1].is_empty() {
                        -1.0
                    } else {
                        -parts[1].parse::<f64>()?
                    })
                }
            }
            // Two negative numbers (-a-bi)
            3 => {
                if parts[0].is_empty() {
                    Ok(-parts[2].parse::<f64>()?)
                } else {
                    Err("IMAGINARY: Invalid complex number format".into())
                }
            }
            _ => Err("IMAGINARY: Invalid complex number format".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_imaginary_complex_number() {
        // =IMAGINARY("3+4i") in US format
        // =IMAGINARY("3+4i") in German format
        let result = codcel_imaginary("3+4i".to_string()).unwrap();
        println!("{result}");
        assert!((result - 4.0).abs() < 0.0001);
    }

    #[test]
    fn test_imaginary_purely_real() {
        // =IMAGINARY("5") in US format
        // =IMAGINARY("5") in German format
        let result = codcel_imaginary("5".to_string()).unwrap();
        println!("{result}");
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_imaginary_purely_imaginary_i() {
        // =IMAGINARY("i") in US format
        // =IMAGINARY("i") in German format
        let result = codcel_imaginary("i".to_string()).unwrap();
        println!("{result}");
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_imaginary_purely_imaginary_with_value() {
        // =IMAGINARY("4i") in US format
        // =IMAGINARY("4i") in German format
        let result = codcel_imaginary("4i".to_string()).unwrap();
        println!("{result}");
        assert!((result - 4.0).abs() < 0.0001);
    }

    #[test]
    fn test_imaginary_negative_imaginary() {
        // =IMAGINARY("-4i") in US format
        // =IMAGINARY("-4i") in German format
        let result = codcel_imaginary("-4i".to_string()).unwrap();
        println!("{result}");
        assert_eq!(result, -4.0);
    }

    #[test]
    fn test_imaginary_negative_real() {
        // =IMAGINARY("-3+4i") in US format
        // =IMAGINARY("-3+4i") in German format
        let result = codcel_imaginary("-3+4i".to_string()).unwrap();
        println!("{result}");
        assert!((result - 4.0).abs() < 0.0001);
    }

    #[test]
    fn test_imaginary_negative_imaginary_in_complex() {
        // =IMAGINARY("3-4i") in US format
        // =IMAGINARY("3-4i") in German format
        let result = codcel_imaginary("3-4i".to_string()).unwrap();
        println!("{result}");
        assert!((result + 4.0).abs() < 0.0001);
    }

    #[test]
    fn test_imaginary_j_symbol() {
        // =IMAGINARY("3+4j") in US format
        // =IMAGINARY("3+4j") in German format
        let result = codcel_imaginary("3+4j".to_string()).unwrap();
        println!("{result}");
        assert!((result - 4.0).abs() < 0.0001);
    }

    #[test]
    fn test_imaginary_invalid_input() {
        // =IMAGINARY("not_a_complex_number") in US format
        // =IMAGINARY("not_a_complex_number") in German format
        let result = codcel_imaginary("not_a_complex_number".to_string());
        assert!(result.is_err());
    }
}
