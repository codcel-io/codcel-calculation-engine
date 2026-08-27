// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Downloads Unicode CLDR data at a pinned release and condenses it into
// `cldr/cldr-condensed.json`, the input to the Rust generator in `src/main.rs`.
//
// Run:  node fetch.mjs
//
// The condensed file is checked in so the generator is reproducible offline.
// Re-run this only when bumping CLDR_TAG.

const CLDR_TAG = '48.2.1';
const BASE = `https://raw.githubusercontent.com/unicode-org/cldr-json/${CLDR_TAG}/cldr-json`;

// CLDR locale ids to condense. Chosen to cover every language and region subtag
// that `ValueFormat` already recognises, plus the desktop app's UI languages.
const LOCALES = [
  // Bare language entries (fallback when no region subtag is supplied)
  'en', 'de', 'fr', 'es', 'pt', 'it', 'nl', 'ro', 'hr', 'sl', 'el', 'tr',
  'pl', 'cs', 'sv', 'fi', 'hu', 'sk', 'et', 'lv', 'lt', 'da', 'nb', 'nn',
  'ru', 'uk', 'bg', 'id', 'ja', 'ko', 'th', 'ms', 'hi', 'bn', 'ta', 'te',
  'mr', 'gu', 'kn', 'ml', 'ar', 'he', 'vi', 'is', 'zh', 'zh-Hant',
  // Region-qualified entries
  'de-DE', 'de-AT', 'de-CH', 'de-LU',
  'en-US', 'en-GB', 'en-CA', 'en-IE', 'en-MT', 'en-AU', 'en-NZ', 'en-ZA',
  'en-SG', 'en-HK', 'en-PH', 'en-IN',
  'fr-FR', 'fr-BE', 'fr-CH', 'fr-CA', 'fr-LU',
  'es-ES', 'es-MX', 'es-CO', 'es-CL', 'es-AR', 'es-US',
  'pt-PT', 'pt-BR',
  'it-IT', 'it-CH',
  'nl-NL', 'nl-BE',
  'el-GR', 'el-CY',
  'ar-SA', 'ar-AE', 'ar-EG',
  'zh-Hans-CN', 'zh-Hant-TW', 'zh-Hant-HK',
  'ro-RO', 'hr-HR', 'sl-SI', 'tr-TR', 'pl-PL', 'cs-CZ', 'sv-SE', 'fi-FI',
  'hu-HU', 'sk-SK', 'et-EE', 'lv-LV', 'lt-LT', 'da-DK', 'nb-NO', 'ru-RU',
  'uk-UA', 'bg-BG', 'id-ID', 'ja-JP', 'ko-KR', 'th-TH', 'ms-MY', 'hi-IN',
  'he-IL', 'vi-VN', 'is-IS',
];

// Region -> the region's currency comes from supplemental/currencyData.json.
// A locale with no region subtag inherits the currency of CLDR's default
// region for that language, resolved below from `likelySubtags`.

async function getJson(url) {
  const res = await fetch(url);
  if (!res.ok) return null;
  return res.json();
}

function pick(obj, keys) {
  const out = {};
  for (const k of keys) if (obj && obj[k] !== undefined) out[k] = obj[k];
  return out;
}

const currencyData = await getJson(`${BASE}/cldr-core/supplemental/currencyData.json`);
const likely = await getJson(`${BASE}/cldr-core/supplemental/likelySubtags.json`);
if (!currencyData || !likely) throw new Error('supplemental fetch failed');

const regionCurrency = {};
for (const [region, entries] of Object.entries(currencyData.supplemental.currencyData.region)) {
  // Entries are ordered oldest-first; the current one has _from but no _to.
  for (const entry of entries) {
    const [code, meta] = Object.entries(entry)[0];
    if (!meta._to && meta._tender !== 'false') regionCurrency[region] = code;
  }
}

const likelySubtags = likely.supplemental.likelySubtags;

function regionFor(loc) {
  const parts = loc.split('-');
  const last = parts[parts.length - 1];
  if (last.length === 2 && last === last.toUpperCase()) return last;
  const expanded = likelySubtags[loc] || likelySubtags[parts[0]];
  if (!expanded) return null;
  const ep = expanded.split('-');
  return ep[ep.length - 1];
}

const out = { cldrVersion: CLDR_TAG, locales: {} };

// `node fetch.mjs --reuse` recomputes only the alias maps from the checked-in
// condensed file, skipping ~230 downloads. Use it when editing alias rules.
const REUSE = process.argv.includes('--reuse');
if (REUSE) {
  const prev = JSON.parse((await import('node:fs')).readFileSync('cldr/cldr-condensed.json', 'utf8'));
  out.locales = prev.locales;
}

