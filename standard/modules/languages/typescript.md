# TypeScript and JavaScript

- **Prefix:** TS
- **Kind:** module
- **Applies when:** lang.typescript or lang.javascript

This module covers TypeScript and JavaScript projects: the linter, the
formatter and the compiler's strict checks, JavaScript's strict mode, the
pinned Node.js release and package manager, and how the project declares,
locks and checks its dependencies. It uses three vendor profiles:
`standard/vendors/eslint/eslint.md`, `standard/vendors/webpro-nl/knip.md` and
`standard/vendors/google/osv-scanner.md`. It names two tools without a
profile: Prettier, which sets the format, and pnpm, which sets the package
manager's settings. The rules that name a package manager accept npm, Yarn and
Bun as well, except the rule that recommends pnpm. The core rules that this
module does not refine apply as written.

## Checks

### USS-TS-001: Lint for correctness, style and imports

- **Level:** MUST
- **Applies when:** always
- **Category:** Maintainability
- **Refines:** USS-QLT-001
- **Verify:** The CI configuration runs a linter on all of the project's
  TypeScript and JavaScript code, and the job fails on any violation (ESLint
  exits with status 1 when a rule at error level fires or when the warnings
  exceed `--max-warnings`, so a rule at warn level fails the job only with
  `--max-warnings 0`). The linter reads type information from the TypeScript
  compiler (for typescript-eslint, `parserOptions.projectService`). Its
  configuration enables rules in each of three groups: correctness (such as
  unused variables, floating promises and unsafe uses of `any`), style, and
  imports (such as duplicate imports, import order or consistent type
  imports). A run on the default branch reports no violation.
- **References:** ESLint 10.15.0 documentation, Command Line Interface
  (exit codes); typescript-eslint 8.74.0 documentation, Shared configs and
  Linting with type information

CI MUST lint all TypeScript and JavaScript code with zero violations against a
type-aware rule set that covers correctness, style and imports.

**Rationale:** The three groups find different defects, and the rules that
read types find the ones that syntax alone cannot show, such as a promise that
nobody awaits. A rule set that leaves one group out lets that kind of defect
through, however clean the report is.

### USS-TS-002: Lint with ESLint and typescript-eslint

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Maintainability
- **Refines:** USS-QLT-001
- **Verify:** CI runs `eslint` on all TypeScript and JavaScript code, and the
  job fails on a finding (ESLint exits with status 1 when a rule at error
  level fires or when the warnings exceed `--max-warnings`). The ESLint
  configuration includes the baseline of the ESLint profile
  (`standard/vendors/eslint/eslint.md`). A project on another linter fails
  this rule (it records a Standard Deviation) and meets USS-TS-001 through
  that tool's own rule set.
- **References:** ESLint 10.15.0 documentation, Configuration Files and
  Command Line Interface; typescript-eslint 8.74.0 documentation, Shared
  configs

The project SHOULD lint with ESLint and typescript-eslint, configured per
their vendor profile (`standard/vendors/eslint/eslint.md`).

**Rationale:** One configuration file sets the type-aware rules, the import
checks and the checks on suppressions, and the profile gives it a tested
floor.

### USS-TS-003: Format with Prettier

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Maintainability
- **Refines:** USS-QLT-002
- **Verify:** CI runs `prettier --check` (or `-c`) over all of the project's
  TypeScript and JavaScript files, and the job fails when Prettier exits with
  a non-zero status (status 1 when a file is not formatted, status 2 when
  Prettier itself fails). A project on another formatter fails this rule (it
  records a Standard Deviation) and meets USS-QLT-002 through that
  formatter's own check mode.
- **References:** Prettier documentation, CLI (read 2026-11-24)

The project SHOULD format with Prettier, and CI SHOULD run `prettier --check`.

**Rationale:** Prettier fixes one format for the code, so a format check ends
style arguments in review and keeps each diff to the change itself.

### USS-TS-004: Compile with strict type checks

