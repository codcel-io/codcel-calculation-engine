// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::value::Value;
use crate::value_format::ValueFormat;
use std::error::Error;

use crate::database::codcel_daverage::codcel_daverage;
use crate::database::codcel_dcount::codcel_dcount;
use crate::database::codcel_dcounta::codcel_dcounta;
use crate::database::codcel_dget::codcel_dget;
use crate::database::codcel_dmax::codcel_dmax;
use crate::database::codcel_dmin::codcel_dmin;
use crate::database::codcel_dproduct::codcel_dproduct;
use crate::database::codcel_dstdev::codcel_dstdev;
use crate::database::codcel_dstdevp::codcel_dstdevp;
use crate::database::codcel_dsum::codcel_dsum;
use crate::database::codcel_dvar::codcel_dvar;
use crate::database::codcel_dvarp::codcel_dvarp;

pub fn d_average(
    database: Value,
    field: Value,
    criteria: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    codcel_daverage(database, field, criteria, value_format)
}

pub fn d_count(
    database: Value,
    field: Value,
    criteria: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    codcel_dcount(database, field, criteria, value_format)
}

pub fn d_count_a(
    database: Value,
    field: Value,
    criteria: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    codcel_dcounta(database, field, criteria, value_format)
}

pub fn d_get(
    database: Value,
    field: Value,
    criteria: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    codcel_dget(database, field, criteria, value_format)
}

pub fn d_max(
    database: Value,
    field: Value,
    criteria: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    codcel_dmax(database, field, criteria, value_format)
}

pub fn d_min(
    database: Value,
    field: Value,
    criteria: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    codcel_dmin(database, field, criteria, value_format)
}

pub fn d_product(
    database: Value,
    field: Value,
    criteria: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    codcel_dproduct(database, field, criteria, value_format)
}

pub fn d_stdev(
    database: Value,
    field: Value,
    criteria: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    codcel_dstdev(database, field, criteria, value_format)
}

pub fn d_stdev_p(
    database: Value,
    field: Value,
    criteria: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    codcel_dstdevp(database, field, criteria, value_format)
}

pub fn d_sum(
    database: Value,
    field: Value,
    criteria: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    codcel_dsum(database, field, criteria, value_format)
}

pub fn d_var(
    database: Value,
    field: Value,
    criteria: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    codcel_dvar(database, field, criteria, value_format)
}

pub fn d_var_p(
    database: Value,
    field: Value,
    criteria: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    codcel_dvarp(database, field, criteria, value_format)
}
