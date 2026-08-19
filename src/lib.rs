// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

//! Codcel Calculation Engine
//!
//! Rust implementations of Excel-compatible functions, covering the financial,
//! statistical, mathematical, engineering, text, date and lookup categories.
//! Excel behavioural fidelity is preserved, including banker's rounding, the
//! 1900 date serial system, and the Lotus 1-2-3 1900 leap-year bug.
//!
//! Set `CODCEL_USE_PORTABLE_MATH=true` to compute transcendental functions via
//! `libm` for bit-identical results across platforms.
//!
//! The crate builds for `wasm32-unknown-unknown` as well as native targets. Locale
//! detection, environment overrides and random seeding all degrade to fixed defaults
//! inside a WASM sandbox — see the README's WebAssembly section for the details.

pub(crate) mod area;
pub mod arithmetic_base;
pub mod array_base;
pub mod codcel_cache;
pub mod codcel_information;
pub mod comparison;
pub mod compatibility;
pub mod compatibility_base;
pub mod compensated_sum;
mod condition;
pub mod condition_base;
pub mod convert;
pub mod database;
pub mod database_base;
pub mod date_and_time;
pub mod date_time_base;
pub mod engineering;
pub mod engineering_base;
pub mod excel_error;
pub mod financial;
pub mod financial_base;
mod information;
pub mod input;
pub mod logical;
pub mod lookup_and_reference;
pub mod lookup_reference_base;
pub mod map_helpers;
mod match_type_and_compare_macro;
pub mod maths;
pub mod portable_math;
pub mod rounding;
pub mod statistical;
pub mod statistical_base;
pub mod text;
pub mod text_base;
mod text_function;
pub mod to_bool;
pub mod to_f64;
pub mod to_i32;
pub mod to_vec_f64;
pub mod value;
pub mod value_format;

#[cfg(test)]
mod value_tests;