for (const loc of REUSE ? [] : LOCALES) {
  const numbers = await getJson(`${BASE}/cldr-numbers-full/main/${loc}/numbers.json`);
  const dates = await getJson(`${BASE}/cldr-dates-full/main/${loc}/ca-gregorian.json`);
  if (!numbers || !dates) { console.error(`skip ${loc} (not in CLDR)`); continue; }

  const n = numbers.main[loc].numbers;
  const ns = n.defaultNumberingSystem === 'latn' ? 'latn' : n.defaultNumberingSystem;
  const sym = n[`symbols-numberSystem-latn`];
  const curFmt = n[`currencyFormats-numberSystem-latn`];
  const decFmt = n[`decimalFormats-numberSystem-latn`];

  const g = dates.main[loc].dates.calendars.gregorian;

  const region = regionFor(loc);
  const currencyCode = (region && regionCurrency[region]) || 'USD';

  const currencies = await getJson(`${BASE}/cldr-numbers-full/main/${loc}/currencies.json`);
  const cEntry = currencies && currencies.main[loc].numbers.currencies[currencyCode];
  const currencySymbol = (cEntry && (cEntry['symbol-alt-narrow'] || cEntry.symbol)) || currencyCode;

  out.locales[loc] = {
    region,
    defaultNumberingSystem: ns,
    symbols: pick(sym, [
      'decimal', 'group', 'list', 'percentSign', 'plusSign', 'minusSign',
      'approximatelySign', 'exponential', 'superscriptingExponent',
      'perMille', 'infinity', 'nan', 'timeSeparator',
    ]),
    decimalStandard: decFmt.standard,
    currency: {
      code: currencyCode,
      symbol: currencySymbol,
      standard: curFmt.standard,
      accounting: curFmt.accounting,
    },
    dates: {
      monthsWide: Object.values(g.months.format.wide),
      monthsAbbrev: Object.values(g.months.format.abbreviated),
      monthsNarrow: Object.values(g.months['stand-alone'].narrow),
      daysWide: g.days.format.wide,
      daysAbbrev: g.days.format.abbreviated,
      am: g.dayPeriods.format.abbreviated.am,
      pm: g.dayPeriods.format.abbreviated.pm,
      dateFormats: pick(g.dateFormats, ['full', 'long', 'medium', 'short']),
      timeFormats: pick(g.timeFormats, ['full', 'long', 'medium', 'short']),
      dateTimeFormats: pick(g.dateTimeFormats, ['full', 'long', 'medium', 'short']),
    },
  };
  console.error(`ok ${loc} (${region} ${currencyCode} ${currencySymbol})`);
}

// ---------------------------------------------------------------------------
// Alias maps.
//
// CLDR omits a locale id when it is the default for its language: there is no
// `de-DE` because `de` already is `de-DE`, no `pt-BR` because Portuguese
// defaults to Brazil. Resolve those collapses here so the Rust lookup can be a
// plain table probe with no subtag algebra at runtime.
// ---------------------------------------------------------------------------

const fetched = new Set(Object.keys(out.locales));

/** Collapse `lang-REGION` onto whichever locale id CLDR actually ships. */
function canonical(lang, region) {
  const exact = `${lang}-${region}`;
  if (fetched.has(exact)) return exact;

  // A language's default region collapses onto the bare id: CLDR ships no
  // `de-DE` because `de` already is `de-DE`, and no `pt-BR` because Portuguese
  // defaults to Brazil rather than Portugal.
  const langParts = (likelySubtags[lang] || '').split('-');
  const defaultRegion = langParts[langParts.length - 1];
  if (region === defaultRegion && fetched.has(lang)) return lang;

  // Otherwise resolve the script CLDR would infer for this region and probe
  // `lang-Script-REGION` then `lang-Script`. This is what separates zh-TW
  // (Traditional) from zh-CN (Simplified), which share a language subtag.
  const parts = (likelySubtags[exact] || likelySubtags[lang] || '').split('-');
  if (parts.length === 3) {
    const withScript = `${parts[0]}-${parts[1]}-${region}`;
    if (fetched.has(withScript)) return withScript;
    const scriptOnly = `${parts[0]}-${parts[1]}`;
    if (fetched.has(scriptOnly)) return scriptOnly;
  }

  return fetched.has(lang) ? lang : null;
}

const languages = [...fetched].map((l) => l.split('-')[0]);
const regions = [...new Set(Object.values(out.locales).map((l) => l.region).filter(Boolean))];

// language -> locale id  (no region subtag supplied)
out.languageAliases = {};
for (const lang of new Set(languages)) {
  const id = fetched.has(lang) ? lang : canonical(lang, null);
  if (id) out.languageAliases[lang] = id;
}

// "lang-REGION" -> locale id
out.localeAliases = {};
for (const lang of new Set(languages)) {
  for (const region of regions) {
    const id = canonical(lang, region);
    if (id && id !== lang) out.localeAliases[`${lang}-${region}`] = id;
  }
}

// region -> locale id. Codcel resolves the region subtag ahead of the language
// subtag for number and currency conventions, so `en-PT` formats like Portugal.
// Prefer a bare-language entry for the region over a qualified one.
out.regionAliases = {};
for (const region of regions) {
  const candidates = Object.entries(out.locales)
    .filter(([, v]) => v.region === region)
    .map(([k]) => k)
    .sort((a, b) => a.split('-').length - b.split('-').length || a.length - b.length);
  if (candidates.length) out.regionAliases[region] = candidates[0];
}

const fs = await import('node:fs');
fs.writeFileSync('cldr/cldr-condensed.json', JSON.stringify(out, null, 1) + '\n');
console.error(
  `\nwrote cldr/cldr-condensed.json — ${Object.keys(out.locales).length} locales, ` +
    `${Object.keys(out.localeAliases).length} locale aliases, ` +
    `${Object.keys(out.regionAliases).length} region aliases`
);
