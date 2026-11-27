# USS follow-ups

[remaining.md](remaining.md) replaces this list from 2026-11-27.

This file lists all the open work after Plan 5. On 2026-11-25, `main` is
at dd5ab83 and is pushed. USS 0.1.0 is not released yet.

The recommendation for each item is in
[USS follow up recommendations](<research/reports/USS follow up recommendations.md>),
with the research notes beside it in `research/research_notes/`.

The sections follow the order of the work: the trial, the decisions, the
questions about the standard, the release plan, housekeeping, then later
work.

This repository's own audit (`.uss/audits/2026-11-25.md`, audited commit
9e57b95) says:

> Not compliant with USS 0.1.0 as of 2026-11-25: 23 MUST failures, 17 SHOULD
> failures, 2 unverified rules.

| Category | Score | Met | Applies |
|---|---|---|---|
| Safety | 69% | 32 | 46 |
| Reliability | 12% | 1 | 8 |
| Efficiency | 66% | 2 | 3 |
| Maintainability | 79% | 27 | 34 |
| Contributability | 62% | 10 | 16 |
| Compatibility | 100% | 16 | 16 |
| Fairness | 100% | 2 | 2 |
| Accessibility & Usability | 46% | 6 | 13 |
| Features | 14% | 1 | 7 |

## 1. The trial on an existing repository

The README section "Try USS before its first release" gives the steps.

- [ ] Optional: fix the install defect first (section 4.1). The trial does
      not need the fix, because you install `uss` before the audit.
- [ ] In this checkout, run `cargo install --locked --path tools/uss`.
- [ ] In Claude Code, run `/plugin marketplace add <path of this checkout>`.
- [ ] Run `/plugin install uss@universal-software-standard`.
- [ ] Make a branch in the project. Run `/uss:audit` on that branch.
- [ ] After each change to USS, install `uss` again with `--force`. Then run
      `/reload-plugins`. The binary carries the standard, so an old binary
      gives the old rules.

Note these points during the trial. They become the tweaks before the
release:

- The number of question rounds. Expect one table of facts, then one round
  for each area, which can be about 20 rounds.
- Facts whose definition does not fit the project.
- Rules whose Verify the agent cannot apply, or applies in a way that you do
  not agree with.
- The time that the audit takes. Also note the size of the
  `uss applicable --full` output.
- Steps in AGENTS.md that the agent skips or reads wrongly.

## 2. Decisions for you

### 2.1 Values in the documents

Plan 5 wrote these values. Confirm or change each one.

| Value | File |
|---|---|
| Security reports: acknowledge in 7 days, assess in 14 days, fix or mitigate in 90 days | `SECURITY.md` |
| Security channel: GitHub private vulnerability reporting (public repositories only) | `SECURITY.md` |
| First response to a contribution in 7 days | `CONTRIBUTING.md` |
| Succession: HaleTechCo appoints a new maintainer or archives the repository | `GOVERNANCE.md` |
| The GitHub login of the code owner | `.github/CODEOWNERS` |
| Code of conduct contact: still open, so USS-CTB-008 fails | `CODE_OF_CONDUCT.md` |

### 2.2 Repository visibility

The repository is private, on the free plan. That blocks several items:

- Branch protection and required checks: USS-SUP-007, USS-SUP-008 and
  USS-CON-003 (HTTP 403 from the API).
- GitHub private vulnerability reporting, the channel in `SECURITY.md`.
- Forks and runs from fork pull requests: USS-CTB-012.
- GitHub secret scanning and code scanning, which can help with USS-SEC-004,
  USS-SEC-005 and USS-SEC-028.
- A public marketplace for the plugin, and a CI template for headless audits.

The choices are to make the repository public, or to pay for a plan that has
these features.

### 2.3 Audit results that need your decision

| Rule | Decision |
|---|---|
| USS-CTB-008 | Give a contact for code of conduct reports. |
| USS-CTB-009 | Require a Developer Certificate of Origin (a `Signed-off-by` line and a CI check), or write a Standard Deviation. |
| USS-CTB-016 | Give admin access to a second person. |
| USS-DEP-006 | Two archived crates are in the lockfile: nohash-hasher 0.2.0 (through serde-saphyr) and cfg-if 1.0.5 (through ahash and encoding_rs). Replace them, write a Standard Deviation, or change the Verify for finished crates (section 3). |
| USS-BAK-013 | Mirror the repository to a second host automatically. |
| USS-DOC-007 | Get the ASD-STE100 specification, so that an audit can check the dictionary. It is unverified now. |
| USS-PRF-001 | Write a performance budget, such as the start-up time of `uss`. |