- **Level:** MUST
- **Applies when:** lang.typescript
- **Category:** Safety
- **Refines:** USS-SAF-001
- **Verify:** The effective TypeScript configuration (the `tsconfig.json` and
  each file that it `extends`, as `tsc --showConfig` prints it) turns on
  `strict`, `noUncheckedIndexedAccess`, `exactOptionalPropertyTypes`,
  `noImplicitOverride`, `noImplicitReturns` and `noFallthroughCasesInSwitch`.
  `strict` is on when it is `true`, and also when it is left unset under
  TypeScript 6.0 or later, where its default is `true`; the other five are not
  part of `strict`, so each is set to `true`. An option set to `false` fails,
  and so does one of the five that is left unset. CI runs the type check
  (such as `tsc --noEmit`) over the project's TypeScript code, and the job
  fails on any error that `tsc` reports.
- **References:** TypeScript TSConfig Reference (read 2026-11-24); TypeScript
  6.0 release notes (strict by default); TypeScript documentation, tsc CLI
  Options (read 2026-11-24)

The effective TypeScript configuration MUST turn on `strict`,
`noUncheckedIndexedAccess`, `exactOptionalPropertyTypes`,
`noImplicitOverride`, `noImplicitReturns` and `noFallthroughCasesInSwitch`,
and CI MUST run the type check.

**Rationale:** `strict` turns on the checks for null values, implicit `any`
and unsafe function types. The five other options close gaps that it leaves:
an index that may be empty, an optional property set to `undefined`, an
override that nothing declares, a path that returns nothing, and a switch case
that runs on into the next.

### USS-TS-005: Run JavaScript in strict mode

- **Level:** MUST
- **Applies when:** lang.javascript
- **Category:** Safety
- **Refines:** USS-SAF-001
- **Verify:** Each JavaScript file is an ES module, or starts with a
  `"use strict"` directive before any other statement. For Node.js, a file is
  an ES module when it has the `.mjs` extension, or has the `.js` extension
  and the nearest `package.json` has `"type": "module"`, or is an ambiguous
  file whose source has ES module syntax (`import` or `export`). A `.cjs`
  file, and a `.js` file that the nearest `package.json` leaves without
  `"type"` or sets to `"commonjs"` and that has no module syntax, needs the
  directive.
- **References:** MDN Web Docs, Strict mode (read 2026-11-24); Node.js
  documentation, Modules: Packages, determining module system (read
  2026-11-24)

JavaScript MUST be written as ES modules, or each script MUST start with
`"use strict"`.

**Rationale:** Strict mode turns silent mistakes, such as assigning to an
undeclared variable, into errors. ES modules are strict without a directive.

### USS-TS-006: Type-check JavaScript

- **Level:** SHOULD
- **Applies when:** lang.javascript
- **Category:** Safety
- **Verify:** CI runs the TypeScript compiler over the project's JavaScript
  files (such as `tsc --noEmit`), and the job fails on any error that `tsc`
  reports. The effective TypeScript configuration (the `tsconfig.json` or
  `jsconfig.json` and each file that it `extends`) sets `checkJs` to `true`
  and has `strict` on (`true`, or unset under TypeScript 6.0 or later). The
  types come from JSDoc comments or from declaration files.
- **References:** TypeScript TSConfig Reference, `checkJs` (read 2026-11-24);
  TypeScript 6.0 release notes (strict by default)

CI SHOULD type-check JavaScript with the TypeScript compiler, with `checkJs`
and `strict` on, from JSDoc types or declaration files.

**Rationale:** The compiler reads JSDoc types, so a JavaScript project gets
the same checks for null values and wrong argument types that a TypeScript
project gets, with no change of language.

## Toolchain

### USS-TS-007: Pin Node.js and the package manager

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Refines:** USS-SUP-001
- **Verify:** A file in the repository names one exact Node.js release with
  three numbers (such as `26.13.1`): `devEngines.runtime` in `package.json`,
  or the version file of a version manager. The package manager's version is
  pinned to one exact version with three numbers: the `packageManager` field
  of `package.json` (`name@version`, such as `pnpm@12.23.0`, with an optional
  hash after the version), or `devEngines.packageManager` with an exact
  `version`. A range, or a version with no patch number, fails. Local
  development and CI use these versions: CI reads the files or names the same
  versions.
