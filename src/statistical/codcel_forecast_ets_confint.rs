// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use super::forecast::*;
use crate::statistical::codcel_norm_dot_s_dot_inv::codcel_norm_dot_s_dot_inv;
use std::error::Error;

/// Excel-compatible `FORECAST.ETS.CONFINT` — confidence interval half-width for an ETS forecast.
///
/// Returns the half-width of the prediction interval at the given confidence level.
/// For example, if the forecast is 100 and CONFINT returns 5, the 95% CI is [95, 105].
pub fn codcel_forecast_ets_confint(
    target_date: f64,
    values: Vec<f64>,
    timeline: Vec<f64>,
    confidence_level: Option<f64>,
    seasonality: Option<i32>,
    data_completion: Option<i32>,
    aggregation: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if values.len() != timeline.len() {
        return Err("FORECAST.ETS.CONFINT: values and timeline must have the same length.".into());
    }
    if values.len() < 3 {
        return Err("FORECAST.ETS.CONFINT: at least 3 data points are required.".into());
    }

    let confidence_level = confidence_level.unwrap_or(0.95);
    if confidence_level <= 0.0 || confidence_level >= 1.0 {
        return Err(
            "FORECAST.ETS.CONFINT: confidence_level must be between 0 and 1 (exclusive).".into(),
        );
    }

    let seasonality = seasonality.unwrap_or(1);
    let data_completion = data_completion.unwrap_or(1);
    let aggregation = aggregation.unwrap_or(1);

    if seasonality < 0 {
        return Err(
            "FORECAST.ETS.CONFINT: seasonality must be 0, 1, or a positive integer.".into(),
        );
    }
    if data_completion != 0 && data_completion != 1 {
        return Err("FORECAST.ETS.CONFINT: data_completion must be 0 or 1.".into());
    }
    if !(1..=7).contains(&aggregation) {
        return Err("FORECAST.ETS.CONFINT: aggregation must be between 1 and 7.".into());
    }

    let (proc_values, proc_timeline, month_day) =
        preprocess_data(&values, &timeline, data_completion, aggregation)?;

    let n = proc_values.len();
    if n < 3 {
        return Err("FORECAST.ETS.CONFINT: insufficient data after preprocessing.".into());
    }

    let season_length = match seasonality {
        0 => 0,
        1 => detect_seasonality(&proc_values),
        s => {
            let s = s as usize;
            if n < 2 * s {
                return Err(
                    "FORECAST.ETS.CONFINT: need at least 2 complete seasonal periods.".into(),
                );
            }
            s
        }
    };

    let step = compute_min_step(&proc_timeline)?;
    let b_eds = season_length < 2;

    // Convert target_date to month space if monthly dates detected
    let effective_target = if month_day > 0 {
        convert_x_to_months(target_date, month_day)
    } else {
        target_date
    };

    let last_x = proc_timeline[n - 1];
    if effective_target <= last_x {
        return Err(
            "FORECAST.ETS.CONFINT: target_date must be after the last timeline value.".into(),
        );
    }

    // Run the full ETS/EDS model.
    let mut model = EtsModel::new(&proc_values, &proc_timeline, step, season_length, b_eds);
    model.init_data();
    if b_eds {
        // EDS: use innovations NM concentrated MLE (same optimizer as FORECAST.ETS).
        // This finds optimal (alpha_innov, beta_innov) for the Hyndman variance formula.
        model.calc_alpha_beta_gamma();
    } else {
        // ETS (seasonal): use nested bisection .
        model.calc_alpha_beta_gamma_golden();
    }

    // Get confidence interval half-width
    let ci = get_confidence_interval(&model, effective_target, confidence_level)?;
    Ok(ci)
}

// =============================================================================
// Confidence interval computation
// =============================================================================

