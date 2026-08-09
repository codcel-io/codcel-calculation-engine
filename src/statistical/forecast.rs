// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

//! Shared ETS model infrastructure for FORECAST.ETS and FORECAST.ETS.CONFINT.

use crate::date_time_base::excel_to_date_time;
use chrono::Datelike;
use std::error::Error;

// =============================================================================
// ETS Model
// =============================================================================
//
// Both seasonal (ETS) and non-seasonal (EDS) modes use Nelder-Mead optimization
// with concentrated MLE (innovations form). For each candidate set of smoothing
// parameters, initial states are computed analytically via least-squares.
//
// Excel uses an undocumented proprietary algorithm that does not minimize SSE.
// See docs/FORECAST_ETS_COMPATIBILITY.md for detailed accuracy comparison.

/// Saved initial states for restoring before each clean refill.
#[cfg(test)]
pub(super) struct InitialStates {
    base0: f64,
    trend0: f64,
    per_idx_init: Vec<f64>,
    forecast0: f64,
}

pub(super) struct EtsModel {
    pub(super) values: Vec<f64>,   // Y values (maRange[].Y)
    pub(super) timeline: Vec<f64>, // X values (maRange[].X) - not used in model, just for forecast
    pub(super) base: Vec<f64>,     // mpBase - level states
    pub(super) trend: Vec<f64>,    // mpTrend
    pub(super) per_idx: Vec<f64>,  // mpPerIdx - seasonal indices
    pub(super) forecast: Vec<f64>, // mpForecast - one-step-ahead forecasts
    pub(super) n: usize,           // mnCount
    pub(super) m: usize,           // mnSmplInPrd - samples in period
    pub(super) step_size: f64,     // mfStepSize
    pub(super) alpha: f64,         // mfAlpha - level smoothing
    pub(super) beta: f64,          // mfBeta - seasonal smoothing (LO convention)
    pub(super) gamma: f64,         // mfGamma - trend smoothing (LO convention)
    pub(super) mse: f64,           // mfMSE
    pub(super) b_eds: bool,        // bEDS - exponential double smoothing (no seasonality)
}

pub(super) const CF_MIN_ABC_RESOLUTION: f64 = 0.001;

impl EtsModel {
    pub(super) fn new(
        values: &[f64],
        timeline: &[f64],
        step_size: f64,
        season_length: usize,
        b_eds: bool,
    ) -> Self {
        let n = values.len();
        EtsModel {
            values: values.to_vec(),
            timeline: timeline.to_vec(),
            base: vec![0.0; n],
            trend: vec![0.0; n],
            per_idx: vec![0.0; n + 1], // extra slot for sentinel
            forecast: vec![0.0; n],
            n,
            m: season_length,
            step_size,
            alpha: 0.0,
            beta: 0.0,
            gamma: 0.0,
            mse: 0.0,
            b_eds,
        }
    }

    /// Initialize data: trend, seasonal indices, base level.
    /// Order matters: trend first, then seasonal, then base.
    pub(super) fn init_data(&mut self) {
        self.forecast[0] = self.values[0];
        self.prefill_trend_data();
        if !self.b_eds {
            self.prefill_per_idx();
        }
        self.prefill_base_data();
    }

    /// EDS: T[0] = (Y[n-1] - Y[0]) / (n-1)
    /// ETS: T[0] = sum(Y[i+m] - Y[i], i=0..m-1) / (m*m)
    fn prefill_trend_data(&mut self) {
        if self.b_eds {
            self.trend[0] = (self.values[self.n - 1] - self.values[0]) / (self.n - 1) as f64;
        } else {
            let m = self.m;
            let mut sum = 0.0;
            for i in 0..m {
                if i + m < self.n {
                    sum += self.values[i + m] - self.values[i];
                }
            }
            self.trend[0] = sum / (m as f64 * m as f64);
        }
    }

    /// Compute initial seasonal indices using period-average method.
    /// For each position j in the season:
    ///   Additive: S[j] = mean over periods of (Y[k*m+j] - (PeriodAvg[k] + (j - 0.5*(m-1)) * T[0]))
    fn prefill_per_idx(&mut self) {
        let m = self.m;
        let n_periods = self.n / m;

        // Compute period averages
        let mut period_avgs: Vec<f64> = Vec::with_capacity(n_periods);
        for k in 0..n_periods {
            let mut sum = 0.0;
            for j in 0..m {
                sum += self.values[k * m + j];
            }
            period_avgs.push(sum / m as f64);
        }

        // Compute seasonal indices (additive)
        for j in 0..m {
            let mut fi = 0.0;
            for (k, &pavg) in period_avgs.iter().enumerate() {
                let idx = k * m + j;
                if idx < self.n {
                    fi += self.values[idx]
                        - (pavg + (j as f64 - 0.5 * (m as f64 - 1.0)) * self.trend[0]);
                }
            }
            self.per_idx[j] = fi / n_periods as f64;
        }

        // Wrap-around value at position m (used when i == m with strictly > comparison)
        if m < self.n {
            self.per_idx[m] = self.per_idx[0];
        }
    }

    /// EDS: L[0] = Y[0]
    /// ETS (additive): L[0] = Y[0] - S[0]
    /// subtraction is the mathematically correct form for additive models. Excel uses
    /// a different initialization that subtraction better approximates.
    fn prefill_base_data(&mut self) {
        if self.b_eds {
            self.base[0] = self.values[0];
        } else {
            self.base[0] = self.values[0] - self.per_idx[0];
        }
    }

    /// Run the model forward with current alpha, beta, gamma.
    /// Updates base, trend, per_idx, forecast arrays and computes MSE.
    pub(super) fn refill(&mut self) {
        for i in 1..self.n {
            if self.b_eds {
                // EDS equations
                self.base[i] = self.alpha * self.values[i]
                    + (1.0 - self.alpha) * (self.base[i - 1] + self.trend[i - 1]);
                self.trend[i] = self.gamma * (self.base[i] - self.base[i - 1])
                    + (1.0 - self.gamma) * self.trend[i - 1];
                self.forecast[i] = self.base[i - 1] + self.trend[i - 1];
            } else {
                // ETS additive equations (LO convention: strictly >)
                let n_idx = if i > self.m { i - self.m } else { i };

                self.base[i] = self.alpha * (self.values[i] - self.per_idx[n_idx])
                    + (1.0 - self.alpha) * (self.base[i - 1] + self.trend[i - 1]);

                // Seasonal update: uses current level (base[i]), not previous
                self.per_idx[i] = self.beta * (self.values[i] - self.base[i])
                    + (1.0 - self.beta) * self.per_idx[n_idx];

                self.trend[i] = self.gamma * (self.base[i] - self.base[i - 1])
                    + (1.0 - self.gamma) * self.trend[i - 1];

                self.forecast[i] = self.base[i - 1] + self.trend[i - 1] + self.per_idx[n_idx];
            }
        }
        self.calc_accuracy_indicators();
    }