### 2.4 CI cost

Plan 5 added a weekly CI run on three operating systems. The run checks the
dependencies for new advisories. On a private repository it uses macOS
minutes, which cost more. Keep it, run it on Linux only, or make the
repository public.

### 2.5 Rulings to confirm or reverse

Plan 5 made these rulings while it ran:

| Ruling | Cost if wrong |
|---|---|
| R1: the README linked to `CONTRIBUTING.md` one task before that file existed. | One task with a broken link (now fixed). |
| R2: `uss applicable --full` prints the rule fields in the order of the rule files. | One function and its test. |
| R3: the new xtask step (`uss check`) has no unit test. | One missing test. |
| R4: in a headless audit, an open question that no rule depends on becomes `unverified` on USS-REL-011. | One workflow sentence. |
| R5: an upgrade merges when the project is compliant, or, if it was not compliant before, when the developer accepts the new report. | One workflow sentence. |
| R6: the documented-commands test checks the position of each option. | A stricter test. |
| R7: `overflow-checks = true` in the release profile of the root `Cargo.toml`. | One build setting. |
| R8: before the first release, a document about a release (such as rebuild steps) waits for the release. Other documents are checked now. | One result and one sentence. |
| R9: the Audit workflow adds the USS snippet, and offers the CI template, when the project has none. | One workflow step. |
| R10: the install defect is parked (section 4.1). | An agent can update the stable Rust toolchain until the fix. |

Other readings that you did not confirm:

- **Before the first release** (plan decision 12, AGENTS.md). USS-LIC-004,
  USS-DST-014, USS-DST-015 and USS-REL-003 pass with the evidence "no
  release yet". USS-DST-002, USS-SUP-013, USS-SUP-014 and USS-SUP-015 fail,
  because they need a published artifact.
- **Adopted MAY rules** (Plan 1). An adopted MAY rule that fails lowers the
  score, but does not change compliance (spec §5.5 as written).
- **Numbers that USS chose** (Plan 3):
  - USS-CTB-005: a margin of 25% on the stated time of the checks.
  - USS-DOC-006: an error that closed reports name more than once is common.
  - USS-PRF-005: 1% CPU over 60 seconds.
  - USS-SAF-004: a weekly schedule for sanitizer and race-detector runs.
  - USS-BAK-007: the production copy counts as a copy.

<details>
<summary>Rulings from Plans 3 and 4, taken as accepted</summary>

You moved on after each handover, so these count as accepted. You can still
reverse any of them.

Plan 3:

- USS-QLT-006 is met by turning on the baseline. USS-QLT-007 (a reason) and
  USS-QLT-008 (security groups) judge a baseline rule that is turned off.
- USS-TRU-004 is a MUST: commercial software lets users export their data,
  whatever the state of the subscription.
- USS-TRU-001 covers paywall moves and removal from the free edition only.
  USS-REL-007 covers removal from every edition.
- Accepted overlaps: USS-AI-001 with USS-SEC-009, -010, -013 and -014;
  USS-HRD-003 with USS-HRD-002.
- USS-OPS-002 leaves out alerts that other rules require (USS-BAK-010,
  USS-AUD-008, USS-AI-007).
- USS-AUD-004 and USS-AUD-006 allow removal only at expiry. USS-DAT-005 keeps
  audit records until their retention ends.
- USS-REL-001 and USS-EXT-002 follow SemVer as written, with no extra rule
  for 0.x.
- A source that could not be read is not cited. A conformance document
  behind a registration is cited by the version of its publisher.

Plan 4:

- USS-GO-006 pins the exact Go release, and CI uses it. USS-GO-010 is
  rewritten without GONOSUMCHECK.
- USS-RUST-010 no longer refines USS-SUP-005.
- USS-TS-015 uses pnpm `allowBuilds`. USS-TS-004 judges the effective
  tsconfig.
- USS-JAVA-001 also refines USS-SAF-001.
- Oxlint replaces eslint-plugin-jsx-a11y (USS-REACT-004).
- The Squawk baseline is the default rules. kube-linter leaves out checks
  that are specific to an organisation.
- USS-K8S-007 is stricter than USS-HRD-008.
- USS-MVN-004 accepts bounded Maven ranges.
- Rule sets of the Bandit class (Ruff S, gosec, Find Security Bugs) count as
  the SAST tool of USS-SEC-028. ESLint does not.
