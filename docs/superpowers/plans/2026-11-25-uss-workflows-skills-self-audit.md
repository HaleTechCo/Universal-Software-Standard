# USS Workflows, Skills and Self-Audit Implementation Plan (Plan 5 of 5, without the release)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make USS usable on another project straight from a local checkout,
with no release: the AGENTS.md workflows (spec §8), the Claude plugin with its
two skills, the adopter templates, and the repository documents of the spec's
layout (§9). Then audit this repository against its own standard, and fix
what files alone can fix. Release 0.1.0 is not part of this plan.

**Architecture:** Task 1 gives `uss` the two commands that the workflows need
(`uss show` and `uss applicable --full`) and embeds `AGENTS.md` and
`CHANGELOG.md`, so one binary carries one version of the standard and of its
workflows. Task 2 applies the decisions on the items that Plan 4 parked. Tasks
3–5 write the workflows, the adopter templates and the plugin. Tasks 6–7 write
the repository's documents and configuration. Tasks 8–10 run this
repository's self-audit by following the new Audit workflow, then fix what
files alone can fix. Task 11 aligns the spec and hands over, with the list of
what the release plan must do.

**Tech Stack:** Rust 1.100.0 (`tools/uss`, `tools/xtask`, `cargo xtask
check`); Markdown; GitHub Actions YAML; Claude Code plugin manifests (JSON).
No new crate dependencies.

**Spec:** `docs/superpowers/specs/2026-11-21-universal-software-standard-design.md`
(§8 agent workflows, §9 layout, validator and dogfooding, §12 packaging and
"how a skill obtains the pinned standard version", §13 step 4). The normative
rulebook is `standard/conventions.md`: sections 5 (applicability), 8
(compliance and scores), 9 (Standard Deviations) and 11 (records, results,
reports, components, release audits).

**Roadmap:** Plans 1–4 are merged into local `main` (f414a61). Plans 1–3 are
pushed (`origin/main` is d8eb4ae); the Plan 4 commits are not. This is Plan 5
without its last step: the user tries USS on one of their own repositories
and makes changes first. A later release plan cuts 0.1.0; Task 11 writes its
list.

## Global Constraints

