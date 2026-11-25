# Python

- **Prefix:** PY
- **Kind:** module
- **Applies when:** lang.python

This module covers Python projects: the linter and type checker, the warnings
of the test run, the Python version, and how the project declares, locks and
checks its dependencies. It uses four vendor profiles:
`standard/vendors/astral-sh/ruff.md`, `standard/vendors/python/mypy.md`,
`standard/vendors/osprey-oss/deptry.md` and
`standard/vendors/google/osv-scanner.md`. It names two tools without a
profile: uv, which sets the package manager's settings, and pytest. The core
rules that this module does not refine apply as written.

## Checks

### USS-PY-001: Lint for correctness, style, imports and security

- **Level:** MUST
- **Applies when:** always
- **Category:** Maintainability
- **Refines:** USS-QLT-001, USS-SEC-028
- **Verify:** The CI configuration runs a linter on all of the project's
  Python code (source, tests and scripts), and the job fails on any
  violation. The linter's configuration enables rules in each of four
  groups: correctness (such as undefined names and unused imports), style,
  import order, and security (such as the Bandit checks). A run on the
  default branch reports no violation.
- **References:** Ruff Docs, Rules (read 2026-11-24)

CI MUST lint all Python code with zero violations against a rule set that
covers correctness, style, import order and security.

**Rationale:** The four groups find different defects. A rule set that leaves
one out lets that kind of defect through, however clean the report is.

### USS-PY-002: Lint and format with Ruff

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Maintainability
- **Refines:** USS-QLT-001, USS-QLT-002, USS-SEC-028
- **Verify:** CI runs `ruff check` and `ruff format --check` on all Python
  code, and each fails the job on a finding. The Ruff configuration includes
  the baseline of the Ruff profile (`standard/vendors/astral-sh/ruff.md`).
  A project on another linter or formatter fails this rule (it records a
  Standard Deviation) and meets USS-PY-001 and USS-QLT-002 through that
  tool's own rule set.
- **References:** Ruff Docs, The Ruff Linter (read 2026-11-24); Ruff Docs,
  The Ruff Formatter (read 2026-11-24)

The project SHOULD lint and format with Ruff, configured per its vendor
profile (`standard/vendors/astral-sh/ruff.md`).

**Rationale:** One tool, with one configuration file, covers the lint groups
and the format check that the outcome rules need, and the profile gives it a
tested floor.

### USS-PY-003: Type-check in strict mode

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Refines:** USS-SAF-001
- **Verify:** CI runs a type checker on all of the project's Python code
  (source, tests and scripts) with its strict mode on (for mypy,
  `strict = true`), and the job fails on any error. A file left out of the
  run, or checked in a lower mode, is a failure.
- **References:** mypy 2.4.1 documentation, The mypy command line, `--strict`

CI MUST type-check all Python code in the type checker's strict mode, so
every function is annotated.

**Rationale:** Strict mode refuses unannotated functions, so the checker sees
the type of every value that crosses a function boundary, and a mismatch is a
build error and not a runtime failure.

### USS-PY-004: Type-check with mypy

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Safety
- **Refines:** USS-SAF-001
- **Verify:** CI runs `mypy` on all Python code, and the job fails on any
  error. The mypy configuration includes the baseline of the mypy profile
  (`standard/vendors/python/mypy.md`). A project on another type checker
  fails this rule (it records a Standard Deviation) and meets USS-PY-003
  through that tool's own strict mode.
- **References:** mypy 2.4.1 documentation, The mypy configuration file

The project SHOULD type-check with mypy, configured per its vendor profile
(`standard/vendors/python/mypy.md`).

**Rationale:** One configuration sets the strict mode and the extra checks
that the profile names, so the type check has a fixed floor.

### USS-PY-005: Turn warnings into errors in tests

- **Level:** MUST
- **Applies when:** always
- **Category:** Maintainability
- **Refines:** USS-QLT-003
- **Verify:** The test runner's configuration turns Python warnings into
  errors (for pytest, a `filterwarnings` list with an `error` entry that no
  later entry overrides, as pytest applies the last entry that matches). Each
  entry that ignores a warning names a warning category or a message, and has
  a comment that gives the reason. A bare `ignore` entry fails.
- **References:** pytest documentation, How to capture warnings (read
  2026-11-24)

The test run MUST turn Python warnings into errors (such as pytest
`filterwarnings = ["error"]`), and each warning that it ignores is named and
has a reason.

**Rationale:** A deprecation warning in a test is the early notice of a
break in a later release of Python or of a dependency. Naming each ignored
warning keeps the exceptions few and reviewable.

## Toolchain

