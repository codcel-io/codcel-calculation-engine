// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use super::forecast::*;
use crate::date_system::DateSemantics;
use std::error::Error;

/// Excel-compatible `FORECAST.ETS.SEASONALITY` — returns the detected seasonal period.
///
/// Returns the length of the repetitive pattern Excel detects for the given time series.
/// Returns 0 if no seasonality is detected (matching Excel behavior).
///
/// **Parameters:**
/// - `values`: historical data array.
/// - `timeline`: time periods array matching values.
/// - `data_completion`: 0=missing as zero, 1=interpolate (default).
/// - `aggregation`: 1=AVERAGE (default), 2=COUNT, 3=COUNTA, 4=MAX, 5=MEDIAN, 6=MIN, 7=SUM.
pub fn codcel_forecast_ets_seasonality(
    values: Vec<f64>,
    timeline: Vec<f64>,
    data_completion: Option<i32>,
    aggregation: Option<i32>,
    dates: DateSemantics,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    if values.len() != timeline.len() {
        return Err(
            "FORECAST.ETS.SEASONALITY: values and timeline must have the same length.".into(),
        );
    }
    if values.len() < 3 {
        return Err("FORECAST.ETS.SEASONALITY: at least 3 data points are required.".into());
    }

    let data_completion = data_completion.unwrap_or(1);
    let aggregation = aggregation.unwrap_or(1);

    if data_completion != 0 && data_completion != 1 {
        return Err("FORECAST.ETS.SEASONALITY: data_completion must be 0 or 1.".into());
    }
    if !(1..=7).contains(&aggregation) {
        return Err("FORECAST.ETS.SEASONALITY: aggregation must be between 1 and 7.".into());
    }

    let (proc_values, _proc_timeline, _month_day) =
        preprocess_data(&values, &timeline, data_completion, aggregation, dates)?;

    if proc_values.len() < 3 {
        return Err("FORECAST.ETS.SEASONALITY: insufficient data after preprocessing.".into());
    }

    let period = detect_seasonality(&proc_values);

    Ok(period as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every expectation here pins Excel's own serial convention, so bind it once
    /// rather than threading `DateSemantics` through each call. Shadows the glob
    /// import from `use super::*`.
    fn codcel_forecast_ets_seasonality(
        values: Vec<f64>,
        timeline: Vec<f64>,
        data_completion: Option<i32>,
        aggregation: Option<i32>,
    ) -> Result<i32, Box<dyn Error + Send + Sync>> {
        super::codcel_forecast_ets_seasonality(
            values,
            timeline,
            data_completion,
            aggregation,
            DateSemantics::EXCEL_1900,
        )
    }

    #[test]
    fn test_seasonality_quarterly_data() {
        let values = vec![
            110.0, 130.0, 150.0, 145.0, 130.0, 150.0, 170.0, 165.0, 150.0, 170.0, 190.0, 185.0,
        ];
        let timeline: Vec<f64> = (1..=12).map(|i| i as f64).collect();
        let result = codcel_forecast_ets_seasonality(values, timeline, None, None).unwrap();
        assert_eq!(result, 4, "Expected seasonality period 4, got {result}");
    }

    #[test]
    fn test_seasonality_constant_data() {
        let values = vec![5.0, 5.0, 5.0, 5.0, 5.0, 5.0];
        let timeline: Vec<f64> = (1..=6).map(|i| i as f64).collect();
        let result = codcel_forecast_ets_seasonality(values, timeline, None, None).unwrap();
        assert_eq!(
            result, 0,
            "Expected 0 (no seasonality) for constant data, got {result}"
        );
    }

    #[test]
    fn test_seasonality_linear_trend() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let timeline: Vec<f64> = (1..=8).map(|i| i as f64).collect();
        let result = codcel_forecast_ets_seasonality(values, timeline, None, None).unwrap();
        assert_eq!(
            result, 0,
            "Expected 0 (no seasonality) for linear trend, got {result}"
        );
    }

    #[test]
    fn test_seasonality_period_3() {
        let values = vec![10.0, 20.0, 30.0, 10.0, 20.0, 30.0, 10.0, 20.0, 30.0];
        let timeline: Vec<f64> = (1..=9).map(|i| i as f64).collect();
        let result = codcel_forecast_ets_seasonality(values, timeline, None, None).unwrap();
        assert_eq!(result, 3, "Expected seasonality period 3, got {result}");
    }

    #[test]
    fn test_seasonality_error_mismatched_arrays() {
        let result =
            codcel_forecast_ets_seasonality(vec![1.0, 2.0, 3.0], vec![1.0, 2.0], None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_seasonality_error_insufficient_data() {
        let result = codcel_forecast_ets_seasonality(vec![1.0], vec![1.0], None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_seasonality_error_invalid_data_completion() {
        let result = codcel_forecast_ets_seasonality(
            vec![1.0, 2.0, 3.0, 4.0],
            vec![1.0, 2.0, 3.0, 4.0],
            Some(2),
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_seasonality_error_invalid_aggregation() {
        let result = codcel_forecast_ets_seasonality(
            vec![1.0, 2.0, 3.0, 4.0],
            vec![1.0, 2.0, 3.0, 4.0],
            None,
            Some(8),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_seasonality_with_duplicates() {
        let values = vec![
            110.0, 130.0, 150.0, 145.0, 130.0, 150.0, 170.0, 165.0, 150.0, 170.0, 190.0, 185.0,
            200.0,
        ];
        let timeline = vec![
            1.0, 2.0, 3.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0,
        ];
        let result = codcel_forecast_ets_seasonality(values, timeline, None, Some(1)).unwrap();
        assert!(
            result >= 1,
            "Result should be a positive integer, got {result}"
        );
    }

    #[test]
    fn test_seasonality_with_missing_data() {
        let values = vec![
            110.0, 130.0, 145.0, 130.0, 170.0, 165.0, 150.0, 190.0, 185.0,
        ];
        let timeline = vec![1.0, 2.0, 4.0, 5.0, 7.0, 8.0, 9.0, 11.0, 12.0];
        let result = codcel_forecast_ets_seasonality(values, timeline, Some(1), None).unwrap();
        assert!(
            result >= 1,
            "Result should be a positive integer, got {result}"
        );
    }

    #[test]
    fn test_seasonality_defaults() {
        let values = vec![
            110.0, 130.0, 150.0, 145.0, 130.0, 150.0, 170.0, 165.0, 150.0, 170.0, 190.0, 185.0,
        ];
        let timeline: Vec<f64> = (1..=12).map(|i| i as f64).collect();
        // With all defaults (data_completion=1, aggregation=1)
        let result = codcel_forecast_ets_seasonality(values, timeline, None, None).unwrap();
        assert_eq!(result, 4, "Expected seasonality period 4 with defaults");
    }
}
