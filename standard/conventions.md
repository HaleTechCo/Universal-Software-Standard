# USS conventions

This document is normative. It states how the rules of the Universal Software
Standard (USS) are written, how applicability and compliance work, and how a
project records its decisions. `uss lint` checks the files of the standard
against it, and `uss check` checks a project's record against it.

## 1. Keywords

The key words "MUST", "MUST NOT", "SHOULD", "SHOULD NOT" and "MAY" in the
standard, this document included, are to be interpreted as described in
BCP 14 (RFC 2119 and RFC 8174) when, and only when, they appear in all
capitals, as shown here. Written any other way, these words have their
ordinary English meaning.

- These five are the only keywords. The synonyms that RFC 2119 also defines
  (`SHALL`, `SHALL NOT`, `REQUIRED`, `RECOMMENDED`, `NOT RECOMMENDED` and
  `OPTIONAL`) MUST NOT be used, so an audit never has to interpret them.
- Each rule MUST have exactly one level, which is one of the five keywords.
  One rule states one requirement at one level.

## 2. Standard files

The standard is made of these files:

| File | Content |
|---|---|
| `standard/conventions.md` | This document. |
| `standard/registry.yaml` | The version of the standard (section 10), the score categories (section 6) and the stack identifiers (section 5). |
| `standard/applicability.md` | The applicability facts (section 5). |
| `standard/core/<area>.md` | Area files. Each one holds the core rules of one area, such as security. |
| `standard/modules/<group>/<module>.md` | Module files. Each one holds the rules for one technology, such as a language or a tool, in a group directory such as `languages`. |

Every `.md` file under `standard/core/` and `standard/modules/` is a rule
file. A rule file has this layout:

1. A title line that starts with `# `, such as `# Security`.
2. The header: a field block straight after the title.
3. Free text, if any.
4. The rules, each under a `###` heading (section 3). A `##` heading only
   groups the rules after it; it has no other meaning.

The header has these fields:

| Field | Present | Value |
|---|---|---|
| Prefix | always | The prefix of every rule ID in the file: an uppercase letter, then 1 to 7 uppercase letters or digits (`^[A-Z][A-Z0-9]{1,7}$`), such as `SEC` or `GO`. |
| Kind | always | `area` in a file under `standard/core/`; `module` in a file under `standard/modules/`. |
| Applies when | when needed | A condition for every rule in the file (section 5). The default is `always`. |

The header MUST NOT have any field that is not in this table.

The `Prefix` header registers the prefix. There is no other list of
prefixes. A prefix MUST be unique across all rule files.

### Fields

Headers, rules and facts use the same field syntax:

- A field is a line of the form `- **Name:** value`. Field names are
  case-sensitive.
- A field block starts at the first line that is not blank after its heading.
  It ends at the first line that is neither a field nor a continuation line.
- A continuation line starts with two spaces. Its text is added to the value
  of the field above it, after one space.
- A field MUST NOT appear twice in one block.
- In a field that holds a list (`Enables`, `Refines`, `Implies`), commas
  separate the items.