    /// Compute MSE over indices 1..n-1 (index 0 excluded since forecast[0] = Y[0]).
    /// Uses SSE/(n-1)
    pub(super) fn calc_accuracy_indicators(&mut self) {
        let mut sum_err_sq = 0.0;
        for i in 1..self.n {
            let error = self.forecast[i] - self.values[i];
            sum_err_sq += error * error;
        }
        self.mse = sum_err_sq / (self.n - 1) as f64;
    }

    // =========================================================================
    // Optimization
    // =========================================================================

    pub(super) fn calc_alpha_beta_gamma(&mut self) {
        if self.b_eds {
            self.beta = 0.0;
            self.optimize_eds_nelder_mead();
        } else {
            self.optimize_ets_innovations();
        }
    }

    /// Optimize for CONFINT — same strategy as calc_alpha_beta_gamma.
    pub(super) fn calc_alpha_beta_gamma_golden(&mut self) {
        self.calc_alpha_beta_gamma();
    }

    /// Optimize ETS using concentrated MLE innovations form.
    /// For fixed (α, β, γ), initial states are computed analytically (concentrated MLE).
    /// Then (α, β, γ) are optimized with Nelder-Mead.
    pub(super) fn optimize_ets_innovations(&mut self) {
        let m = self.m;
        let n = self.n;

        let starting_points: Vec<[f64; 3]> = vec![
            [0.01, 0.01, 0.25],
            [0.001, 0.001, 0.25],
            [0.1, 0.1, 0.1],
            [0.5, 0.5, 0.5],
            [0.0, 0.0, 0.0],
            [0.002, 0.001, 0.25],
            [0.3, 0.1, 0.3],
            [0.01, 0.5, 0.01],
        ];

        let mut global_best = [0.0_f64; 3];
        let mut global_best_sse = f64::MAX;
        let mut global_best_init: Vec<f64> = vec![0.0; m + 2];

        for start in &starting_points {
            let (params, sse, init_states) = Self::run_ets_concentrated_nm(&self.values, m, start);
            if sse < global_best_sse {
                global_best_sse = sse;
                global_best = params;
                global_best_init = init_states;
            }
        }

        self.alpha = global_best[0];
        self.beta = global_best[1];
        self.gamma = global_best[2];

        let opt_l0 = global_best_init[0];
        let opt_b0 = global_best_init[1];
        let opt_s: Vec<f64> = global_best_init[2..2 + m].to_vec();

        // Fill model arrays using innovations form with optimal initial states
        self.base[0] = opt_l0;
        self.trend[0] = opt_b0;
        self.per_idx[..m].copy_from_slice(&opt_s[..m]);
        self.forecast[0] = self.values[0];

        let mut level = opt_l0;
        let mut trend = opt_b0;
        let mut seasonal = opt_s;

        for i in 0..n {
            let s_idx = i % m;
            let forecast_val = level + trend + seasonal[s_idx];
            let error = self.values[i] - forecast_val;

            if i > 0 {
                self.forecast[i] = forecast_val;
            }

            let new_level = level + trend + self.alpha * error;
            let new_trend = trend + self.gamma * error;
            seasonal[s_idx] += self.beta * error;

            self.base[i] = new_level;
            self.trend[i] = new_trend;
            self.per_idx[i] = seasonal[s_idx];

            level = new_level;
            trend = new_trend;
        }

        self.mse = global_best_sse / (n - 1) as f64;
    }

    // =========================================================================
    // EDS: ETS(A,A,N) innovations state-space model with concentrated MLE
    // =========================================================================
    //
    // Excel uses the ETS innovations (error-correction) form:
    //   e[t] = Y[t] - (l[t] + b[t])        forecast error
    //   l[t+1] = l[t] + b[t] + alpha * e[t] level update
    //   b[t+1] = b[t] + beta * e[t]         trend update
    //   Forecast = l[n] + b[n]
    //
    // Concentrated MLE: for each (alpha, beta) pair, the optimal (l0, b0) are
    // computed analytically by solving a least-squares system. Then only (alpha, beta)
    // are optimized numerically using Nelder-Mead.

