# Code quality

- **Prefix:** QLT
- **Kind:** area

This area covers the checks that keep code clean and the rules that keep
those checks honest. Its rules count toward Maintainability by default. The
check-integrity rules (USS-QLT-006 to USS-QLT-014) apply to every kind of
check: linters, compilers, type checkers, security scanners, tests and
coverage. A "zero violations" target is meaningless if a project can reach it
by turning the check off. The Conduct (CON) area holds the Definition of
Done, the Testing area holds the tests themselves, and the Language and
runtime safety (SAF) area holds the safety options of each toolchain.

## Checks

### USS-QLT-001: Lint with zero violations

- **Level:** MUST
- **Applies when:** always
- **Category:** Maintainability
- **Verify:** The CI configuration runs a linter or static analyser for each
  language that the project's own code uses, on each pull request and each
  push to the default branch. The job exits with a failure when it reports
  any violation. A run of the linter on the default branch reports none.
- **References:** NIST SP 800-218 (SSDF 1.1) PW.7.2

CI MUST run a linter or static analyser for each language in the project, and
any violation MUST fail the build.

**Rationale:** A violation that does not fail the build is easy to ignore, and
the count grows until nobody reads the report.

### USS-QLT-002: Check formatting in CI

- **Level:** MUST
- **Applies when:** always
- **Category:** Maintainability
- **Verify:** A formatter's configuration file (or its documented defaults,
  named in the contributor documentation) sets the code's format, and a CI
  job runs the formatter in check mode (such as a `--check` or `--diff`
  option) and fails when it would change any file.
- **References:** none

A formatter's configuration MUST set the code's format, and CI MUST fail on
code that the formatter would change.

**Rationale:** An automatic format ends style arguments in review and keeps
each diff to the change itself.

### USS-QLT-003: Fail the build on warnings

- **Level:** MUST
- **Applies when:** always
- **Category:** Maintainability
- **Verify:** For each compiler, type checker and build tool in the CI
  configuration, the build turns warnings into errors (such as `-Werror`,
  `-D warnings` or a strict mode) or fails on any warning in its output. A
  clean build on the default branch prints no warning.
- **References:** NIST SP 800-218 (SSDF 1.1) PW.6.2

CI MUST treat the warnings of every compiler, type checker and build tool as
errors.

**Rationale:** A warning names a defect that the tool can see and the author
may not. If warnings do not fail the build, new ones hide among the old.

### USS-QLT-004: No dead code

- **Level:** MUST NOT
- **Applies when:** always
- **Category:** Maintainability
- **Verify:** The project's linter or compiler is set to report unreachable
  code and unused functions, types, variables and imports, and its report is
  empty. A search of the files changed since the last audit (or in the last
  12 months when there is none) finds no commented-out code: no comment whose
  text parses as a statement or declaration of the language.
- **References:** CWE-561

The code MUST NOT contain unreachable code, unused functions, types or
variables, or commented-out code.

**Rationale:** Dead code is read, maintained and searched like live code, and
it misleads the reader about what the software does. The version history
keeps what was removed.

### USS-QLT-005: No swallowed errors

- **Level:** MUST NOT
- **Applies when:** always
- **Category:** Maintainability
- **Verify:** Look for each place where the code catches an error or receives
  one as a return value and drops it: an empty catch or except block, an
  error assigned to a discard name, an unchecked return code, a promise
  without a rejection handler. Each one is handled, passed on or reported
  (logged, returned or shown), or has a comment on or directly above that
  line that says why ignoring it is safe. A linter's unchecked-error rules
  (such as `errcheck`) can list the places.
- **References:** CWE-252; CWE-390

An error MUST NOT be caught or returned and then ignored: each one is
handled, passed on or reported, or a comment at that line says why ignoring
it is safe.

**Rationale:** An ignored error leaves the program running on a false belief
that the operation worked, and the failure shows up later, far from its
cause.

## Check integrity

### USS-QLT-006: Configure checks at least as strictly as the baseline

- **Level:** MUST
- **Applies when:** always
- **Category:** Maintainability
- **Verify:** For each check that the project runs, find its configuration.
  The baseline is, in this order: the one that the check's USS vendor profile
  defines; with no profile, the rule set that the USS module of the project's
  language names; with neither, the tool's own recommended set. The
  configuration enables every rule of the baseline, at the baseline's
  severity or higher. A rule of the baseline that the configuration turns
  off fails here and also needs a reason under USS-QLT-007.
- **References:** none

Each check's configuration MUST enable at least the baseline that its USS
vendor profile defines; with no profile, the rule set that the language's USS
module names; with neither, the tool's own recommended set.

**Rationale:** A project can reach zero violations by choosing a check with no
rules. A baseline fixes the minimum that "zero violations" means.

### USS-QLT-007: Give a reason for each rule turned off

- **Level:** MUST
- **Applies when:** always
- **Category:** Maintainability
- **Verify:** In each configuration file of a check, each rule that is turned
  off or lowered for the whole project (an ignore list, a disable list, a
  severity of `off`) has a comment or an adjacent field next to it that says
  why. A reason such as "noisy" with no cause fails; one that names the
  cause (for example, the conflict with the formatter) passes.
- **References:** none

Each rule of a check that is turned off for the whole project MUST have its
reason written next to it in the configuration.

**Rationale:** A rule turned off without a reason cannot be reviewed later, so
it stays off after its reason is gone.

### USS-QLT-008: Keep security rules on