- Keywords: exactly `MUST`, `MUST NOT`, `SHOULD`, `SHOULD NOT`, `MAY` (RFC 2119
  as clarified by RFC 8174). One rule, one level. A rule body contains its own
  keyword and no other keyword, rationale included (conventions §3 "Keyword
  scan"); put any other keyword only inside a fenced code block.
- Rule IDs are never reused or renumbered. A new rule takes the next free
  number at the end of its file.
- References MUST be checked against the current source when written, and MUST
  pin a version or `(read YYYY-MM-DD)`. A source that could not be read is not
  cited.
- Verify text checks exactly what the body requires: it adds no requirement
  and no exemption.
- The standard's version stays `0.1.0`. Nothing is released, tagged or pushed.
  No GitHub setting is changed. `gh` is used only for read-only `GET` calls.
- The content of `Initial_Idea.md` and `todo.txt` is never modified.
- Files are UTF-8 with LF line endings. Wrap prose at 80 columns. Table rows,
  fenced code and YAML may be longer.
- Rust: `cargo xtask check` passes after every task. The library
  (`tools/uss/src`, outside `cli/`) does no file, network or process I/O
  (spec §9).
- Tools are never installed, `uss` included: inside this repository, run it
  as `cargo run --locked -q -p uss -- <args>`. A step that runs another tool
  runs it only when `<tool> --version` already succeeds; otherwise the report
  says "not run".
- Steps that a person or an agent follows (the AGENTS.md workflows, the skill
  bodies, the snippet, install and upgrade steps, the contributing guide's
  procedures, the troubleshooting guide) follow the writing rules of
  ASD-STE100 (USS-DOC-007): one instruction per sentence, the imperative mood,
  the active voice, at most 20 words in a procedural sentence and 25 in a
  descriptive one, and one term for one thing.
- Each command in `AGENTS.md`, the README and the contributing guide runs as
  written in a fresh clone (USS-CTB-007). Each `uss` command and option that a
  document names exists (the `documented_commands_exist` test, from Task 3).
- GitHub Actions YAML pins each action by its full commit SHA with the version
  in a comment (USS-SUP-003), sets least-privilege `permissions`
  (USS-SUP-010), and passes event data to scripts only through `env`, never as
  `${{ }}` inside `run:` (USS-SUP-012).
- No email address or other personal data goes into a repository file. A
  GitHub login goes only where a file format needs one (CODEOWNERS).

## Plan-level decisions (refining the spec, for the user's review)

1. **One binary, one version, workflows included.** The build embeds
   `AGENTS.md` and `CHANGELOG.md` with `standard/`, `schema/` and
   `templates/`. `uss show [PATH]` prints an embedded file, or lists them all.
   `uss applicable --full <record>` prints the full text of each rule that
   applies. The workflows read the standard only through these two commands,
   at the version that the record pins. This answers spec §12 ("how a skill
   obtains the pinned standard version"): install the `uss` release of that
   version and read the standard and the workflows through it.
2. **The workflows are in `AGENTS.md`** (spec §8), which serves two readers:
   "Use USS on a project" first, then "Agent conduct", then "Work on this
   repository" (USS-CTB-007). `CLAUDE.md` holds only `@AGENTS.md`.
3. **Questions come in batches** (spec anti-goal: no 150 questions one at a
   time). The facts come in one table, with an answer filled in from the scan
   and the evidence for it. SHOULD and SHOULD NOT rules come one area at a
   time, with "comply" as the recommended answer, and the developer can
   accept a whole area in one reply. MAY rules come one area at a time, each
   with its score category, answered in one reply per area.
4. **`uss` is needed for the report.** Scores and compliance are never
   computed by hand (conventions §11), so the workflows tell the agent to
   install `uss` before it starts.
5. **Headless audits** (no person present): the agent asks nothing and
   changes no fact, decision or deviation. Each open question becomes an
   `unverified` result on each applicable rule whose result depends on it, with
   the question in the evidence. A release audit at an unchanged version has
   no new questions unless the project's facts changed.
6. **The plugin.** Plugin `uss`, at the repository root (`"source": "./"`), in
   the marketplace `universal-software-standard`. Its skills are
   `skills/adopt/` and `skills/audit/`, which Claude Code names `/uss:adopt`
   and `/uss:audit` (the spec's `uss-adopt` and `uss-audit`, with the
   namespace that Claude Code adds). The audit skill also runs release audits
   and upgrades. Each skill only tells Claude to follow a workflow in
   `${CLAUDE_PLUGIN_ROOT}/AGENTS.md`; it copies no workflow text (spec §8).
   A plugin from a local-path marketplace loads in place, so the user's trial
   needs no release; a git marketplace copies only tracked files, so `target/`
   is never copied. A root `CLAUDE.md` makes `claude plugin validate` warn;
   that warning is accepted.
7. **Adopter templates.** `templates/AGENTS.snippet.md` names no version (it
   points to the record), so an upgrade does not make it stale.
   `templates/ci/uss-release.yml` runs `uss release-check` when a GitHub
   release is published and attaches the report to it. A template that runs
   the agent audit headless in CI waits for the release plan: it needs a
   public marketplace to install the plugin from.
8. **Plan 4's parked items.** The user decided two on 2026-11-25:
   - USS-DEP-001: a peer dependency range may span several major versions,
     but it MUST have an upper bound.
   - USS-SUP-006: when a supplier fixes only its latest release (such as Rust),
     the release before the latest also counts as supported.

   Rulings for the rest:
   - A project may use a tool version outside a profile's `Versions`. The
     auditor then checks that each tool rule that the profile names exists in
     that version (conventions §12). This settles the Clippy profile's
     one-release range without a change to it.
   - USS-MVN-007 and USS-GO-009 stay MUST: the strongest rule they refine is
     USS-DEP-004, a MUST (conventions §7). A Maven project that keeps a
     dependency that the analysis reports as unused lists it in the plugin's
     ignore setting, with a reason.
   - USS-TS-007 also checks that CI installs the pinned versions, as USS-SUP-001
     requires.
   - USS-TS-001 and TypeScript 7: Task 2 reads typescript-eslint's supported
     range and records the gap in the ESLint profile if there is one.
   - USS-MVN-004 is retitled "Bound each dependency version".
   - Spec §6.3's phrase on tool rules is aligned with conventions §7.
   - USS-CON-001 stays unmapped in the Ruff and golangci-lint profiles. Their
     TODO checks report every marker, which is stricter than the rule, so a
     baseline with them would exceed the standard.
9. **The developer's answers for this repository's record** (Tasks 8–9):
   - Stack: `languages: [rust]`.
   - Facts true: `has-cli`, `parses-untrusted-input` (records and results from
     any repository, in CI on pull requests too), `is-developer-tool`,
     `distributed-as-package` (spec §9: release binaries and `cargo install`),
     `publishes-releases`, `has-dependencies`, `accepts-external-contributions`
     (spec §6.6: vendors send profiles) and `is-foss` (MIT).
   - Every other asked fact is false. `has-multiple-maintainers` is false
     (one committer), and so is `is-library`: no other project uses the
     library yet.
   - MAY decisions: adopt USS-DST-002 (standalone binaries, spec §9) and
     USS-DST-008 (the plugin). Decline USS-DST-001, USS-DST-003, USS-DST-006
     and USS-DST-007 (the IDE extension and the MCP server are "later" in spec
     §11), and USS-EXT-001.
   - SHOULD and SHOULD NOT: comply with every one (the recommended answer).
     No deviation is drafted, because approving one is the user's call. A
     SHOULD that is not met is recorded as `fail`.
10. **What Plan 5 fixes** (the user chose "audit and in-repo fixes"): only
    files, meaning documents, CI YAML, Dependabot, hook scripts and templates.
    The release plan's list takes everything else:
    - code changes for audit findings (a log-level option, keeping text out of
      code, fuzzing, property tests, performance budgets);
    - secret scanning and static security analysis, which must also run in
      `cargo xtask check` (USS-CTB-003), so xtask must download and verify
      them on three operating systems;
    - GitHub settings, items that only a release creates, and anything that
      needs a decision from the user.
11. **Contacts and commitments, for the user to confirm.**
    - Security reports: `SECURITY.md` sends them through GitHub private
      vulnerability reporting, which the user turns on.
    - Response times: acknowledge within 7 days, assess within 14 days, and fix
      or mitigate within 90 days.
    - Code of conduct: the reporting contact stays open, so USS-CTB-008 fails
      until the user gives one.
    - Succession: HaleTechCo, which owns the repository, appoints a new
      maintainer or archives the repository and says so in the README.
12. **Before the first release.** When a rule's Verify judges each release
    and no release exists, the auditor records `pass` with the evidence "no
    release yet". When it needs a published artifact (binaries, an SBOM,
    signatures, provenance), the auditor records `fail`. The Audit workflow
    says this, because every new project meets it.

## File structure

| Path | Task | Responsibility |
|---|---|---|
| `tools/uss/build.rs` | 1 | Also embeds the root files `AGENTS.md` and `CHANGELOG.md` when they exist. |
| `tools/uss/src/source.rs` | 1 | `ROOT_FILES`, `uss_files`, `uss_path`: the list of USS files and path normalising. |
| `tools/uss/src/catalog/export.rs` | 1 | `rule_markdown`: one rule's full text. |
| `tools/uss/src/catalog/markdown.rs`, `catalog/mod.rs` | 1 | One fence tracker; a test for the reach limit. |
| `tools/uss/src/cli/mod.rs` | 1, 3–5 | `show`, `applicable --full`; tests for documented commands, the CI template and the plugin version. |
| `standard/core/dependencies.md`, `core/supply-chain.md`, `conventions.md` | 2 | DEP-001 peers, SUP-006 latest-only suppliers, profile `Versions`. |
| `standard/modules/tools/maven.md`, `languages/typescript.md`, `vendors/eslint/eslint.md` | 2 | MVN-004 title, MVN-007 rationale, TS-007, TypeScript range. |
| `AGENTS.md`, `CLAUDE.md` | 3 | The workflows, agent conduct, work on this repository. |
| `templates/AGENTS.snippet.md`, `templates/ci/uss-release.yml` | 4 | What the adopt workflow copies into a project. |
| `.claude-plugin/plugin.json`, `.claude-plugin/marketplace.json`, `skills/adopt/SKILL.md`, `skills/audit/SKILL.md` | 5 | The Claude plugin. |
| `README.md`, `SECURITY.md`, `CHANGELOG.md`, `docs/cli.md`, `docs/architecture.md`, `docs/dependencies.md`, `docs/troubleshooting.md` | 6 | Documents for users. |
| `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `GOVERNANCE.md`, `.github/CODEOWNERS`, `.github/ISSUE_TEMPLATE/`, `.github/pull_request_template.md`, `.github/dependabot.yml`, `.githooks/pre-commit` | 7 | Documents and configuration for contributors. |
| `.uss/record.yaml`, `.uss/audits/<date>.yaml`, `.uss/audits/<date>.md`, `tools/xtask/src/main.rs` | 8–10 | This repository's record, audit and report; xtask runs `uss check` on the record. |
| Spec | 11 | Alignment with what the plan built. |

## Review Focus

1. **Paths that an agent gives `uss show`.** On Windows an agent writes
   `standard\core\security.md` or `./templates/record.yaml`. Both print the
   file. A path with `..`, or any path that is not in the listing, is refused
   with a hint to run `uss show`. It never reads a file outside the standard,
   with `--standard-root` too. Task 1 tests.
2. **A record that pins another version than the installed `uss`.** Every
   command that reads the record refuses, and names the install command (Plan
   1 test). `uss show` reads no record, so it still works. The workflow checks
   `uss version` against the record before anything else (Task 3), and Task 1
   tests that `uss show` needs no record.
3. **An existing codebase with no record.** The Audit workflow starts the
   record with `uss show templates/record.yaml`, fills in the answers from the
   scan, and has the developer confirm each fact. The template's `false`
   values are placeholders (Task 3). The self-audit (Task 8) follows exactly
   this path.
4. **Documents drift from the CLI.** When a command or an option is renamed,
   the `documented_commands_exist` test fails (Task 3 creates it; Tasks 4–7
   add their files to it).
5. **A repository with several released units.** The workflows make one
   record for each unit and list them in `components` (conventions §11
   "Components"). Task 3 writes this. The reviewer checks it against
   conventions §11, because the self-audit has one unit.

---

### Task 1: `uss show`, `uss applicable --full`, embedded workflows

**Files:**
- Modify: `tools/uss/build.rs`, `tools/uss/src/source.rs`,
  `tools/uss/src/catalog/export.rs`, `tools/uss/src/catalog/markdown.rs`,
  `tools/uss/src/catalog/mod.rs` (test only), `tools/uss/src/cli/mod.rs`

**Interfaces:**
- Produces:
  - `pub const ROOT_FILES: [&str; 2] = ["AGENTS.md", "CHANGELOG.md"]` and
    `pub const USS_DIRS: [&str; 3] = ["standard", "schema", "templates"]` in
    `source.rs`.
  - `pub fn uss_files(src: &dyn Source) -> Vec<String>`: the files under
    `USS_DIRS`, plus each root file that `src` can read, sorted.
  - `pub fn uss_path(input: &str) -> String`: turns `\` into `/` and removes
    each leading `./`.
  - `pub fn rule_markdown(rule: &ExportRule) -> Option<String>` in
    `catalog/export.rs`: `None` for a retired rule.
  - CLI: `uss show [PATH]` and `uss applicable --full <record>`.

- [ ] **Step 1: Write the failing tests**

In `source.rs`:
- `uss_files_lists_dirs_and_root_files`: a `MemSource` with
  `standard/a.md`, `schema/s.json`, `templates/t.yaml`, `AGENTS.md`,
  `README.md` and `other/x.md`. The result is exactly `["AGENTS.md",
  "schema/s.json", "standard/a.md", "templates/t.yaml"]`.
- `uss_path_normalises`: `.\standard\core\a.md` gives `standard/core/a.md`;
  `./templates/record.yaml` gives `templates/record.yaml`; `standard/x.md`
  is unchanged.

In `catalog/export.rs`:
- `rule_markdown_layout`: a fixture MUST rule with one `Refines` ID gives
  exactly this text. Each field is printed in the order that conventions §3
  lists the fields, then `File`. `Applies when` is the effective condition,
  as `uss export` gives it. `Enables` and `Refines` lines appear only when
  they are not empty.

  ```text
  ### USS-ABC-001: Title

  - **Level:** MUST
  - **Applies when:** <effective condition>
  - **Category:** Safety
  - **Refines:** USS-SEC-001
  - **Verify:** <verify text>
  - **References:** <references>
  - **File:** standard/modules/x.md

  <body>
  ```
- `rule_markdown_retired_is_none`.

In `catalog/mod.rs`:
- `refines_reach_over_limit`: a module rule refines a core rule, and their
  conditions together name 17 distinct identifiers. `load` reports one
  diagnostic that contains `over the limit of 16`.

In `cli/mod.rs`, with the fixture root and `--standard-root`:
- `show_prints_file`: `uss show templates/record.yaml` exits 0 and prints the
  file's bytes exactly.
- `show_lists_files`: `uss show` exits 0 and prints `uss_files` of the
  fixture, one path per line.
- `show_accepts_windows_paths`: `uss show .\templates\record.yaml` prints the
  same bytes as `show_prints_file`.
- `show_unknown_path`: `uss show standard/nope.md` exits 1, prints nothing to
  stdout, and stderr contains `no file "standard/nope.md"` and `uss show`.
- `show_refuses_outside_paths`: write `secret.txt` at the fixture root,
  outside the USS directories. `uss show secret.txt`, `uss show
  standard/../secret.txt` and `uss show ../secret.txt` each exit 1 and print
  nothing to stdout.
- `show_needs_no_record`: `uss show` works from a directory that has no
  `.uss/`.
- `applicable_full_prints_rule_text`: `uss applicable --full <record>` exits
  0. The output holds `rule_markdown` for each applicable rule, in the same
  order as `uss applicable`, separated by one blank line.
- `applicable_full_conflicts_with_json`: `--full --json` exits 2.

- [ ] **Step 2: Run the tests to verify that they fail**

Run: `cargo test --locked -p uss`
Expected: the new tests fail to compile or fail.

- [ ] **Step 3: Implement**

- `build.rs` embeds each name in `ROOT_FILES` that exists at the repository
  root (with `cargo:rerun-if-changed` for each, present or not), together
  with the three directories. `build.rs` cannot import from the crate, so it
  keeps its own copy of the two lists, with a comment that names
  `source::ROOT_FILES` and `source::USS_DIRS`.
- `uss show PATH` normalises with `uss_path`, then prints the file only when
  the normalised path is in `uss_files(src)`. That check is what keeps
  `--standard-root` inside the standard. The error message is: `uss show: no
  file "<path>" in USS <version>. Run "uss show" to list the files.`
- `--full` uses `rule_markdown` on the export of each applicable rule, and
  clap's `conflicts_with = "json"`.
- Fences: one tracker type in `markdown.rs`, used by `parse_doc` and
  `unfenced`. It replaces their two copies of the fence loop. The existing
  fence tests stay unchanged and pass.

- [ ] **Step 4: Run the tests to verify that they pass**

Run: `cargo xtask check`
Expected: exit 0; the new tests pass, and `uss lint: ok (443 rules, 61 files)`.

- [ ] **Step 5: Commit**

```bash
git add tools/uss
git commit -m "feat(uss): add show and applicable --full, embed the workflows"
```

### Task 2: Plan 4's parked items in the standard

**Files:**
- Modify: `standard/core/dependencies.md` (USS-DEP-001),
  `standard/core/supply-chain.md` (USS-SUP-006), `standard/conventions.md`
  (§12), `standard/modules/tools/maven.md` (USS-MVN-004, USS-MVN-007),
  `standard/modules/languages/typescript.md` (USS-TS-007),
  `standard/vendors/eslint/eslint.md` (only if Step 6 finds a gap), the spec
  (§6.3)

- [ ] **Step 1: USS-DEP-001, peer ranges.** Add to the body and to Verify: a
  peer dependency range (npm `peerDependencies`, or the same field of another
  ecosystem) may admit several major versions, such as `^18 || ^19`. It still
  has an upper bound, so `*`, `latest` and an unbounded `>=` fail. Add one
  rationale sentence: a library can support several majors of its host.
  Confirm that USS-TS-009's sentence "`peerDependencies` are left to
  USS-DEP-001" is still true.
- [ ] **Step 2: USS-SUP-006, latest-only suppliers.** Add to the body and to
  Verify: when a supplier gives security fixes only in its latest release
  (such as Rust), the release before the latest also counts as supported.
  The rationale says that this is a USS addition: it gives about one release
  cycle to update a pin. Check that USS-GO-007, which refines USS-SUP-006,
  still agrees with it.
- [ ] **Step 3: Conventions §12, versions outside the range.** After the
  header table, add: a project may use a tool version outside the profile's
  `Versions`. The auditor then checks that each tool rule or setting that the
  baseline and the mappings name exists in that version. A mapping whose tool
  rule does not exist there gives no evidence for its rule.
- [ ] **Step 4: Retitle USS-MVN-004** to "Bound each dependency version". Search
  the repository for the old title and update each match.
- [ ] **Step 5: USS-MVN-007, rationale.** Add one sentence: a project that keeps
  a dependency that the analysis reports as unused (because it deviates from
  USS-DEP-013, or because the program loads the dependency by reflection)
  lists it in `ignoredUnusedDeclaredDependencies`, with a reason
  (USS-QLT-007). First confirm the parameter in the Maven Dependency Plugin
  3.11.0 `analyze` documentation, and cite it.
- [ ] **Step 6: USS-TS-007 and USS-TS-001.**
  - USS-TS-007: add to the body and to Verify that CI installs the pinned
    Node.js and package manager. For example, `actions/setup-node` reads the
    version file, and Corepack or the package manager's setup reads
    `packageManager`. Confirm each named mechanism in its current
    documentation.
  - Read typescript-eslint's current supported TypeScript range. If the newest
    TypeScript major is outside it, add a sentence to the ESLint profile's
    free text that states the supported range. Then add to USS-TS-001's free
    text (not its body) that a project on a newer major needs a type-aware
    linter that supports that major. Name a linter only when its current
    documentation shows that support. Otherwise change nothing, and say so in
    the report.
- [ ] **Step 7: Spec §6.3.** Replace "refines the same core rules as its
  outcome" with the wording of conventions §7 ("refines the core rules whose
  outcome it serves").
- [ ] **Step 8: Verify.** Run `cargo xtask check`. Expected: exit 0, and `uss
  lint: ok` with the same rule count (no rule added or removed).
- [ ] **Step 9: Commit**

```bash
git add standard docs/superpowers/specs
git commit -m "fix(standard): settle the items that Plan 4 parked"
```

### Task 3: AGENTS.md and CLAUDE.md

**Files:**
- Create: `AGENTS.md`, `CLAUDE.md`
- Modify: `tools/uss/src/cli/mod.rs` (test), `tools/uss/src/source.rs` (test)

**Interfaces:**
- Consumes: `uss show`, `uss applicable --full` (Task 1).
- Produces: the `AGENTS.md` headings, which Task 5's skills and Task 4's
  snippet name exactly: `## Use USS on a project`, `### Before every
  workflow`, `### Questions to the developer`, `### Adopt a new project`,
  `### Audit an existing project`, `### Projects with several released
  units`, `### Release audit`, `### Upgrade the pinned version`, `## Agent
  conduct`, `## Work on this repository`. Also the test constant `DOC_FILES`,
  which later tasks add to.

- [ ] **Step 1: Write the failing tests**
- `embedded_has_agents_md` in `source.rs`: `Embedded.read("AGENTS.md")` is
  `Some`.
- `documented_commands_exist` in `cli/mod.rs`: for each entry of `const
  DOC_FILES: &[(&str, &str)] = &[("AGENTS.md",
  include_str!("../../../../AGENTS.md"))]` (name and text), find every `uss`
  command and check it against clap's command tree:
  - A command is an inline code span, or a line of a fenced block, whose
    first word is `uss`. A span or line that starts with `cargo run` and
    holds `-p uss --` is the command `uss` followed by the words after `--`.
    A span or line that starts with any other word (such as `cargo install
    --path tools/uss --force`) is not a command.
  - A leading `--standard-root <DIR>` is skipped.
  - The first word that does not start with `-` names a subcommand. Each
    `--name` word is a long option of that subcommand or of `uss`.
  - `uss --version`, `uss -V`, `uss --help` and `uss -h` on their own are
    valid.
  - The test fails with the file, the line and the command.

- [ ] **Step 2: Run the tests to verify that they fail.** Run `cargo test
  --locked -p uss`. Expected: both fail, because `AGENTS.md` does not exist
  yet.

- [ ] **Step 3: Write `AGENTS.md`**

Write it in the ASD-STE100 style (Global Constraints). Every procedure is a
numbered list. Open with two or three sentences on what USS is, and which
section each reader needs. The sections contain this:

- **Before every workflow:**
  - Find the version: `standard.version` in `.uss/record.yaml`, or, with no
    record, the version of the installed `uss`.
  - Run `uss version`. It must print that version.
  - Otherwise, install that version: `cargo install --locked --git
    https://github.com/HaleTechCo/Universal-Software-Standard --tag
    v<version> uss`. From a USS checkout, use `cargo install --locked --path
    tools/uss`.
  - Read the workflows of that version with `uss show AGENTS.md`. Follow them,
    not a copy of another version.
  - Read conventions sections 5, 8, 9 and 11 with `uss show
    standard/conventions.md`.
  - Read the rules that apply with `uss applicable --full .uss/record.yaml`.
  - Never compute applicability, scores or compliance by hand. Without `uss`,
    stop and install it.
- **Questions to the developer:** decision 3.
  - Each batch gives the recommended answer, and the score category of each
    MAY.
  - For each SHOULD or SHOULD NOT that the developer declines, ask for the
    reason. Then draft a Standard Deviation at
    `docs/standard-deviations/<RULE-ID>.md` from `uss show
    templates/standard-deviation.md`.
  - The developer, under their own organisation's policy, approves it. The
    agent never fills in `approved-by`.
  - Tell the developer that a MUST deviation makes the project "not
    compliant".
- **Adopt a new project:** spec §8.1, steps 1–7, with commands:
  - Start the record from `uss show templates/record.yaml`. Take stack
    identifiers from `standard/registry.yaml`.
  - Answer every fact; the template's `false` values are placeholders.
  - Run `uss applicable`. Ask the SHOULD and MAY questions.
  - After the developer adopts a MAY, run `uss applicable` again. Ask the
    SHOULD questions for the new rules only.
  - Run `uss check .uss/record.yaml` and fix each problem.
  - Add `templates/AGENTS.snippet.md` to the project's `AGENTS.md` (or
    `CLAUDE.md`).
  - When the project publishes releases on GitHub, copy
    `templates/ci/uss-release.yml`.
  - Build to the rules.