    /// For given (alpha, beta), compute the optimal (l0, b0) analytically and return SSE.
    /// The ETS(A,A,N) model is linear in (l0, b0): Y[t] = c_l[t]*l0 + c_b[t]*b0 + noise.
    /// We use least squares to find optimal l0, b0.
    /// Returns (sse, l0, b0, final_level, final_trend).
    fn ets_aan_concentrated(values: &[f64], alpha: f64, beta: f64) -> (f64, f64, f64, f64, f64) {
        let n = values.len();

        // The forecast at time t is: f[t] = l[t] + b[t]
        // l[t] and b[t] are linear functions of (l0, b0):
        //   l[t] = a_l[t] * l0 + a_b[t] * b0 + a_c[t]  (a_c depends on past Y values)
        //   b[t] = d_l[t] * l0 + d_b[t] * b0 + d_c[t]
        //   f[t] = (a_l[t]+d_l[t]) * l0 + (a_b[t]+d_b[t]) * b0 + (a_c[t]+d_c[t])
        //
        // We build the linear system: Y[t] = coeff_l[t]*l0 + coeff_b[t]*b0 + const[t] + e[t]
        // and solve for l0, b0 via normal equations.

        // Track coefficients for l and b as linear functions of (l0, b0)
        // l[t] = al * l0 + ab * b0 + ac (ac depends on past Y)
        // b[t] = dl * l0 + db * b0 + dc
        let mut al = 1.0_f64; // dl/dl0
        let mut ab = 0.0_f64; // dl/db0
        let mut ac = 0.0_f64; // constant part
        let mut dl = 0.0_f64; // db/dl0
        let mut db = 1.0_f64; // db/db0
        let mut dc = 0.0_f64;

        // Normal equations: sum((Y[t] - cl*l0 - cb*b0 - cc)^2)
        // ATA * [l0, b0]' = ATb
        let mut ata00 = 0.0; // sum(cl^2)
        let mut ata01 = 0.0; // sum(cl*cb)
        let mut ata11 = 0.0; // sum(cb^2)
        let mut atb0 = 0.0; // sum(cl*(Y-cc))
        let mut atb1 = 0.0; // sum(cb*(Y-cc))

        for &val in values.iter().take(n) {
            // Forecast coefficients at time t
            let cl = al + dl; // coefficient of l0 in forecast[t]
            let cb = ab + db; // coefficient of b0 in forecast[t]
            let cc = ac + dc; // constant part of forecast[t]

            // Accumulate normal equations
            let residual_const = val - cc;
            ata00 += cl * cl;
            ata01 += cl * cb;
            ata11 += cb * cb;
            atb0 += cl * residual_const;
            atb1 += cb * residual_const;

            // Update state coefficients for t+1
            // e[t] = Y[t] - (l[t] + b[t]) = Y[t] - cl*l0 - cb*b0 - cc
            // l[t+1] = l[t] + b[t] + alpha * e[t]
            //        = (al+dl)*l0 + (ab+db)*b0 + (ac+dc) + alpha*(Y[t] - cl*l0 - cb*b0 - cc)
            //        = (1-alpha)*cl*l0 + (1-alpha)*cb*b0 + (1-alpha)*cc + alpha*Y[t]
            let new_al = (1.0 - alpha) * cl;
            let new_ab = (1.0 - alpha) * cb;
            let new_ac = (1.0 - alpha) * cc + alpha * val;

            // b[t+1] = b[t] + beta * e[t]
            //        = dl*l0 + db*b0 + dc + beta*(Y[t] - cl*l0 - cb*b0 - cc)
            //        = (dl - beta*cl)*l0 + (db - beta*cb)*b0 + (dc - beta*cc + beta*Y[t])
            let new_dl = dl - beta * cl;
            let new_db = db - beta * cb;
            let new_dc = dc - beta * cc + beta * val;

            al = new_al;
            ab = new_ab;
            ac = new_ac;
            dl = new_dl;
            db = new_db;
            dc = new_dc;
        }

        // Solve 2x2 system: ATA * [l0, b0]' = ATb
        let det = ata00 * ata11 - ata01 * ata01;
        let (l0, b0) = if det.abs() > 1e-30 {
            let l0 = (ata11 * atb0 - ata01 * atb1) / det;
            let b0 = (ata00 * atb1 - ata01 * atb0) / det;
            (l0, b0)
        } else {
            // Degenerate: fall back to OLS-style initialization
            let mean_y: f64 = values.iter().sum::<f64>() / values.len() as f64;
            let n_f = values.len() as f64;
            let mean_x = (n_f - 1.0) / 2.0;
            let mut num = 0.0;
            let mut den_val = 0.0;
            for (i, &y) in values.iter().enumerate() {
                let x = i as f64;
                num += (x - mean_x) * (y - mean_y);
                den_val += (x - mean_x) * (x - mean_x);
            }
            let slope = if den_val > 0.0 { num / den_val } else { 0.0 };
            (mean_y - slope * mean_x, slope)
        };

        // Compute actual SSE and final states with the optimal l0, b0
        let mut level = l0;
        let mut trend = b0;
        let mut sse = 0.0;

        for &val in values.iter().take(n) {
            let error = val - (level + trend);
            sse += error * error;
            let new_level = level + trend + alpha * error;
            let new_trend = trend + beta * error;
            level = new_level;
            trend = new_trend;
        }

        (sse, l0, b0, level, trend)
    }

    pub(super) fn optimize_eds_nelder_mead(&mut self) {
        // Concentrated MLE: optimize only (alpha, beta) using Nelder-Mead,
        // with l0, b0 computed analytically for each (alpha, beta) pair.
        let starting_points: Vec<[f64; 2]> = vec![
            [0.1, 0.1],
            [0.01, 0.01],
            [0.5, 0.5],
            [0.3, 0.1],
            [0.01, 0.5],
            [0.5, 0.01],
        ];

        let mut global_best_ab = [0.0, 0.0];
        let mut global_best_sse = f64::MAX;
        let mut global_best_l0 = 0.0;
        let mut global_best_b0 = 0.0;
        let mut global_best_final_l = 0.0;
        let mut global_best_final_t = 0.0;

        for start in &starting_points {
            let (ab, sse, l0, b0, final_l, final_t) =
                Self::run_nelder_mead_concentrated(&self.values, start);
            if sse < global_best_sse {
                global_best_sse = sse;
                global_best_ab = ab;
                global_best_l0 = l0;
                global_best_b0 = b0;
                global_best_final_l = final_l;
                global_best_final_t = final_t;
            }
        }

        self.alpha = global_best_ab[0].clamp(0.0, 1.0);
        self.gamma = global_best_ab[1].clamp(0.0, 1.0);
        self.base[0] = global_best_l0;
        self.trend[0] = global_best_b0;
        self.base[self.n - 1] = global_best_final_l;
        self.trend[self.n - 1] = global_best_final_t;
        // For CONFINT, use n-4 degrees of freedom (4 parameters: alpha, beta, l0, b0)
        let dof = if self.n > 4 { self.n - 4 } else { 1 };
        self.mse = global_best_sse / dof as f64;
    }