/// Compute the half-width of the prediction interval for a target date.
/// - EDS (non-seasonal): Hyndman ETS(A,A,N) analytical variance formula
/// - ETS (seasonal): Monte Carlo simulation with 1000 scenarios
fn get_confidence_interval(
    model: &EtsModel,
    target_date: f64,
    confidence_level: f64,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let last_x = model.timeline[model.n - 1];

    let z = codcel_norm_dot_s_dot_inv((1.0 + confidence_level) / 2.0)?;

    if model.b_eds {
        // EDS: Hyndman ETS(A,A,N) analytical prediction interval formula.
        //
        // For the innovations state-space model ETS(A,A,N):
        //   e[t] = Y[t] - (l[t] + b[t])
        //   l[t+1] = l[t] + b[t] + α * e[t]
        //   b[t+1] = b[t] + β * e[t]
        //
        // The h-step-ahead prediction variance is:
        //   Var(h) = σ² × Σ_{j=0}^{h-1} c_j²
        // where c_0 = 1, c_j = α + j × β  for j ≥ 1
        //
        // Hybrid variance estimation:
        //   When NM finds α + β > 1 (nonstationary/random-walk regime), the MLE
        //   residual variance is unreliable for prediction intervals. Instead, use
        //   the MSSD/2 estimator (mean squared successive differences / 2), which
        //   is the natural variance estimator for random walk data, combined with
        //   sqrt(h) step factors (random walk prediction interval).
        //   When α + β ≤ 1 (stationary regime), use the MLE residual MSE with
        //   the Hyndman step factors.
        //
        // Reference: Hyndman et al., "Forecasting with Exponential Smoothing",
        //            Table 6.3, ETS(A,A,N) model.
        let f_target = target_date - last_x;
        let steps_f = f_target / model.step_size;
        let h_floor = steps_f.floor() as usize;
        let h_ceil = steps_f.ceil() as usize;

        if h_ceil == 0 {
            return Ok(0.0);
        }

        // For EDS, the NM optimizer stores innovations-form parameters directly:
        //   model.alpha = α_innov (innovations level smoothing)
        //   model.gamma = β_innov (innovations trend smoothing)
        let alpha_innov = model.alpha;
        let beta_innov = model.gamma;

        // Choose variance estimator and step factor based on parameter regime.
        //
        // When α + β > 1 (nonstationary): both the MLE variance and Hyndman step
        // factors are unreliable. Use MSSD/2 with sqrt(h) step factors.
        //
        // When α + β ≈ 0 (degenerate/OLS): MLE variance is good for 1-step, but
        // Hyndman step factors give c_j = 0 for j≥1, so PI doesn't grow with h.
        // Use MLE variance with sqrt(h) step factors.
        //
        // Otherwise (stationary regime): use MLE variance with Hyndman step factors.
        let (sigma_sq, variance_factor): (f64, Box<dyn Fn(usize) -> f64>) =
            if alpha_innov + beta_innov > 1.0 {
                // Nonstationary regime: use MSSD/2 = Σ(y[i+1]-y[i])² / (2n)
                // with random walk step factor sqrt(h)
                let mut sum_d_sq = 0.0;
                for i in 0..model.n - 1 {
                    let d = model.values[i + 1] - model.values[i];
                    sum_d_sq += d * d;
                }
                let mssd_half = sum_d_sq / (2.0 * model.n as f64);
                (mssd_half, Box::new(|h: usize| h as f64))
            } else if (alpha_innov + beta_innov).abs() < 1e-8 {
                // Degenerate (0,0): MLE variance with sqrt(h) step factors
                (model.mse, Box::new(|h: usize| h as f64))
            } else {
                // Stationary regime: use MLE residual MSE with Hyndman factors
                let a = alpha_innov;
                let b = beta_innov;
                (
                    model.mse,
                    Box::new(move |h: usize| -> f64 {
                        let mut sum = 1.0;
                        for j in 1..h {
                            let c_j = a + (j as f64) * b;
                            sum += c_j * c_j;
                        }
                        sum
                    }),
                )
            };

        let rmse = crate::portable_math::sqrt(sigma_sq);
        let frac = steps_f - h_floor as f64;

        if frac.abs() < CF_MIN_ABC_RESOLUTION || h_floor == h_ceil {
            // Exact integer steps
            let h = if h_floor > 0 { h_floor } else { h_ceil };
            let factor = crate::portable_math::sqrt(variance_factor(h));
            Ok(z * rmse * factor)
        } else {
            // Fractional step: interpolate between floor and ceil
            let pi_floor = z * rmse * crate::portable_math::sqrt(variance_factor(h_floor.max(1)));
            let pi_ceil = z * rmse * crate::portable_math::sqrt(variance_factor(h_ceil));
            Ok(pi_floor + frac * (pi_ceil - pi_floor))
        }
    } else {
        // ETS (seasonal): Monte Carlo simulation with deterministic seed
        // uses 1000 random scenarios. We use a simple LCG PRNG
        // with a fixed seed for deterministic, reproducible results.
        let rmse = crate::portable_math::sqrt(model.mse);
        let steps_f = (target_date - last_x) / model.step_size;
        let n_steps = steps_f.ceil() as usize;
        if n_steps == 0 {
            return Ok(0.0);
        }
        get_ets_confidence_interval_monte_carlo(model, n_steps, confidence_level, rmse)
    }
}