### USS-PY-006: Pin the Python version

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Refines:** USS-SUP-001
- **Verify:** A file in the repository (such as `.python-version`) names one
  exact Python version, with major, minor and patch numbers (such as
  `3.13.2`). A range, or a version with no patch number, fails. Local
  development and CI use that version: CI reads the file or names the same
  version.
- **References:** uv Docs, Python versions (read 2026-11-24)

The repository MUST pin the exact Python version for development and CI, such
as in `.python-version`.

**Rationale:** Python versions differ in behaviour and in security fixes, so
a build that takes whichever version is installed is not repeatable.

### USS-PY-007: No end-of-life Python versions

- **Level:** MUST NOT
- **Applies when:** always
- **Category:** Safety
- **Refines:** USS-SUP-006
- **Verify:** Take the status table of the Python Developer's Guide at the
  audit date. No released minor version that `requires-python` admits, and
  no version in the CI test matrix, has the status end-of-life. A
  `requires-python` that admits an end-of-life version (such as `>=3.10`
  after the end of life of 3.10) fails.
- **References:** Python Developer's Guide, Status of Python versions (read
  2026-11-24)

`requires-python` and the CI matrix MUST NOT include a Python version that is
end-of-life.

**Rationale:** An end-of-life version gets no security fixes, and a project
that declares it supported invites its users to run on it.

## Dependencies

### USS-PY-008: Declare dependencies in pyproject.toml

- **Level:** MUST
- **Applies when:** has-dependencies
- **Category:** Safety
- **Refines:** USS-DEP-004
- **Verify:** Each direct dependency is declared in `pyproject.toml`: the
  runtime ones in the `[project]` table (`dependencies`, or
  `optional-dependencies`) and the others in `[dependency-groups]`. A direct
  dependency that only a requirements file declares fails.
- **References:** PEP 621; PEP 735

Direct dependencies MUST be declared in `pyproject.toml` (PEP 621 project
table, PEP 735 dependency groups), not only in a requirements file.

**Rationale:** `pyproject.toml` is the one place that every Python tool reads
for the project's metadata. A requirements file is read only by the tool that
the project points at it.

### USS-PY-009: Bound each version specifier

- **Level:** MUST
- **Applies when:** has-dependencies
- **Category:** Safety
- **Refines:** USS-DEP-001
- **Verify:** Each specifier in `[project] dependencies`, in each
  `[project.optional-dependencies]` list and in each `[dependency-groups]`
  list admits no version of the next major release or later: it is `==`, a
  compatible release (`~=`), or has an upper bound that excludes the next
  major version (`<` with the next major version, or `<=` with a version
  below it). `>=1.2,<2` and `~=1.2` pass. `>=1.2`, `>=1.2,<3`, `>=1.2,<=2`
  and an unversioned name fail.
- **References:** PEP 440

Each dependency specifier MUST have an upper bound below the next major
version, such as `~=1.2` or `>=1.2,<2`.

**Rationale:** A lower bound alone accepts every later release, so a new
major version can change what the project builds with no change to the
project.

### USS-PY-010: Install from a hashed lock file

- **Level:** MUST
- **Applies when:** has-dependencies
- **Category:** Safety
- **Refines:** USS-DEP-002, USS-DEP-003
- **Verify:** The repository holds a lock file (such as `uv.lock`, or a
  PEP 751 `pylock.toml`) that records a hash for the files of each package
  that it resolves. CI and the build scripts install from it with a command
  that fails when the lock file is out of date with `pyproject.toml` (such
  as `uv sync --locked`). A step that resolves versions afresh, or that uses
  the lock file without checking that it is current (such as `--frozen`),
  fails.
- **References:** PEP 751; uv Docs, Locking and syncing (read 2026-11-24);
  uv Docs, Resolution (2026-11-05)

CI and builds MUST install from a committed lock file with hashes (such as
`uv.lock`, or a PEP 751 `pylock.toml`), with a command that fails when the
lock file is out of date (such as `uv sync --locked`).

**Rationale:** The hashes show that the code that is installed is the code
that was resolved, and the failing command shows that the lock file still
matches `pyproject.toml`.

### USS-PY-011: Manage dependencies with uv

