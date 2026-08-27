// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Emits `../src/locale/data.rs` from the condensed CLDR snapshot produced by
// `fetch.mjs` plus the hand-maintained Excel tables in `excel/`.
//
// Run:  node generate.mjs
//
// The output is checked in. This never runs at build time: the crate publishes
// with `include = ["src/**/*.rs", ...]` and has no build script, so generated
// data has to be committed Rust source.

import fs from 'node:fs';

const cldr = JSON.parse(fs.readFileSync('cldr/cldr-condensed.json', 'utf8'));
const excel = JSON.parse(fs.readFileSync('excel/excel-locale.json', 'utf8'));
const functionNames = JSON.parse(fs.readFileSync('excel/function-names.json', 'utf8'));

const ERROR_ORDER = ['Null', 'Div0', 'Value', 'Ref', 'Name', 'Num', 'Na'];

// Deprecated or macro-language codes that CLDR does not ship under their own
// id. `ValueFormat` normalises `nb`/`nn` to the macro-language `no`, and older
// locale strings still use the pre-1989 ISO codes `iw` and `in`.
const LANGUAGE_SYNONYMS = { no: 'nb', iw: 'he', in: 'id', mo: 'ro' };

// --- Rust string literal escaping -----------------------------------------
// Format characters (RTL marks, non-breaking spaces) are escaped rather than
// emitted raw so the generated file stays readable and diffable.
function rustStr(s) {
  if (s === undefined || s === null) return '""';
  let out = '"';
  for (const ch of String(s)) {
    const cp = ch.codePointAt(0);
    if (ch === '"') out += '\\"';
    else if (ch === '\\') out += '\\\\';
    else if (cp < 0x20 || cp === 0x7f) out += `\\u{${cp.toString(16)}}`;
    else if (
      cp === 0xa0 || cp === 0xad || (cp >= 0x2000 && cp <= 0x200f) ||
      (cp >= 0x2028 && cp <= 0x202f) || (cp >= 0x2060 && cp <= 0x206f) ||
      cp === 0x061c || cp === 0xfeff
    ) out += `\\u{${cp.toString(16)}}`;
    else out += ch;
  }
  return out + '"';
}

const arr = (xs) => `&[${xs.map(rustStr).join(', ')}]`;
const fixed = (xs, n) => {
  if (xs.length !== n) throw new Error(`expected ${n} entries, got ${xs.length}`);
  return `[${xs.map(rustStr).join(', ')}]`;
};

// --- LDML skeleton -> Excel number-format date code ------------------------
// CLDR expresses patterns in LDML letters (`dd.MM.y`, `h:mm a`); Excel uses its
// own set (`dd.mm.yyyy`, `h:mm AM/PM`). Convert here so the shipped table is
// directly usable by the format tokenizer.
//
// Note LDML lowercase `m` is *minute* and uppercase `M` is *month*, while Excel
// spells both `m` and resolves them positionally — so both map to `m` and the
// tokenizer's existing month-vs-minute pass sorts them out.
function ldmlToExcel(pattern) {
  if (!pattern) return '';
  let out = '';
  let i = 0;
  while (i < pattern.length) {
    const ch = pattern[i];

    if (ch === "'") {
      // LDML quotes literals in single quotes; '' is a literal apostrophe.
      i++;
      let lit = '';
      while (i < pattern.length) {
        if (pattern[i] === "'") {
          if (pattern[i + 1] === "'") { lit += "'"; i += 2; continue; }
          i++;
          break;
        }
        lit += pattern[i++];
      }
      out += lit === '' ? "\\'" : `"${lit.replace(/"/g, '')}"`;
      continue;
    }

    let run = 1;
    while (pattern[i + run] === ch) run++;
    const n = run;
    i += n;

    switch (ch) {
      case 'y': case 'u': case 'r':
        out += n === 2 ? 'yy' : 'yyyy'; break;
      case 'M': case 'L':
        out += 'm'.repeat(Math.min(n, 5)); break;
      case 'd':
        out += 'd'.repeat(Math.min(n, 2)); break;
      case 'E': case 'c': case 'e':
        out += n >= 4 ? 'dddd' : 'ddd'; break;
      case 'h': case 'H': case 'K': case 'k':
        out += 'h'.repeat(Math.min(n, 2)); break;
      case 'm':
        out += 'm'.repeat(Math.min(n, 2)); break;
      case 's':
        out += 's'.repeat(Math.min(n, 2)); break;
      case 'a': case 'b': case 'B':
        out += 'AM/PM'; break;
      case 'G': case 'z': case 'Z': case 'v': case 'V': case 'x': case 'X': case 'O':
        break; // era and timezone fields have no Excel format-code equivalent
      case 'S':
        out += '.' + '0'.repeat(Math.min(n, 3)); break;
      default:
        // Punctuation and spacing pass through. Excel treats these as literals.
        out += ch.repeat(n);
    }
  }
  return out.replace(/\s+$/, '');
}