- **Level:** SHOULD NOT
- **Applies when:** always
- **Category:** Safety
- **Verify:** In each configuration file of a check, no rule in a security
  group (such as the Bandit rules of Ruff, `gosec` rules of golangci-lint,
  or the security rules of an ESLint plugin) is turned off for the whole
  project. A rule that the configuration turns off only for named paths (such
  as test fixtures) does not fail.
- **References:** none

A rule in a security group of a check SHOULD NOT be turned off for the whole
project.

**Rationale:** Security rules find the flaws that cost the most, and a rule
turned off for the whole project stops finding them in code that is not yet
written. Where one finding is a false positive, a suppression with a reason
(USS-QLT-009) is narrower.

### USS-QLT-009: Name the rule and the reason in each suppression

- **Level:** MUST
- **Applies when:** always
- **Category:** Maintainability
- **Verify:** Search the code for each inline suppression (such as `# noqa`,
  `# type: ignore`, `// nolint`, `// eslint-disable-next-line`,
  `#[allow(...)]`, `@SuppressWarnings`). Each one names the specific rule it
  suppresses and has a reason on the same line, in the syntax that the tool
  offers (such as Rust's `reason = "..."` parameter, or a plain comment). A
  tool's own check can list the gaps, such as Ruff's `PGH004`, golangci-lint's
  `nolintlint` and Clippy's `allow_attributes_without_reason`.
- **References:** none

Each inline suppression MUST name the specific rule that it suppresses and
give a reason on the same line.

**Rationale:** A suppression that names its rule hides nothing else. The
reason lets a reviewer, now or later, judge whether the suppression is still
right.

### USS-QLT-010: No blanket suppressions

- **Level:** MUST NOT
- **Applies when:** always
- **Category:** Maintainability
- **Verify:** Search the code for suppressions of all rules (such as a bare
  `# noqa`, a bare `# type: ignore`, an `eslint-disable` with no rule) and for
  file-level directives (such as `eslint-disable` at the top of a file,
  `# ruff: noqa`, `// @ts-nocheck`). None is present. A suppression that
  names one rule and covers one line, statement or block passes.
- **References:** none

A suppression MUST NOT cover all rules, or a whole file.

**Rationale:** A blanket suppression also hides every problem that is added
later to the lines it covers.

### USS-QLT-011: Use suppressions that expire

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Maintainability
- **Verify:** For each language of the project, check whether it offers a
  suppression that fails when the problem is gone (such as Rust `#[expect]` or
  TypeScript `@ts-expect-error`). Where it does, the project's suppressions
  of that kind use it, and none uses the plain form (such as `#[allow]` or
  `@ts-ignore`). A language with no such suppression has nothing to fail.
- **References:** The Rust Reference, Diagnostic attributes (read 2026-11-24),
  The expect attribute; TypeScript 3.9 release notes (read 2026-11-24),
  `// @ts-expect-error` comments

Where the language offers a suppression that fails when the problem is gone
(such as Rust `#[expect]` or TypeScript `@ts-expect-error`), suppressions
SHOULD use it.

**Rationale:** A suppression of the plain form stays in the code after the
problem is fixed, and then hides the next problem on that line.

### USS-QLT-012: Fail on stale suppressions

- **Level:** MUST
- **Applies when:** always
- **Category:** Maintainability
- **Verify:** For each check that has suppressions, the configuration turns on
  the tool's check for suppressions that suppress nothing, at a level that
  fails the build. Examples are Ruff's `RUF100`, golangci-lint's `nolintlint`
  with unused directives not allowed, ESLint's
  `--report-unused-disable-directives` (or its severity option set to
  `error`), mypy's `warn_unused_ignores` and Rust's
  `unfulfilled_lint_expectations`. Where the toolchain offers no such check,
  a CI script that finds stale suppressions and fails passes; with neither,
  the rule fails.
- **References:** Ruff, unused-noqa (RUF100) (read 2026-11-24); golangci-lint,
  nolintlint settings (read 2026-11-24); ESLint, command line interface,
  `--report-unused-disable-directives` (read 2026-11-24); mypy configuration
  file, `warn_unused_ignores` (read 2026-11-24)

A suppression that no longer suppresses anything MUST fail the check.

**Rationale:** Code and tools change. A suppression that nothing needs any more
is noise at best, and at worst it hides a new problem.

### USS-QLT-013: Link each skipped test to an issue

- **Level:** MUST
- **Applies when:** always
- **Category:** Maintainability
- **Verify:** Search the tests for each skipped or disabled test (such as
  `skip`, `xfail`, `@Disabled`, `#[ignore]`, `t.Skip`, `xit`, a commented-out
  test). Each one names a tracked issue, by its ID or link, in its reason or
  on its line, and that issue exists in the project's tracker. A skip that
  depends on the platform (such as a Windows-only test that is skipped on
  Linux) is a condition, not a disabled test, and passes.
- **References:** none

Each skipped or disabled test MUST name a tracked issue.

**Rationale:** A skipped test is a check that is turned off. The issue is the
record that someone will turn it on again.

### USS-QLT-014: Give a reason for each coverage exclusion

- **Level:** MUST
- **Applies when:** always
- **Category:** Maintainability
- **Verify:** In the coverage tool's configuration and in the code (such as
  `omit` and `exclude_lines` settings, `# pragma: no cover`,
  `/* istanbul ignore */`, `// coverage:ignore`), each excluded line, block
  or file has a reason written next to it. A project that excludes nothing
  passes.
- **References:** none

Each line, block or file excluded from coverage measurement MUST have its
reason written next to it.

**Rationale:** A coverage figure only means something if its exclusions are
known. An exclusion with no reason lets untested code count as covered.
