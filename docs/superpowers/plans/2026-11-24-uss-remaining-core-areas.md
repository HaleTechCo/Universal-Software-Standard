# USS Remaining Core Areas Implementation Plan (Plan 3 of 5)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Draft the 28 core areas that remain after Plan 2 (spec §6.1), with
the facts that their rules need, as rule files that pass `uss lint`. A project
whose facts are all false must still see at most a quarter of the core rules
(spec §6.4).

**Architecture:** Content, as in Plan 2. Each area is one rule file under
`standard/core/`, in the format that `standard/conventions.md` defines and
`uss lint` enforces. Task 1 adds eight asked facts. Tasks 2–10 each draft a
batch of related areas, point the existing rules at the new IDs, and update
the test that pins the all-false floor. Task 11 adds the budget assertion,
aligns the spec with this plan's decisions, and hands over. The validator's
features do not change; only test values do.

**Tech Stack:** Markdown rule files; the `uss` validator (Rust 1.100.0,
`cargo xtask check`). No new dependencies.

**Spec:** `docs/superpowers/specs/2026-11-21-universal-software-standard-design.md`
(§5.5 score categories, §6.1 the area rows, §6.2 Extensibility, §6.4
rule-writing bar and floor budget, §6.5 check integrity). The normative
rulebook is `standard/conventions.md`: read sections 1–7 before writing any
rule. The three calibrated areas (`standard/core/security.md`,
`supply-chain.md`, `dependencies.md`) are the model for tone, Verify text and
references: read one of them before writing.

**Roadmap:** Plans 1 and 2 are merged and pushed (`main` at 9a07f2d). This is
Plan 3. Plan 4 adds the modules, the vendor profiles (OpenSSF Scorecard among
them: recover its coverage notes with
`git show 5ed257a:standard/core/supply-chain.md`), the regulation modules (the
CRA module adds the SBOM duty) and validator support for them. Plan 5 adds the
AGENTS.md workflows, the skills and this repo's self-audit, then releases
0.1.0.

## Global Constraints

- Keywords: exactly `MUST`, `MUST NOT`, `SHOULD`, `SHOULD NOT`, `MAY` (RFC 2119
  as clarified by RFC 8174). `SHALL`, `SHALL NOT`, `REQUIRED`, `RECOMMENDED`,
  `NOT RECOMMENDED`, `OPTIONAL` are never used. One rule, one level.
- A rule body contains its own keyword and no other keyword, rationale
  included (conventions §3 "Keyword scan"). A keyword wrapped across a line
  break still counts. Put any other keyword only inside a fenced code block.
- Rule IDs: `USS-<PREFIX>-<NNN>`. In each new file the IDs run from 001 in
  file order, with no gaps, exactly as the tables give them.
- Every rule has `Level`, `Applies when`, `Category`, `Verify` and
  `References`, and a body. `Category` is the category name exactly as
  `standard/registry.yaml` writes it: `Safety`, `Reliability`, `Efficiency`,
  `Maintainability`, `Contributability`, `Compatibility`, `Fairness`,
  `Accessibility & Usability`, `Features`.
- The rule-writing bar (spec §6.4, conventions §7): one requirement per rule;
  verifiable from the project's artifacts; language-neutral (tools appear
  only as examples, "such as …"); a `**Rationale:**` paragraph when the
  reason is not self-evident.
- Reference, don't reinvent: where an external standard defines the content
  (WCAG, SemVer, Common Changelog, RFC 9457, OpenAPI, OpenTelemetry, CIS
  Benchmarks, the Kubernetes Pod Security Standards, NIST SP 800-53, the
  OWASP Top 10 for LLM Applications, the MCP specification, and so on), the
  rule requires conformance and cites it, and never restates it.
- References MUST be checked against the current source when the rule is
  written, not cited from memory, and MUST pin a version (conventions §7). A
  page without a version is pinned `(read YYYY-MM-DD)`. A CWE is cited by its
  ID alone. `none` is allowed only where no source applies. The tables name
  what to cite; the implementer confirms the current version and the exact
  section from the source itself.
- One requirement, one home: the Homes table below says which area holds
  each requirement that two areas' scopes share. A rule body may name another
  rule by ID only when that rule already exists in `standard/core/`;
  otherwise it names the area in words.
- Verify text says what an auditor looks at:
  - A requirement in repository-host settings: Verify names the settings or
    API, and says that the auditor records `unverified` without access.
  - A requirement judged from history (each bug fix, each release, each new
    component): Verify uses the changes since the last audit, or the last
    12 months when there is none.
  - A word that needs judgement ("main workflow", "significant decision"):
    Verify gives the auditor a concrete test, as the tables' notes say.
  - A MAY rule: Verify says what adoption means. An adopted MAY is judged
    `pass` or `fail` like any rule.
- Floor budget (spec §6.4): a record whose facts are all false
  (`templates/record.yaml`) sees at most a quarter of the core rules. The
  test `evaluate::tests::all_false_record_floor` in
  `tools/uss/src/evaluate/mod.rs` pins `(floor, core)`; each task sets the
  values it gives.
- The standard's version stays `0.1.0`. It is unreleased; Plan 5 releases it.
- The content of `Initial_Idea.md` and `todo.txt` is never modified.
- Files are UTF-8 with LF line endings. Wrap prose at 80 columns, as the
  existing standard files do.

## Plan-level decisions (refining the spec, for the user's review)

1. **Eight new asked facts** (Task 1), each because a rule needs to scope
   itself and no existing fact can:
   - `publishes-releases` gates REL, DST and the operator-facing BAK rules,
     so software that is never released (an internal service, a script) is
     not asked about SemVer or install methods.
   - `has-memory-unsafe-code` gates the memory-safety rules (SAF-002 to
     SAF-006) and level-scales fuzzing (TST-011, TST-012).
   - `uses-concurrency` gates the race-detector rule.
   - `parses-untrusted-input` gates fuzzing.
   - `handles-personal-data` (implies `handles-sensitive-data`) gates the
     privacy inventory and retention rules; credential data alone needs no
     inventory.
   - `collects-telemetry` gates the opt-in telemetry rules.
   - `has-ai-tool-use` gates least-privilege tools and confirmation of
     destructive actions. `exposes-mcp-server` implies it, so the AI area
     applies when `uses-ai-models or has-ai-tool-use`.
   - `is-developer-tool` gates the IDE-extension MAY.
2. **Two facts set by MAY rules** (Task 9): `has-plugin-interface` (by
   USS-EXT-001) and `has-plugin-marketplace` (by USS-EXT-008), as Plan 1
   planned.
3. **Install methods are MAY rules without `Enables`**, gated by the kind of
   software, so a library is never asked about a Helm chart. They cannot
   enable `ships-container-image`, `deploys-to-kubernetes` or
   `exposes-mcp-server`, because those facts are asked, and a fact cannot be
   both asked and enabled (conventions §5). Instead, the Verify of DST-003,
   DST-005 and DST-007 checks that the record answers the matching fact
   `true` when the MAY is adopted.
4. **Data export and import are MAY rules** (Features), as spec §5.5 says
   ("data-portability MAYs"). For commercial software, TRU-004 still makes
   export work whatever the subscription state.
5. **LIC's default category is Compatibility.** The spec §5.5 table gives LIC
   no category; a licence decides whether others can use and integrate the
   software. Task 11 adds LIC to that table.
6. **LIC-002 (OSI-approved licence) is a SHOULD for all released software.**
   A proprietary product records a Standard Deviation, as the spec's row
   reads. "Preferring permissive" stays guidance in the rationale, not a
   scored rule, so copyleft projects are not marked down.
7. **Level scaling by facts**, as for ASVS: fuzzing parsers of untrusted
   input is a MUST in memory-unsafe code (TST-011) and a SHOULD elsewhere
   (TST-012).
8. **Deceptive design (UX-004) counts toward Fairness**, not Accessibility
   & Usability: it is about honesty toward users.
9. **What is not a project rule:** integer-overflow checks come through
   SAF-001 (the language modules name the flags); each tool's own
   suppression guards come through the vendor-profile baseline (QLT-006); the
   list of every suppression is part of the audit workflow (Plan 5) and the
   results schema, not a rule; the MAJOR version and the breaking-change flag
   come from SemVer (REL-001) and Common Changelog (REL-005), which REL
   requires, so REL does not restate them.
10. **Numbers USS adds**, where the spec says only "a schedule" or
    "generous": restore tests at least quarterly (BAK-011, as CIS Controls
    11.5); manual accessibility tests per release or quarterly (ACC-003); a
    licence grace period of at least 30 days (COM-013); at least three
    licence models (COM-015).
11. **Area headers.** API, DAT, REL, DST, HRD, OPS, AUD, AI, ACC and COM
    have a file-level `Applies when`, because every rule in them shares the
    condition.
12. **The floor after this plan:** 59 of 317 core rules (18.6%), within the
    budget of 79.

## Homes

Where two areas' scopes share a requirement, it lives in one rule; the other
area names it (by ID once it exists) or leaves it out.

| Requirement | Home | Elsewhere |
|---|---|---|
| Check integrity: rules turned off, suppressions, skipped tests, coverage exclusions | QLT-006 to QLT-014 | SEC-005 and SEC-028 point to them; CON and TST have none |
| CI decides whether a change is done | CON-003, with SUP-008 | — |
| Tests run in CI | TST-001 | — |
| Accessibility testing | ACC-002, ACC-003 | not in TST |
| Fuzzing | TST-011, TST-012 | not in SAF |
| Sanitizer and race-detector runs | SAF-004, SAF-007 | not in TST |
| Toolchains installed per project | CTB-002 | SUP intro points to it |
| Development setup and how to run checks | CTB-001, CTB-003, CTB-006 | not in DOC |
| Security policy content | SEC-029, SEC-030 | not in DOC |
| Changelog | REL-005, REL-006 | not in DOC |
| Install, upgrade and uninstall steps | DST-009 | not in DOC |
| Migration guide for a breaking release | REL-008 | — |
| Settings that turn security off, documented | DOC-004 | SEC-002 points to it |
| Signing release artifacts | SUP-014 | not in REL or DST |
| How to verify signatures | DST-015 | SUP-014 points to it |
| The project's own images by digest | DST-016 | SUP-004 points to it |
| Dependency licence compatibility | LIC-003 | DEP-012 points to it |
| Data migrations run on upgrade; tested | DAT-001; TST-007 | — |
| Configuration carried across upgrades | DST-011 | — |
| Backup before an upgrade | BAK-003 | DST-012 points to it |
| Health and readiness endpoints; probes in manifests | OBS-003; HRD-006 | — |
| Personal-data retention | PRV-003 | AUD-007, BAK-012 point to it |
| Secrets and personal data kept out of logs | PRV-004 | — |
| Audit logging at no extra charge | TRU-003 | not in AUD |
| Instructions in Simplified Technical English | DOC-007 | OPS-003 does not repeat it |
| Fail closed (security); atomic writes (data) | SEC-012; RES-006 | — |
| API version; release version | API-002; REL-001 | — |

