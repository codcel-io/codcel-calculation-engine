// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

#[derive(Debug, PartialEq, Clone, Copy)]
enum UnitCategory {
    Weight,
    Distance,
    Time,
    Pressure,
    Force,
    Energy,
    Power,
    Magnetism,
    Temperature,
    Volume,
    Area,
    Speed,
    Information,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Unit {
    // Weight and Mass
    Gram,
    Pound,
    Slug,
    AtomicMass,
    OunceMass,
    Ton,
    Stone,
    Hundredweight,
    ShortHundredweight,
    Grain,
    LongTon,
    ShortTon,
    MetricTon,

    // Distance
    Meter,
    StatuteMile,
    NauticalMile,
    Inch,
    Foot,
    Yard,
    Angstrom,
    Pica,
    Point,
    Ell,
    LightYear,
    Parsec,
    USSurveyMile,

    // Time
    Year,
    Day,
    Hour,
    Minute,
    Second,

    // Pressure
    Pascal,
    Atmosphere,
    MmHg,
    Psi,
    Torr,

    // Force
    Newton,
    Dyne,
    PoundForce,
    Pond,

    // Energy
    Joule,
    Erg,
    ITCalorie,
    ThermochemicalCalorie,
    ElectronVolt,
    HorsepowerHour,
    WattHour,
    FootPound,
    Btu,

    // Power
    Horsepower,
    MetricHorsepower,
    Watt,

    // Magnetism
    Tesla,
    Gauss,

    // Temperature
    Celsius,
    Fahrenheit,
    Kelvin,
    Rankine,
    Reaumur,

    // Volume
    Teaspoon,
    Tablespoon,
    USFluidOunce,
    USCup,
    USPint,
    USQuart,
    USGallon,
    UKGallon,
    UKQuart,
    UKPint,
    Liter,
    CubicAngstrom,
    CubicCentimeter,
    CubicFoot,
    CubicInch,
    CubicMeter,
    CubicMile,
    CubicYard,
    OilBarrel,
    Bushel,
    GrossRegisteredTon,
    MeasurementTon,

    // Area
    SquareAngstrom,
    SquareCentimeter,
    SquareFoot,
    SquareInch,
    SquareKilometer,
    SquareMeter,
    SquareMillimeter,
    SquareMile,
    SquareYard,
    Acre,
    USSurveyAcre,
    Hectare,
    Are,
    Morgen,

    // Speed
    MetersPerSecond,
    MetersPerHour,
    MilesPerHour,
    KilometersPerHour,
    Knot,
    AdmiraltyKnot,

    // Information
    Bit,
    Byte,
}

impl Unit {
    fn category(&self) -> UnitCategory {
        use Unit::*;
        match self {
            Gram | Pound | Slug | AtomicMass | OunceMass | Ton | Stone | Hundredweight
            | ShortHundredweight | Grain | LongTon | ShortTon | MetricTon => UnitCategory::Weight,

            Meter | StatuteMile | NauticalMile | Inch | Foot | Yard | Angstrom | Pica | Point
            | Ell | LightYear | Parsec | USSurveyMile => UnitCategory::Distance,

            Year | Day | Hour | Minute | Second => UnitCategory::Time,

            Pascal | Atmosphere | MmHg | Psi | Torr => UnitCategory::Pressure,

            Newton | Dyne | PoundForce | Pond => UnitCategory::Force,

            Joule | Erg | ITCalorie | ThermochemicalCalorie | ElectronVolt | HorsepowerHour
            | WattHour | FootPound | Btu => UnitCategory::Energy,

            Horsepower | MetricHorsepower | Watt => UnitCategory::Power,

            Tesla | Gauss => UnitCategory::Magnetism,

            Celsius | Fahrenheit | Kelvin | Rankine | Reaumur => UnitCategory::Temperature,

            Teaspoon | Tablespoon | USFluidOunce | USCup | USPint | USQuart | USGallon
            | UKGallon | UKQuart | UKPint | Liter | CubicAngstrom | CubicCentimeter | CubicFoot
            | CubicInch | CubicMeter | CubicMile | CubicYard | OilBarrel | Bushel
            | GrossRegisteredTon | MeasurementTon => UnitCategory::Volume,

            SquareAngstrom | SquareCentimeter | SquareFoot | SquareInch | SquareKilometer
            | SquareMeter | SquareMillimeter | SquareMile | SquareYard | Acre | USSurveyAcre
            | Hectare | Are | Morgen => UnitCategory::Area,

            MetersPerSecond | MetersPerHour | MilesPerHour | KilometersPerHour | Knot
            | AdmiraltyKnot => UnitCategory::Speed,

            Bit | Byte => UnitCategory::Information,
        }
    }