// --- emit ------------------------------------------------------------------
const ids = Object.keys(cldr.locales).sort();

function localeConst(id) {
  return id.toUpperCase().replace(/-/g, '_');
}

function emitLocale(id, loc) {
  const lang = id.split('-')[0];
  const s = loc.symbols;
  const d = loc.dates;
  const aliases = excel.dateTokenAliases[lang] || [];
  const errors = excel.errors[lang] || excel.errors.en;
  const errorSuffix = excel.errorSuffix[lang] || excel.errorSuffix.en;
  // Sorted by English name so the Rust side can binary-search.
  const functions = Object.entries(functionNames[lang] || {})
    .filter(([, localized]) => localized)
    .sort(([a], [b]) => (a < b ? -1 : 1));

  const days = ['sun', 'mon', 'tue', 'wed', 'thu', 'fri', 'sat'];

  const df = (o, conv = ldmlToExcel) => `DateFormats {
            full: ${rustStr(conv(o.full))},
            long: ${rustStr(conv(o.long))},
            medium: ${rustStr(conv(o.medium))},
            short: ${rustStr(conv(o.short))},
        }`;

  return `pub(super) static ${localeConst(id)}: Locale = Locale {
    tag: ${rustStr(id)},
    numbers: NumberSymbols {
        decimal: ${rustStr(s.decimal)},
        group: ${rustStr(s.group)},
        list: ${rustStr(s.list)},
        percent_sign: ${rustStr(s.percentSign)},
        plus_sign: ${rustStr(s.plusSign)},
        minus_sign: ${rustStr(s.minusSign)},
        approximately_sign: ${rustStr(s.approximatelySign ?? '~')},
        exponential: ${rustStr(s.exponential)},
        superscripting_exponent: ${rustStr(s.superscriptingExponent ?? '×')},
        per_mille: ${rustStr(s.perMille)},
        infinity: ${rustStr(s.infinity)},
        nan: ${rustStr(s.nan)},
        time_separator: ${rustStr(s.timeSeparator ?? ':')},
    },
    dates: Dates {
        months: ${fixed(d.monthsWide, 12)},
        months_short: ${fixed(d.monthsAbbrev, 12)},
        months_letter: ${fixed(d.monthsNarrow, 12)},
        day_names: ${fixed(days.map((k) => d.daysWide[k]), 7)},
        day_names_short: ${fixed(days.map((k) => d.daysAbbrev[k]), 7)},
        am_pm: ${fixed([d.am, d.pm], 2)},
        date_formats: ${df(d.dateFormats)},
        time_formats: ${df(d.timeFormats)},
        date_time_formats: ${df(d.dateTimeFormats, (x) => x)},
    },
    currency: CurrencyFormats {
        symbol: ${rustStr(loc.currency.symbol)},
        iso_code: ${rustStr(loc.currency.code)},
        standard: ${rustStr(loc.currency.standard)},
        accounting: ${rustStr(loc.currency.accounting)},
        decimal_standard: ${rustStr(loc.decimalStandard)},
    },
    errors: ErrorNames {
${ERROR_ORDER.map((n, k) => {
      const f = n.toLowerCase();
      return `        ${f === 'ref' ? 'r#ref' : f}: ${rustStr(errors[k])},`;
    }).join('\n')}
        suffix: ${rustStr(errorSuffix)},
    },
    date_token_aliases: &[${aliases.map(([a, b]) => `(${rustStr(a)}, ${rustStr(b)})`).join(', ')}],
    function_names: &[${functions.map(([a, b]) => `(${rustStr(a)}, ${rustStr(b)})`).join(', ')}],
};`;
}

