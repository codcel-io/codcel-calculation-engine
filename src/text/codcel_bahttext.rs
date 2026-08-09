// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

const DIGITS: [&str; 10] = [
    "ศูนย์",
    "หนึ่ง",
    "สอง",
    "สาม",
    "สี่",
    "ห้า",
    "หก",
    "เจ็ด",
    "แปด",
    "เก้า",
];

const PLACES: [&str; 6] = ["", "สิบ", "ร้อย", "พัน", "หมื่น", "แสน"];

/// Converts a group of up to 6 digits to Thai text.
/// `group` must be in range 0..=999999.
fn group_to_thai(group: u64) -> String {
    if group == 0 {
        return String::new();
    }

    let mut result = String::new();
    let digits: Vec<u64> = {
        let mut g = group;
        let mut d = Vec::new();
        while g > 0 {
            d.push(g % 10);
            g /= 10;
        }
        d
    };

    // Process from highest place to lowest
    for i in (0..digits.len()).rev() {
        let digit = digits[i];
        if digit == 0 {
            continue;
        }

        if i == 1 {
            // Tens place
            if digit == 1 {
                result.push_str("สิบ");
            } else if digit == 2 {
                result.push_str("ยี่สิบ");
            } else {
                result.push_str(DIGITS[digit as usize]);
                result.push_str("สิบ");
            }
        } else if i == 0 {
            // Ones place
            if digits.len() > 1 && digit == 1 {
                // "เอ็ด" when ones digit is 1 and there are higher digits
                result.push_str("เอ็ด");
            } else {
                result.push_str(DIGITS[digit as usize]);
            }
        } else {
            // Hundreds, thousands, ten-thousands, hundred-thousands
            result.push_str(DIGITS[digit as usize]);
            result.push_str(PLACES[i]);
        }
    }

    result
}

/// Excel-compatible `BAHTTEXT` that converts a number to Thai text
/// representing Thai Baht currency.
/// - `number`: the number to convert to Thai Baht text.
///
/// Returns the Thai text representation of the number as Baht currency.
pub fn codcel_bahttext(number: f64) -> Result<String, Box<dyn Error + Send + Sync>> {
    let is_negative = number < 0.0;
    let abs_number = number.abs();

    // Round to 2 decimal places (satang)
    let rounded = (abs_number * 100.0).round() / 100.0;

    let integer_part = rounded as u64;
    let satang = ((rounded * 100.0).round() as u64) % 100;

    let mut result = String::new();

    if is_negative {
        result.push_str("ลบ");
    }

    // Handle integer part
    if integer_part == 0 && satang == 0 {
        return Ok("ศูนย์บาทถ้วน".to_string());
    }

    if integer_part == 0 {
        // No baht, just satang
    } else if integer_part <= 999_999 {
        result.push_str(&group_to_thai(integer_part));
    } else {
        // Process in groups of 6 digits (ล้าน = million)
        let mut groups: Vec<u64> = Vec::new();
        let mut remaining = integer_part;
        // First group: the bottom 6 digits
        groups.push(remaining % 1_000_000);
        remaining /= 1_000_000;
        // Subsequent groups are also 6 digits each
        while remaining > 0 {
            groups.push(remaining % 1_000_000);
            remaining /= 1_000_000;
        }

        // Build from highest group to lowest
        for i in (1..groups.len()).rev() {
            let g = groups[i];
            if g > 0 {
                result.push_str(&group_to_thai(g));
                result.push_str("ล้าน");
            }
        }
        // Last group (ones through hundred-thousands)
        let last = groups[0];
        if last > 0 {
            result.push_str(&group_to_thai(last));
        }
    }

    if integer_part > 0 {
        result.push_str("บาท");
    }

    if satang == 0 {
        result.push_str("ถ้วน");
    } else {
        result.push_str(&group_to_thai(satang));
        result.push_str("สตางค์");
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        assert_eq!(codcel_bahttext(0.0).unwrap(), "ศูนย์บาทถ้วน");
    }

    #[test]
    fn test_one() {
        assert_eq!(codcel_bahttext(1.0).unwrap(), "หนึ่งบาทถ้วน");
    }

    #[test]
    fn test_ten() {
        assert_eq!(codcel_bahttext(10.0).unwrap(), "สิบบาทถ้วน");
    }

    #[test]
    fn test_eleven() {
        assert_eq!(codcel_bahttext(11.0).unwrap(), "สิบเอ็ดบาทถ้วน");
    }

    #[test]
    fn test_twenty_one() {
        assert_eq!(codcel_bahttext(21.0).unwrap(), "ยี่สิบเอ็ดบาทถ้วน");
    }

    #[test]
    fn test_hundred() {
        assert_eq!(codcel_bahttext(100.0).unwrap(), "หนึ่งร้อยบาทถ้วน");
    }

    #[test]
    fn test_with_decimals() {
        assert_eq!(
            codcel_bahttext(1234.50).unwrap(),
            "หนึ่งพันสองร้อยสามสิบสี่บาทห้าสิบสตางค์"
        );
    }

    #[test]
    fn test_negative() {
        assert_eq!(codcel_bahttext(-5.0).unwrap(), "ลบห้าบาทถ้วน");
    }

    #[test]
    fn test_million() {
        assert_eq!(codcel_bahttext(1_000_000.0).unwrap(), "หนึ่งล้านบาทถ้วน");
    }

    #[test]
    fn test_satang_only() {
        assert_eq!(codcel_bahttext(0.25).unwrap(), "ยี่สิบห้าสตางค์");
    }

    #[test]
    fn test_one_satang() {
        assert_eq!(codcel_bahttext(0.01).unwrap(), "หนึ่งสตางค์");
    }

    #[test]
    fn test_large_number() {
        assert_eq!(
            codcel_bahttext(1_234_567.89).unwrap(),
            "หนึ่งล้านสองแสนสามหมื่นสี่พันห้าร้อยหกสิบเจ็ดบาทแปดสิบเก้าสตางค์"
        );
    }

    #[test]
    fn test_twelve() {
        assert_eq!(codcel_bahttext(12.0).unwrap(), "สิบสองบาทถ้วน");
    }

    #[test]
    fn test_twenty() {
        assert_eq!(codcel_bahttext(20.0).unwrap(), "ยี่สิบบาทถ้วน");
    }

    #[test]
    fn test_thirty_five() {
        assert_eq!(codcel_bahttext(35.0).unwrap(), "สามสิบห้าบาทถ้วน");
    }

    #[test]
    fn test_two_million() {
        assert_eq!(codcel_bahttext(2_000_000.0).unwrap(), "สองล้านบาทถ้วน");
    }

    #[test]
    fn test_negative_with_satang() {
        assert_eq!(
            codcel_bahttext(-1234.56).unwrap(),
            "ลบหนึ่งพันสองร้อยสามสิบสี่บาทห้าสิบหกสตางค์"
        );
    }
}