- **Level:** SHOULD
- **Applies when:** has-dependencies
- **Category:** Safety
- **Refines:** USS-DEP-002, USS-DEP-003, USS-SUP-001
- **Verify:** The repository holds a `uv.lock`. CI installs the dependencies
  with uv (such as `uv sync --locked`), and gets its Python version through
  uv (such as `uv python install`, or uv's use of `.python-version`). A
  project on another tool (such as Poetry, PDM or pip-tools) fails this rule
  (it records a Standard Deviation) and meets USS-PY-010 and USS-PY-006
  through that tool's own lock file and version pin.
- **References:** uv Docs, Locking and syncing (read 2026-11-24); uv Docs,
  Python versions (read 2026-11-24)

The project SHOULD resolve, lock and install its dependencies and its Python
version with uv.

**Rationale:** One tool covers the resolver, the lock file, the install
command that checks the lock file and the Python version, so the project
needs no glue between them.

### USS-PY-012: Keep development dependencies out of the runtime set

- **Level:** MUST
- **Applies when:** has-dependencies
- **Category:** Safety
- **Refines:** USS-DEP-005
- **Verify:** Each package that only tests, linting, type-checking,
  documentation or the build uses is in a `[dependency-groups]` list or in
  `[build-system] requires`. A package of that kind in `[project]
  dependencies` fails.
- **References:** PEP 735; uv Docs, Managing dependencies (read 2026-11-24)

Test, lint and build-only dependencies MUST be in dependency groups or the
build-system table, never in the project's runtime dependencies.

**Rationale:** A dependency group is not part of the built distribution, so
the tools stay out of what users install.

### USS-PY-013: Check that every import is declared

- **Level:** MUST
- **Applies when:** has-dependencies
- **Category:** Safety
- **Refines:** USS-DEP-004
- **Verify:** The CI configuration runs a check that compares the imports of
  all Python code (source, tests and scripts) with the declared direct
  dependencies, and the job fails on an import of a third-party package that
  is not declared (for deptry, the job runs `deptry .` and fails on its
  non-zero exit status, which deptry returns when it finds any violation).
- **References:** deptry Docs, Rules and Violations (read 2026-11-24);
  deptry 0.25.1 source, `python/deptry/core.py` (exit status)

CI MUST fail when the code imports a third-party package that is not a
declared direct dependency.

**Rationale:** A package that the code imports and that nothing declares is
there only because another package pulled it in, so it can vanish with no
change to the project.

### USS-PY-014: Check imports with deptry

- **Level:** SHOULD
- **Applies when:** has-dependencies
- **Category:** Safety
- **Refines:** USS-DEP-004, USS-DEP-013
- **Verify:** CI runs `deptry` on the project, and the job fails on a
  violation. The deptry configuration includes the baseline of the deptry
  profile (`standard/vendors/osprey-oss/deptry.md`). A project on another
  tool fails this rule (it records a Standard Deviation) and meets
  USS-PY-013 through that tool's own profile or check.
- **References:** deptry Docs, Usage and Configuration (read 2026-11-24)

The project SHOULD check its declared and imported dependencies with deptry,
configured per its vendor profile (`standard/vendors/osprey-oss/deptry.md`).

**Rationale:** One run reports undeclared, unused, transitive and misplaced
dependencies, from the metadata that `pyproject.toml` already holds.

### USS-PY-015: Scan with OSV-Scanner

- **Level:** SHOULD
- **Applies when:** has-dependencies
- **Category:** Safety
- **Refines:** USS-DEP-007
- **Verify:** CI runs OSV-Scanner over the project's lock file (`uv.lock` or
  `pylock.toml`) with a configuration that includes the baseline of the
  OSV-Scanner profile (`standard/vendors/google/osv-scanner.md`), and the
  job fails on a finding. A project on another scanner fails this rule (it
  records a Standard Deviation) and meets USS-DEP-007 through that tool's own
  profile or settings.
- **References:** OSV-Scanner Docs, Supported Artifacts and Manifests (read
  2026-11-24)

The project SHOULD scan its lock file with OSV-Scanner, configured per its
vendor profile (`standard/vendors/google/osv-scanner.md`).

**Rationale:** OSV-Scanner reads the lock file, so it sees the transitive
dependencies, and it reads the same vulnerability data for Python,
TypeScript and Maven projects.

### USS-PY-016: Hold back new releases with uv

- **Level:** SHOULD
- **Applies when:** has-dependencies
- **Category:** Safety
- **Refines:** USS-DEP-015
- **Verify:** The uv configuration that applies to the project (`[tool.uv]`
  in `pyproject.toml`, or `uv.toml`) sets `exclude-newer` to a duration of at
  least three days (such as `3 days`, `1 week` or `P7D`). An absolute
  timestamp, `false` and a missing setting fail.
- **References:** uv Docs, Settings (read 2026-11-24), exclude-newer; uv
  Docs, Configuration files (read 2026-11-24); uv Docs, Resolution
  (2026-11-05), dependency cooldowns

uv's `exclude-newer` SHOULD be set as a relative age of at least three days,
so resolution ignores releases younger than that.

**Rationale:** A relative age moves with the clock, so each new resolution
keeps the delay. An absolute timestamp stops delaying anything once it is
old.
