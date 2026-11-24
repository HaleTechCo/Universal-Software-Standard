# Internationalization

- **Prefix:** I18N
- **Kind:** area

This area covers how the software handles text, numbers and points in time
for people in different languages and places. Its rules count toward
Accessibility & Usability. The timestamps of audit records belong to the Audit
logging area (USS-AUD-005), and the contract of an API to the API area
(USS-API-001). Reading a UI with assistive technology belongs to the
Accessibility (ACC) area.

### USS-I18N-001: Use Unicode throughout

- **Level:** MUST
- **Applies when:** has-ui or has-cli or stores-persistent-data or uses-network
- **Category:** Accessibility & Usability
- **Verify:** Find where text enters the software (the UI, the command line,
  files, network input), where it is stored (databases, files), and where it
  leaves (network output, files, the command line). The code handles text as
  Unicode strings, and nothing converts it through a legacy code page. Each
  store and each exchange of text uses UTF-8: the database or column character
  set, the encoding that file writers name, and the charset of each network
  format (such as the `charset` of an HTTP content type, or JSON). Then, in a
  test instance, pass the string `Zoë 漢字 😀` (it holds an accented letter, a
  non-Latin script and a character outside the Basic Multilingual Plane)
  through each entry point to each store and out again: it comes back
  identical. A store or exchange that uses another encoding, or a string that
  changes, fails.
- **References:** The Unicode Standard, Version 18.0.0 (16 September 2026);
  RFC 3629 (November 2003)

Text MUST be handled as Unicode, and encoded as UTF-8 where it is stored or
exchanged.

**Rationale:** A name, a message or a file path in any script has to survive
the software unharmed. Code pages lose characters silently, and a mix of
encodings turns text into garbage at the places where they meet. UTF-8 is the
one encoding that every system reads.

### USS-I18N-002: Keep user-facing text out of code

- **Level:** SHOULD
- **Applies when:** has-ui or has-cli
- **Category:** Accessibility & Usability
- **Verify:** Find the resource files or message catalogues that hold the
  text for users (such as gettext files, ICU message files, `.resx` files, or
  string tables). Search the code for text that a user sees: labels, buttons,
  messages, errors, help text, and the strings that the command line prints.
  Each is read from the resource files by a key, and not written in the code
  as a literal (apart from the text of logs and developer-only messages, which
  users do not see). A user-facing literal in the code fails.
- **References:** W3C Internationalization, Localization vs.
  Internationalization (read 2026-11-24)

Text that users see SHOULD be kept in resource files, apart from the code, so
that it can be translated.

**Rationale:** Text in the code can be translated only by changing the code,
and only by a programmer. Resource files let a translator work alone, and they
show at once which strings still lack a translation.

### USS-I18N-003: Format for the user's locale

- **Level:** SHOULD
- **Applies when:** has-ui
- **Category:** Accessibility & Usability
- **Verify:** Find each place where the UI shows a number, a date, a time or a
  currency amount. Each is formatted by the platform's or a library's locale
  formatting (which takes the user's locale, such as from the system language
  or a user setting), and not by a format string that the code fixes. Then run
  the UI with two locales whose formats differ (such as `en-US` and `de-DE`)
  and compare a screen that shows each kind of value: the group and decimal
  separators, the order of the date, and the currency position change with the
  locale. A value that is the same in both, or a hand-built format, fails.
- **References:** Unicode CLDR, Version 49 (22 October 2026); W3C
  Internationalization, Localization vs. Internationalization (read
  2026-11-24)

Numbers, dates, times and currencies SHOULD be shown in the format of the
user's locale.

**Rationale:** `1.234` is one thousand two hundred thirty-four in one country
and a little more than one in another, and `03/04` is two different days. A
value in the reader's own format is read without a second thought, and without
a wrong one.

### USS-I18N-004: Store timestamps in UTC

- **Level:** MUST
- **Applies when:** stores-persistent-data
- **Category:** Accessibility & Usability
- **Verify:** List the fields and columns that store a point in time (a
  timestamp, a date and time, an epoch value), leaving out the timestamps of
  audit records, which USS-AUD-005 covers. Run the software with the host time
  zone set to one that is not UTC (such as `America/New_York`), write a record
  of each kind, and read the stored value as it is, not through the software:
  it is the UTC instant (for example, a value with `Z`, an epoch value, or a
  timestamp type that the database keeps in UTC). For each field that holds a
  future local time (such as a meeting at 09:00 in a city), the record also
  stores the time zone, as a zone identifier (such as `Europe/Paris`). A
  stored time in the host's local time, or a future local time with no zone,
  fails.
- **References:** W3C Working with Time and Timezones (Group Note Draft, 20
  November 2026), 4.1 Timestamps and 4.2.2 Past and Future Events

Points in time MUST be stored in UTC, except in audit records, whose timestamps
USS-AUD-005 holds; a future local time also stores its time zone.

**Rationale:** A time stored in a local zone is wrong when the server moves,
when daylight-saving time starts, or when a user in another zone reads it. A
future local time cannot be turned into a UTC instant for good, because the
rules of the zone can change before it comes; so the zone is kept with it.

### USS-I18N-005: Exchange timestamps in RFC 3339 format

- **Level:** MUST
- **Applies when:** exposes-network-api
- **Category:** Accessibility & Usability
- **Verify:** List the fields of the API's requests and responses that hold a
  point in time (from the code, and from the API's contract where it has one).
  Take a sample of responses from a test instance that holds each field: each
  value parses as an RFC 3339 `date-time` (a date, `T`, a time and an offset
  of `Z` or `+hh:mm` or `-hh:mm`) with a validator or a parser of that format.
  Send each request field a value in that format, and one that is not (such as
  `03/04/2026` or an epoch number): the first is accepted, and the second is
  rejected. A field that sends or accepts another format fails.
- **References:** RFC 3339 (July 2002)

Points in time that the API sends or accepts MUST use the RFC 3339 format.

**Rationale:** A time with no offset, or in a local format, is read in the
reader's own zone, and different clients read it differently. RFC 3339 names
the instant exactly, and every common language reads it.

### USS-I18N-006: Offer more than one language

- **Level:** MAY
- **Applies when:** has-ui
- **Category:** Accessibility & Usability
- **Verify:** Adopting this rule means that the UI is available in at least two
  languages. The project lists its languages (in the documentation, or as the
  language files that it ships), and the user can use each one (by the system
  language or by a setting). For each listed language, open the screens of the
  main workflows (those that the user documentation describes): their text is
  in that language. A listed language that shows the text of another language
  on those screens fails, and so does a project that adopts the rule and lists
  fewer than two languages.
- **References:** W3C Internationalization, Localization vs.
  Internationalization (read 2026-11-24)

The user interface MAY be available in more than one language.

**Rationale:** Most people use software best in the language they think in. The
rule on keeping text out of code (USS-I18N-002) is what makes a second
language a matter of a new file and not of a new version.