- **Audit an existing project:** spec §8.2, steps 1–7.
  - The quick scan detects these: languages, frameworks, build tools, CI,
    containers, UI, databases, authentication, network use, releases and
    docs.
  - With no record, start one as in Adopt, with the answers taken from the
    scan.
  - Audit the committed state: the working tree is clean, and `commit` is
    `git rev-parse HEAD`.
  - Apply each applicable rule's Verify exactly as written, with nothing added
    and nothing left out.
  - The evidence of `pass`, `fail` and `deviation` names a `file:line`, a
    configuration key, a CI job, or a command and its output.
  - When the auditor cannot decide, record `unverified` and never guess.
    Decision 12 covers projects with no release yet.
  - Write the results area by area into `.uss/audits/<date>.yaml`, so that a
    stopped session can continue. Fill in `suppressions` with every
    suppression in the project's checks (conventions §11).
  - Add the audit to `audits`. Run `uss check`, then `uss report
    .uss/audits/<date>.yaml`. Commit the record, the results and the report
    together.
  - Fix only with the developer's go-ahead: MUST failures first, then the
    SHOULDs that the developer chose to comply with, in batches by area.
  - Verify each fix before its result changes to `pass`. After a batch,
    commit the fixes. Then set `commit` to the new `HEAD`, check again each
    result whose evidence the fixes touched, and run `uss report` again.