    fn base_factor(&self) -> f64 {
        use Unit::*;
        match self {
            // Weight and Mass - base: Gram
            Gram => 1.0,
            Pound => 453.59237,
            Slug => 14593.9029,
            AtomicMass => 1.66053886e-24,
            OunceMass => 28.349523125,
            Ton => 1016046.9088,
            Stone => 6350.29318,
            Hundredweight => 50802.34544,
            ShortHundredweight => 45359.237,
            Grain => 0.06479891,
            LongTon => 1016046.9088,
            ShortTon => 907184.74,
            MetricTon => 1000000.0,

            // Distance - base: Meter
            Meter => 1.0,
            StatuteMile => 1609.344,
            NauticalMile => 1852.0,
            Inch => 0.0254,
            Foot => 0.3048,
            Yard => 0.9144,
            Angstrom => 1e-10,
            Pica => 0.00423333333,
            Point => 0.000352777778,
            Ell => 1.143,
            LightYear => 9.46073e+15,
            Parsec => 3.08567758e+16,
            USSurveyMile => 1609.3472186944,

            // Time - base: Second
            Second => 1.0,
            Minute => 60.0,
            Hour => 3600.0,
            Day => 86400.0,
            Year => 31536000.0,

            // Pressure - base: Pascal
            Pascal => 1.0,
            Atmosphere => 101325.0,
            MmHg => 133.322,
            Psi => 6894.757293168,
            Torr => 133.3223684211,

            // Force - base: Newton
            Newton => 1.0,
            Dyne => 1e-5,
            PoundForce => 4.44822,
            Pond => 0.00980665,

            // Energy - base: Joule
            Joule => 1.0,
            Erg => 1e-7,
            ITCalorie => 4.1868,
            ThermochemicalCalorie => 4.184,
            ElectronVolt => 1.602176565e-19,
            HorsepowerHour => 2684519.538,
            WattHour => 3600.0,
            FootPound => 1.355817948,
            Btu => 1055.05585262,

            // Power - base: Watt
            Watt => 1.0,
            Horsepower => 745.69987158227,
            MetricHorsepower => 735.49875,

            // Magnetism - base: Tesla
            Tesla => 1.0,
            Gauss => 0.0001,

            // Temperature - special case (factor not used directly)
            Celsius | Fahrenheit | Kelvin | Rankine | Reaumur => 1.0,

            // Volume - base: Liter
            Liter => 1.0,
            Teaspoon => 0.00492892159375,
            Tablespoon => 0.01478676478125,
            USFluidOunce => 0.0295735295625,
            USCup => 0.2365882365,
            USPint => 0.473176473,
            USQuart => 0.946352946,
            USGallon => 3.78541178,
            UKGallon => 4.54609,
            UKQuart => 1.1365225,
            UKPint => 0.56826125,
            CubicAngstrom => 1e-30,
            CubicCentimeter => 0.001,
            CubicFoot => 28.316846592,
            CubicInch => 0.016387064,
            CubicMeter => 1000.0,
            CubicMile => 4.168181825e+12,
            CubicYard => 764.554857984,
            OilBarrel => 158.987294928,
            Bushel => 35.23907016688,
            GrossRegisteredTon => 2831.6846592,
            MeasurementTon => 1132.67386368,

            // Area - base: Square Meter
            SquareMeter => 1.0,
            SquareAngstrom => 1e-20,
            SquareCentimeter => 0.0001,
            SquareFoot => 0.09290304,
            SquareInch => 0.00064516,
            SquareKilometer => 1000000.0,
            SquareMillimeter => 0.000001,
            SquareMile => 2589988.110336,
            SquareYard => 0.83612736,
            Acre => 4046.8564224,
            USSurveyAcre => 4046.872609874252,
            Hectare => 10000.0,
            Are => 100.0,
            Morgen => 2500.0,

            // Speed - base: Meters per Second
            MetersPerSecond => 1.0,
            MetersPerHour => 1.0 / 3600.0,
            MilesPerHour => 0.44704,
            KilometersPerHour => 0.277777778,
            Knot => 0.514444444,
            AdmiraltyKnot => 0.51477333333,

            // Information - base: Bit
            Bit => 1.0,
            Byte => 8.0,
        }
    }
}

/// Normalize caret notation: "m^2" -> "m2", "ft^3" -> "ft3"
fn normalize_caret(input: &str) -> String {
    input.replace("^2", "2").replace("^3", "3")
}

/// Try to match an exact unit string to a Unit enum variant.
fn exact_unit_match(unit: &str) -> Option<Unit> {
    match unit {
        // Weight and Mass
        "g" => Some(Unit::Gram),
        "sg" => Some(Unit::Slug),
        "lbm" => Some(Unit::Pound),
        "u" => Some(Unit::AtomicMass),
        "ozm" => Some(Unit::OunceMass),
        "ton" => Some(Unit::Ton),
        "stone" => Some(Unit::Stone),
        "cwt" => Some(Unit::Hundredweight),
        "shweight" => Some(Unit::ShortHundredweight),
        "grain" => Some(Unit::Grain),
        "uk_ton" | "lcwt" | "LTON" => Some(Unit::LongTon),
        "s_ton" | "us_ton" => Some(Unit::ShortTon),
        "m_ton" | "tonne" => Some(Unit::MetricTon),

        // Distance
        "m" => Some(Unit::Meter),
        "mi" => Some(Unit::StatuteMile),
        "Nmi" | "nmi" => Some(Unit::NauticalMile),
        "in" => Some(Unit::Inch),
        "ft" => Some(Unit::Foot),
        "yd" => Some(Unit::Yard),
        "ang" => Some(Unit::Angstrom),
        "pica" | "Pica" | "Picapt" => Some(Unit::Pica),
        "pt" => Some(Unit::Point),
        "ell" => Some(Unit::Ell),
        "ly" => Some(Unit::LightYear),
        "parsec" | "pc" => Some(Unit::Parsec),
        "survey_mi" => Some(Unit::USSurveyMile),

        // Hardcoded metric distance variants (kept for backward compat, also matched by prefix)
        "km" => Some(Unit::Meter),   // handled via prefix path, but keep for exact match
        "cm" => Some(Unit::Meter),   // handled via prefix path
        "mm" => Some(Unit::Meter),   // handled via prefix path
        "µm" | "um" => Some(Unit::Meter), // handled via prefix path
        "nm" => Some(Unit::Meter),   // handled via prefix path
        "pm" => Some(Unit::Meter),   // handled via prefix path

        // Time
        "yr" => Some(Unit::Year),
        "day" | "d" => Some(Unit::Day),
        "hr" => Some(Unit::Hour),
        "mn" | "min" => Some(Unit::Minute),
        "sec" | "s" => Some(Unit::Second),

        // Pressure
        "Pa" | "pa" => Some(Unit::Pascal),
        "atm" | "at" => Some(Unit::Atmosphere),
        "mmHg" => Some(Unit::MmHg),
        "psi" => Some(Unit::Psi),
        "Torr" => Some(Unit::Torr),

        // Force
        "N" => Some(Unit::Newton),
        "dyn" | "dyne" => Some(Unit::Dyne),
        "lbf" => Some(Unit::PoundForce),
        "pond" => Some(Unit::Pond),

        // Energy
        "J" | "j" => Some(Unit::Joule),
        "e" | "erg" => Some(Unit::Erg),
        "c" => Some(Unit::ThermochemicalCalorie),
        "cal" => Some(Unit::ITCalorie),
        "eV" | "ev" => Some(Unit::ElectronVolt),
        "HPh" | "hh" => Some(Unit::HorsepowerHour),
        "Wh" | "wh" => Some(Unit::WattHour),
        "flb" => Some(Unit::FootPound),
        "BTU" | "btu" => Some(Unit::Btu),

        // Power
        "HP" | "hp" | "h" => Some(Unit::Horsepower),
        "PS" => Some(Unit::MetricHorsepower),
        "W" | "w" => Some(Unit::Watt),

        // Magnetism
        "T" => Some(Unit::Tesla),
        "ga" => Some(Unit::Gauss),

        // Temperature
        "C" | "cel" => Some(Unit::Celsius),
        "F" | "fah" => Some(Unit::Fahrenheit),
        "K" | "kel" => Some(Unit::Kelvin),
        "Rank" | "rank" => Some(Unit::Rankine),
        "Reau" => Some(Unit::Reaumur),

        // Volume
        "tsp" | "tspm" => Some(Unit::Teaspoon),
        "tbs" => Some(Unit::Tablespoon),
        "oz" => Some(Unit::USFluidOunce),
        "cup" => Some(Unit::USCup),
        "us_pt" => Some(Unit::USPint),
        "qt" | "us_qt" => Some(Unit::USQuart),
        "gal" | "us_gal" => Some(Unit::USGallon),
        "uk_gal" => Some(Unit::UKGallon),
        "uk_qt" => Some(Unit::UKQuart),
        "uk_pt" => Some(Unit::UKPint),
        "l" | "L" | "lt" => Some(Unit::Liter),
        "ang3" => Some(Unit::CubicAngstrom),
        "cm3" => Some(Unit::CubicCentimeter),
        "ft3" => Some(Unit::CubicFoot),
        "in3" => Some(Unit::CubicInch),
        "m3" => Some(Unit::CubicMeter),
        "mi3" => Some(Unit::CubicMile),
        "yd3" => Some(Unit::CubicYard),
        "ml" => Some(Unit::Liter), // handled via prefix path too
        "barrel" => Some(Unit::OilBarrel),
        "bushel" => Some(Unit::Bushel),
        "GRT" | "regton" => Some(Unit::GrossRegisteredTon),
        "MTON" => Some(Unit::MeasurementTon),

        // Area
        "ang2" => Some(Unit::SquareAngstrom),
        "cm2" => Some(Unit::SquareCentimeter),
        "ft2" => Some(Unit::SquareFoot),
        "in2" => Some(Unit::SquareInch),
        "km2" => Some(Unit::SquareKilometer),
        "m2" => Some(Unit::SquareMeter),
        "mm2" => Some(Unit::SquareMillimeter),
        "mi2" => Some(Unit::SquareMile),
        "yd2" => Some(Unit::SquareYard),
        "acre" | "uk_acre" => Some(Unit::Acre),
        "us_acre" => Some(Unit::USSurveyAcre),
        "ha" => Some(Unit::Hectare),
        "ar" => Some(Unit::Are),
        "Morgen" => Some(Unit::Morgen),

        // Speed
        "m/s" | "m/sec" => Some(Unit::MetersPerSecond),
        "m/h" | "m/hr" => Some(Unit::MetersPerHour),
        "mph" => Some(Unit::MilesPerHour),
        "km/h" | "kph" => Some(Unit::KilometersPerHour),
        "knot" | "kn" => Some(Unit::Knot),
        "admkn" => Some(Unit::AdmiraltyKnot),

        // Information
        "bit" => Some(Unit::Bit),
        "byte" => Some(Unit::Byte),

        _ => None,
    }
}

/// Check if a unit string is a metric base unit that accepts SI prefixes.
/// Returns the Unit if it's a valid metric base.
fn metric_base_unit(unit: &str) -> Option<Unit> {
    match unit {
        "g" => Some(Unit::Gram),
        "m" => Some(Unit::Meter),
        "s" | "sec" => Some(Unit::Second),
        "Pa" | "pa" => Some(Unit::Pascal),
        "N" => Some(Unit::Newton),
        "dyn" | "dyne" => Some(Unit::Dyne),
        "pond" => Some(Unit::Pond),
        "J" | "j" => Some(Unit::Joule),
        "e" | "erg" => Some(Unit::Erg),
        "eV" | "ev" => Some(Unit::ElectronVolt),
        "Wh" | "wh" => Some(Unit::WattHour),
        "W" | "w" => Some(Unit::Watt),
        "T" => Some(Unit::Tesla),
        "ga" => Some(Unit::Gauss),
        "l" | "L" | "lt" => Some(Unit::Liter),
        "bit" => Some(Unit::Bit),
        "byte" => Some(Unit::Byte),
        _ => None,
    }
}

/// Try to parse an SI metric prefix, returning its multiplier.
fn si_prefix(prefix: &str) -> Option<f64> {
    match prefix {
        "Y" => Some(1e24),
        "Z" => Some(1e21),
        "E" => Some(1e18),
        "P" => Some(1e15),
        "T" => Some(1e12),
        "G" => Some(1e9),
        "M" => Some(1e6),
        "k" => Some(1e3),
        "h" => Some(1e2),
        "da" => Some(1e1),
        "d" => Some(1e-1),
        "c" => Some(1e-2),
        "m" => Some(1e-3),
        "u" | "µ" => Some(1e-6),
        "n" => Some(1e-9),
        "p" => Some(1e-12),
        "f" => Some(1e-15),
        "a" => Some(1e-18),
        "z" => Some(1e-21),
        "y" => Some(1e-24),
        _ => None,
    }
}

/// Try to parse a binary prefix (IEC), returning its multiplier.
fn binary_prefix(prefix: &str) -> Option<f64> {
    match prefix {
        "Ki" => Some(1024.0),
        "Mi" => Some(1048576.0),
        "Gi" => Some(1073741824.0),
        "Ti" => Some(1099511627776.0),
        "Pi" => Some(1125899906842624.0),
        "Ei" => Some(1152921504606846976.0),
        "Zi" => Some(1180591620717411303424.0),
        "Yi" => Some(1208925819614629174706176.0),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy)]
struct ParsedUnit {
    unit: Unit,
    prefix_multiplier: f64,
}

impl ParsedUnit {
    fn parse(input: &str) -> Result<ParsedUnit, Box<dyn Error + Send + Sync>> {
        // First try exact match (highest priority)
        if let Some(unit) = exact_unit_match(input) {
            // For backward-compatible hardcoded metric variants, compute the correct multiplier
            let (resolved_unit, multiplier) = match input {
                "km" => (Unit::Meter, 1e3),
                "cm" => (Unit::Meter, 1e-2),
                "mm" => (Unit::Meter, 1e-3),
                "µm" | "um" => (Unit::Meter, 1e-6),
                "nm" => (Unit::Meter, 1e-9),
                "pm" => (Unit::Meter, 1e-12),
                "ml" => (Unit::Liter, 1e-3),
                _ => (unit, 1.0),
            };
            return Ok(ParsedUnit {
                unit: resolved_unit,
                prefix_multiplier: multiplier,
            });
        }

        // Try binary prefixes (2-char, only for bit/byte)
        if input.len() > 2 {
            let (prefix, remainder) = input.split_at(2);
            if let Some(multiplier) = binary_prefix(prefix) {
                if remainder == "bit" || remainder == "byte" {
                    let unit = if remainder == "bit" {
                        Unit::Bit
                    } else {
                        Unit::Byte
                    };
                    return Ok(ParsedUnit {
                        unit,
                        prefix_multiplier: multiplier,
                    });
                }
            }
        }

        // Try SI prefixes: two-character prefix first ("da")
        if input.len() > 2 {
            let (prefix, remainder) = input.split_at(2);
            if let Some(multiplier) = si_prefix(prefix) {
                if let Some(unit) = metric_base_unit(remainder) {
                    return Ok(ParsedUnit {
                        unit,
                        prefix_multiplier: multiplier,
                    });
                }
            }
        }

        // Try SI prefixes: one-character prefix
        if input.len() > 1 {
            let (prefix, remainder) = input.split_at(1);
            if let Some(multiplier) = si_prefix(prefix) {
                if let Some(unit) = metric_base_unit(remainder) {
                    return Ok(ParsedUnit {
                        unit,
                        prefix_multiplier: multiplier,
                    });
                }
            }
            // Also try with µ (2 bytes in UTF-8)
            if input.starts_with('µ') {
                let remainder = &input['µ'.len_utf8()..];
                if let Some(unit) = metric_base_unit(remainder) {
                    return Ok(ParsedUnit {
                        unit,
                        prefix_multiplier: 1e-6,
                    });
                }
            }
        }

        Err(format!("CONVERT: Unit {input} is not supported").into())
    }
}

pub(crate) fn convert_values(
    value: f64,
    from_unit_str: &str,
    to_unit_str: &str,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Normalize caret notation
    let from_normalized = normalize_caret(from_unit_str);
    let to_normalized = normalize_caret(to_unit_str);

    let from_parsed = ParsedUnit::parse(&from_normalized)?;
    let to_parsed = ParsedUnit::parse(&to_normalized)?;

    let from_unit = from_parsed.unit;
    let to_unit = to_parsed.unit;

    // Validate same category
    if from_unit.category() != to_unit.category() {
        return Err(format!(
            "CONVERT: Cannot convert between different unit categories ({from_unit_str} and {to_unit_str})"
        )
        .into());
    }

    // Same unit with same prefix
    if from_unit == to_unit && from_parsed.prefix_multiplier == to_parsed.prefix_multiplier {
        return Ok(value);
    }

    // Special case for temperature conversions
    if from_unit.category() == UnitCategory::Temperature {
        // Apply prefix (unusual for temperature but technically possible)
        let prefixed_value = value * from_parsed.prefix_multiplier;
        let converted = convert_temperature(prefixed_value, from_unit, to_unit)?;
        return Ok(converted / to_parsed.prefix_multiplier);
    }

    // Standard conversion: value * from_prefix * from_factor / to_factor / to_prefix
    let base_value = value * from_parsed.prefix_multiplier * from_unit.base_factor();
    let result = base_value / (to_parsed.prefix_multiplier * to_unit.base_factor());

    Ok(result)
}

fn convert_temperature(
    value: f64,
    from: Unit,
    to: Unit,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    use Unit::*;
    if from == to {
        return Ok(value);
    }
    match (from, to) {
        // Celsius conversions
        (Celsius, Fahrenheit) => Ok((value * 9.0 / 5.0) + 32.0),
        (Celsius, Kelvin) => Ok(value + 273.15),
        (Celsius, Rankine) => Ok((value + 273.15) * 9.0 / 5.0),
        (Celsius, Reaumur) => Ok(value * 4.0 / 5.0),

        // Fahrenheit conversions
        (Fahrenheit, Celsius) => Ok((value - 32.0) * 5.0 / 9.0),
        (Fahrenheit, Kelvin) => Ok(((value - 32.0) * 5.0 / 9.0) + 273.15),
        (Fahrenheit, Rankine) => Ok(value + 459.67),
        (Fahrenheit, Reaumur) => Ok((value - 32.0) * 4.0 / 9.0),

        // Kelvin conversions
        (Kelvin, Celsius) => Ok(value - 273.15),
        (Kelvin, Fahrenheit) => Ok(((value - 273.15) * 9.0 / 5.0) + 32.0),
        (Kelvin, Rankine) => Ok(value * 9.0 / 5.0),
        (Kelvin, Reaumur) => Ok((value - 273.15) * 4.0 / 5.0),

        // Rankine conversions
        (Rankine, Celsius) => Ok((value * 5.0 / 9.0) - 273.15),
        (Rankine, Fahrenheit) => Ok(value - 459.67),
        (Rankine, Kelvin) => Ok(value * 5.0 / 9.0),
        (Rankine, Reaumur) => Ok((value * 5.0 / 9.0 - 273.15) * 4.0 / 5.0),

        // Reaumur conversions
        (Reaumur, Celsius) => Ok(value * 5.0 / 4.0),
        (Reaumur, Fahrenheit) => Ok((value * 9.0 / 4.0) + 32.0),
        (Reaumur, Kelvin) => Ok((value * 5.0 / 4.0) + 273.15),
        (Reaumur, Rankine) => Ok((value * 5.0 / 4.0 + 273.15) * 9.0 / 5.0),

        _ => Ok(value),
    }
}
