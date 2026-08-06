// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

//! Financial functions mirroring Excel equivalents.
//!
//! Each submodule implements a single Excel-compatible financial function,
//! keeping parameter ordering and behavior as close as possible to the
//! spreadsheet originals. Errors are surfaced as `Result` values rather
//! than returning Excel error codes, making the API ergonomic for Rust
//! applications while preserving familiar semantics.
pub(crate) mod helpers;

// Financial functions
pub mod codcel_accr_int;
pub mod codcel_accr_intm;
pub mod codcel_amor_degrc;
pub mod codcel_amor_linc;
// Uncomment as files are created
pub mod codcel_coup_day_bs;
pub mod codcel_coup_days;
pub mod codcel_coup_days_nc;
pub mod codcel_coup_ncd;
pub mod codcel_coup_num;
pub mod codcel_coup_pcd;
pub mod codcel_cum_i_pmt;
pub mod codcel_cum_princ;
pub mod codcel_db;
pub mod codcel_ddb;
pub mod codcel_disc;
pub mod codcel_dollar_de;
pub mod codcel_dollar_fr;
pub mod codcel_duration;
pub mod codcel_effect;
pub mod codcel_fv;
pub mod codcel_fv_schedule;
pub mod codcel_i_pmt;
pub mod codcel_int_rate;
pub mod codcel_irr;
pub mod codcel_is_pmt;
pub mod codcel_m_duration;
pub mod codcel_m_irr;
pub mod codcel_n_per;
pub mod codcel_nominal;
pub mod codcel_npv;
pub mod codcel_odd_f_price;
pub mod codcel_odd_f_yield;
pub mod codcel_odd_l_price;
pub mod codcel_odd_l_yield;
pub mod codcel_p_duration;
pub mod codcel_p_pmt;
pub mod codcel_pmt;
pub mod codcel_price;
pub mod codcel_price_disc;
pub mod codcel_price_mat;
pub mod codcel_pv;
pub mod codcel_rate;
pub mod codcel_received;
pub mod codcel_rri;
pub mod codcel_sln;
pub mod codcel_syd;
pub mod codcel_t_bill_eq;
pub mod codcel_t_bill_price;
pub mod codcel_t_bill_yield;
pub mod codcel_vdb;
pub mod codcel_x_irr;
pub mod codcel_x_npv;
pub mod codcel_yield;
pub mod codcel_yield_disc;
pub mod codcel_yield_mat;