- **Projects with several released units:** conventions §11 "Components".
  There is one record for each unit, and the root record lists them. Every
  record pins the same version. Audit each record. `uss release-check`
  covers them all.
- **Release audit:** spec §8.3.
  - With a record at the same version and the same facts, ask nothing.
  - Locally: audit, commit the results, then tag that commit.
  - In CI: `uss release-check` checks the release (the template). With
    `--require-compliant`, it fails when a record is not compliant.
  - With no person present, follow decision 5.
- **Upgrade the pinned version:** spec §8.4.
  - Install the new `uss`. Read `uss show CHANGELOG.md` for the rule IDs that
    were added, changed and retired between the two versions.
  - On a branch, change `standard.version` and answer the new facts. Ask
    questions only about added and changed rules. Remove decisions for
    retired rules.
  - Run `uss check`, then run a full audit.
  - Merge the branch when the project is compliant. `uss` refuses a record
    that pins another version, so the upgrade happens on a branch.
- **Agent conduct:** spec §6.1 CON. These rules bind every agent that follows
  the workflows, but audits do not count them:
  - No silent scope reduction.
  - Report only results that you ran and saw. CI is the source of truth for
    pass and fail.
  - No changes outside the task.
  - Do not turn off a check or skip a test to make it pass.
- **Work on this repository:** USS-CTB-007.
  - Commands:
    - `cargo xtask check` sets up and runs every check. The first run
      installs the pinned toolchain and cargo-deny under `.tools`.
    - `cargo run --locked -p uss -- lint` checks the standard files.
    - `cargo test --locked -p uss` runs the tests.
  - Conventions:
    - Follow `standard/conventions.md` before you write a rule.
    - IDs are never reused.
    - The version stays 0.1.0 until the release.
    - `Initial_Idea.md` and `todo.txt` are never changed.
    - Commit messages use `type(scope): summary`, as in `git log`.
    - Steps follow ASD-STE100.
  - Definition of Done:
    - `cargo xtask check` passes.
    - Changed behaviour has tests.
    - Documents change together with any command that changes.
    - Each changed reference was read.

