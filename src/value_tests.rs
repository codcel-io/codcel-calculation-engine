// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::value::*;
use crate::value_format::ValueFormat;
use chrono::{NaiveTime, TimeZone, Utc};

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a default ValueFormat for testing
    fn default_value_format() -> ValueFormat {
        ValueFormat {
            use_excel_rounding: true,
            ..Default::default()
        }
    }

    #[test]
    fn test_value_creation() {
        // Test creating different Value variants
        let f64_value = f64(42.5);
        let i32_value = i32(42);
        let string_value = string("test".to_string());
        let bool_value = bool(true);
        let none_value = none();

        // Print results
        println!("f64_value: {:?}", f64_value);
        println!("i32_value: {:?}", i32_value);
        println!("string_value: {:?}", string_value);
        println!("bool_value: {:?}", bool_value);
        println!("none_value: {:?}", none_value);

        // Assert the correct types were created
        assert!(matches!(f64_value, Value::F64(_)));
        assert!(matches!(i32_value, Value::I32(_)));
        assert!(matches!(string_value, Value::String(_)));
        assert!(matches!(bool_value, Value::Bool(_)));
        assert!(matches!(none_value, Value::None));
    }

    #[test]
    fn test_value_equality() {
        // Test equality between values
        let value1 = f64(42.0);
        let value2 = f64(42.0);
        let value3 = f64(43.0);
        let value4 = i32(42);

        let result1 = value1 == value2;
        let result2 = value1 == value3;
        let result3 = value1 == value4;

        println!("result1: {}", result1);
        println!("result2: {}", result2);
        println!("result3: {}", result3);

        assert!(result1);
        assert!(!result2);
        assert!(!result3);
    }

    #[test]
    fn test_value_comparison() {
        // Test comparison between values
        let value1 = f64(42.0);
        let value2 = f64(43.0);

        let result1 = value1 < value2;
        let result2 = value1 > value2;

        println!("result1: {}", result1);
        println!("result2: {}", result2);

        assert!(result1);
        assert!(!result2);
    }

    #[test]
    fn test_f64_conversion() {
        let value_format = default_value_format();

        // Test f64 conversion from different Value types
        let f64_value = f64(42.5);
        let i32_value = i32(42);
        let string_value = string("42.5".to_string());
        let bool_value = bool(true);

        let result1 = f64_value.f64(&value_format);
        let result2 = i32_value.f64(&value_format);
        let result3 = string_value.f64(&value_format);
        let result4 = bool_value.f64(&value_format);

        println!("result1: {:?}", result1);
        println!("result2: {:?}", result2);
        println!("result3: {:?}", result3);
        println!("result4: {:?}", result4);

        assert_eq!(result1.unwrap(), 42.5);
        assert_eq!(result2.unwrap(), 42.0);
        assert_eq!(result3.unwrap(), 42.5);
        assert_eq!(result4.unwrap(), 1.0);
    }

    #[test]
    fn test_i32_conversion() {
        let value_format = default_value_format();

        // Test i32 conversion from different Value types
        let f64_value = f64(42.5);
        let i32_value = i32(42);
        let string_value = string("42".to_string());
        let bool_value = bool(true);

        let result1 = f64_value.i32(&value_format);
        let result2 = i32_value.i32(&value_format);
        let result3 = string_value.i32(&value_format);
        let result4 = bool_value.i32(&value_format);

        println!("result1: {:?}", result1);
        println!("result2: {:?}", result2);
        println!("result3: {:?}", result3);
        println!("result4: {:?}", result4);

        assert_eq!(result1.unwrap(), 42); // Truncates from 42.5
        assert_eq!(result2.unwrap(), 42);
        assert_eq!(result3.unwrap(), 42);
        assert_eq!(result4.unwrap(), 1);
    }

    #[test]
    fn test_string_conversion() {
        let value_format = default_value_format();

        // Test string conversion from different Value types
        let f64_value = f64(42.5);
        let i32_value = i32(42);
        let string_value = string("test".to_string());
        let bool_value = bool(true);

        let result1 = f64_value.string(&value_format);
        let result2 = i32_value.string(&value_format);
        let result3 = string_value.string(&value_format);
        let result4 = bool_value.string(&value_format);

        println!("result1: {:?}", result1);
        println!("result2: {:?}", result2);
        println!("result3: {:?}", result3);
        println!("result4: {:?}", result4);

        assert_eq!(result1.unwrap(), "42.5");
        assert_eq!(result2.unwrap(), "42");
        assert_eq!(result3.unwrap(), "test");
        assert_eq!(result4.unwrap(), "TRUE");
    }

    #[test]
    fn test_bool_conversion() {
        let value_format = default_value_format();

        // Test bool conversion from different Value types
        let f64_value_true = f64(1.0);
        let f64_value_false = f64(0.0);
        let i32_value_true = i32(1);
        let i32_value_false = i32(0);
        let string_value_true = string("true".to_string());
        let string_value_false = string("false".to_string());
        let bool_value_true = bool(true);
        let bool_value_false = bool(false);

        let result1 = f64_value_true.bool(&value_format);
        let result2 = f64_value_false.bool(&value_format);
        let result3 = i32_value_true.bool(&value_format);
        let result4 = i32_value_false.bool(&value_format);
        let result5 = string_value_true.bool(&value_format);
        let result6 = string_value_false.bool(&value_format);
        let result7 = bool_value_true.bool(&value_format);
        let result8 = bool_value_false.bool(&value_format);

        println!("result1: {:?}", result1);
        println!("result2: {:?}", result2);
        println!("result3: {:?}", result3);
        println!("result4: {:?}", result4);
        println!("result5: {:?}", result5);
        println!("result6: {:?}", result6);
        println!("result7: {:?}", result7);
        println!("result8: {:?}", result8);

        assert!(result1.unwrap());
        assert!(!result2.unwrap());
        assert!(result3.unwrap());
        assert!(!result4.unwrap());
        assert!(result5.unwrap());
        assert!(!result6.unwrap());
        assert!(result7.unwrap());
        assert!(!result8.unwrap());
    }

    #[test]
    fn test_date_time_conversion() {
        let value_format = default_value_format();

        // Create a DateTime value
        let dt = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
        let dt_value = date_time(dt);
        let string_value = string("2023-01-01T12:00:00Z".to_string());

        let result1 = dt_value.date_time(&value_format);
        let result2 = string_value.date_time(&value_format);

        println!("result1: {:?}", result1);
        println!("result2: {:?}", result2);

        assert_eq!(result1.unwrap(), dt);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_time_conversion() {
        let value_format = default_value_format();

        // Create a Time value
        let test_time = NaiveTime::from_hms_opt(12, 0, 0).unwrap();
        let time_value = time(test_time);
        let string_value = string("12:00:00".to_string());

        let result1 = time_value.time(&value_format);
        let result2 = string_value.time(&value_format);

        println!("result1: {:?}", result1);
        println!("result2: {:?}", result2);

        assert_eq!(result1.unwrap(), test_time);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_option_conversions() {
        let value_format = default_value_format();

        // Test option conversions
        let some_f64_value = some_f64(42.5);
        let some_i32_value = some_i32(42);
        let some_string_value = some_string("test".to_string());
        let some_bool_value = some_bool(true);
        let none_value = none();

        let result1 = some_f64_value.option_f64(&value_format);
        let result2 = some_i32_value.option_i32(&value_format);
        let result3 = some_string_value.option_string(&value_format);
        let result4 = some_bool_value.option_bool(&value_format);
        let result5 = none_value.option_f64(&value_format);

        println!("result1: {:?}", result1);
        println!("result2: {:?}", result2);
        println!("result3: {:?}", result3);
        println!("result4: {:?}", result4);
        println!("result5: {:?}", result5);

        assert_eq!(result1.unwrap(), Some(42.5));
        assert_eq!(result2.unwrap(), Some(42));
        assert_eq!(result3.unwrap(), Some("test".to_string()));
        assert_eq!(result4.unwrap(), Some(true));
        assert_eq!(result5.unwrap(), None);
    }

    #[test]
    fn test_vec_conversions() {
        let value_format = default_value_format();

        // Test vector conversions
        let vec_f64_values = vec_f64(vec![1.0, 2.0, 3.0]);
        let vec_i32_values = vec_i32(vec![1, 2, 3]);
        let vec_string_values = vec_string(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        let vec_bool_values = vec_bool(vec![true, false, true]);

        let result1 = vec_f64_values.vec_f64(&value_format);
        let result2 = vec_i32_values.vec_i32(&value_format);
        let result3 = vec_string_values.vec_string(&value_format);
        let result4 = vec_bool_values.vec_bool(&value_format);

        println!("result1: {:?}", result1);
        println!("result2: {:?}", result2);
        println!("result3: {:?}", result3);
        println!("result4: {:?}", result4);

        assert_eq!(result1.unwrap(), vec![1.0, 2.0, 3.0]);
        assert_eq!(result2.unwrap(), vec![1, 2, 3]);
        assert_eq!(
            result3.unwrap(),
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
        assert_eq!(result4.unwrap(), vec![true, false, true]);
    }

    #[test]
    fn test_area_conversions() {
        let value_format = default_value_format();

        // Test area conversions
        let area_f64_values = area_f64(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let area_i32_values = area_i32(vec![vec![1, 2], vec![3, 4]]);
        let area_string_values = area_string(vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string(), "d".to_string()],
        ]);
        let area_bool_values = area_bool(vec![vec![true, false], vec![false, true]]);

        let result1 = area_f64_values.area_of_f64(true, &value_format);
        let result2 = area_i32_values.area_of_i32(true, &value_format);
        let result3 = area_string_values.area_of_string(true, &value_format);
        let result4 = area_bool_values.area_of_bool(true, &value_format);

        println!("result1: {:?}", result1);
        println!("result2: {:?}", result2);
        println!("result3: {:?}", result3);
        println!("result4: {:?}", result4);

        assert_eq!(result1.unwrap(), vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        assert_eq!(result2.unwrap(), vec![vec![1, 2], vec![3, 4]]);
        assert_eq!(
            result3.unwrap(),
            vec![
                vec!["a".to_string(), "b".to_string()],
                vec!["c".to_string(), "d".to_string()]
            ]
        );
        assert_eq!(result4.unwrap(), vec![vec![true, false], vec![false, true]]);
    }

    #[test]
    fn test_is_functions() {
        // Test is_* functions
        let f64_value = f64(42.5);
        let string_value = string("test".to_string());
        let vec_value = vec_f64(vec![1.0, 2.0, 3.0]);
        let area_value = area_f64(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let dt = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
        let dt_value = date_time(dt);
        let test_time = NaiveTime::from_hms_opt(12, 0, 0).unwrap();
        let time_value = time(test_time);

        let result1 = f64_value.is_count_number_type();
        let result2 = string_value.is_single_string();
        let result3 = vec_value.is_array();
        let result4 = area_value.is_area();
        let result5 = dt_value.is_datetime();
        let result6 = time_value.is_time();
        let result7 = string_value.is_string();

        println!("result1: {}", result1);
        println!("result2: {}", result2);
        println!("result3: {}", result3);
        println!("result4: {}", result4);
        println!("result5: {}", result5);
        println!("result6: {}", result6);
        println!("result7: {}", result7);

        assert!(result1);
        assert!(result2);
        assert!(result3);
        assert!(result4);
        assert!(result5);
        assert!(result6);
        assert!(result7);
    }

    #[test]
    fn test_to_single_value() {
        // Test to_single_value function
        let vec_value = vec_f64(vec![1.0, 2.0, 3.0]);
        let area_value = area_f64(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);

        let result1 = vec_value.to_single_value();
        let result2 = area_value.to_single_value();

        println!("result1: {:?}", result1);
        println!("result2: {:?}", result2);

        assert!(matches!(result1, Value::F64(_)));
        assert!(matches!(result2, Value::F64(_)));
    }

    #[test]
    fn test_flatten_functions() {
        let value_format = default_value_format();

        // Test flatten functions
        let area_f64_values = area_f64(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);

        let result1 = area_f64_values.to_flatterned_vec_f64(&value_format);

        println!("result1: {:?}", result1);

        assert_eq!(result1.unwrap(), vec![1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_vec_value_to_vec_functions() {
        let value_format = default_value_format();

        // Test vec_value_to_vec_* functions
        let values = vec![f64(1.0), f64(2.0), f64(3.0)];

        let result1 = vec_value_to_vec_f64(values.clone(), &value_format);
        let result2 = vec_value_to_vec_i32(values.clone(), &value_format);
        let result3 = vec_value_to_vec_string(values.clone(), &value_format);
        let result4 = vec_value_to_vec_value(values.clone());
        let result5 = vec_value_to_vec_boolean(values.clone(), false, &value_format);

        println!("result1: {:?}", result1);
        println!("result2: {:?}", result2);
        println!("result3: {:?}", result3);
        println!("result4: {:?}", result4);
        println!("result5: {:?}", result5);

        assert_eq!(result1.unwrap(), vec![1.0, 2.0, 3.0]);
        assert_eq!(result2.unwrap(), vec![1, 2, 3]);
        assert_eq!(
            result3.unwrap(),
            vec!["1".to_string(), "2".to_string(), "3".to_string()]
        );
        assert_eq!(result4.unwrap().len(), 3);
        assert_eq!(result5.unwrap(), vec![true, true, true]);
    }

    #[test]
    fn test_invalid_string_to_f64() {
        // Test that an invalid string cannot be converted to f64
        let value_format = default_value_format();

        let string_value = string("not a number".to_string());

        let result = string_value.f64(&value_format);

        println!("result: {:?}", result);

        assert!(result.is_err());
    }

    #[test]
    fn test_hash_implementation() {
        use std::collections::HashMap;

        // Test Hash implementation
        let mut map = HashMap::new();
        let key1 = f64(42.0);
        let key2 = f64(42.0);
        let key3 = f64(43.0);

        map.insert(key1, "value1");

        let result1 = map.contains_key(&key2);
        let result2 = map.contains_key(&key3);

        println!("result1: {}", result1);
        println!("result2: {}", result2);

        assert!(result1);
        assert!(!result2);
    }

    #[test]
    fn test_raw_string() {
        // Test raw_string function
        let f64_value = f64(42.5);
        let i32_value = i32(42);
        let string_value = string("test".to_string());
        let bool_value = bool(true);

        let result1 = f64_value.raw_string();
        let result2 = i32_value.raw_string();
        let result3 = string_value.raw_string();
        let result4 = bool_value.raw_string();

        println!("result1: {}", result1);
        println!("result2: {}", result2);
        println!("result3: {}", result3);
        println!("result4: {}", result4);

        assert_eq!(result1, "42.5");
        assert_eq!(result2, "42");
        assert_eq!(result3, "test");
        assert_eq!(result4, "TRUE");
    }
}
