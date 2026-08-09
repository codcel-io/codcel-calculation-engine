// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::date_time_base::excel_to_date_time;
use crate::value_format::ValueFormat;
use chrono::{Datelike, Weekday};
use std::error::Error;
use std::fmt::Write;

// ---------------------------------------------------------------------------
// Token types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
enum FormatToken {
    Zero,                  // '0' — digit or zero
    Hash,                  // '#' — digit or nothing
    Question,              // '?' — digit or space
    DecimalPoint,          // '.'
    ThousandsSep,          // ',' between digit placeholders
    ScaleComma,            // trailing ',' — divide by 1000 each
    Percent,               // '%'
    Exponent(bool, usize), // E+/E- with zero count
    FractionSlash,         // '/' in fraction context
    Year(usize),           // 2 or 4
    Month(usize),          // 1-5
    Day(usize),            // 1-4
    Hour(usize, bool),     // count, is_12hour
    Minute(usize),         // 1 or 2
    Second(usize),         // 1 or 2
    AmPm(String),          // "AM/PM", "am/pm", "A/P"
    ElapsedHours,
    ElapsedMinutes,
    ElapsedSeconds,
    LiteralText(String),
    LiteralChar(char),
    ColorCode(String),
    AtSign,
    Asterisk(char),
    Underscore(char),
}

#[derive(Debug, Clone)]
struct FormatSection {
    tokens: Vec<FormatToken>,
    is_date_time: bool,
    has_fraction: bool,
}

#[derive(Debug, Clone)]
struct ParsedFormat {
    sections: Vec<FormatSection>,
}

// ---------------------------------------------------------------------------
// Unified entry point
// ---------------------------------------------------------------------------

pub(crate) fn format_value(
    value: f64,
    format: &str,
    value_format: &ValueFormat,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    // Special cases
    if format.is_empty() {
        return Ok(String::new());
    }
    if format == "General" {
        return Ok(format_general(value));
    }
    if format == "@" {
        return Ok(format_general(value));
    }

    let parsed = parse_format(format);

    // Check for text-only section (contains @)
    // If we have 4 sections, the 4th is text
    // If we have other configs with @, handle text placeholder

    // Select section based on value sign
    let (section, fmt_value) = select_section(&parsed, value);

    // Check if section is text-only (@)
    let is_text_section = section
        .tokens
        .iter()
        .any(|t| matches!(t, FormatToken::AtSign))
        && !section.tokens.iter().any(|t| {
            matches!(
                t,
                FormatToken::Zero
                    | FormatToken::Hash
                    | FormatToken::Question
                    | FormatToken::DecimalPoint
                    | FormatToken::Percent
                    | FormatToken::Year(_)
                    | FormatToken::Month(_)
                    | FormatToken::Day(_)
                    | FormatToken::Hour(_, _)
                    | FormatToken::Minute(_)
                    | FormatToken::Second(_)
            )
        });

    if is_text_section {
        // Replace @ with the value representation
        let value_str = format_general(value);
        let mut result = String::new();
        for token in &section.tokens {
            match token {
                FormatToken::AtSign => result.push_str(&value_str),
                FormatToken::LiteralText(text) => result.push_str(text),
                FormatToken::LiteralChar(ch) => result.push(*ch),
                _ => {}
            }
        }
        return Ok(result);
    }

    if section.is_date_time {
        format_date_time_section(value, section, value_format)
    } else {
        format_number_section(fmt_value, section, value_format)
    }
}

fn format_general(value: f64) -> String {
    if value == value.floor() && value.abs() < 1e15 {
        format!("{}", value as i64)
    } else {
        let s = format!("{}", value);
        s
    }
}

