// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use super::forecast::*;
use crate::compensated_sum::CompensatedSum;
use crate::date_system::DateSemantics;
use std::error::Error;

/// Excel-compatible `FORECAST.ETS.STAT` — returns a statistical value for the ETS model.
///
/// **stat_type values:**
/// - 1 = Alpha (level smoothing parameter)
/// - 2 = Beta (trend smoothing parameter)
/// - 3 = Gamma (seasonal smoothing parameter)
/// - 4 = MASE (Mean Absolute Scaled Error)
/// - 5 = SMAPE (Symmetric Mean Absolute Percentage Error)
/// - 6 = MAE (Mean Absolute Error)
/// - 7 = RMSE (Root Mean Square Error)
/// - 8 = Step (seasonal period length)
///
/// **Parameter naming convention:**
/// - `model.alpha` = level smoothing → Excel "Alpha" (stat_type=1)
/// - `model.gamma` = trend smoothing → Excel "Beta" (stat_type=2)
/// - `model.beta` = seasonal smoothing → Excel "Gamma" (stat_type=3)
pub fn codcel_forecast_ets_stat(
    values: Vec<f64>,
    timeline: Vec<f64>,
    stat_type: i32,
    seasonality: Option<i32>,
    data_completion: Option<i32>,
    aggregation: Option<i32>,
    dates: DateSemantics,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if values.len() != timeline.len() {
        return Err("FORECAST.ETS.STAT: values and timeline must have the same length.".into());
    }
    if values.len() < 3 {
        return Err("FORECAST.ETS.STAT: at least 3 data points are required.".into());
    }
    if !(1..=8).contains(&stat_type) {
        return Err("FORECAST.ETS.STAT: stat_type must be between 1 and 8.".into());
    }

    let seasonality = seasonality.unwrap_or(1);
    let data_completion = data_completion.unwrap_or(1);
    let aggregation = aggregation.unwrap_or(1);

    if seasonality < 0 {
        return Err("FORECAST.ETS.STAT: seasonality must be 0, 1, or a positive integer.".into());
    }
    if data_completion != 0 && data_completion != 1 {
        return Err("FORECAST.ETS.STAT: data_completion must be 0 or 1.".into());
    }
    if !(1..=7).contains(&aggregation) {
        return Err("FORECAST.ETS.STAT: aggregation must be between 1 and 7.".into());
    }

    let (proc_values, proc_timeline, month_day) =
        preprocess_data(&values, &timeline, data_completion, aggregation, dates)?;

    let n = proc_values.len();
    if n < 3 {
        return Err("FORECAST.ETS.STAT: insufficient data after preprocessing.".into());
    }

    let season_length = match seasonality {
        0 => 0,
        1 => detect_seasonality(&proc_values),
        s => {
            let s = s as usize;
            if n < 2 * s {
                return Err("FORECAST.ETS.STAT: need at least 2 complete seasonal periods.".into());
            }
            s
        }
    };

    // For stat_type 8 (Step), return the step size in original timeline units.
    if stat_type == 8 {
        let mut pairs: Vec<(f64, f64)> = timeline
            .iter()
            .copied()
            .zip(values.iter().copied())
            .collect();
        pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        let (orig_timeline, _) = aggregate_duplicates(&pairs, aggregation);

        if month_day > 0 {
            // When month-day detection fires, the timeline was converted to month-space.
            // Compute step in month-space for accuracy.
            let month_step = compute_min_step(&proc_timeline)?;
            if month_step < 1.5 {
                // Pure monthly data: compute_min_step on original dates is distorted by
                // February (28 or 29 days). Use the first difference instead, which gives
                // the number of days in the first month of the data.
                if orig_timeline.len() >= 2 {
                    return Ok(orig_timeline[1] - orig_timeline[0]);
                }
            }
        }
        // For non-monthly or non-month-day-detected data, use compute_min_step
        // on the original timeline (works correctly for quarterly, yearly, etc.)
        let orig_step = compute_min_step(&orig_timeline)?;
        return Ok(orig_step);
    }

    let step = compute_min_step(&proc_timeline)?;
    let b_eds = season_length < 2;

    // Fit the ETS/EDS model (same optimizer as FORECAST.ETS)
    let mut model = EtsModel::new(&proc_values, &proc_timeline, step, season_length, b_eds);
    model.init_data();
    model.calc_alpha_beta_gamma();
    // The EDS Nelder-Mead optimizer doesn't call refill(), so we need it for EDS.
    // The ETS CMLE optimizer (optimize_ets_innovations) already fills all arrays;
    // calling refill() would overwrite innovations-form states with observation-form.
    if b_eds {
        model.refill();
    }

    match stat_type {
        // Alpha: level smoothing parameter
        1 => Ok(model.alpha),
        // Beta: trend smoothing (model.gamma in Codcel/LO convention)
        2 => Ok(model.gamma),
        // Gamma: seasonal smoothing (model.beta in Codcel/LO convention)
        // For EDS (non-seasonal), return 0.0
        3 => Ok(if b_eds { 0.0 } else { model.beta }),
        // MASE, SMAPE, MAE, RMSE: compute from one-step-ahead residuals
        4..=7 => {
            // Residuals: forecast[i] - values[i] for i=1..n-1
            // (index 0 excluded since forecast[0] = values[0])
            let count = (model.n - 1) as f64;

            let mut sum_abs_err = CompensatedSum::new();
            let mut sum_sq_err = CompensatedSum::new();
            let mut sum_smape = CompensatedSum::new();

            for i in 1..model.n {
                let err = model.forecast[i] - model.values[i];
                let abs_err = err.abs();
                sum_abs_err.add(abs_err);
                sum_sq_err.add(err * err);

                let denom = (model.values[i].abs() + model.forecast[i].abs()) / 2.0;
                if denom > 0.0 {
                    sum_smape.add(abs_err / denom);
                }
            }

            let mae = sum_abs_err.total() / count;

            match stat_type {
                4 => {
                    // MASE = MAE / mean(|Y[i] - Y[i-1]|)
                    let mut sum_naive = CompensatedSum::new();
                    for i in 1..model.n {
                        sum_naive.add((model.values[i] - model.values[i - 1]).abs());
                    }
                    let mean_naive = sum_naive.total() / count;
                    if mean_naive > 0.0 {
                        Ok(mae / mean_naive)
                    } else {
                        Ok(0.0)
                    }
                }
                5 => {
                    // SMAPE
                    Ok(sum_smape.total() / count)
                }
                6 => {
                    // MAE
                    Ok(mae)
                }
                7 => {
                    // RMSE = sqrt(MSE) where MSE = SSE/(n-1)
                    Ok(crate::portable_math::sqrt(sum_sq_err.total() / count))
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every expectation here pins Excel's own serial convention, so bind it once
    /// rather than threading `DateSemantics` through each call. Shadows the glob
    /// import from `use super::*`.
    fn codcel_forecast_ets_stat(
        values: Vec<f64>,
        timeline: Vec<f64>,
        stat_type: i32,
        seasonality: Option<i32>,
        data_completion: Option<i32>,
        aggregation: Option<i32>,
    ) -> Result<f64, Box<dyn Error + Send + Sync>> {
        super::codcel_forecast_ets_stat(
            values,
            timeline,
            stat_type,
            seasonality,
            data_completion,
            aggregation,
            DateSemantics::EXCEL_1900,
        )
    }

    fn quarterly_values() -> Vec<f64> {
        vec![
            110.0, 130.0, 150.0, 145.0, 130.0, 150.0, 170.0, 165.0, 150.0, 170.0, 190.0, 185.0,
        ]
    }

    fn quarterly_timeline() -> Vec<f64> {
        vec![
            43831.0, 43922.0, 44013.0, 44105.0, 44197.0, 44287.0, 44378.0, 44470.0, 44562.0,
            44652.0, 44743.0, 44835.0,
        ]
    }

    #[test]
    fn test_stat_alpha_seasonal() {
        let result = codcel_forecast_ets_stat(
            quarterly_values(),
            quarterly_timeline(),
            1,
            Some(4),
            None,
            None,
        )
        .unwrap();
        assert!(
            (0.0..=1.0).contains(&result),
            "Alpha should be in [0, 1], got {result}"
        );
    }

    #[test]
    fn test_stat_beta_seasonal() {
        let result = codcel_forecast_ets_stat(
            quarterly_values(),
            quarterly_timeline(),
            2,
            Some(4),
            None,
            None,
        )
        .unwrap();
        assert!(
            (0.0..=1.0).contains(&result),
            "Beta should be in [0, 1], got {result}"
        );
    }

    #[test]
    fn test_stat_gamma_seasonal() {
        let result = codcel_forecast_ets_stat(
            quarterly_values(),
            quarterly_timeline(),
            3,
            Some(4),
            None,
            None,
        )
        .unwrap();
        assert!(
            (0.0..=1.0).contains(&result),
            "Gamma should be in [0, 1], got {result}"
        );
    }

    #[test]
    fn test_stat_gamma_no_seasonality() {
        // Gamma should be 0 when there's no seasonality
        let result = codcel_forecast_ets_stat(
            quarterly_values(),
            quarterly_timeline(),
            3,
            Some(0),
            None,
            None,
        )
        .unwrap();
        assert!(
            (result - 0.0).abs() < 1e-10,
            "Gamma should be 0 for non-seasonal data, got {result}"
        );
    }

    #[test]
    fn test_stat_mase() {
        let result = codcel_forecast_ets_stat(
            quarterly_values(),
            quarterly_timeline(),
            4,
            Some(4),
            None,
            None,
        )
        .unwrap();
        assert!(
            result >= 0.0 && result.is_finite(),
            "MASE should be non-negative and finite, got {result}"
        );
    }

    #[test]
    fn test_stat_smape() {
        let result = codcel_forecast_ets_stat(
            quarterly_values(),
            quarterly_timeline(),
            5,
            Some(4),
            None,
            None,
        )
        .unwrap();
        assert!(
            (0.0..=2.0).contains(&result),
            "SMAPE should be in [0, 2], got {result}"
        );
    }

    #[test]
    fn test_stat_mae() {
        let result = codcel_forecast_ets_stat(
            quarterly_values(),
            quarterly_timeline(),
            6,
            Some(4),
            None,
            None,
        )
        .unwrap();
        assert!(
            result >= 0.0 && result.is_finite(),
            "MAE should be non-negative and finite, got {result}"
        );
    }

    #[test]
    fn test_stat_rmse() {
        let result = codcel_forecast_ets_stat(
            quarterly_values(),
            quarterly_timeline(),
            7,
            Some(4),
            None,
            None,
        )
        .unwrap();
        assert!(
            result >= 0.0 && result.is_finite(),
            "RMSE should be non-negative and finite, got {result}"
        );
    }

    #[test]
    fn test_stat_rmse_gte_mae() {
        // RMSE >= MAE always holds
        let mae = codcel_forecast_ets_stat(
            quarterly_values(),
            quarterly_timeline(),
            6,
            Some(4),
            None,
            None,
        )
        .unwrap();
        let rmse = codcel_forecast_ets_stat(
            quarterly_values(),
            quarterly_timeline(),
            7,
            Some(4),
            None,
            None,
        )
        .unwrap();
        assert!(
            rmse >= mae - 1e-10,
            "RMSE ({rmse}) should be >= MAE ({mae})"
        );
    }

    #[test]
    fn test_stat_step_returns_timeline_step() {
        // stat_type 8 returns the minimum step between timeline points,
        // NOT the seasonal period. For quarterly data with ~90-day gaps,
        // the step is ~90 regardless of seasonality setting.
        let result = codcel_forecast_ets_stat(
            quarterly_values(),
            quarterly_timeline(),
            8,
            Some(4),
            None,
            None,
        )
        .unwrap();
        assert!(
            result > 80.0 && result < 100.0,
            "Step should be ~90 for quarterly dates, got {result}"
        );

        // Same result regardless of seasonality
        let result_no_seas = codcel_forecast_ets_stat(
            quarterly_values(),
            quarterly_timeline(),
            8,
            Some(0),
            None,
            None,
        )
        .unwrap();
        assert!(
            (result - result_no_seas).abs() < 1e-10,
            "Step should be same regardless of seasonality"
        );
    }

    #[test]
    fn test_stat_step_simple_timeline() {
        // Simple timeline with step=1
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let timeline = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let result = codcel_forecast_ets_stat(values, timeline, 8, Some(0), None, None).unwrap();
        assert!(
            (result - 1.0).abs() < 1e-10,
            "Step should be 1.0 for unit timeline, got {result}"
        );
    }

    #[test]
    fn test_stat_step_monthly_dates() {
        // Monthly dates on the 1st: Jan 2021 through Dec 2022 (24 points)
        // Excel serial dates for 1st of each month
        let timeline = vec![
            44197.0, 44228.0, 44256.0, 44287.0, 44317.0, 44348.0, 44378.0, 44409.0, 44440.0,
            44470.0, 44501.0, 44531.0, 44562.0, 44593.0, 44621.0, 44652.0, 44682.0, 44713.0,
            44743.0, 44774.0, 44805.0, 44835.0, 44866.0, 44896.0,
        ];
        let values: Vec<f64> = (1..=24).map(|i| 200.0 + i as f64 * 5.0).collect();
        let result = codcel_forecast_ets_stat(values, timeline, 8, None, None, None).unwrap();
        assert!(
            (result - 31.0).abs() < 1e-10,
            "Step should be 31.0 for monthly dates, got {result}"
        );
    }

    #[test]
    fn test_stat_step_quarterly_dates() {
        // Quarterly dates: exact test data from Excel
        let result = codcel_forecast_ets_stat(
            quarterly_values(),
            quarterly_timeline(),
            8,
            None,
            None,
            None,
        )
        .unwrap();
        assert!(
            (result - 90.0).abs() < 1e-10,
            "Step should be 90.0 for quarterly dates, got {result}"
        );
    }

    #[test]
    fn test_stat_invalid_stat_type() {
        let result = codcel_forecast_ets_stat(
            quarterly_values(),
            quarterly_timeline(),
            0,
            None,
            None,
            None,
        );
        assert!(result.is_err());

        let result = codcel_forecast_ets_stat(
            quarterly_values(),
            quarterly_timeline(),
            9,
            None,
            None,
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_stat_mismatched_arrays() {
        let result =
            codcel_forecast_ets_stat(vec![1.0, 2.0, 3.0], vec![1.0, 2.0], 1, None, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_stat_insufficient_data() {
        let result = codcel_forecast_ets_stat(vec![1.0], vec![1.0], 1, None, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_stat_eds_linear_trend() {
        // Linear data: EDS mode
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let timeline = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];

        let alpha =
            codcel_forecast_ets_stat(values.clone(), timeline.clone(), 1, Some(0), None, None)
                .unwrap();
        let beta =
            codcel_forecast_ets_stat(values.clone(), timeline.clone(), 2, Some(0), None, None)
                .unwrap();
        let gamma =
            codcel_forecast_ets_stat(values.clone(), timeline.clone(), 3, Some(0), None, None)
                .unwrap();
        let mae =
            codcel_forecast_ets_stat(values.clone(), timeline.clone(), 6, Some(0), None, None)
                .unwrap();
        let rmse =
            codcel_forecast_ets_stat(values.clone(), timeline.clone(), 7, Some(0), None, None)
                .unwrap();

        assert!((0.0..=1.0).contains(&alpha), "Alpha={alpha}");
        assert!((0.0..=1.0).contains(&beta), "Beta={beta}");
        assert!(
            (gamma - 0.0).abs() < 1e-10,
            "Gamma should be 0 for EDS, got {gamma}"
        );
        assert!(mae.is_finite(), "MAE should be finite, got {mae}");
        assert!(rmse.is_finite(), "RMSE should be finite, got {rmse}");
    }

    #[test]
    fn test_stat_defaults() {
        // All optional params default
        let result = codcel_forecast_ets_stat(
            quarterly_values(),
            quarterly_timeline(),
            1,
            None,
            None,
            None,
        )
        .unwrap();
        assert!(
            (0.0..=1.0).contains(&result),
            "Alpha with defaults should be in [0, 1], got {result}"
        );
    }

    #[test]
    fn test_stat_invalid_data_completion() {
        let result = codcel_forecast_ets_stat(
            quarterly_values(),
            quarterly_timeline(),
            1,
            None,
            Some(2),
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_stat_invalid_aggregation() {
        let result = codcel_forecast_ets_stat(
            quarterly_values(),
            quarterly_timeline(),
            1,
            None,
            None,
            Some(8),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_stat_invalid_seasonality() {
        let result = codcel_forecast_ets_stat(
            quarterly_values(),
            quarterly_timeline(),
            1,
            Some(-1),
            None,
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_stat_constant_data() {
        // Constant data: test that all stat types return finite values
        let values = vec![5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0];
        let timeline = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];

        let mae =
            codcel_forecast_ets_stat(values.clone(), timeline.clone(), 6, Some(0), None, None)
                .unwrap();
        let rmse =
            codcel_forecast_ets_stat(values.clone(), timeline.clone(), 7, Some(0), None, None)
                .unwrap();
        let smape =
            codcel_forecast_ets_stat(values.clone(), timeline.clone(), 5, Some(0), None, None)
                .unwrap();

        assert!(mae.is_finite(), "MAE should be finite, got {mae}");
        assert!(rmse.is_finite(), "RMSE should be finite, got {rmse}");
        assert!(smape.is_finite(), "SMAPE should be finite, got {smape}");
    }
}