/// Monte Carlo simulation for seasonal ETS prediction intervals.
/// Runs 1000 scenarios with random perturbations to estimate the CI.
/// Uses a deterministic PRNG seeded from the data for reproducibility.
fn get_ets_confidence_interval_monte_carlo(
    model: &EtsModel,
    n_steps: usize,
    confidence_level: f64,
    rmse: f64,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    const N_SCENARIOS: usize = 1000;

    // Simple LCG PRNG for deterministic results
    // seed from data to get consistent results for the same input
    let mut rng_state: u64 = 12345;
    for &v in &model.values {
        rng_state = rng_state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(v.to_bits());
    }

    let mut scenario_forecasts: Vec<f64> = Vec::with_capacity(N_SCENARIOS);

    for _scenario in 0..N_SCENARIOS {
        // Run the ETS model forward from the last state with random perturbations.
        // Each scenario generates a possible future path including random errors.
        let mut level = model.base[model.n - 1];
        let mut trend = model.trend[model.n - 1];
        // Copy the last m seasonal indices
        let mut seasonal: Vec<f64> = Vec::with_capacity(model.m);
        for j in 0..model.m {
            seasonal.push(model.per_idx[model.n - 1 - model.m + j]);
        }

        let mut actual_val = 0.0;
        for step in 1..=n_steps {
            let s_idx = (step - 1) % model.m;
            let point_forecast = level + trend + seasonal[s_idx];

            // Generate random deviation: RMSE * gaussinv(uniform(0.5, 1.0))
            // LCG step
            rng_state = rng_state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let u = 0.5 + 0.5 * ((rng_state >> 33) as f64 / (1u64 << 31) as f64);
            let u = u.clamp(0.501, 0.999); // avoid edge values
            let rand_dev = if let Ok(g) = codcel_norm_dot_s_dot_inv(u) {
                rmse * g
            } else {
                0.0
            };

            // The "actual" value in this scenario includes the random error
            actual_val = point_forecast + rand_dev;

            // Update state using the error (innovations form)
            let error = rand_dev;
            level = level + trend + model.alpha * error;
            trend += model.gamma * error;
            seasonal[s_idx] += model.beta * error;
        }

        scenario_forecasts.push(actual_val);
    }

    // Sort and compute percentiles
    scenario_forecasts.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let upper_pct = (1.0 + confidence_level) / 2.0;
    let median_pct = 0.5;

    let upper = percentile(&scenario_forecasts, upper_pct);
    let median = percentile(&scenario_forecasts, median_pct);

    Ok(upper - median)
}

