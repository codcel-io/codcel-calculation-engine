// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::date_system::DateSemantics;
use serde::{Deserialize, Serialize};

/// Locale and calculation settings carried alongside every value.
///
/// `#[serde(default)]` is applied at the container level so that a JSON payload
/// written against an older version of this struct still deserialises: the
/// generated FFI and JNI `*_with_format` entry points parse caller-supplied
/// JSON straight into this type, and a newly added field must not break them.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct ValueFormat {
    pub decimal_separator: String,
    pub currency_symbol: String,
    pub thousands_separator: String,
    pub use_excel_rounding: bool,
    pub language: String,
    pub allow_lotus_1_2_3_1900_date_bug: bool,
}

/// Hand-written rather than derived: `#[derive(Default)]` would give
/// `decimal_separator: ""`, which makes every formatted number unparseable.
/// These values match the fallback that `from_language` uses for an unknown
/// language tag.
impl Default for ValueFormat {
    fn default() -> Self {
        ValueFormat {
            decimal_separator: ".".to_string(),
            currency_symbol: "$".to_string(),
            thousands_separator: ",".to_string(),
            use_excel_rounding: false,
            language: "en".to_string(),
            allow_lotus_1_2_3_1900_date_bug: true,
        }
    }
}

impl ValueFormat {
    /// Creates a `ValueFormat` by layering:
    /// 1. OS locale detection (via `sys-locale`)
    /// 2. `CODCEL_*` environment variable overrides
    /// 3. Transpiler-provided `fallback` values for anything not resolved above
    ///
    /// Works on macOS, Linux, and Windows.
    /// For WASM targets (where locale detection and env vars are unavailable),
    /// use `from_env_with_fallback` instead.
    pub fn from_system_with_env(fallback: ValueFormat) -> ValueFormat {
        let system = Self::detect_system_locale(&fallback);
        Self::overlay_env(system)
    }

    /// Creates a `ValueFormat` by layering `CODCEL_*` environment variable
    /// overrides on top of the provided `fallback` values.
    /// No OS locale detection — suitable for WASM or environments where
    /// system locale is not meaningful.
    pub fn from_env_with_fallback(fallback: ValueFormat) -> ValueFormat {
        Self::overlay_env(fallback)
    }

    /// Creates a `ValueFormat` from a language tag (e.g. "en-US", "de", "fr-FR").
    /// Maps the language to locale-appropriate formatting conventions.
    /// For unknown languages, returns sensible defaults (period decimal, comma thousands, `$`).
    pub fn from_language(lang: &str) -> ValueFormat {
        Self::from_language_internal(lang, &ValueFormat::default())
    }

    /// Creates a `ValueFormat` from a language tag with `CODCEL_*` env var overrides.
    pub fn from_language_with_env(lang: &str) -> ValueFormat {
        Self::overlay_env(Self::from_language(lang))
    }

    /// Detect system locale and map to formatting conventions.
    fn detect_system_locale(fallback: &ValueFormat) -> ValueFormat {
        let locale_tag = sys_locale::get_locale().unwrap_or_default();
        Self::from_language_internal(&locale_tag, fallback)
    }

    fn from_language_internal(lang_tag: &str, fallback: &ValueFormat) -> ValueFormat {
        let parts: Vec<&str> = lang_tag.split(['-', '_']).collect();
        let lang = parts.first().map(|s| s.to_lowercase()).unwrap_or_default();

        // Extract region subtag (2-letter uppercase, e.g. "PT" from "en-PT")
        let region = parts
            .get(1)
            .filter(|r| r.len() == 2 && r.chars().all(|c| c.is_ascii_alphabetic()))
            .map(|r| r.to_uppercase());

        // Try region-based formatting first (if region exists)
        // This handles cases like en-PT, en-DE where the user's language is English
        // but their region determines number/currency formatting.
        let format_from_region = region.as_deref().and_then(Self::format_for_region);

        let (decimal, thousands, currency) = if let Some(fmt) = format_from_region {
            fmt
        } else {
            // Fall back to language-based formatting
            match lang.as_str() {
                // Comma decimal, period thousands
                "de" | "it" | "es" | "nl" | "ro" | "hr" | "sl" | "el" | "tr" => (",", ".", "€"),
                "fr" | "pl" | "cs" | "sv" | "fi" | "hu" | "sk" | "et" | "lv" | "lt" => {
                    (",", " ", "€")
                }
                "pt" => (",", ".", "R$"),
                "da" => (",", ".", "kr"),
                "no" | "nb" | "nn" => (",", " ", "kr"),
                "ru" => (",", " ", "₽"),
                "uk" => (",", " ", "₴"),
                "bg" => (",", " ", "лв"),
                "id" => (",", ".", "Rp"),
                // Period decimal, comma thousands
                "en" => (".", ",", "$"),
                "ja" | "zh" => (".", ",", "¥"),
                "ko" => (".", ",", "₩"),
                "th" => (".", ",", "฿"),
                "ms" => (".", ",", "RM"),
                "hi" | "bn" | "ta" | "te" | "mr" | "gu" | "kn" | "ml" => (".", ",", "₹"),
                "ar" => (".", ",", "د.إ"),
                "he" | "iw" => (".", ",", "₪"),
                // Unknown — use fallback formatting
                _ => {
                    return ValueFormat {
                        language: if lang.is_empty() {
                            fallback.language.clone()
                        } else {
                            lang
                        },
                        ..fallback.clone()
                    };
                }
            }
        };

        // Normalize language codes
        let language = match lang.as_str() {
            "nb" | "nn" => "no".to_string(),
            "iw" => "he".to_string(),
            "" => fallback.language.clone(),
            _ => lang,
        };

        ValueFormat {
            decimal_separator: decimal.to_string(),
            thousands_separator: thousands.to_string(),
            currency_symbol: currency.to_string(),
            language,
            use_excel_rounding: fallback.use_excel_rounding,
            allow_lotus_1_2_3_1900_date_bug: fallback.allow_lotus_1_2_3_1900_date_bug,
        }
    }