- **References:** npm Docs, package.json: `devEngines` (npm 11.20.0, read
  2026-11-24); pnpm Docs, package.json: `packageManager` and
  `devEngines` (read 2026-11-24); Corepack README (read 2026-11-24)

The repository MUST pin the exact Node.js release and the package manager's
version, such as in the `packageManager` field.

**Rationale:** Node.js and the package manager differ in behaviour and in
security fixes from release to release, so a build that takes whichever
version is installed is not repeatable. Corepack, which reads the
`packageManager` field, is not bundled with Node.js 25 and later, so the
project's CI installs it or uses a package manager that reads the field
itself.

### USS-TS-008: No end-of-life Node.js

- **Level:** MUST NOT
- **Applies when:** always
- **Category:** Safety
- **Refines:** USS-SUP-006
- **Verify:** Take the release schedule of Node.js at the audit date. A release
  line is end-of-life when the schedule marks it EOL or its end date has
  passed. No released line that `engines.node` admits, and no line in the CI
  test matrix, is end-of-life. An `engines.node` that admits an end-of-life
  line fails (such as `>=20` after 2026-04-30, or `>=22`, which admits the
  end-of-life lines 23 and 25); `^22.12.0 || ^24.0.0 || ^26.0.0` passes.
- **References:** Node.js Release Working Group, release schedule (read
  2026-11-24); Node.js, Previous Releases (read 2026-11-24); npm Docs,
  package.json: `engines` (npm 11.20.0, read 2026-11-24)

`engines.node` and the CI matrix MUST NOT include a Node.js release line that
is end-of-life.

**Rationale:** An end-of-life line gets no security fixes, and a project that
declares it supported invites its users to run on it.

## Dependencies

### USS-TS-009: Bound each version range

- **Level:** MUST
- **Applies when:** has-dependencies
- **Category:** Safety
- **Refines:** USS-DEP-001
- **Verify:** Each dependency in each `package.json` (the dependencies,
  devDependencies, peerDependencies and optionalDependencies tables) has a
  range that admits at most one major version, read by npm's range rules. A
  caret (`^1.2.0`, and `^0.3.0`, which stops before `0.4.0`), a tilde, an
  exact version, an x-range such as `1.x`, and a range with an upper bound
  below the next major version (`>=1.2.0 <2.0.0`) pass. `*`, an empty string,
  `x`, a dist-tag such as `latest`, a lower bound with no upper bound
  (`>=1.2.0`), a range that admits two major versions (`>=1.2.0 <3.0.0`, or
  `1.2.0 - 2.0.0`), and a tarball URL fail. A git dependency passes only when
  it names a commit hash (`#<commit>`); a branch, a tag, a `#semver:` range or
  no suffix fails.
- **References:** npm Docs, package.json: dependencies, URLs as dependencies,
  git URLs as dependencies (npm 11.20.0, read 2026-11-24)

Each dependency MUST have a version range within one major version, such as
`^1.2.0`; `*`, `latest`, `x`, a URL and a git reference without a commit do
not meet this rule.

**Rationale:** A range with no upper bound accepts a new major version, and a
URL, a branch or a tag can change what it serves, so the same manifest can
install different code with no change to the project.

### USS-TS-010: Install with a frozen lock file

- **Level:** MUST
- **Applies when:** has-dependencies
- **Category:** Safety
- **Refines:** USS-DEP-002, USS-DEP-003
- **Verify:** The repository holds the lock file of its package manager
  (`package-lock.json`, `pnpm-lock.yaml`, `yarn.lock` or `bun.lock`). Each CI
  step that installs dependencies uses the manager's frozen mode, which fails
  when the lock file is out of date with `package.json`: `npm ci`,
  `pnpm install --frozen-lockfile`, `yarn install --immutable` (or
  `--frozen-lockfile`, its alias), `bun ci` or `bun install --frozen-lockfile`.
  pnpm and Yarn turn their frozen mode on by default in a CI environment, so a
  plain `pnpm install` or `yarn install` there passes unless the job turns the
  mode off (such as `--no-frozen-lockfile`, `frozenLockfile: false` or
  `CI=false` for pnpm). A step that resolves versions afresh (such as
  `npm install`) fails.