`CLAUDE.md` holds only the line `@AGENTS.md`.

- [ ] **Step 4: Check the writing.** Take ten steps from the workflows and
  check each one against the ASD-STE100 rules in Global Constraints. Correct
  each one that breaks a rule. List the ten in the report.
- [ ] **Step 5: Verify.** Run `cargo xtask check`. Expected: exit 0; the two
  new tests pass.
- [ ] **Step 6: Commit**

```bash
git add AGENTS.md CLAUDE.md tools/uss
git commit -m "docs: add the agent workflows in AGENTS.md"
```

### Task 4: Adopter templates

**Files:**
- Create: `templates/AGENTS.snippet.md`, `templates/ci/uss-release.yml`
- Modify: `tools/uss/src/cli/mod.rs` (tests); `AGENTS.md` only if a name
  changed

- [ ] **Step 1: Write the failing tests**
- Add `templates/AGENTS.snippet.md` and `templates/ci/uss-release.yml` to
  `DOC_FILES` (each a name and an `include_str!` of the file).
- `ci_template_is_hardened`: read the embedded `templates/ci/uss-release.yml`
  and parse it as YAML with `serde-saphyr`. Then assert these:
  - `on.release.types` is exactly `["published"]`.
  - The top-level `permissions` is exactly `{contents: read}`.
  - Every `uses:` value matches `^[^@]+@[0-9a-f]{40}$`.
  - The checkout step sets `fetch-depth: 0`.
  - One step runs `uss release-check`.
  - No `run:` value contains `${{`.