/// Compute percentile using linear interpolation (Excel PERCENTILE.INC style).
fn percentile(sorted: &[f64], p: f64) -> f64 {
    let n = sorted.len();
    if n == 0 {
        return 0.0;
    }
    if n == 1 {
        return sorted[0];
    }
    let rank = p * (n - 1) as f64;
    let lower = rank.floor() as usize;
    let upper = rank.ceil() as usize;
    let frac = rank - lower as f64;
    if lower == upper || upper >= n {
        sorted[lower.min(n - 1)]
    } else {
        sorted[lower] + frac * (sorted[upper] - sorted[lower])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confint_debug_quarterly_noseas() {
        let values = vec![
            110.0, 130.0, 150.0, 145.0, 130.0, 150.0, 170.0, 165.0, 150.0, 170.0, 190.0, 185.0,
        ];
        let timeline = vec![
            43831.0, 43922.0, 44013.0, 44105.0, 44197.0, 44287.0, 44378.0, 44470.0, 44562.0,
            44652.0, 44743.0, 44835.0,
        ];
        let (proc_values, proc_timeline, _month_day) =
            preprocess_data(&values, &timeline, 1, 1).unwrap();
        let step = compute_min_step(&proc_timeline).unwrap();

        let z = codcel_norm_dot_s_dot_inv(0.975).unwrap();
        let target_rmse = 25.070267517764044 / z;
        let target_mse = target_rmse * target_rmse;
        println!("Target MSE(n-1)={target_mse}, RMSE={target_rmse}");

        // Fine scan of alpha with gamma=0 (fixed initial states)
        let mut model = EtsModel::new(&proc_values, &proc_timeline, step, 0, true);
        model.init_data();
        let mut best_alpha = 0.0;
        let mut best_mse = f64::MAX;
        for i in 0..=10000 {
            let a = i as f64 / 10000.0;
            model.alpha = a;
            model.gamma = 0.0;
            model.refill();
            if model.mse < best_mse {
                best_mse = model.mse;
                best_alpha = a;
            }
            if (model.mse - target_mse).abs() < 0.01 {
                println!(
                    "CLOSE! alpha={a}, MSE={:.6}, target={target_mse:.6}",
                    model.mse
                );
            }
        }
        println!("Best: alpha={best_alpha}, MSE={best_mse}");
        println!("Gap from target: {}", best_mse - target_mse);

        // Also scan with gamma varying
        let mut best_a2 = 0.0;
        let mut best_g2 = 0.0;
        let mut best_mse2 = f64::MAX;
        for i in 0..=100 {
            for j in 0..=100 {
                let a = i as f64 / 100.0;
                let g = j as f64 / 100.0;
                model.alpha = a;
                model.gamma = g;
                model.refill();
                if model.mse < best_mse2 {
                    best_mse2 = model.mse;
                    best_a2 = a;
                    best_g2 = g;
                }
            }
        }
        println!("2D best: alpha={best_a2}, gamma={best_g2}, MSE={best_mse2}");

        let result =
            codcel_forecast_ets_confint(44927.0, values, timeline, Some(0.95), Some(0), None, None)
                .unwrap();
        println!("Result: {result}");
        println!("Expected: 25.070267517764044");
    }

    #[test]
    fn test_confint_debug_monthly() {
        let values = vec![
            205.0, 211.0, 218.0, 227.0, 237.0, 245.0, 253.0, 253.0, 249.0, 245.0, 240.0, 239.0,
            241.0, 247.0, 254.0, 263.0, 273.0, 281.0, 289.0, 289.0, 285.0, 281.0, 276.0, 275.0,
        ];
        let timeline = vec![
            44197.0, 44228.0, 44256.0, 44287.0, 44317.0, 44348.0, 44378.0, 44409.0, 44440.0,
            44470.0, 44501.0, 44531.0, 44562.0, 44593.0, 44621.0, 44652.0, 44682.0, 44713.0,
            44743.0, 44774.0, 44805.0, 44835.0, 44866.0, 44896.0,
        ];
        let (proc_values, proc_timeline, month_day) =
            preprocess_data(&values, &timeline, 1, 1).unwrap();
        let step = compute_min_step(&proc_timeline).unwrap();
        println!("Month day: {month_day}");
        println!("Step: {step}");
        println!("Proc timeline: {proc_timeline:?}");
        println!("Proc values: {proc_values:?}");
        println!("n={}", proc_values.len());

        // Check target date in month space
        let target = 44927.0;
        let effective_target = if month_day > 0 {
            convert_x_to_months(target, month_day)
        } else {
            target
        };
        let last_x = proc_timeline[proc_timeline.len() - 1];
        println!("Target: {target} -> effective: {effective_target}");
        println!("Last x: {last_x}");
        println!("Steps ahead: {}", (effective_target - last_x) / step);

        // EDS: scan for target MSE
        let z = codcel_norm_dot_s_dot_inv(0.975).unwrap();
        let target_rmse = 8.561959083085696 / z;
        let target_mse = target_rmse * target_rmse;
        println!("Target MSE: {target_mse}");
        let mut model_eds = EtsModel::new(&proc_values, &proc_timeline, step, 0, true);
        model_eds.init_data();

        // Print MSE at key alpha/gamma values
        for (a, g) in [
            (0.0, 0.0),
            (0.5, 0.0),
            (0.5, 0.5),
            (1.0, 0.0),
            (1.0, 1.0),
            (0.0, 1.0),
        ] {
            model_eds.alpha = a;
            model_eds.gamma = g;
            model_eds.refill();
            println!(
                "alpha={a}, gamma={g}: MSE={:.4}, RMSE={:.4}",
                model_eds.mse,
                model_eds.mse.sqrt()
            );
        }

        model_eds.calc_alpha_beta_gamma_golden();
        println!(
            "EDS golden: alpha={}, gamma={}, MSE={}, RMSE={}",
            model_eds.alpha,
            model_eds.gamma,
            model_eds.mse,
            model_eds.mse.sqrt()
        );

        // NM (concentrated MLE)
        let mut model_nm = EtsModel::new(&proc_values, &proc_timeline, step, 0, true);
        model_nm.init_data();
        model_nm.calc_alpha_beta_gamma(); // NM for EDS
        println!(
            "NM: alpha={}, gamma={}, base[0]={}, trend[0]={}",
            model_nm.alpha, model_nm.gamma, model_nm.base[0], model_nm.trend[0]
        );
        model_nm.refill();
        println!(
            "NM+refill: MSE={}, RMSE={}",
            model_nm.mse,
            model_nm.mse.sqrt()
        );
        println!("NM PI: {}", z * model_nm.mse.sqrt());

        let z = codcel_norm_dot_s_dot_inv(0.975).unwrap();
        println!("PI: {}", z * model_eds.mse.sqrt());
        println!("Expected: 8.561959083085696");
        println!("Expected RMSE: {}", 8.561959083085696 / z);
    }

    #[test]
    fn test_confint_eds_basic() {
        // Data with noise so MSE > 0, EDS mode (seasonality=0)
        let values = vec![10.0, 12.0, 11.0, 14.0, 13.0, 16.0, 15.0, 18.0];
        let timeline = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let result =
            codcel_forecast_ets_confint(9.0, values, timeline, Some(0.95), Some(0), None, None)
                .unwrap();
        assert!(result > 0.0, "CI should be positive, got {result}");
        assert!(result.is_finite(), "CI should be finite, got {result}");
    }

    #[test]
    fn test_confint_eds_increasing_with_horizon() {
        // CI should increase with forecast horizon
        let values = vec![10.0, 12.0, 11.0, 14.0, 13.0, 16.0, 15.0, 18.0];
        let timeline = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];

        let ci_1 = codcel_forecast_ets_confint(
            9.0,
            values.clone(),
            timeline.clone(),
            Some(0.95),
            Some(0),
            None,
            None,
        )
        .unwrap();
        let ci_5 =
            codcel_forecast_ets_confint(13.0, values, timeline, Some(0.95), Some(0), None, None)
                .unwrap();

        assert!(
            ci_5 > ci_1,
            "CI for 5 steps ({ci_5}) should be greater than 1 step ({ci_1})"
        );
    }

    #[test]
    fn test_confint_confidence_level_ordering() {
        // Higher confidence level should give wider CI
        let values = vec![10.0, 12.0, 11.0, 14.0, 13.0, 16.0, 15.0, 18.0];
        let timeline = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];

        let ci_90 = codcel_forecast_ets_confint(
            9.0,
            values.clone(),
            timeline.clone(),
            Some(0.90),
            Some(0),
            None,
            None,
        )
        .unwrap();
        let ci_95 = codcel_forecast_ets_confint(
            9.0,
            values.clone(),
            timeline.clone(),
            Some(0.95),
            Some(0),
            None,
            None,
        )
        .unwrap();
        let ci_99 =
            codcel_forecast_ets_confint(9.0, values, timeline, Some(0.99), Some(0), None, None)
                .unwrap();

        assert!(
            ci_99 > ci_95 && ci_95 > ci_90,
            "Expected CI_99 ({ci_99}) > CI_95 ({ci_95}) > CI_90 ({ci_90})"
        );
    }

    #[test]
    fn test_confint_default_confidence_level() {
        // Default confidence level is 0.95
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let timeline = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];

        let ci_default = codcel_forecast_ets_confint(
            9.0,
            values.clone(),
            timeline.clone(),
            None,
            Some(0),
            None,
            None,
        )
        .unwrap();
        let ci_explicit =
            codcel_forecast_ets_confint(9.0, values, timeline, Some(0.95), Some(0), None, None)
                .unwrap();

        assert!(
            (ci_default - ci_explicit).abs() < 1e-10,
            "Default ({ci_default}) should equal explicit 0.95 ({ci_explicit})"
        );
    }

    #[test]
    fn test_confint_within_range_error() {
        // Target within data range should return error
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let timeline = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let result =
            codcel_forecast_ets_confint(5.0, values, timeline, Some(0.95), Some(0), None, None);
        assert!(result.is_err(), "Within-range target should return error");
    }

    #[test]
    fn test_confint_invalid_confidence_level() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let timeline = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];

        // confidence_level = 0 should error
        let r = codcel_forecast_ets_confint(
            9.0,
            values.clone(),
            timeline.clone(),
            Some(0.0),
            Some(0),
            None,
            None,
        );
        assert!(r.is_err());

        // confidence_level = 1 should error
        let r = codcel_forecast_ets_confint(
            9.0,
            values.clone(),
            timeline.clone(),
            Some(1.0),
            Some(0),
            None,
            None,
        );
        assert!(r.is_err());

        // confidence_level < 0 should error
        let r = codcel_forecast_ets_confint(
            9.0,
            values.clone(),
            timeline.clone(),
            Some(-0.5),
            Some(0),
            None,
            None,
        );
        assert!(r.is_err());

        // confidence_level > 1 should error
        let r = codcel_forecast_ets_confint(9.0, values, timeline, Some(1.5), Some(0), None, None);
        assert!(r.is_err());
    }

    #[test]
    fn test_confint_seasonal() {
        // Seasonal data with period 4, with noise so MSE > 0
        let values = vec![
            100.0, 122.0, 138.0, 112.0, 108.0, 133.0, 148.0, 118.0, 121.0, 138.0, 162.0, 128.0,
        ];
        let timeline: Vec<f64> = (1..=12).map(|i| i as f64).collect();

        let result =
            codcel_forecast_ets_confint(13.0, values, timeline, Some(0.95), Some(4), None, None)
                .unwrap();
        assert!(
            result >= 0.0,
            "Seasonal CI should be non-negative, got {result}"
        );
        assert!(
            result.is_finite(),
            "Seasonal CI should be finite, got {result}"
        );
    }

    #[test]
    fn test_confint_seasonal_increasing_with_horizon() {
        let values = vec![
            100.0, 122.0, 138.0, 112.0, 108.0, 133.0, 148.0, 118.0, 121.0, 138.0, 162.0, 128.0,
        ];
        let timeline: Vec<f64> = (1..=12).map(|i| i as f64).collect();

        let ci_1 = codcel_forecast_ets_confint(
            13.0,
            values.clone(),
            timeline.clone(),
            Some(0.95),
            Some(4),
            None,
            None,
        )
        .unwrap();
        let ci_4 =
            codcel_forecast_ets_confint(16.0, values, timeline, Some(0.95), Some(4), None, None)
                .unwrap();

        assert!(
            ci_4 > ci_1,
            "CI for 4 steps ({ci_4}) should be greater than 1 step ({ci_1})"
        );
    }

    #[test]
    fn test_confint_mismatched_arrays() {
        let result = codcel_forecast_ets_confint(
            5.0,
            vec![1.0, 2.0, 3.0],
            vec![1.0, 2.0],
            None,
            None,
            None,
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_confint_insufficient_data() {
        let result = codcel_forecast_ets_confint(5.0, vec![1.0], vec![1.0], None, None, None, None);
        assert!(result.is_err());
    }
}