    /// Returns (decimal_separator, thousands_separator, currency_symbol) for a region code.
    fn format_for_region(region: &str) -> Option<(&'static str, &'static str, &'static str)> {
        match region {
            // Eurozone — comma decimal, period thousands
            "DE" | "AT" | "ES" | "IT" | "NL" | "PT" | "GR" | "CY" | "LU" | "HR" | "SI" => {
                Some((",", ".", "€"))
            }
            // Eurozone — comma decimal, space thousands
            "FR" | "BE" | "FI" | "SK" | "EE" | "LV" | "LT" => Some((",", " ", "€")),
            // Eurozone — period decimal (exceptions)
            "IE" | "MT" => Some((".", ",", "€")),
            // Non-euro European
            "GB" => Some((".", ",", "£")),
            "CH" => Some((".", "'", "CHF")),
            "SE" => Some((",", " ", "kr")),
            "NO" => Some((",", " ", "kr")),
            "DK" => Some((",", ".", "kr")),
            "PL" => Some((",", " ", "zł")),
            "CZ" => Some((",", " ", "Kč")),
            "HU" => Some((",", " ", "Ft")),
            "RO" => Some((",", ".", "lei")),
            "BG" => Some((",", " ", "лв")),
            "UA" => Some((",", " ", "₴")),
            "RU" => Some((",", " ", "₽")),
            "TR" => Some((",", ".", "₺")),
            "IS" => Some((",", ".", "kr")),
            // Americas
            "US" => Some((".", ",", "$")),
            "CA" => Some((".", ",", "$")),
            "MX" | "CO" | "CL" | "AR" => Some((",", ".", "$")),
            "BR" => Some((",", ".", "R$")),
            // Asia-Pacific
            "JP" => Some((".", ",", "¥")),
            "CN" => Some((".", ",", "¥")),
            "KR" => Some((".", ",", "₩")),
            "IN" => Some((".", ",", "₹")),
            "TH" => Some((".", ",", "฿")),
            "MY" | "SG" => Some((".", ",", "RM")),
            "ID" => Some((",", ".", "Rp")),
            "VN" => Some((",", ".", "₫")),
            "PH" => Some((".", ",", "₱")),
            "TW" => Some((".", ",", "NT$")),
            "HK" => Some((".", ",", "HK$")),
            "AU" | "NZ" => Some((".", ",", "$")),
            // Middle East / Africa
            "IL" => Some((".", ",", "₪")),
            "SA" | "AE" => Some((".", ",", "﷼")),
            "ZA" => Some((".", ",", "R")),
            _ => None,
        }
    }

    /// Overlay `CODCEL_*` environment variables on top of the given base.
    fn overlay_env(mut base: ValueFormat) -> ValueFormat {
        if let Ok(v) = std::env::var("CODCEL_DECIMAL_SEPARATOR") {
            base.decimal_separator = v;
        }
        if let Ok(v) = std::env::var("CODCEL_CURRENCY_SYMBOL") {
            base.currency_symbol = v;
        }
        if let Ok(v) = std::env::var("CODCEL_THOUSANDS_SEPARATOR") {
            base.thousands_separator = v;
        }
        if let Ok(v) = std::env::var("CODCEL_USE_EXCEL_ROUNDING") {
            if let Ok(b) = v.parse::<bool>() {
                base.use_excel_rounding = b;
            }
        }
        if let Ok(v) = std::env::var("CODCEL_LANGUAGE") {
            base.language = v;
        }
        if let Ok(v) = std::env::var("CODCEL_ALLOW_LOTUS_1_2_3_1900_DATE_BUG") {
            if let Ok(b) = v.parse::<bool>() {
                base.allow_lotus_1_2_3_1900_date_bug = b;
            }
        }
        base
    }

