// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

//! Reference locale data: number symbols, calendar names, date patterns,
//! currency formats, Excel date-code aliases and Excel error values.
//!
//! # Relationship to [`ValueFormat`](crate::value_format::ValueFormat)
//!
//! These are two layers with different jobs, and they deliberately do not
//! collapse into one.
//!
//! [`ValueFormat`] owns the three settings a caller can override — decimal
//! separator, thousands separator, currency symbol. They are transpiler flags,
//! `CODCEL_*` environment variables and `Accept-Language` headers, they are
//! part of the FFI/JNI wire format, and generated projects have been pinning
//! them since before this module existed. Their values stay exactly what
//! `ValueFormat` resolves.
//!
//! [`Locale`] supplies everything a caller *cannot* override because Excel
//! does not expose it as a setting: month and weekday names, the ten number
//! symbols beyond the two separators, where a currency symbol sits relative to
//! its amount, and what `#VALUE!` is called in German.
//!
//! Where the two overlap — [`NumberSymbols::decimal`] and
//! [`NumberSymbols::group`] against [`ValueFormat`]'s separators —
//! **`ValueFormat` wins**. CLDR's grouping separator for French is U+202F
//! NARROW NO-BREAK SPACE and for Portuguese the currency is the euro, neither
//! of which matches what Codcel has emitted for these locales to date. Letting
//! CLDR override would silently change the output of every generated project
//! on upgrade. The CLDR values remain readable here as reference data.
//!
//! [`ValueFormat`]: crate::value_format::ValueFormat

mod data;
#[cfg(test)]
mod golden_tests;

pub use data::CLDR_VERSION;

/// The thirteen CLDR number symbols.
///
/// Excel substitutes these into a number format at display time; a stored
/// format code such as `#,##0.00` is always written with `.` and `,` in the
/// file regardless of the authoring locale.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumberSymbols {
    /// Decimal separator. See the module note — [`ValueFormat`](crate::value_format::ValueFormat) overrides this.
    pub decimal: &'static str,
    /// Grouping separator. See the module note — [`ValueFormat`](crate::value_format::ValueFormat) overrides this.
    pub group: &'static str,
    /// Argument separator between list items, `;` in most locales.
    pub list: &'static str,
    pub percent_sign: &'static str,
    pub plus_sign: &'static str,
    /// Minus sign. Not always ASCII `-`: several locales prefix a directional mark.
    pub minus_sign: &'static str,
    pub approximately_sign: &'static str,
    /// The `E` of scientific notation.
    pub exponential: &'static str,
    pub superscripting_exponent: &'static str,
    pub per_mille: &'static str,
    pub infinity: &'static str,
    pub nan: &'static str,
    /// Separator between hours, minutes and seconds.
    pub time_separator: &'static str,
}

/// A pattern at each of CLDR's four widths, as Excel number-format codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateFormats {
    pub full: &'static str,
    pub long: &'static str,
    pub medium: &'static str,
    pub short: &'static str,
}