fn select_section(parsed: &ParsedFormat, value: f64) -> (&FormatSection, f64) {
    let n = parsed.sections.len();
    match n {
        0 => unreachable!(),
        1 => (&parsed.sections[0], value),
        2 => {
            if value < 0.0 {
                (&parsed.sections[1], value.abs())
            } else {
                (&parsed.sections[0], value)
            }
        }
        _ => {
            if value > 0.0 {
                (&parsed.sections[0], value)
            } else if value < 0.0 {
                (&parsed.sections[1], value.abs())
            } else {
                (&parsed.sections[std::cmp::min(2, n - 1)], 0.0)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Format string parser
// ---------------------------------------------------------------------------

fn parse_format(format: &str) -> ParsedFormat {
    let section_strs = split_sections(format);
    let sections: Vec<FormatSection> = section_strs
        .iter()
        .map(|s| {
            let mut tokens = tokenize(s);
            resolve_minutes(&mut tokens);
            resolve_12hour(&mut tokens);
            let is_date_time = tokens.iter().any(|t| {
                matches!(
                    t,
                    FormatToken::Year(_)
                        | FormatToken::Month(_)
                        | FormatToken::Day(_)
                        | FormatToken::Hour(_, _)
                        | FormatToken::Minute(_)
                        | FormatToken::Second(_)
                        | FormatToken::AmPm(_)
                        | FormatToken::ElapsedHours
                        | FormatToken::ElapsedMinutes
                        | FormatToken::ElapsedSeconds
                )
            });
            let has_fraction = tokens
                .iter()
                .any(|t| matches!(t, FormatToken::FractionSlash));

            // In date/time formats, commas between non-digit tokens are literals, not separators
            if is_date_time {
                for token in tokens.iter_mut() {
                    if matches!(token, FormatToken::ThousandsSep) {
                        *token = FormatToken::LiteralChar(',');
                    }
                }
            }

            FormatSection {
                tokens,
                is_date_time,
                has_fraction,
            }
        })
        .collect();
    ParsedFormat { sections }
}

fn split_sections(format: &str) -> Vec<String> {
    let mut sections = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut in_brackets = false;
    let chars: Vec<char> = format.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c == '"' && !in_brackets {
            in_quotes = !in_quotes;
            current.push(c);
        } else if c == '[' && !in_quotes {
            in_brackets = true;
            current.push(c);
        } else if c == ']' && !in_quotes {
            in_brackets = false;
            current.push(c);
        } else if c == ';' && !in_quotes && !in_brackets {
            sections.push(current.clone());
            current.clear();
        } else {
            current.push(c);
        }
        i += 1;
    }
    sections.push(current);
    sections
}

fn tokenize(section: &str) -> Vec<FormatToken> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = section.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let c = chars[i];
        match c {
            '"' => {
                i += 1;
                let mut text = String::new();
                while i < len && chars[i] != '"' {
                    text.push(chars[i]);
                    i += 1;
                }
                if i < len {
                    i += 1;
                }
                tokens.push(FormatToken::LiteralText(text));
            }
            '\\' => {
                i += 1;
                if i < len {
                    tokens.push(FormatToken::LiteralChar(chars[i]));
                    i += 1;
                }
            }
            '[' => {
                i += 1;
                let mut content = String::new();
                while i < len && chars[i] != ']' {
                    content.push(chars[i]);
                    i += 1;
                }
                if i < len {
                    i += 1;
                }
                let lower = content.to_lowercase();
                if lower == "h" || lower == "hh" {
                    tokens.push(FormatToken::ElapsedHours);
                } else if lower == "m" || lower == "mm" {
                    tokens.push(FormatToken::ElapsedMinutes);
                } else if lower == "s" || lower == "ss" {
                    tokens.push(FormatToken::ElapsedSeconds);
                } else {
                    tokens.push(FormatToken::ColorCode(content));
                }
            }
            'y' | 'Y' => {
                let mut count = 0;
                while i < len && (chars[i] == 'y' || chars[i] == 'Y') {
                    count += 1;
                    i += 1;
                }
                tokens.push(FormatToken::Year(if count >= 4 { 4 } else { 2 }));
            }
            'm' | 'M' => {
                let mut count = 0;
                while i < len && (chars[i] == 'm' || chars[i] == 'M') {
                    count += 1;
                    i += 1;
                }
                tokens.push(FormatToken::Month(count.min(5)));
            }
            'd' | 'D' => {
                let mut count = 0;
                while i < len && (chars[i] == 'd' || chars[i] == 'D') {
                    count += 1;
                    i += 1;
                }
                tokens.push(FormatToken::Day(count.min(4)));
            }
            'h' | 'H' => {
                let mut count = 0;
                while i < len && (chars[i] == 'h' || chars[i] == 'H') {
                    count += 1;
                    i += 1;
                }
                tokens.push(FormatToken::Hour(count.min(2), false));
            }
            's' | 'S' => {
                let mut count = 0;
                while i < len && (chars[i] == 's' || chars[i] == 'S') {
                    count += 1;
                    i += 1;
                }
                tokens.push(FormatToken::Second(count.min(2)));
            }
            'A' | 'a' => {
                let remaining: String = chars[i..].iter().collect();
                if remaining.starts_with("AM/PM") || remaining.starts_with("am/pm") {
                    let matched: String = chars[i..i + 5].iter().collect();
                    tokens.push(FormatToken::AmPm(matched));
                    i += 5;
                } else if remaining.starts_with("A/P") || remaining.starts_with("a/p") {
                    let matched: String = chars[i..i + 3].iter().collect();
                    tokens.push(FormatToken::AmPm(matched));
                    i += 3;
                } else {
                    tokens.push(FormatToken::LiteralChar(c));
                    i += 1;
                }
            }
            '0' => {
                tokens.push(FormatToken::Zero);
                i += 1;
            }
            '#' => {
                tokens.push(FormatToken::Hash);
                i += 1;
            }
            '?' => {
                tokens.push(FormatToken::Question);
                i += 1;
            }
            '.' => {
                tokens.push(FormatToken::DecimalPoint);
                i += 1;
            }
            ',' => {
                tokens.push(FormatToken::ThousandsSep);
                i += 1;
            }
            '%' => {
                tokens.push(FormatToken::Percent);
                i += 1;
            }
            'E' | 'e' => {
                if i + 1 < len && (chars[i + 1] == '+' || chars[i + 1] == '-') {
                    let is_plus = chars[i + 1] == '+';
                    i += 2;
                    let mut zero_count = 0;
                    while i < len && chars[i] == '0' {
                        zero_count += 1;
                        i += 1;
                    }
                    if zero_count > 0 {
                        tokens.push(FormatToken::Exponent(is_plus, zero_count));
                    } else {
                        tokens.push(FormatToken::LiteralChar('E'));
                    }
                } else {
                    tokens.push(FormatToken::LiteralChar(c));
                    i += 1;
                }
            }
            '/' => {
                let has_num_before = tokens.iter().rev().any(|t| {
                    matches!(
                        t,
                        FormatToken::Zero | FormatToken::Hash | FormatToken::Question
                    )
                });
                let has_num_after = i + 1 < len
                    && (chars[i + 1] == '?'
                        || chars[i + 1] == '#'
                        || chars[i + 1] == '0'
                        || chars[i + 1].is_ascii_digit());
                if has_num_before && has_num_after {
                    tokens.push(FormatToken::FractionSlash);
                } else {
                    tokens.push(FormatToken::LiteralChar(c));
                }
                i += 1;
            }
            '@' => {
                tokens.push(FormatToken::AtSign);
                i += 1;
            }
            '*' => {
                i += 1;
                if i < len {
                    tokens.push(FormatToken::Asterisk(chars[i]));
                    i += 1;
                }
            }
            '_' => {
                i += 1;
                if i < len {
                    tokens.push(FormatToken::Underscore(chars[i]));
                    i += 1;
                }
            }
            '$' | '£' | '¥' | '€' | '-' | '+' | '(' | ')' | ' ' | ':' => {
                tokens.push(FormatToken::LiteralChar(c));
                i += 1;
            }
            _ => {
                tokens.push(FormatToken::LiteralChar(c));
                i += 1;
            }
        }
    }

    // Resolve commas: determine which are thousands separators and which are scale commas
    resolve_commas(&mut tokens);
    tokens
}

fn resolve_commas(tokens: &mut [FormatToken]) {
    let first_digit_idx = tokens.iter().position(|t| {
        matches!(
            t,
            FormatToken::Zero | FormatToken::Hash | FormatToken::Question
        )
    });

    let decimal_pos = tokens
        .iter()
        .position(|t| matches!(t, FormatToken::DecimalPoint));

    if first_digit_idx.is_none() {
        // No digit placeholders — all commas become literals
        for token in tokens.iter_mut() {
            if matches!(token, FormatToken::ThousandsSep) {
                *token = FormatToken::LiteralChar(',');
            }
        }
        return;
    }

    let first = first_digit_idx.unwrap();
    let int_end = decimal_pos.unwrap_or(tokens.len());

    for i in 0..tokens.len() {
        if matches!(tokens[i], FormatToken::ThousandsSep) {
            if i < first || i >= int_end {
                // Outside integer digit range — literal
                tokens[i] = FormatToken::LiteralChar(',');
            } else {
                // Check if there's a digit placeholder after this comma (in the integer part)
                let has_digit_after = tokens[i + 1..int_end].iter().any(|t| {
                    matches!(
                        t,
                        FormatToken::Zero | FormatToken::Hash | FormatToken::Question
                    )
                });
                if has_digit_after {
                    // Thousands separator — keep as is
                } else {
                    // Trailing comma — scale comma (divide by 1000)
                    tokens[i] = FormatToken::ScaleComma;
                }
            }
        }
    }
}

fn resolve_minutes(tokens: &mut [FormatToken]) {
    let len = tokens.len();
    for i in 0..len {
        if let FormatToken::Month(n) = tokens[i] {
            if n <= 2 {
                // Check if preceded by Hour/ElapsedHours (ignoring ':', spaces, and literal text)
                let mut found = false;
                if i > 0 {
                    let mut j = i - 1;
                    loop {
                        match &tokens[j] {
                            FormatToken::LiteralChar(':')
                            | FormatToken::LiteralChar(' ')
                            | FormatToken::LiteralText(_) => {}
                            FormatToken::Hour(_, _) | FormatToken::ElapsedHours => {
                                found = true;
                                break;
                            }
                            _ => break,
                        }
                        if j == 0 {
                            break;
                        }
                        j -= 1;
                    }
                }
                // Check if followed by Second/ElapsedSeconds (ignoring ':', spaces, and literal text)
                if !found {
                    let mut j = i + 1;
                    while j < len {
                        match &tokens[j] {
                            FormatToken::LiteralChar(':')
                            | FormatToken::LiteralChar(' ')
                            | FormatToken::LiteralText(_) => {
                                j += 1;
                            }
                            FormatToken::Second(_) | FormatToken::ElapsedSeconds => {
                                found = true;
                                break;
                            }
                            _ => break,
                        }
                    }
                }
                if found {
                    tokens[i] = FormatToken::Minute(n);
                }
            }
        }
    }
}

fn resolve_12hour(tokens: &mut [FormatToken]) {
    let has_ampm = tokens.iter().any(|t| matches!(t, FormatToken::AmPm(_)));
    if has_ampm {
        for token in tokens.iter_mut() {
            if let FormatToken::Hour(n, _) = token {
                *token = FormatToken::Hour(*n, true);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Number formatting
// ---------------------------------------------------------------------------

fn format_number_section(
    value: f64,
    section: &FormatSection,
    value_format: &ValueFormat,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    // Check for @ (text placeholder) mixed with number formats
    if section
        .tokens
        .iter()
        .any(|t| matches!(t, FormatToken::AtSign))
    {
        let value_str = format_general(value);
        let mut result = String::new();
        for token in &section.tokens {
            match token {
                FormatToken::AtSign => result.push_str(&value_str),
                FormatToken::LiteralText(text) => result.push_str(text),
                FormatToken::LiteralChar(ch) => result.push(*ch),
                _ => {}
            }
        }
        return Ok(result);
    }

    // Handle fraction format
    if section.has_fraction {
        return format_fraction(value, section, value_format);
    }

    // Check for scientific notation
    if section
        .tokens
        .iter()
        .any(|t| matches!(t, FormatToken::Exponent(_, _)))
    {
        return format_scientific(value, section, value_format);
    }

    // Count scale commas
    let scale_count = section
        .tokens
        .iter()
        .filter(|t| matches!(t, FormatToken::ScaleComma))
        .count();
    let mut val = value;
    for _ in 0..scale_count {
        val /= 1000.0;
    }

    // Handle percentage
    let has_percent = section
        .tokens
        .iter()
        .any(|t| matches!(t, FormatToken::Percent));
    if has_percent {
        val *= 100.0;
    }

    // Check for pattern format (like ###-##-#### or (###) ###-####)
    let is_pattern = is_pattern_format(section);
    if is_pattern {
        return format_pattern(val, section);
    }

    // Determine decimal places from format
    let decimal_pos = section
        .tokens
        .iter()
        .position(|t| matches!(t, FormatToken::DecimalPoint));
    let (max_decimals, min_decimals) = if let Some(dp) = decimal_pos {
        let after: Vec<&FormatToken> = section.tokens[dp + 1..]
            .iter()
            .take_while(|t| {
                matches!(
                    t,
                    FormatToken::Zero | FormatToken::Hash | FormatToken::Question
                )
            })
            .collect();
        let max = after.len();
        let min = after
            .iter()
            .filter(|t| matches!(t, FormatToken::Zero))
            .count();
        (max, min)
    } else {
        (0, 0)
    };

    // Count integer format digits
    let int_end = decimal_pos.unwrap_or(section.tokens.len());
    let int_zeros: usize = section.tokens[..int_end]
        .iter()
        .filter(|t| matches!(t, FormatToken::Zero))
        .count();
    let has_thousands = section.tokens[..int_end]
        .iter()
        .any(|t| matches!(t, FormatToken::ThousandsSep));

    // Count ? placeholders in integer part for space-padding
    let int_questions: usize = section.tokens[..int_end]
        .iter()
        .filter(|t| matches!(t, FormatToken::Question))
        .count();
    let int_min_width = int_zeros + int_questions;

    // Round the value
    let is_negative = val < 0.0;
    let abs_value = val.abs();
    let rounded_str = format!("{:.prec$}", abs_value, prec = max_decimals);

    // Split into integer and decimal parts
    let parts: Vec<&str> = rounded_str.split('.').collect();
    let int_part = parts[0];
    let dec_part = if parts.len() > 1 { parts[1] } else { "" };

    // Trim trailing zeros from decimal part, but keep min_decimals
    let mut trimmed_dec = dec_part.to_string();
    while trimmed_dec.len() > min_decimals && trimmed_dec.ends_with('0') {
        trimmed_dec.pop();
    }

    // Format integer part
    let mut int_digits = int_part.to_string();

    // For hash-only integer (no zeros, no ?), suppress "0"
    let int_is_hash_only = int_zeros == 0
        && int_questions == 0
        && section.tokens[..int_end]
            .iter()
            .any(|t| matches!(t, FormatToken::Hash));
    if int_is_hash_only && int_digits == "0" {
        int_digits.clear();
    }

    // Zero-pad integer part if needed (for 0 placeholders)
    while int_digits.len() < int_zeros {
        int_digits.insert(0, '0');
    }
    // Space-pad integer part for ? placeholders
    while int_digits.len() < int_min_width {
        int_digits.insert(0, ' ');
    }

    // Add thousands separators if needed
    if has_thousands {
        int_digits = add_thousands_sep(&int_digits, &value_format.thousands_separator);
    }

    // Now assemble the output by walking tokens
    let mut result = String::new();
    if is_negative {
        result.push('-');
    }

    let mut int_emitted = false;
    let mut dec_emitted = false;

    for (idx, token) in section.tokens.iter().enumerate() {
        let in_integer_part = decimal_pos.is_none_or(|dp| idx < dp);

        match token {
            FormatToken::Zero | FormatToken::Hash | FormatToken::Question => {
                if in_integer_part {
                    if !int_emitted {
                        result.push_str(&int_digits);
                        int_emitted = true;
                    }
                } else if !dec_emitted {
                    result.push_str(&trimmed_dec);
                    dec_emitted = true;
                }
            }
            FormatToken::DecimalPoint => {
                if !trimmed_dec.is_empty() || min_decimals > 0 || decimal_pos.is_some() {
                    result.push_str(&value_format.decimal_separator);
                }
                if !int_emitted {
                    int_emitted = true;
                }
            }
            FormatToken::ThousandsSep | FormatToken::ScaleComma => {}
            FormatToken::Percent => {
                result.push('%');
            }
            FormatToken::LiteralText(text) => {
                result.push_str(text);
            }
            FormatToken::LiteralChar(ch) => {
                result.push(*ch);
            }
            FormatToken::ColorCode(_) | FormatToken::Asterisk(_) => {}
            FormatToken::Underscore(_) => {
                result.push(' ');
            }
            FormatToken::AtSign => {
                result.push_str(&format_general(abs_value));
            }
            _ => {}
        }
    }

    Ok(result)
}

fn is_pattern_format(section: &FormatSection) -> bool {
    // A pattern format has digit placeholders interleaved with literal dashes or parens
    // but no ThousandsSep, no DecimalPoint, no Percent
    let has_digit = section
        .tokens
        .iter()
        .any(|t| matches!(t, FormatToken::Zero | FormatToken::Hash));
    let has_thousands = section
        .tokens
        .iter()
        .any(|t| matches!(t, FormatToken::ThousandsSep));
    let has_decimal = section
        .tokens
        .iter()
        .any(|t| matches!(t, FormatToken::DecimalPoint));
    let has_percent = section
        .tokens
        .iter()
        .any(|t| matches!(t, FormatToken::Percent));

    if !has_digit || has_thousands || has_decimal || has_percent {
        return false;
    }

    // Check that digit placeholders and dashes/parens are interleaved
    let mut saw_digit = false;
    let mut saw_literal_after_digit = false;
    let mut saw_digit_after_literal = false;
    for token in &section.tokens {
        match token {
            FormatToken::Zero | FormatToken::Hash | FormatToken::Question => {
                if saw_literal_after_digit {
                    saw_digit_after_literal = true;
                }
                saw_digit = true;
            }
            FormatToken::LiteralChar('-')
            | FormatToken::LiteralChar(' ')
            | FormatToken::LiteralChar('(')
            | FormatToken::LiteralChar(')')
                if saw_digit =>
            {
                saw_literal_after_digit = true;
            }
            _ => {}
        }
    }
    saw_digit_after_literal
}

fn format_pattern(
    value: f64,
    section: &FormatSection,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let int_value = value.abs().round() as u64;
    let digits: Vec<char> = format!("{}", int_value).chars().collect();

    let placeholder_count = section
        .tokens
        .iter()
        .filter(|t| {
            matches!(
                t,
                FormatToken::Zero | FormatToken::Hash | FormatToken::Question
            )
        })
        .count();

    // Pad digits to match placeholder count
    let mut padded_digits: Vec<char> = vec!['0'; placeholder_count.saturating_sub(digits.len())];
    padded_digits.extend_from_slice(&digits);

    // Walk tokens, fill in digits
    let mut result = String::new();
    let mut digit_idx = padded_digits.len().saturating_sub(placeholder_count);

    for token in &section.tokens {
        match token {
            FormatToken::Zero | FormatToken::Hash | FormatToken::Question => {
                if digit_idx < padded_digits.len() {
                    result.push(padded_digits[digit_idx]);
                    digit_idx += 1;
                }
            }
            FormatToken::LiteralChar(ch) => result.push(*ch),
            FormatToken::LiteralText(text) => result.push_str(text),
            _ => {}
        }
    }

    Ok(result)
}

fn format_scientific(
    value: f64,
    section: &FormatSection,
    value_format: &ValueFormat,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let (is_plus, exp_zeros) = section
        .tokens
        .iter()
        .find_map(|t| {
            if let FormatToken::Exponent(p, z) = t {
                Some((*p, *z))
            } else {
                None
            }
        })
        .unwrap_or((true, 2));

    let decimal_pos = section
        .tokens
        .iter()
        .position(|t| matches!(t, FormatToken::DecimalPoint));
    let exp_pos = section
        .tokens
        .iter()
        .position(|t| matches!(t, FormatToken::Exponent(_, _)));
    let dec_places = if let (Some(dp), Some(ep)) = (decimal_pos, exp_pos) {
        section.tokens[dp + 1..ep]
            .iter()
            .filter(|t| matches!(t, FormatToken::Zero | FormatToken::Hash))
            .count()
    } else {
        2
    };

    let abs_value = value.abs();
    let (mantissa, exponent) = if abs_value == 0.0 {
        (0.0, 0i32)
    } else {
        let exp = crate::portable_math::log10(abs_value).floor() as i32;
        let man = abs_value / 10f64.powi(exp);
        (man, exp)
    };

    let mantissa_str = format!("{:.prec$}", mantissa, prec = dec_places);
    let exp_sign = if exponent >= 0 {
        if is_plus {
            "+"
        } else {
            ""
        }
    } else {
        "-"
    };
    let exp_str = format!("{:0>width$}", exponent.abs(), width = exp_zeros);

    let mut result = String::new();
    if value < 0.0 {
        result.push('-');
    }
    write!(result, "{}E{}{}", mantissa_str, exp_sign, exp_str)?;
    if value_format.decimal_separator != "." {
        result = result.replace('.', &value_format.decimal_separator);
    }
    Ok(result)
}

fn format_fraction(
    value: f64,
    section: &FormatSection,
    _value_format: &ValueFormat,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let is_negative = value < 0.0;
    let abs_value = value.abs();

    let slash_pos = section
        .tokens
        .iter()
        .position(|t| matches!(t, FormatToken::FractionSlash))
        .unwrap();

    // Count numerator placeholders (? or # or 0 immediately before the slash)
    let num_placeholders: Vec<&FormatToken> = section.tokens[..slash_pos]
        .iter()
        .rev()
        .take_while(|t| {
            matches!(
                t,
                FormatToken::Zero | FormatToken::Hash | FormatToken::Question
            )
        })
        .collect();
    let num_width = num_placeholders.len();

    // Find the start position of the numerator placeholders
    let num_start_pos = slash_pos - num_width;

    // Check if there are whole number placeholders before the numerator
    let has_whole = num_start_pos > 0
        && section.tokens[..num_start_pos].iter().any(|t| {
            matches!(
                t,
                FormatToken::Hash | FormatToken::Zero | FormatToken::Question
            )
        });

    let whole = if has_whole {
        abs_value.floor() as i64
    } else {
        0
    };
    let frac = if has_whole {
        abs_value - whole as f64
    } else {
        abs_value
    };

    // Determine the denominator format (after the slash)
    let after_slash = &section.tokens[slash_pos + 1..];

    // Check if denominator contains fixed digits.
    // In the format "??/100", the tokenizer produces LiteralChar('1'), Zero, Zero
    // because '0' is always tokenized as FormatToken::Zero.
    // First, check if we have any literal digit chars — if so, treat all
    // subsequent Zero tokens as part of the fixed denominator number.
    let has_literal_digit_after_slash = after_slash
        .iter()
        .any(|t| matches!(t, FormatToken::LiteralChar(c) if c.is_ascii_digit()));

    let mut fixed_denom_str = String::new();
    let mut denom_placeholder_count = 0;
    for t in after_slash {
        match t {
            FormatToken::LiteralChar(c) if c.is_ascii_digit() => {
                fixed_denom_str.push(*c);
            }
            FormatToken::Zero if has_literal_digit_after_slash => {
                // In fixed denominator context, '0' is part of the number
                fixed_denom_str.push('0');
            }
            FormatToken::Zero | FormatToken::Hash | FormatToken::Question
                if !has_literal_digit_after_slash =>
            {
                denom_placeholder_count += 1;
            }
            _ => break,
        }
    }

    let denom_width = if !fixed_denom_str.is_empty() {
        fixed_denom_str.len()
    } else {
        denom_placeholder_count
    };

    let (numerator, denominator) = if !fixed_denom_str.is_empty() {
        let denom: i64 = fixed_denom_str.parse().unwrap_or(1);
        let num = (frac * denom as f64).round() as i64;
        (num, denom)
    } else {
        let max_denom = 10i64.pow(denom_placeholder_count.max(1) as u32) - 1;
        best_fraction(frac, max_denom)
    };

    let mut result = String::new();
    if is_negative {
        result.push('-');
    }

    // Emit any leading literal tokens before the first placeholder
    let first_placeholder_pos = section
        .tokens
        .iter()
        .position(|t| {
            matches!(
                t,
                FormatToken::Zero
                    | FormatToken::Hash
                    | FormatToken::Question
                    | FormatToken::FractionSlash
            )
        })
        .unwrap_or(0);

    for token in &section.tokens[..first_placeholder_pos] {
        match token {
            FormatToken::LiteralChar(ch) => result.push(*ch),
            FormatToken::LiteralText(text) => result.push_str(text),
            FormatToken::ColorCode(_) | FormatToken::Asterisk(_) => {}
            FormatToken::Underscore(_) => result.push(' '),
            _ => {}
        }
    }

    if has_whole {
        if whole > 0 || (numerator == 0 && frac == 0.0) {
            write!(result, "{}", whole)?;
        }

        if numerator == 0 {
            // Whole number only — pad to fill the fraction space
            // Space between whole and fraction + numerator width + "/" + denominator width
            let sep_width = 1; // the space between whole and fraction
            let frac_width = num_width + 1 + denom_width;
            let total_pad = sep_width + frac_width;
            for _ in 0..total_pad {
                result.push(' ');
            }
        } else {
            // Emit the separator between whole and numerator
            // Walk tokens between whole placeholders and numerator to find literal separators
            let whole_end = section.tokens[..num_start_pos]
                .iter()
                .rposition(|t| {
                    matches!(
                        t,
                        FormatToken::Hash | FormatToken::Zero | FormatToken::Question
                    )
                })
                .map(|p| p + 1)
                .unwrap_or(0);

            for token in &section.tokens[whole_end..num_start_pos] {
                match token {
                    FormatToken::LiteralChar(ch) => result.push(*ch),
                    FormatToken::LiteralText(text) => result.push_str(text),
                    _ => {}
                }
            }

            let num_str = format!("{:>width$}", numerator, width = num_width);
            let denom_display = if !fixed_denom_str.is_empty() {
                fixed_denom_str.clone()
            } else {
                format!("{:<width$}", denominator, width = denom_width)
            };
            write!(result, "{}/{}", num_str, denom_display)?;
        }
    } else {
        // No whole part — just numerator/denominator
        let num_str = format!("{:>width$}", numerator, width = num_width);
        let denom_display = if !fixed_denom_str.is_empty() {
            fixed_denom_str.clone()
        } else {
            format!("{:<width$}", denominator, width = denom_width)
        };
        write!(result, "{}/{}", num_str, denom_display)?;
    }

    Ok(result)
}

fn best_fraction(frac: f64, max_denom: i64) -> (i64, i64) {
    if frac == 0.0 {
        return (0, 1);
    }
    let mut best_num = 0i64;
    let mut best_den = 1i64;
    let mut best_err = f64::MAX;
    for d in 1..=max_denom {
        let n = (frac * d as f64).round() as i64;
        let err = (frac - n as f64 / d as f64).abs();
        if err < best_err {
            best_err = err;
            best_num = n;
            best_den = d;
            if err < 1e-12 {
                break;
            }
        }
    }
    (best_num, best_den)
}

fn add_thousands_sep(int_str: &str, sep: &str) -> String {
    let mut result = String::new();
    let is_negative = int_str.starts_with('-');
    let digits = if is_negative { &int_str[1..] } else { int_str };
    let len = digits.len();
    for (i, c) in digits.chars().enumerate() {
        result.push(c);
        let remaining = len - i - 1;
        if remaining > 0 && remaining % 3 == 0 {
            result.push_str(sep);
        }
    }
    if is_negative {
        result.insert(0, '-');
    }
    result
}

// ---------------------------------------------------------------------------
// Date/time formatting
// ---------------------------------------------------------------------------

fn format_date_time_section(
    excel_serial: f64,
    section: &FormatSection,
    value_format: &ValueFormat,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    // Extract date using excel_to_date_time
    // For serial < 1 (pure time), use the Excel epoch base date (Jan 0, 1900 → treat as Jan 1)
    let (year, month, day, weekday) = if excel_serial < 1.0 {
        // Excel serial 0 = January 0, 1900. For display purposes, use January 1, 1900.
        (1900i32, 1u32, 0u32, chrono::Weekday::Sat)
    } else {
        let dt = excel_to_date_time(excel_serial, value_format.allow_lotus_1_2_3_1900_date_bug)?;
        (dt.year(), dt.month(), dt.day(), dt.weekday())
    };

    // Extract time from fractional part directly (more reliable than DateTime for edge cases)
    let frac = excel_serial.fract().abs();
    let total_seconds_of_day = (frac * 86400.0).round() as u32;
    let hour24 = total_seconds_of_day / 3600;
    let minute = (total_seconds_of_day % 3600) / 60;
    let second = total_seconds_of_day % 60;

    // Check for AM/PM
    let has_ampm = section
        .tokens
        .iter()
        .any(|t| matches!(t, FormatToken::AmPm(_)));
    let (hour_display, am_pm) = if has_ampm {
        let h = if hour24 == 0 {
            12
        } else if hour24 > 12 {
            hour24 - 12
        } else {
            hour24
        };
        let period = if hour24 < 12 { "AM" } else { "PM" };
        (h, period)
    } else {
        (hour24, "")
    };

    let mut result = String::new();

    for token in &section.tokens {
        match token {
            FormatToken::Year(2) => write!(result, "{:02}", year % 100)?,
            FormatToken::Year(_) => write!(result, "{:04}", year)?,
            FormatToken::Month(1) => write!(result, "{}", month)?,
            FormatToken::Month(2) => write!(result, "{:02}", month)?,
            FormatToken::Month(3) => result.push_str(month_abbrev(month)),
            FormatToken::Month(4) => result.push_str(month_name(month)),
            FormatToken::Month(5) => {
                let name = month_name(month);
                if let Some(c) = name.chars().next() {
                    result.push(c);
                }
            }
            FormatToken::Month(_) => write!(result, "{:02}", month)?,
            FormatToken::Day(1) => write!(result, "{}", day)?,
            FormatToken::Day(2) => write!(result, "{:02}", day)?,
            FormatToken::Day(3) => result.push_str(weekday_abbrev(weekday)),
            FormatToken::Day(4) => result.push_str(weekday_name(weekday)),
            FormatToken::Day(_) => write!(result, "{:02}", day)?,
            FormatToken::Hour(1, true) => write!(result, "{}", hour_display)?,
            FormatToken::Hour(_, true) => write!(result, "{:02}", hour_display)?,
            FormatToken::Hour(1, false) => write!(result, "{}", hour24)?,
            FormatToken::Hour(_, false) => write!(result, "{:02}", hour24)?,
            FormatToken::Minute(1) => write!(result, "{}", minute)?,
            FormatToken::Minute(_) => write!(result, "{:02}", minute)?,
            FormatToken::Second(1) => write!(result, "{}", second)?,
            FormatToken::Second(_) => write!(result, "{:02}", second)?,
            FormatToken::AmPm(_) => {
                // Excel always outputs AM/PM in uppercase regardless of format case
                result.push_str(am_pm);
            }
            FormatToken::ElapsedHours => {
                let total_hours = (excel_serial * 24.0).floor() as i64;
                write!(result, "{}", total_hours)?;
            }
            FormatToken::ElapsedMinutes => {
                let total_minutes = (excel_serial * 1440.0).floor() as i64;
                write!(result, "{}", total_minutes)?;
            }
            FormatToken::ElapsedSeconds => {
                let total_seconds = (excel_serial * 86400.0).floor() as i64;
                write!(result, "{}", total_seconds)?;
            }
            FormatToken::LiteralText(text) => result.push_str(text),
            FormatToken::LiteralChar(ch) => result.push(*ch),
            FormatToken::ColorCode(_) | FormatToken::Asterisk(_) => {}
            FormatToken::Underscore(_) => result.push(' '),
            _ => {}
        }
    }

    Ok(result)
}

fn month_abbrev(m: u32) -> &'static str {
    match m {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "???",
    }
}

fn month_name(m: u32) -> &'static str {
    match m {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "???",
    }
}

fn weekday_abbrev(wd: Weekday) -> &'static str {
    match wd {
        Weekday::Mon => "Mon",
        Weekday::Tue => "Tue",
        Weekday::Wed => "Wed",
        Weekday::Thu => "Thu",
        Weekday::Fri => "Fri",
        Weekday::Sat => "Sat",
        Weekday::Sun => "Sun",
    }
}

fn weekday_name(wd: Weekday) -> &'static str {
    match wd {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn vf() -> ValueFormat {
        ValueFormat {
            decimal_separator: ".".to_string(),
            currency_symbol: "$".to_string(),
            thousands_separator: ",".to_string(),
            use_excel_rounding: true,
            language: "en".to_string(),
            allow_lotus_1_2_3_1900_date_bug: true,
        }
    }

    #[test]
    fn test_basic_decimal() {
        assert_eq!(format_value(123.45, "0.00", &vf()).unwrap(), "123.45");
        assert_eq!(format_value(123.4, "0.00", &vf()).unwrap(), "123.40");
        assert_eq!(format_value(0.5, "0.00", &vf()).unwrap(), "0.50");
    }

    #[test]
    fn test_rounding() {
        assert_eq!(format_value(999.999, "0.00", &vf()).unwrap(), "1000.00");
        assert_eq!(format_value(1.999, "0.00", &vf()).unwrap(), "2.00");
        assert_eq!(format_value(1.995, "0.00", &vf()).unwrap(), "2.00");
        assert_eq!(
            format_value(6789.83945, "#,##0.00", &vf()).unwrap(),
            "6,789.84"
        );
    }

    #[test]
    fn test_thousands() {
        assert_eq!(format_value(1234.0, "#,##0", &vf()).unwrap(), "1,234");
        assert_eq!(
            format_value(1234567.89, "#,##0.00", &vf()).unwrap(),
            "1,234,567.89"
        );
    }

    #[test]
    fn test_currency() {
        assert_eq!(
            format_value(1234.5678, "$#,##0.00", &vf()).unwrap(),
            "$1,234.57"
        );
    }

    #[test]
    fn test_percentage() {
        assert_eq!(format_value(0.456, "0.0%", &vf()).unwrap(), "45.6%");
        assert_eq!(format_value(0.1575, "0.0%", &vf()).unwrap(), "15.8%");
    }

    #[test]
    fn test_multi_section() {
        assert_eq!(
            format_value(-100.0, "#,##0;(#,##0)", &vf()).unwrap(),
            "(100)"
        );
        assert_eq!(format_value(100.0, "#,##0;(#,##0)", &vf()).unwrap(), "100");
        assert_eq!(
            format_value(0.0, "#,##0;(#,##0);\"Zero\"", &vf()).unwrap(),
            "Zero"
        );
    }

    #[test]
    fn test_date_weekday() {
        // 45658 = Jan 1, 2025 = Wednesday
        assert_eq!(format_value(45658.0, "dddd", &vf()).unwrap(), "Wednesday");
        assert_eq!(format_value(45658.0, "ddd", &vf()).unwrap(), "Wed");
    }

    #[test]
    fn test_date_short() {
        assert_eq!(format_value(45658.0, "m/d/yy", &vf()).unwrap(), "1/1/25");
        assert_eq!(
            format_value(45658.0, "mmmm d, yyyy", &vf()).unwrap(),
            "January 1, 2025"
        );
    }

    #[test]
    fn test_time_ampm() {
        // 0.75 = 18:00
        assert_eq!(format_value(0.75, "h:mm AM/PM", &vf()).unwrap(), "6:00 PM");
    }

    #[test]
    fn test_elapsed_hours() {
        // 2.75 days = 66 hours
        assert_eq!(format_value(2.75, "[h]", &vf()).unwrap(), "66");
    }

    #[test]
    fn test_literal_text_in_format() {
        assert_eq!(
            format_value(0.75, "hh\"h \"mm\"m \"ss\"s\"", &vf()).unwrap(),
            "18h 00m 00s"
        );
    }

    #[test]
    fn test_zero_padded() {
        assert_eq!(format_value(1.0, "000", &vf()).unwrap(), "001");
        assert_eq!(format_value(42.0, "00000", &vf()).unwrap(), "00042");
    }

    #[test]
    fn test_color_code_stripped() {
        assert_eq!(
            format_value(-5678.9, "[Red]#,##0;[Blue](#,##0)", &vf()).unwrap(),
            "(5,679)"
        );
    }

    #[test]
    fn test_scientific() {
        assert_eq!(format_value(1234.0, "0.00E+00", &vf()).unwrap(), "1.23E+03");
    }

    #[test]
    fn test_pattern_ssn() {
        assert_eq!(
            format_value(123456789.0, "###-##-####", &vf()).unwrap(),
            "123-45-6789"
        );
    }

    #[test]
    fn test_pattern_phone() {
        assert_eq!(
            format_value(5551234567.0, "(###) ###-####", &vf()).unwrap(),
            "(555) 123-4567"
        );
    }

    #[test]
    fn test_general() {
        assert_eq!(format_value(1234.0, "General", &vf()).unwrap(), "1234");
    }

    #[test]
    fn test_at_sign() {
        assert_eq!(format_value(1234.0, "@", &vf()).unwrap(), "1234");
    }

    #[test]
    fn test_at_sign_with_text() {
        assert_eq!(
            format_value(1234.0, "0\" units\"", &vf()).unwrap(),
            "1234 units"
        );
    }

    #[test]
    fn test_no_decimal_hash() {
        assert_eq!(format_value(3.14159, "#", &vf()).unwrap(), "3");
        assert_eq!(format_value(3.14159, "0", &vf()).unwrap(), "3");
    }

    #[test]
    fn test_fraction_simple() {
        assert_eq!(format_value(0.5, " ?/?", &vf()).unwrap(), " 1/2");
        assert_eq!(format_value(0.25, " ?/?", &vf()).unwrap(), " 1/4");
    }

    #[test]
    fn test_fraction_with_whole() {
        // # ?/? with value 0.5 → leading space + fraction (no whole shown)
        assert_eq!(format_value(0.5, "# ?/?", &vf()).unwrap(), " 1/2");
        assert_eq!(format_value(0.25, "# ?/?", &vf()).unwrap(), " 1/4");
        // With whole number
        assert_eq!(format_value(3.5, "# ?/?", &vf()).unwrap(), "3 1/2");
        // Zero value
        assert_eq!(format_value(0.0, "# ?/?", &vf()).unwrap(), "0    ");
    }

    #[test]
    fn test_fraction_fixed_denom() {
        // # ?/8 with 0.875 → " 7/8"
        assert_eq!(format_value(0.875, "# ?/8", &vf()).unwrap(), " 7/8");
        // # ??/8 with 0.25 → "  2/8"
        assert_eq!(format_value(0.25, "# ??/8", &vf()).unwrap(), "  2/8");
        // # ??/16 with 0.25 → "  4/16"
        assert_eq!(format_value(0.25, "# ??/16", &vf()).unwrap(), "  4/16");
        // # ??/100 with 0.14159 → " 14/100"
        assert_eq!(format_value(0.14159, "# ??/100", &vf()).unwrap(), " 14/100");
    }

    #[test]
    fn test_fraction_three_digit() {
        // # ???/??? with 3.14159 → "3  16/113"
        assert_eq!(
            format_value(3.14159265358979, "# ???/???", &vf()).unwrap(),
            "3  16/113"
        );
    }

    #[test]
    fn test_ampm_always_uppercase() {
        // h:mm am/pm should still output uppercase AM/PM
        assert_eq!(format_value(0.75, "h:mm am/pm", &vf()).unwrap(), "6:00 PM");
        assert_eq!(format_value(0.25, "h:mm am/pm", &vf()).unwrap(), "6:00 AM");
    }

    #[test]
    fn test_date_serial_less_than_one() {
        // mm format with serial < 1 should give month=1 (January)
        assert_eq!(format_value(0.354166666666667, "mm", &vf()).unwrap(), "01");
    }

    #[test]
    fn test_question_mark_padding() {
        // ??.00 with 1.0 → " 1.00"
        assert_eq!(format_value(1.0, "??.00", &vf()).unwrap(), " 1.00");
    }

    #[test]
    fn test_neg_paren_multi_section() {
        assert_eq!(format_value(-42.0, "0;(0)", &vf()).unwrap(), "(42)");
    }

    #[test]
    fn test_scale_comma() {
        // Trailing comma divides by 1000
        assert_eq!(format_value(1500000.0, "#,##0,", &vf()).unwrap(), "1,500");
    }

    #[test]
    fn test_date_with_comma() {
        // Comma in date format should be literal, not thousands separator
        assert_eq!(
            format_value(45658.0, "dddd, mmmm d, yyyy", &vf()).unwrap(),
            "Wednesday, January 1, 2025"
        );
    }

    #[test]
    fn test_leading_zeros() {
        assert_eq!(format_value(42.0, "000000", &vf()).unwrap(), "000042");
        assert_eq!(format_value(1234.0, "00000", &vf()).unwrap(), "01234");
    }
}