    /// The serial-number convention this format implies.
    ///
    /// Always the 1900 system — [`DateSemantics`] cannot express anything else.
    /// A workbook saved with the 1904 epoch has its serials rebased as it is
    /// read, so by the time a value reaches a `ValueFormat` there is only one
    /// convention left in play.
    pub fn date_semantics(&self) -> DateSemantics {
        DateSemantics {
            lotus_1900_bug: self.allow_lotus_1_2_3_1900_date_bug,
        }
    }

    /// Copy the non-locale calculation settings across from `other`.
    ///
    /// Deriving a `ValueFormat` from a language tag (an `Accept-Language` header,
    /// say) resets rounding and date semantics to that language's defaults, which
    /// is never what a caller wants — those are properties of the transpiled
    /// workbook, not of the reader's locale. Every transport that builds a
    /// per-request format calls this to restore them from the project's `FORMAT`.
    pub fn with_calculation_flags_from(mut self, other: &ValueFormat) -> Self {
        self.use_excel_rounding = other.use_excel_rounding;
        self.allow_lotus_1_2_3_1900_date_bug = other.allow_lotus_1_2_3_1900_date_bug;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Pinned field by field so that swapping the hand-written impl for
    /// `#[derive(Default)]` fails loudly: the derive would give
    /// `decimal_separator: ""`, which makes every formatted number unparseable.
    #[test]
    fn default_has_usable_locale_values() {
        let d = ValueFormat::default();
        assert_eq!(d.decimal_separator, ".");
        assert_eq!(d.currency_symbol, "$");
        assert_eq!(d.thousands_separator, ",");
        assert!(!d.use_excel_rounding);
        assert_eq!(d.language, "en");
        assert!(d.allow_lotus_1_2_3_1900_date_bug);
    }

    /// The generated FFI and JNI `*_with_format` entry points deserialise
    /// caller-supplied JSON straight into this struct, so a payload missing any
    /// field must still parse rather than breaking every existing caller on
    /// upgrade. That is what the container-level `#[serde(default)]` buys.
    #[test]
    fn a_partial_json_payload_still_deserialises() {
        let legacy = r#"{
            "decimal_separator": ",",
            "currency_symbol": "€",
            "thousands_separator": ".",
            "use_excel_rounding": true,
            "language": "de",
            "allow_lotus_1_2_3_1900_date_bug": true
        }"#;
        let vf: ValueFormat = serde_json::from_str(legacy).unwrap();
        assert_eq!(vf.decimal_separator, ",");
        assert_eq!(vf.language, "de");
        assert!(
            vf.use_excel_rounding,
            "a field present in the payload must be honoured"
        );

        // A payload missing a field falls back to the default rather than erroring.
        let sparse: ValueFormat = serde_json::from_str(r#"{"language": "fr"}"#).unwrap();
        assert_eq!(sparse.language, "fr");
        assert_eq!(sparse.decimal_separator, ValueFormat::default().decimal_separator);
    }

    #[test]
    fn empty_json_deserialises_to_default() {
        let vf: ValueFormat = serde_json::from_str("{}").unwrap();
        assert_eq!(vf, ValueFormat::default());
    }

    #[test]
    fn date_semantics_maps_the_1900_conventions() {
        let excel = ValueFormat {
            ..Default::default()
        };
        assert_eq!(excel.date_semantics(), DateSemantics::EXCEL_1900);

        let astronomical = ValueFormat {
            allow_lotus_1_2_3_1900_date_bug: false,
            ..Default::default()
        };
        assert_eq!(
            astronomical.date_semantics(),
            DateSemantics::ASTRONOMICAL_1900
        );
    }

    /// Deriving a format from a language tag resets the calculation flags to
    /// that language's defaults; they are properties of the transpiled workbook,
    /// not of the reader's locale, so every transport restores them this way.
    #[test]
    fn with_calculation_flags_from_restores_non_locale_settings() {
        let project = ValueFormat {
            use_excel_rounding: true,
            allow_lotus_1_2_3_1900_date_bug: false,
            ..Default::default()
        };
        let per_request = ValueFormat {
            decimal_separator: ",".to_string(),
            language: "de".to_string(),
            ..Default::default()
        }
        .with_calculation_flags_from(&project);

        assert_eq!(per_request.decimal_separator, ",", "locale is preserved");
        assert_eq!(per_request.language, "de", "locale is preserved");
        assert!(per_request.use_excel_rounding);
        assert!(!per_request.allow_lotus_1_2_3_1900_date_bug);
    }

    #[test]
    fn test_from_system_with_env_returns_valid_format() {
        let fallback = ValueFormat {
            ..Default::default()
        };
        let format = ValueFormat::from_system_with_env(fallback);
        // Should not panic and should have non-empty fields
        assert!(!format.decimal_separator.is_empty());
        assert!(!format.currency_symbol.is_empty());
        assert!(!format.language.is_empty());
    }

    #[test]
    fn test_env_overlay() {
        std::env::set_var("CODCEL_DECIMAL_SEPARATOR", ",");
        std::env::set_var("CODCEL_CURRENCY_SYMBOL", "£");
        std::env::set_var("CODCEL_THOUSANDS_SEPARATOR", ".");
        std::env::set_var("CODCEL_USE_EXCEL_ROUNDING", "true");
        std::env::set_var("CODCEL_LANGUAGE", "de");
        std::env::set_var("CODCEL_ALLOW_LOTUS_1_2_3_1900_DATE_BUG", "false");

        let fallback = ValueFormat {
            ..Default::default()
        };
        let format = ValueFormat::from_env_with_fallback(fallback);

        assert_eq!(format.decimal_separator, ",");
        assert_eq!(format.currency_symbol, "£");
        assert_eq!(format.thousands_separator, ".");
        assert!(format.use_excel_rounding);
        assert_eq!(format.language, "de");
        assert!(!format.allow_lotus_1_2_3_1900_date_bug);

        // Clean up
        std::env::remove_var("CODCEL_DECIMAL_SEPARATOR");
        std::env::remove_var("CODCEL_CURRENCY_SYMBOL");
        std::env::remove_var("CODCEL_THOUSANDS_SEPARATOR");
        std::env::remove_var("CODCEL_USE_EXCEL_ROUNDING");
        std::env::remove_var("CODCEL_LANGUAGE");
        std::env::remove_var("CODCEL_ALLOW_LOTUS_1_2_3_1900_DATE_BUG");
    }

    #[test]
    fn test_from_language_german() {
        let f = ValueFormat::from_language("de");
        assert_eq!(f.decimal_separator, ",");
        assert_eq!(f.thousands_separator, ".");
        assert_eq!(f.currency_symbol, "€");
        assert_eq!(f.language, "de");
    }

    #[test]
    fn test_from_language_with_region() {
        let f = ValueFormat::from_language("fr-FR");
        assert_eq!(f.decimal_separator, ",");
        assert_eq!(f.thousands_separator, " ");
        assert_eq!(f.currency_symbol, "€");
        assert_eq!(f.language, "fr");
    }

    #[test]
    fn test_from_language_english() {
        let f = ValueFormat::from_language("en-US");
        assert_eq!(f.decimal_separator, ".");
        assert_eq!(f.thousands_separator, ",");
        assert_eq!(f.currency_symbol, "$");
        assert_eq!(f.language, "en");
    }

    #[test]
    fn test_from_language_english_portugal_region() {
        // Language is English, but region is Portugal → Euro formatting
        let f = ValueFormat::from_language("en-PT");
        assert_eq!(f.decimal_separator, ",");
        assert_eq!(f.thousands_separator, ".");
        assert_eq!(f.currency_symbol, "€");
        assert_eq!(f.language, "en");
    }

    #[test]
    fn test_from_language_english_germany_region() {
        let f = ValueFormat::from_language("en-DE");
        assert_eq!(f.decimal_separator, ",");
        assert_eq!(f.thousands_separator, ".");
        assert_eq!(f.currency_symbol, "€");
        assert_eq!(f.language, "en");
    }

    #[test]
    fn test_from_language_english_gb_region() {
        let f = ValueFormat::from_language("en-GB");
        assert_eq!(f.decimal_separator, ".");
        assert_eq!(f.thousands_separator, ",");
        assert_eq!(f.currency_symbol, "£");
        assert_eq!(f.language, "en");
    }

    #[test]
    fn test_from_language_english_no_region() {
        // No region → falls back to language-based (English defaults)
        let f = ValueFormat::from_language("en");
        assert_eq!(f.decimal_separator, ".");
        assert_eq!(f.thousands_separator, ",");
        assert_eq!(f.currency_symbol, "$");
        assert_eq!(f.language, "en");
    }

    #[test]
    fn test_from_language_unknown() {
        let f = ValueFormat::from_language("xx");
        // Unknown language falls back to defaults
        assert_eq!(f.decimal_separator, ".");
        assert_eq!(f.language, "xx");
    }

    #[test]
    fn test_from_language_unknown_region_known() {
        // Unknown language but known region → region formatting
        let f = ValueFormat::from_language("xx-JP");
        assert_eq!(f.decimal_separator, ".");
        assert_eq!(f.thousands_separator, ",");
        assert_eq!(f.currency_symbol, "¥");
        assert_eq!(f.language, "xx");
    }
}
