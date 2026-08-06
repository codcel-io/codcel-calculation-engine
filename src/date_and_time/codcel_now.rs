// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use chrono::{DateTime, Utc};
use std::error::Error;

/// Excel-compatible `NOW` that returns the current date and time.
///   Returns the current UTC datetime (date and time components).
pub fn codcel_now() -> Result<DateTime<Utc>, Box<dyn Error + Send + Sync>> {
    Ok(Utc::now())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_now() {
        // =NOW() in US format
        // =NOW() in German format
        let before = Utc::now();
        let result = codcel_now().unwrap();
        let after = Utc::now();
        println!("{result}");

        // Check that the result is between before and after
        assert!(result >= before);
        assert!(result <= after);
    }

    #[test]
    fn test_now_close_to_current_time() {
        // =NOW() in US format
        // =NOW() in German format
        let current_time = Utc::now();
        let result = codcel_now().unwrap();
        println!("{result}");

        // Check that the result is within 1 second of the current time
        let diff = if result > current_time {
            result.signed_duration_since(current_time)
        } else {
            current_time.signed_duration_since(result)
        };

        assert!(diff.num_milliseconds() < 1000);
    }
}