- The end-of-life rules of the modules judge what a project builds, tests
  and runs with, not its declared minimum versions.
- USS-REACT-005 exempts plain string literals.
- USS-QLT-012 judges inline suppressions only.
- USS-SAF-001 binds the MUST rules of the modules that refine it.

</details>

## 3. Questions about the standard

The self-audit raised these questions. Each one can change rules for every
project.

- [ ] **`stores-persistent-data`.** Its definition is literal enough to be
      true for a tool that writes report files. For USS, that adds 10 rules.
      Narrow the definition, or answer `true` for USS.
- [ ] **USS-PRF-002.** It passes when a project has no budget at all,
      because its Verify has nothing to judge. USS-PRF-001 fails in that
      case. Decide if that is enough.
- [ ] **USS-DEP-006.** It fails crates that are archived because they are
      finished. cfg-if is replaced by `cfg_select!`, but it still gets
      security fixes. Decide if a finished crate fails.
- [ ] **USS-TST-003.** It counts each commit labelled `fix` that changes
      only text (documents or rule text) as a bug fix with no test. Exempt
      changes that hold no code, or label such commits `docs`. Until you
      decide, the next audit counts each new text-only `fix` commit as a
      failure.

## 4. The release plan for 0.1.0

Spec §13 says that 0.1.0 waits for the trial and for the release plan. Check
each item against the audit that is current when you write the plan.

### 4.1 Defect to fix first (ruling R10)

AGENTS.md, "Before every workflow", step 5, installs `uss` with
`rustup toolchain install stable` and `cargo +stable install`. That can
update the developer's stable toolchain.

- [ ] Install a pinned toolchain in its place:
      `rustup toolchain install 1.100.0 --profile minimal --no-self-update`.
- [ ] Use `cargo +1.100.0 install …` for the install.
- [ ] Add a sentence for a `rust-version` newer than the pin, and one for a
      machine without rustup.
- [ ] Add a test that ties the pinned number to `rust-version`.

### 4.2 Code changes in `tools/uss` (15 results)

| Rule | Change |
|---|---|
| USS-UX-002 | **Defect:** `uss report --out` overwrites any file, also the results file itself. Refuse to overwrite without `--force`, and never write over the input. |
| USS-SEC-014 | **Defect:** `uss check` and `uss release-check` print control characters from records and file names as they are. Escape them. |
| USS-UX-011 | `uss report` and `uss release-check` print the problems that stop them on stdout. Print them on stderr. |
| USS-UX-001 | Give a next step in each error message. |
| USS-UX-006 | Add `-o`/`--output`, `-` for stdin and stdout, and a support link in `--help`. |
| USS-UX-009 | Add `--json` to `show`, `check`, `lint` and `report`. |
| USS-OBS-002 | Add an option that sets the log level. |
| USS-I18N-002 | Move the messages into a resource file. |
| USS-QLT-005 | Handle or explain the ten `let _ = writeln!(err, …)` lines, and the dropped `read_dir` errors in `cli/files.rs`. |
| USS-SEC-009 | Add `maxLength` to the schemas and a size limit for the files that `uss` reads. |
| USS-ARC-005 | Write the allowed dependencies between the library modules, and add a test that keeps I/O out of the library. |
| USS-TST-002 | Test the error paths of `build.rs`, of xtask and of the exit-code fallback in `main.rs`. |
| USS-TST-005 | Add end-to-end tests that run the built binary (`CARGO_BIN_EXE_uss`). |
| USS-TST-010 | Add property-based tests (proptest) for the parsers and the scores. |
| USS-TST-012 | Fuzz the parsers of records and results with cargo-fuzz, on a schedule in CI. |

### 4.3 Configuration and xtask (6 results)

| Rule | Change |
|---|---|
| USS-RUST-001, USS-QLT-006 | Deny the six Clippy lints of the baseline: `dbg_macro`, `let_underscore_must_use`, `todo`, `undocumented_unsafe_blocks`, `unimplemented`, `unused_result_ok`. Add the cargo-deny baseline: `db-urls`, `unmaintained = "all"`, `unused-ignored-advisory = "deny"`, `unused-license-exception = "deny"`, `required-git-spec = "rev"`. |
| USS-RUST-010 | Update cargo-deny to 0.22.1 or later (xtask pins 0.22.0). |
| USS-QLT-003 | Remove the warnings from the CI log: the cargo PATH warning, the rustup override notice and the duplicate `syn`. |
| USS-QLT-001 | Run ShellCheck on `.githooks/` in xtask. |
| USS-CTB-001 | Give one setup command that also sets `core.hooksPath`. |