- [ ] **Step 2: Run the tests to verify that they fail.** Run `cargo test
  --locked -p uss`. Expected: they fail.
- [ ] **Step 3: Write the snippet.** Start with the heading `## Universal
  Software Standard (USS)`. Write it in the ASD-STE100 style. It contains:
  - The project follows USS at the version that `.uss/record.yaml` pins. The
    snippet names no version.
  - Before you change the project, run `uss applicable --full
    .uss/record.yaml`.
  - Follow every applicable MUST and MUST NOT rule. A MUST deviation makes the
    project not compliant.
  - Follow every SHOULD and SHOULD NOT rule unless the record lists a
    deviation for it.
  - Do not change the record's facts, decisions or deviations without the
    developer.
  - To audit, follow the Audit workflow in `uss show AGENTS.md`.
  - An `### Agent conduct` section that repeats the four rules of AGENTS.md
    word for word.
- [ ] **Step 4: Write the CI template.**
  - Header comments: copy the file to `.github/workflows/`, and set
    `USS_VERSION` to the record's version.
  - It runs on `release: types: [published]`, with top-level `permissions:
    contents: read`.
  - One job has `contents: write`, so it can upload the report to the
    release. Its steps:
    1. `actions/checkout`, pinned to the SHA in this repository's CI, with
       `fetch-depth: 0` and `persist-credentials: false`.
    2. Install the Rust toolchain that `uss` needs (`rust-version` in
       `Cargo.toml`).
    3. `cargo install --locked --git
       https://github.com/HaleTechCo/Universal-Software-Standard --tag
       "v${USS_VERSION}" uss`.
    4. `uss release-check --out uss-report.md`, with a comment about
       `--require-compliant`.
    5. `gh release upload "$TAG" uss-report.md --clobber`, with `TAG` and
       `GH_TOKEN` set through `env`.
- [ ] **Step 5: Verify.** Run `cargo xtask check`. Expected: exit 0.
- [ ] **Step 6: Commit**

```bash
git add templates tools/uss AGENTS.md
git commit -m "feat(templates): add the AGENTS.md snippet and the release audit workflow"
```

### Task 5: The Claude plugin

**Files:**
- Create: `.claude-plugin/plugin.json`, `.claude-plugin/marketplace.json`,
  `skills/adopt/SKILL.md`, `skills/audit/SKILL.md`
- Modify: `tools/uss/src/cli/mod.rs` (tests)

- [ ] **Step 1: Write the failing tests**
- Add `skills/adopt/SKILL.md` and `skills/audit/SKILL.md` to `DOC_FILES`.
- `plugin_matches_standard`: parse `.claude-plugin/plugin.json` and
  `.claude-plugin/marketplace.json` (through `include_str!`) with
  `serde_json`. Then assert these:
  - The plugin's `version` equals the registry version of the embedded
    standard.
  - Its `name` is `uss`.
  - The marketplace `name` is `universal-software-standard`.
  - It has exactly one plugin, with name `uss` and source `./`.
  - Each skill body names `${CLAUDE_PLUGIN_ROOT}/AGENTS.md` and a heading
    that exists in `AGENTS.md`.
- [ ] **Step 2: Run the tests to verify that they fail.**
- [ ] **Step 3: Write the manifests.**
  - `plugin.json` sets `name`, `version` `0.1.0`, `description`, `author`
    (`{"name": "HaleTechCo"}`), `homepage` and `repository` (the GitHub URL),
    `license` `MIT` and `keywords`.
  - `marketplace.json` sets `name`, `owner` (`{"name": "HaleTechCo"}`) and
    one entry: `name` `uss`, `source` `./` and a `description`. The entry has
    no `version`, so `plugin.json` alone sets it.
- [ ] **Step 4: Write the skills.**
  - The frontmatter has `description` and `argument-hint`, under the
    documented limit of 1,536 characters.
  - The body is in the ASD-STE100 style and under 40 lines. It says to read
    `${CLAUDE_PLUGIN_ROOT}/AGENTS.md`, then follow the named workflow from
    "Before every workflow" on.
  - `adopt` follows "Adopt a new project".
  - `audit` follows "Audit an existing project". With the argument `release`,
    it follows "Release audit". With `upgrade <version>`, it follows "Upgrade
    the pinned version". With `headless`, it follows the rules for no person
    present.
  - Neither skill copies workflow steps.
- [ ] **Step 5: Validate.** Run `claude plugin validate .`. Expected: no
  errors. The only warning allowed is the one about the root `CLAUDE.md`. If
  `claude --version` fails, the report says "not run".
- [ ] **Step 6: Verify.** Run `cargo xtask check`. Expected: exit 0.
- [ ] **Step 7: Commit**

```bash
git add .claude-plugin skills tools/uss
git commit -m "feat(plugin): add the uss Claude plugin with the adopt and audit skills"
```

### Task 6: Documents for users

**Files:**
- Create: `SECURITY.md`, `CHANGELOG.md`, `docs/cli.md`,
  `docs/architecture.md`, `docs/dependencies.md`, `docs/troubleshooting.md`
- Modify: `README.md`, `tools/uss/src/cli/mod.rs` (add each file that names
  `uss` commands to `DOC_FILES`)

Each document meets the Verify text of the rule in brackets. Read the rule
with `cargo run --locked -q -p uss -- show <file>` before you write.

- [ ] **Step 1: README.md** (USS-DOC-001, USS-DST-009, USS-DST-012):
  - What USS is, and its status (0.x, unreleased).
  - How to adopt and how to audit, with an agent: `/uss:adopt` and
    `/uss:audit`, or "follow the workflow in AGENTS.md".
  - **"Try USS before its first release"**, for the user's trial:
    1. `cargo install --locked --path tools/uss` in the USS checkout.
    2. `/plugin marketplace add <path of the checkout>`, then `/plugin
       install uss@universal-software-standard`.
    3. In the other project, run `/uss:audit`.
    4. After you change USS, run the install command again with `--force`.
       The binary carries the standard, so a change reaches it only through
       a new install. `/reload-plugins` loads the changed workflows.
  - Install from a release tag, upgrade, roll back to an earlier tag, and
    uninstall (`cargo uninstall uss`, `/plugin uninstall`).
  - The development section points to AGENTS.md and CONTRIBUTING.md.
- [ ] **Step 2: SECURITY.md** (USS-SEC-029, USS-SEC-030, USS-SEC-031,
  USS-REL-004), following decision 11:
  - Report privately through GitHub private vulnerability reporting.
  - What happens after a report, the response times, and that fixes come
    with a GitHub security advisory.
  - Supported versions: the latest 0.x release.
