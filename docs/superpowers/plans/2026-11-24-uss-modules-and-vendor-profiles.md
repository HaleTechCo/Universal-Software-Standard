# USS Modules and Vendor Profiles Implementation Plan (Plan 4 of 5)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add the v1 modules (spec §6.3), their community-maintained vendor
profiles (spec §6.6), the three regulation modules (spec §6.7), and the
validator support that checks them, so that every file passes `uss lint`.

**Architecture:** Tasks 1–3 change the validator. Task 1 clears the items
that Plan 1 deferred. Task 2 makes `uss lint` check every rule ID written in
the text and the reach of each `Refines`. Task 3 adds vendor profiles and the
sections of a regulation module to the file formats. Task 4 applies the items
that Plan 3 parked in the core areas. Tasks 5–14 write the content: modules
under `standard/modules/<group>/`, and profiles under
`standard/vendors/<vendor>/`. Task 15 aligns the spec and hands over.

**Tech Stack:** Rust 1.100.0 (`tools/uss`, `tools/xtask`, `cargo xtask
check`); Markdown rule and profile files. No new crate dependencies.

**Spec:** `docs/superpowers/specs/2026-11-21-universal-software-standard-design.md`
(§5.2 rule format and `Refines`, §5.3 applicability, §6.3 modules, §6.4
rule-writing bar, §6.5 check integrity, §6.6 vendor profiles, §6.7 regulation
modules, §9 validator, §12 left to the builder). The normative rulebook is
`standard/conventions.md`: read sections 1–7 before writing a rule, and the
sections that Task 3 adds before writing a profile or a regulation module.

**Roadmap:** Plans 1–3 are merged into local `main` (d8eb4ae; Plan 3 is not
pushed yet). This is Plan 4. Plan 5 adds the AGENTS.md workflows, the Claude
skills and this repository's self-audit, then releases 0.1.0.

## Global Constraints

- Keywords: exactly `MUST`, `MUST NOT`, `SHOULD`, `SHOULD NOT`, `MAY` (RFC 2119
  as clarified by RFC 8174). `SHALL`, `SHALL NOT`, `REQUIRED`, `RECOMMENDED`,
  `NOT RECOMMENDED`, `OPTIONAL` are never used. One rule, one level.
- A rule body contains its own keyword and no other keyword, rationale
  included (conventions §3 "Keyword scan"). A keyword wrapped across a line
  break still counts. Put any other keyword only inside a fenced code block.
- Rule IDs: `USS-<PREFIX>-<NNN>`. In each new file the IDs run from 001 in
  file order, with no gaps, exactly as the tables give them. IDs are never
  reused or renumbered.
- Every rule has `Level`, `Applies when`, `Category`, `Verify` and
  `References`, and a body. `Category` is the category name exactly as
  `standard/registry.yaml` writes it: `Safety`, `Reliability`, `Efficiency`,
  `Maintainability`, `Contributability`, `Compatibility`, `Fairness`,
  `Accessibility & Usability`, `Features`.
- The rule-writing bar (spec §6.4, conventions §7): one requirement per rule;
  verifiable from the project's artifacts; a `**Rationale:**` paragraph when
  the reason is not self-evident.
- Reference, don't reinvent: a rule requires conformance to an external
  standard, specification or tool and cites it; it never restates it.
- References MUST be checked against the current source when written, not
  cited from memory, and MUST pin a version (conventions §7). A page without a
  version is pinned `(read YYYY-MM-DD)`. A CWE is cited by its ID alone.
  `none` is allowed only where no source applies, after a search. A source
  that the implementer could not read is not cited.
- A rule body, a free text or a table names a rule by ID only when that rule
  exists when the task ends (Task 2 makes `uss lint` enforce this).
- Verify text checks exactly what the body requires: it adds no requirement
  and no exemption. Where a requirement is judged from history, Verify uses
  the changes since the last audit, or the last 12 months when there is none.
  Where it needs host settings the auditor may not see, Verify says to record
  `unverified` without access.
- The standard's version stays `0.1.0`. It is unreleased; Plan 5 releases it.
- The content of `Initial_Idea.md` and `todo.txt` is never modified.
- Files are UTF-8 with LF line endings. Wrap prose at 80 columns. Table rows
  and fenced code may be longer.
- Rust: `cargo xtask check` passes after every task (fmt, Clippy with the
  workspace lints, tests, the wasm32 library build, cargo-deny, `uss lint`).
  The library (`tools/uss/src`, outside `cli/`) does no file, network or
  process I/O (spec §9).
- Tools are never installed by a task. A step that runs a tool runs it only
  when `<tool> --version` already succeeds; otherwise the report says "not
  run".

## Plan-level decisions (refining the spec, for the user's review)

