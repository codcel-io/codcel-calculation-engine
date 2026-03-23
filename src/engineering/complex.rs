// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Parses a complex number string into its real and imaginary parts
pub(crate) fn parse_complex(complex: &str) -> Result<(f64, f64), Box<dyn Error + Send + Sync>> {
    let complex = complex.trim();

    // Handle purely real numbers (no i or j)
    if !complex.contains('i') && !complex.contains('j') {
        if let Ok(num) = complex.parse::<f64>() {
            return Ok((num, 0.0));
        }
        return Err("Invalid complex number format".into());
    }

    // Remove the imaginary symbol
    let s = complex.replace(['i', 'j'], "");
    let s = s.trim();

    // Handle special cases: "i", "+i", "-i"
    if s.is_empty() || s == "+" {
        return Ok((0.0, 1.0));
    }
    if s == "-" {
        return Ok((0.0, -1.0));
    }

    // Find the separator between real and imaginary parts.
    // Skip the first character (might be a sign prefix for the real part)
    // and skip exponent signs ('+'/'-' after 'E'/'e').
    let separator_pos = {
        let bytes = s.as_bytes();
        let mut found = None;
        for i in 1..bytes.len() {
            if (bytes[i] == b'+' || bytes[i] == b'-')
                && bytes[i - 1] != b'E'
                && bytes[i - 1] != b'e'
            {
                found = Some(i);
                break;
            }
        }
        found
    };

    match separator_pos {
        None => {
            // Purely imaginary: "5", "-5" (from "5i", "-5i")
            Ok((0.0, s.parse::<f64>()?))
        }
        Some(pos) => {
            let real_str = &s[..pos];
            let imag_str = &s[pos..];

            let real = real_str.parse::<f64>()?;
            let imag = if imag_str == "+" {
                1.0
            } else if imag_str == "-" {
                -1.0
            } else {
                imag_str.parse::<f64>()?
            };

            Ok((real, imag))
        }
    }
}

// TODO: implement the comma separator properly from value_format
/// Formats a complex number from its real and imaginary parts
pub(crate) fn format_complex(
    real: f64,
    imag: f64,
    im_symbol: char,
    decimal_separator: &str,
    use_excel_rounding: bool,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    const EPSILON: f64 = 1e-10;

    // Handle special cases where parts are effectively zero
    let real = if real.abs() < EPSILON { 0.0 } else { real };
    let imag = if imag.abs() < EPSILON { 0.0 } else { imag };

    if imag == 0.0 {
        return Ok(number_to_string(
            real,
            decimal_separator,
            use_excel_rounding,
        ));
    }

    if real == 0.0 {
        if (imag - 1.0).abs() < EPSILON {
            return Ok(format!("{im_symbol}"));
        }
        if (imag + 1.0).abs() < EPSILON {
            return Ok(format!("-{im_symbol}"));
        }
        return Ok(format!(
            "{}{}",
            number_to_string(imag, decimal_separator, use_excel_rounding),
            im_symbol
        ));
    }

    if (imag - 1.0).abs() < EPSILON {
        Ok(format!(
            "{}+{}",
            number_to_string(real, decimal_separator, use_excel_rounding),
            im_symbol
        ))
    } else if (imag + 1.0).abs() < EPSILON {
        Ok(format!(
            "{}-{}",
            number_to_string(real, decimal_separator, use_excel_rounding),
            im_symbol
        ))
    } else if imag > 0.0 {
        Ok(format!(
            "{}+{}{}",
            number_to_string(real, decimal_separator, use_excel_rounding),
            number_to_string(imag, decimal_separator, use_excel_rounding),
            im_symbol
        ))
    } else {
        Ok(format!(
            "{}{}{}",
            number_to_string(real, decimal_separator, use_excel_rounding),
            number_to_string(imag, decimal_separator, use_excel_rounding),
            im_symbol
        ))
    }
}

pub(crate) fn number_to_string(
    number: f64,
    decimal_separator: &str,
    use_excel_rounding: bool,
) -> String {
    if use_excel_rounding {
        // Format directly with 15 significant digits rather than using
        // codcel_excel_standard_rounding (which introduces floating-point noise
        // through multiply-round-divide). The format! macro rounds during output
        // more precisely.
        let formatted = format_15_significant_digits(number);
        formatted.replace(".", decimal_separator)
    } else {
        number.to_string().replace(".", decimal_separator)
    }
}

/// Formats a number with exactly 15 significant digits, stripping trailing zeros.
/// Matches Excel's display format for complex number components.
fn format_15_significant_digits(number: f64) -> String {
    if number == 0.0 {
        return "0".to_string();
    }
    if number.is_infinite() || number.is_nan() {
        return number.to_string();
    }

    let abs = number.abs();
    let exponent = crate::portable_math::log10(abs).floor() as i32;

    // For very small numbers (|x| < 0.0001), use scientific notation like Excel
    if exponent < -4 {
        let s = format!("{:.14E}", number);
        // Strip trailing zeros in the mantissa before E
        if let Some(e_pos) = s.find('E') {
            let mantissa = s[..e_pos].trim_end_matches('0').trim_end_matches('.');
            return format!("{mantissa}E{}", &s[e_pos + 1..]);
        }
        return s;
    }

    // For very large numbers where no decimal places are needed
    if exponent >= 14 {
        return format!("{:.0}", number);
    }

    // Standard case: calculate decimal places for 15 significant digits
    let decimal_places = (14 - exponent).max(0) as usize;
    let s = format!("{:.prec$}", number, prec = decimal_places);

    // Strip trailing zeros after decimal point
    if s.contains('.') {
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        s
    }
}

pub(crate) fn format_complex_with_precision(
    real: f64,
    imag: f64,
    im_symbol: char,
    epsilon: f64,
    decimal_separator: &str,
    use_excel_rounding: bool,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    // Use epsilon for zeroing out small values
    let real = if real.abs() < epsilon { 0.0 } else { real };
    let imag = if imag.abs() < epsilon { 0.0 } else { imag };

    // Always use a reasonable tolerance for ±1 checks, independent of epsilon
    const ONE_EPS: f64 = 1e-10;

    if imag == 0.0 {
        return Ok(number_to_string(
            real,
            decimal_separator,
            use_excel_rounding,
        ));
    }

    if real == 0.0 {
        if (imag - 1.0).abs() < ONE_EPS {
            return Ok(format!("{im_symbol}"));
        }
        if (imag + 1.0).abs() < ONE_EPS {
            return Ok(format!("-{im_symbol}"));
        }
        return Ok(format!(
            "{}{}",
            number_to_string(imag, decimal_separator, use_excel_rounding),
            im_symbol
        ));
    }

    if (imag - 1.0).abs() < ONE_EPS {
        Ok(format!(
            "{}+{}",
            number_to_string(real, decimal_separator, use_excel_rounding),
            im_symbol
        ))
    } else if (imag + 1.0).abs() < ONE_EPS {
        Ok(format!(
            "{}-{}",
            number_to_string(real, decimal_separator, use_excel_rounding),
            im_symbol
        ))
    } else if imag > 0.0 {
        Ok(format!(
            "{}+{}{}",
            number_to_string(real, decimal_separator, use_excel_rounding),
            number_to_string(imag, decimal_separator, use_excel_rounding),
            im_symbol
        ))
    } else {
        Ok(format!(
            "{}{}{}",
            number_to_string(real, decimal_separator, use_excel_rounding),
            number_to_string(imag, decimal_separator, use_excel_rounding),
            im_symbol
        ))
    }
}