const header = `// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

//! @generated by \`locale-gen/generate.mjs\` — do not edit by hand.
//!
//! Number symbols, calendar names, date patterns and currency formats are
//! derived from Unicode CLDR ${cldr.cldrVersion}. Date-code aliases and error
//! values are Excel's, from \`locale-gen/excel/excel-locale.json\`; CLDR does
//! not carry either.
//!
//! Regenerate with:
//! \`\`\`text
//! cd locale-gen && node fetch.mjs && node generate.mjs
//! \`\`\`

use super::{CurrencyFormats, DateFormats, Dates, ErrorNames, Locale, NumberSymbols};

/// The CLDR release this table was generated from.
pub const CLDR_VERSION: &str = ${rustStr(cldr.cldrVersion)};
`;

const parts = [header];

// `en` is always compiled in; it is the fallback for every unresolved tag.
parts.push('\n' + emitLocale('en', cldr.locales.en) + '\n');

parts.push(`
// Every other locale is behind \`locale-data\`. With the feature off the table
// collapses to \`en\` alone, which keeps the wasm bundle of an English-only
// project from carrying ${ids.length - 1} unused locales.
#[cfg(feature = "locale-data")]
mod full {
    use super::*;
`);
for (const id of ids) {
  if (id === 'en') continue;
  parts.push('\n    ' + emitLocale(id, cldr.locales[id]).replace(/\n/g, '\n    ') + '\n');
}
parts.push('}\n');

parts.push(`
#[cfg(feature = "locale-data")]
use full::*;

/// Every shipped locale, sorted by tag so lookup can binary-search.
#[cfg(feature = "locale-data")]
pub(super) static LOCALES: &[(&str, &Locale)] = &[
${ids.map((id) => `    (${rustStr(id)}, &${localeConst(id)}),`).join('\n')}
];

#[cfg(not(feature = "locale-data"))]
pub(super) static LOCALES: &[(&str, &Locale)] = &[("en", &EN)];
`);

const languageAliases = Object.entries(LANGUAGE_SYNONYMS)
  .filter(([, v]) => ids.includes(v))
  .sort(([a], [b]) => (a < b ? -1 : 1));
const localeAliases = Object.entries(cldr.localeAliases).sort(([a], [b]) => (a < b ? -1 : 1));
const regionAliases = Object.entries(cldr.regionAliases).sort(([a], [b]) => (a < b ? -1 : 1));

parts.push(`
/// Language subtags CLDR ships under a different id, sorted by key.
///
/// \`ValueFormat\` normalises the Norwegian written standards \`nb\` and \`nn\` onto
/// the macro-language \`no\`, which CLDR has no data file for, and locale strings
/// from older systems still carry the pre-1989 ISO codes \`iw\` and \`in\`.
pub(super) static LANGUAGE_ALIASES: &[(&str, &str)] = &[
${languageAliases.map(([k, v]) => `    (${rustStr(k)}, ${rustStr(v)}),`).join('\n')}
];

/// \`lang-REGION\` tags that CLDR ships under a different id, sorted by key.
///
/// CLDR omits a locale whose id is the default for its language — there is no
/// \`de-DE\` because \`de\` already is \`de-DE\` — and routes script-differentiated
/// regions through a script subtag, which is what separates \`zh-TW\`
/// (Traditional) from \`zh-CN\` (Simplified). Tags that simply collapse onto
/// their bare language are absent here and fall through to the language probe.
pub(super) static LOCALE_ALIASES: &[(&str, &str)] = &[
${localeAliases.map(([k, v]) => `    (${rustStr(k)}, ${rustStr(v)}),`).join('\n')}
];

/// Region subtag to locale id, sorted by key.
///
/// Codcel resolves the region subtag ahead of the language subtag for number
/// and currency conventions, so \`en-PT\` formats the way Portugal does while
/// keeping English names.
pub(super) static REGION_ALIASES: &[(&str, &str)] = &[
${regionAliases.map(([k, v]) => `    (${rustStr(k)}, ${rustStr(v)}),`).join('\n')}
];
`);

fs.mkdirSync('../src/locale', { recursive: true });
fs.writeFileSync('../src/locale/data.rs', parts.join(''));
console.error(
  `wrote ../src/locale/data.rs — ${ids.length} locales, ${languageAliases.length} language aliases, ` +
    `${localeAliases.length} locale aliases, ${regionAliases.length} region aliases`
);