## Review Focus

1. **A small project sees only the rules that fit it** (spec §2 anti-goal).
   The all-false floor stays within a quarter of the core rules. Pinned per
   task by `all_false_record_floor`; Task 11 adds the budget assertion.
2. **A MAY rule asked of software that it does not fit**, such as a library
   asked whether it offers a Helm chart. A project must decide every
   applicable MAY, so a badly gated MAY costs every such project a question
   and a Features point. Pinned in Task 11 Step 3 (a library profile sees
   only USS-EXT-001).
3. **One requirement in two rules**, within this plan or against SEC, SUP and
   DEP. Each task's review checks its rules against the Homes table and the
   existing areas.
4. **A rule an auditor cannot decide**: "main workflows", "significant
   decisions", "security-relevant code". Each task's review checks that
   Verify turns each such word into a concrete test.
5. **A rule that names an ID that does not exist**, such as a typo or a
   forward reference. `uss lint` does not check IDs in prose. Pinned in
   Task 11 Step 4.

---

### Task 1: New asked facts

**Files:**
- Modify: `standard/applicability.md`
- Modify: `templates/record.yaml`
- Modify: `tools/uss/src/catalog/facts.rs` (test `applicability_file_is_valid`)

**Interfaces:**
- Produces these asked facts, used by Tasks 2–10: `publishes-releases`,
  `is-developer-tool`, `has-memory-unsafe-code`, `uses-concurrency`,
  `parses-untrusted-input`, `handles-personal-data`, `collects-telemetry`,
  `has-ai-tool-use`.

Place each new fact after the fact named in "After":

| Fact | After | Question | Definition (Yes when …) | Implies |
|---|---|---|---|---|
| collects-telemetry | uses-network | Does it send usage or diagnostic data to the project or a third party? | it sends data about how it is used, or crash and diagnostic reports, to the project or to a third party, rather than only to its own operator's systems | uses-network |
| handles-personal-data | handles-sensitive-data | Does it handle personal data? | it collects, stores or processes information about an identified or identifiable person, such as a name, an email address, an account identifier or a location | handles-sensitive-data |
| has-memory-unsafe-code | high-assurance | Does any of its own code run without memory safety? | the project's own code includes a language that does not guarantee memory safety (such as C, C++ or assembly), or uses the unsafe features of a memory-safe language (such as Rust `unsafe` blocks, Go's `unsafe` package or C# `unsafe` code). Code in dependencies does not count | |
| uses-concurrency | has-memory-unsafe-code | Does its code run in parallel and share data? | its own code runs threads, goroutines, parallel tasks or processes that share memory or other mutable data. Asynchronous code on a single thread does not count | |
| parses-untrusted-input | uses-concurrency | Does it parse data that an untrusted source could have written? | it parses files, documents, images, archives, network messages or other structured input that a person or system it does not trust could have written. Configuration that only its operator writes does not count | |
| is-developer-tool | is-library | Is it a tool that software developers use in their work? | its main users are software developers, who use it while they write, build, test or ship code, such as a compiler, a linter, a build tool, a test framework or a command-line tool for a developer service | |
| publishes-releases | distributed-as-package | Does it publish versioned releases? | the project publishes numbered versions of its software for others to install, run or build on, such as packages, binaries, container images, charts or tagged source releases. A service that only the project itself deploys does not count | |
| has-ai-tool-use | exposes-mcp-server | Can an AI model make it take actions? | output from an AI model can make the software take an action, such as calling a tool, a function or an API, running a command or changing data, rather than only being shown to a person. This includes software that runs the model itself and software that offers tools to an outside agent, such as an MCP server | |

Add to the `Implies` of existing facts (keep what they already have,
comma-separated):

| Existing fact | Add to Implies |
|---|---|
| distributed-as-package | publishes-releases |
| ships-container-image | publishes-releases |
| exposes-mcp-server | has-ai-tool-use |
| processes-eu-personal-data | handles-personal-data |

- [ ] **Step 1: Update the test first**

In `applicability_file_is_valid`, add the eight IDs to `want` in the file
order that the "After" column gives, and change `assert_eq!(want.len(), 36);`
to `assert_eq!(want.len(), 44);`.

- [ ] **Step 2: Run the test to verify it fails**

Run: `cargo test --locked -p uss applicability_file_is_valid`
Expected: FAIL (the eight facts do not exist yet).

- [ ] **Step 3: Add the facts and the `Implies` lines to `standard/applicability.md`**

Follow the existing style: `### <fact-id>`, then `- **Question:**`,
`- **Definition:** Yes when …`, and `- **Implies:**` where the tables say so.
No RFC 2119 keywords in this file.

- [ ] **Step 4: Add the eight facts to `templates/record.yaml`**

Same order as `applicability.md`. Each has the template's existing form:
`# <Question>`, `# Answer this. false is not a default.`, then `<fact>: false`.

- [ ] **Step 5: Run all checks**

Run: `cargo xtask check`
Expected: exit 0; `applicability_file_is_valid`,
`record_template_matches_standard` and `all_false_record_floor` pass (the
floor is still `(21, 66)`); `uss lint: ok (66 rules, 3 files)`.

- [ ] **Step 6: Commit**

```bash
git add standard/applicability.md templates/record.yaml tools/uss/src/catalog/facts.rs
git commit -m "feat(standard): add facts for releases, memory safety, personal data, telemetry and AI tools"
```

---

## How to write each area (Tasks 2–10)

The tables fix each rule's ID, title, level, `Applies when` and requirement.
The implementer writes the body, the Verify text and the rationale, and
confirms each reference.

**Interfaces for every area task:** each consumes the facts in
`standard/applicability.md` (Task 1's included) and the rules already in
`standard/core/`; each produces the rule IDs in its tables. A body cites a
rule by ID only when an earlier task, or the same task, creates it; the
tables mark each such citation, such as "(USS-PRV-003)".

- **Header:** `# <Title>`, then `- **Prefix:** <PREFIX>`,
  `- **Kind:** area`, and `- **Applies when:** <expr>` when the task gives one.
  With a header condition, the table's `Applies when` is the rule's own field
  (`always` adds nothing), and the rule's effective condition is the header
  ANDed with it.
- **Free text:** one short paragraph after the header: what the area covers,
  its default category, and which areas hold the neighbouring requirements
  (from the Homes table).
- **Groups:** `##` headings exactly as the tables' bold rows give them.
- **Each rule:** the heading `### USS-<PREFIX>-<NNN>: <title>` with the
  table's title; the five fields (plus `Enables` where the table says so); a
  body whose first sentence states the table's requirement with the rule's
  keyword; a `**Rationale:**` paragraph unless the reason is self-evident.
  `Category` is the area's default unless the task lists an override.
- **References:** confirm each one from its source (web fetch). Pin the
  version and name the section where one exists. If a source does not say
  what the table expects, report it instead of citing it.

Each area task runs these steps, with the values the task gives:

- [ ] **Step 1: Confirm the references.** For each rule, fetch the sources
  the task names and note the current version and section. Write the list
  (rule ID, reference as cited, source URL) into the task report. Flag any
  row whose source does not support the requirement.
- [ ] **Step 2: Update the floor test first.** In `all_false_record_floor`,
  set `assert_eq!((floor, core), …)` to the task's values. Run
  `cargo test --locked -p uss all_false_record_floor`. Expected: FAIL.
- [ ] **Step 3: Write the area files** from the tables.
- [ ] **Step 4: Point the existing rules at the new IDs**, where the task
  lists such edits. Keep each sentence's meaning; change only the pointer.
- [ ] **Step 5: Lint.** Run `cargo run --locked -p uss -- lint`. Expected:
  the task's `uss lint: ok (…)` line.
- [ ] **Step 6: Compare the export with the tables.** Run
  `cargo run --locked -p uss -- export --json`. Expected: for each new rule,
  `id`, `title`, `level` and `category` equal the table, and `appliesWhen` is
  equivalent to the header ANDed with the table's condition (the exporter
  adds parentheses).
- [ ] **Step 7: Check line lengths.** Run
  `awk 'length > 80 { print FILENAME ":" FNR }' <the task's new files>`.
  Expected: no output.
- [ ] **Step 8: Run all checks.** Run `cargo xtask check`. Expected: exit 0,
  `all_false_record_floor` passes, and the lint step prints the task's line.
- [ ] **Step 9: Commit** with the task's message, adding the task's files,
  the edited existing area files and `tools/uss/src/evaluate/mod.rs`.

---

### Task 2: Conduct, Code quality, Language and runtime safety

**Files:**
- Create: `standard/core/conduct.md`, `standard/core/code-quality.md`,
  `standard/core/runtime-safety.md`
- Modify: `standard/core/security.md` (SEC-005 and SEC-028 pointers)
- Modify: `tools/uss/src/evaluate/mod.rs` (floor test)

**Values:** floor test `(39, 90)`; lint `uss lint: ok (90 rules, 6 files)`;
commit `feat(standard): draft the Conduct, Code quality and Language and runtime safety areas`.

**Pointers:** in SEC-005 and SEC-028, replace "Suppressions follow the
standard's check-integrity rules." with a sentence that names
USS-QLT-006 to USS-QLT-014.

**Conduct** — `# Conduct`, prefix `CON`, default category Maintainability.
The free text says that conduct which leaves no artifact (no silent scope
reduction, no fabricated success reports, no out-of-scope changes) is not an
audited rule; it belongs in AGENTS.md's "Agent conduct" section (spec §6.1).

| ID | Title | Level | Applies when | Requirement |
|---|---|---|---|---|
| CON-001 | No placeholder code | MUST NOT | always | No production code path holds a stub, a placeholder, a fake result or a "not implemented" branch, unless the user documentation names that feature as unsupported. |
| CON-002 | Write down the Definition of Done | MUST | always | The repository documents the Definition of Done: the conditions that every change meets before it merges. |
| CON-003 | Check the Definition of Done in CI | MUST | always | Each condition of the Definition of Done that a tool can check runs as a required CI check, so CI, not a person's or an agent's report, decides whether a change is done. |

Cite: The Scrum Guide (November 2020), Definition of Done; NIST SP 800-218
(SSDF 1.1) PO.4. CON-001: `none` unless a source is found.

**Code quality** — `# Code quality`, prefix `QLT`, default category
Maintainability. Override: QLT-008 Safety. The free text says that these
rules apply to every kind of check (linters, compilers, type checkers,
security scanners, tests and coverage), per spec §6.5.