/// Gregorian calendar names and patterns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dates {
    /// Full month names, January first. Excel `mmmm`.
    pub months: [&'static str; 12],
    /// Abbreviated month names. Excel `mmm`.
    pub months_short: [&'static str; 12],
    /// Single-letter month names. Excel `mmmmm`.
    pub months_letter: [&'static str; 12],
    /// Full weekday names, **Sunday first**. Excel `dddd`.
    pub day_names: [&'static str; 7],
    /// Abbreviated weekday names, Sunday first. Excel `ddd`.
    pub day_names_short: [&'static str; 7],
    /// AM and PM markers.
    pub am_pm: [&'static str; 2],
    pub date_formats: DateFormats,
    pub time_formats: DateFormats,
    /// Glue patterns combining a date and a time, with `{1}` the date and
    /// `{0}` the time.
    pub date_time_formats: DateFormats,
}

/// Currency patterns. `¤` stands in for the currency symbol, as in CLDR.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrencyFormats {
    /// The region's currency symbol. See the module note —
    /// [`ValueFormat`](crate::value_format::ValueFormat) overrides this.
    pub symbol: &'static str,
    /// ISO 4217 code, e.g. `EUR`.
    pub iso_code: &'static str,
    /// Standard pattern, e.g. `#,##0.00 ¤`. Carries the symbol's position and
    /// spacing, which is what `DOLLAR` needs and a bare symbol cannot express.
    pub standard: &'static str,
    /// Accounting pattern, which brackets negatives in most locales.
    pub accounting: &'static str,
    /// Plain decimal pattern, e.g. `#,##0.###`.
    pub decimal_standard: &'static str,
}

/// Excel's localized error values, in [`ExcelError`](crate::excel_error::ExcelError)
/// declaration order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ErrorNames {
    pub null: &'static str,
    pub div0: &'static str,
    pub value: &'static str,
    pub r#ref: &'static str,
    pub name: &'static str,
    pub num: &'static str,
    pub na: &'static str,
    /// Codcel's own annotation appended to an error message — *not* something
    /// Excel emits. Falls back to the English text for languages the table has
    /// no phrasing for.
    pub suffix: &'static str,
}

/// One resolved locale.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Locale {
    /// The CLDR id this entry was generated from, e.g. `de-CH`.
    pub tag: &'static str,
    pub numbers: NumberSymbols,
    pub dates: Dates,
    pub currency: CurrencyFormats,
    pub errors: ErrorNames,
    /// Excel date-code letters in this language paired with their canonical
    /// English equivalent, longest first: German `[("jjjj", "yyyy"), …]`.
    ///
    /// Empty for languages whose letters cannot be resolved by substitution —
    /// Turkish `s` is both *saat* (hour) and *saniye* (second).
    pub date_token_aliases: &'static [(&'static str, &'static str)],
    /// Canonical English function names paired with this language's names,
    /// sorted by the English name. See [`function_name`] and
    /// [`function_name_from`].
    ///
    /// **Partial, and nothing in the transpiler reads it.** Both are
    /// deliberate; see the note on [`function_name`].
    pub function_names: &'static [(&'static str, &'static str)],
}

/// This language's name for an English function name — `WENN` for `IF` in
/// German — or `None` if the table has no entry.
///
/// # Why nothing calls this
///
/// A workbook stores formula text in canonical English in the sheet XML
/// regardless of the language it was authored in; Excel localizes only what it
/// draws on screen. Codcel reads files rather than keystrokes, so no localized
/// function name ever reaches its parser, and there is nothing for a localized
/// lexer to do. **Do not wire this into the transpiler's parser.**
///
/// It exists to render a formula back to a person in their own language, and as
/// the prerequisite if a runtime formula parser is ever added.
///
/// # Coverage
///
/// Partial by necessity: CLDR does not model function names and Microsoft's
/// translation workbook is not machine-fetchable, so the table is
/// hand-maintained in `locale-gen/excel/function-names.json` and currently
/// covers a common core across eight languages. A name with no entry resolves
/// to its English form, which is also what the file stores — a missing entry is
/// a display gap, never a wrong answer.
pub fn function_name(locale: &'static Locale, english: &str) -> Option<&'static str> {
    let upper = english.to_uppercase();
    locale
        .function_names
        .binary_search_by(|(k, _)| (*k).cmp(upper.as_str()))
        .ok()
        .map(|i| locale.function_names[i].1)
}

/// The canonical English name for a localized one — `IF` for `WENN`.
///
/// The reverse of [`function_name`], and subject to the same coverage caveat.
/// Linear rather than binary search: the table is sorted by the English name,
/// and a second index sorted the other way would double its size for a lookup
/// nothing on the hot path performs.
pub fn function_name_from(locale: &'static Locale, localized: &str) -> Option<&'static str> {
    let upper = localized.to_uppercase();
    locale
        .function_names
        .iter()
        .find(|(_, v)| v.eq_ignore_ascii_case(&upper) || *v == upper)
        .map(|(k, _)| *k)
}

/// Splits a BCP-47-ish tag into its language and region subtags.
///
/// Accepts both `-` and `_` as separators, since a POSIX `LANG` value arrives
/// as `de_DE.UTF-8`. Returns the language lowercased and the region uppercased.
/// A subtag that is not two ASCII letters is not a region.
pub fn parse_tag(tag: &str) -> (String, Option<String>) {
    let cleaned = tag.split('.').next().unwrap_or(tag);
    let mut parts = cleaned.split(['-', '_']);
    let language = parts.next().unwrap_or_default().to_lowercase();
    let region = parts
        .find(|p| p.len() == 2 && p.chars().all(|c| c.is_ascii_alphabetic()))
        .map(|p| p.to_uppercase());
    (language, region)
}

fn probe(table: &'static [(&'static str, &'static Locale)], key: &str) -> Option<&'static Locale> {
    table
        .binary_search_by(|(k, _)| (*k).cmp(key))
        .ok()
        .map(|i| table[i].1)
}

fn alias(table: &'static [(&'static str, &'static str)], key: &str) -> Option<&'static str> {
    table
        .binary_search_by(|(k, _)| (*k).cmp(key))
        .ok()
        .map(|i| table[i].1)
}

/// The locale for a language tag.
///
/// Total: an unrecognised or empty tag resolves to `en` rather than failing,
/// which is what the crate's `panic = "deny"` lint requires of a lookup this
/// deep in the formatting path.
///
/// Resolution order:
///
/// 1. the exact `lang-REGION` pair, so `fr-CH` gets French names with Swiss
///    number conventions;
/// 2. the bare language, then its synonyms;
/// 3. the region alone;
/// 4. `en`.
///
/// The language is tried before the region because the two subtags govern
/// different things and this table bundles both. In Excel a month name comes
/// from the interface language while the grouping separator comes from the
/// regional settings, so an English speaker in Germany sees `January` written
/// `1.234,56`. Falling back to the language keeps the names right; the
/// separators are not this table's to decide, because
/// [`ValueFormat`](crate::value_format::ValueFormat) resolves those from the
/// region itself and overrides whatever is here.
///
/// The region probe is what catches the remaining case — a language Codcel has
/// no data for at all, where the region is the only usable signal.
pub fn lookup(tag: &str) -> &'static Locale {
    let (language, region) = parse_tag(tag);
    lookup_parts(&language, region.as_deref())
}

/// [`lookup`] over an already-split tag.
///
/// [`ValueFormat`](crate::value_format::ValueFormat) keeps the language and
/// region in separate fields — its `language` has always held the bare subtag,
/// and generated projects pin that — so it resolves through here rather than
/// re-joining them into a tag just to have `parse_tag` split it again.
pub fn lookup_parts(language: &str, region: Option<&str>) -> &'static Locale {
    let region = region.filter(|r| !r.is_empty());

    if let Some(region) = region {
        let exact = format!("{language}-{region}");
        if let Some(found) = probe(data::LOCALES, &exact) {
            return found;
        }
        if let Some(found) =
            alias(data::LOCALE_ALIASES, &exact).and_then(|id| probe(data::LOCALES, id))
        {
            return found;
        }
    }

    probe(data::LOCALES, language)
        .or_else(|| alias(data::LANGUAGE_ALIASES, language).and_then(|id| probe(data::LOCALES, id)))
        .or_else(|| {
            region
                .and_then(|r| alias(data::REGION_ALIASES, r))
                .and_then(|id| probe(data::LOCALES, id))
        })
        .unwrap_or(&data::EN)
}

/// The English locale. The fallback for every tag that does not resolve.
pub fn english() -> &'static Locale {
    &data::EN
}

/// The format code for an Excel built-in `numFmtId`, or `None` if the id has no
/// built-in meaning.
///
/// Ids 14 and 22 are locale-dependent — Excel renders `numFmtId="14"` as
/// `m/d/yy` in the United States and `dd/mm/yyyy` in the United Kingdom — so
/// they resolve through the locale's short date pattern. The rest are fixed by
/// ECMA-376 §18.8.30 and are returned verbatim.
pub fn builtin_format(id: u16, locale: &'static Locale) -> Option<String> {
    let short_date = locale.dates.date_formats.short;
    Some(match id {
        14 => short_date.to_string(),
        15 => "d-mmm-yy".to_string(),
        16 => "d-mmm".to_string(),
        17 => "mmm-yy".to_string(),
        18 => "h:mm AM/PM".to_string(),
        19 => "h:mm:ss AM/PM".to_string(),
        20 => "h:mm".to_string(),
        21 => "h:mm:ss".to_string(),
        22 => format!("{short_date} h:mm"),
        45 => "mm:ss".to_string(),
        46 => "[h]:mm:ss".to_string(),
        47 => "mmss.0".to_string(),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `lookup` binary-searches all three tables, so a generator change that
    /// disturbed their order would silently start missing locales.
    #[test]
    fn generated_tables_are_sorted() {
        assert!(
            data::LOCALES.windows(2).all(|w| w[0].0 < w[1].0),
            "LOCALES is not sorted by tag"
        );
        assert!(
            data::LOCALE_ALIASES.windows(2).all(|w| w[0].0 < w[1].0),
            "LOCALE_ALIASES is not sorted by tag"
        );
        assert!(
            data::REGION_ALIASES.windows(2).all(|w| w[0].0 < w[1].0),
            "REGION_ALIASES is not sorted by region"
        );
        assert!(
            data::LANGUAGE_ALIASES.windows(2).all(|w| w[0].0 < w[1].0),
            "LANGUAGE_ALIASES is not sorted by language"
        );
    }

    /// The function tables are binary-searched by the English name.
    #[cfg(feature = "locale-data")]
    #[test]
    fn function_tables_are_sorted() {
        for (tag, locale) in data::LOCALES {
            assert!(
                locale.function_names.windows(2).all(|w| w[0].0 < w[1].0),
                "{tag} function_names is not sorted"
            );
        }
    }

    #[cfg(feature = "locale-data")]
    #[test]
    fn function_names_round_trip() {
        let de = lookup("de");
        assert_eq!(function_name(de, "IF"), Some("WENN"));
        assert_eq!(function_name(de, "if"), Some("WENN"), "case-insensitive");
        assert_eq!(function_name(de, "VLOOKUP"), Some("SVERWEIS"));
        assert_eq!(function_name_from(de, "WENN"), Some("IF"));
        assert_eq!(function_name_from(de, "SVERWEIS"), Some("VLOOKUP"));

        assert_eq!(function_name(lookup("fr"), "IF"), Some("SI"));
        assert_eq!(function_name(lookup("ru"), "VLOOKUP"), Some("ВПР"));
    }

    /// A name the table does not carry resolves to nothing rather than to
    /// something wrong, and the caller falls back to the English form — which
    /// is what the file stores anyway.
    #[test]
    fn an_uncovered_function_or_language_returns_none() {
        assert_eq!(
            function_name(english(), "IF"),
            None,
            "English needs no table"
        );
        #[cfg(feature = "locale-data")]
        assert_eq!(function_name(lookup("de"), "BESSELJ"), None);
        #[cfg(feature = "locale-data")]
        assert_eq!(function_name(lookup("th"), "IF"), None);
    }

    /// Every alias must land on a locale that is actually shipped.
    #[cfg(feature = "locale-data")]
    #[test]
    fn every_alias_resolves() {
        for (key, target) in data::LOCALE_ALIASES {
            assert!(
                probe(data::LOCALES, target).is_some(),
                "locale alias {key} -> {target} has no entry"
            );
        }
        for (key, target) in data::REGION_ALIASES {
            assert!(
                probe(data::LOCALES, target).is_some(),
                "region alias {key} -> {target} has no entry"
            );
        }
        for (key, target) in data::LANGUAGE_ALIASES {
            assert!(
                probe(data::LOCALES, target).is_some(),
                "language alias {key} -> {target} has no entry"
            );
        }
    }

    /// `ValueFormat` normalises `nb` and `nn` onto the macro-language `no`,
    /// which CLDR has no data file for. Without the synonym table every
    /// Norwegian format would silently fall back to English month names.
    #[cfg(feature = "locale-data")]
    #[test]
    fn language_synonyms_resolve() {
        assert_eq!(lookup("no").dates.months[0], lookup("nb").dates.months[0]);
        assert_ne!(lookup("no").dates.months[0], "January");
        assert_eq!(lookup("iw").tag, "he");
    }

    #[test]
    fn parse_tag_handles_the_shapes_a_locale_arrives_in() {
        assert_eq!(
            parse_tag("de-DE"),
            ("de".to_string(), Some("DE".to_string()))
        );
        assert_eq!(
            parse_tag("de_DE.UTF-8"),
            ("de".to_string(), Some("DE".to_string()))
        );
        assert_eq!(parse_tag("DE"), ("de".to_string(), None));
        assert_eq!(
            parse_tag("zh-Hant-TW"),
            ("zh".to_string(), Some("TW".to_string()))
        );
        assert_eq!(parse_tag(""), (String::new(), None));
    }

    #[test]
    fn unknown_tags_fall_back_to_english() {
        for tag in ["", "xx", "xx-YY", "not a tag", "klingon"] {
            assert_eq!(lookup(tag).dates.months[0], "January", "tag {tag:?}");
        }
    }

    #[cfg(feature = "locale-data")]
    #[test]
    fn resolution_prefers_the_exact_pair_then_language_then_region() {
        // Exact pair: French names, Swiss number conventions.
        let fr_ch = lookup("fr-CH");
        assert_eq!(fr_ch.tag, "fr-CH");
        assert_eq!(fr_ch.dates.months[0], "janvier");
        assert_eq!(fr_ch.numbers.group, "'");

        // Language beats region when the pair is not shipped: an English
        // speaker in Germany still reads English month names.
        assert_eq!(lookup("en-DE").dates.months[0], "January");

        // Region is the fallback only when the language is unknown, where it is
        // the one usable signal left.
        assert_eq!(
            lookup("xx-JP").dates.months[0],
            lookup("ja").dates.months[0]
        );

        // Bare language.
        assert_eq!(lookup("de").dates.months[0], "Januar");

        // A language whose default region CLDR omits still resolves.
        assert_eq!(lookup("de-DE").dates.months[0], "Januar");
        assert_eq!(lookup("en-US").dates.months[0], "January");
    }

    /// Chinese splits on script, not region, so the region subtag has to survive
    /// into the script-qualified locale id.
    #[cfg(feature = "locale-data")]
    #[test]
    fn chinese_resolves_by_script() {
        assert_eq!(lookup("zh-TW").tag, "zh-Hant");
        assert_eq!(lookup("zh-HK").tag, "zh-Hant-HK");
        assert_eq!(lookup("zh-CN").tag, "zh");
    }

    #[cfg(feature = "locale-data")]
    #[test]
    fn currency_patterns_carry_symbol_position() {
        // A bare currency symbol cannot express this difference, which is why
        // `DOLLAR` needs the pattern rather than the symbol.
        assert!(lookup("en-US").currency.standard.starts_with('\u{a4}'));
        assert!(lookup("de").currency.standard.ends_with('\u{a4}'));
    }

    /// Excel renders `numFmtId="14"` per the user's locale, so the same
    /// workbook shows a different date format either side of the Atlantic.
    #[test]
    fn builtin_short_date_is_locale_dependent() {
        assert_eq!(builtin_format(14, english()).as_deref(), Some("m/d/yy"));
        assert_eq!(builtin_format(15, english()).as_deref(), Some("d-mmm-yy"));
        assert_eq!(builtin_format(99, english()), None);
    }

    #[cfg(feature = "locale-data")]
    #[test]
    fn builtin_short_date_differs_across_locales() {
        assert_eq!(
            builtin_format(14, lookup("en-GB")).as_deref(),
            Some("dd/mm/yyyy")
        );
        assert_eq!(
            builtin_format(14, lookup("de")).as_deref(),
            Some("dd.mm.yy")
        );
    }
}