### 4.4 Scanners (3 results)

These also run in `cargo xtask check` (USS-CTB-003), so xtask must download
and verify them on three operating systems.

- [ ] USS-SEC-004 (unverified): scan the tree and the history for secrets.
- [ ] USS-SEC-005: run a secret scanner in CI.
- [ ] USS-SEC-028: run a SAST tool in CI.

### 4.5 GitHub settings (4 results)

They depend on section 2.2.

- [ ] USS-SUP-007: protect the default branch from force pushes and deletion.
- [ ] USS-SUP-008: require the CI checks before a merge.
- [ ] USS-CON-003: make `cargo xtask check` a required check.
- [ ] USS-CTB-012: allow forks, and runs from fork pull requests.

### 4.6 What the release itself creates

Five results fail until a release exists:

- [ ] USS-DST-002: publish standalone binaries.
- [ ] USS-SUP-013: publish an SBOM.
- [ ] USS-SUP-014: sign the release artifacts.
- [ ] USS-SUP-015: publish SLSA level 2 provenance.
- [ ] USS-CTB-007: make the documented install from a tag work. It needs the
      tag.

The release also needs these items:

- [ ] USS-LIC-004: put a notices file for the dependencies in the binary.
- [ ] USS-REL-003: write the steps to rebuild the release artifacts from a
      tag.
- [ ] Add a 0.1.0 entry to `CHANGELOG.md`.
- [ ] Do a release audit of the release commit, and run `uss release-check`.
- [ ] Put the tag `v0.1.0` on the commit that holds the results.
- [ ] Make the marketplace public, if section 2.2 allows it.

### 4.7 Results that clear at the next audit (3)

These rules judge the changes since the last audit. They clear at the next
audit if the new changes follow them.

- USS-DEP-011: the change that adds a dependency records its purpose.
- USS-DEP-012: the change that adds a dependency records its health check.
- USS-TST-003: each new `fix` commit has a test that fails without the fix
  (see the question in section 3).

### 4.8 Tool improvements that are not audit results

- [ ] Add a filter to `uss applicable --full`, such as by area. Its output
      is 148 KB for this repository.
- [ ] `uss check` does not examine the content of results files. Only
      `uss report` does.
- [ ] CI template (`templates/ci/uss-release.yml`): split it into a
      read-only job and an upload job. Consider `--rev <sha>` in place of
      `--tag`. Its checkout pin is outside the reach of Dependabot, because
      projects copy the template.
- [ ] Add a CI template that runs the agent audit with no person present.
      It needs a public marketplace.

### 4.9 References to check before the release

These are left from the Plan 4 handover:

- [ ] Most baselines of the vendor profiles were never run, because the
      tools were not installed. golangci-lint ran only at 2.12.2.
- [ ] EN 301 549 V4.1.1 is cited, but was not read. The statement that the
      EAA has no harmonised reference is not conclusive.
- [ ] USS-CRA-004: the start of the five-year support period is USS's
      reading of the CRA.
- [ ] CIS Controls 11.3 was read through a third-party copy.
- [ ] USS-AUD-009 cites no source for CEF.

Read EU law through the Cellar API of the Publications Office (ELI with
`Accept: application/xhtml+xml`), because EUR-Lex blocks scripts.

## 5. Housekeeping

- [ ] Four worktrees remain under `.claude/worktrees/`, each locked and with
      its branch: `uss-plan2-security`, `uss-plan3-core-areas`,
      `uss-plan4-modules` and `uss-plan5-workflows`. All are merged into
      `main`. Remove them when you no longer need them.
- [ ] `todo.txt` has uncommitted lines (see section 6).

## 6. Later work

- [ ] From `todo.txt`: "translations/locale" and "metadata". Say what each
      one means. USS already has an internationalisation area (I18N) for
      software. Do you mean translations of the standard itself? Metadata
      of what?
- [ ] Spec §11, not in 0.1.0:
  - an MCP server that serves rules and records;
  - more modules (C/C++, C#/.NET, Kotlin, Swift, Gradle, Vue/Angular, CLI
    conventions, mobile, embedded, games);
  - dashboards over audit results across many projects;
  - regulation modules for PCI DSS, HIPAA and US federal requirements;
  - a VS Code extension, then other IDE plugins, through WebAssembly or a
    bundled binary;
  - cloud cost management, and export controls on cryptography;
  - a compliance badge.
- [ ] 1.0.0 comes when you declare it.
