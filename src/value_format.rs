// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::date_system::DateSemantics;
use crate::locale::{self, Locale};
use serde::{Deserialize, Serialize};

/// Locale and calculation settings carried alongside every value.
///
/// This is the *overridable* half of Codcel's locale model: three format
/// settings a caller can set from a transpiler flag, a `CODCEL_*` environment
/// variable or an `Accept-Language` header. Everything Excel does not expose as
/// a setting — month names, the other ten number symbols, where a currency
/// symbol sits relative to its amount, localized error values — comes from
/// [`Locale`], reachable via [`ValueFormat::locale`].
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
    /// The region subtag of the tag this format was resolved from, uppercased —
    /// `"GB"` for `en-GB` — or empty when none was supplied.
    ///
    /// [`ValueFormat::language`] deliberately holds only the language subtag and
    /// always has, so the region needs somewhere of its own to live. Without it
    /// [`ValueFormat::locale`] could not tell `en-GB` from `en-US`, and Excel
    /// renders those differently: `numFmtId="14"` is `dd/mm/yyyy` in one and
    /// `m/d/yy` in the other.
    pub region: String,
    /// IANA timezone name for `NOW` and `TODAY`, e.g. `Europe/Berlin`. Empty
    /// means the host's local zone, which is what Excel reads.
    ///
    /// Not derived from the language tag: a locale is not a timezone. Spain and
    /// the Canaries share `es-ES` and are an hour apart, and the United States
    /// spans six zones under `en-US`. It has to be stated or left to the host.
    pub timezone: String,
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
            region: String::new(),
            timezone: String::new(),
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
        // Shared with `locale::lookup` so a tag resolves to the same language
        // and region here as it does when picking month names.
        let (lang, region) = locale::parse_tag(lang_tag);

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
                        region: region.unwrap_or_default(),
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
            region: region.unwrap_or_default(),
            // A language tag says nothing about where the machine is, so the
            // timezone is carried across from the fallback rather than guessed.
            timezone: fallback.timezone.clone(),
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
            // A full tag here sets both fields, so `CODCEL_LANGUAGE=en-GB` picks
            // up British date patterns rather than being read as bare `en`.
            let (language, region) = locale::parse_tag(&v);
            base.language = language;
            if let Some(region) = region {
                base.region = region;
            }
        }
        if let Ok(v) = std::env::var("CODCEL_REGION") {
            base.region = v.to_uppercase();
        }
        if let Ok(v) = std::env::var("CODCEL_TIMEZONE") {
            base.timezone = v;
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

    /// The CLDR locale for [`ValueFormat::language`].
    ///
    /// Total: an unrecognised tag resolves to English rather than failing.
    ///
    /// Note that the three fields on this struct take precedence over their
    /// counterparts on the returned [`Locale`]. `self.decimal_separator` is
    /// authoritative over `self.locale().numbers.decimal`, and likewise for the
    /// thousands separator and currency symbol — those three are user-settable
    /// and the locale table is reference data. See the module note on
    /// [`crate::locale`] for why the two are not collapsed into one.
    pub fn locale(&self) -> &'static Locale {
        locale::lookup_parts(&self.language, Some(self.region.as_str()))
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
        // The timezone is a property of the deployment, not of the reader's
        // language — `Accept-Language: de` says nothing about where the machine
        // is — so it is restored here alongside the calculation flags.
        self.timezone = other.timezone.clone();
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
        assert_eq!(
            sparse.decimal_separator,
            ValueFormat::default().decimal_separator
        );
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
        // `CODCEL_LANGUAGE` accepts a full tag, so `en-GB` must reach the
        // locale rather than being truncated to bare `en`. Asserted here rather
        // than in a test of its own because these tests mutate shared process
        // environment and run in parallel.
        std::env::set_var("CODCEL_LANGUAGE", "en-GB");
        let tagged = ValueFormat::from_env_with_fallback(ValueFormat::default());
        assert_eq!(tagged.language, "en");
        assert_eq!(tagged.region, "GB");
        #[cfg(feature = "locale-data")]
        assert_eq!(tagged.locale().tag, "en-GB");

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

    /// The region has to survive onto the struct, or `locale()` cannot tell
    /// `en-GB` from `en-US` — and Excel renders `numFmtId="14"` differently in
    /// each.
    #[test]
    fn region_subtag_is_retained_alongside_the_language() {
        let gb = ValueFormat::from_language("en-GB");
        assert_eq!(gb.language, "en", "language stays the bare subtag");
        assert_eq!(gb.region, "GB");

        let us = ValueFormat::from_language("en-US");
        assert_eq!(us.region, "US");

        let bare = ValueFormat::from_language("de");
        assert!(bare.region.is_empty());

        // Which locale those resolve to needs the table to be compiled in.
        #[cfg(feature = "locale-data")]
        {
            assert_eq!(gb.locale().tag, "en-GB");
            assert_eq!(us.locale().tag, "en");
            assert_eq!(bare.locale().tag, "de");
        }
    }

    /// The region governs number conventions while the language governs names,
    /// which is the whole reason the two are separate fields.
    #[test]
    fn an_english_speaker_in_germany_gets_german_numbers_and_english_names() {
        let f = ValueFormat::from_language("en-DE");
        assert_eq!(f.decimal_separator, ",");
        assert_eq!(f.locale().dates.months[0], "January");
    }

    #[test]
    fn locale_falls_back_to_english_for_an_unknown_language() {
        let f = ValueFormat::from_language("xx");
        assert_eq!(f.locale().dates.months[0], "January");
    }

    /// Codcel's separator and currency choices are **not** CLDR's, and that is
    /// deliberate: these three fields are user-settable, have been emitted into
    /// generated projects since before the locale table existed, and adopting
    /// CLDR's values would change the output of every one of them on upgrade.
    ///
    /// This pins the known divergences so they read as decisions rather than as
    /// drift waiting to be tidied up. See the module note on [`crate::locale`].
    // Asserts a non-English locale, which only exists with `locale-data` on.
    #[cfg(feature = "locale-data")]
    #[test]
    fn codcel_separators_deliberately_diverge_from_cldr() {
        // CLDR groups French thousands with U+202F NARROW NO-BREAK SPACE and
        // Swedish, Polish, Russian and Czech with U+00A0. Codcel emits a plain
        // ASCII space for all of them.
        for tag in ["fr-FR", "sv-SE", "pl-PL", "ru-RU", "cs-CZ"] {
            let f = ValueFormat::from_language(tag);
            assert_eq!(f.thousands_separator, " ", "{tag} separator");
            assert_ne!(
                f.locale().numbers.group,
                " ",
                "{tag}: CLDR is expected to differ here — if it no longer does, \
                 this test has stopped guarding anything"
            );
        }

        // Codcel keys the currency off the region; CLDR keys it off the locale
        // it ships, and `pt` is Brazil.
        let pt = ValueFormat::from_language("pt-PT");
        assert_eq!(pt.currency_symbol, "€");
        assert_eq!(pt.locale().currency.symbol, "€");
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