    /// Nelder-Mead for 2-parameter concentrated MLE: optimize (alpha, beta).
    /// Returns (best_ab, sse, l0, b0, final_level, final_trend).
    fn run_nelder_mead_concentrated(
        values: &[f64],
        x0: &[f64; 2],
    ) -> ([f64; 2], f64, f64, f64, f64, f64) {
        let n_params = 2;
        let max_iter = 10000;
        let tol = 1e-14;

        let mut simplex: Vec<[f64; 2]> = Vec::with_capacity(n_params + 1);
        simplex.push(*x0);
        let deltas = [0.3, 0.3];
        for j in 0..n_params {
            let mut vertex = *x0;
            vertex[j] += deltas[j];
            simplex.push(vertex);
        }

        let eval = |params: &[f64; 2]| -> f64 {
            let a = params[0].clamp(0.0, 1.0);
            let b = params[1].clamp(0.0, 1.0);
            let (sse, _, _, _, _) = Self::ets_aan_concentrated(values, a, b);
            let penalty = 1e8 * ((params[0] - a).powi(2) + (params[1] - b).powi(2));
            sse + penalty
        };

        let mut f_values: Vec<f64> = simplex.iter().map(&eval).collect();

        for _iter in 0..max_iter {
            let mut order: Vec<usize> = (0..=n_params).collect();
            order.sort_by(|&a, &b| f_values[a].partial_cmp(&f_values[b]).unwrap());

            let best = order[0];
            let worst = order[n_params];
            let second_worst = order[n_params - 1];

            if f_values[worst] - f_values[best] < tol {
                break;
            }

            let mut centroid = [0.0; 2];
            for &idx in &order[..n_params] {
                for j in 0..n_params {
                    centroid[j] += simplex[idx][j];
                }
            }
            for c in centroid.iter_mut().take(n_params) {
                *c /= n_params as f64;
            }

            let mut reflected = [0.0; 2];
            for j in 0..n_params {
                reflected[j] = centroid[j] + (centroid[j] - simplex[worst][j]);
            }
            let f_reflected = eval(&reflected);

            if f_reflected < f_values[best] {
                let mut expanded = [0.0; 2];
                for j in 0..n_params {
                    expanded[j] = centroid[j] + 2.0 * (reflected[j] - centroid[j]);
                }
                let f_expanded = eval(&expanded);
                if f_expanded < f_reflected {
                    simplex[worst] = expanded;
                    f_values[worst] = f_expanded;
                } else {
                    simplex[worst] = reflected;
                    f_values[worst] = f_reflected;
                }
            } else if f_reflected < f_values[second_worst] {
                simplex[worst] = reflected;
                f_values[worst] = f_reflected;
            } else {
                let use_outside = f_reflected < f_values[worst];
                let ref_point = if use_outside {
                    reflected
                } else {
                    simplex[worst]
                };
                let f_ref = if use_outside {
                    f_reflected
                } else {
                    f_values[worst]
                };

                let mut contracted = [0.0; 2];
                for j in 0..n_params {
                    contracted[j] = centroid[j] + 0.5 * (ref_point[j] - centroid[j]);
                }
                let f_contracted = eval(&contracted);

                if f_contracted < f_ref {
                    simplex[worst] = contracted;
                    f_values[worst] = f_contracted;
                } else {
                    for &idx in &order[1..] {
                        let best_vertex = simplex[best];
                        for (s, &b) in simplex[idx]
                            .iter_mut()
                            .zip(best_vertex.iter())
                            .take(n_params)
                        {
                            *s = b + 0.5 * (*s - b);
                        }
                        f_values[idx] = eval(&simplex[idx]);
                    }
                }
            }
        }

        let best_idx = f_values
            .iter()
            .enumerate()
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;

        let best_ab = [
            simplex[best_idx][0].clamp(0.0, 1.0),
            simplex[best_idx][1].clamp(0.0, 1.0),
        ];
        let (sse, l0, b0, fl, ft) = Self::ets_aan_concentrated(values, best_ab[0], best_ab[1]);
        (best_ab, sse, l0, b0, fl, ft)
    }

    // =========================================================================
    // ETS(A,A,A) concentrated MLE optimizer with Nelder-Mead
    // =========================================================================
    //
    // Uses the innovations state-space model:
    //   e[t] = Y[t] - (l[t] + b[t] + s[t % m])
    //   l[t+1] = l[t] + b[t] + α·e[t]
    //   b[t+1] = b[t] + γ·e[t]
    //   s[t%m] += β·e[t]
    //
    // For fixed (α, β, γ), the model is LINEAR in (l0, b0, s[0]..s[m-1]).
    // We solve analytically for the optimal initial states using least squares
    // (concentrated MLE), then optimize only (α, β, γ) with Nelder-Mead.

    /// Concentrated MLE for ETS(A,A,A): for fixed (α, β, γ), solve for optimal
    /// (l0, b0, s[0..m-1]) analytically via least squares.
    ///
    /// The forecast at time t is linear in the initial states:
    ///   f[t] = c_l[t]*l0 + c_b[t]*b0 + sum_j(c_s[t][j]*s_j) + c_const[t]
    ///
    /// We build the normal equations and solve the (m+2)×(m+2) system.
    pub(super) fn ets_aaa_concentrated(
        values: &[f64],
        m: usize,
        alpha: f64,
        beta: f64,
        gamma: f64,
    ) -> (f64, Vec<f64>) {
        let n = values.len();
        // Free parameters: l0, b0, s[0..m-2] (s[m-1] = -sum(s[0..m-2]))
        // This sum-to-zero constraint ensures identifiability.
        let dim = m + 1;

        // Track state coefficients as vectors of size dim+1:
        // state = coeffs[0]*l0 + coeffs[1]*b0 + coeffs[2..m+1]*s[j] + coeffs[m+1] (constant)

        // Level coefficients
        let mut lev = vec![0.0; dim + 1];
        lev[0] = 1.0; // l0

        // Trend coefficients
        let mut trd = vec![0.0; dim + 1];
        trd[1] = 1.0; // b0

        // Seasonal coefficients: one per position
        let mut sea: Vec<Vec<f64>> = (0..m)
            .map(|j| {
                let mut v = vec![0.0; dim + 1];
                if j < m - 1 {
                    v[2 + j] = 1.0; // s[j] is free
                } else {
                    // s[m-1] = -(s[0]+...+s[m-2])
                    for k in 0..m - 1 {
                        v[2 + k] = -1.0;
                    }
                }
                v
            })
            .collect();

        // Normal equations: ATA and ATb
        let mut ata = vec![vec![0.0; dim]; dim];
        let mut atb = vec![0.0; dim];

        for (t, &val) in values.iter().enumerate().take(n) {
            let s_idx = t % m;

            // Forecast coefficients: f[t] = l[t] + b[t] + s[t%m]
            let mut fc = vec![0.0; dim + 1];
            for k in 0..=dim {
                fc[k] = lev[k] + trd[k] + sea[s_idx][k];
            }

            // Accumulate normal equations
            let fc_const = fc[dim]; // constant part
            let y_minus_const = val - fc_const;

            for i in 0..dim {
                atb[i] += fc[i] * y_minus_const;
                for j in 0..dim {
                    ata[i][j] += fc[i] * fc[j];
                }
            }

            // Compute new state coefficients after this time step
            // e[t] = Y[t] - f[t]
            // e[t] has coefficients: -fc[0..dim-1] for the params, and (Y[t] - fc_const) as constant part
            // But we need the linear-in-initial-states coefficients for the error:
            // e_coeff[k] = -fc[k] for k < dim, e_const = Y[t] - fc_const

            // l[t+1] = l[t] + b[t] + α*e[t]
            //        = (lev + trd) + α*(Y[t] - (lev + trd + sea[s_idx]))
            //        = (1-α)*(lev + trd) + α*(Y[t] - sea[s_idx])
            //        = (1-α)*lev + (1-α)*trd - α*sea[s_idx] + α*Y[t]   -- NO, error
            // Actually: l[t+1] = l[t] + b[t] + α*e[t]
            //         = l[t] + b[t] + α*(Y[t] - l[t] - b[t] - s[t%m])
            //         = (1-α)*(l[t] + b[t]) - α*s[t%m] + α*Y[t]
            //         = (1-α)*l[t] + (1-α)*b[t] - α*s[t%m] + α*Y[t]

            let mut new_lev = vec![0.0; dim + 1];
            for k in 0..dim {
                new_lev[k] = (1.0 - alpha) * (lev[k] + trd[k]) - alpha * sea[s_idx][k];
            }
            new_lev[dim] =
                (1.0 - alpha) * (lev[dim] + trd[dim]) - alpha * sea[s_idx][dim] + alpha * val;

            // b[t+1] = b[t] + γ*e[t]
            //        = b[t] + γ*(Y[t] - l[t] - b[t] - s[t%m])
            //        = -γ*l[t] + (1-γ)*b[t] - γ*s[t%m] + γ*Y[t]
            let mut new_trd = vec![0.0; dim + 1];
            for k in 0..dim {
                new_trd[k] = -gamma * lev[k] + (1.0 - gamma) * trd[k] - gamma * sea[s_idx][k];
            }
            new_trd[dim] = -gamma * lev[dim] + (1.0 - gamma) * trd[dim] - gamma * sea[s_idx][dim]
                + gamma * val;

            // s[t%m] += β*e[t]
            //         = s[t%m] + β*(Y[t] - l[t] - b[t] - s[t%m])
            //         = -β*l[t] - β*b[t] + (1-β)*s[t%m] + β*Y[t]
            let mut new_sea = vec![0.0; dim + 1];
            for k in 0..dim {
                new_sea[k] = -beta * lev[k] - beta * trd[k] + (1.0 - beta) * sea[s_idx][k];
            }
            new_sea[dim] =
                -beta * lev[dim] - beta * trd[dim] + (1.0 - beta) * sea[s_idx][dim] + beta * val;

            lev = new_lev;
            trd = new_trd;
            sea[s_idx] = new_sea;
        }

        // Solve ATA * x = ATb using Gaussian elimination with partial pivoting
        let x = Self::solve_linear_system(&ata, &atb);

        // Reconstruct full initial states: l0, b0, s[0..m-1]
        let l0 = x[0];
        let b0 = x[1];
        let mut full_init = vec![l0, b0];
        let mut s_sum = 0.0;
        for j in 0..m - 1 {
            full_init.push(x[2 + j]);
            s_sum += x[2 + j];
        }
        full_init.push(-s_sum); // s[m-1] = -(s[0]+...+s[m-2])

        // Compute SSE with the optimal initial states
        let mut level = l0;
        let mut trend = b0;
        let mut seasonal: Vec<f64> = full_init[2..2 + m].to_vec();
        let mut sse = 0.0;

        for (t, &val) in values.iter().enumerate().take(n) {
            let s_idx = t % m;
            let forecast = level + trend + seasonal[s_idx];
            let error = val - forecast;
            sse += error * error;

            let new_level = level + trend + alpha * error;
            let new_trend = trend + gamma * error;
            seasonal[s_idx] += beta * error;

            level = new_level;
            trend = new_trend;
        }

        (sse, full_init)
    }

