/*!
SPDX-FileCopyrightText: Copyright (c) 2026 Codcel.
SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial

Codcel Calculation Engine
https://codcel.io

Licensing: MIT or Apache-2.0 for permitted uses, or the Codcel Commercial License for
Automated Code Generation Systems (excluding Codcel itself). See the LICENSE files
in the project root for full terms.
*/



pub(crate) mod area;
pub mod arithmetic_base;
pub mod codcel_cache;
pub mod array_base;
pub mod codcel_information;
pub mod comparison;
pub mod compatibility;
pub mod compatibility_base;
mod condition;
pub mod condition_base;
pub mod convert;
pub mod date_and_time;
pub mod date_time_base;
pub mod engineering;
pub mod engineering_base;
pub mod financial;
pub mod financial_base;
mod information;
pub mod input;
pub mod logical;
pub mod lookup_and_reference;
pub mod lookup_reference_base;
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
pub mod map_helpers;

#[cfg(test)]
mod value_tests;
