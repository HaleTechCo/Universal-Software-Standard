# Locale and translation: in USS audits, and of USS itself

Research notes for the `todo.txt` item "translations/locale"
(`docs/superpowers/follow-ups.md:339-341`). The maintainer's answer covers
three subjects:

- (A) What an audit checks in a project about locale and translation.
- (B) The audit itself in the language and locale of the developer.
- (C) Translations of USS itself: the standard, the workflows, the messages
  of `uss` and the report.

Repository state read: `main` at `dd5ab83` (2026-11-25). Every web source
below was read on 2026-11-25 unless the note says otherwise. Nothing in the
repository was changed except this file.

Headline recommendations:

| Part | Recommended design | Effort |
|---|---|---|
| A | Give USS-I18N-006 `Enables: offers-multiple-languages` and extend it to `has-cli`. Add 13 rules (I18N-007 to I18N-019): 6 for every UI or CLI (direction isolation, locale-independent machine formats, user time zone, Unicode equivalence, collation, grapheme-safe cutting), 7 behind the new fact (message patterns and plurals, BCP 47 tags, language negotiation and fallback, RTL mirroring, catalogue checks in CI, pseudo-locale tests, translator context). | M (USS); adopters mostly S |
| B | English stays the language of everything normative or machine-read. The agent speaks the developer's language. A new optional `project.language` (BCP 47) in the record sets the language of evidence, reasons and the report. `uss report --lang` and Fluent catalogues render the report; dates stay `YYYY-MM-DD`; a non-English report also carries the English statement. | S (record, workflow) + M (report) |
| C | W3C model: English is the only normative text, translations are informative and pinned to the same USS version. Keywords stay in English capitals in parentheses after the translated verb. po4a PO files per English file, kept in the repository, any PO tool or Weblate as front end; Fluent `.ftl` for `uss`. `uss show --lang` and `uss applicable --full --lang` serve them with RFC 4647 lookup and English fallback per paragraph. | L |

## (A) What an audit checks in a project about locale and translation

### Takeaway

The six I18N rules cover encoding, text kept out of code, locale formatting,
UTC storage, RFC 3339 exchange and an optional second language; WCAG 2.2
through USS-ACC-001 already covers the `lang` attribute of a UI. The gaps are
Unicode text handling (normalisation, collation, grapheme clusters),
bidirectional text, time-zone display, locale-independent machine formats,
and every rule for software that is actually translated (plurals and message
patterns, BCP 47 tags, negotiation and fallback, RTL layout, catalogue checks,
pseudo-localisation, translator context). The clean gate for the last group
is a fact that USS-I18N-006 enables, as USS-EXT-001 enables
`has-plugin-interface`.

### Cited Findings

Repository:

- The I18N area has six rules: USS-I18N-001 Unicode throughout (MUST,
  `has-ui or has-cli or stores-persistent-data or uses-network`), 002 keep
  user-facing text out of code (SHOULD, `has-ui or has-cli`), 003 format for
  the user's locale (SHOULD, `has-ui`; numbers, dates, times, currency), 004
  store timestamps in UTC, with a zone ID for future local times (MUST,
  `stores-persistent-data`), 005 exchange timestamps in RFC 3339 (MUST,
  `exposes-network-api`), 006 offer more than one language (MAY, `has-ui`, no
  `Enables`) — `standard/core/internationalization.md:12-155`.
- The I18N area cites The Unicode Standard 18.0.0, RFC 3629, Unicode CLDR 49
  (22 October 2026), W3C Working with Time and Timezones (Group Note Draft,
  20 November 2026) and RFC 3339 —
  `standard/core/internationalization.md:29-127`.
- USS-ACC-001 requires WCAG 2.2 Level AA for every UI (area gate `has-ui`),
  with WCAG2ICT for non-web UIs — `standard/core/accessibility.md:1-28`.
- The Usability area hands "the language and locale of the interface" to the
  I18N area — `standard/core/usability.md:10-11`.
- USS-DOC-007 asks for ASD-STE100 in every set of steps, and its rationale
  names readers "who do not have the project's language as their first one" —
  `standard/core/documentation.md:133-157`.
- The facts file has no fact for translated software; `has-ui`, `has-cli`,
  `stores-persistent-data`, `uses-network` and `exposes-network-api` exist,
  and an `Enabled by` fact pattern exists (`has-plugin-interface` by
  USS-EXT-001) — `standard/applicability.md:9-63`, `:253-261`.
- A fact can be switched on by a MAY rule's `Enables` field, and the two sides
  must agree — `standard/conventions.md:290-300`.
- No core rule mentions collation, normalisation, grapheme clusters, bidi,
  pseudo-localisation, `Accept-Language` or the tz database (search of
  `standard/` for those words found only the I18N area and unrelated hits).
- The next free numbers: I18N-007, ACC-006, UX-012, DOC-008, TST-013 (last
  headings in each file).

External:

- WCAG 2.2 SC 3.1.2 Language of Parts is Level AA: "The human language of
  each passage or phrase in the content can be programmatically determined",
  with exceptions for proper names, technical terms and similar —
  [W3C Understanding SC 3.1.2](https://www.w3.org/WAI/WCAG22/Understanding/language-of-parts.html).
  The page names SC 3.1.1 Language of Page but did not state its level; it is
  Level A in [WCAG 2.2](https://www.w3.org/TR/WCAG22/) (not re-read in this
  session).
- Language tag syntax is defined by BCP 47; the current RFC is 5646; all
  subtags are in the IANA Language Subtag Registry; "keep your language tag as
  short as possible"; script subtags only when they add information; region
  subtags only when they distinguish the tag — [W3C, Choosing a Language
  Tag](https://www.w3.org/International/questions/qa-choosing-language-tags)
  (no date on the page).
- RFC 4647 (September 2006) is the matching half of BCP 47. Lookup "returns
  exactly one tag, or a default"; "the language range is progressively
  truncated from the end until a matching language tag is located"; "If no
  language tag matches the request, the 'default' value is returned"; each
  range of a priority list is tried before the default —
  [RFC 4647](https://www.rfc-editor.org/rfc/rfc4647.html).
- POSIX.1-2024 (IEEE Std 1003.1-2024, Base Definitions 8.2): the first of
  these that is set and not null decides: `LC_ALL`, then the `LC_*` variable of
  the category (such as `LC_MESSAGES`), then `LANG`, then an
  implementation-defined default —
  [Open Group Base Specifications Issue 8, chapter 8](https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/V1_chap08.html).
- W3C RTL guidance: "If the overall document direction is right-to-left, add
  `dir="rtl"` to the `html` tag"; "Never use CSS to apply the base direction";
  "Set the `dir` attribute to `auto` on forms and inserted text"; use `bdi` to
  isolate a name or label; use logical CSS properties so layouts flip —
  [W3C, Structural markup and right-to-left text in HTML](https://www.w3.org/International/questions/qa-html-dir)
  (no date on the page).
- Microsoft pseudolocalization guidance (ms.date 2022-06-07): pseudo can be
  applied "in the earliest phases of development before translation begins"
  and "sustained throughout the development cycle"; pseudotranslation adds
  accents and other scripts, lengthens text (about 40% for English source;
  real cases reach 200% to 400%), adds start and end delimiters that reveal
  truncation and concatenation, and leaves unlocalised resources visibly
  untranslated; it "isn't a complete replacement for validating the translated
  application"; pseudo-locales find number, date, time and currency format
  faults; Windows has `qps-ploc`, `qps-plocm` (mirrored) and `qps-ploca`; ICU
  has `en-XA` (accents) and `en-XB` (bidi); pseudo-locale names must never
  describe a real locale (the `tk-TM` collision), and `qaa`-`qtz` are reserved
  for private use —
  [Microsoft Learn, Pseudolocalization](https://learn.microsoft.com/en-us/globalization/methodology/pseudolocalization).
- Unicode Standard Annex #15, Unicode Normalization Forms: Version Unicode
  18.0.0, Revision 58, 2026-08-12; "Two equivalent strings will have precisely
  the same normalized form"; the KC and KD forms "must not be blindly applied
  to arbitrary text"; W3C specifications recommend NFC for all content —
  [UAX #15](https://www.unicode.org/reports/tr15/).
- The latest Unicode Standard is 18.0.0 (`/versions/latest/` redirects to
  `/versions/Unicode18.0.0/`) —
  [unicode.org/versions/latest](https://www.unicode.org/versions/latest/).
- CLDR releases: 49 (2026-10-22), 48.2 (2026-03-17), 48.1 (2026-01-08), 48
  (2025-10-29) — [CLDR downloads](https://cldr.unicode.org/index/downloads).
  This confirms the CLDR 49 reference that USS-I18N-003 already uses.
- The latest IANA Time Zone Database is 2026f, released 2026-10-27 —
  [IANA time zones](https://www.iana.org/time-zones). (The letter f means six
  releases in 2026 so far: an inference from the naming scheme.)
- Unicode MessageFormat 2 is "a Unicode standard for localizable dynamic
  message strings"; it "supports pluralization ... even in languages with
  complex plural rules"; it is "built directly into ICU4C, ICU4J, and soon,
  ECMA-402". The page does not say which LDML/CLDR release made it stable —
  [messageformat.unicode.org](https://messageformat.unicode.org/).
- Project Fluent: "A simple string in English can map to a complex
  multi-variant translation in another language"; "Translators should be able
  to use the entire expressive power of their language without asking
  developers for permission"; variants use CLDR plural categories (`one`,
  `other`; Polish adds `few` and `many`); "Fluent Syntax 1.0 was released in
  April 2019"; Rust implementation fluent-rs —
  [projectfluent.org](https://projectfluent.org/).
- fluent-rs crates include `fluent-bundle`, `fluent-syntax`,
  `fluent-fallback`, `fluent-resmgr`, `fluent-pseudo` (pseudolocalization) and
  `intl-memoizer`; licences Apache-2.0 and MIT —
  [github.com/projectfluent/fluent-rs](https://github.com/projectfluent/fluent-rs).

### Inferences

#### Coverage and gaps

| Topic | Now | Proposal | Level | Gate |
|---|---|---|---|---|
| Text out of code | I18N-002 | keep | SHOULD | `has-ui or has-cli` |
| Locale formatting of numbers, dates, currency | I18N-003 (display only) | optionally extend to input parsing (pre-release change) | SHOULD | `has-ui` |
| Machine formats independent of locale | none | new I18N-008 | MUST | as I18N-001 |
| Time zones: storage, exchange | I18N-004, I18N-005 | keep | MUST | as now |
| Time zones: display in the user's zone, IANA tz | none | new I18N-009 | SHOULD | `has-ui` |
| Unicode encoding | I18N-001 | keep | MUST | as now |
| Normalisation and case folding in comparisons | none | new I18N-010 | SHOULD | as I18N-001 |
| Collation for sorted lists | none | new I18N-011 | SHOULD | `has-ui` |
| Grapheme clusters when cutting text | none | new I18N-012 | SHOULD | `has-ui or has-cli` |
| Bidi isolation of outside text | none (WCAG does not cover it) | new I18N-007 | SHOULD | `has-ui` |
| `lang` attribute, language of parts | USS-ACC-001 (WCAG 2.2 3.1.1 A, 3.1.2 AA) | keep; no new rule | MUST | `has-ui` |
| More than one language | I18N-006 | add `Enables: offers-multiple-languages`, add `has-cli` | MAY | `has-ui or has-cli` |
| Plurals, gender, no concatenation | none | new I18N-013 | MUST | `offers-multiple-languages` |
| BCP 47 tags outside the UI | none | new I18N-014 | MUST | `offers-multiple-languages` |
| Negotiation and fallback | I18N-006 Verify only ("by the system language or by a setting") | new I18N-015 | SHOULD | `offers-multiple-languages` |
| RTL layout mirroring | none | new I18N-016 | MUST | `offers-multiple-languages` |
| Catalogue checks in CI | none | new I18N-017 | MUST | `offers-multiple-languages` |
| Pseudo-localisation | none | new I18N-018 | SHOULD | `offers-multiple-languages` |
| Translator context | none | new I18N-019 | SHOULD | `offers-multiple-languages` |

Why these levels (conventions section 7, "Choosing a level"):

- MUST where the absence is a defect in every project that the gate selects
  and any team can meet it with platform or library support: a machine format
  that changes with the host locale (I18N-008); in translated software, a
  sentence joined from parts or an English-only plural (I18N-013), a
  non-standard language code (I18N-014), a left-to-right layout in a
  right-to-left language (I18N-016), and an unchecked placeholder that can
  crash one language (I18N-017).
- SHOULD where legitimate reasons to skip exist, such as a tool whose only
  sorted list is of file names, or a project without CI screenshots
  (I18N-007, 009 to 012, 015, 018, 019).
- No new MAY rule is needed. A MAY rule "Translate the user documentation"
  (gate `offers-multiple-languages`, category Features) is optional and left
  out of the recommended set.

#### Recommended design

1. Change USS-I18N-006 in place before 0.1.0: add
   `Enables: offers-multiple-languages`, and widen it to `has-ui or has-cli`
   (command-line tools are translated too). After 0.1.0 the same change would
   need a retired rule and a new ID (conventions section 4).
2. Add the enabled fact `offers-multiple-languages`. An enabled fact fits
   better than an asked fact: a project that already ships two languages
   adopts I18N-006, so the record states it once, and no second question
   duplicates the MAY decision.
3. Add I18N-007 to I18N-019 (drafts below), in a MINOR release (0.2.0), after
   each reference is read at writing time.
4. Handle right-to-left in the Verify of I18N-016 ("if no offered language is
   right-to-left, pass with the list as evidence"), not with a new fact.
5. Leave security-related Unicode out of this area: confusable identifiers
   (UTS #39) and bidi control characters in source code (the "Trojan Source"
   attack, CVE-2021-42574) belong to the Security or Code quality areas.

Alternatives, one line each:

- Put all translation quality into the Verify of I18N-006: rejected, because
  one MAY rule gives no score signal and mixes many requirements.
- An asked fact such as `is-localized`: rejected, because it duplicates the
  I18N-006 decision and can disagree with it.
- A module (such as `standard/modules/i18n/`): rejected, because modules are
  for technologies, and these rules are language-neutral core rules.
- An asked fact `offers-rtl-language` for I18N-016: possible, but it adds a
  question to every record for a rare case.

Effort: M for the USS maintainers (13 rules, about 10 references to read and
pin). For adopters: S for a single-language UI or CLI (6 SHOULD rules and 1
MUST rule, most met by the platform); M to L for translated software, which
is the cost of translation done correctly.

#### Draft: new fact (`standard/applicability.md`, after `has-plugin-marketplace` or with the other enabled facts)

```markdown
### offers-multiple-languages

- **Enabled by:** USS-I18N-006
- **Definition:** Yes when the project adopts USS-I18N-006: its user interface or command line is available in at least two languages.
```

#### Draft: changed rule USS-I18N-006 (pre-release change in place)

```markdown
### USS-I18N-006: Offer more than one language

- **Level:** MAY
- **Applies when:** has-ui or has-cli
- **Category:** Accessibility & Usability
- **Enables:** offers-multiple-languages
- **Verify:** Adopting this rule means that the UI or the command line is
  available in at least two languages. The project lists its languages (in
  the documentation, or as the language files that it ships). The user can
  use each one, by the system language or by a setting. For each listed
  language, open the screens of the main workflows, or run the documented
  commands: their text is in that language. A listed language that shows the
  text of another language there fails. A project that adopts the rule and
  lists fewer than two languages fails.
- **References:** W3C Internationalization, Localization vs.
  Internationalization (read 2026-11-24)

The user interface or the command line MAY be available in more than one
language.

**Rationale:** Most people use software best in the language they think in.
The rule on keeping text out of code (USS-I18N-002) is what makes a second
language a matter of a new file and not of a new version. Adopting this rule
switches on the rules for translated software.
```

#### Draft: new rules (append to `standard/core/internationalization.md`)

The references marked "confirm" were not read in this session; conventions
section 7 requires reading them at writing time.

```markdown
### USS-I18N-007: Isolate the direction of text from outside

- **Level:** SHOULD
- **Applies when:** has-ui
- **Category:** Accessibility & Usability
- **Verify:** Find where the UI shows text that users or outside sources
  supply, such as names, titles, messages and file names. Each such text has
  its own base direction, isolated from the text around it. Examples are
  HTML `dir="auto"`, the `bdi` element, and the platform's bidi isolation. In
  a test instance, enter `שלום עולם` as a name, and open a screen that shows
  the name with a number or brackets after it. The name and the UI text keep
  their order, and the brackets are not split or reversed. A UI that shows no
  such text passes; give the list of screens as evidence. A text that moves
  the text around it fails.
- **References:** Unicode Standard Annex #9, Unicode Bidirectional Algorithm
  (revision for Unicode 18.0.0, confirm); W3C Internationalization,
  Structural markup and right-to-left text in HTML (read 2026-11-25)

Text that users or outside sources supply SHOULD be shown with its own base
direction, isolated from the text around it.

**Rationale:** A name in Arabic or Hebrew can move the numbers and the
punctuation of the sentence around it, so that the sentence says something
else. Any UI can show such a name, in any language of its own.

### USS-I18N-008: Keep machine formats independent of the locale

- **Level:** MUST
- **Applies when:** has-ui or has-cli or stores-persistent-data or uses-network
- **Category:** Compatibility
- **Verify:** Find where the software writes or reads data for programs:
  files, configuration, network messages, logs that tools parse, and
  machine-readable output (USS-UX-009). Each formats and parses numbers,
  dates and the case of text with a fixed setting, such as the C or root
  locale or an invariant culture. It does not use the locale of the host. Run
  the tests or a test instance twice: once with `LC_ALL=C.UTF-8`, once with
  `LC_ALL=tr_TR.UTF-8` (on Windows, the `tr-TR` culture). Write the same data
  in each run. Compare the outputs, apart from values that change between
  runs, such as the current time. They are the same, and each run reads the
  output of the other. Turkish uses a decimal comma and a dotless `ı`, so the
  two common faults show. A difference, or a failure to read, fails.
- **References:** IEEE Std 1003.1-2024 (POSIX.1-2024), Base Definitions, 8.2
  Internationalization Variables; a guidance source on invariant formats
  (confirm)

Data that programs read MUST be formatted and parsed independently of the
locale of the host.

**Rationale:** A program that writes `1,5` on one machine and reads `1.5` on
another loses data with no error. The Turkish dotless `ı` changes keys and
commands in the same way when code changes the case of text with the host's
locale.

### USS-I18N-009: Show times in the user's time zone

- **Level:** SHOULD
- **Applies when:** has-ui
- **Category:** Accessibility & Usability
- **Verify:** Find each place where the UI shows a point in time. Each is
  converted from the stored instant to the user's time zone (from the system
  or a user setting). The conversion uses the IANA time zone database through
  the platform or a maintained library, not a fixed offset in the code. Run
  the UI with the time zone `America/New_York`, then `Asia/Kolkata`. Open a
  screen that shows a stored time in January: the shown time moves by 10
  hours 30 minutes. A fixed offset, or a time that does not move, fails.
- **References:** IANA Time Zone Database, version 2026f (27 October 2026);
  W3C Working with Time and Timezones (Group Note Draft, 20 November 2026)
  (section to confirm)

Points in time SHOULD be shown in the user's time zone, converted with the
IANA time zone database.

**Rationale:** A time in the server's zone is wrong for every user in another
zone. The rules of time zones change several times a year, and only the
maintained database follows them.

### USS-I18N-010: Compare text by Unicode equivalence

- **Level:** SHOULD
- **Applies when:** has-ui or has-cli or stores-persistent-data or uses-network
- **Category:** Accessibility & Usability
- **Verify:** Find where the code compares, looks up or removes duplicates of
  text that users or outside sources supply, such as names, identifiers,
  search terms and keys. Before each comparison, the code brings both texts
  to one normalisation form (NFC, or NFKC where compatibility variants must
  match). Where the comparison ignores case, it uses Unicode full case
  folding, not an ASCII-only conversion. In a test instance, store `café`
  with `é` as one code point (U+00E9). Search for `café` with `e` followed by
  U+0301: it matches. Where case is ignored, `STRASSE` also matches `straße`.
  A comparison of bytes or code points only fails.
- **References:** Unicode Standard Annex #15, Unicode Normalization Forms,
  Revision 58 (12 August 2026); The Unicode Standard, Version 18.0.0,
  chapter 3, Default Case Algorithms (confirm)

Text from users or outside sources SHOULD be compared by Unicode
equivalence: in one normalisation form and, where case is ignored, after full
case folding.

**Rationale:** The same word can arrive as different code points from
different keyboards and systems. A comparison of bytes then finds two users
with one name, or no match for a word that is there.

### USS-I18N-011: Sort text with the collation of the user's language

- **Level:** SHOULD
- **Applies when:** has-ui
- **Category:** Accessibility & Usability
- **Verify:** Find each list that the UI sorts by text, such as names, files
  and tags. Each sort uses the collation of the user's locale: the Unicode
  Collation Algorithm with CLDR data, through the platform or a library such
  as ICU. It does not use code-point or byte order. Sort the items `Zebra`,
  `Äpfel`, `apple` and `Österreich` in the UI. In `de-DE` the order is
  `Äpfel, apple, Österreich, Zebra`. In `sv-SE` it is
  `apple, Zebra, Äpfel, Österreich`. The code-point order
  `Zebra, apple, Äpfel, Österreich` fails.
- **References:** Unicode Technical Standard #10, Unicode Collation Algorithm
  (version for Unicode 18.0.0, confirm); Unicode CLDR, Version 49
  (22 October 2026)

Lists of text that people read SHOULD be sorted with the collation of the
user's locale.

**Rationale:** Each language has its own alphabetical order, and code-point
order matches none of them. It puts every capital letter before every small
letter, and every accented letter at the end.

### USS-I18N-012: Cut text only between grapheme clusters

- **Level:** SHOULD
- **Applies when:** has-ui or has-cli
- **Category:** Accessibility & Usability
- **Verify:** Find where the code shortens text for display, limits its
  length, or moves a cursor through it. Each counts and cuts by extended
  grapheme clusters, through the platform or a library, not by bytes or code
  units. In a test instance, give a text that has `👍🏽`, and `é` written as
  `e` and U+0301, at the place where the code cuts it. No character loses its
  accent or skin tone, and no invalid UTF-8 or lone surrogate appears. A cut
  inside a cluster fails.
- **References:** Unicode Standard Annex #29, Unicode Text Segmentation
  (revision for Unicode 18.0.0, confirm)

Text SHOULD be counted and cut only at grapheme cluster boundaries.

**Rationale:** What a reader sees as one character can be several code
points. A cut between them leaves a letter without its accent, an emoji in two
parts, or bytes that are not valid text.

### USS-I18N-013: Build each message from one translatable pattern

- **Level:** MUST
- **Applies when:** offers-multiple-languages
- **Category:** Accessibility & Usability
- **Verify:** Read the message catalogues and the code that formats messages.
  Each message that a user sees is one catalogue entry, with named
  placeholders for its values. The code does not join translated parts into a
  sentence. It does not choose between the singular and plural forms of a
  message. A message with a count uses the CLDR plural categories (such as
  `one`, `few`, `many` and `other`) through its message format. Examples are
  ICU MessageFormat, Fluent selectors and gettext `ngettext`. Search the code
  for text joined to a translated string, and for a condition on a count that
  chooses a message. A joined sentence, or a plural form that the code
  chooses, fails.
- **References:** Unicode CLDR, Version 49 (22 October 2026), plural rules;
  Unicode MessageFormat 2 (messageformat.unicode.org, read 2026-11-25);
  Fluent Syntax 1.0 (April 2019)

Each user-facing message MUST be one translatable pattern that holds its
placeholders and its plural forms.

**Rationale:** Languages put words in different orders, and many have more
plural forms than English. Parts joined in code fix the English order and the
English plural, so no translator can write a correct sentence.

### USS-I18N-014: Identify languages with BCP 47 tags

- **Level:** MUST
- **Applies when:** offers-multiple-languages
- **Category:** Accessibility & Usability
- **Verify:** List each place where the software names a language or a
  locale: settings, stored values, API fields, the HTML `lang` attribute, and
  the HTTP `Content-Language` and `Accept-Language` headers. Each value is a
  well-formed BCP 47 tag whose subtags are in the IANA Language Subtag
  Registry, such as `pt-BR` or `zh-Hant`. A catalogue file name can use the
  platform's form of a tag, such as `pt_BR.po` for gettext. Check each value
  with a BCP 47 parser. A language name, a country code used for a language
  (such as `jp`), or a private code fails.
- **References:** BCP 47 (RFC 5646, September 2009; RFC 4647, September
  2006); IANA Language Subtag Registry (read at writing time); W3C
  Internationalization, Choosing a Language Tag (read 2026-11-25)

Each language and locale that the software names MUST be identified by a
BCP 47 language tag.

**Rationale:** Browsers, operating systems, assistive technology and
translation tools all read BCP 47 tags. With a private code, each of them
guesses the language, and a screen reader can read the text with the wrong
voice.

### USS-I18N-015: Negotiate the language of each message

- **Level:** SHOULD
- **Applies when:** offers-multiple-languages
- **Category:** Accessibility & Usability
- **Verify:** Find how the software picks its language. It reads the user's
  ordered preferences: a setting of its own first, then the
  `Accept-Language` header, the system languages, or `LC_ALL`, `LC_MESSAGES`
  and `LANG` in that order. It takes the first language that it offers, by
  BCP 47 lookup: it removes subtags from the end of each tag until one
  matches. Run it with the preferences `fr-CA, de` and catalogues for `en`
  and `fr`: it shows French. Remove one message from the French catalogue:
  that message shows in the default language, not as its key. Another
  language, or a key on the screen, fails.
- **References:** RFC 4647 (September 2006), 3.4 Lookup; IEEE Std
  1003.1-2024 (POSIX.1-2024), Base Definitions, 8.2 Internationalization
  Variables; RFC 9110 (June 2022), Accept-Language (confirm)

Each message SHOULD be shown in the first language of the user's
preferences that has a translation of it, or else in the default language.

**Rationale:** Many people read more than one language and say so in their
settings. A key such as `btn_save_2` helps no one, but a message in a second
language still helps.

### USS-I18N-016: Mirror the layout for right-to-left languages

- **Level:** MUST
- **Applies when:** offers-multiple-languages
- **Category:** Accessibility & Usability
- **Verify:** List the languages that the project offers. If none is written
  from right to left (such as Arabic, Hebrew, Persian or Urdu), the rule
  passes; give the list as evidence. Otherwise, run the UI in each such
  language. The base direction is right to left, such as `dir="rtl"` on the
  `html` element or the platform's layout direction. The layout is mirrored:
  navigation, alignment, and icons that show a direction, such as arrows.
  Numbers, code and logos are not mirrored. A screen that keeps the
  left-to-right layout fails.
- **References:** W3C Internationalization, Structural markup and
  right-to-left text in HTML (read 2026-11-25); Unicode Standard Annex #9,
  Unicode Bidirectional Algorithm (revision for Unicode 18.0.0, confirm)

The UI MUST use a right-to-left base direction and a mirrored layout in each
right-to-left language that it offers.

**Rationale:** A reader of Arabic or Hebrew starts at the right. A
left-to-right layout with translated text puts the start of each line, and the
first step of each flow, on the wrong side.

### USS-I18N-017: Check the message catalogues in CI

- **Level:** MUST
- **Applies when:** offers-multiple-languages
- **Category:** Accessibility & Usability
- **Verify:** Find the CI job that checks the message catalogues. It parses
  every catalogue. It fails on a syntax error, on a message that the source
  catalogue does not have, and on a placeholder that differs from the source
  message. It reports the share of the messages that each language
  translates. Use the latest run on the default branch. Then, in a copy, add
  to one translation a placeholder that its source message does not have.
  Run the check: it fails. A missing job, or a check that passes the broken
  message, fails.
- **References:** Fluent Syntax 1.0 (April 2019); GNU gettext manual,
  `msgfmt --check` (version to confirm)

CI MUST check every message catalogue for its syntax and for placeholders
that match the source messages.

**Rationale:** A translated message with a wrong placeholder can crash the
software or show the wrong value, and only in that one language. No test in
the source language finds it.

### USS-I18N-018: Test with a pseudo-locale

- **Level:** SHOULD
- **Applies when:** offers-multiple-languages
- **Category:** Accessibility & Usability
- **Verify:** Find the pseudo-locale of the project. Examples are `en-XA` and
  `en-XB` in ICU, `qps-ploc` and `qps-plocm` in Windows, and the transforms
  of a library such as `fluent-pseudo`. It adds accents, makes each text
  about 40% longer, and marks the start and the end of each text. Find the
  CI job that runs the UI tests, or takes images of the main screens, in the
  pseudo-locale. In the latest run on the default branch, each text on the
  tested screens is pseudo-translated and shows both marks. A text with no
  marks is in the code, a text with no end mark is cut off, and marks inside
  a text show joined parts. Each of these fails.
- **References:** Microsoft Globalization documentation, Pseudolocalization
  (7 June 2022, read 2026-11-25); RFC 5646 (September 2009), 2.2.1
  (private-use language subtags)

The UI or the command line SHOULD be tested in CI with a pseudo-locale that
marks, accents and lengthens each text.

**Rationale:** A pseudo-locale finds text left in the code, text that a
translation will cut off, and sentences joined from parts. It finds them
before a translator starts, and nobody needs to know another language.

### USS-I18N-019: Give translators the context of each message

- **Level:** SHOULD
- **Applies when:** offers-multiple-languages
- **Category:** Accessibility & Usability
- **Verify:** Read a sample of 20 messages from the source catalogue, or all
  of them when there are fewer. Each message that has a placeholder, or that
  has three words or fewer, has a comment for translators. The comment says
  where the message appears, what each placeholder holds, and any length
  limit. A word with several meanings, such as "Open" or "Order", has a
  separate key or a context field (such as gettext `msgctxt`) for each
  meaning. A message with no comment, or one key for two meanings, fails.
- **References:** GNU gettext manual, Contexts and comments for translators
  (version to confirm); Fluent Syntax 1.0 (April 2019), comments

Each message SHOULD carry the context that a translator needs to translate
it correctly.

**Rationale:** A translator sees a list of strings, not the screen. "Open"
can be a command on a button or a state in a list, and many languages use two
different words.
```

Optional pre-release change to USS-I18N-003: add input to the requirement
("Numbers, dates, times and currencies SHOULD be shown and read in the format
of the user's locale") and a Verify step "enter `1.234,5` in a number field
in `de-DE`: it reads as 1234.5". Done before 0.1.0, it needs no retired rule.

### Gaps

- The revision numbers of UAX #9, UAX #29 and UTS #10 for Unicode 18.0.0 were
  not read: `https://www.unicode.org/reports/` shows no revisions. Read each
  report's header at writing time.
- No guidance source on locale-independent ("invariant") machine formats was
  read for I18N-008. Candidates: the Microsoft globalization pages on
  culture-invariant formatting, or the Java and .NET API documentation.
- The GNU gettext manual (locale variables, `LANGUAGE`, `msgctxt`,
  `msgfmt --check`) returned HTTP 429 twice. The order of `LANGUAGE` before
  `LC_ALL` in gettext is therefore unverified; I18N-015 cites only the POSIX
  order.
- The page did not confirm which LDML or CLDR release made MessageFormat 2
  stable.
- RFC 9110's `Accept-Language` section and the IANA subtag registry date were
  not read.
- Personal names and addresses (the W3C guidance on names around the world),
  and money stored as an amount plus an ISO 4217 code, are candidate rules
  that were not researched.
- Whether the WCAG2ICT mapping of SC 3.1.1 and 3.1.2 to non-web software
  gives a CLI any language-tagging duty was not checked.

## (B) The audit in the developer's language and locale

### Takeaway

Keep English for everything that is normative or that a program reads: rule
IDs, the five keywords, fact IDs, record keys, outcomes, the Verify text that
the agent applies, and the compliance statement of record. Let the agent talk
in the developer's language, and add one optional record key,
`project.language` (a BCP 47 tag), for the language of the evidence, the
reasons and the report. The report language must come from the record, not
from the environment, because `uss release-check` regenerates reports in CI.

### Cited Findings

Repository:

- The record schema allows only `project.name` under `project`, and refuses
  unknown keys (`additionalProperties: false`) —
  `schema/record.schema.json:18-25`; `standard/conventions.md:581-585`.
- Dates in the record and the results are `YYYY-MM-DD` (schema pattern
  `^\d{4}-\d{2}-\d{2}$`) — `schema/record.schema.json:89`;
  `standard/conventions.md:603`, `:633`.
- The compliance statement has a fixed English form, `Compliant with USS
  <version> as of <date>.` or `Not compliant ...: <parts>`, and English
  singular and plural parts (`1 MUST failure`) —
  `standard/conventions.md:485-516`.
- `uss` builds the statement in code with hard-coded English singular and
  plural pairs (`"MUST failure", "MUST failures"`, and so on) —
  `tools/uss/src/evaluate/score.rs:285-312`.
- The report section headings ("Fail", "Unverified", "Deviation", "Declined",
  "Pass") are literals in `tools/uss/src/report.rs:17-23`; the `percent`
  filter writes `N%` or `n/a` in code (`report.rs:164-166`); the other
  headings and sentences ("## Scores", "## Stack without a module", the table
  headers) are English text in `templates/audit-report.md.j2`.
- The report is rendered with minijinja from `templates/audit-report.md.j2`
  (`report.rs:13-14`, `:188-214`).
- The self-audit records USS-I18N-002 as `fail`: "messages are literals in the
  code ... the help text is in doc comments (cli/mod.rs:26-99). Only the
  report text is apart from the code" — `.uss/audits/2026-11-25.yaml:175-177`.
  `docs/superpowers/follow-ups.md:232` lists the fix "Move the messages into a
  resource file."
- `uss release-check` "generates each report again" for each record —
  `standard/conventions.md:705-707`.
- The fact questions reach the developer as English comments in
  `templates/record.yaml` and as `Question` fields in
  `standard/applicability.md`.
- `tools/uss/build.rs` embeds `standard/`, `schema/`, `templates/`,
  `AGENTS.md` and `CHANGELOG.md` into the binary (`build.rs:13-17`).
- `uss` depends on clap, jsonschema, minijinja, regex, serde, serde-saphyr and
  serde_json; nothing for messages or locales — `tools/uss/Cargo.toml`.

External:

- POSIX.1-2024 sets the order `LC_ALL`, then `LC_MESSAGES` (the category
  variable), then `LANG` —
  [Open Group, chapter 8](https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/V1_chap08.html).
- RFC 4647 Lookup is meant for cases where one item must be chosen, "such as
  selecting an error message", with a default that the application defines —
  [RFC 4647](https://www.rfc-editor.org/rfc/rfc4647.html).
- Fluent selects variants by CLDR plural category and lets each language add
  variants (Polish `few` and `many`) without code changes —
  [projectfluent.org](https://projectfluent.org/); a Rust implementation,
  fluent-rs, is Apache-2.0 or MIT —
  [fluent-rs](https://github.com/projectfluent/fluent-rs).
- W3C: language tags should be as short as possible; region only when it
  distinguishes —
  [W3C, Choosing a Language Tag](https://www.w3.org/International/questions/qa-choosing-language-tags).

### Inferences

#### Recommended design

1. What stays in English, always:
   - rule IDs, fact IDs, stack identifiers, record and results keys, outcomes
     (`pass`, `fail`, ...);
   - the five keywords, in capitals;
   - the Verify text that the agent applies (the English text is normative;
     see part C);
   - commands, paths, code and command output quoted in evidence;
   - the English compliance statement (conventions section 8), which every
     report keeps.
2. What follows the developer:
   - The conversation. The agent asks the facts table and the rule batches in
     the developer's language, with fact IDs and rule IDs in English. No
     record field is needed: the conversation leaves nothing in the
     repository.
3. What follows the record (`project.language`, default `en`):
   - the evidence and the reasons in the results file, the draft Standard
     Deviations, and the report. One language per record keeps one results
     file readable by the next auditor, the next agent and CI.
4. `uss report` and `uss release-check` take the report language from
   `--lang`, else from `project.language`, else English. They never read it
   from the environment, so a report regenerated in CI matches the committed
   one.
5. Other `uss` messages (errors, `check` problems, help) take their language
   from `--lang`, else `LC_ALL`, `LC_MESSAGES`, `LANG` (POSIX order), else
   English. `uss show` prints English unless `--lang` is given.
6. A language that `uss` does not have falls back by RFC 4647 lookup (`pt-BR`,
   then `pt`), then to English. `uss` says so on stderr.
7. Dates stay `YYYY-MM-DD` in the record, the results and the report, in every
   language: one unambiguous form, and the report is a committed artifact.
   Score percentages use a per-language message (`{ $p }%` in English,
   `{ $p } %` in French), not a locale library; the counts are small whole
   numbers that need no grouping separator.
8. The report shows the statement in the report language. When that is not
   English, the English statement follows, marked as normative. Readers and
   tools that look for "Compliant with USS" still find it.
9. Implementation: move every literal into Fluent catalogues
   (`i18n/en/report.ftl`, `i18n/en/uss.ftl`), render the template with a
   `t()` function, and key category names by their registry ID (`safety`,
   ...). This also fixes USS-I18N-002 for `uss`. One template for all
   languages keeps the layout the same in each.

Alternatives, one line each:

- Everything in English, with the agent translating only the conversation:
  simplest, but developers cannot read their own evidence and report.
- The report language from `LANG`: rejected, because `release-check` in CI
  would write another report than the committed one.
- One template per language (`audit-report.de.md.j2`): rejected, because the
  layouts drift apart; one template with catalogues keeps them equal.
- A `language` key in each results file as well: possible, to record the
  language of old evidence; not needed, because the report language is chosen
  at render time.
- gettext PO in place of Fluent for `uss`: workable, but Fluent handles
  plurals and translator-controlled variants directly, and has a Rust
  implementation of its own.

Effort: S for the workflow, the schema key, `uss check` validation of the tag
and the conventions text; M for the Fluent move of all `uss` messages and the
report (it also closes USS-I18N-002); S for each further report language.

#### Draft: AGENTS.md, new subsection under "Use USS on a project" (after "Questions to the developer")

```markdown
### Language of the audit

1. Speak to the developer in the language that the developer uses.
2. Find the language of the record. It is `project.language` in the record.
   With no key, it is English.
3. Write the evidence, the reasons and the draft Standard Deviations in the
   language of the record.
4. Copy commands, paths, code and command output exactly. Do not translate
   them.
5. Do not translate rule IDs, fact IDs, record keys, outcomes or stack
   identifiers.
6. Write MUST, MUST NOT, SHOULD, SHOULD NOT and MAY in English capitals.
7. Apply the English text of each rule. A translation is informative.
8. To show rules in the language of the developer, run
   `uss applicable --full --lang <tag> .uss/record.yaml`.
9. If USS has no translation of a text, translate it yourself. Say that your
   translation is informative.
10. Write each date as `YYYY-MM-DD`.
```

Changes to existing AGENTS.md steps:

- "Questions to the developer", step 1, add: "Give each fact ID in English.
  Ask its question in the language of the developer."
- "Start the record", after step 2, add: "Ask the developer for the language
  of the evidence and the reports. Write its BCP 47 tag in
  `project.language`, such as `de` or `pt-BR`. Leave out the key for
  English."
- "With no person present", add: "Write the evidence in the language of the
  record."

#### Draft: record schema and template

`schema/record.schema.json`, under `project.properties` (the pattern is the
"keep it simple" subset: language, optional script, optional region):

```json
"language": {
  "type": "string",
  "pattern": "^[a-z]{2,3}(-[A-Z][a-z]{3})?(-([A-Z]{2}|[0-9]{3}))?$"
}
```

`templates/record.yaml`, under `project`:

```yaml
  # The language of the evidence, the reasons and the reports, as a BCP 47
  # tag, such as en, de or pt-BR. Leave it out for English.
  language: en
```

`standard/conventions.md`, section 11, new row of the record table:

```markdown
| `project.language` | when needed | The language of the evidence, the reasons and the report, as a BCP 47 tag (section 14). The default is `en`. |
```

`standard/conventions.md`, section 8, after the statement examples:

```markdown
When the language of the report is not English, the report shows the
statement in that language, then the English statement. The English statement
is normative.
```

#### Draft: CLI documentation (README and `uss --help`)

```markdown
- `uss report --lang <tag> <results>` writes the report in the language
  `<tag>`, in place of the language of the record.
- `uss release-check --lang <tag>` does the same for each report.
- Without `--lang`, `uss report` uses `project.language` in the record, then
  English. Other messages use `LC_ALL`, `LC_MESSAGES` and `LANG`, then
  English.
- If USS has no catalogue for `<tag>`, `uss` removes subtags from its end
  until one matches, then uses English. It says so on stderr.
```

#### Draft: report catalogue (illustration of the plural handling that `score.rs:285-312` now does in code)

```ftl
# i18n/en/report.ftl
statement-compliant = Compliant with USS { $version } as of { $date }.
statement-not-compliant = Not compliant with USS { $version } as of { $date }: { $parts }.
part-must-failures =
    { $n ->
        [one] { $n } MUST failure
       *[other] { $n } MUST failures
    }
score-percent = { $p }%
score-none = n/a
section-fail = Fail
category-safety = Safety

# i18n/de/report.ftl (keyword stays in English capitals)
part-must-failures =
    { $n ->
        [one] { $n } MUST-Verstoß
       *[other] { $n } MUST-Verstöße
    }
```

### Gaps

- Whether clap's derive API can take run-time (translated) help strings for
  every command was not checked; it decides how much of `cli/mod.rs:26-99`
  must move to the builder API.
- fluent-bundle wraps placeables in Unicode isolation marks by default (from
  memory, not verified in this session). In a Markdown report they are
  invisible but present; decide whether to turn them off for left-to-right
  languages.
- The CLDR percent patterns per language (such as the narrow no-break space
  in French) were not read; translators would copy them from CLDR.
- No source was found on how other audit tools localise their reports; the
  design rests on the repository's constraints (deterministic
  `release-check`) rather than on precedent.

## (C) Translations of USS itself

### Takeaway

Comparable standards agree that one language is normative and translations
are best-effort community work; they differ in keyword handling and in how
they show lag. For USS: English is the only normative text (W3C model), each
translation pins the same USS version and falls back to English paragraph by
paragraph (po4a), keywords stay in English capitals in parentheses after the
translated verb (the German and Japanese SemVer pattern, which `uss lint` can
check), and `uss show --lang` serves translations with a status notice. A
translation is complete for a version when every unit is translated, a second
person approved each unit, and the translation lint passes.

### Cited Findings

How comparable standards publish translations:

- W3C: "In all cases and in case of dispute, the authoritative version remains
  the English version"; each authorized translation carries a disclaimer at
  the top in the target language, naming the English original; "translations
  created through the volunteer translation process do not have an official
  status"; authorized translations go through a Lead Translation Organization,
  a Candidate Authorized Translation, a review of at least 30 days and W3C
  approval, with an errata page; "Any translation of a new version of a W3C
  document should go through the same process"; the disclaimer has "This
  version", "Most recent version" and "Original version" links —
  [W3C Authorized Translations policy](https://www.w3.org/2005/02/TranslationPolicy)
  (no date on the page; copyright 2026).
- ISO: English, French and Russian are the official languages; ISO Guide 47
  says the three official versions are equally valid in a dispute;
  translations into other languages are the responsibility of member bodies
  and are labelled as translations; Spanish-speaking members coordinate one
  Spanish version (STTF groups) — from search-result summaries of
  [ISO Guide 47:1986 (sample)](https://cdn.standards.iteh.ai/samples/19753/a23a5e8403bd4d87bbe0456505ce6f5c/ISO-Guide-47-1986.pdf)
  and [UNE, ISO en español](https://www.en.une.org/la-asociacion/sala-de-informacion-une/noticias/iso-en-espanol).
  The guide is from 1986 and was not read in full; the current ISO/IEC
  Directives may differ.
- IETF: RFC 5377 states that the IETF wishes "to continue to permit anyone to
  make full copies and translations of RFCs"; the Trust Legal Provisions grant
  no licence to modify documents "other than to make translations" or to use
  Code Components — [RFC 5377](https://www.rfc-editor.org/rfc/inline-errata/rfc5377.html);
  [IETF Trust Legal Provisions 5](https://trustee.ietf.org/documents/trust-legal-provisions/tlp-5)
  (from search-result summaries; the full clause was not read).
- OWASP ASVS: the English 5.0.0 release (May 2025) is the reference; the
  README calls translations "a best effort" and says "there is only so much we
  can do to ensure the translations are correct"; 5.0.0 translations listed
  include Turkish, French, Korean and Russian; licence CC BY-SA 4.0 — from a
  search-result summary of [github.com/OWASP/ASVS](https://github.com/OWASP/ASVS)
  (README not fetched directly).
- OWASP Top 10:2025 publishes translations as paths of the same release
  (`/Top10/2025/es/`, `/it/`, `/pt-BR/`, `/fa/`) —
  [OWASP Top 10:2025 (es)](https://owasp.org/Top10/2025/es/) (search
  result). The Top 10 for LLM project uses a progress spreadsheet, a
  translation lead, and a branch and folder per language, in Markdown —
  search-result summary; the project wiki page was not read.
- Contributor Covenant: "carefully translated into over 40 languages by
  volunteers"; 41 translations across four versions (3.0: 10, 2.1: 15, 2.0: 3,
  1.4: 13), so most lag the latest version; translators follow "Instructions
  for translators" on GitHub; no official or review status is stated —
  [Contributor Covenant translations](https://www.contributor-covenant.org/translations/).
- Semantic Versioning: translations of 2.0.0 with three keyword styles.
  Spanish translates the keywords (`DEBE`, `NO DEBE`, `DEBERÍA`, `PUEDE`) and
  cites "RFC 2119-es" — [semver.org/lang/es](https://semver.org/lang/es/).
  German keeps the English keyword in parentheses: "muss (MUST) eine
  öffentliche API definieren" — [semver.org/lang/de](https://semver.org/lang/de/).
  Japanese does the same: "修正してはなりません（MUST NOT）" —
  [semver.org/lang/ja](https://semver.org/lang/ja/). None of the three pages
  says that English is authoritative.
- Keep a Changelog: one page per language and version; an old translation
  shows "There is a newer version available: Español 1.1.0"; the language
  selector marks a language that lacks the current version with "(n/a)"; 28
  languages, versions 1.1, 1.0 and 0.3 —
  [keepachangelog.com/es-ES/0.3.0](https://keepachangelog.com/es-ES/0.3.0/).

Tooling:

- po4a "extracts the translatable material from the original document, and
  places it into a PO file"; it then "re-injects the translation into the
  structure of the original document"; "If the translation is only partial,
  the original (English) text is used instead for the missing parts";
  Markdown is an option of its text format — [po4a.org](https://po4a.org/).
  The page did not state a default minimum-translation threshold.
- Weblate supports Markdown since version 5.0; each occurrence of a string is
  a unit; YAML front matter is one block by default; "Translation changes made
  in the translated files are not imported back into Weblate", so Weblate is
  the source of truth —
  [Weblate docs, Markdown](https://docs.weblate.org/en/latest/formats/markdown.html).
- Crowdin is free for open-source projects that apply (criteria include
  three months of activity and an active community); the open-source licence
  requires contributing translations to Crowdin's Global Translation Memory;
  Crowdin has a Markdown format with options to exclude code blocks and front
  matter — search-result summaries of
  [Crowdin for open source](https://crowdin.com/product/for-open-source) and
  [Crowdin store, Markdown](https://store.crowdin.com/md) (not fetched).
- Fluent and fluent-rs (including `fluent-pseudo`) as in part A —
  [projectfluent.org](https://projectfluent.org/),
  [fluent-rs](https://github.com/projectfluent/fluent-rs).

Repository:

- Keywords count only in capitals and as whole words; `uss lint` scans each
  rule body for the rule's own keyword and rejects any other keyword or a
  synonym — `standard/conventions.md:8-20`, `:149-170`.
- A PATCH release is "wording, references or verification text with no
  normative change" — `standard/conventions.md:562-563`.
- Each `uss` release carries one version of the standard, embedded at build
  time — `standard/conventions.md:573-575`; `tools/uss/build.rs:1-17`.
- Vendor-maintained profiles are owned through CODEOWNERS, and USS
  maintainers review them for accuracy — `standard/conventions.md:719-725`,
  `:786-789`. The same model can serve translations.
- USS is under the MIT licence (`LICENSE`, per the self-audit evidence for
  USS-LIC-001, `.uss/audits/2026-11-25.yaml:181-183`).

### Inferences

#### Recommended design

1. Normative status. English is the only normative text, as at W3C. Each
   translated file starts with a notice in its own language: the USS version,
   the share translated, and that the English text decides in a dispute.
   Unlike ISO, USS has no second official language to keep equal.
2. Keywords. A translation writes the translated verb, then the English
   keyword in capitals in parentheses: `muss (MUST)`, `しなければならない
   (MUST)`. This is the German and Japanese SemVer pattern. `uss lint` can
   then check each translated rule: the same ID, the same fields (Level,
   Applies when, Category, References copied unchanged), and the same keyword
   in the body and no other. The Spanish pattern (translated keywords only)
   cannot be checked by a program and needs a translated RFC 2119.
3. Unit and tooling. One PO file per English Markdown file, made by po4a,
   kept in the repository under `i18n/<tag>/po/`. Any PO editor, or Weblate,
   works on it. `msgmerge` marks a changed English paragraph as fuzzy, and
   po4a shows English for each fuzzy or missing paragraph, so a translation
   never shows text from another version. The generated Markdown is
   committed under `i18n/<tag>/text/`, and CI fails when it is out of date
   (the same pattern as a lockfile). The messages of `uss` and the report use
   Fluent (`i18n/<tag>/uss.ftl`, `report.ftl`), as in part B.
4. Versioning. A translation is built from the English text of the same
   commit, so it always pins the same USS version. A lag shows as a lower
   share, not as an old version. Translation updates are not normative
   changes, so they can ship in a PATCH release. A release never waits for a
   translation. `CHANGELOG.md` lists the share per language for each release.
5. Serving. `build.rs` embeds `i18n/`. `uss show --lang <tag> <path>` prints
   the translated file with its notice. `uss show --lang <tag>` lists the
   translated files and their shares. `uss applicable --full --lang <tag>`
   prints translated rule text. Missing languages fall back by RFC 4647
   lookup, then to English, with a note on stderr.
6. What to translate first, by who reads it: (1) the report catalogue and the
   `uss` messages; (2) the `Question` and `Definition` of each fact; (3) rule
   titles; (4) `templates/standard-deviation.md`, the comments of
   `templates/record.yaml`, and `templates/AGENTS.snippet.md`; (5) rule
   bodies, Verify and rationale, one area at a time; (6) `README.md` and
   `standard/conventions.md`. Do not translate `AGENTS.md` at first: agents
   apply the English workflows and talk to the developer in their language.
   The ASD-STE100 style of the workflows keeps a machine translation reliable
   for a person who wants one.
7. Review. Each language has a maintainer and at least one reviewer, named in
   CODEOWNERS for `i18n/<tag>/`. A unit counts when a second person who reads
   the language well approves it. Machine translation can make a first draft;
   a person approves each unit before it counts. USS maintainers review only
   the structure, through the lint.
8. Complete. A translation is complete for a version when every unit is
   translated and approved, and the translation lint passes. Otherwise the
   README lists it as partial, with its share.

Alternatives, one line each:

- Full-file copies per version (SemVer, Keep a Changelog, Contributor
  Covenant): easy to start, but whole files go stale with no paragraph-level
  tracking, as the 41 Contributor Covenant translations across four versions
  show.
- Weblate's own Markdown support: no po4a step, but Weblate becomes the source
  of truth and edits in the repository are not imported back.
- Crowdin: free for open source and supports Markdown, but proprietary and it
  requires contributing translations to its Global Translation Memory.
- Authorized translations in the W3C way (lead organisation, 30-day review):
  too heavy for a young project with few translators.
- Translated keywords with a translated RFC 2119 (Spanish SemVer): natural to
  read, but no program can check keyword parity.
- On-the-fly machine translation by the agent only: no reviewed text, and two
  agents give two translations.

Effort: L. Conventions section 14, the `i18n/` layout, po4a configuration,
the translation lint, `--lang` for `show` and `applicable`, embedding, a
governance page and one pilot language.

#### Draft: `standard/conventions.md`, new section

```markdown
## 14. Languages and translations

The English text of the standard is the only normative text. A translation
is informative. In a dispute, the English text of the same version decides.

- A translation pins the USS version of its English source. It MUST say at
  its top that the English text is normative.
- A translation MUST keep rule IDs, fact IDs, field names, record keys,
  outcomes and stack identifiers in English.
- A translated rule MUST keep its keyword in English capitals, in
  parentheses after the translated verb, such as `muss (MUST)`.
- A translated rule MUST have the same Level, Applies when, Category and
  References as its English rule.
- A translation that lacks a paragraph shows the English paragraph. It states
  the share of the text that it translates.
- Records, results files and reports write dates as `YYYY-MM-DD` in every
  language.

`uss lint` checks each translation against its English file: the same rule
IDs, the same fields, and the same keyword in each rule body, and no other.

A translation is complete for a version when every paragraph and message is
translated, a second person who reads the language approved each one, and
`uss lint` passes.
```

#### Draft: `uss show --lang` documentation

```markdown
- `uss show --lang <tag> <path>` prints the translation of `<path>` into the
  language `<tag>`. It starts with a notice: the USS version, the share of
  the file that is translated, and that the English text is normative.
  Paragraphs with no translation are in English.
- `uss show --lang <tag>` lists the translated files and their shares.
- If USS has no translation into `<tag>`, `uss` removes subtags from its end
  until one matches. With no match, it prints the English file and says so on
  stderr.
```

#### Draft: governance page (`docs/translations.md` or a CONTRIBUTING section)

```markdown
## Translate USS

1. Open an issue that names the language and its BCP 47 tag.
2. Find a second person who reads the language well. That person reviews.
3. Add both people to CODEOWNERS for `i18n/<tag>/`.
4. Translate the units in the order of `i18n/PRIORITY.md`.
5. Keep each keyword in English capitals, in parentheses after the verb.
6. Do not translate rule IDs, fact IDs, record keys, outcomes or code.
7. Run `cargo xtask check`. Correct each problem that it reports.
8. Ask the reviewer to approve each unit before the merge.
```

### Gaps

- The exact OWASP ASVS README wording and its list of 5.0.0 translations come
  from a search-result summary, not a direct read.
- ISO's current policy (ISO/IEC Directives) was not read; ISO Guide 47 dates
  from 1986.
- Whether Weblate (and Crowdin) read Fluent `.ftl` files, and the terms of
  Weblate's gratis hosting for libre projects, were not checked.
- po4a's default threshold for writing a partial translation (from memory,
  80% with `--keep`) was not confirmed on its site; set it to 0 so that
  partial files are always built, or confirm the default.
- The licence of contributed translations (MIT, like the rest of USS) and
  whether contributors sign anything were not researched.
- Contributor Covenant's "Instructions for translators" page was not read, so
  its review rules are unknown.

## Order of work: before 0.1.0 and later

### Takeaway

Before 0.1.0, do only what would later need a retired rule or a record-format
change, plus the cheap workflow text: the I18N-006 change and its fact, the
`project.language` key and the "Language of the audit" steps, the English-is-
normative statement, and a decision on the USS-I18N-002 self-audit failure.
Everything else is MINOR or PATCH work after 0.1.0.

### Cited Findings

- A change to the meaning of a rule must retire the rule and add a new ID —
  `standard/conventions.md:183-184`. The maintainer brief states that 0.1.0
  is not released, so rules can still change in place.
- New rules, areas, modules or facts are MINOR; until 1.0.0, a MINOR release
  can also change the record format — `standard/conventions.md:560-567`.
- The self-audit fails USS-I18N-002 (a SHOULD rule), which keeps USS "not
  compliant" unless it is fixed or has a Standard Deviation —
  `.uss/audits/2026-11-25.yaml:175-177`; `standard/conventions.md:450-456`.

### Inferences

Before 0.1.0:

1. Change USS-I18N-006 (Enables, `has-cli`) and add the fact
   `offers-multiple-languages`. S.
2. Add `project.language` to the schema, the template, `uss check` (a
   well-formed tag) and conventions section 11. S.
3. Add "Language of the audit" to `AGENTS.md`, and the three small changes to
   existing steps. S.
4. Add the first paragraph of conventions section 14 ("English is the only
   normative text"), so that early translators know the rule. S.
5. Decide on USS-I18N-002 for `uss`: move the messages to `i18n/en/*.ftl`
   with fluent-bundle (M, and it prepares parts B and C), or record a
   Standard Deviation with that remediation plan (S). The move is the better
   choice if the release can wait for it.
6. Optionally widen USS-I18N-003 to input, while that needs no retired rule.
   S.

After 0.1.0:

7. 0.2.0 (MINOR): rules I18N-007 to I18N-019, each with its references read
   and pinned. M.
8. `uss report --lang`, `release-check --lang`, the English statement line,
   and one pilot report language. M.
9. The translation pipeline: po4a, `i18n/` layout, the translation lint,
   `uss show --lang`, `uss applicable --full --lang`, the governance page, and
   a pilot language for the report, the facts and the rule titles. L.
10. Further languages and the rule bodies, one area at a time, shipped in
    PATCH releases. S per area per language.

### Gaps

- The release date of 0.1.0 was not known, so the choice in step 5 (fix or
  deviation) is left to the maintainer.
- Whether the plugin skills in `skills/adopt` and `skills/audit` repeat
  workflow text that would also need the language steps was not checked.