    /// Solve a linear system Ax = b using Gaussian elimination with partial pivoting.
    fn solve_linear_system(a: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
        let n = b.len();
        // Build augmented matrix
        let mut aug: Vec<Vec<f64>> = (0..n)
            .map(|i| {
                let mut row = a[i].clone();
                row.push(b[i]);
                row
            })
            .collect();

        // Forward elimination with partial pivoting
        for col in 0..n {
            // Find pivot
            let mut max_val = aug[col][col].abs();
            let mut max_row = col;
            for (row_idx, aug_row) in aug.iter().enumerate().take(n).skip(col + 1) {
                let abs_val = aug_row[col].abs();
                if abs_val > max_val {
                    max_val = abs_val;
                    max_row = row_idx;
                }
            }
            aug.swap(col, max_row);

            let pivot = aug[col][col];
            if pivot.abs() < 1e-30 {
                continue; // Skip near-singular column
            }

            for row in (col + 1)..n {
                let (top, bottom) = aug.split_at_mut(row);
                let factor = bottom[0][col] / pivot;
                for (dest, &src) in bottom[0][col..=n].iter_mut().zip(top[col][col..=n].iter()) {
                    *dest -= factor * src;
                }
            }
        }

        // Back substitution
        let mut x = vec![0.0; n];
        for i in (0..n).rev() {
            let mut sum = aug[i][n];
            for j in (i + 1)..n {
                sum -= aug[i][j] * x[j];
            }
            if aug[i][i].abs() > 1e-30 {
                x[i] = sum / aug[i][i];
            }
        }

        x
    }