- **References:** npm Docs, npm ci (npm 11.20.0, read 2026-11-24); pnpm Docs,
  pnpm install, `--frozen-lockfile` (read 2026-11-24); Yarn Docs, yarn
  install, `--immutable` (read 2026-11-24); Bun Docs, bun install,
  `--frozen-lockfile` (read 2026-11-24)

The package manager's lock file MUST be committed, and CI MUST install in the
manager's frozen mode, such as `pnpm install --frozen-lockfile` or `npm ci`.

**Rationale:** The lock file fixes the versions and their integrity hashes,
and the frozen mode stops the install when it no longer matches
`package.json`, so CI builds what the lock file records.

### USS-TS-011: Use pnpm

- **Level:** SHOULD
- **Applies when:** has-dependencies
- **Category:** Safety
- **Refines:** USS-DEP-003, USS-DEP-004
- **Verify:** The repository holds a `pnpm-lock.yaml`, and CI installs the
  dependencies with pnpm (such as `pnpm install --frozen-lockfile`). A
  project on another package manager (such as npm,
  Yarn or Bun) fails this rule (it records a Standard Deviation) and meets
  USS-TS-010 and USS-TS-013 through that manager's own lock file and checks.
- **References:** pnpm Docs, Motivation (read 2026-11-24); pnpm Docs, pnpm
  install (read 2026-11-24)

The project SHOULD install with pnpm, whose isolated `node_modules` stops code
from importing undeclared packages.

**Rationale:** By default pnpm links only the direct dependencies of the
project into the root of `node_modules`, so an import of a package that
`package.json` does not declare fails at once, and not only when a dependency
happens to bring the package in.

### USS-TS-012: Keep build and test tools in devDependencies

- **Level:** MUST
- **Applies when:** has-dependencies
- **Category:** Safety
- **Refines:** USS-DEP-005
- **Verify:** Each package that only building, testing, linting or other
  tooling uses is in `devDependencies`, and `dependencies` holds only packages
  that the program needs at run time. A package of the first kind in
  `dependencies` fails.
- **References:** npm Docs, package.json: devDependencies (npm 11.20.0, read
  2026-11-24)

Packages used only to build, test or lint MUST be in `devDependencies`, and
`dependencies` MUST hold only runtime packages.

**Rationale:** A package manager that installs for production leaves out
`devDependencies`, so the tools stay out of what runs in production.

### USS-TS-013: Check that every import is declared

- **Level:** MUST
- **Applies when:** has-dependencies
- **Category:** Safety
- **Refines:** USS-DEP-004
- **Verify:** The CI configuration runs a check that compares the imports of
  all of the project's TypeScript and JavaScript code (source, tests and
  scripts) with the dependencies that `package.json` declares, and the job
  fails on an import of a package that is not declared (for knip, the job runs
  `knip` and fails on its exit status 1, which knip returns when it reports at
  least one issue, the `unlisted` issue type among them).
- **References:** knip 6.48.0 documentation, Issue Types and CLI (read
  2026-11-24)

CI MUST fail when code imports a package that its `package.json` does not
declare.

**Rationale:** A package that the code imports and that nothing declares is
there only because another package pulled it in, so it can vanish with no
change to the project.

### USS-TS-014: Check dependencies and dead code with knip

- **Level:** SHOULD
- **Applies when:** has-dependencies
- **Category:** Safety
- **Refines:** USS-DEP-004, USS-DEP-013, USS-QLT-004
- **Verify:** CI runs `knip` on the project, and the job fails when knip
  reports an issue (it exits with status 1 for at least one issue; the options
  `--no-exit-code` and a `--max-issues` above 0 hide issues, so the job does
  not use them). The knip configuration that knip reads (such as `knip.jsonc`)
  includes the baseline of the knip profile
  (`standard/vendors/webpro-nl/knip.md`). A project on another tool fails this
  rule (it records a Standard Deviation) and meets USS-TS-013 and the outcome
  rules through that tool's own profile or check.