- [ ] **Step 3: CHANGELOG.md** (USS-REL-005): the heading `# Changelog` only.
  The 0.1.0 entry is written at the release (Common Changelog keeps no
  "Unreleased" section; check this in the guide and cite it in the report).
- [ ] **Step 4: docs/cli.md** (USS-DOC-003, USS-DOC-004, USS-UX-008): each
  command and option of `uss`, from `--help`, with its default. It also
  covers the exit codes 0, 1 and 2, and the fact that `uss` reads no
  environment variable and no configuration file. Say which option, if any,
  weakens a check.
- [ ] **Step 5: docs/architecture.md** (USS-DOC-002): the library and the thin
  CLI (I/O only in `cli/`), the embedded standard, the modules `catalog`,
  `evaluate`, `record`, `report` and `release`, and the ADRs.
- [ ] **Step 6: docs/dependencies.md** (USS-DEP-011): each direct dependency in
  `tools/uss/Cargo.toml` and `tools/xtask/Cargo.toml`, with its purpose and
  why it is needed.
- [ ] **Step 7: docs/troubleshooting.md** (USS-DOC-006): a record that pins
  another version; a stale binary after a change to the standard
  (reinstall, or `--standard-root`); `uss check` problems; `release-check` in
  a shallow clone; the plugin skill that does not appear (`/reload-plugins`).
- [ ] **Step 8: Verify.** Run `cargo xtask check`. Expected: exit 0, and
  `documented_commands_exist` covers the new files.
- [ ] **Step 9: Commit**

```bash
git add README.md SECURITY.md CHANGELOG.md docs/cli.md docs/architecture.md docs/dependencies.md docs/troubleshooting.md tools/uss
git commit -m "docs: add the user documents and the trial instructions"
```

### Task 7: Documents and configuration for contributors

**Files:**
- Create: `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `GOVERNANCE.md`,
  `.github/CODEOWNERS`, `.github/ISSUE_TEMPLATE/bug.md`,
  `.github/ISSUE_TEMPLATE/rule.md`, `.github/ISSUE_TEMPLATE/config.yml`,
  `.github/pull_request_template.md`, `.github/dependabot.yml`,
  `.githooks/pre-commit`
- Modify: `AGENTS.md` (the hooks command), `tools/uss/src/cli/mod.rs`
  (`DOC_FILES`)

Read each rule in brackets with `uss show` first.

- [ ] **Step 1: CONTRIBUTING.md** (USS-CTB-006, USS-CTB-005, USS-CTB-013,
  USS-CTB-015, USS-CON-002, USS-DEP-012):
  - How to add a rule, an area, a module, a vendor profile and a regulation
    module (point to the conventions sections). The ID policy.
  - The Definition of Done (the same as AGENTS.md).
  - How changes are reviewed, and a first response within a stated number of
    days.
  - How long `cargo xtask check` takes: run it, measure a cold run and a warm
    run on this machine, and state both.
  - The health checks before a dependency is added.
  - Contributors are credited in the changelog.
  - Licence: contributions are under MIT.
  - Vendor-maintained profiles and CODEOWNERS.
  - Do not ask for a DCO sign-off or a CLA (USS-CTB-009 is the user's decision,
    in Task 11's list).
- [ ] **Step 2: CODE_OF_CONDUCT.md** (USS-CTB-008): the Contributor Covenant
  3.0 text, from its official source, with its attribution. Decision 11
  leaves the reporting contact open: write one plain sentence that the
  contact is added before the first release. Do not keep the template's
  placeholder.
- [ ] **Step 3: GOVERNANCE.md** (USS-CTB-017): the maintainer role, how
  decisions are made, and the succession of decision 11.
- [ ] **Step 4: .github files.**
  - `CODEOWNERS`: `*` and `/standard/vendors/`, both owned by the login that
    `gh api user --jq .login` prints.
  - Issue templates for a bug and for a rule proposal. `config.yml` links to
    `SECURITY.md` for vulnerabilities.
  - A pull request template with the Definition of Done as a checklist
    (USS-CTB-010).
- [ ] **Step 5: .github/dependabot.yml** (USS-DEP-014, USS-DEP-015): the
  ecosystems `cargo` and `github-actions`, weekly, with `cooldown`
  `default-days` of at least 3. Confirm the keys in the current Dependabot
  options reference.
- [ ] **Step 6: .githooks/pre-commit** (USS-CTB-004): a POSIX `sh` script with
  `set -eu` that runs `cargo fmt --all --check`, then `cargo run --locked -q
  -p uss -- lint`. Add the line `git config core.hooksPath .githooks` to
  AGENTS.md "Work on this repository" and to CONTRIBUTING.md. Commit the file
  as executable (`git update-index --chmod=+x`).
- [ ] **Step 7: Verify.** Run `cargo xtask check`. Expected: exit 0. Then run
  `sh .githooks/pre-commit`. Expected: exit 0.
- [ ] **Step 8: Commit**

```bash
git add CONTRIBUTING.md CODE_OF_CONDUCT.md GOVERNANCE.md .github .githooks AGENTS.md tools/uss
git commit -m "docs: add the contributor documents, templates and hooks"
```

### Task 8: Self-audit, part 1 (record; Safety, Reliability, Efficiency, Fairness)

**Files:**
- Create: `.uss/record.yaml`, `.uss/audits/<date>.yaml`
- Modify: `tools/xtask/src/main.rs`

This task tests the workflow as much as it audits the repository. Follow
`AGENTS.md` "Before every workflow" and "Audit an existing project" as
written. Use `cargo run --locked -q -p uss --` wherever the workflow says
`uss`. Decision 9 gives the developer's answers.

- [ ] **Step 1: The record.** Run the quick scan. Start the record from `uss
  show templates/record.yaml`, with the answers of decision 9 and
  `project.name: universal-software-standard`. Run `uss applicable
  .uss/record.yaml`. Expected: 145 rules (7 MAY, 74 MUST, 14 MUST NOT, 45
  SHOULD, 5 SHOULD NOT). If the count differs, explain the difference in the
  report. Add the MAY decisions. Run `uss check .uss/record.yaml`. Expected:
  `ok`.
- [ ] **Step 2: xtask.** Add a step `==> uss check` after `uss lint` that runs
  `uss check .uss/record.yaml`. Add a test that the step list holds it, if
  xtask has a test that lists steps; otherwise it needs no test.
- [ ] **Step 3: Commit the record**

```bash
git add .uss/record.yaml tools/xtask
git commit -m "chore(uss): add this repository's USS record"
```

- [ ] **Step 4: Audit.** Audit the commit of Step 3 (`git rev-parse HEAD`,
  working tree clean) for every applicable rule in the categories Safety,
  Reliability, Efficiency and Fairness (about 59 rules).
  - Use Verify exactly, with the evidence formats of the workflow.
  - For GitHub settings, use only read-only `gh api` GET calls. Without
    access, follow the rule's Verify, which usually says `unverified`.
  - Do not install a tool that a Verify runs. Record what you could not run.
  - Write `suppressions` for every suppression in the repository's checks:
    `#[expect]` and `#[allow]` attributes in `tools/`, and the ignores and
    exceptions in `deny.toml` and `clippy.toml`.
  - Use `auditor: claude`.
