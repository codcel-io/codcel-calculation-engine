// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::convert::convert_values;
use std::error::Error;

/// Excel-compatible `CONVERT` that converts a number from one measurement unit to another.
/// - `value`: the numeric value to convert.
/// - `from_unit`: the unit to convert from (e.g., `"m"`, `"kg"`, `"F"`).
/// - `to_unit`: the unit to convert to.
///   Returns the converted value, or an error when the unit combination is unsupported.
pub fn codcel_convert<X: AsRef<str>, S: AsRef<str>>(
    value: f64,
    from_unit: X,
    to_unit: S,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    convert_values(value, from_unit.as_ref(), to_unit.as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== Original tests (regression suite) ==========

    #[test]
    fn test_convert_weight() {
        let result = codcel_convert(100.0, "g", "lbm").unwrap();
        assert!((result - 0.220462).abs() < 0.0001);
    }

    #[test]
    fn test_convert_weight_additional() {
        let result = codcel_convert(100.0, "g", "ozm").unwrap();
        assert!((result - 3.52739619).abs() < 0.0001);

        let result = codcel_convert(10.0, "lbm", "g").unwrap();
        assert!((result - 4535.9237).abs() < 0.1);

        let result = codcel_convert(1.0, "tonne", "us_ton").unwrap();
        assert!((result - 1.10231).abs() < 0.0001);
    }

    #[test]
    fn test_convert_distance() {
        let result = codcel_convert(1.0, "mi", "km").unwrap();
        assert!((result - 1.60934).abs() < 0.001);
    }

    #[test]
    fn test_convert_distance_additional() {
        let result = codcel_convert(1.0, "m", "ft").unwrap();
        assert!((result - 3.28084).abs() < 0.0001);

        let result = codcel_convert(10.0, "in", "cm").unwrap();
        assert!((result - 25.4).abs() < 0.0001);

        let result = codcel_convert(1.0, "Nmi", "km").unwrap();
        assert!((result - 1.852).abs() < 0.0001);
    }

    #[test]
    fn test_convert_temperature() {
        let result = codcel_convert(32.0, "F", "C").unwrap();
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_convert_temperature_additional() {
        let result = codcel_convert(0.0, "C", "K").unwrap();
        assert!((result - 273.15).abs() < 0.0001);

        let result = codcel_convert(32.0, "F", "K").unwrap();
        assert!((result - 273.15).abs() < 0.0001);

        let result = codcel_convert(273.15, "K", "Rank").unwrap();
        assert!((result - 491.67).abs() < 0.01);
    }

    #[test]
    fn test_convert_volume() {
        let result = codcel_convert(1.0, "gal", "l").unwrap();
        assert!((result - 3.78541).abs() < 0.0001);
    }

    #[test]
    fn test_convert_volume_additional() {
        let result = codcel_convert(1.0, "l", "in3").unwrap();
        assert!((result - 61.0237).abs() < 0.001);

        let result = codcel_convert(1.0, "tbs", "tsp").unwrap();
        assert!((result - 3.0).abs() < 0.0001);

        let result = codcel_convert(1.0, "us_gal", "uk_gal").unwrap();
        assert!((result - 0.83267).abs() < 0.0001);
    }

    #[test]
    fn test_convert_time() {
        let result = codcel_convert(1.0, "hr", "sec").unwrap();
        assert!((result - 3600.0).abs() < 0.0001);
    }

    #[test]
    fn test_convert_time_additional() {
        let result = codcel_convert(1.0, "day", "hr").unwrap();
        assert!((result - 24.0).abs() < 0.0001);

        let result = codcel_convert(1.0, "yr", "day").unwrap();
        assert!((result - 365.0).abs() < 0.0001);

        let result = codcel_convert(1.0, "min", "sec").unwrap();
        assert!((result - 60.0).abs() < 0.0001);
    }

    #[test]
    fn test_convert_pressure() {
        let result = codcel_convert(101325.0, "Pa", "atm").unwrap();
        assert!((result - 1.0).abs() < 0.0001);

        let result = codcel_convert(1.0, "atm", "mmHg").unwrap();
        assert!((result - 760.0).abs() < 0.1);
    }

    #[test]
    fn test_convert_force() {
        let result = codcel_convert(1.0, "N", "dyn").unwrap();
        assert!((result - 100000.0).abs() < 0.1);

        let result = codcel_convert(1.0, "lbf", "N").unwrap();
        assert!((result - 4.44822).abs() < 0.0001);
    }

    #[test]
    fn test_convert_energy() {
        let result = codcel_convert(4.1868, "J", "cal").unwrap();
        assert!((result - 1.0).abs() < 0.0001);

        let result = codcel_convert(1.0, "BTU", "J").unwrap();
        assert!((result - 1055.06).abs() < 0.01);
    }

    #[test]
    fn test_convert_power() {
        let result = codcel_convert(745.7, "W", "HP").unwrap();
        assert!((result - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_convert_magnetism() {
        let result = codcel_convert(0.0001, "T", "ga").unwrap();
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_convert_area() {
        let result = codcel_convert(1.0, "m2", "ft2").unwrap();
        assert!((result - 10.7639).abs() < 0.0001);

        let result = codcel_convert(2.47105, "acre", "ha").unwrap();
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_convert_speed() {
        let result = codcel_convert(1.60934, "km/h", "mph").unwrap();
        assert!((result - 1.0).abs() < 0.0001);

        let result = codcel_convert(1.0, "knot", "m/s").unwrap();
        assert!((result - 0.514444).abs() < 0.0001);
    }

    #[test]
    fn test_convert_information() {
        let result = codcel_convert(1.0, "byte", "bit").unwrap();
        assert!((result - 8.0).abs() < 0.0001);
    }

    #[test]
    fn test_convert_same_unit() {
        let result = codcel_convert(100.0, "g", "g").unwrap();
        assert_eq!(result, 100.0);
    }

    #[test]
    fn test_convert_unsupported_unit() {
        let result = codcel_convert(100.0, "g", "xyz");
        assert!(result.is_err());
    }

    // ========== Bug fix: Cross-category validation ==========

    #[test]
    fn test_convert_cross_category_error() {
        // Weight to Distance should fail
        let result = codcel_convert(1.0, "g", "m");
        assert!(result.is_err());

        // Distance to Time should fail
        let result = codcel_convert(1.0, "m", "sec");
        assert!(result.is_err());

        // Temperature to Weight should fail
        let result = codcel_convert(100.0, "C", "g");
        assert!(result.is_err());

        // Energy to Power should fail
        let result = codcel_convert(1.0, "J", "W");
        assert!(result.is_err());
    }

    // ========== Bug fix: UK volume units ==========

    #[test]
    fn test_convert_uk_volume_units() {
        // 1 UK gallon = 8 UK pints
        let result = codcel_convert(1.0, "uk_gal", "uk_pt").unwrap();
        assert!((result - 8.0).abs() < 0.001);

        // 1 UK gallon = 4 UK quarts
        let result = codcel_convert(1.0, "uk_gal", "uk_qt").unwrap();
        assert!((result - 4.0).abs() < 0.001);

        // 1 UK quart = 2 UK pints
        let result = codcel_convert(1.0, "uk_qt", "uk_pt").unwrap();
        assert!((result - 2.0).abs() < 0.001);

        // UK pint to liters
        let result = codcel_convert(1.0, "uk_pt", "l").unwrap();
        assert!((result - 0.56826125).abs() < 0.0001);
    }

    // ========== Bug fix: Calorie types ==========

    #[test]
    fn test_convert_calorie_types() {
        // "c" = thermochemical calorie = 4.184 J
        let result = codcel_convert(1.0, "c", "J").unwrap();
        assert!((result - 4.184).abs() < 0.001);

        // "cal" = IT calorie = 4.1868 J
        let result = codcel_convert(1.0, "cal", "J").unwrap();
        assert!((result - 4.1868).abs() < 0.001);

        // They should differ
        let thermo = codcel_convert(1.0, "c", "J").unwrap();
        let it = codcel_convert(1.0, "cal", "J").unwrap();
        assert!((thermo - it).abs() > 0.001);
    }

    // ========== New units: Pressure ==========

    #[test]
    fn test_convert_pressure_new_units() {
        // PSI to Pascal
        let result = codcel_convert(1.0, "psi", "Pa").unwrap();
        assert!((result - 6894.757).abs() < 0.01);

        // 1 atm ≈ 14.696 PSI
        let result = codcel_convert(1.0, "atm", "psi").unwrap();
        assert!((result - 14.696).abs() < 0.01);

        // Torr: 1 atm = 760 Torr (exactly)
        let result = codcel_convert(1.0, "atm", "Torr").unwrap();
        assert!((result - 760.0).abs() < 0.01);

        // Torr to mmHg (should be very close, ~1:1)
        let result = codcel_convert(1.0, "Torr", "mmHg").unwrap();
        assert!((result - 1.0).abs() < 0.001);
    }

    // ========== New units: Force ==========

    #[test]
    fn test_convert_force_pond() {
        // 1 pond = 0.00980665 N
        let result = codcel_convert(1.0, "pond", "N").unwrap();
        assert!((result - 0.00980665).abs() < 0.0000001);

        // 1 N = ~101.972 pond
        let result = codcel_convert(1.0, "N", "pond").unwrap();
        assert!((result - 101.972).abs() < 0.01);
    }

    // ========== New units: Power ==========

    #[test]
    fn test_convert_power_new_units() {
        // PS (metric horsepower) to Watt
        let result = codcel_convert(1.0, "PS", "W").unwrap();
        assert!((result - 735.49875).abs() < 0.01);

        // HP vs PS
        let hp_watts = codcel_convert(1.0, "HP", "W").unwrap();
        let ps_watts = codcel_convert(1.0, "PS", "W").unwrap();
        assert!(hp_watts > ps_watts); // mechanical HP > metric HP

        // "h" alias for HP
        let result = codcel_convert(1.0, "h", "W").unwrap();
        assert!((result - 745.7).abs() < 0.1);
    }

    // ========== New units: Temperature (Reaumur) ==========

    #[test]
    fn test_convert_temperature_reaumur() {
        // 0°C = 0°Ré
        let result = codcel_convert(0.0, "C", "Reau").unwrap();
        assert!((result - 0.0).abs() < 0.0001);

        // 100°C = 80°Ré
        let result = codcel_convert(100.0, "C", "Reau").unwrap();
        assert!((result - 80.0).abs() < 0.0001);

        // 80°Ré = 100°C
        let result = codcel_convert(80.0, "Reau", "C").unwrap();
        assert!((result - 100.0).abs() < 0.0001);

        // 32°F = 0°Ré
        let result = codcel_convert(32.0, "F", "Reau").unwrap();
        assert!((result - 0.0).abs() < 0.0001);

        // 212°F = 80°Ré
        let result = codcel_convert(212.0, "F", "Reau").unwrap();
        assert!((result - 80.0).abs() < 0.0001);

        // 273.15K = 0°Ré
        let result = codcel_convert(273.15, "K", "Reau").unwrap();
        assert!((result - 0.0).abs() < 0.0001);

        // Reaumur to Kelvin
        let result = codcel_convert(80.0, "Reau", "K").unwrap();
        assert!((result - 373.15).abs() < 0.0001);

        // Reaumur to Rankine
        let result = codcel_convert(0.0, "Reau", "Rank").unwrap();
        assert!((result - 491.67).abs() < 0.01);

        // Rankine to Reaumur
        let result = codcel_convert(491.67, "Rank", "Reau").unwrap();
        assert!((result - 0.0).abs() < 0.01);
    }

    // ========== New units: Volume ==========

    #[test]
    fn test_convert_volume_new_units() {
        // Oil barrel = 42 US gallons
        let result = codcel_convert(1.0, "barrel", "gal").unwrap();
        assert!((result - 42.0).abs() < 0.01);

        // Bushel to liters
        let result = codcel_convert(1.0, "bushel", "l").unwrap();
        assert!((result - 35.2391).abs() < 0.001);

        // GRT = 100 cubic feet
        let result = codcel_convert(1.0, "GRT", "ft3").unwrap();
        assert!((result - 100.0).abs() < 0.01);

        // regton alias
        let result = codcel_convert(1.0, "regton", "ft3").unwrap();
        assert!((result - 100.0).abs() < 0.01);

        // MTON = 40 cubic feet
        let result = codcel_convert(1.0, "MTON", "ft3").unwrap();
        assert!((result - 40.0).abs() < 0.01);

        // tspm alias for teaspoon
        let result = codcel_convert(1.0, "tspm", "tsp").unwrap();
        assert!((result - 1.0).abs() < 0.0001);
    }

    // ========== New units: Area ==========

    #[test]
    fn test_convert_area_new_units() {
        // Are = 100 m²
        let result = codcel_convert(1.0, "ar", "m2").unwrap();
        assert!((result - 100.0).abs() < 0.001);

        // Hectare = 100 are
        let result = codcel_convert(1.0, "ha", "ar").unwrap();
        assert!((result - 100.0).abs() < 0.001);

        // Morgen = 2500 m²
        let result = codcel_convert(1.0, "Morgen", "m2").unwrap();
        assert!((result - 2500.0).abs() < 0.01);

        // US survey acre vs international acre (should be slightly different)
        let intl = codcel_convert(1.0, "acre", "m2").unwrap();
        let survey = codcel_convert(1.0, "us_acre", "m2").unwrap();
        assert!(survey > intl);
        assert!((survey - intl).abs() < 0.02);

        // uk_acre alias for acre
        let result = codcel_convert(1.0, "uk_acre", "m2").unwrap();
        assert!((result - intl).abs() < 0.0001);

        // Square kilometer
        let result = codcel_convert(1.0, "km2", "m2").unwrap();
        assert!((result - 1000000.0).abs() < 0.1);

        // Square millimeter
        let result = codcel_convert(1000000.0, "mm2", "m2").unwrap();
        assert!((result - 1.0).abs() < 0.0001);
    }

    // ========== New units: Speed ==========

    #[test]
    fn test_convert_speed_new_units() {
        // Admiralty knot
        let result = codcel_convert(1.0, "admkn", "m/s").unwrap();
        assert!((result - 0.51477).abs() < 0.0001);

        // Meters per hour
        let result = codcel_convert(3600.0, "m/h", "m/s").unwrap();
        assert!((result - 1.0).abs() < 0.0001);

        // m/hr alias
        let result = codcel_convert(3600.0, "m/hr", "m/s").unwrap();
        assert!((result - 1.0).abs() < 0.0001);

        // kn alias for knot
        let result = codcel_convert(1.0, "kn", "m/s").unwrap();
        assert!((result - 0.514444).abs() < 0.0001);
    }

    // ========== New units: Distance ==========

    #[test]
    fn test_convert_distance_new_units() {
        // US survey mile
        let result = codcel_convert(1.0, "survey_mi", "m").unwrap();
        assert!((result - 1609.347).abs() < 0.001);

        // Picapt alias for pica
        let result = codcel_convert(1.0, "Picapt", "m").unwrap();
        let result2 = codcel_convert(1.0, "pica", "m").unwrap();
        assert!((result - result2).abs() < 0.0000001);
    }

    // ========== Feature: Caret notation ==========

    #[test]
    fn test_convert_caret_notation() {
        // m^2 should work like m2
        let result1 = codcel_convert(1.0, "m^2", "ft^2").unwrap();
        let result2 = codcel_convert(1.0, "m2", "ft2").unwrap();
        assert!((result1 - result2).abs() < 0.0001);

        // ft^3 should work like ft3
        let result1 = codcel_convert(1.0, "ft^3", "m^3").unwrap();
        let result2 = codcel_convert(1.0, "ft3", "m3").unwrap();
        assert!((result1 - result2).abs() < 0.0001);

        // Mixed: m^2 to ft2
        let result = codcel_convert(1.0, "m^2", "ft2").unwrap();
        assert!((result - 10.7639).abs() < 0.001);
    }

    // ========== Feature: SI metric prefixes ==========

    #[test]
    fn test_convert_si_prefix_weight() {
        // kg to g
        let result = codcel_convert(1.0, "kg", "g").unwrap();
        assert!((result - 1000.0).abs() < 0.0001);

        // mg to g
        let result = codcel_convert(1000.0, "mg", "g").unwrap();
        assert!((result - 1.0).abs() < 0.0001);

        // µg to g
        let result = codcel_convert(1e6, "ug", "g").unwrap();
        assert!((result - 1.0).abs() < 0.0001);

        // Mg (megagram) to g
        let result = codcel_convert(1.0, "Mg", "g").unwrap();
        assert!((result - 1e6).abs() < 0.1);
    }

    #[test]
    fn test_convert_si_prefix_distance() {
        // km to m (exact match takes priority, same result)
        let result = codcel_convert(1.0, "km", "m").unwrap();
        assert!((result - 1000.0).abs() < 0.0001);

        // cm to m
        let result = codcel_convert(100.0, "cm", "m").unwrap();
        assert!((result - 1.0).abs() < 0.0001);

        // mm to m
        let result = codcel_convert(1000.0, "mm", "m").unwrap();
        assert!((result - 1.0).abs() < 0.0001);

        // nm to m
        let result = codcel_convert(1e9, "nm", "m").unwrap();
        assert!((result - 1.0).abs() < 0.0001);

        // pm to m
        let result = codcel_convert(1e12, "pm", "m").unwrap();
        assert!((result - 1.0).abs() < 0.0001);

        // µm to m (unicode prefix)
        let result = codcel_convert(1e6, "µm", "m").unwrap();
        assert!((result - 1.0).abs() < 0.0001);

        // um to m (ascii alias)
        let result = codcel_convert(1e6, "um", "m").unwrap();
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_convert_si_prefix_energy() {
        // kJ to J
        let result = codcel_convert(1.0, "kJ", "J").unwrap();
        assert!((result - 1000.0).abs() < 0.0001);

        // MJ to J
        let result = codcel_convert(1.0, "MJ", "J").unwrap();
        assert!((result - 1e6).abs() < 0.1);

        // GJ to J
        let result = codcel_convert(1.0, "GJ", "J").unwrap();
        assert!((result - 1e9).abs() < 1.0);
    }

    #[test]
    fn test_convert_si_prefix_power() {
        // kW to W
        let result = codcel_convert(1.0, "kW", "W").unwrap();
        assert!((result - 1000.0).abs() < 0.0001);

        // MW to W
        let result = codcel_convert(1.0, "MW", "W").unwrap();
        assert!((result - 1e6).abs() < 0.1);

        // GW to W
        let result = codcel_convert(1.0, "GW", "W").unwrap();
        assert!((result - 1e9).abs() < 1.0);
    }

    #[test]
    fn test_convert_si_prefix_pressure() {
        // kPa to Pa
        let result = codcel_convert(1.0, "kPa", "Pa").unwrap();
        assert!((result - 1000.0).abs() < 0.0001);

        // hPa to Pa (hecto = 100)
        let result = codcel_convert(1.0, "hPa", "Pa").unwrap();
        assert!((result - 100.0).abs() < 0.0001);

        // MPa to Pa
        let result = codcel_convert(1.0, "MPa", "Pa").unwrap();
        assert!((result - 1e6).abs() < 0.1);
    }

    #[test]
    fn test_convert_si_prefix_force() {
        // kN to N
        let result = codcel_convert(1.0, "kN", "N").unwrap();
        assert!((result - 1000.0).abs() < 0.0001);

        // MN to N
        let result = codcel_convert(1.0, "MN", "N").unwrap();
        assert!((result - 1e6).abs() < 0.1);
    }

    #[test]
    fn test_convert_si_prefix_volume() {
        // ml to l (exact match)
        let result = codcel_convert(1000.0, "ml", "l").unwrap();
        assert!((result - 1.0).abs() < 0.0001);

        // cl (centiliter) to l via prefix
        let result = codcel_convert(100.0, "cl", "l").unwrap();
        assert!((result - 1.0).abs() < 0.0001);

        // kl (kiloliter) to l
        let result = codcel_convert(1.0, "kl", "l").unwrap();
        assert!((result - 1000.0).abs() < 0.0001);
    }

    #[test]
    fn test_convert_si_prefix_cross_prefix() {
        // km to cm (both prefixed)
        let result = codcel_convert(1.0, "km", "cm").unwrap();
        assert!((result - 100000.0).abs() < 0.01);

        // kg to mg
        let result = codcel_convert(1.0, "kg", "mg").unwrap();
        assert!((result - 1e6).abs() < 0.1);

        // kW to MW
        let result = codcel_convert(1000.0, "kW", "MW").unwrap();
        assert!((result - 1.0).abs() < 0.0001);
    }

    // ========== Feature: Binary prefixes ==========

    #[test]
    fn test_convert_binary_prefix() {
        // Kibibyte to byte
        let result = codcel_convert(1.0, "Kibyte", "byte").unwrap();
        assert!((result - 1024.0).abs() < 0.0001);

        // Mebibyte to byte
        let result = codcel_convert(1.0, "Mibyte", "byte").unwrap();
        assert!((result - 1048576.0).abs() < 0.0001);

        // Gibibyte to byte
        let result = codcel_convert(1.0, "Gibyte", "byte").unwrap();
        assert!((result - 1073741824.0).abs() < 0.1);

        // Kibibit to bit
        let result = codcel_convert(1.0, "Kibit", "bit").unwrap();
        assert!((result - 1024.0).abs() < 0.0001);

        // Tebibyte to byte
        let result = codcel_convert(1.0, "Tibyte", "byte").unwrap();
        assert!((result - 1099511627776.0).abs() < 1.0);

        // Kibibyte to Mebibyte
        let result = codcel_convert(1024.0, "Kibyte", "Mibyte").unwrap();
        assert!((result - 1.0).abs() < 0.0001);

        // Binary to SI: Kibibyte to kbyte (kilobyte)
        let result = codcel_convert(1.0, "Kibyte", "kbyte").unwrap();
        assert!((result - 1.024).abs() < 0.001);
    }

    #[test]
    fn test_convert_si_prefix_information() {
        // kbyte (kilobyte) to byte
        let result = codcel_convert(1.0, "kbyte", "byte").unwrap();
        assert!((result - 1000.0).abs() < 0.0001);

        // Mbyte (megabyte) to byte
        let result = codcel_convert(1.0, "Mbyte", "byte").unwrap();
        assert!((result - 1e6).abs() < 0.1);

        // Gbit (gigabit) to bit
        let result = codcel_convert(1.0, "Gbit", "bit").unwrap();
        assert!((result - 1e9).abs() < 1.0);

        // kbit to bit
        let result = codcel_convert(1.0, "kbit", "bit").unwrap();
        assert!((result - 1000.0).abs() < 0.0001);
    }

    // ========== Aliases ==========

    #[test]
    fn test_convert_aliases() {
        // LTON alias for long ton
        let result = codcel_convert(1.0, "LTON", "g").unwrap();
        let result2 = codcel_convert(1.0, "uk_ton", "g").unwrap();
        assert!((result - result2).abs() < 0.0001);
    }
}