    /// Run Nelder-Mead for (α, β, γ) with concentrated MLE for initial states.
    fn run_ets_concentrated_nm(
        values: &[f64],
        m: usize,
        x0: &[f64; 3],
    ) -> ([f64; 3], f64, Vec<f64>) {
        let n_params = 3;
        let max_iter = 10000;
        let tol = 1e-14;

        // Constrain to [0, 0.99] to prevent degenerate boundary solutions
        let eval = |p: &[f64; 3]| -> (f64, Vec<f64>) {
            let a = p[0].clamp(0.0, 0.99);
            let b = p[1].clamp(0.0, 0.99);
            let g = p[2].clamp(0.0, 0.99);
            let (sse, init_states) = Self::ets_aaa_concentrated(values, m, a, b, g);
            let penalty = 1e8 * ((p[0] - a).powi(2) + (p[1] - b).powi(2) + (p[2] - g).powi(2));
            (sse + penalty, init_states)
        };

        let mut simplex: Vec<[f64; 3]> = Vec::with_capacity(n_params + 1);
        simplex.push(*x0);
        let deltas = [0.2, 0.2, 0.2];
        for j in 0..n_params {
            let mut vertex = *x0;
            vertex[j] = (vertex[j] + deltas[j]).min(0.99);
            simplex.push(vertex);
        }

        let mut results: Vec<(f64, Vec<f64>)> = simplex.iter().map(&eval).collect();
        let mut f_values: Vec<f64> = results.iter().map(|(f, _)| *f).collect();

        for _iter in 0..max_iter {
            let mut order: Vec<usize> = (0..=n_params).collect();
            order.sort_by(|&a, &b| f_values[a].partial_cmp(&f_values[b]).unwrap());

            let best = order[0];
            let worst = order[n_params];
            let second_worst = order[n_params - 1];

            if f_values[worst] - f_values[best] < tol {
                break;
            }

            let mut centroid = [0.0; 3];
            for &idx in &order[..n_params] {
                for j in 0..n_params {
                    centroid[j] += simplex[idx][j];
                }
            }
            for c in centroid.iter_mut() {
                *c /= n_params as f64;
            }

            let mut reflected = [0.0; 3];
            for j in 0..n_params {
                reflected[j] = centroid[j] + (centroid[j] - simplex[worst][j]);
            }
            let (f_reflected, init_reflected) = eval(&reflected);

            if f_reflected < f_values[best] {
                let mut expanded = [0.0; 3];
                for j in 0..n_params {
                    expanded[j] = centroid[j] + 2.0 * (reflected[j] - centroid[j]);
                }
                let (f_expanded, init_expanded) = eval(&expanded);
                if f_expanded < f_reflected {
                    simplex[worst] = expanded;
                    f_values[worst] = f_expanded;
                    results[worst] = (f_expanded, init_expanded);
                } else {
                    simplex[worst] = reflected;
                    f_values[worst] = f_reflected;
                    results[worst] = (f_reflected, init_reflected);
                }
            } else if f_reflected < f_values[second_worst] {
                simplex[worst] = reflected;
                f_values[worst] = f_reflected;
                results[worst] = (f_reflected, init_reflected);
            } else {
                let use_outside = f_reflected < f_values[worst];
                let ref_point = if use_outside {
                    reflected
                } else {
                    simplex[worst]
                };
                let f_ref = if use_outside {
                    f_reflected
                } else {
                    f_values[worst]
                };

                let mut contracted = [0.0; 3];
                for j in 0..n_params {
                    contracted[j] = centroid[j] + 0.5 * (ref_point[j] - centroid[j]);
                }
                let (f_contracted, init_contracted) = eval(&contracted);

                if f_contracted < f_ref {
                    simplex[worst] = contracted;
                    f_values[worst] = f_contracted;
                    results[worst] = (f_contracted, init_contracted);
                } else {
                    let best_vertex = simplex[best];
                    for &idx in &order[1..] {
                        for j in 0..n_params {
                            simplex[idx][j] =
                                best_vertex[j] + 0.5 * (simplex[idx][j] - best_vertex[j]);
                        }
                        let (f_shrunk, init_shrunk) = eval(&simplex[idx]);
                        f_values[idx] = f_shrunk;
                        results[idx] = (f_shrunk, init_shrunk);
                    }
                }
            }
        }

        let best_idx = f_values
            .iter()
            .enumerate()
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;

        let best = [
            simplex[best_idx][0].clamp(0.0, 0.99),
            simplex[best_idx][1].clamp(0.0, 0.99),
            simplex[best_idx][2].clamp(0.0, 0.99),
        ];
        (best, f_values[best_idx], results[best_idx].1.clone())
    }

    // =========================================================================
    // Innovations-form NM with fixed prefill initial states
    // =========================================================================
    //
    // Uses the innovations form equations:
    //   e[t] = Y[t] - (l[t] + b[t] + s[t%m])
    //   l[t+1] = l[t] + b[t] + α*e[t]
    //   b[t+1] = b[t] + γ*e[t]      (gamma = trend smoothing in LO convention)
    //   s[t%m] += β*e[t]              (beta = seasonal smoothing in LO convention)
    //
    // Initial states are FIXED from prefill (l0=base[0], b0=trend[0], s[j]=per_idx[j]).
    // Only (α, β, γ) are optimized with Nelder-Mead.

    // =========================================================================
    // Forecast computation
    // =========================================================================

    /// Compute forecast for a target date.
    /// Compute forecast for a target date.
    /// - Within data range: return actual observed value (with interpolation)
    /// - Beyond data range: extrapolate from final level + trend (+ seasonal)
    pub(super) fn get_forecast(&self, target_date: f64) -> f64 {
        let first_x = self.timeline[0];
        let last_x = self.timeline[self.n - 1];

        if target_date <= last_x {
            // Within data range: return actual data value
            let offset = target_date - first_x;
            let n_idx = (offset / self.step_size) as usize;
            let n_idx = n_idx.min(self.n - 1);
            let f_interpolate = offset - n_idx as f64 * self.step_size;

            let mut forecast = self.values[n_idx];

            if f_interpolate >= CF_MIN_ABC_RESOLUTION && n_idx + 1 < self.n {
                let f_interpolate_factor = f_interpolate / self.step_size;
                let fc_1 = self.values[n_idx + 1];
                forecast += f_interpolate_factor * (fc_1 - forecast);
            }

            forecast
        } else {
            // Beyond data range: extrapolate
            let steps_f = (target_date - last_x) / self.step_size;
            let n_steps = steps_f as usize;
            let f_interpolate = (target_date - last_x) - n_steps as f64 * self.step_size;

            let forecast = if self.b_eds {
                self.base[self.n - 1] + n_steps as f64 * self.trend[self.n - 1]
            } else {
                // Additive seasonal forecast
                let s_idx = self.n - 1 - self.m + (n_steps % self.m);
                self.base[self.n - 1]
                    + n_steps as f64 * self.trend[self.n - 1]
                    + self.per_idx[s_idx]
            };

            // Interpolation if target doesn't fall exactly on a step
            if f_interpolate >= CF_MIN_ABC_RESOLUTION {
                let f_interpolate_factor = f_interpolate / self.step_size;
                let fc_1 = if self.b_eds {
                    self.base[self.n - 1] + (n_steps + 1) as f64 * self.trend[self.n - 1]
                } else {
                    let s_idx_1 = self.n - 1 - self.m + ((n_steps + 1) % self.m);
                    self.base[self.n - 1]
                        + (n_steps + 1) as f64 * self.trend[self.n - 1]
                        + self.per_idx[s_idx_1]
                };
                forecast + f_interpolate_factor * (fc_1 - forecast)
            } else {
                forecast
            }
        }
    }
}

// =============================================================================
// Seasonality Auto-Detection
// =============================================================================