| ID | Title | Level | Applies when | Requirement |
|---|---|---|---|---|
| **## Checks** | | | | |
| QLT-001 | Lint with zero violations | MUST | always | CI runs a linter or static analyser for each language in the project, and any violation fails the build. |
| QLT-002 | Check formatting in CI | MUST | always | A formatter's configuration sets the code's format, and CI fails on code that the formatter would change. |
| QLT-003 | Fail the build on warnings | MUST | always | CI treats the warnings of every compiler, type checker and build tool as errors. |
| QLT-004 | No dead code | MUST NOT | always | The code contains no unreachable code, no unused functions, types or variables, and no commented-out code. |
| QLT-005 | No swallowed errors | MUST NOT | always | No error is caught or returned and then ignored: each one is handled, passed on or reported, or a comment at that line says why ignoring it is safe. |
| **## Check integrity** | | | | |
| QLT-006 | Configure checks at least as strictly as the baseline | MUST | always | Each check's configuration enables at least the baseline that its USS vendor profile defines; with no profile, the rule set that the language module names; with neither, the tool's own recommended set. |
| QLT-007 | Give a reason for each rule turned off | MUST | always | Each rule of a check that is turned off for the whole project has its reason written next to it in the configuration. |
| QLT-008 | Keep security rules on | SHOULD NOT | always | No rule in a security group of a check (such as Ruff's Bandit rules or gosec) is turned off for the whole project. |
| QLT-009 | Name the rule and the reason in each suppression | MUST | always | Each inline suppression names the specific rule that it suppresses and gives a reason on the same line. |
| QLT-010 | No blanket suppressions | MUST NOT | always | No suppression covers all rules, or a whole file. |
| QLT-011 | Use suppressions that expire | SHOULD | always | Where the language offers a suppression that fails when the problem is gone (such as Rust `#[expect]` or TypeScript `@ts-expect-error`), suppressions use it. |
| QLT-012 | Fail on stale suppressions | MUST | always | A suppression that no longer suppresses anything fails the check. |
| QLT-013 | Link each skipped test to an issue | MUST | always | Each skipped or disabled test names a tracked issue. |
| QLT-014 | Give a reason for each coverage exclusion | MUST | always | Each line, block or file excluded from coverage measurement has its reason written next to it. |

Cite: CWE-561 (QLT-004); CWE-252, CWE-390 (QLT-005); the tools' own guards
as examples (Ruff `PGH004` and `RUF100`, golangci-lint `nolintlint`, ESLint
`--report-unused-disable-directives`, Clippy
`allow_attributes_without_reason`, mypy `warn_unused_ignores`), confirmed in
each tool's documentation. QLT-006 also names the vendor profiles of spec §6.6
(Plan 4 writes them) in words.

**Language and runtime safety** — `# Language and runtime safety`, prefix
`SAF`, default category Safety. The free text says that the language modules
name the exact flags and tools.

| ID | Title | Level | Applies when | Requirement |
|---|---|---|---|---|
| SAF-001 | Turn on the toolchain's safety checks | MUST | always | Every safety check, strict mode and hardening option that the language's module names is on; for a language with no module, every one that the toolchain documents as a safety or strict option (such as integer-overflow checks or a type checker's strict mode). |
| SAF-002 | Harden native binaries | MUST | has-memory-unsafe-code | Native code is built with the options that the OpenSSF Compiler Options Hardening Guide for C and C++ recommends, or that the language module names. |
| SAF-003 | Justify each use of unsafe code | MUST | has-memory-unsafe-code | In a memory-safe language, each use of an unsafe feature is as small as possible and has a comment that says why it is sound. |
| SAF-004 | Run sanitizers in CI | MUST | has-memory-unsafe-code | CI runs the tests with the memory-error and undefined-behaviour sanitizers that the toolchain offers (such as AddressSanitizer and UndefinedBehaviorSanitizer, or Miri for Rust). |
| SAF-005 | Publish a memory-safety roadmap | SHOULD | has-memory-unsafe-code | A published roadmap names the memory-unsafe components, and the plan and dates to move them to a memory-safe language or to isolate them. |
| SAF-006 | Write new components in a memory-safe language | SHOULD | has-memory-unsafe-code | Each new component is written in a memory-safe language. |
| SAF-007 | Run a race detector in CI | SHOULD | uses-concurrency | CI runs the tests of concurrent code with a data-race detector where the toolchain offers one (such as Go's `-race` or ThreadSanitizer). |

Cite: OpenSSF Compiler Options Hardening Guide for C and C++ (read date);
CISA and partners, The Case for Memory Safe Roadmaps (2023-12); CISA and FBI,
Product Security Bad Practices (confirm version); NSA, Software Memory Safety
(confirm version); CWE-119, CWE-787, CWE-416 (SAF-002 to SAF-006); CWE-190
(SAF-001); CWE-362 (SAF-007); the Clang sanitizer and Miri documentation as
examples.

---

### Task 3: Architecture, Documentation, Licensing, Contributability

**Files:**
- Create: `standard/core/architecture.md`, `standard/core/documentation.md`,
  `standard/core/licensing.md`, `standard/core/contributability.md`
- Modify: `standard/core/security.md` (SEC-002), `standard/core/dependencies.md`
  (DEP-012), `standard/core/supply-chain.md` (the intro)
- Modify: `tools/uss/src/evaluate/mod.rs` (floor test)

**Values:** floor test `(54, 123)`; lint
`uss lint: ok (123 rules, 10 files)`; commit
`feat(standard): draft the Architecture, Documentation, Licensing and Contributability areas`.

**Pointers:** SEC-002 "Documenting the settings that turn a feature off is a
Documentation (DOC) rule." names USS-DOC-004. DEP-012 "Licence compatibility
is a Licensing (LIC) rule." names USS-LIC-003. The supply-chain intro
"Installing toolchains per project, rather than for the whole system, is a
Contributability (CTB) rule." names USS-CTB-002.

**Architecture** — `# Architecture`, prefix `ARC`, default category
Maintainability.

| ID | Title | Level | Applies when | Requirement |
|---|---|---|---|---|
| ARC-001 | Record significant decisions | MUST | always | Each significant technical decision has an architecture decision record (ADR) in the repository that states the context, the options considered and the reason for the choice. |
| ARC-002 | Keep configuration out of the code | MUST | is-network-service or operates-service or ships-container-image | Every setting that differs between deployments (such as addresses, credentials, limits and feature switches) is read from the environment or a configuration file, not set in code. |
| ARC-003 | Keep service instances stateless | SHOULD | is-network-service | A service instance keeps no state that another instance needs in its own memory or disk; shared state lives in a backing service, so any instance can serve any request. |
| ARC-004 | Do not split into services without a reason | SHOULD NOT | is-network-service | The software is not split into separately deployed services unless an ADR records why the split is needed. |
| ARC-005 | Check module boundaries in CI | SHOULD | always | The allowed dependencies between the project's modules are written down, and a CI check fails a change that breaks them. |

ARC-001 Verify: "significant" means the choice of languages and main
frameworks, each data store, each external service the software depends on,
the split into deployable parts, and any other choice the project names as
costly to reverse. Cite: MADR 4.0.0; Michael Nygard, Documenting
Architecture Decisions (2011-11-15); The Twelve-Factor App, III Config and
VI Processes (read date); ArchUnit, import-linter and dependency-cruiser as
examples.

**Documentation** — `# Documentation`, prefix `DOC`, default category
Maintainability. Override: DOC-007 Accessibility & Usability.

| ID | Title | Level | Applies when | Requirement |
|---|---|---|---|---|
| DOC-001 | Keep a README | MUST | always | The repository root has a README that says what the software does, how to install or build it, and how to start using it. |
| DOC-002 | Describe the architecture | SHOULD | always | An architecture document (such as ARCHITECTURE.md) names the main components, says what each does and how they interact, and says where each lives in the repository. |
| DOC-003 | Document every setting | MUST | has-cli or has-ui or is-network-service | A configuration reference documents every setting (option, environment variable or configuration key) with its default and its effect. |
| DOC-004 | Mark the settings that weaken security | MUST | has-cli or has-ui or is-network-service | The configuration reference marks each setting that turns off or weakens a security feature, and states the risk. |
| DOC-005 | Document the public API | MUST | is-library | Every public item of the library has reference documentation, generated from the code or kept next to it. |
| DOC-006 | Write a troubleshooting guide | SHOULD | publishes-releases and (has-cli or has-ui or is-network-service) | The user documentation lists common errors and failure modes, each with its cause and its fix. |
| DOC-007 | Write instructions in Simplified Technical English | SHOULD | always | Every set of steps for a person or an agent to follow (such as install, upgrade and runbook steps, and AGENTS.md workflows) is written in ASD-STE100 Simplified Technical English. |

DOC-007 Verify, as spec §6.1 says: without the specification the auditor
checks STE's writing rules (one instruction per sentence, imperative, active
voice, sentence-length limits); full dictionary conformance is `unverified`
unless the auditor has the specification or an STE checker. The rule
references ASD-STE100 and does not reproduce it. Cite: ASD-STE100 (confirm
the current issue and how it is distributed); CISA Secure by Design
(2023-10-25), "loosening guides" (DOC-004); matklad, ARCHITECTURE.md
(2021-02-06) as an example for DOC-002.

**Licensing** — `# Licensing`, prefix `LIC`, default category Compatibility.

| ID | Title | Level | Applies when | Requirement |
|---|---|---|---|---|
| LIC-001 | Declare the licence | MUST | always | The repository root has a licence file, and each package manifest declares the licence as an SPDX licence expression. |
| LIC-002 | Use an OSI-approved licence | SHOULD | publishes-releases | The software is released under a licence that the Open Source Initiative has approved. |
| LIC-003 | Allow only compatible dependency licences | MUST | has-dependencies | A written policy lists the dependency licences that are compatible with the project's licence and the way it is distributed, and CI fails on a dependency whose licence the policy does not allow. |
| LIC-004 | Ship the notices that dependencies require | MUST | has-dependencies and publishes-releases | Each release artifact includes the licence texts and notices that its dependencies' licences require. |

LIC-002's rationale says that a permissive licence (such as MIT) is the usual
choice, and that a proprietary product records a Standard Deviation. Cite:
SPDX Specification 3.0.1 (licence expressions); the SPDX License List
(version); OSI Approved Licenses (read date); ISO/IEC 5230:2020 (OpenChain);
cargo-deny, ScanCode and license-checker as examples.

**Contributability** — `# Contributability`, prefix `CTB`, default category
Contributability.

| ID | Title | Level | Applies when | Requirement |
|---|---|---|---|---|
| **## Setup and checks** | | | | |
| CTB-001 | Set up the development environment with one command | MUST | always | One documented command, or a dev container, sets up a complete development environment from a fresh clone. |
| CTB-002 | Install toolchains per project | SHOULD | always | Toolchains and build tools are installed for the project only (such as through a version manager, a wrapper script or a dev container), not system-wide. |
| CTB-003 | Run the CI checks locally with one command | MUST | always | One documented command runs locally the same checks that CI requires. |
| CTB-004 | Run fast checks before each commit | SHOULD | always | The repository provides pre-commit hooks that run the fast checks that CI runs, such as formatting and lint. |
| CTB-005 | State how long the checks take | SHOULD | always | The contributing guide states how long the one-command checks take on a typical machine, and the checks stay within that time. |
| **## Guides** | | | | |
| CTB-006 | Write a contributing guide | MUST | always | A CONTRIBUTING.md covers setup, the workflow for a change, how to run the checks and what makes a change acceptable. |
| CTB-007 | Write an AGENTS.md | SHOULD | always | An AGENTS.md gives coding agents the exact setup, build, test and check commands and the project's conventions. |
| **## External contributions** | | | | |
| CTB-008 | Adopt a code of conduct | MUST | accepts-external-contributions | The repository has a code of conduct, such as the Contributor Covenant, that says how to report a problem. |
| CTB-009 | Use DCO sign-off, not a CLA | SHOULD | accepts-external-contributions | Contributors certify their right to contribute by Developer Certificate of Origin sign-off, which CI checks, and no contributor licence agreement is required. |
| CTB-010 | Provide issue and pull request templates | SHOULD | accepts-external-contributions | The repository has templates for bug reports, feature requests and pull requests. |
| CTB-011 | Label issues for newcomers | SHOULD | accepts-external-contributions | Issues that suit a first contribution carry a label that says so (such as `good first issue`), and the contributing guide names the label. |
| CTB-012 | Run CI on pull requests from forks | MUST | accepts-external-contributions | CI runs the required checks on pull requests from forks. |
| CTB-013 | Document the review process | SHOULD | accepts-external-contributions | The contributing guide says how changes are reviewed and how soon a contributor can expect a first response. |
| CTB-014 | Document governance and maintainers | SHOULD | accepts-external-contributions and has-multiple-maintainers | A GOVERNANCE.md says how decisions are made and how people become maintainers, and a MAINTAINERS file lists the current maintainers. |
| CTB-015 | Credit contributors | SHOULD | accepts-external-contributions | Each contributor is credited, such as in the release notes or a contributors list. |
| **## Continuity** | | | | |
| CTB-016 | Give two people admin and release access | SHOULD | always | At least two people have admin access to the repository and can publish a release. |
| CTB-017 | Document a succession plan | SHOULD | always | A documented plan says who takes the project over, or how it is archived and its users told, if its maintainers leave. |

CTB-012's body says that USS-SUP-011 keeps secrets away from that code.
CTB-016's body says that a backup who takes no part in reviews counts here,
unlike for the fact `has-multiple-maintainers`; its Verify names the host's
access settings and records `unverified` without access. Cite: OpenSSF Best
Practices Badge criteria (read date; the continuity criteria such as
`access_continuity` and `bus_factor`); CHAOSS Bus Factor metric (read date);
Development Containers Specification (containers.dev, read date); AGENTS.md
(agents.md, read date); Contributor Covenant (confirm version); Developer
Certificate of Origin 1.1; pre-commit as an example.

---

### Task 4: APIs, Data, Privacy, Testing

**Files:**
- Create: `standard/core/apis.md`, `standard/core/data.md`,
  `standard/core/privacy.md`, `standard/core/testing.md`
- Modify: `tools/uss/src/evaluate/mod.rs` (floor test)

**Values:** floor test `(58, 154)`; lint
`uss lint: ok (154 rules, 14 files)`; commit
`feat(standard): draft the APIs, Data, Privacy and Testing areas`.

**APIs** — `# APIs`, prefix `API`, `Applies when: exposes-network-api`,
default category Compatibility.

| ID | Title | Level | Applies when | Requirement |
|---|---|---|---|---|
| API-001 | Publish a machine-readable contract | MUST | always | Each network API has a machine-readable contract in the repository (such as OpenAPI, AsyncAPI, a protobuf definition or a GraphQL schema) that matches the implementation. |
| API-002 | Version the API | MUST | always | The API has a version, and each breaking change to it is released under a new major API version. |
| API-003 | Keep old versions through a deprecation period | SHOULD | always | After a new major API version is released, the previous one keeps working for a documented period, and its responses signal the deprecation and the end date. |
| API-004 | Use a standard error format | MUST | always | Errors use one documented format: RFC 9457 problem details for HTTP APIs, or the protocol's own error model (such as gRPC status or GraphQL errors). |
| API-005 | Paginate unbounded collections | MUST | always | Each operation that returns a collection that can grow without limit returns it in pages, with a maximum page size. |
| API-006 | Make writes safe to retry | SHOULD | always | Each operation that creates or changes state is idempotent, or accepts an idempotency key, so a client can retry it safely. |

Cite: OpenAPI Specification (confirm the current 3.x version); AsyncAPI
3.0.0; RFC 9457; RFC 9745 (Deprecation header) and RFC 8594 (Sunset header);
the IETF Idempotency-Key header draft (confirm the current version); the gRPC
status codes and GraphQL specification (October 2021) as examples.

**Data** — `# Data`, prefix `DAT`, `Applies when: stores-persistent-data`,
default category Reliability. Overrides: DAT-004 Safety; DAT-006 and DAT-007
Features.

| ID | Title | Level | Applies when | Requirement |
|---|---|---|---|---|
| **## Changing stored data** | | | | |
| DAT-001 | Change stored data only through versioned migrations | MUST | always | Each change to the structure of stored data (a database schema or a file format) is made by a versioned migration in the repository, which runs automatically on upgrade. |
| DAT-002 | No data loss in automatic migrations | MUST NOT | always | A migration that runs automatically never deletes stored data (drops a table, a column or a field, or truncates); deleting data needs a separate step that an operator runs on purpose. |
| DAT-003 | Document the data model | SHOULD | always | The structure of stored data is documented, such as by schema files or generated documentation. |
| **## Protecting and returning data** | | | | |
| DAT-004 | Encrypt sensitive data at rest | MUST | handles-sensitive-data | Sensitive data is encrypted where it is stored, by the storage layer or by the software. |
| DAT-005 | Delete a user's data on request | MUST | stores-user-data | The software can delete all the data of one user, including copies in indexes and caches, when the user, or an operator acting for them, asks. |
| DAT-006 | Offer data export | MAY | stores-user-data | Users can export all their data in a documented, open format. |
| DAT-007 | Offer data import | MAY | stores-user-data | Users can import data in the format that the export writes. |

Cite: CWE-311, CWE-312 (DAT-004); OWASP ASVS 5.0, the data-protection
chapter (confirm its number); NIST SP 800-111; Martin Fowler, ParallelChange
(expand and contract, read date) as an example for DAT-002.

**Privacy** — `# Privacy`, prefix `PRV`, default category Safety.

| ID | Title | Level | Applies when | Requirement |
|---|---|---|---|---|
| PRV-001 | Keep a personal-data inventory | MUST | handles-personal-data | A maintained inventory lists each kind of personal data that the software handles, with its purpose, where it is stored and how long it is kept. |
| PRV-002 | Collect only what each purpose needs | MUST | handles-personal-data | The software collects no personal data beyond what the purposes in the inventory need. |
| PRV-003 | Delete personal data when its retention ends | MUST | handles-personal-data and stores-persistent-data | Stored personal data is deleted automatically when its retention period in the inventory ends. |
| PRV-004 | Keep sensitive data out of logs and telemetry | MUST NOT | handles-sensitive-data or uses-secrets | Logs, traces, metrics, crash reports and telemetry never hold secrets, credentials or other sensitive data in clear text. |
| PRV-005 | Make telemetry opt-in | MUST | collects-telemetry | Telemetry is off until the user turns it on. |
| PRV-006 | Document what telemetry sends | MUST | collects-telemetry | The documentation lists every item that telemetry sends, and where it sends it. |

PRV-004's body says that the identities an audit record must name (the
Audit logging area) are outside it. Cite: NIST Privacy Framework 1.0
(2020-01-16), the inventory and data-processing subcategories; ISO/IEC
29100 (confirm version); OWASP Logging Cheat Sheet (read date); CWE-532,
CWE-359.

**Testing** — `# Testing`, prefix `TST`, default category Reliability.

| ID | Title | Level | Applies when | Requirement |
|---|---|---|---|---|
| **## Tests in CI** | | | | |
| TST-001 | Run the tests in CI | MUST | always | CI runs the whole automated test suite on every change, and a failing test fails the build. |
| **## Kinds of test** | | | | |
| TST-002 | Unit-test the logic | MUST | always | Each module that contains logic has automated tests of its behaviour, its error cases included. |
| TST-003 | Add a regression test for each bug fix | MUST | always | Each bug fix comes with a test that fails without the fix. |
| TST-004 | Test integrations against real services | SHOULD | uses-database or uses-network | Tests of code that talks to a database, a message broker or another service run against a real instance of it (such as a container in CI), not only against mocks. |
| TST-005 | Test user workflows end to end | MUST | has-ui or has-cli | Each workflow that the user documentation describes has an end-to-end test that drives the real interface. |
| TST-006 | Test the API against its contract | MUST | exposes-network-api | Tests check that the running API's requests and responses match its contract (USS-API-001). |
| TST-007 | Test each data migration | MUST | stores-persistent-data | Each migration (USS-DAT-001) is tested against data in the format that it migrates from. |
| TST-008 | Test that access control denies | MUST | handles-authentication | For each protected operation, a test shows that a caller without the permission is refused. |
| **## Test effectiveness** | | | | |
| TST-009 | Run mutation testing on security-relevant code | SHOULD | handles-authentication or high-assurance | Mutation testing runs on the authentication, authorization and other security-relevant code, and on the code whose failure the project names as critical, and each surviving mutant is fixed or recorded with a reason. |
| TST-010 | Use property-based tests for wide input ranges | SHOULD | always | Code that accepts a wide range of inputs (such as parsers, encoders, numeric calculations and data transformations) has property-based tests. |
| TST-011 | Fuzz parsers in memory-unsafe code | MUST | parses-untrusted-input and has-memory-unsafe-code | Each parser of untrusted input is fuzzed, with sanitizers on, in CI or on a continuous fuzzing service. |
| TST-012 | Fuzz other parsers of untrusted input | SHOULD | parses-untrusted-input and not has-memory-unsafe-code | Each parser of untrusted input is fuzzed, in CI or on a continuous fuzzing service. |

TST-003 and TST-005 Verify: the bug fixes in the history window, and the
workflows that the user documentation describes. Cite: NIST SP 800-218
(SSDF 1.1) PW.8; CWE-20 (TST-011, TST-012); OSS-Fuzz (read date); the tools
as examples (Testcontainers; Stryker, PIT, mutmut, cargo-mutants;
Hypothesis, fast-check, proptest; libFuzzer, AFL++, cargo-fuzz).

---

### Task 5: Backup, Observability, Resilience

**Files:**
- Create: `standard/core/backup.md`, `standard/core/observability.md`,
  `standard/core/resilience.md`
- Modify: `tools/uss/src/evaluate/mod.rs` (floor test)

**Values:** floor test `(59, 179)`; lint
`uss lint: ok (179 rules, 17 files)`; commit
`feat(standard): draft the Backup, Observability and Resilience areas`.

**Backup** — `# Backup`, prefix `BAK`, default category Reliability. The
free text names the three scopes of spec §6.1.

| ID | Title | Level | Applies when | Requirement |
|---|---|---|---|---|
| **## What the software offers its operators** | | | | |
| BAK-001 | Provide backup and restore | MUST | stores-persistent-data and publishes-releases | The software offers a documented way to back up and restore all its persistent data: built-in commands, or documented copy steps where the data is plain files that a copy captures consistently. |
| BAK-002 | Make backups consistent | MUST | stores-persistent-data and publishes-releases | A backup taken while the software runs is consistent, such as through the database's own dump, a snapshot, or a pause in writes. |
| BAK-003 | Back up before each upgrade | SHOULD | stores-persistent-data and publishes-releases | The upgrade takes a backup before it changes stored data, so that the previous version can be restored. |
| BAK-004 | Restore onto a fresh install | MUST | stores-persistent-data and publishes-releases | A backup restores onto a fresh install of the same version, and a CI test proves it. |
| **## The project's own deployments** | | | | |
| BAK-005 | Set recovery objectives | MUST | operates-service and stores-persistent-data | Each production data store has a documented recovery point objective (RPO) and recovery time objective (RTO). |
| BAK-006 | Back up on a schedule | MUST | operates-service and stores-persistent-data | Backups run automatically, on a schedule that meets the RPO. |
| BAK-007 | Keep three copies, one immutable | MUST | operates-service and stores-persistent-data | Backups follow the 3-2-1 rule (three copies, on two kinds of storage, one off-site), and one copy is immutable or offline. |
| BAK-008 | Encrypt backups | MUST | operates-service and stores-persistent-data | Backups are encrypted. |
| BAK-009 | Keep backup credentials apart from production | MUST | operates-service and stores-persistent-data | The credentials that can change or delete backups are not available to the production systems. |
| BAK-010 | Alert on failed backups | MUST | operates-service and stores-persistent-data | A failed or missed backup raises an alert to a person. |
| BAK-011 | Test restores on a schedule | MUST | operates-service and stores-persistent-data | A restore from backup is tested on a documented schedule, at least once a quarter, and the results are recorded. |
| BAK-012 | Expire personal data from backups | MUST | operates-service and stores-persistent-data and handles-personal-data | Backup retention is set so that deleted personal data leaves the backups within the documented retention period (USS-PRV-003). |
| **## The source** | | | | |
| BAK-013 | Mirror the repository | SHOULD | always | The repository, with its full history and tags, is mirrored automatically to a second location that does not depend on its primary host. |

Cite: NIST SP 800-34 Rev. 1; CIS Critical Security Controls v8.1, Control 11
(Data Recovery), safeguards 11.2 to 11.5; CISA #StopRansomware Guide
(confirm date), offline and immutable backups.

**Observability** — `# Observability`, prefix `OBS`, default category
Reliability.

| ID | Title | Level | Applies when | Requirement |
|---|---|---|---|---|
| OBS-001 | Write structured logs | MUST | is-network-service or operates-service | Each log record is structured (such as JSON), with at least a timestamp, a level and a message. |
| OBS-002 | Let operators set the log level | MUST | is-network-service or has-cli | The log level can be changed through configuration or an option, without a rebuild. |
| OBS-003 | Provide health and readiness checks | MUST | is-network-service | The service reports separately whether it is alive and whether it is ready to serve, such as through two HTTP endpoints. |
| OBS-004 | Expose metrics | SHOULD | is-network-service | The service exposes metrics for its request rate, errors, latency and saturation in a standard format (such as OpenTelemetry or the Prometheus exposition format). |
| OBS-005 | Emit traces | SHOULD | is-network-service | The service emits OpenTelemetry traces and passes W3C Trace Context on to the services that it calls. |
| OBS-006 | Shut down gracefully | MUST | is-network-service | On a termination signal, the service stops taking new work, finishes or hands off its work in progress within a time limit, and then exits. |

Cite: OpenTelemetry Specification (confirm version), including its logs data
model; W3C Trace Context (confirm the current Recommendation); OpenMetrics or
the Prometheus exposition formats (read date); Google SRE Book (2016),
chapter 6, the four golden signals.

**Resilience** — `# Resilience`, prefix `RES`, default category Reliability.

| ID | Title | Level | Applies when | Requirement |
|---|---|---|---|---|
| RES-001 | Time out every network call | MUST | uses-network | Every outgoing network call has a finite timeout. |
| RES-002 | Retry with backoff and jitter | MUST | uses-network | A failed call is retried only when it is safe to repeat, and then with exponential backoff, jitter and a limit on attempts. |
| RES-003 | Degrade gracefully | SHOULD | is-network-service | When a dependency that is not essential fails, the service keeps serving what does not need it and reports what is unavailable. |
| RES-004 | Bound every queue, buffer and pool | MUST | is-network-service | Each queue, buffer, cache, connection pool and accepted request body has a maximum size, and work over the limit is refused. |
| RES-005 | Recover when a dependency returns | MUST | is-network-service | After a dependency fails and comes back, the service uses it again without a restart. |
| RES-006 | Write related changes atomically | MUST | stores-persistent-data | Changes to stored data that belong together are written atomically (such as in a transaction, or by writing a new file and renaming it), so a crash never leaves data half written. |

Cite: Amazon Builders' Library, Timeouts, retries, and backoff with jitter
(read date); Google SRE Book (2016), chapter 22, Addressing Cascading
Failures; CWE-400, CWE-770.

---

### Task 6: Release, Distribution and install, Deployment hardening

**Files:**
- Create: `standard/core/release.md`, `standard/core/distribution.md`,
  `standard/core/deployment-hardening.md`
- Modify: `standard/core/supply-chain.md` (SUP-004 and SUP-014)
- Modify: `tools/uss/src/evaluate/mod.rs` (floor test)

**Values:** floor test `(59, 215)`; lint
`uss lint: ok (215 rules, 20 files)`; commit
`feat(standard): draft the Release, Distribution and install and Deployment hardening areas`.

**Pointers:** SUP-004's "How the project's published charts and manifests
name its own images is a Distribution (DST) rule." names USS-DST-016.
SUP-014's "instructions for verifying a signature are a Distribution (DST)
rule" names USS-DST-015.

**Release** — `# Release`, prefix `REL`, `Applies when: publishes-releases`,
default category Maintainability. Override: REL-007 to REL-010
Compatibility. The free text says that signing release artifacts is
USS-SUP-014.

| ID | Title | Level | Applies when | Requirement |
|---|---|---|---|---|
| **## Versions** | | | | |
| REL-001 | Use Semantic Versioning | MUST | always | Releases follow Semantic Versioning 2.0.0, and the documentation defines the public API, including any command-line interface, configuration and stored-data format. |
| REL-002 | Tag each release | MUST | always | Each release has a tag in the repository, on the commit it was built from, named after its version. |
| REL-003 | Make release builds reproducible | SHOULD | always | Anyone can rebuild each release artifact from its tag, bit for bit, by documented steps. |
| REL-004 | Publish a support policy | MUST | always | A published policy says which versions get fixes, security fixes included, and until when. |
| **## Changelog** | | | | |
| REL-005 | Follow Common Changelog | MUST | always | CHANGELOG.md follows Common Changelog. |
| REL-006 | Cite the advisory for each security fix | MUST | always | Each changelog entry for a security fix names its advisory or CVE ID. |
| **## Breaking changes** | | | | |
| REL-007 | Avoid breaking changes | SHOULD NOT | always | A release does not introduce a breaking change. |
| REL-008 | Ship a migration guide with each breaking release | MUST | always | Each release with a breaking change comes with a migration guide that says how to move from the previous major version. |
| REL-009 | Deprecate before removing | SHOULD | always | A feature is deprecated, with a run-time warning, in at least one release before the release that removes it. |
| REL-010 | Automate migrations | SHOULD | always | Each breaking change comes with an automated migration, such as a codemod or a configuration converter. |
| **## Release audit** | | | | |
| REL-011 | Audit each release | MUST | always | Each release has an audit, against the USS version that the record pins, of the release commit or of a commit whose only later changes are under `.uss/`. |
| REL-012 | Publish the audit report with each release | MUST | always | The audit report for each release is generated by `uss` and published with the release. |

REL-011 Verify: `uss release-check` passes for the release commit
(conventions §11, "Release audits"). Cite: Semantic Versioning 2.0.0;
Common Changelog (confirm version or read date); Reproducible Builds,
definitions (read date); NIST SP 800-218 (SSDF 1.1) PS.3; the CVE Program
and GitHub Security Advisories as examples for REL-006.

**Distribution and install** — `# Distribution and install`, prefix `DST`,
`Applies when: publishes-releases`, default category Compatibility.
Override: DST-001 to DST-008 Features.

| ID | Title | Level | Applies when | Requirement |
|---|---|---|---|---|
| **## Install methods** | | | | |
| DST-001 | Offer native packages | MAY | has-cli or has-ui or is-network-service | The software is released as packages for the package managers of the platforms it supports (such as deb, RPM, Homebrew or winget). |
| DST-002 | Offer standalone binaries | MAY | has-cli or has-ui or is-network-service | The software is released as self-contained binaries or archives for each platform it supports. |
| DST-003 | Offer a container image | MAY | has-cli or is-network-service | The software is released as an OCI container image. |
| DST-004 | Offer a Compose file | MAY | is-network-service | The software is released with a Compose file that runs it and the services it needs. |
| DST-005 | Offer a Helm chart | MAY | is-network-service | The software is released with a Helm chart. |
| DST-006 | Offer an IDE extension | MAY | is-developer-tool | The software is released as an extension for at least one widely used IDE or editor. |
| DST-007 | Offer an MCP server | MAY | has-cli or exposes-network-api | The software offers its functions to AI agents through an MCP server. |
| DST-008 | Offer an agent skill | MAY | has-cli or exposes-network-api | The software is released with an agent skill or plugin that tells coding agents how to use it. |
| **## Install and upgrade** | | | | |
| DST-009 | Document install, upgrade and uninstall | MUST | always | For each install method offered, the documentation gives the steps to install, to upgrade and to uninstall. |
| DST-010 | Upgrade in place | MUST | has-cli or has-ui or is-network-service | Installing a new version over an existing install detects it and upgrades it in place, keeping its configuration and data. |
| DST-011 | Carry configuration forward | MUST | has-cli or has-ui or is-network-service | Configuration from every supported earlier version keeps working after an upgrade, or the upgrade migrates it automatically. |
| DST-012 | Support rollback | SHOULD | has-cli or has-ui or is-network-service | An upgrade can be rolled back to the previous version together with its data, by documented steps. |
| DST-013 | Support offline install | SHOULD | always | Each install method has a documented way to install without internet access. |
| DST-014 | Build for x86-64 and ARM64 | SHOULD | has-cli or has-ui or is-network-service | Each release artifact that is specific to a processor architecture is built for both x86-64 and ARM64 on each supported operating system. |
| **## Release artifacts** | | | | |
| DST-015 | Explain how to verify signatures | SHOULD | distributed-as-package or ships-container-image | The release documentation says how to verify the signature of each release artifact (USS-SUP-014). |
| DST-016 | Name the project's own images by digest | SHOULD | ships-container-image | Each chart, manifest or Compose file that the project publishes names the project's own images by digest. |

The Verify of DST-003, DST-005 and DST-007 also checks, when the record
adopts the rule, that it answers `ships-container-image`,
`deploys-to-kubernetes` and `exposes-mcp-server` (respectively) `true`.
DST-012's body names USS-BAK-003. Cite: OCI Image Format Specification
(confirm version); Compose Specification (read date); Helm documentation,
chart best practices (read date); MCP Specification (confirm the current
revision); the Sigstore documentation on verifying signatures (read date);
the agent-skill format of a widely used agent (confirm source and date).

**Deployment hardening** — `# Deployment hardening`, prefix `HRD`,
`Applies when: ships-container-image or deploys-to-kubernetes`, default
category Safety. Override: HRD-006 Reliability.

| ID | Title | Level | Applies when | Requirement |
|---|---|---|---|---|
| **## Container images** | | | | |
| HRD-001 | Build images to the CIS Docker Benchmark | MUST | ships-container-image | Each published image meets the Level 1 recommendations of the CIS Docker Benchmark for container images and build files. |
| HRD-002 | Run containers to the CIS Docker Benchmark | MUST | ships-container-image | Each Compose file and `docker run` instruction that the project publishes meets the Level 1 recommendations of the CIS Docker Benchmark for the container runtime. |
| HRD-003 | Use a read-only root filesystem | SHOULD | always | Containers run with a read-only root filesystem, and each path that the software writes to is a declared volume. |
| **## Kubernetes** | | | | |
| HRD-004 | Meet the Restricted Pod Security Standard | MUST | deploys-to-kubernetes | Every workload in the project's Kubernetes manifests and charts passes the Restricted profile of the Kubernetes Pod Security Standards. |
| HRD-005 | Set resource requests and limits | MUST | deploys-to-kubernetes | Every container in the manifests and charts has CPU and memory requests and a memory limit. |
| HRD-006 | Define liveness and readiness probes | MUST | deploys-to-kubernetes | Every long-running container in the manifests and charts has a liveness probe and a readiness probe. |
| HRD-007 | Deny network traffic that is not needed | SHOULD | deploys-to-kubernetes | The manifests and charts include network policies that allow only the traffic that the software needs. |
| HRD-008 | Limit service-account access | MUST | deploys-to-kubernetes | A workload mounts a service-account token only when it calls the Kubernetes API, and then its role grants only the permissions it needs. |

Cite: CIS Docker Benchmark (confirm version), sections 4 and 5; Kubernetes
Pod Security Standards (read date); NSA and CISA Kubernetes Hardening Guide
1.2 (2022-08); the Kubernetes documentation on probes and resource
management (read date).

---

### Task 7: Operations, Performance and efficiency, Audit logging

**Files:**
- Create: `standard/core/operations.md`, `standard/core/performance.md`,
  `standard/core/audit-logging.md`
- Modify: `tools/uss/src/evaluate/mod.rs` (floor test)

**Values:** floor test `(59, 237)`; lint
`uss lint: ok (237 rules, 23 files)`; commit
`feat(standard): draft the Operations, Performance and efficiency and Audit logging areas`.

**Operations** — `# Operations`, prefix `OPS`,
`Applies when: operates-service`, default category Reliability.

| ID | Title | Level | Applies when | Requirement |
|---|---|---|---|---|
| OPS-001 | Set service-level objectives | MUST | always | Each user-facing service has documented service-level objectives, with an error-budget policy that says what happens when the budget is spent. |
| OPS-002 | Alert on symptoms | SHOULD | always | Paging alerts fire on symptoms that users feel (such as the error-budget burn rate, the error rate or latency), not on causes alone. |
| OPS-003 | Link a runbook from every alert | MUST | always | Every alert links to a runbook that says how to diagnose and resolve it. |
| OPS-004 | Document incident response | MUST | always | A documented incident-response process names the roles and the severity levels, and says how to escalate and how to communicate. |
| OPS-005 | Hold blameless postmortems | SHOULD | always | Each major incident gets a blameless postmortem, and its follow-up actions are tracked to completion. |
| OPS-006 | Roll out changes progressively | SHOULD | always | Production changes roll out in stages (such as canary releases), and roll back automatically when health or SLO signals fail. |
| OPS-007 | Tell users about incidents | SHOULD | always | During an incident that users feel, a status page or channel tells them what is affected, and shows when it was last updated. |

Cite: Google SRE Book (2016), chapters 4, 6 and 15; The Site Reliability
Workbook (2018), chapters 2 and 5; NIST SP 800-61 Rev. 3 (confirm date).

**Performance and efficiency** — `# Performance and efficiency`, prefix
`PRF`, default category Efficiency.

| ID | Title | Level | Applies when | Requirement |
|---|---|---|---|---|
| PRF-001 | Set performance budgets | SHOULD | has-ui or has-cli or is-network-service or is-library | Written budgets set limits for the measures that matter to the software's users, such as startup time, latency, memory use, and binary or bundle size. |
| PRF-002 | Fail CI on performance regressions | SHOULD | has-ui or has-cli or is-network-service or is-library | Benchmarks for the budgeted measures run in CI, and a regression beyond a stated threshold fails the build. |
| PRF-003 | Meet the Core Web Vitals | SHOULD | has-web-ui | Web pages meet the "good" thresholds of the Core Web Vitals at the 75th percentile. |
| PRF-004 | Load-test the service | SHOULD | operates-service and is-network-service | Before each release, a load test runs the service at its expected peak load plus a stated margin. |
| PRF-005 | Do not busy-wait | MUST NOT | has-ui or has-cli or is-network-service | The software never waits in a loop that uses CPU while it has nothing to do; when idle, it waits on events or timers. |
| PRF-006 | Measure carbon intensity | SHOULD | operates-service | The project calculates the Software Carbon Intensity of the service that it operates, and tracks it from release to release. |

Cite: web.dev, Core Web Vitals (read date); ISO/IEC 21031:2024 (Software
Carbon Intensity); CWE-1050 (PRF-005).

**Audit logging** — `# Audit logging`, prefix `AUD`,
`Applies when: handles-authentication or has-multiple-users or operates-service`,
default category Safety. The free text says that in USS "audit" alone means a
compliance audit, and that offering audit logging at no extra charge belongs
to the Fair dealing area (TRU), named in words because its rules come later.

| ID | Title | Level | Applies when | Requirement |
|---|---|---|---|---|
| AUD-001 | Record security-relevant events | MUST | always | The software records an audit event for each sign-in success and failure, authorization failure, change of privileges or roles, configuration change, administrative action, access to sensitive data, and access to the audit trail. |
| AUD-002 | Record who, what, when, where and the outcome | MUST | always | Each audit record names the actor, the action, the target, the time, the source (such as the client address) and the outcome. |
| AUD-003 | Keep audit records apart from other logs | MUST | always | Audit records go to their own stream or store, which log-level settings cannot turn off. |
| AUD-004 | Make the audit trail tamper-evident | MUST | always | The audit trail is append-only for the software, and a change to a record, or its removal, can be detected. |
| AUD-005 | Use synchronized UTC time | MUST | always | Audit timestamps are in UTC, from a clock that is synchronized to a time source. |
| AUD-006 | Restrict access to the audit trail | MUST | always | Only roles granted audit access can read the audit trail, and no role can change or delete records through the software. |
| AUD-007 | Make audit retention configurable | MUST | always | Operators can set how long audit records are kept, and the documented default fits the personal-data retention rule (USS-PRV-003). |
| AUD-008 | Alert when audit logging fails | MUST | always | A failure to write audit records raises an alert to an operator. |
| AUD-009 | Export audit records in a standard format | SHOULD | always | Audit records can be exported or streamed to a SIEM in a standard format (such as syslog per RFC 5424, OCSF or CEF). |

Cite: NIST SP 800-53 Rev. 5 (AU-2, AU-3, AU-5, AU-8, AU-9, AU-11); the DISA
Application Security and Development STIG (confirm version); OWASP ASVS 5.0,
the security-logging chapter (confirm its number); NIST SP 800-92 (confirm
whether a revision is final); RFC 5424; OCSF (confirm version); CWE-778.

---

### Task 8: AI features, Accessibility, Usability, Internationalization

**Files:**
- Create: `standard/core/ai-features.md`, `standard/core/accessibility.md`,
  `standard/core/usability.md`, `standard/core/internationalization.md`
- Modify: `tools/uss/src/evaluate/mod.rs` (floor test)

**Values:** floor test `(59, 269)`; lint
`uss lint: ok (269 rules, 27 files)`; commit
`feat(standard): draft the AI features, Accessibility, Usability and Internationalization areas`.

**AI features** — `# AI features`, prefix `AI`,
`Applies when: uses-ai-models or has-ai-tool-use`, default category Safety.
Override: AI-006 Reliability.

| ID | Title | Level | Applies when | Requirement |
|---|---|---|---|---|
| AI-001 | Treat model output as untrusted input | MUST | uses-ai-models | Model output is validated or encoded, like any untrusted input, before the software uses it, such as in a query, a command, a page or a tool call. |
| AI-002 | Document prompt-injection defences | MUST | uses-ai-models | For each path by which content that the project does not control reaches a model, the design documents the controls that limit what an injected instruction can do. |
| AI-003 | Give models only the tools they need | MUST | has-ai-tool-use | Each tool that a model can use grants only the permissions that its task needs, and never more than those of the user it acts for. |
| AI-004 | Confirm destructive actions with a person | MUST | has-ai-tool-use | An action that a model starts, and that deletes data, spends money, sends a message or cannot be undone, runs only after a person confirms it. |
| AI-005 | Pin model versions | MUST | uses-ai-models | The software names exact model versions, not aliases that the provider can move, so behaviour changes only with a deliberate update. |
| AI-006 | Evaluate models like tests | MUST | uses-ai-models | An evaluation suite with pass thresholds runs in CI before each change of model, prompt or model settings merges, and a regression fails the check. |
| AI-007 | Limit cost, rate and tokens | MUST | uses-ai-models | The software limits the tokens per request, the requests per user and the total spend, and alerts when a limit is reached. |
| AI-008 | No sensitive data to models without a basis | MUST NOT | uses-ai-models and handles-sensitive-data | Sensitive data is never sent to a model unless the design records the purpose and the basis for it, and the model provider's terms allow it. |
| AI-009 | Tell people when they deal with AI | MUST | uses-ai-models and has-ui | The interface tells people when they are interacting with an AI system, and marks content that AI generated. |
| AI-010 | Authenticate MCP clients | MUST | exposes-mcp-server and exposes-web-api | A remote MCP server authenticates its clients as the MCP authorization specification describes. |

AI-004's Verify says how an MCP server meets it (through the client's
confirmation or the protocol's elicitation), as confirmed from the MCP
specification. AI-008's body names USS-PRV-001 for personal data. Cite: OWASP
Top 10 for LLM Applications 2025 (LLM01, LLM02, LLM05, LLM06, LLM10); MCP
Specification (confirm the current revision), Authorization and Security Best
Practices; NIST AI 100-1 (AI RMF 1.0); NIST AI 600-1 (2024-07); Regulation
(EU) 2024/1689 (AI Act), Article 50.

**Accessibility** — `# Accessibility`, prefix `ACC`, `Applies when: has-ui`,
default category Accessibility & Usability.

| ID | Title | Level | Applies when | Requirement |
|---|---|---|---|---|
| ACC-001 | Meet WCAG 2.2 Level AA | MUST | always | Every user interface meets WCAG 2.2 at Level AA; a UI that is not web content meets it as WCAG2ICT applies it. |
| ACC-002 | Check accessibility automatically in CI | MUST | has-web-ui | CI runs an automated accessibility checker (such as axe-core) on the UI's pages, and a violation fails the build. |
| ACC-003 | Test by hand with a keyboard and a screen reader | MUST | always | The main workflows are tested by hand, with a keyboard only and with a screen reader, at least once per release or once a quarter, and the dated results are recorded. |
| ACC-004 | Respect reduced-motion settings | MUST | always | When the system asks for reduced motion, the UI turns off animation that is not essential. |
| ACC-005 | Publish an accessibility statement | SHOULD | always | An accessibility statement names the standard and level that the UI meets, its known gaps, and how to report a problem. |

ACC-003 Verify: "main workflows" are the workflows that the user
documentation describes. Cite: WCAG 2.2 (W3C Recommendation, confirm the
dated edition); WCAG2ICT (W3C Group Note, confirm date); CSS Media Queries
Level 5, `prefers-reduced-motion` (ACC-004); WCAG 2.2 success criterion 2.3.3
(context for ACC-004); W3C WAI, Developing an Accessibility Statement (read
date); axe-core as an example.

**Usability** — `# Usability`, prefix `UX`, default category Accessibility &
Usability. Override: UX-004 Fairness.

| ID | Title | Level | Applies when | Requirement |
|---|---|---|---|---|
| **## All interfaces** | | | | |
| UX-001 | Say what went wrong and how to fix it | MUST | has-ui or has-cli | Each error message that users see says what went wrong and what they can do about it. |
| UX-002 | Confirm or undo destructive actions | MUST | has-ui or has-cli | An action that deletes or overwrites user data asks for confirmation first or can be undone; a command-line option such as `--force` counts as confirmation. |
| UX-003 | Start with zero configuration | SHOULD | has-ui or has-cli or is-network-service | The software starts and does its main job with default settings and no configuration file. |
| UX-004 | No deceptive design | MUST NOT | has-ui | The UI uses no deceptive design pattern, such as hidden costs, confirmshaming, pre-ticked consent, or a cancellation that is harder than the sign-up. |
| UX-005 | Keep files where the platform expects them | SHOULD | has-ui or has-cli | Configuration, data, cache and state files go in the locations that the platform's conventions name (such as the XDG Base Directory Specification on Linux). |
| **## Command-line interfaces** | | | | |
| UX-006 | Follow the Command Line Interface Guidelines | SHOULD | has-cli | The command-line interface follows the Command Line Interface Guidelines. |
| UX-007 | Provide --help and --version | MUST | has-cli | The command-line interface supports `--help` and `--version`. |
| UX-008 | Use meaningful exit codes | MUST | has-cli | Commands exit with 0 on success and with a non-zero, documented code on failure. |
| UX-009 | Offer machine-readable output | SHOULD | has-cli | Commands that print data can print it in a machine-readable format, such as JSON. |
| UX-010 | Honour NO_COLOR | MUST | has-cli | The interface prints no ANSI colour when the `NO_COLOR` environment variable is set to a value that is not empty. |
| UX-011 | Keep output and messages apart | MUST | has-cli | Commands write their output to standard output, and their messages and errors to standard error. |

Cite: Command Line Interface Guidelines (clig.dev, read date); NO_COLOR
(no-color.org, read date); XDG Base Directory Specification 0.8, with the
Windows Known Folders and Apple file-system documentation (read date); EDPB
Guidelines 03/2022 on deceptive design patterns (confirm version); FTC staff
report, Bringing Dark Patterns to Light (2022-09).

**Internationalization** — `# Internationalization`, prefix `I18N`, default
category Accessibility & Usability.

| ID | Title | Level | Applies when | Requirement |
|---|---|---|---|---|
| I18N-001 | Use Unicode throughout | MUST | has-ui or has-cli or stores-persistent-data or uses-network | Text is handled as Unicode, and encoded as UTF-8 where it is stored or exchanged. |
| I18N-002 | Keep user-facing text out of code | SHOULD | has-ui or has-cli | Text that users see is kept in resource files, apart from the code, so that it can be translated. |
| I18N-003 | Format for the user's locale | SHOULD | has-ui | Numbers, dates, times and currencies are shown in the format of the user's locale. |
| I18N-004 | Store timestamps in UTC | MUST | stores-persistent-data | Points in time are stored in UTC; a future local time also stores its time zone. |
| I18N-005 | Exchange timestamps in RFC 3339 format | MUST | exposes-network-api | Points in time that the API sends or accepts use the RFC 3339 format. |
| I18N-006 | Offer more than one language | MAY | has-ui | The user interface is available in more than one language. |

Cite: The Unicode Standard (confirm version); RFC 3629; RFC 3339; Unicode
CLDR (confirm version); W3C Internationalization, best practices (read date).

---

### Task 9: Extensibility, Independence

**Files:**
- Create: `standard/core/extensibility.md`, `standard/core/independence.md`
- Modify: `standard/applicability.md` (two facts that a rule sets)
- Modify: `tools/uss/src/evaluate/mod.rs` (floor test)

**Values:** floor test `(59, 295)`; lint
`uss lint: ok (295 rules, 29 files)`; commit
`feat(standard): draft the Extensibility and Independence areas`.

**Facts set by a rule.** Add at the end of `standard/applicability.md`, in
the existing style. They have `Enabled by`, not `Question`, so the record
template and the `want` list of `applicability_file_is_valid` do not change:

| Fact | Enabled by | Definition (Yes when …) |
|---|---|---|
| has-plugin-interface | USS-EXT-001 | the project adopts USS-EXT-001: the software offers an interface through which plugins add features |
| has-plugin-marketplace | USS-EXT-008 | the project adopts USS-EXT-008: the project runs or ships a marketplace through which users find and install plugins |

**Extensibility** — `# Extensibility`, prefix `EXT`, default category
Features. Overrides: EXT-004, EXT-005, EXT-009, EXT-010, EXT-011 and EXT-014
Safety.

| ID | Title | Level | Applies when | Requirement |
|---|---|---|---|---|
| **## Plugin interface** | | | | |
| EXT-001 | Provide a plugin interface | MAY | has-ui or has-cli or is-network-service or is-library | The software offers an interface through which plugins add features. `Enables: has-plugin-interface`. |
| EXT-002 | Version the plugin contract | MUST | has-plugin-interface | The plugin contract is documented and versioned under Semantic Versioning, with stated compatibility guarantees. |
| EXT-003 | Require a plugin manifest | MUST | has-plugin-interface | Each plugin carries a manifest that states its identity, its version, the host versions it works with, the permissions it requests and its licence. |
| EXT-004 | Grant plugins only the permissions they request | MUST | has-plugin-interface | A plugin gets only the permissions that its manifest requests and that the user or operator grants. |
| EXT-005 | Isolate plugins from the host | MUST | has-plugin-interface | A plugin that fails or misbehaves cannot crash or take over the host, such as because it runs in its own process or sandbox. |
| EXT-006 | Manage the plugin lifecycle | MUST | has-plugin-interface | Plugins can be installed, enabled, disabled, upgraded and uninstalled, and uninstalling removes everything that the plugin added. |
| EXT-007 | Ship an SDK, an example plugin and a test harness | SHOULD | has-plugin-interface | Plugin authors get an SDK, a working example plugin and a test harness. |
| **## Plugin marketplace** | | | | |
| EXT-008 | Provide a plugin marketplace | MAY | has-plugin-interface | The project runs or ships a marketplace through which users find and install plugins. `Enables: has-plugin-marketplace`. |
| EXT-009 | Sign plugins and verify publishers | MUST | has-plugin-marketplace | Each plugin package is signed, its publisher's identity is verified, and the host checks the signature before it installs the plugin. |
| EXT-010 | Publish plugin provenance | SHOULD | has-plugin-marketplace | Each plugin package comes with build provenance that links it to its source, such as SLSA provenance. |
| EXT-011 | Scan and review plugins | MUST | has-plugin-marketplace | Each plugin version is scanned automatically for malware and known vulnerabilities, and goes through a documented review, before it is listed. |
| EXT-012 | Keep a versioned index | MUST | has-plugin-marketplace | The marketplace keeps a versioned index of plugins, and resolves the dependencies between them. |
| EXT-013 | Deliver plugin updates | MUST | has-plugin-marketplace | Hosts learn of plugin updates through an update channel, and can apply them. |
| EXT-014 | Revoke bad plugins | MUST | has-plugin-marketplace | The marketplace can revoke a plugin version, and hosts then disable it. |
| EXT-015 | Show each plugin's licence | MUST | has-plugin-marketplace | Each listing shows the plugin's licence as an SPDX licence expression. |
| EXT-016 | Offer search | SHOULD | has-plugin-marketplace | Users can search and browse the marketplace. |
| EXT-017 | Allow self-hosting and mirroring | SHOULD | has-plugin-marketplace | The marketplace can be self-hosted or mirrored, for use without internet access. |

Cite: Semantic Versioning 2.0.0; The Update Framework specification (confirm
version), for EXT-009, EXT-013 and EXT-014; SLSA 1.2, provenance; SPDX
License List; the extension manifests of widely used hosts (such as VS Code)
as examples.

**Independence** — `# Independence`, prefix `IND`, default category
Compatibility.

| ID | Title | Level | Applies when | Requirement |
|---|---|---|---|---|
| **## Self-hosting** | | | | |
| IND-001 | Support self-hosting | SHOULD | is-network-service | The software runs entirely on the user's own infrastructure, with a documented self-hosting guide, and needs no connection to a service outside it. |
| IND-002 | Do not cut down the self-hosted edition | SHOULD NOT | is-network-service and operates-service | The self-hosted edition does not lack features that the hosted edition has. |
| IND-003 | Run without internet access | SHOULD | is-network-service | Every feature that does not inherently need the internet works where outbound traffic is restricted, or where there is no internet access. |
| **## Open dependencies** | | | | |
| IND-004 | Need no commercial product | SHOULD NOT | is-foss | The software does not require a commercial product or subscription to work: every required service or tool has a FOSS option, and proprietary ones are optional integrations. |
| IND-005 | Need no single cloud provider | SHOULD NOT | uses-network | The software does not require one cloud provider's proprietary services; it uses open or widely implemented protocols, such as S3-compatible storage, OIDC or OpenTelemetry. |
| **## Databases** | | | | |
| IND-006 | Support SQLite, PostgreSQL and MariaDB or MySQL | SHOULD | uses-relational-database | The software supports SQLite, PostgreSQL, and MariaDB or MySQL. |
| IND-007 | Test every supported database in CI | MUST | uses-relational-database | The full test suite runs in CI against every database that the software claims to support. |
| IND-008 | Access databases through an abstraction layer | SHOULD | uses-relational-database | Database access goes through an abstraction layer: an ORM, a query builder, or repository interfaces with an adapter for each database. |
| IND-009 | Write migrations for each supported database | SHOULD | uses-relational-database | Each migration is written or generated for every supported database. |

IND-006's rationale says that a project that depends on one database's
distinctive features (such as PostGIS, JSONB operators or LISTEN/NOTIFY)
records a Standard Deviation (spec §6.1). Cite: mostly `none`; OpenID Connect
Core 1.0 and OpenTelemetry as examples for IND-005.

---

### Task 10: Fair dealing, Commercial licensing

**Files:**
- Create: `standard/core/fair-dealing.md`,
  `standard/core/commercial-licensing.md`
- Modify: `tools/uss/src/evaluate/mod.rs` (floor test)

**Values:** floor test `(59, 317)`; lint
`uss lint: ok (317 rules, 31 files)`; commit
`feat(standard): draft the Fair dealing and Commercial licensing areas`.

**Fair dealing** — `# Fair dealing`, prefix `TRU`, default category
Fairness. Override: TRU-003 Safety.

| ID | Title | Level | Applies when | Requirement |
|---|---|---|---|---|
| TRU-001 | Keep free features free | SHOULD NOT | publishes-releases or operates-service | A feature that users once had at no charge is not removed, or moved behind a payment or a paid tier, in a later version. |
| TRU-002 | Do not relicense to more restrictive terms | SHOULD NOT | publishes-releases | A later version is not released under a licence that is more restrictive than the licence of the earlier versions. |
| TRU-003 | Do not charge extra for security essentials | SHOULD NOT | is-commercial and handles-authentication | Single sign-on (OIDC, SAML or LDAP), multi-factor authentication and audit logging are not sold separately, or only in higher tiers. |
| TRU-004 | Never hold users' data hostage | MUST NOT | is-commercial and stores-user-data | Users' export of their own data, in an open and documented format, never depends on the state of their subscription or licence. |
| TRU-005 | Announce pricing and licence changes in advance | SHOULD | is-commercial | Each change to pricing or to licence terms is announced in the changelog before it takes effect. |

TRU-004's body names USS-DAT-006. Cite: CISA Secure by Design (2023-10-25),
the SSO and logging principles (confirm the sections); `none` where no source
applies.

**Commercial licensing** — `# Commercial licensing`, prefix `COM`,
`Applies when: is-commercial`, default category Fairness.

| ID | Title | Level | Applies when | Requirement |
|---|---|---|---|---|
| **## Transparent pricing** | | | | |
| COM-001 | Publish prices | SHOULD | always | Each licence model, tier and offering has a published list price and a published list of what it includes. |
| COM-002 | Keep negotiated terms no worse than published ones | SHOULD NOT | always | A negotiated agreement has no term worse than the published terms for the same thing, and sets aside no other rule of this area. |
| COM-003 | Sell each feature on its own | SHOULD | always | Each feature can be bought on its own. |
| COM-004 | Do not penalise buying features separately | SHOULD NOT | always | No feature costs more on its own than the cheapest bundle that includes it. |
| **## Price changes** | | | | |
| COM-005 | State a cap on price increases | SHOULD | always | Each agreement states a cap on future price increases. |
| COM-006 | Keep increases within inflation plus 3 points | SHOULD NOT | always | A price increase does not exceed a named inflation index (such as CPI) plus 3 percentage points a year. |
| COM-007 | Raise prices only at renewal | SHOULD NOT | always | A price is not increased during the term of an agreement. |
| COM-008 | Give 90 days' notice of price increases | SHOULD | always | Each price increase is announced at least 90 days before it applies. |
| **## Licence terms** | | | | |
| COM-009 | Do not penalise virtualisation | SHOULD NOT | publishes-releases | Licence metrics do not count resources that the software does not use, such as all the physical cores of a host that runs it in a virtual machine or a container. |
| COM-010 | Show licence usage in the product | SHOULD | always | The product shows customers their current licence usage against their entitlement. |
| COM-011 | Offer an offline licence file | SHOULD | publishes-releases | A licence can be delivered as a file that works with no connection to a licence service. |
| COM-012 | Make any licence server self-hostable | SHOULD | publishes-releases | Any licence server that the software uses can run on the customer's own infrastructure. |
| COM-013 | Allow a grace period when the licence service is unreachable | SHOULD | publishes-releases | When the software cannot reach a licence service, it keeps working for a documented grace period of at least 30 days. |
| COM-014 | Degrade gracefully when a licence expires | SHOULD | publishes-releases | When a licence expires, the software keeps users' access to their data, such as by switching to read-only, rather than stopping. |
| **## Licence models** | | | | |
| COM-015 | Offer several licence models | SHOULD | always | Customers can choose from at least three licence models, such as named user, concurrent, node-locked, subscription and perpetual. |
| COM-016 | Offer a perpetual or perpetual-fallback licence | SHOULD | publishes-releases | Customers can buy a perpetual licence, or get a perpetual-fallback licence that keeps the last version they paid for. |
| COM-017 | Offer a licence for CI/CD | SHOULD | publishes-releases | A service-account licence for automation uses no human seat, works on ephemeral and containerised runners without a node lock, activates from a secret token without interaction, and is priced for automation. |

Cite: `none` for most rules (they are USS's own); name the inflation indexes
as examples only.

---

### Task 11: Budget, cross-references, spec alignment and handover

**Files:**
- Modify: `tools/uss/src/evaluate/mod.rs` (test `all_false_record_floor`)
- Modify: `docs/superpowers/specs/2026-11-21-universal-software-standard-design.md`
  (§5.5 and §6.1)

- [ ] **Step 1: Add the budget assertion**

In `all_false_record_floor`, after the `assert_eq!`, add:

```rust
assert!(
    floor * 4 <= core,
    "{floor} of {core} core rules is over the spec §6.4 budget"
);
```

Change the end of the doc comment to say that the first assertion pins
today's counts and the second holds the budget.

- [ ] **Step 2: Run the test**

Run: `cargo test --locked -p uss all_false_record_floor`
Expected: PASS (59 × 4 = 236 ≤ 317).

- [ ] **Step 3: Check two profiles**

Copy `templates/record.yaml` to a scratch directory twice.

- **CLI:** set `has-cli`, `uses-network` and `has-dependencies` to `true`.
  Run `uss applicable` on it. Expected: 101 rules (Plan 2's 38 for this
  profile, the 38 new `always` rules, and 25 more), and the only MAY rule
  among them is USS-EXT-001.
- **Library:** set `is-library`, `distributed-as-package` and
  `has-dependencies` to `true`. Run `uss applicable` on it. Expected: the
  only MAY rule is USS-EXT-001, and the only DST rules are USS-DST-009,
  USS-DST-013 and USS-DST-015.

If a count differs, find which side is wrong (the plan's tables or the
files) before you go on, and report it.

- [ ] **Step 4: Check that every rule ID in the standard exists**

Write a short script (in the scratchpad, not the repository) that collects
every `USS-<PREFIX>-<NNN>` in `standard/` and in the spec, and compares the
set with the IDs in `uss export --json`. Expected: every ID in
`standard/core/` exists; IDs in the spec and in `conventions.md` exist or
are clearly examples (such as `USS-GO-004` or `USS-EXA-005`). Report the
list of examples.

- [ ] **Step 5: Align the spec with this plan**

Edit the spec, wrapping at 80 columns:

- §5.5 table, Compatibility row, "Mostly from": `DST, API, IND, LIC`.
- §6.1 CON: after "no disabled checks or skipped tests without a tracked
  reason", add "(the check-integrity rules that enforce this are QLT's)".
- §6.1 TST: "no disabled tests without tracking" becomes QLT's, and
  "accessibility" testing becomes ACC's; say so in the row.
- §6.1 DOC: CHANGELOG.md's format is REL's, the upgrade guide is DST's (per
  install method), and dev-environment setup is CTB's; add "a public API
  reference for libraries".
- §6.1 LIC: add "Default category Compatibility"; "preferring permissive" is
  guidance in the rationale, not a scored rule.
- §6.1 REL: the MAJOR version and the breaking flag come from SemVer and
  Common Changelog, which REL requires.
- §6.1 DAT: export and import are MAY rules (Features).
- §6.1 DST: the install-method MAYs apply only to released software of a
  kind that the method fits (`publishes-releases`).
- §6.1 UX: deceptive design counts toward Fairness.
- §6.1 AI: "When `uses-ai-models` or `has-ai-tool-use` (which
  `exposes-mcp-server` implies)".
- §6.1 SAF: integer safety comes through the language modules' safety
  checks; race detection applies when `uses-concurrency`.

- [ ] **Step 6: Run all checks**

Run: `cargo xtask check`
Expected: exit 0; `uss lint: ok (317 rules, 31 files)`.

- [ ] **Step 7: Commit**

```bash
git add tools/uss/src/evaluate/mod.rs docs/superpowers/specs/2026-11-21-universal-software-standard-design.md
git commit -m "test(uss): hold the floor budget; docs(spec): align areas with Plan 3"
```

- [ ] **Step 8: Hand over**

Give the user: the rule count per area and level; the floor (59 of 317,
18.6%) and the two profiles' counts; the plan-level decisions above, marked
where the drafting changed them; and every reference that the implementers
could not confirm. Plan 4 is written after this hand-over.