1. **Levels of module rules.** A module rule that makes core rules concrete
   names them in `Refines` and has the strength of the strongest of them
   (MUST and MUST NOT are one strength, SHOULD and SHOULD NOT another), so a
   module never weakens or silently raises a core rule. The best-in-class
   tool for an outcome is a separate SHOULD (spec §6.3), a "tool rule": it
   refines the same core rules as its outcome and is exempt from the
   strength rule. A SHOULD that is not a tool rule refines no MUST rule, so
   `checkJs` (TS-006) and NullAway (JAVA-005) have no `Refines`. A module's
   rules that refine USS-SAF-001 are the safety checks that SAF-001 makes
   binding for its language. A regulation module
   may set a stronger level than core where the law does (the CRA's SBOM is a
   MUST over SUP-013's SHOULD); its rules do not use `Refines`, and its
   obligations table maps the law to the rules instead.
2. **What goes in a module and what goes in a profile.** A module states the
   technology's own flags and settings: the compiler, the standard formatter,
   the package manager, and, for the Maven, Compose and Kubernetes modules,
   the configuration of the technology that the module is about. A vendor
   profile holds the configuration of a separate checking tool whose rule set
   sets a floor (a linter, a type checker, a scanner with a rule set). A tool
   without such a rule set (a formatter, `govulncheck`, `kubeconform`) is
   named by its module rule and has no profile.
3. **Tool choices**, each confirmed by the implementer from current sources,
   who reports a disagreement instead of switching:
   - Python: Ruff, mypy, uv, deptry.
   - Go: golangci-lint, `govulncheck`.
   - Rust: Clippy, cargo-deny.
   - TypeScript and JavaScript: ESLint with typescript-eslint, Prettier,
     pnpm, knip.
   - Java: Error Prone, NullAway, SpotBugs with Find Security Bugs,
     google-java-format.
   - Maven: Maven Enforcer, the Maven Dependency Plugin, the maven-lockfile
     plugin, the Maven Wrapper.
   - Known vulnerabilities in Python, TypeScript and Maven projects:
     OSV-Scanner.
   - Kubernetes: kube-linter and kubeconform.
   - PostgreSQL: Squawk.
   - React: eslint-plugin-react-hooks and eslint-plugin-jsx-a11y.

   mypy over pyright (spec §6.3 leaves the choice to current sources): mypy
   is the reference implementation of the Python typing specification and
   has the plugin ecosystem (Django, Pydantic) that strict typing of real
   projects needs.
4. **Seventeen profiles**, all community-maintained, none with
   tool-specific rules: Ruff, mypy, deptry, OSV-Scanner, golangci-lint,
   Clippy, cargo-deny, ESLint with typescript-eslint, knip, Error Prone,
   NullAway, SpotBugs, kube-linter, Squawk, eslint-plugin-react-hooks,
   eslint-plugin-jsx-a11y and OpenSSF Scorecard. The validator still supports
   tool-specific rules (spec §6.6). No CODEOWNERS entries yet: USS
   maintainers own community profiles, and a vendor that takes one over adds
   its own entry (Plan 5's CONTRIBUTING.md explains how).
5. **No Dockerfile module in v1.** USS-HRD-001 covers images through the CIS
   Docker Benchmark; a Dockerfile module (with hadolint) is a candidate for
   later. The Compose module covers only Compose files.
6. **A stack entry that no module covers** is listed in the audit report as
   "no module" (spec §6.3), not rejected by `uss check`: a project in a
   language with no module is audited against the core rules. This settles
   Plan 1's deferred "validate stack entries" item.
7. **Facts.** New asked fact `covered-by-eaa`. `sold-in-eu` now implies
   `publishes-releases`, and its definition says that a service the project
   only runs for its users is not placed on the market.
8. **JavaScript:** strict mode is a MUST; type-checking JavaScript with the
   TypeScript compiler (`checkJs`) is a SHOULD.
9. **Plan 3's parked items** (Task 4): SEC-031, SEC-032, SUP-013 to SUP-016
   and DST-015 apply on `publishes-releases`; new BAK-014 (offer encrypted
   backups); BAK-008 counts toward Safety; DST-010 keeps the data in place;
   COM-002 no longer judges other rules; AI-009 also covers command-line
   output; wording fixes in DAT-004, EXT-002 and AI-007. TRU-005 and COM-008
   stay as they are: one judges where a change is announced, the other how
   early. An operated service that removes a feature from every edition is
   judged by no rule; the spec says so (Task 15).
10. **`uss lint: ok (<N> rules, <M> files)`:** `<M>` now counts area, module
    and profile files.

## Homes

| Requirement | Home | Elsewhere |
|---|---|---|
| Linter, type checker and scanner configuration | the tool's vendor profile | modules name the tool and the profile |
| Compiler, formatter and package-manager settings | the language module | — |
| Java build, dependencies and plugins | MVN | JAVA covers the compiler and the language |
| Known-vulnerability scanning for Python, TypeScript and Maven | OSV-Scanner profile | PY, TS and MVN each recommend it |
| Container hardening in Compose files | COMPOSE | HRD holds the benchmark (HRD-001, HRD-002) |
| Kubernetes hardening fields | HRD-004 to HRD-008 | K8S adds the linters, Helm and the token mount |
| React-only lint rules | REACT | TS holds the rest of the lint |
| Obligations of a law | the regulation module's obligations table | its rules cover only what core leaves open |

## Review Focus

1. **A module rule that applies where the core rule it refines does not**,
   such as a dependency rule for a project with no dependencies. A project
   would be asked about rules that do not fit it. Pinned by the reach check
   of `Refines` (Task 2) and by the all-false floor test, which also asserts
   that no module rule applies to the all-false record (Task 1).
2. **A profile mapping that claims more than the configuration delivers**,
   such as `full` for a rule that the tool checks only in part, or a mapping
   in the wrong category. Task 3 makes `uss lint` check the IDs, the
   categories and the coverage format; each task's review checks each
   `full` claim against the tool's documentation.
3. **A baseline configuration that the tool rejects**, such as a renamed
   rule code. Each profile task runs the tool on the baseline when the tool
   is already installed, and the review checks each code against the docs.
4. **A tool that is no longer the best choice or no longer maintained.**
   Each task's Step 1 confirms each tool from current sources.
5. **A regulation module that reads as legal advice or gets a date wrong.**
   Task 3 makes `uss lint` require the disclaimer and the obligations table;
   Tasks 13 and 14 confirm each date and article from the official text.

---

### Task 1: Validator follow-ups from Plan 1

**Files:**
- Modify: `tools/xtask/src/main.rs`
- Create: `tools/uss/src/evaluate/components.rs`
- Modify: `tools/uss/src/evaluate/{mod.rs,check.rs,project.rs,score.rs}`
- Modify: `tools/uss/src/catalog/mod.rs`
- Modify: `tools/uss/src/cli/{mod.rs,files.rs}`
- Modify: `tools/uss/src/report.rs`, `templates/audit-report.md.j2`
- Modify: `standard/conventions.md` (§5 "Stack identifiers", §11 "The report")

**Interfaces:**
- Produces, in `uss::evaluate`:
  - `pub fn walk(src: &dyn Source, root: &Record, project: &dyn ProjectFiles) -> Walk`
  - `pub struct Walk { pub records: Vec<Found>, pub problems: Vec<Diagnostic> }`
  - `pub struct Found { pub dir: String, pub record: Record }`: `dir` is the
    record's directory relative to the root record's directory, with
    forward slashes, `""` for the root. Order: the root first, then each
    component depth first, in `components` order.
  - `pub fn project_root(record_path: &str) -> String`: the parent of the
    `.uss` directory that holds the record, or the record's own directory;
    forward slashes; `"."` when empty.
  - `pub fn version_mismatch(pinned: &str, carried: &str) -> Option<String>`:
    `None` when equal; otherwise the text that `cli::version_error` returns
    today.
- Produces in `uss::catalog`: `pub struct ModuleFile { pub file: String, pub
  title: String, pub applies: Expr }` and `Catalog::modules: Vec<ModuleFile>`
  (every file under `standard/modules/`, sorted by path; `applies` is the
  file's header condition).
- Produces `Evaluation::uncovered_stack: Vec<String>`: each stack fact of the
  record (`lang.<id>`, `framework.<id>`, `platform.<id>`, `tool.<id>`) that
  no `ModuleFile::applies.idents()` contains, sorted.

- [ ] **Step 1: Lint the wasm32 library build.** In `check()` of
  `tools/xtask/src/main.rs`, before the `==> wasm32 library build` step, add
  a step that prints `==> wasm32 library clippy` and runs
  `cargo clippy --locked -p uss --lib --no-default-features --target
  wasm32-unknown-unknown -- -D warnings`. Run `cargo xtask check`. Expected:
  exit 0. Fix any warning that the new step finds, then commit:
  `build(xtask): lint the wasm32 library build with Clippy`.

- [ ] **Step 2: Write the failing tests.**
  - `evaluate::components::tests::walk_orders_records_depth_first`: a root
    with `components: [a, b]`, where `a` lists `[c]`; the `dir`s are
    `["", "a", "a/c", "b"]`.
  - `evaluate::components::tests::walk_reports_bad_components`:
    `components: ["../x", ".", missing]` gives the three problems that
    `check_record` reports today (`leaves the project`, `is the record's own
    directory`, `component record "missing/.uss/record.yaml" not found`), and
    a child that pins another version gives `pins USS 9.9.9; the root record
    pins USS 0.1.0` and is not walked. `records` holds only the root.
  - `evaluate::project::tests::project_root_strings`:
    `".uss/record.yaml"` → `"."`; `"proj/.uss/record.yaml"` → `"proj"`;
    `"proj\\.uss\\record.yaml"` → `"proj"`; `"x/record.yaml"` → `"x"`;
    `"record.yaml"` → `"."`.
  - `evaluate::tests::version_mismatch_text`: `("0.1.0", "0.1.0")` → `None`;
    `("0.2.0", "0.1.0")` → a message that starts with
    `record pins USS 0.2.0; this uss carries USS 0.1.0.`
  - `evaluate::score::tests::evaluation_lists_uncovered_stack`: the fixture
    record `stack: { languages: [go, cobol], tools: [make] }` gives
    `["lang.cobol", "tool.make"]` (the fixture's Go module covers `lang.go`).
  - `report::tests::render_lists_uncovered_stack`: the rendered report holds,
    after the Scores section and before the Results section:

    ```markdown
    ## Stack without a module

    No USS module covers these stack entries, so the audit applied the core rules to them directly: `lang.cobol`, `tool.make`.
    ```
    With an empty list the section is absent, so `render_golden` is
    unchanged.
  - `cli::tests::release_check_component_problems_are_all_reported`: add
    that no output line appears twice.
  - `evaluate::tests::all_false_record_floor`: add
    `assert!(applicable(&c, &r).iter().all(|rule| is_core(rule)), "a non-core rule applies to the all-false record");`

  Run `cargo test --locked -p uss`. Expected: the new tests fail.

- [ ] **Step 3: Implement.** `check_record` keeps its signature and its
  messages and is built on `walk` (each found record is checked by
  `check_own`, through a `SubProject` for its `dir`). `cli`'s
  `collect_records` is removed: `release-check` takes its records from
  `walk` and its problems from `check_record` only, so the dedupe set goes.
  `release-check` still names each record by the path it shows today: the
  root record's project root, joined with `dir` and `.uss/record.yaml`.
  `cli::project_root` and `cli::version_error` call the library functions.
  `evaluate` fills `uncovered_stack`; the report template renders the
  section.

- [ ] **Step 4: Update `standard/conventions.md`.** In §5 "Stack
  identifiers", after "so a stack entry that the registry does not list
  matches no rule.", add: "The audit report lists each stack entry that no
  module covers, which is each one that no module file's `Applies when`
  header names: the audit applied the core rules to it directly." In §11
  "The report", after "The report shows the compliance statement and the
  category scores first.", add: "Then it lists the stack entries that no
  module covers."

- [ ] **Step 5: Run all checks.** `cargo xtask check`. Expected: exit 0;
  `uss lint: ok (317 rules, 31 files)`.

- [ ] **Step 6: Commit** `refactor(uss): walk component records once, and note stack entries without a module`.

---

### Task 2: Check rule IDs in text and the reach of `Refines`

**Files:**
- Modify: `tools/uss/src/catalog/{markdown.rs,mod.rs}`
- Modify: `standard/conventions.md` (§3 `Refines` row, §4)

**Interfaces:**
- Produces `pub(crate) fn rule_refs(data: &[u8]) -> Vec<(usize, String)>` in
  `catalog/markdown.rs`: each match of `\bUSS-[A-Z][A-Z0-9]{1,7}-\d{3}\b`
  outside fenced code blocks, with its 1-based line. Task 3 calls it for
  profile files.

`load` gains two checks:

1. **IDs in text.** For `standard/applicability.md` and each area and module
   file, each ID that `rule_refs` finds names a rule of the catalog (active
   or retired). Otherwise: `<file>:<line>: unknown rule <ID>`.
2. **Reach of `Refines`.** For each active rule R and each active rule T in
   its `Refines`: take the identifiers of R's and T's effective conditions.
   For every true/false assignment of them, close the true set under the
   facts' `Implies`; when R is true and T is false, report at R:
   `<R> applies where <T>, which it refines, does not (for example when true: <idents>)`,
   with the closed true set sorted and comma-separated, or `nothing`. Report
   the first such assignment only. With more than 16 identifiers, report
   `<R>: cannot check that it applies only where <T> does: <n> identifiers, over the limit of 16`.

- [ ] **Step 1: Write the failing tests** in `catalog::tests`:
  - `load_unknown_rule_reference`: adding `See USS-SEC-099.` to the body of
    the fixture's USS-SEC-001 gives `unknown rule USS-SEC-099` at that line;
    the same text inside a fenced block gives no diagnostic; `USS-NOPE-001`
    in a definition in `applicability.md` gives `unknown rule USS-NOPE-001`.
  - `load_refines_reach`: the fixture's USS-GO-001 refining USS-SEC-002
    (`uses-database`) with `Applies when: always` gives
    `USS-GO-001 applies where USS-SEC-002, which it refines, does not (for example when true: lang.go)`;
    with `Applies when: uses-database` it loads; refining USS-SEC-003
    (`has-ui`) with `Applies when: has-web-ui` loads, because `has-web-ui`
    implies `has-ui`.

  Run `cargo test --locked -p uss load_`. Expected: the two new tests fail.
- [ ] **Step 2: Implement** both checks in `catalog/mod.rs`, and
  `rule_refs` in `markdown.rs`.
- [ ] **Step 3: Run `cargo run --locked -p uss -- lint`.** Expected:
  `uss lint: ok (317 rules, 31 files)`. If it reports an unknown ID in a
  standard file, correct the pointer (keep the sentence's meaning) and list
  it in the report.
- [ ] **Step 4: Update `standard/conventions.md`.** §3, `Refines` row: "The
  IDs of the core rules that this rule makes concrete for its technology.
  Each one MUST be a rule in an area file, and the rule MUST apply only where
  each of them applies." §4, after the list of ID rules, add: "Each rule ID
  that a standard file names outside a fenced code block MUST be a rule of
  the standard. `uss lint` checks `applicability.md` and every area, module
  and profile file."
- [ ] **Step 5: Run all checks.** `cargo xtask check`. Expected: exit 0.
- [ ] **Step 6: Commit** `feat(uss): check rule IDs in text and the reach of Refines`.

---

### Task 3: Vendor profiles and regulation modules in the validator

**Files:**
- Modify: `tools/uss/src/catalog/markdown.rs` (groups, tables)
- Create: `tools/uss/src/catalog/profiles.rs`
- Modify: `tools/uss/src/catalog/{rules.rs,mod.rs,export.rs}`
- Modify: `tools/uss/src/testfixture.rs`
- Modify: `standard/conventions.md` (§2, §3, §7; new §12 and §13)

**Interfaces:**
- `markdown.rs`: `pub(crate) struct Line { pub n: usize, pub text: String,
  pub fenced: bool }`; `pub(crate) struct Group { pub heading: String, pub
  line: usize, pub lines: Vec<Line> }`; `Doc::groups: Vec<Group>` (each `##`
  heading with the lines up to the next `##` or `###` heading);
  `pub(crate) struct Table { pub header: Vec<String>, pub rows: Vec<Row> }`;
  `pub(crate) struct Row { pub line: usize, pub cells: Vec<String> }`;
  `pub(crate) fn parse_table(lines: &[Line]) -> Result<Table, (usize, String)>`
  (the first run of unfenced lines that start with `|`; the second line is
  the separator; cells split on `|` that is not escaped as `\|`, trimmed).
- `rules.rs`: `FileKind::Profile`; `parse_rule` becomes `pub(crate)`.
- `catalog`: `pub struct Profile { pub file: String, pub product: String,
  pub vendor: String, pub maintainer: Maintainer, pub versions: String, pub
  mappings: Vec<Mapping> }`; `pub enum Maintainer { Community, Vendor }`;
  `pub struct Mapping { pub line: usize, pub rule: String, pub category:
  String, pub tool: String, pub coverage: Coverage, pub verify: String }`;
  `pub enum Coverage { Full, Partial(String) }`; `Catalog::profiles:
  Vec<Profile>`. Profile rules join `Catalog::rules` with
  `FileKind::Profile`.
- `Export::profiles: Vec<ExportProfile>`: `file`, `product`, `vendor`,
  `maintainer` (`"community"` or `"vendor"`), `versions`, `mappings` (each
  `rule`, `category`, `tool`, `coverage` (`"full"` or `"partial"`), `needs`
  (the partial text or `null`), `verify`).

**The profile format** (written into conventions §12 in Step 5):

- Path `standard/vendors/<vendor>/<product>.md`: exactly one directory
  below `standard/vendors/`. `<vendor>` is the lowercase name of the
  organisation that owns the tool's source repository.
- Header fields: `Kind` (`profile`), `Product`, `Vendor`, `Maintainer`
  (`community` or `vendor`), `Versions` (the tool versions that the profile
  was checked against, as a range such as `>=0.14.0, <0.15.0`), and, only
  when the profile has rules, `Prefix` and `Applies when`. No other field.
- `##` sections: `## Baseline configuration` (exactly one fenced code
  block), `## Mappings` (one mapping table), `## Check-integrity guards`, and
  `## Rules` only when the profile has rules. No other `##` heading.
- Mapping table header, exactly: `Rule | Category | Tool rules or settings |
  Coverage | How to verify`. Each row: one active rule ID; its category,
  exactly as the rule has it; rows in registry category order, then by rule
  ID; no rule twice; `Coverage` is `full` or `partial: <what else is
  needed>`; the tool and verify cells are not empty.
- Tool-specific rules: the normal rule format, after `## Rules`, with the
  profile's `Prefix`. `Refines` is allowed on module and profile rules.
- Profile files count in the `<M>` of `uss lint: ok (<N> rules, <M> files)`.
- `rule_refs` (Task 2) runs on profile files too.

**The regulation module format** (conventions §13): a module file under
`standard/modules/regulations/` has `## Disclaimer` (text that is not empty)
and `## Obligations` (one table), and its rules have no `Refines`.
Obligations table header, exactly: `Obligation | Source | Rules |
Coverage`. `Rules` is `none` or comma-separated active rule IDs. `Coverage`
is `full`, `partial: <what is left>` or `none: <what is left>`; `none`
rules go with `none:` coverage, and `full` or `partial` with at least one
rule.

- [ ] **Step 1: Extend the fixture.** Add `standard/vendors/acme/lint.md`: a
  community profile of product `Lint`, vendor `Acme`, versions
  `>=1.0.0, <2.0.0`, a `toml` baseline block, mappings for USS-SEC-001
  (Safety, `full`) and USS-EXT-002 (Features, `partial: a reviewer checks
  the plugin rights`), and a guards section. Add
  `standard/modules/regulations/law.md`: prefix `LAW`, `Applies when:
  has-web-ui`, a disclaimer, an obligations table with one `full` row
  (USS-SEC-003, USS-LAW-001) and one `none: …` row, and one rule
  USS-LAW-001 (MUST, Safety). Update the counts in `load_fixture_ok`
  (8 rules, 5 files) and the expectations of `export_fixture`.
- [ ] **Step 2: Write the failing tests.**
  - `markdown::tests::parse_doc_groups` and
    `markdown::tests::parse_table_cases` (header and rows, an escaped `\|`,
    a missing separator line, a row with too many cells, a fenced table that
    is ignored).
  - `profiles::tests::profile_valid` (the fixture profile loads;
    `Catalog::profiles[0].mappings` has 2 entries; coverage `Full` and
    `Partial("a reviewer checks the plugin rights")`).
  - `profiles::tests::profile_errors`, one case each, asserting the message
    text: a profile directly under `standard/vendors/`; an unknown header
    field; `Maintainer: someone`; a missing section; two code blocks in the
    baseline; a wrong table header; a mapping to an unknown rule; a mapping
    to a retired rule; a category that is not the rule's; Features before
    Safety; one rule mapped twice; coverage `partly`; an empty verify cell;
    a `###` rule in a profile with no `Prefix`.
  - `catalog::tests::load_profile_with_rules`: the fixture profile with
    `Prefix: LINT`, `Applies when: lang.go`, `## Rules` and one rule loads,
    and the rule has `FileKind::Profile`.
  - `catalog::tests::regulation_errors`: no disclaimer; no obligations
    table; `none` rules with `full` coverage; `partial: …` coverage with
    rules `none`; an unknown rule; a rule with `Refines`.

  Run `cargo test --locked -p uss`. Expected: the new tests fail.
- [ ] **Step 3: Implement** the scanner changes, `profiles.rs`, the load of
  `standard/vendors/`, the regulation checks and the export.
- [ ] **Step 4: Run `cargo run --locked -p uss -- lint`.** Expected:
  `uss lint: ok (317 rules, 31 files)` (the real standard has no profiles
  yet).
- [ ] **Step 5: Update `standard/conventions.md`.**
  - §2: add the row `standard/vendors/<vendor>/<product>.md` (vendor
    profiles, section 12) to the file table, `profile` to the `Kind` values,
    and a sentence that a module under `standard/modules/regulations/` is a
    regulation module (section 13). "`<M>` counts the rule files" becomes
    "`<M>` counts the area, module and profile files".
  - §3: the `Refines` row says "module and profile rules only".
  - §7: add "### Module rules": decisions 1 and 2 of this plan, written as
    rules for writers (a module rule names the core rules it makes concrete
    in `Refines`, applies only where they apply, and has the strength of the
    strongest of them; the tool for an outcome is a separate SHOULD that
    names the tool's vendor profile and is exempt from the strength rule; a
    SHOULD that is not a tool rule refines no MUST rule; a module states its
    technology's own
    settings, and a profile holds a separate checking tool's configuration;
    a regulation module may set a stronger level than core where the law
    does, and maps instead of refining).
  - §12 "Vendor profiles": the profile format above, plus the rules of spec
    §6.6 on completeness (a mapping claims `full` only when the baseline
    configuration decides the rule; for each category that a profile lists,
    it maps every MUST within what the tool can check), check-integrity
    guards (a profile lists the tool's guards when the tool has any) and the
    effect on audits.
  - §13 "Regulation modules": the regulation format above, and that the
    disclaimer says USS compliance is not legal compliance and not legal
    advice.
- [ ] **Step 6: Run all checks.** `cargo xtask check`. Expected: exit 0.
- [ ] **Step 7: Commit** `feat(uss): add vendor profiles and regulation modules to the standard's formats`.

---

### Task 4: Core follow-ups from Plan 3

**Files:** `standard/core/{security,supply-chain,distribution,data,backup,commercial-licensing,extensibility,ai-features}.md`;
`tools/uss/src/evaluate/mod.rs` (floor test).

**Values:** floor test `(59, 318)`; lint `uss lint: ok (318 rules, 31 files)`;
commit `fix(standard): apply the items that Plan 3 parked`.

| Rule | Change |
|---|---|
| SEC-031, SEC-032, SUP-013, SUP-014, SUP-015 | `Applies when` becomes `publishes-releases` (a binary release that is not a package is a release too). |
| SUP-016 | `Applies when` becomes `publishes-releases and high-assurance`. |
| DST-015 | Its own `Applies when` becomes `always` (the file header gives `publishes-releases`), to match SUP-014. |
| DAT-004 | Body: "Sensitive data MUST be encrypted where the software stores it, by the storage layer or by the software." Add: backups are the Backup area's (name USS-BAK-008 and USS-BAK-014). |
| BAK-008 | `Category: Safety` (it protects confidentiality). |
| BAK-014 (new) | "Offer encrypted backups", SHOULD, `Applies when: stores-persistent-data and publishes-releases and handles-sensitive-data`, Category Safety: the backup procedure that the software offers (USS-BAK-001) SHOULD be able to write encrypted backups. Place it after USS-BAK-004, in the same group. Cite CWE-311 and the source that BAK-008 cites for encryption, confirmed. |
| DST-010 | Body and Verify: the upgrade keeps the existing data where it is (not deleted, moved aside or replaced); changes that a data migration makes stay USS-DAT-002's. |
| COM-002 | Remove "nor a term that sets aside another rule of this area" from the body, and the matching sentence from Verify. Free text of the area: add "The other rules of this area apply to negotiated agreements too." |
| COM free text | After "(USS-TRU-004)", say that it is a MUST rule. |
| EXT-002 | Verify: "while MAJOR is 0, anything may change (SemVer item 4), so no change fails". References: "Semantic Versioning 2.0.0, items 1 to 4 and 8". |
| AI-007 | Body: "The software MUST limit the requests per user, the tokens per request when the model works in tokens, and the total spend when calls cost money, and MUST alert when a limit is reached." |
| AI-009 | Own `Applies when`: `uses-ai-models and (has-ui or has-cli)`; Verify also covers command-line output. |

- [ ] **Step 1: Update the floor test first** to `(59, 318)` and run
  `cargo test --locked -p uss all_false_record_floor`. Expected: FAIL.
- [ ] **Step 2: Make the edits** in the table.
- [ ] **Step 3: Re-check the references that Plan 3 left weak.** Fetch each
  source and correct the citation, or report it: CON-002 and CON-003 (NIST
  SSDF 1.1, PO.4); PRF-001, PRF-004, PRF-005; HRD-003 (the Compose
  citation); AUD-009 (CEF); I18N-001 (Unicode 18.0.0: is the release
  final?); HRD-001 and HRD-002 (the CIS profile level of each cited
  recommendation).
- [ ] **Step 4: Lint and run all checks.** `cargo xtask check`. Expected:
  exit 0, the values above.
- [ ] **Step 5: Commit.**

---

## How to write a module and its profiles (Tasks 5–14)

The tables fix each rule's ID, title, level, `Applies when`, category,
`Refines` and requirement. The implementer writes the body, the Verify text
and the rationale, confirms each tool and reference, and writes each
profile from its specification.

**Interfaces for every content task:** each consumes the file formats of
Tasks 1–3 (conventions §7, §12 and §13) and the core rules as Task 4 leaves
them; each produces the rule IDs in its tables and the files it lists. The
OSV-Scanner profile of Task 5 serves Tasks 8 and 9 too.

**Module file:** `# <Title>`, then `- **Prefix:**`, `- **Kind:** module`,
`- **Applies when:** <the task's header condition>`. Free text: one short
paragraph that says what the module covers, the profiles it uses (by path),
the tools it names without a profile, and that the core rules it does not
refine apply as written. Then `##` groups as the tables' bold rows give
them, and the rules. A rule's `Applies when`, `Category` and `Refines` are
the table's; the effective condition is the header ANDed with
`Applies when`. An empty `Refines` cell means no `Refines` field.

**A tool rule's Verify** checks that CI runs the tool with a configuration
that includes its profile's baseline, and says that a project on another
tool fails this rule (it records a Standard Deviation) and meets the outcome
rule through that tool's own profile.

**Profile file** (format: conventions §12, written by Task 3): header with
`Maintainer: community`, and `Versions` from the current release to the
next release that may rename rules (the next minor for a 0.x tool, the next
major otherwise). Free text: what the tool checks, which module recommends
it, where the baseline goes, and "Community-maintained: written by USS
contributors until the vendor takes it over." The baseline is a copy-ready
fragment that the implementer builds from the task's "Baseline" line and
the tool's documentation. The mappings cover at least the task's "Map"
list; a rule the tool checks only in part is `partial: <what else>`. The
guards section lists the tool's own check-integrity settings as a table
`Guard | Setting | Rule`, or says in one sentence that the tool has none. A
mapping never lists a tool rule (such as USS-PY-002) for its own tool.

Each content task runs these steps, with the values the task gives:

- [ ] **Step 1: Confirm the tools and references.** For each tool, fetch its
  documentation and release page: is it maintained (a release in the last
  12 months), is it still the leading choice for the outcome, and what is
  its current version? For each rule and each mapped tool rule, confirm the
  setting or rule code. Write the list (item, source URL, version) into the
  task report. Report any tool that current sources do not support, before
  writing the files.
- [ ] **Step 2: Write the module file(s)** from the tables.
- [ ] **Step 3: Write the profile file(s)** from their specifications.
- [ ] **Step 4: Run each profiled tool on its baseline** when the tool is
  already installed: put the fragment in a scratch project with one small
  source file and run the tool's check command. Record the command and the
  result, or "not run", in the report.
- [ ] **Step 5: Lint.** `cargo run --locked -p uss -- lint`. Expected: the
  task's line.
- [ ] **Step 6: Compare the export with the tables.**
  `cargo run --locked -p uss -- export --json`: for each new rule, `id`,
  `title`, `level`, `category` and `refines` equal the table, and
  `appliesWhen` is the header ANDed with the table's condition.
- [ ] **Step 7: Check line lengths.** Run
  `awk '/^(```|~~~)/{f=!f} !f && !/^\|/ && length > 80 { print FILENAME ":" FNR }' <the task's new files>`.
  Expected: no output.
- [ ] **Step 8: Run all checks.** `cargo xtask check`. Expected: exit 0 and
  the task's lint line.
- [ ] **Step 9: Commit** with the task's message.

---

### Task 5: Python, with the Ruff, mypy, deptry and OSV-Scanner profiles

**Files:** Create `standard/modules/languages/python.md`,
`standard/vendors/astral-sh/ruff.md`, `standard/vendors/python/mypy.md`,
`standard/vendors/<org>/deptry.md` (the organisation that owns deptry's
repository today), `standard/vendors/google/osv-scanner.md`.

**Values:** lint `uss lint: ok (334 rules, 36 files)`; commit
`feat(standard): add the Python module and its tool profiles`.

**Python** — `# Python`, prefix `PY`, header `lang.python`.

| ID | Title | Level | Applies when | Category | Refines | Requirement |
|---|---|---|---|---|---|---|
| **## Checks** | | | | | | |
| PY-001 | Lint for correctness, style, imports and security | MUST | always | Maintainability | QLT-001, SEC-028 | CI lints all Python code with zero violations against a rule set that covers correctness, style, import order and security. |
| PY-002 | Lint and format with Ruff | SHOULD | always | Maintainability | QLT-001, QLT-002, SEC-028 | The project lints and formats with Ruff, configured per its vendor profile. |
| PY-003 | Type-check in strict mode | MUST | always | Safety | SAF-001 | CI type-checks all Python code in the type checker's strict mode, so every function is annotated. |
| PY-004 | Type-check with mypy | SHOULD | always | Safety | SAF-001 | The project type-checks with mypy, configured per its vendor profile. |
| PY-005 | Turn warnings into errors in tests | MUST | always | Maintainability | QLT-003 | The test run turns Python warnings into errors (such as pytest `filterwarnings = ["error"]`), and each warning it ignores is named and has a reason. |
| **## Toolchain** | | | | | | |
| PY-006 | Pin the Python version | MUST | always | Safety | SUP-001 | The repository pins the exact Python version for development and CI, such as in `.python-version`. |
| PY-007 | No end-of-life Python versions | MUST NOT | always | Safety | SUP-006 | `requires-python` and the CI matrix do not include a Python version that is end-of-life. |
| **## Dependencies** | | | | | | |
| PY-008 | Declare dependencies in pyproject.toml | MUST | has-dependencies | Safety | DEP-004 | Direct dependencies are declared in `pyproject.toml` (PEP 621 project table, PEP 735 dependency groups), not only in a requirements file. |
| PY-009 | Bound each version specifier | MUST | has-dependencies | Safety | DEP-001 | Each dependency specifier has an upper bound below the next major version, such as `~=1.2` or `>=1.2,<2`. |
| PY-010 | Install from a hashed lock file | MUST | has-dependencies | Safety | DEP-002, DEP-003 | CI and builds install from a committed lock file with hashes (such as `uv.lock`, or a PEP 751 `pylock.toml`), with a command that fails when the lock file is out of date (such as `uv sync --locked`). |
| PY-011 | Manage dependencies with uv | SHOULD | has-dependencies | Safety | DEP-002, DEP-003, SUP-001 | The project resolves, locks and installs its dependencies and its Python version with uv. |
| PY-012 | Keep development dependencies out of the runtime set | MUST | has-dependencies | Safety | DEP-005 | Test, lint and build-only dependencies are in dependency groups or the build-system table, never in the project's runtime dependencies. |
| PY-013 | Check that every import is declared | MUST | has-dependencies | Safety | DEP-004 | CI fails when the code imports a third-party package that is not a declared direct dependency. |
| PY-014 | Check imports with deptry | SHOULD | has-dependencies | Safety | DEP-004, DEP-013 | The project checks its declared and imported dependencies with deptry, configured per its vendor profile. |
| PY-015 | Scan with OSV-Scanner | SHOULD | has-dependencies | Safety | DEP-007 | The project scans its lock file with OSV-Scanner, configured per its vendor profile. |
| PY-016 | Hold back new releases with uv | SHOULD | has-dependencies | Safety | DEP-015 | uv's `exclude-newer` is set as a relative age of at least three days, so resolution ignores releases younger than that. |

Cite: PEP 621, PEP 735, PEP 751, PEP 440 (version specifiers), the Python
Developer's Guide "Status of Python versions" (read date), the pytest and
uv documentation, and each tool's documentation. PY-016: confirm that uv
accepts a relative `exclude-newer`; if it does not, report before writing.

**Profiles:**

- **Ruff** (`astral-sh/ruff.md`, vendor Astral). Baseline: `[tool.ruff.lint]`
  selects at least `F`, `E`, `W`, `I`, `N`, `UP`, `B`, `S`, `BLE`, `A`,
  `C4`, `SIM`, `PT`, `RET`, `ERA`, `PGH`, `PL`, `TRY`, `RUF`; no preview
  rules. Guards: `PGH004` (blanket `noqa`) and `RUF100` (unused `noqa`).
  Map: PY-001, QLT-001, QLT-002, QLT-004, QLT-005, QLT-009, QLT-010,
  QLT-012, SEC-004, SEC-010, SEC-011, SEC-013, SEC-023, SEC-026, SEC-028.
- **mypy** (`python/mypy.md`, vendor the Python project's mypy team).
  Baseline: `strict = true`, `warn_unreachable = true`, and
  `enable_error_code` with at least `ignore-without-code`, `redundant-expr`,
  `truthy-bool`, `possibly-undefined`. Guards: `warn_unused_ignores`,
  `ignore-without-code`. Map: PY-003, SAF-001, QLT-004, QLT-009, QLT-012.
- **deptry**. Baseline: every rule (`DEP001` to `DEP004`) on, no ignores.
  Guards: none (say so; ignores are judged by USS-QLT-007). Map: PY-013,
  DEP-004, DEP-005, DEP-013.
- **OSV-Scanner** (`google/osv-scanner.md`). Baseline: the scan command
  over every lock file of the repository, failing on any finding, and an
  `osv-scanner.toml` whose ignore entries each carry a reason and an expiry
  date. Guards: the reason and the expiry of each ignore entry. Map:
  DEP-007, DEP-008. The free text says that the Python, TypeScript and Maven
  modules recommend it.

---

### Task 6: Go, with the golangci-lint profile

**Files:** Create `standard/modules/languages/go.md`,
`standard/vendors/golangci/golangci-lint.md`.

**Values:** lint `uss lint: ok (348 rules, 38 files)`; commit
`feat(standard): add the Go module and the golangci-lint profile`.

**Go** — `# Go`, prefix `GO`, header `lang.go`. Free text: Go links only the
packages that the code imports, so USS-DEP-005 is met by the toolchain;
tool dependencies use the `tool` directive.

| ID | Title | Level | Applies when | Category | Refines | Requirement |
|---|---|---|---|---|---|---|
| **## Checks** | | | | | | |
| GO-001 | Lint for correctness, style, imports and security | MUST | always | Maintainability | QLT-001, SEC-028 | CI lints all Go code with zero violations against a rule set that covers correctness, style, imports and security. |
| GO-002 | Lint with golangci-lint | SHOULD | always | Maintainability | QLT-001, QLT-002, SEC-028 | The project lints with golangci-lint, configured per its vendor profile. |
| GO-003 | Check formatting with gofmt | MUST | always | Maintainability | QLT-002 | CI fails when `gofmt -l` (or a stricter gofmt-compatible formatter, such as gofumpt) lists a file. |
| GO-004 | Run go vet | MUST | always | Safety | SAF-001 | CI runs `go vet ./...` with its default analyzers, and a finding fails the build. |
| GO-005 | Run tests with the race detector | SHOULD | uses-concurrency | Safety | SAF-007 | CI runs the tests with `go test -race`. |
| **## Toolchain** | | | | | | |
| GO-006 | Pin the Go toolchain | MUST | always | Safety | SUP-001 | `go.mod` pins the exact Go toolchain release, with a `toolchain` directive or a `go` directive that names a patch release. |
| GO-007 | No unsupported Go releases | MUST NOT | always | Safety | SUP-006 | The `go` and `toolchain` directives do not name a Go release that the Go project no longer supports. |
| GO-008 | Build release binaries with -trimpath | SHOULD | publishes-releases | Maintainability | REL-003 | Release builds use `-trimpath`, so binaries hold no paths of the build machine. |
| **## Dependencies** | | | | | | |
| GO-009 | Keep go.mod and go.sum tidy | MUST | has-dependencies | Safety | DEP-004, DEP-013 | CI fails when `go mod tidy` would change `go.mod` or `go.sum`. |
| GO-010 | Keep checksum verification on | MUST | has-dependencies | Safety | DEP-002, DEP-003 | `go.sum` is committed, and no `GONOSUMDB`, `GONOSUMCHECK`, `GOINSECURE` or `GOFLAGS=-mod=mod` setting turns off checksum verification for public modules. |
| GO-011 | No local replace directives in a released module | MUST NOT | has-dependencies and publishes-releases | Safety | DEP-003 | A released `go.mod` has no `replace` directive that points to a local path (local development uses `go.work`). |
| GO-012 | Scan with govulncheck | SHOULD | has-dependencies | Safety | DEP-007 | CI scans the module with `govulncheck`. |
| **## Network** | | | | | | |
| GO-013 | No HTTP clients without a timeout | MUST NOT | uses-network | Reliability | RES-001 | Outbound HTTP calls do not use `http.DefaultClient`, the package-level helpers such as `http.Get`, or an `http.Client` with no `Timeout`, unless each request carries a context with a deadline. |
| GO-014 | Set server timeouts | MUST | is-network-service | Reliability | RES-001 | Each `http.Server` sets `ReadHeaderTimeout`, and the read, write and idle timeouts that its handlers allow. |

Cite: The Go Programming Language Specification and the `go` command
documentation (Go version), "Go Toolchains" (go.dev/doc/toolchain), the Go
release policy, Go Modules Reference, `govulncheck` documentation, `net/http`
package documentation; CWE-400 (GO-013, GO-014).

**Profile: golangci-lint** (`golangci/golangci-lint.md`, vendor the
golangci-lint project). Baseline: a version 2 configuration with the
`standard` linter set plus at least `gosec`, `errorlint`, `bodyclose`,
`noctx`, `nolintlint`, `contextcheck`, `nilerr`, `rowserrcheck`,
`sqlclosecheck`, `forcetypeassert`, `revive`, `gocritic`; the formatters
`gofmt` (or `gofumpt`) and `goimports`; `nolintlint` requiring a reason and a
named linter and reporting unused directives; and issue limits that hide no
finding (`max-issues-per-linter: 0`, `max-same-issues: 0`). Guards:
`nolintlint` (reason, named linter, unused), the issue limits. Map: GO-001,
GO-003, GO-004, QLT-001, QLT-002, QLT-004, QLT-005, QLT-009, QLT-010,
QLT-012, SEC-010, SEC-013, SEC-023, SEC-026, SEC-028, GO-013.

---

### Task 7: Rust, with the Clippy and cargo-deny profiles

**Files:** Create `standard/modules/languages/rust.md`,
`standard/vendors/rust-lang/clippy.md`,
`standard/vendors/embarkstudios/cargo-deny.md`.

**Values:** lint `uss lint: ok (359 rules, 41 files)`; commit
`feat(standard): add the Rust module and the Clippy and cargo-deny profiles`.

**Rust** — `# Rust`, prefix `RUST`, header `lang.rust`. Free text: Clippy
and rustfmt ship with the toolchain, so their rules are MUSTs, not tool
SHOULDs; this repository's own `Cargo.toml` lint table is an example.

| ID | Title | Level | Applies when | Category | Refines | Requirement |
|---|---|---|---|---|---|---|
| **## Checks** | | | | | | |
| RUST-001 | Lint with Clippy | MUST | always | Maintainability | QLT-001, QLT-003 | CI runs `cargo clippy` on all targets and features with warnings as errors, at the lint levels of its vendor profile. |
| RUST-002 | Check formatting with rustfmt | MUST | always | Maintainability | QLT-002 | CI runs `cargo fmt --check`. |
| RUST-003 | Forbid unsafe code where none is needed | MUST | not has-memory-unsafe-code | Safety | SAF-001 | A crate with no unsafe code sets `unsafe_code = "forbid"` in its lint table. |
| RUST-004 | Keep overflow checks in release builds | MUST | always | Safety | SAF-001 | The release profile sets `overflow-checks = true`. |
| RUST-005 | Run Miri on unsafe code | MUST | has-memory-unsafe-code | Safety | SAF-004 | CI runs the tests that exercise unsafe code under Miri. |
| RUST-006 | Write a SAFETY comment for each unsafe block | MUST | has-memory-unsafe-code | Safety | SAF-003 | Each `unsafe` block has a `// SAFETY:` comment that says why it is sound. |
| **## Toolchain** | | | | | | |
| RUST-007 | Pin the toolchain in rust-toolchain.toml | MUST | always | Safety | SUP-001 | `rust-toolchain.toml` pins an exact Rust release, such as `1.100.0`, with the components that CI uses. |
| **## Dependencies** | | | | | | |
| RUST-008 | Build with --locked | MUST | has-dependencies | Safety | DEP-002, DEP-003 | `Cargo.lock` is committed (libraries included), and CI builds and tests with `--locked`. |
| RUST-009 | Bound each version requirement | MUST | has-dependencies | Safety | DEP-001 | Each dependency has a requirement with an upper bound below the next incompatible version, such as Cargo's default caret form `1.2`, and each git dependency names a `rev`. |
| RUST-010 | Check dependencies with cargo-deny | SHOULD | has-dependencies | Safety | DEP-001, DEP-007, LIC-003, SUP-005 | The project checks advisories, licences, bans and sources with cargo-deny, configured per its vendor profile. |
| RUST-011 | Test the minimal versions | SHOULD | has-dependencies and is-library | Safety | DEP-017 | CI builds and tests with the lowest versions that the requirements allow for direct dependencies (such as `cargo update -Z direct-minimal-versions`). |

Cite: The Cargo Book (version), The rustc book lint levels, The Rust
Reference (version), Miri's README (read date), The Rust Unstable Book for
`direct-minimal-versions`, the rustup documentation; CWE-190 (RUST-004).

**Profiles:**

- **Clippy** (`rust-lang/clippy.md`, vendor the Rust project). Baseline: a
  `[workspace.lints.clippy]` and `[workspace.lints.rust]` table with at
  least this repository's levels (`all` and `pedantic` deny at priority -1,
  `unwrap_used`, `expect_used`, `panic`, `allow_attributes`,
  `allow_attributes_without_reason` deny; `missing_docs` deny for
  libraries) plus `undocumented_unsafe_blocks`, `dbg_macro`, `todo`,
  `unimplemented` deny. Guards: `allow_attributes`,
  `allow_attributes_without_reason`. Map: RUST-001, RUST-006, QLT-001,
  QLT-004, QLT-005, QLT-009, QLT-011, QLT-012, CON-001, DOC-005.
- **cargo-deny** (`embarkstudios/cargo-deny.md`, vendor Embark Studios).
  Baseline: a `deny.toml` that denies yanked crates, wildcard requirements
  and unknown registries and git sources, sets a licence allow list (the
  project's own policy, USS-LIC-003), and turns on the advisory database.
  Guards: each ignored advisory and each licence exception carries a
  reason. Map: RUST-009, DEP-001, DEP-006, DEP-007, DEP-008, LIC-003,
  SUP-005. This repository's `deny.toml` is an example.

---

### Task 8: TypeScript and JavaScript, with the ESLint and knip profiles

**Files:** Create `standard/modules/languages/typescript.md`,
`standard/vendors/eslint/eslint.md`, `standard/vendors/webpro-nl/knip.md`.

**Values:** lint `uss lint: ok (376 rules, 44 files)`; commit
`feat(standard): add the TypeScript and JavaScript module and the ESLint and knip profiles`.

**TypeScript and JavaScript** — `# TypeScript and JavaScript`, prefix `TS`,
header `lang.typescript or lang.javascript`. Free text: the OSV-Scanner
profile (Task 5) serves this module too.

| ID | Title | Level | Applies when | Category | Refines | Requirement |
|---|---|---|---|---|---|---|
| **## Checks** | | | | | | |
| TS-001 | Lint for correctness, style and imports | MUST | always | Maintainability | QLT-001 | CI lints all TypeScript and JavaScript with zero violations against a type-aware rule set that covers correctness, style and imports. |
| TS-002 | Lint with ESLint and typescript-eslint | SHOULD | always | Maintainability | QLT-001 | The project lints with ESLint and typescript-eslint, configured per their vendor profile. |
| TS-003 | Format with Prettier | SHOULD | always | Maintainability | QLT-002 | The project formats with Prettier, and CI runs `prettier --check`. |
| TS-004 | Compile with strict type checks | MUST | lang.typescript | Safety | SAF-001 | `tsconfig.json` sets `strict`, `noUncheckedIndexedAccess`, `exactOptionalPropertyTypes`, `noImplicitOverride`, `noImplicitReturns` and `noFallthroughCasesInSwitch`, and CI runs the type check. |
| TS-005 | Run JavaScript in strict mode | MUST | lang.javascript | Safety | SAF-001 | JavaScript is written as ES modules, or each script starts with `"use strict"`. |
| TS-006 | Type-check JavaScript | SHOULD | lang.javascript | Safety | | CI type-checks JavaScript with the TypeScript compiler (`checkJs` with `strict`), from JSDoc types or declaration files. |
| **## Toolchain** | | | | | | |
| TS-007 | Pin Node.js and the package manager | MUST | always | Safety | SUP-001 | The repository pins the exact Node.js release and the package manager's version (such as the `packageManager` field). |
| TS-008 | No end-of-life Node.js | MUST NOT | always | Safety | SUP-006 | `engines.node` and the CI matrix do not include a Node.js release line that is end-of-life. |
| **## Dependencies** | | | | | | |
| TS-009 | Bound each version range | MUST | has-dependencies | Safety | DEP-001 | Each dependency has a range within one major version, such as `^1.2.0`; never `*`, `latest`, `x`, a URL, or a git reference without a commit. |
| TS-010 | Install with a frozen lock file | MUST | has-dependencies | Safety | DEP-002, DEP-003 | The package manager's lock file is committed, and CI installs in its frozen mode (such as `pnpm install --frozen-lockfile` or `npm ci`). |
| TS-011 | Use pnpm | SHOULD | has-dependencies | Safety | DEP-003, DEP-004 | The project installs with pnpm, whose isolated `node_modules` stops code from importing undeclared packages. |
| TS-012 | Keep build and test tools in devDependencies | MUST | has-dependencies | Safety | DEP-005 | Packages used only to build, test or lint are in `devDependencies`, and `dependencies` holds only runtime packages. |
| TS-013 | Check that every import is declared | MUST | has-dependencies | Safety | DEP-004 | CI fails when code imports a package that its `package.json` does not declare. |
| TS-014 | Check dependencies and dead code with knip | SHOULD | has-dependencies | Safety | DEP-004, DEP-013, QLT-004 | The project checks unused and undeclared dependencies, files and exports with knip, configured per its vendor profile. |
| TS-015 | Do not run dependencies' install scripts | SHOULD NOT | has-dependencies | Safety | | An install does not run the lifecycle scripts of dependencies, except those on an allow list (such as pnpm's `onlyBuiltDependencies`). |
| TS-016 | Wait before adopting new releases | SHOULD | has-dependencies | Safety | DEP-015 | The package manager's minimum release age (such as pnpm `minimumReleaseAge`) is at least three days. |
| TS-017 | Scan with OSV-Scanner | SHOULD | has-dependencies | Safety | DEP-007 | The project scans its lock file with OSV-Scanner, configured per its vendor profile. |

Cite: the TypeScript TSConfig Reference (TypeScript version), ECMAScript
(edition) for strict mode and modules, the Node.js release schedule (read
date), the pnpm, npm and Corepack documentation (confirm how a project pins
its package manager today), Prettier documentation. TS-015 and TS-016:
confirm each setting in the pnpm documentation.

**Profiles:**

- **ESLint with typescript-eslint** (`eslint/eslint.md`, product "ESLint
  with typescript-eslint", vendor the ESLint and typescript-eslint
  projects). Baseline: a flat config with ESLint's recommended set,
  typescript-eslint's `strictTypeChecked` and `stylisticTypeChecked`,
  `reportUnusedDisableDirectives` and `reportUnusedInlineConfigs` as errors,
  and the ESLint comments plugin's `require-description` and
  `no-unlimited-disable`. Guards: those four. Map: TS-001, QLT-001, QLT-004,
  QLT-005, QLT-009, QLT-010, QLT-012, SAF-001, SEC-028 (`partial`: a SAST
  tool such as CodeQL or Semgrep is still needed).
- **knip** (`webpro-nl/knip.md`, vendor the knip project). Baseline: every
  issue type on as an error, the configuration in a format that allows
  comments (so each ignore entry carries its reason). Guards: none of its
  own (say so). Map: TS-013, DEP-004, DEP-005, DEP-013, QLT-004.

---

### Task 9: Java and Maven, with the Error Prone, NullAway and SpotBugs profiles

**Files:** Create `standard/modules/languages/java.md`,
`standard/modules/tools/maven.md`, `standard/vendors/google/error-prone.md`,
`standard/vendors/uber/nullaway.md`, `standard/vendors/spotbugs/spotbugs.md`.

**Values:** lint `uss lint: ok (395 rules, 49 files)`; commit
`feat(standard): add the Java and Maven modules and their tool profiles`.

**Java** — `# Java`, prefix `JAVA`, header `lang.java`. Free text: build,
dependency and plugin rules are in the Maven module.

| ID | Title | Level | Applies when | Category | Refines | Requirement |
|---|---|---|---|---|---|---|
| **## Checks** | | | | | | |
| JAVA-001 | Compile with all warnings as errors | MUST | always | Maintainability | QLT-003 | `javac` runs with `-Xlint:all` and `-Werror`, and each lint category that it leaves out has a reason. |
| JAVA-002 | Check for bug patterns when compiling | MUST | always | Maintainability | QLT-001 | CI runs a bug-pattern checker on all code, and a finding fails the build. |
| JAVA-003 | Check with Error Prone | SHOULD | always | Maintainability | QLT-001 | The build runs Error Prone, configured per its vendor profile. |
| JAVA-004 | Analyse with SpotBugs and Find Security Bugs | SHOULD | always | Safety | QLT-001, SEC-028 | CI runs SpotBugs with the Find Security Bugs plugin, configured per its vendor profile. |
| JAVA-005 | Check nullness with NullAway | SHOULD | always | Safety | | Code marks nullness with JSpecify annotations and the build checks it with NullAway, configured per its vendor profile. |
| JAVA-006 | Format with google-java-format | SHOULD | always | Maintainability | QLT-002 | The project formats with google-java-format, and CI fails on a file that it would change. |
| **## Toolchain** | | | | | | |
| JAVA-007 | Pin the JDK | MUST | always | Safety | SUP-001 | The repository pins the exact JDK distribution and release for development and CI. |
| JAVA-008 | No end-of-life Java releases | MUST NOT | always | Safety | SUP-006 | The project does not build with, or target, a Java release that its JDK vendor no longer supports with security fixes. |
| **## Language hazards** | | | | | | |
| JAVA-009 | No unfiltered deserialization | MUST NOT | always | Safety | SEC-011 | Untrusted data does not reach `ObjectInputStream` without a deserialization filter (JEP 290) that allows only the expected classes. |
| JAVA-010 | Turn off external entities in XML parsers | MUST | always | Safety | SEC-009 | Each XML parser that reads untrusted input turns off DTDs and external entities. |

Cite: the `javac` documentation (JDK version), Error Prone, NullAway,
JSpecify 1.0.0, SpotBugs and Find Security Bugs documentation,
google-java-format, JEP 290, the OWASP XML External Entity Prevention Cheat
Sheet (read date), the vendor's Java support roadmap (read date); CWE-502
(JAVA-009), CWE-611 (JAVA-010).

**Maven** — `# Maven`, prefix `MVN`, header `tool.maven`. Free text: the
module holds Maven's own settings, so it names plugin goals and
configuration directly (decision 2).

| ID | Title | Level | Applies when | Category | Refines | Requirement |
|---|---|---|---|---|---|---|
| **## Build** | | | | | | |
| MVN-001 | Pin Maven with the wrapper | MUST | always | Safety | SUP-001 | The project builds through the Maven Wrapper, which names an exact Maven version and the distribution's checksum. |
| MVN-002 | Pin every plugin version | MUST | always | Safety | SUP-001 | Each plugin has an explicit version, enforced with the Enforcer rule `requirePluginVersions`. |
| MVN-003 | Resolve only over HTTPS | MUST | always | Safety | SUP-005 | Each repository and plugin repository uses HTTPS, and checksum checks fail the build. |
| **## Dependencies** | | | | | | |
| MVN-004 | Pin release versions | MUST | has-dependencies | Safety | DEP-001 | Each dependency has a release version (directly or through `dependencyManagement` or a BOM); never `LATEST`, `RELEASE` or a `-SNAPSHOT` version in a released build. |
| MVN-005 | Fail on dependency version conflicts | MUST | has-dependencies | Safety | DEP-001 | The build runs the Enforcer rule `dependencyConvergence` (or `requireUpperBoundDeps`), so conflict mediation never picks a version silently. |
| MVN-006 | Lock and verify every resolved artifact | MUST | has-dependencies | Safety | DEP-002, DEP-003 | A committed lock file records the checksum of every resolved artifact, transitive ones included, and CI fails when the resolved set differs (such as with the maven-lockfile plugin). |
| MVN-007 | Check undeclared and unused dependencies | MUST | has-dependencies | Safety | DEP-004, DEP-013 | CI runs `dependency:analyze` with `failOnWarning`. |
| MVN-008 | Scope non-runtime dependencies | MUST | has-dependencies | Safety | DEP-005 | Test-only dependencies have `test` scope, and dependencies that the runtime provides have `provided` scope. |
| MVN-009 | Scan with OSV-Scanner | SHOULD | has-dependencies | Safety | DEP-007 | The project scans its resolved dependencies with OSV-Scanner, configured per its vendor profile. |

Cite: Apache Maven documentation (Maven version): Maven Wrapper, Enforcer
built-in rules, Dependency Plugin `analyze`, settings and repositories;
the maven-lockfile plugin's documentation (confirm that it is maintained;
otherwise report before writing MVN-006).

**Profiles:**

- **Error Prone** (`google/error-prone.md`, vendor Google). Baseline: Error
  Prone as a `javac` plugin with its default ERROR checks and a stated set
  of WARNING checks raised to ERROR. Guards: suppressions name the check
  (`@SuppressWarnings("CheckName")`); say that it has no reason field. Map:
  JAVA-002, QLT-001, QLT-005.
- **NullAway** (`uber/nullaway.md`, vendor Uber). Baseline: NullAway at
  ERROR, in JSpecify mode, covering all `@NullMarked` code. Guards: none of
  its own (say so). Map: JAVA-005, SAF-001.
- **SpotBugs** (`spotbugs/spotbugs.md`, product "SpotBugs with Find Security
  Bugs"). Baseline: the Maven plugin at `effort` Max and `threshold` Low,
  failing the build, with Find Security Bugs, and an empty exclude filter.
  Guards: `@SuppressFBWarnings` with `justification`. Map: JAVA-004,
  QLT-001, SEC-010, SEC-011, SEC-013, SEC-028.

---

### Task 10: Docker Compose and Kubernetes with Helm, with the kube-linter profile

**Files:** Create `standard/modules/platforms/docker-compose.md`,
`standard/modules/platforms/kubernetes.md`,
`standard/vendors/stackrox/kube-linter.md`.

**Values:** lint `uss lint: ok (410 rules, 52 files)`; commit
`feat(standard): add the Docker Compose and Kubernetes modules and the kube-linter profile`.

**Docker Compose** — `# Docker Compose`, prefix `COMPOSE`, header
`platform.docker-compose`. Free text: the hardening rules judge the Compose
files that the documentation tells people to run; a Compose file only for
local development is judged by COMPOSE-001, COMPOSE-006 and COMPOSE-007.

| ID | Title | Level | Applies when | Category | Refines | Requirement |
|---|---|---|---|---|---|---|
| COMPOSE-001 | Pin each third-party image by digest | MUST | uses-container-images | Safety | SUP-004 | Each third-party `image:` names a digest (`@sha256:…`). |
| COMPOSE-002 | Harden each service | MUST | ships-container-image | Safety | HRD-002 | Each service runs as a non-root user, drops all capabilities and adds back only those it needs with a reason, sets `no-new-privileges`, is not `privileged`, uses no host network, PID or IPC namespace, and has memory and PID limits. |
| COMPOSE-003 | Mount the root filesystem read-only | SHOULD | ships-container-image | Safety | HRD-003 | Each service sets `read_only: true`, with a `tmpfs` or volume for each path that it writes. |
| COMPOSE-004 | Define a health check | MUST | ships-container-image and is-network-service | Reliability | OBS-003 | Each long-running service has a `healthcheck`. |
| COMPOSE-005 | Do not mount the Docker socket | MUST NOT | ships-container-image | Safety | HRD-002 | No service mounts the Docker daemon's socket. |
| COMPOSE-006 | Keep secrets out of Compose files | MUST NOT | always | Safety | SEC-004 | A Compose file in the repository holds no secret value; secrets come from Compose `secrets` or from an environment file outside the repository. |
| COMPOSE-007 | Bind each published port to a stated address | MUST | always | Safety | SEC-002 | Each published port names the host address that it binds to, such as `127.0.0.1:5432:5432`. |

Cite: the Compose Specification (version or read date), Docker Engine
documentation on published ports and the firewall (read date), CIS Docker
Benchmark 1.9.0 (the section of each rule, as HRD-002 cites it); CWE-250.

**Kubernetes and Helm** — `# Kubernetes and Helm`, prefix `K8S`, header
`platform.kubernetes or platform.helm`. Free text: HRD-004 to HRD-008 hold
the hardening fields; this module adds the checks and Helm.

| ID | Title | Level | Applies when | Category | Refines | Requirement |
|---|---|---|---|---|---|---|
| K8S-001 | Lint rendered manifests for hardening | MUST | deploys-to-kubernetes | Safety | HRD-004, HRD-005, HRD-006, HRD-008 | CI renders every chart and manifest and checks the result with a policy linter that covers the hardening rules, and a violation fails the build. |
| K8S-002 | Lint with kube-linter | SHOULD | deploys-to-kubernetes | Safety | HRD-004, HRD-005, HRD-006, HRD-008 | The project lints rendered manifests with kube-linter, configured per its vendor profile. |
| K8S-003 | Validate manifests against the API schemas | MUST | deploys-to-kubernetes | Reliability | | CI validates the rendered manifests against the API schemas of each Kubernetes version that the project supports (such as with kubeconform). |
| K8S-004 | Lint charts strictly | MUST | platform.helm and deploys-to-kubernetes | Maintainability | QLT-001 | CI runs `helm lint --strict` on each chart. |
| K8S-005 | Ship a values schema | SHOULD | platform.helm and deploys-to-kubernetes | Maintainability | | Each chart ships a `values.schema.json` that rejects unknown and mistyped values. |
| K8S-006 | Lock chart dependencies | MUST | platform.helm and deploys-to-kubernetes and has-dependencies | Safety | DEP-001, DEP-002 | Chart dependencies have bounded version ranges, and `Chart.lock` is committed. |
| K8S-007 | Do not mount unneeded service-account tokens | MUST NOT | deploys-to-kubernetes | Safety | HRD-008 | A Pod that does not call the Kubernetes API does not mount a service-account token (`automountServiceAccountToken: false`). |
| K8S-008 | Ship a default-deny network policy | SHOULD | deploys-to-kubernetes | Safety | HRD-007 | The chart or manifests include a NetworkPolicy that denies traffic by default, and policies that allow only what each workload needs. |

Cite: Kubernetes documentation (version), the Pod Security Standards, Helm
documentation (version), kube-linter and kubeconform documentation.

**Profile: kube-linter** (`stackrox/kube-linter.md`, vendor StackRox).
Baseline: `.kube-linter.yaml` with all built-in checks on and no exclusions.
Guards: the `ignore-check.kube-linter.io/<check>` annotation, whose value is
the reason. Map: K8S-001, HRD-003, HRD-004, HRD-005, HRD-006, HRD-008,
K8S-007.

---

### Task 11: PostgreSQL and React, with the Squawk and React lint profiles

**Files:** Create `standard/modules/platforms/postgresql.md`,
`standard/modules/frameworks/react.md`, `standard/vendors/sbdchd/squawk.md`,
`standard/vendors/facebook/eslint-plugin-react-hooks.md`,
`standard/vendors/jsx-eslint/eslint-plugin-jsx-a11y.md`.

**Values:** lint `uss lint: ok (424 rules, 57 files)`; commit
`feat(standard): add the PostgreSQL and React modules and their lint profiles`.

**PostgreSQL** — `# PostgreSQL`, prefix `PG`, header `platform.postgresql`.

| ID | Title | Level | Applies when | Category | Refines | Requirement |
|---|---|---|---|---|---|---|
| PG-001 | Lint migrations for unsafe operations | MUST | stores-persistent-data | Reliability | DAT-002 | CI checks each new migration for operations that lose data or hold long locks, and a finding fails the build. |
| PG-002 | Lint migrations with Squawk | SHOULD | stores-persistent-data | Reliability | DAT-002 | The project lints migrations with Squawk, configured per its vendor profile. |
| PG-003 | Set a lock timeout in migrations | MUST | stores-persistent-data | Reliability | DAT-001 | Each migration sets `lock_timeout`, so it fails instead of blocking other queries. |
| PG-004 | Connect with verify-full TLS | MUST | uses-network | Safety | SEC-022, SEC-023 | Each connection over a network uses `sslmode=verify-full`. |
| PG-005 | Connect with a least-privilege role | MUST | uses-secrets | Safety | SEC-007 | The application connects with a role that owns no objects and has only the privileges it uses; migrations run with a separate role. |
| PG-006 | Set a statement timeout | MUST | uses-network | Reliability | RES-001 | Application connections set `statement_timeout`. |
| PG-007 | No unsupported PostgreSQL versions | MUST NOT | always | Safety | SUP-006 | The project does not run on, or test against, a PostgreSQL major version that the PostgreSQL project no longer supports. |
| PG-008 | Test each supported major version | SHOULD | uses-relational-database | Reliability | TST-004 | CI runs the integration tests against each PostgreSQL major version that the project supports. |

Cite: PostgreSQL documentation (version): client connection parameters,
`libpq` SSL support, roles and privileges, the versioning policy; Squawk
documentation.

**React** — `# React`, prefix `REACT`, header `framework.react`. Free text:
the TypeScript and JavaScript module holds the rest of the lint.

| ID | Title | Level | Applies when | Category | Refines | Requirement |
|---|---|---|---|---|---|---|
| REACT-001 | Lint the Rules of Hooks | MUST | always | Maintainability | QLT-001 | CI lints components and hooks against the Rules of Hooks and hook dependencies, as errors. |
| REACT-002 | Lint with eslint-plugin-react-hooks | SHOULD | always | Maintainability | QLT-001 | The project lints with eslint-plugin-react-hooks, configured per its vendor profile. |
| REACT-003 | Lint JSX for accessibility | MUST | has-web-ui | Accessibility & Usability | ACC-002 | CI lints JSX against accessibility rules, and a violation fails the build. |
| REACT-004 | Lint with eslint-plugin-jsx-a11y | SHOULD | has-web-ui | Accessibility & Usability | ACC-002 | The project lints JSX with eslint-plugin-jsx-a11y, configured per its vendor profile. |
| REACT-005 | No unsanitized HTML | MUST NOT | always | Safety | SEC-014 | `dangerouslySetInnerHTML` receives no value that a vetted HTML sanitizer (such as DOMPurify) has not cleaned. |
| REACT-006 | Render the root in StrictMode | MUST | always | Safety | SAF-001 | The application renders its root inside `<StrictMode>`. |

Cite: React documentation (version): Rules of Hooks, `StrictMode`,
`dangerouslySetInnerHTML`; DOMPurify; CWE-79 (REACT-005).

**Profiles:**

- **Squawk** (`sbdchd/squawk.md`). Baseline: every rule on, no excluded
  rules, the PostgreSQL version set. Guards: the ignore comment names the
  rule; say whether it takes a reason. Map: PG-001, PG-003 (if a rule checks
  it), DAT-002.
- **eslint-plugin-react-hooks** (`facebook/eslint-plugin-react-hooks.md`,
  vendor the React team at Meta). Baseline: its recommended config with
  `rules-of-hooks` and `exhaustive-deps` both errors. Guards: those of the
  ESLint profile apply. Map: REACT-001, QLT-001.
- **eslint-plugin-jsx-a11y** (`jsx-eslint/eslint-plugin-jsx-a11y.md`).
  Baseline: its strict config. Guards: those of the ESLint profile apply.
  Map: REACT-003, ACC-001 (`partial`), ACC-002 (`partial`: needs a check of
  the rendered pages, such as axe-core).

---

### Task 12: OpenSSF Scorecard profile

**Files:** Create `standard/vendors/ossf/scorecard.md`.

**Values:** lint `uss lint: ok (424 rules, 58 files)`; commit
`feat(standard): add the OpenSSF Scorecard profile`.

Recover the coverage notes that Plan 2 cut from the Supply chain Verify
texts: `git show 5ed257a:standard/core/supply-chain.md`, rules draft SUP-004
to SUP-013 (lines 78–295). Draft SUP-0NN is today's SUP-0(NN−1): draft
SUP-004 is USS-SUP-003, and draft SUP-013 is USS-SUP-012.

Baseline: the Scorecard GitHub Action workflow (pinned by commit, results
published, on pushes to the default branch and on a schedule), and the rule
that a check meets a mapped rule only at the score that the mapping names.
Guards: Scorecard's maintainer annotations, if the current version has
them, with their reason field; otherwise say that it has none. Map, each
with the check that gives the evidence: SUP-003 and SUP-004
(Pinned-Dependencies), SUP-007 and SUP-008 (Branch-Protection), SUP-009
(Code-Review), SUP-010 (Token-Permissions), SUP-011 and SUP-012
(Dangerous-Workflow), SEC-028 (SAST), SEC-029 (Security-Policy), DEP-007
(Vulnerabilities), DEP-014 (Dependency-Update-Tool), SUP-014
(Signed-Releases), TST-011 and TST-012 (Fuzzing), LIC-001 (License),
TST-001 (CI-Tests). Most are `partial`: say what the check does not see,
from the recovered notes and the check documentation.

---

### Task 13: EU Cyber Resilience Act

**Files:** Create `standard/modules/regulations/cra.md`; modify
`standard/applicability.md` (`sold-in-eu`).

**Values:** lint `uss lint: ok (433 rules, 59 files)`; floor test still
`(59, 318)`; commit `feat(standard): add the EU Cyber Resilience Act module`.

**`sold-in-eu`:** Definition: "Yes when it is placed on the market of the
European Union: made available as a product, such as installable software,
firmware or a device, in the course of a commercial activity. A service
that the project only runs for its users does not count." Add
`Implies: publishes-releases`.

**Module** — `# EU Cyber Resilience Act`, prefix `CRA`, header `sold-in-eu`.
Free text: the regulation (Regulation (EU) 2024/2847), the dates on which
its obligations apply (confirm: reporting under Article 14 from
2026-09-11; the rest from 2027-12-11), and that the open-source steward and
other special regimes are for a qualified person to judge. `## Disclaimer`:
USS compliance is not legal compliance and not legal advice; this module
shows where USS helps meet the regulation and what is left, for a qualified
person to confirm. `## Obligations`: one row for each obligation below,
each source confirmed from the official text on EUR-Lex.

| Obligation | Source | Rules | Coverage |
|---|---|---|---|
| Design to a risk assessment | Art. 13(2)–(3); Annex I Part I(1) | USS-SEC-001 | partial: the risk assessment in the technical documentation |
| No known exploitable vulnerabilities | Annex I Part I(2)(a) | USS-DEP-007, USS-DEP-008 | partial |
| Secure by default, with a reset | Annex I Part I(2)(b) | USS-SEC-002 | partial: reset to the original state |
| Automatic security updates | Annex I Part I(2)(c) | USS-CRA-008 | partial: where automatic updates do not fit |
| Access control | Annex I Part I(2)(d) | USS-SEC-015, USS-SEC-018 | partial |
| Confidentiality | Annex I Part I(2)(e) | USS-SEC-022, USS-DAT-004 | partial |
| Integrity | Annex I Part I(2)(f) | USS-SEC-009, USS-SUP-014 | partial |
| Data minimisation | Annex I Part I(2)(g) | USS-PRV-002 | partial |
| Availability and resilience | Annex I Part I(2)(h)–(i) | USS-RES-004 | partial |
| Limited attack surface | Annex I Part I(2)(j) | USS-SEC-003 | partial |
| Exploit mitigation | Annex I Part I(2)(k) | USS-SAF-001, USS-SAF-002 | partial |
| Security monitoring | Annex I Part I(2)(l) | USS-AUD-001 | partial |
| Remove data and settings | Annex I Part I(2)(m) | USS-CRA-009 | full |
| SBOM | Annex I Part II(1) | USS-CRA-001, USS-SUP-013 | full |
| Remediate without delay; separate security updates | Annex I Part II(2) | USS-DEP-008, USS-CRA-006 | partial |
| Tests and reviews | Annex I Part II(3) | USS-TST-001, USS-SEC-028 | partial |
| Disclose fixed vulnerabilities | Annex I Part II(4) | USS-SEC-031 | full |
| Coordinated vulnerability disclosure | Annex I Part II(5)–(6) | USS-SEC-029, USS-SEC-030 | full |
| Secure distribution of updates | Annex I Part II(7) | USS-SUP-014, USS-SEC-022 | partial |
| Free security updates with advisories | Annex I Part II(8) | USS-CRA-005, USS-SEC-031 | full |
| Support period | Art. 13(8) | USS-CRA-004, USS-REL-004 | full |
| Updates kept available | Art. 13(9) | USS-CRA-007 | full |
| Report exploited vulnerabilities | Art. 14(1)–(2) | USS-CRA-002 | full |
| Report severe incidents | Art. 14(3)–(4) | USS-CRA-003 | full |
| Information for users | Annex II | USS-CRA-004, USS-SEC-029, USS-DOC-001 | partial |
| Technical documentation | Art. 31; Annex VII | none | none: a qualified person prepares it |
| Conformity assessment, EU declaration, CE marking | Arts. 28, 30, 32 | none | none: a qualified person handles them |

Rules (Category Safety; the module's own rules, no `Refines`):

| ID | Title | Level | Applies when | Requirement |
|---|---|---|---|---|
| CRA-001 | Draw up an SBOM for each release | MUST | always | Each release has an SBOM in a commonly used, machine-readable format (SPDX or CycloneDX) that covers at least the top-level dependencies. |
| CRA-002 | Report actively exploited vulnerabilities | MUST | always | A written procedure reports each actively exploited vulnerability through the single reporting platform within the deadlines of Article 14 (early warning within 24 hours, notification within 72 hours, final report within 14 days of a corrective measure). |
| CRA-003 | Report severe incidents | MUST | always | A written procedure reports each severe incident within the deadlines of Article 14 (24 hours, 72 hours, one month). |
| CRA-004 | State the support period | MUST | always | The product information states the end date of the support period (at least month and year), and the period is at least five years unless the product's expected use is shorter. |
| CRA-005 | Provide security updates free of charge | MUST | always | Security updates during the support period are free of charge and come with advisory messages. |
| CRA-006 | Ship security fixes apart from new features | MUST | always | Where it is technically possible, each security fix is available in a release that adds no new features. |
| CRA-007 | Keep each security update available | MUST | always | Each security update stays available for at least ten years after its release, or for the rest of the support period if that is longer. |
| CRA-008 | Install security updates automatically by default | MUST | has-ui or has-cli | Security updates install automatically by default, with a clear opt-out, a notice to users, and a way to postpone them. |
| CRA-009 | Let users remove all their data and settings | MUST | stores-persistent-data | Users can remove all their data and settings permanently, securely and easily. |

Cite each rule to its article or annex point; confirm each deadline,
duration and date from the official text.

---

### Task 14: European Accessibility Act and GDPR

**Files:** Create `standard/modules/regulations/eaa.md`,
`standard/modules/regulations/gdpr.md`; modify `standard/applicability.md`,
`templates/record.yaml`, `tools/uss/src/catalog/facts.rs`
(`applicability_file_is_valid`: `want` gains `covered-by-eaa`, length 45).

**Values:** lint `uss lint: ok (443 rules, 61 files)`; floor test still
`(59, 318)`; commit `feat(standard): add the European Accessibility Act and GDPR modules`.

**New fact** `covered-by-eaa`, after `processes-eu-personal-data`. Question:
"Is it a product or service that the European Accessibility Act covers?"
Definition: "Yes when it is, or is part of, a product or a service that the
European Accessibility Act covers and that is offered to consumers in the
European Union, such as an e-commerce service, a consumer banking service,
an e-book or its reading software, an electronic communications service, a
service that gives access to audiovisual media, or the operating system of
a consumer computer or phone. A service from a microenterprise does not
count." Confirm the list and the microenterprise exemption from the
Directive. Add it to `templates/record.yaml` in the same order. Before the
shared Step 2, add `covered-by-eaa` to `want` in
`applicability_file_is_valid` (length 45) and run
`cargo test --locked -p uss applicability_file_is_valid`. Expected: FAIL.

**European Accessibility Act** — `# European Accessibility Act`, prefix
`EAA`, header `covered-by-eaa`, Category Accessibility & Usability.
Free text: Directive (EU) 2019/882, applied from 2025-06-28 through each
member state's law; the harmonised standard that gives a presumption of
conformity (confirm which version of EN 301 549 holds that status today).
Disclaimer and obligations as in Task 13: accessibility requirements
(Annex I) → USS-ACC-001, USS-EAA-001; information on how the service meets
them (Annex V) → USS-EAA-002, USS-ACC-005; accessible information and
instructions → USS-EAA-003; support services → `none: …`; conformity and
market surveillance → `none: …`.

| ID | Title | Level | Applies when | Requirement |
|---|---|---|---|---|
| EAA-001 | Meet EN 301 549 | MUST | has-ui | The product or service meets the requirements of EN 301 549 (the version confirmed above) that apply to it. |
| EAA-002 | Publish how the service meets the accessibility requirements | MUST | always | The terms and conditions, or an equivalent public page, say how the service meets the accessibility requirements. |
| EAA-003 | Make product information and instructions accessible | MUST | always | Information on the use of the product and its instructions are available in accessible formats. |

**GDPR** — `# GDPR`, prefix `GDPR`, header `processes-eu-personal-data`,
Category Safety. Free text: Regulation (EU) 2016/679; who is the controller
is for a qualified person to judge; for software that others run, these
rules ask that the software lets its operator meet each right. Disclaimer
and obligations: lawful basis (Art. 6) → `none: …`; consent (Art. 7) →
USS-GDPR-004, USS-GDPR-005; information to people (Arts. 13–14) →
`none: …`; access and portability (Arts. 15, 20) → USS-GDPR-001;
rectification (Art. 16) → USS-GDPR-002; erasure (Art. 17) → USS-GDPR-003,
USS-DAT-005; by design and by default (Art. 25) → USS-PRV-002,
USS-PRV-005, USS-SEC-002 (`partial`); records of processing (Art. 30) →
USS-GDPR-007, USS-PRV-001; security (Art. 32) → USS-SEC-022, USS-DAT-004
(`partial`); breach notification (Arts. 33–34) → USS-GDPR-006; impact
assessment (Art. 35) → `none: …`; transfers (Chapter V) → `none: …`.

| ID | Title | Level | Applies when | Requirement |
|---|---|---|---|---|
| GDPR-001 | Give each person a copy of their data | MUST | always | The software exports all personal data about one person in a structured, commonly used and machine-readable format. |
| GDPR-002 | Let personal data be corrected | MUST | always | The software lets the personal data about one person be corrected. |
| GDPR-003 | Erase one person's data on request | MUST | always | The software erases all personal data about one person on request, in every store that it controls. |
| GDPR-004 | Record each consent | MUST | always | When the software relies on consent, it records who consented, when, and to what. |
| GDPR-005 | Make withdrawing consent as easy as giving it | MUST | always | When the software relies on consent, a person can withdraw it as easily as they gave it. |
| GDPR-006 | Write down the breach notification procedure | MUST | operates-service | A written procedure notifies the supervisory authority of a personal-data breach within 72 hours, and the people affected when Article 34 requires it. |
| GDPR-007 | Keep a record of processing activities | MUST | operates-service | The project keeps the record of processing activities that Article 30 describes. |

GDPR-004 and GDPR-005 Verify: software that asks for no consent passes,
with that as the evidence.

---

### Task 15: Align the spec and hand over

**Files:** Modify
`docs/superpowers/specs/2026-11-21-universal-software-standard-design.md`.

- [ ] **Step 1: Align the spec.**
  - §6.1 TRU row: features once free SHOULD NOT be taken out of the free
    edition or moved behind a payment while a paid edition keeps them;
    removal from every edition is a breaking change (REL); an operated
    service with no releases that removes a feature from every edition is
    judged by no rule.
  - §6.1 SUP row: the SBOM, signing and SLSA rules apply to every project
    that publishes releases.
  - §6.3: the module files and prefixes of this plan; formatters and
    fixed-set scanners are named without a profile; Dockerfiles are a later
    module.
  - §6.6: the seventeen v1 profiles.
  - §6.7: the confirmed CRA dates; the `covered-by-eaa` fact.
  - §5.3: `covered-by-eaa` among the example facts.
- [ ] **Step 2: Gather the numbers.** From `uss export --json`: rules per
  module and per level; profiles and their mapping counts. The floor
  `(59, 318)`.
- [ ] **Step 3: Check every ID in the plan's tables against the export.**
  Expected: each exists, or each difference is in a ruling.
- [ ] **Step 4: Run all checks.** `cargo xtask check`. Expected: exit 0,
  `uss lint: ok (443 rules, 61 files)`.
- [ ] **Step 5: Commit** `docs(spec): align modules, profiles and regulations with Plan 4`.
- [ ] **Step 6: Hand over.** Report: rule counts per module and level; the
  profiles; the floor; the plan-level decisions, marked where drafting
  changed them; each tool that current sources did not support; each
  reference that could not be confirmed; each profile whose baseline was
  not run; and every ruling made during the run, with its cost if wrong.
