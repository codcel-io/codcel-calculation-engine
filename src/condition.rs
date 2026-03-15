// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::str::FromStr;
#[derive(Debug, PartialEq)]
pub(crate) enum Condition {
    Equal,
    LessThan,
    GreaterThan,
    NotEqual,
    LessThanOrEqual,
    GreaterThanOrEqual,
}

impl FromStr for Condition {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "<=" => Ok(Condition::LessThanOrEqual),
            ">=" => Ok(Condition::GreaterThanOrEqual),
            "<" => Ok(Condition::LessThan),
            ">" => Ok(Condition::GreaterThan),
            "<>" => Ok(Condition::NotEqual),
            "=" | "" => Ok(Condition::Equal), // Assume "=" if empty
            _ => Err(format!("Unknown condition: {input}")),
        }
    }
}

pub(crate) fn parse_condition(input: &str) -> (Condition, String) {
    let trimmed = input.trim();

    if let Some(stripped) = trimmed.strip_prefix("<>") {
        (Condition::NotEqual, stripped.trim().to_string())
    } else if let Some(stripped) = trimmed.strip_prefix("<=") {
        (Condition::LessThanOrEqual, stripped.trim().to_string())
    } else if let Some(stripped) = trimmed.strip_prefix(">=") {
        (Condition::GreaterThanOrEqual, stripped.trim().to_string())
    } else if let Some(stripped) = trimmed.strip_prefix("<") {
        (Condition::LessThan, stripped.trim().to_string())
    } else if let Some(stripped) = trimmed.strip_prefix(">") {
        (Condition::GreaterThan, stripped.trim().to_string())
    } else {
        (Condition::Equal, trimmed.to_string())
    }
}