- [ ] **Step 5: Workflow feedback.** In the report, list each place where the
  AGENTS.md workflow was unclear, wrong or missing a step, with what you did
  in its place. Task 10 fixes these.
- [ ] **Step 6: Commit the results**

```bash
git add .uss/audits
git commit -m "chore(uss): audit Safety, Reliability, Efficiency and Fairness rules"
```

### Task 9: Self-audit, part 2 (the other categories; the report)

**Files:**
- Modify: `.uss/audits/<date>.yaml`, `.uss/record.yaml`
- Create: `.uss/audits/<date>.md`

- [ ] **Step 1: Audit** the same commit as Task 8 (its `commit` value; only
  `.uss/` has changed since). Cover every remaining applicable rule:
  Maintainability, Contributability, Compatibility, Accessibility & Usability
  and Features. Use the same method as Task 8 Step 4. Each declined MAY gets
  `declined`. Each adopted MAY is judged by its Verify and decision 12.
- [ ] **Step 2: Record the audit.** Add the audit to `audits` in the record,
  then run `uss check .uss/record.yaml`. Expected: `ok`.
- [ ] **Step 3: Report.** Run `uss report .uss/audits/<date>.yaml`. It writes
  the `.md` file and prints the statement and the scores. Copy them into the
  task report. The statement is expected to be "Not compliant"; the report
  lists the failures.
- [ ] **Step 4: Workflow feedback**, as in Task 8 Step 5.
- [ ] **Step 5: Verify.** Run `cargo xtask check`. Expected: exit 0.
- [ ] **Step 6: Commit**

```bash
git add .uss
git commit -m "chore(uss): complete this repository's self-audit and report"
```

### Task 10: Fix with files, and fix the workflow

**Files:** depends on the findings. Only documents, CI YAML, templates,
`.github/` and `.githooks/` files, `AGENTS.md`, the skills, and `.uss/`.
Nothing under `tools/` and nothing under `standard/`.

- [ ] **Step 1: Sort the findings.** For each `fail` and `unverified` result
  of Tasks 8–9, decide whether files alone can fix it, within decision 10's
  limits and without a user decision. List the others, each with the rule
  ID, why it is not fixed here, and what it needs. That list goes to Task 11.
- [ ] **Step 2: Fix the workflow.** Apply the workflow feedback of Tasks 8
  and 9 to `AGENTS.md` (and to the skills or the snippet, if they are
  affected). Keep the ASD-STE100 style and the heading names.
- [ ] **Step 3: Fix the files.** Make each fix from Step 1. Verify each one
  against its rule's Verify.
- [ ] **Step 4: Verify and commit the fixes.** Run `cargo xtask check`.
  Expected: exit 0. Then commit:

```bash
git add -A -- . ':!todo.txt' ':!Initial_Idea.md'
git commit -m "fix: meet the self-audit findings that files can fix"
```

- [ ] **Step 5: Audit again.**
  - Set `commit` in the results to the commit of Step 4, and `date` to
    today. If the date changed, rename both files and update the record.
  - Check each result whose evidence Steps 2–3 touched, and each result of a
    fixed rule. Change only what the new state changes.
  - Run `uss check`, then `uss report`. Copy the new statement and scores
    into the report.
- [ ] **Step 6: Commit**

```bash
git add .uss
git commit -m "chore(uss): audit again after the fixes"
```

### Task 11: Align the spec and hand over

**Files:**
- Modify: the spec

- [ ] **Step 1: Align the spec** with what this plan built:
  - §8: the batches of questions, the headless rules, components, and before
    the first release.
  - §9: `uss show`, `uss applicable --full`, the embedded `AGENTS.md` and
    `CHANGELOG.md`, the skill names and the plugin details, and the new
    files (`CODE_OF_CONDUCT.md`, `GOVERNANCE.md`, `docs/`).
  - §12: the answer on how a skill gets the pinned version.
  - §13: step 4 is done, and step 5 waits for the user's trial.
  - Change no other section.
- [ ] **Step 2: Verify.** Run `cargo xtask check`. Expected: exit 0.
- [ ] **Step 3: Commit**

```bash
git add docs/superpowers/specs
git commit -m "docs(spec): align the design with the workflows, plugin and self-audit"
```

- [ ] **Step 4: Hand over.** The final message to the user gives these:
  - The self-audit statement and scores.
  - How to start the trial (the README section).
  - **The release plan's list**, grouped:
    1. User decisions: the conduct contact (USS-CTB-008); confirm the
       response times (USS-SEC-030) and the succession (USS-CTB-017); DCO
       sign-off (USS-CTB-009); a second admin (USS-CTB-016), or a deviation;
       which failing SHOULDs to fix or to deviate from.
    2. GitHub settings: branch protection and required checks (USS-SUP-007,
       USS-SUP-008, USS-CTB-003), private vulnerability reporting, newcomer
       labels (USS-CTB-011), a mirror (USS-BAK-013).
    3. Code work: secret scanning and static security analysis in xtask and
       CI (USS-SEC-005, USS-SEC-028), and every Task 10 Step 1 item that
       needs code.
    4. The release: a release workflow with binaries for x86-64 and ARM64
       (USS-DST-002, USS-DST-013, USS-DST-014); signatures and how to verify
       them (USS-SUP-014, USS-DST-015); an SBOM (USS-SUP-013); SLSA Build
       Level 2 provenance (USS-SUP-015); reproducible builds (USS-REL-003);
       the 0.1.0 changelog entry with its rule IDs; the `v0.1.0` tag; the
       audit of the release commit, with `uss release-check` and the
       published report (USS-REL-011, USS-REL-012); the template for a
       headless agent audit; a public repository and a public marketplace;
       and changes from the user's trial.
  - Every ruling made during the run.