Lines in a fenced code block (one that starts with ```` ``` ```` or `~~~`)
are never read as headings or fields. A code fence MUST be closed.

Files are UTF-8 with LF line endings. The tools also read CRLF line endings
and a leading UTF-8 byte order mark.

### Checking the standard

`uss lint` loads every file of the standard and reports each problem it
finds, with the file and line. When it finds none, it prints
`uss lint: ok (<N> rules, <M> files)`, where `<M>` counts the rule files.

1. Go to the root of your USS checkout.
2. Run `cargo run --locked -p uss -- lint`.
3. Correct each problem that it reports.
4. Run the command again until it prints `uss lint: ok`.

The option `--standard-root <dir>` makes any `uss` command use the USS
checkout at `<dir>` instead of the copy of the standard built into `uss`.

## 3. Rule format

Each rule is a `###` heading, a field block and a body. The body holds the
normative text, then the rationale where the reason is not self-evident:

```markdown
### USS-SEC-013: Use parameterized queries

- **Level:** MUST
- **Applies when:** uses-database
- **Category:** Safety
- **Verify:** No query is built by concatenating external input; all queries
  use bound parameters, or a query builder that binds them.
- **References:** OWASP ASVS 5.0 (injection prevention); CWE-89

Every database query that includes externally supplied values MUST pass those
values as bound parameters.

**Rationale:** The database reads a bound parameter only as a value, never as
SQL, so external input cannot change what a query does.
```

### Heading

The heading is `### USS-<PREFIX>-<NNN>: <title>`: the rule ID (section 4), a
colon, a space and a short title. In a rule file, every `###` heading MUST
have this form, and the prefix in its ID MUST be the file's `Prefix`.

### Fields

| Field | Present | Value |
|---|---|---|
| Level | always | One of the five keywords (section 1), in capitals. |
| Applies when | always | `always`, or an applicability expression (section 5). A module rule has it too. |
| Category | always | The name of one score category (section 6), exactly as `registry.yaml` writes it. |
| Verify | always | How an auditor determines pass or fail from the project's artifacts. |
| References | always | The external standards, CWEs and specifications that the rule relies on, with their versions, or `none`. |
| Enables | MAY rules only, when needed | The facts that become true when a project adopts the rule (section 5). |
| Refines | module rules only, when needed | The IDs of the core rules that this rule makes concrete for its technology. Each one MUST be a rule in an area file. |
| Status | retired rules | `Retired` (section 4). A rule without `Status` is active. |
| Retired in | retired rules | The version of the standard that retired the rule. |
| Replaced by | retired rules | The ID of the rule that replaces it, or `none`. |

An active rule MUST have the first five fields. A rule MUST NOT have any
field that is not in this table.

### Body

The body is the text after the field block, up to the next `##` or `###`
heading. An active rule MUST have a body, and the body MUST state the
requirement with the rule's keyword. The body can hold fenced code blocks as
examples. A paragraph that starts with `**Rationale:**` follows the normative
text and gives the reason for the rule.

### Keyword scan

`uss lint` scans the prose of the body of each active rule:

- The prose is the body without its fenced code blocks. Inline code and the
  `**Rationale:**` paragraph are part of it. Fields are never scanned.
- A run of whitespace, line breaks included, reads as one space. So `MUST` at
  the end of a line and `NOT` at the start of the next line read as
  `MUST NOT`.
- Only whole words in capitals count (RFC 8174). `must` and `Must` are
  ordinary words.

Each of these is an error:

- one of the synonyms that section 1 lists;
- any of the five keywords other than the rule's own level, so a MUST rule
  cannot contain `MUST NOT`, and a MUST NOT rule cannot contain `MUST` without
  `NOT` after it;
- a body that does not contain the rule's own keyword.

To show a different keyword in a rule, for example in a quoted configuration
file, put it in a fenced code block.

## 4. Rule IDs and retiring rules

A rule ID has the form `USS-<PREFIX>-<NNN>`: `USS`, the prefix of the rule's
file (section 2) and three digits, such as `USS-SEC-012`. A module rule uses
the prefix of its module, such as `USS-GO-004`, `USS-PY-010` or
`USS-MVN-002`.

- A rule ID MUST be unique in the standard.
- A rule ID MUST NOT be reused or renumbered.
- A rule MUST NOT be deleted. A rule that is no longer wanted is retired in
  place, so links to it and old records stay valid.
- A change to the meaning of a rule MUST be made by retiring the rule and
  adding a new rule with a new ID.

To retire a rule:

1. Keep its heading, with its ID and title.
2. Add `- **Status:** Retired` to its field block.
3. Add `- **Retired in:** <version>`. Use the version of the standard that
   retires the rule.
4. Add `- **Replaced by:** <rule ID>`. Use the ID of the rule that replaces
   it, or write `none`.

```markdown
### USS-EXA-005: Old rule title

- **Status:** Retired
- **Retired in:** 0.2.0
- **Replaced by:** USS-EXA-006
```

`Retired in` MUST be a semantic version, and a `Replaced by` rule MUST exist.
`uss lint` does not check the other fields or the body of a retired rule, so
they can stay as a record of the old text. A retired rule never applies.

## 5. Applicability

A rule applies to a project when its `Applies when` expression is true for
the project's facts.

### Facts

A fact is a yes-or-no statement about a project, such as `has-web-ui`. Each
fact is defined once, in `standard/applicability.md`, under a
`### <fact-id>` heading. A fact ID is made of lowercase letters and digits, in
words joined by hyphens (`^[a-z0-9]+(-[a-z0-9]+)*$`).

| Field | Present | Value |
|---|---|---|
| Question | asked facts | The question that sets the fact. An agent answers it from the code, then confirms the answer with the developer. |
| Enabled by | facts that a rule sets | The ID of the MAY rule that sets the fact. |
| Definition | always | When the answer is yes. |
| Implies | when needed | Other facts that are also true when this fact is true. |

- A fact MUST have exactly one of `Question` and `Enabled by`.
- A fact MUST NOT have any field that is not in this table.
- Each fact that `Implies` names MUST exist, and implications MUST NOT form a
  cycle.
- When a fact is true, every fact that it implies is true too, whatever the
  record answers for that fact.

### Stack identifiers

`standard/registry.yaml` lists the known stack identifiers by kind: `lang`,
`framework`, `platform` and `tool`. A project lists its stack in its record,
and each entry sets a stack fact:

| Record list | Stack fact |
|---|---|
| `stack.languages` | `lang.<id>`, such as `lang.go` |
| `stack.frameworks` | `framework.<id>`, such as `framework.react` |
| `stack.platforms` | `platform.<id>`, such as `platform.postgresql` |
| `stack.tools` | `tool.<id>`, such as `tool.maven` |

An expression can use only the stack identifiers that `registry.yaml` lists,
so a stack entry that the registry does not list matches no rule. The audit
report lists each stack entry that no module covers, which is each one that no
module file's `Applies when` header names: the audit applied the core rules to
it directly.

### Expressions

An `Applies when` value follows this grammar:

```text
expr  = and { "or" and }
and   = unary { "and" unary }
unary = "not" unary | atom
atom  = "always" | ident | "(" expr ")"
ident = fact | kind "." fact
kind  = "lang" | "framework" | "platform" | "tool"
fact  = word { "-" word }
word  = one or more lowercase ASCII letters or digits
```

- `not` binds most tightly, then `and`, then `or`. So `a or b and c` means
  `a or (b and c)`. Use parentheses to group in another way.
- Whitespace separates the words. A parenthesis needs no space around it.
- The words `always`, `not`, `and` and `or` are lowercase only.
- Each identifier MUST be a fact in `applicability.md` or a stack identifier
  that `registry.yaml` lists.
- A fact that is not true is false.

Examples: `always`, `uses-database`, `has-web-ui or exposes-network-api`,
`lang.python and not is-library`.

### File conditions

The `Applies when` header of a rule file (section 2) is ANDed with the
`Applies when` of each rule in the file. A module file uses it to name its
technology, such as `lang.python`, so that its rules apply only to projects
that use that technology. In that file, a rule with `Applies when: is-library`
applies when `lang.python and is-library` is true.

### Enables

A MAY rule can switch facts on when a project adopts it. Its `Enables` field
names those facts, and the `Enabled by` field of each of those facts names the
rule. For example, a MAY rule to provide a plugin interface can enable
`has-plugin-interface`, which makes the plugin-interface rules apply.

- Only a MAY rule can have `Enables`.
- The two sides MUST agree: each fact in `Enables` names the rule in its
  `Enabled by`, and the rule in each `Enabled by` is a MAY rule that lists the
  fact in its `Enables`.

### Resolving facts

The true facts of a project are found in this order:

1. The facts that the record answers `true`.
2. The stack facts from the record's stack lists.
3. Then, until nothing changes: each fact that a true fact implies, and each
   fact that an applicable MAY rule enables when the record adopts that rule
   (`adopt: true`).

## 6. Score categories

Each rule counts toward exactly one score category, which its `Category` field
names. `standard/registry.yaml` lists the categories, each with an ID and a
name, in this order. Reports show them in this order.

| Category | ID | Measures |
|---|---|---|
| Safety | `safety` | Security, memory and runtime safety, supply chain, dependencies, hardening, privacy, accountability, safe AI use |
| Reliability | `reliability` | Keeps working, fails safely, recovers, is tested, can be restored, is operated well |
| Efficiency | `efficiency` | Fast, lean, and sparing with energy and resources |
| Maintainability | `maintainability` | Easy to change correctly |
| Contributability | `contributability` | Easy for someone new (human or agent) to start contributing, and the project outlives any one person |
| Compatibility | `compatibility` | Runs where users need it, on their own terms; upgrades and integrates cleanly |
| Fairness | `fairness` | Keeps its promises to users; honest, flexible, non-predatory commercial terms |
| Accessibility & Usability | `accessibility-usability` | Usable by everyone, in any language, without confusion |
| Features | `features` | Optional capabilities that make software complete |

- The `Category` field MUST hold the name of a category, not its ID, such as
  `Accessibility & Usability`.
- The rules of one area normally share one category. A rule MAY name another
  category when its effect belongs there; for example, a rule on signed
  release artifacts counts toward Safety.

## 7. Writing rules

### The bar for every rule

- A rule MUST state one requirement.
- A rule MUST be verifiable from the project's artifacts.
- A core rule MUST be language-neutral. A rule for one technology goes in a
  module.
- A rule whose reason is not self-evident MUST have a rationale.
- References MUST be checked against the current source when the rule is
  written, not cited from memory, and MUST pin a version, such as WCAG 2.2 or
  OWASP ASVS 5.0.
- A document without a version is pinned by its publication date or, for a
  page that changes in place, by the date it was read, written
  `(read YYYY-MM-DD)`. A CWE entry is cited by its ID alone, such as CWE-89.
- Each release of the standard MUST review whether to adopt newer versions of
  the standards that its rules reference.

### Reference, don't reinvent

Where an established standard or specification already defines something
(SemVer, Keep a Changelog, RFC 9457, OpenAPI, WCAG, CycloneDX / SPDX, OWASP
ASVS, SLSA, CIS Benchmarks, OpenTelemetry, NIST SSDF, the OpenSSF Compiler
Hardening Guide, and so on), a rule MUST require conformance to it and cite
it, and MUST NOT restate its content. USS adds only what nothing else covers,
plus the glue: which standard, which level, and when it applies. The same
goes for tools: when a widely adopted tool already encodes best practice, a
module points to it.

### Choosing a level

- Use MUST or MUST NOT when the absence is a defect in every project the rule
  applies to, and any team of any size can meet the rule. Examples: safety
  flags, input validation, secrets handling, pinned dependencies, tests in CI,
  a licence file.
- Use SHOULD or SHOULD NOT when the practice is valuable but legitimate
  reasons to skip it exist. Examples: an SBOM for an internal tool, signed
  artifacts, ARM64 builds, offline install.
- Use MAY for optional capabilities. MAY rules count toward scores
  (section 8).
- Keep small projects light through applicability. A level MUST NOT be
  weakened for that purpose.
- Scale the levels of an external standard with facts, not tiers. For OWASP
  ASVS: Level 1 is a MUST when `has-web-ui or exposes-web-api`; Level 2 is
  a MUST when, in addition, `handles-authentication or
  handles-sensitive-data`; Level 3 is a SHOULD when, in addition,
  `high-assurance`. SLSA build levels and CIS profiles follow the same
  pattern.

### Check integrity

A rule that sets a "zero violations" or "all passing" target MUST be paired
with check-integrity rules, because otherwise a project can meet it by
switching the check off. The check-integrity rules cover every kind of check
(linters, compilers, type checkers, security scanners, tests and coverage):
they set a minimum configuration, need a reason for each disabled rule and
each suppression, and make audits list every suppression.

## 8. Compliance and scores

### Outcomes

An audit gives each applicable rule exactly one outcome:

| Outcome | Meaning | Allowed for |
|---|---|---|
| `pass` | The rule is met. | any rule |
| `fail` | The rule is not met. | any rule |
| `deviation` | The rule is not met, and the record lists a Standard Deviation for it (section 9). | MUST, MUST NOT, SHOULD and SHOULD NOT rules |
| `declined` | The project chose not to adopt the rule. | MAY rules that the record declines (`adopt: false`) |
| `unverified` | The auditor could not decide; a human has to confirm. | any rule |
| `not-applicable` | The rule does not apply to the project. | active rules that do not apply |

- A `pass`, `fail` or `deviation` result MUST give evidence, such as a
  `file:line`, a configuration key or a CI job.
- An auditor that cannot decide a rule MUST record `unverified`. It MUST NOT
  guess.
- An applicable rule with no result is `unverified`. The one exception is an
  applicable MAY rule that the record declines: with no result, it is
  `declined`.
- When the record declines a MAY rule, a result for that rule MUST be
  `declined`.
- `not-applicable` is ignored: it is in no count and no score. On a rule that
  applies, it is an error.
- A result for an unknown rule or a retired rule is an error. So is a result
  other than `not-applicable` for a rule that does not apply.

### Compliance

Compliance answers "are all gaps justified?". It is pass or fail. A project is
compliant when:

- every applicable MUST and MUST NOT rule is `pass`;
- every applicable SHOULD and SHOULD NOT rule is `pass` or `deviation`;
- every applicable MAY rule has a decision in the record; and
- no applicable rule is `unverified`.

Declining a MAY rule needs no justification and does not affect compliance.
An adopted MAY rule that fails lowers the score of its category, but it does
not affect compliance.

### Scores

Scores answer "how much of the ideal is actually there?".

- The score of a category is floor(passed × 100 ÷ applicable): *applicable* is
  the number of applicable rules in the category, and *passed* is how many of
  them are `pass`. It is a whole percentage, rounded down.
- A category with no applicable rules shows `n/a`.
- Every rule has equal weight, so a score reads plainly as "share of the ideal
  achieved".
- `deviation`, `declined`, `fail` and `unverified` all score 0. A score
  measures what is there; compliance measures whether the gaps are justified.
  A declined plugin marketplace is why a project can be 100% Safety and 40%
  Features.
- The rules that an adopted MAY rule switches on (section 5) count from then
  on.
- There is no overall score: one number would hide exactly the differences
  that the categories exist to show.

A project is **as perfect as it can be** when it is compliant, with 100% in
every category that has applicable rules. This means zero deviations and
every applicable MAY rule adopted.

### Statements

A compliance statement always names the version and the date. `<version>` is
the USS version of the audit results, and `<date>` is the audit date.

- Compliant: `Compliant with USS <version> as of <date>.`
- Not compliant: `Not compliant with USS <version> as of <date>: <parts>.`

`<parts>` lists each count that is not zero, in this order, separated by
`, `:

| Part | Counts |
|---|---|
| `N MUST failures` | MUST and MUST NOT rules that are `fail` |
| `N documented MUST deviations` | MUST and MUST NOT rules that are `deviation` |
| `N SHOULD failures` | SHOULD and SHOULD NOT rules that are `fail` |
| `N unverified rules` | rules of any level that are `unverified` |
| `N missing MAY decisions` | applicable MAY rules with no decision in the record |

When N is 1, the part is singular, such as `1 MUST failure`.

The score line lists every category in registry order, as `<Name> <P>%` or
`<Name> n/a`, separated by `, ` and ending with `.`. For example:

```text
Compliant with USS 1.0.0 as of 2026-11-21.
Safety 100%, Reliability 94%, Efficiency 88%, Maintainability 97%, Contributability 90%, Compatibility 80%, Fairness 100%, Accessibility & Usability n/a, Features 40%.
```

```text
Not compliant with USS 1.0.0 as of 2026-11-21: 1 documented MUST deviation, 2 unverified rules.
```

## 9. Standard Deviations

A Standard Deviation records an approved decision not to follow a rule.

- Each deviation MUST have its own Markdown document, written from
  `templates/standard-deviation.md`. The document states the rule ID and
  level, the USS version, the scope (what exactly is not followed), the
  reason, the risks and impact, the compensating controls, the approval
  (approver and date), the review-by date, and the remediation plan, if any.
- The approval follows the adopter's own policy. USS sets no approval process.
- Each deviation MUST be listed in `deviations` in the record (section 11),
  with `rule`, `document`, `approved-by`, `date` and, when set, `review-by`.
  `document` MUST name a file that exists.
- A deviation MUST be for a MUST, MUST NOT, SHOULD or SHOULD NOT rule that
  applies to the project. A MAY rule takes a decision instead.
- A record MUST NOT list two deviations for one rule.
- `review-by` MUST NOT be earlier than `date`.

A deviation from a SHOULD or SHOULD NOT rule keeps the project compliant. A
MUST or MUST NOT rule can be deviated from too: organisations do what they
have to, and the Standard Deviation records it honestly. But the project is
then **not compliant**, and its statement reports the deviation as
`N documented MUST deviations`. The word "compliant" keeps its meaning.

To record a deviation:

1. Copy `templates/standard-deviation.md` into the project, for example to
   `docs/standard-deviations/USS-SEC-012.md`.
2. Fill in every section of the copy.
3. Get approval as your organisation's policy says.
4. Add the deviation to `deviations` in `.uss/record.yaml`.
5. Run `uss check .uss/record.yaml`.
6. In the next audit, record the rule's result as `deviation`, with the
   document as evidence.

## 10. Versioning of the standard

The standard uses SemVer, applied to the standard as a document. Projects pin
a version in their record, and upgrading is a deliberate act (just like pinned
dependencies), so:

- **MAJOR**: changes to the conventions, rule format, record format or
  compliance definition; raising a rule's level; retiring rules en masse.
- **MINOR**: new rules, areas, modules or facts.
- **PATCH**: wording, references or verification text with no normative
  change.

Until the USS maintainers declare 1.0.0, releases are **0.x**: the rule
format, record format and compliance definition may still change in a MINOR
release. Release 1.0.0 does not have to include every planned module; modules
added after 1.0.0 are MINOR releases.

- `CHANGELOG.md` MUST list the added, changed and retired rule IDs of each
  release, so an agent upgrading a project's pinned version only asks about
  what changed.
- Each release of the `uss` tool carries one version of the standard, and has
  the same version number. `uss` refuses a record that pins another version.
  To work with that record, use the `uss` release with the record's version.

## 11. Records and audit results

### The record

A project keeps its record at `.uss/record.yaml`, in its root directory. The
record holds the inputs to compliance; audit results are in separate files.
`schema/record.schema.json` defines the format. `uss` refuses a record that
does not match the schema, including one with an unknown key. Start from
`templates/record.yaml`.

| Key | Present | Content |
|---|---|---|
| `standard.version` | always | The pinned USS version (section 10). |
| `project.name` | always | The project's name. |
| `stack` | when needed | `languages`, `frameworks`, `platforms` and `tools`: lists of stack identifiers (section 5), each without duplicates. |
| `facts` | always | `<fact-id>: true` or `<fact-id>: false` for each asked fact (section 5). |
| `decisions` | when needed | `<rule-id>: { adopt: true }` or `<rule-id>: { adopt: false }` for each applicable MAY rule. |
| `deviations` | when needed | One entry for each Standard Deviation (section 9). |
| `audits` | when needed | One entry for each audit: `date`, `results` (the results file) and `report` (the report file). |
| `components` | when needed | The directories of child components (see "Components"). |

- The record MUST answer every fact that has a `Question`. It MUST NOT answer
  an unknown fact or a fact that a rule enables.
- The record MUST have a decision for every applicable MAY rule, and MUST NOT
  have a decision for any other rule.
- The results file of each audit MUST exist.
- Dates have the form `YYYY-MM-DD`.
- A path in the record is relative to the project root: the directory that
  holds `.uss/`. It can use `/` or `\`. It MUST NOT be absolute, start with a
  drive such as `C:`, or contain `..`.

`uss check .uss/record.yaml` checks the record against the standard and the
project's files, reports each problem it finds, then checks each component
record in the same way.

To start a record:

1. Copy `templates/record.yaml` to `.uss/record.yaml` in the project root.
2. Set `project.name`.
3. List the project's stack.
4. Answer each fact. The `false` values in the template are placeholders,
   not defaults.
5. Run `uss applicable .uss/record.yaml` to list the rules that apply.
6. Add a decision for each applicable MAY rule.
7. Run `uss check .uss/record.yaml`.
8. Correct each problem that it reports.

### Audit results

An audit writes its results to `.uss/audits/<date>.yaml`, where `<date>` is
the audit date. `schema/audit-results.schema.json` defines the format.

| Key | Present | Content |
|---|---|---|
| `standard.version` | always | The USS version of the audit. It MUST be the version that the record pins. |
| `commit` | always | The full 40-character SHA of the audited commit, in lowercase hexadecimal. |
| `date` | always | The audit date, `YYYY-MM-DD`. |
| `auditor` | when known | Who or what ran the audit. |
| `results` | always | `<rule-id>: { result: <outcome>, evidence: <text> }` for each rule (section 8). |
| `suppressions` | when found | Every suppression in the project's checks: `file`, `line` (when known), `rule` (the suppressed check's own rule, such as `E501` or `clippy::unwrap_used`, not a USS rule ID) and `reason`. |

```yaml
standard:
  version: 0.1.0
commit: 0123456789abcdef0123456789abcdef01234567
date: 2026-11-21
auditor: claude
results:
  USS-SEC-013:
    result: pass
    evidence: src/db/query.go:42
```

### The report

`uss report` generates the readable report `.uss/audits/<date>.md` from the
results, with the layout in `templates/audit-report.md.j2`. `uss report` and
`uss release-check` first run the checks of `uss check` on the record, and
refuse a record that fails them. The report shows the compliance statement
and the category scores first. Then it lists the stack entries that no module
covers. Then it shows every applicable rule with its outcome and evidence,
grouped by outcome in the order fail, unverified, deviation, declined and
pass. In each group, the rules are sorted by level, from MUST to MAY, then by
ID. Then it lists the deviations and the suppressions. Scores and reports
MUST NOT be computed or written by hand.

To record an audit:

1. Go to the project root.
2. Write the results to `.uss/audits/<date>.yaml`.
3. Add the audit to `audits` in `.uss/record.yaml`.
4. Run `uss check .uss/record.yaml`.
5. Correct each problem that it reports.
6. Run `uss report .uss/audits/<date>.yaml`. It writes
   `.uss/audits/<date>.md`, then prints the statement and the score line.
7. Commit the record, the results and the report together.

### Components

A repository can hold several independently released units. Each one MUST
have its own record. The repository's root `.uss/record.yaml` covers the
repository-wide rules (such as CI, supply chain, `SECURITY.md` and the
licence). It lists the directory of each component in `components`, such as
`components: [libs/core, cli, web]`.

- Each component directory MUST hold its own `.uss/record.yaml`, with its own
  facts, stack, decisions, deviations and audits.
- A path in a component's record is relative to the component's directory.
- A component path MUST NOT leave the project, and MUST NOT be the record's
  own directory.
- A component record MUST pin the same USS version as the root record.
- A component record can list components of its own.

The repository is compliant when its root record and every listed component
record are compliant.

### Release audits

An audit can run at any time. At a release, `uss release-check` proves that
the audits describe the release commit:

- For the root record and each component record, it takes the latest audit in
  `audits`: the one with the latest date or, for equal dates, the later entry.
- The audited commit MUST be the release commit (`HEAD`), or an ancestor of
  it whose only later changes are under a `.uss/` directory. A changed path is
  under a `.uss/` directory when any part of it is `.uss`, such as
  `svc/.uss/record.yaml`. A rename counts as a change to both its old path and
  its new path.
- It generates each report again, writes them all to one file
  (`uss-report.md` by default), and prints each compliance statement. With
  `--require-compliant`, it fails when any record is not compliant.
- The audited commit MUST be in the clone, so a CI job that runs
  `uss release-check` MUST fetch the full history. With `actions/checkout`,
  set `fetch-depth: 0`.