- **References:** knip 6.48.0 documentation, Configuration, Rules & Filters
  and CLI (read 2026-11-24)

The project SHOULD check unused and undeclared dependencies, files and exports
with knip, configured per its vendor profile
(`standard/vendors/webpro-nl/knip.md`).

**Rationale:** One run reports the dependencies that nothing uses, the ones
that the code uses and `package.json` lacks, and the files and exports that
nothing reaches.

### USS-TS-015: Do not run dependencies' install scripts

- **Level:** SHOULD NOT
- **Applies when:** has-dependencies
- **Category:** Safety
- **Verify:** An install does not run the lifecycle scripts of dependencies,
  except those of packages on an allow list. This holds when the project uses
  pnpm 11 or later (which blocks them by default) and does not set
  `dangerouslyAllowAllBuilds` to `true`, and any `allowBuilds` entries in
  `pnpm-workspace.yaml` are the allow list; or the npm setting
  `ignore-scripts=true` is set, with an explicit rebuild step for each
  allowed package; or the package manager's own setting has the same effect
  (such as Bun's `trustedDependencies` list, since Bun does not run
  dependencies' scripts otherwise). An install on a manager and a version that
  have no such setting in effect fails.
- **References:** pnpm Docs, Settings: Build settings (read 2026-11-24); pnpm
  11.0 release notes (read 2026-11-24); npm Docs, config: `ignore-scripts`
  (npm 11.20.0, read 2026-11-24); Bun Docs, bun install (read 2026-11-24)

An install SHOULD NOT run the lifecycle scripts of dependencies, except those
of packages on an allow list (such as pnpm's `allowBuilds`).

**Rationale:** An install script runs with the rights of the person or the CI
job that installs, so one malicious or hijacked release can run code before
anyone has looked at it. An allow list keeps the few packages that need a
build step, and review stays on those.

### USS-TS-016: Wait before adopting new releases

- **Level:** SHOULD
- **Applies when:** has-dependencies
- **Category:** Safety
- **Refines:** USS-DEP-015
- **Verify:** The package manager's minimum release age, wherever the manager
  reads it from, is at least three days. For pnpm it is `minimumReleaseAge`
  (in minutes, so at least `4320`) in `pnpm-workspace.yaml`, or set by a
  `PNPM_CONFIG_*` variable or a command-line option in CI; pnpm's default is
  `1440`, so an unset value fails. For npm it is `min-release-age` (in days,
  so at least `3`). Bun's `minimumReleaseAge` counts seconds (at least
  `259200`).
- **References:** pnpm Docs, Settings: Dependency resolution,
  `minimumReleaseAge` (read 2026-11-24); npm Docs, config: `min-release-age`
  (npm 11.20.0, read 2026-11-24); Bun Docs, bun install (read 2026-11-24)

The package manager's minimum release age (such as pnpm `minimumReleaseAge`)
SHOULD be at least three days.

**Rationale:** A malicious or broken release is usually found and withdrawn
within days of publication, so a delay keeps the project clear of most of
them.

### USS-TS-017: Scan with OSV-Scanner

- **Level:** SHOULD
- **Applies when:** has-dependencies
- **Category:** Safety
- **Refines:** USS-DEP-007
- **Verify:** CI runs OSV-Scanner over the project's lock file
  (`package-lock.json`, `pnpm-lock.yaml`, `yarn.lock` or `bun.lock`) with a
  configuration that includes the baseline of the OSV-Scanner profile
  (`standard/vendors/google/osv-scanner.md`), and the job fails on a finding.
  A project on another scanner fails this rule (it records a Standard
  Deviation) and meets USS-DEP-007 through that tool's own profile or
  settings.
- **References:** OSV-Scanner Docs, Supported Languages and Lockfiles (read
  2026-11-24)

The project SHOULD scan its lock file with OSV-Scanner, configured per its
vendor profile (`standard/vendors/google/osv-scanner.md`).

**Rationale:** OSV-Scanner reads the lock file, so it sees the transitive
dependencies, and it reads the same vulnerability data for Python,
TypeScript and Maven projects.