/// Detect seasonality period.
/// For each candidate period length, compute the mean absolute difference
/// of first-differences at that lag. Period with smallest error wins.
/// Requires at least 3 complete cycles to confirm seasonality (matching Excel).
pub(crate) fn detect_seasonality(values: &[f64]) -> usize {
    let n = values.len();
    if n < 4 {
        return 0;
    }

    // Compute first differences
    let diffs: Vec<f64> = values.windows(2).map(|w| w[1] - w[0]).collect();
    let nd = diffs.len();

    let mut best_period = 1;
    let mut best_error = f64::MAX;

    // Require at least 3 complete cycles: max_period = n / 3
    // This matches Excel behavior: e.g., 24 monthly points → max period 8 (not 12)
    let max_period = n / 3;
    for period in 1..=max_period {
        let num_comparisons = nd.saturating_sub(period);
        if num_comparisons == 0 {
            continue;
        }

        let mut error_sum = 0.0;
        let mut count = 0;
        for i in 0..num_comparisons {
            error_sum += (diffs[i] - diffs[i + period]).abs();
            count += 1;
        }

        if count > 0 {
            let mean_error = error_sum / count as f64;
            if mean_error < best_error {
                best_error = mean_error;
                best_period = period;
            }
        }
    }

    if best_period <= 1 {
        0
    } else {
        best_period
    }
}

// =============================================================================
// Nested bisection optimization for ETS (test-only)
// =============================================================================

#[cfg(test)]
impl EtsModel {
    /// Save the initial states computed by init_data() so they can be
    /// restored before each refill during optimization (preventing per_idx carry-over).
    pub(super) fn save_initial_states(&self) -> InitialStates {
        let m = self.m;
        InitialStates {
            base0: self.base[0],
            trend0: self.trend[0],
            per_idx_init: self.per_idx[0..=m].to_vec(), // 0..m plus sentinel at m
            forecast0: self.forecast[0],
        }
    }

    /// Restore initial states before a refill to get a clean MSE evaluation
    /// independent of previous refill iterations.
    pub(super) fn restore_initial_states(&mut self, init: &InitialStates) {
        self.base[0] = init.base0;
        self.trend[0] = init.trend0;
        self.forecast[0] = init.forecast0;
        for i in 0..init.per_idx_init.len() {
            self.per_idx[i] = init.per_idx_init[i];
        }
    }

    /// Run the model forward with a clean restore of initial states first.
    /// This prevents per_idx carry-over between optimizer iterations.
    pub(super) fn refill_clean(&mut self, init: &InitialStates) {
        self.restore_initial_states(init);
        self.refill();
    }

    pub(super) fn calc_alpha_beta_gamma_bisection(&mut self) {
        let init = self.save_initial_states();

        // Evaluate at alpha = 0.0
        let f0 = 0.0_f64;
        self.alpha = f0;
        self.calc_beta_gamma(&init);
        self.refill_clean(&init);
        let mut fe0 = self.mse;

        // Evaluate at alpha = 1.0
        let f2_init = 1.0_f64;
        self.alpha = f2_init;
        self.calc_beta_gamma(&init);
        self.refill_clean(&init);
        let mut fe2 = self.mse;

        // Evaluate at alpha = 0.5
        let f1_init = 0.5_f64;
        self.alpha = f1_init;
        self.calc_beta_gamma(&init);
        self.refill_clean(&init);

        // If all three give same MSE, set alpha=0
        if fe0 == self.mse && self.mse == fe2 {
            self.alpha = 0.0;
            self.calc_beta_gamma(&init);
            self.refill_clean(&init);
            return;
        }

        let mut f0 = f0;
        let mut f1 = f1_init;
        let mut f2 = f2_init;

        // Bisection loop
        while (f2 - f1) > CF_MIN_ABC_RESOLUTION {
            if fe2 > fe0 {
                f2 = f1;
                fe2 = self.mse;
                f1 = (f0 + f1) / 2.0;
            } else {
                f0 = f1;
                fe0 = self.mse;
                f1 = (f1 + f2) / 2.0;
            }
            self.alpha = f1;
            self.calc_beta_gamma(&init);
            self.refill_clean(&init);
        }

        // Final check: pick best of f0, f1, f2
        if fe2 > fe0 {
            if fe0 < self.mse {
                self.alpha = f0;
                self.calc_beta_gamma(&init);
                self.refill_clean(&init);
            }
        } else {
            if fe2 < self.mse {
                self.alpha = f2;
                self.calc_beta_gamma(&init);
                self.refill_clean(&init);
            }
        }

        self.calc_accuracy_indicators();
    }

    fn calc_beta_gamma(&mut self, init: &InitialStates) {
        // Evaluate at beta = 0.0
        let f0 = 0.0_f64;
        self.beta = f0;
        self.calc_gamma_bisect(init);
        self.refill_clean(init);
        let mut fe0 = self.mse;

        // Evaluate at beta = 1.0
        let f2_init = 1.0_f64;
        self.beta = f2_init;
        self.calc_gamma_bisect(init);
        self.refill_clean(init);
        let mut fe2 = self.mse;

        // Evaluate at beta = 0.5
        let f1_init = 0.5_f64;
        self.beta = f1_init;
        self.calc_gamma_bisect(init);
        self.refill_clean(init);

        if fe0 == self.mse && self.mse == fe2 {
            self.beta = 0.0;
            self.calc_gamma_bisect(init);
            self.refill_clean(init);
            return;
        }

        let mut f0 = f0;
        let mut f1 = f1_init;
        let mut f2 = f2_init;

        while (f2 - f1) > CF_MIN_ABC_RESOLUTION {
            if fe2 > fe0 {
                f2 = f1;
                fe2 = self.mse;
                f1 = (f0 + f1) / 2.0;
            } else {
                f0 = f1;
                fe0 = self.mse;
                f1 = (f1 + f2) / 2.0;
            }
            self.beta = f1;
            self.calc_gamma_bisect(init);
            self.refill_clean(init);
        }

        if fe2 > fe0 {
            if fe0 < self.mse {
                self.beta = f0;
                self.calc_gamma_bisect(init);
                self.refill_clean(init);
            }
        } else {
            if fe2 < self.mse {
                self.beta = f2;
                self.calc_gamma_bisect(init);
                self.refill_clean(init);
            }
        }
    }

    fn calc_gamma_bisect(&mut self, init: &InitialStates) {
        // Evaluate at gamma = 0.0
        let f0 = 0.0_f64;
        self.gamma = f0;
        self.refill_clean(init);
        let mut fe0 = self.mse;

        // Evaluate at gamma = 1.0
        let f2_init = 1.0_f64;
        self.gamma = f2_init;
        self.refill_clean(init);
        let mut fe2 = self.mse;

        // Evaluate at gamma = 0.5
        let f1_init = 0.5_f64;
        self.gamma = f1_init;
        self.refill_clean(init);

        if fe0 == self.mse && self.mse == fe2 {
            self.gamma = 0.0;
            self.refill_clean(init);
            return;
        }

        let mut f0 = f0;
        let mut f1 = f1_init;
        let mut f2 = f2_init;

        while (f2 - f1) > CF_MIN_ABC_RESOLUTION {
            if fe2 > fe0 {
                f2 = f1;
                fe2 = self.mse;
                f1 = (f0 + f1) / 2.0;
            } else {
                f0 = f1;
                fe0 = self.mse;
                f1 = (f1 + f2) / 2.0;
            }
            self.gamma = f1;
            self.refill_clean(init);
        }

        if fe2 > fe0 {
            if fe0 < self.mse {
                self.gamma = f0;
                self.refill_clean(init);
            }
        } else {
            if fe2 < self.mse {
                self.gamma = f2;
                self.refill_clean(init);
            }
        }
    }
}

// =============================================================================
// Preprocessing functions
// =============================================================================

/// Returns (values, timeline, month_day).
/// month_day > 0 means monthly dates were detected and timeline has been
/// converted to month-number space (year*12 + month).
#[allow(clippy::type_complexity)]
pub(super) fn preprocess_data(
    values: &[f64],
    timeline: &[f64],
    data_completion: i32,
    aggregation: i32,
) -> Result<(Vec<f64>, Vec<f64>, i32), Box<dyn Error + Send + Sync>> {
    let mut pairs: Vec<(f64, f64)> = timeline
        .iter()
        .copied()
        .zip(values.iter().copied())
        .collect();
    pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    let (agg_timeline, agg_values) = aggregate_duplicates(&pairs, aggregation);

    if agg_timeline.len() < 2 {
        return Err("FORECAST.ETS: need at least 2 unique timeline points.".into());
    }

    // Detect monthly dates: all dates on the same day-of-month
    let month_day = detect_month_day(&agg_timeline);

    // If monthly, convert timeline to month-number space
    let work_timeline = if month_day > 0 {
        agg_timeline
            .iter()
            .map(|&x| convert_x_to_months(x, month_day))
            .collect()
    } else {
        agg_timeline
    };

    // Compute step in the (possibly converted) timeline
    let step = compute_min_step(&work_timeline)?;

    let (filled_values, filled_timeline) =
        fill_missing_data(&agg_values, &work_timeline, step, data_completion);

    Ok((filled_values, filled_timeline, month_day))
}

// =============================================================================
// Date utilities for month-day detection
// =============================================================================

/// Check if all dates in the timeline share the same day-of-month.
/// Returns the day (1-31) if monthly, or 0 if not.
/// Uses the existing `excel_to_date_time` which correctly handles the Lotus 1900 bug.
fn detect_month_day(timeline: &[f64]) -> i32 {
    if timeline.len() < 2 {
        return 0;
    }

    let mut common_day: Option<u32> = None;
    for &serial in timeline {
        if let Ok(dt) = excel_to_date_time(serial, true) {
            let day = dt.day();
            match common_day {
                None => common_day = Some(day),
                Some(d) if d != day => return 0,
                _ => {}
            }
        } else {
            return 0;
        }
    }
    common_day.unwrap_or(0) as i32
}

/// Convert serial date to month-number space: year*12 + month.
/// Uses the existing `excel_to_date_time` for correct date conversion.
pub(super) fn convert_x_to_months(serial: f64, _month_day: i32) -> f64 {
    if let Ok(dt) = excel_to_date_time(serial, true) {
        (dt.year() as f64) * 12.0 + (dt.month() as f64)
    } else {
        serial // fallback
    }
}

/// Compute minimum positive step in a sorted timeline.
pub(super) fn compute_min_step(timeline: &[f64]) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if timeline.len() < 2 {
        return Err("FORECAST.ETS: need at least 2 timeline points to compute step.".into());
    }

    let mut min_step = f64::MAX;
    for w in timeline.windows(2) {
        let diff = w[1] - w[0];
        if diff > 0.0 && diff < min_step {
            min_step = diff;
        }
    }

    if min_step == f64::MAX || min_step <= 0.0 {
        return Err("FORECAST.ETS: timeline must be strictly increasing.".into());
    }

    Ok(min_step)
}

pub(super) fn aggregate_duplicates(pairs: &[(f64, f64)], method: i32) -> (Vec<f64>, Vec<f64>) {
    if pairs.is_empty() {
        return (vec![], vec![]);
    }

    let mut result_t: Vec<f64> = Vec::new();
    let mut result_v: Vec<f64> = Vec::new();
    let mut current_t = pairs[0].0;
    let mut group: Vec<f64> = vec![pairs[0].1];

    for &(t, v) in &pairs[1..] {
        if (t - current_t).abs() < 1e-10 {
            group.push(v);
        } else {
            result_t.push(current_t);
            result_v.push(aggregate_group(&group, method));
            current_t = t;
            group = vec![v];
        }
    }
    result_t.push(current_t);
    result_v.push(aggregate_group(&group, method));

    (result_t, result_v)
}

pub(super) fn aggregate_group(group: &[f64], method: i32) -> f64 {
    match method {
        1 => group.iter().sum::<f64>() / group.len() as f64,
        2 | 3 => group.len() as f64,
        4 => group.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
        5 => {
            let mut sorted = group.to_vec();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let n = sorted.len();
            if n.is_multiple_of(2) {
                (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
            } else {
                sorted[n / 2]
            }
        }
        6 => group.iter().cloned().fold(f64::INFINITY, f64::min),
        7 => group.iter().sum::<f64>(),
        _ => group.iter().sum::<f64>() / group.len() as f64,
    }
}

fn fill_missing_data(
    values: &[f64],
    timeline: &[f64],
    step: f64,
    data_completion: i32,
) -> (Vec<f64>, Vec<f64>) {
    let mut filled_v: Vec<f64> = Vec::new();
    let mut filled_t: Vec<f64> = Vec::new();

    filled_v.push(values[0]);
    filled_t.push(timeline[0]);

    for i in 1..values.len() {
        let gap_count = ((timeline[i] - timeline[i - 1]) / step).round() as usize;

        if gap_count > 1 {
            for g in 1..gap_count {
                let t = timeline[i - 1] + step * g as f64;
                filled_t.push(t);
                if data_completion == 1 {
                    let fraction = g as f64 / gap_count as f64;
                    let interpolated = values[i - 1] + fraction * (values[i] - values[i - 1]);
                    filled_v.push(interpolated);
                } else {
                    filled_v.push(0.0);
                }
            }
        }

        filled_v.push(values[i]);
        filled_t.push(timeline[i]);
    }

    (filled_v, filled_t)
}
