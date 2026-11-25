# Go

- **Prefix:** GO
- **Kind:** module
- **Applies when:** lang.go

This module covers Go projects: the linter, the formatter and `go vet`, the
race detector, the Go toolchain, how the project keeps `go.mod` and `go.sum`,
and the timeouts of HTTP clients and servers. It uses one vendor profile:
`standard/vendors/golangci/golangci-lint.md`. It names `govulncheck` without a
profile. Go links only the packages that the code imports, so USS-DEP-005 is
met by the toolchain; tool dependencies use the `tool` directive of `go.mod`.
USS-GO-004 is the Go safety check that USS-SAF-001 makes binding. The core
rules that this module does not refine apply as written.

## Checks

### USS-GO-001: Lint for correctness, style, imports and security

- **Level:** MUST
- **Applies when:** always
- **Category:** Maintainability
- **Refines:** USS-QLT-001, USS-SEC-028
- **Verify:** The CI configuration runs a linter on all of the project's Go
  code (source and tests), and the job fails on any violation. The linter's
  configuration enables rules in each of four groups: correctness (such as
  unchecked errors and the `go vet` analyzers), style, imports (grouping and
  order), and security (such as the gosec checks). A run on the default branch
  reports no violation.
- **References:** golangci-lint Docs, Linters (read 2026-11-24)

CI MUST lint all Go code with zero violations against a rule set that covers
correctness, style, imports and security.

**Rationale:** The four groups find different defects. A rule set that leaves
one out lets that kind of defect through, however clean the report is.

### USS-GO-002: Lint with golangci-lint

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Maintainability
- **Refines:** USS-QLT-001, USS-QLT-002, USS-SEC-028
- **Verify:** CI runs `golangci-lint run` over all of the project's Go
  modules, and the job fails on a finding (the command exits with status 1
  when it reports an issue, unless the configuration sets `issues-exit-code`
  to another value, which fails this rule). The golangci-lint configuration
  includes the baseline of the golangci-lint profile
  (`standard/vendors/golangci/golangci-lint.md`). A project that uses another
  tool fails this rule (it records a Standard Deviation) and meets the outcome
  rule through that tool, configured to at least the baseline that
  USS-QLT-006 sets.
- **References:** golangci-lint Docs, Configuration file (read 2026-11-24);
  golangci-lint Docs, Linters (read 2026-11-24)

The project SHOULD lint with golangci-lint, configured per its vendor profile
(`standard/vendors/golangci/golangci-lint.md`).

**Rationale:** One tool, with one configuration file, runs the linters and
formatters that the outcome rules need, and the profile gives it a tested
floor.

### USS-GO-003: Check formatting with gofmt

- **Level:** MUST
- **Applies when:** always
- **Category:** Maintainability
- **Refines:** USS-QLT-002
- **Verify:** A CI job runs `gofmt -l` (or a stricter gofmt-compatible
  formatter, such as gofumpt) over all Go files, and fails when it lists a
  file. The `gofmt` documentation states no exit status for `-l`, so a step
  that only runs the command and ignores its output does not fail the job: the
  step has to fail on non-empty output, or use the formatter's own check
  mode (`golangci-lint run` with the `gofmt` formatter enabled reports a file
  that `gofmt` would change, and fails the job).
- **References:** Go Command Documentation, gofmt (Go 1.27.4)

CI MUST fail when `gofmt -l` (or a stricter gofmt-compatible formatter, such
as gofumpt) lists a file.

**Rationale:** The Go toolchain fixes one format, so a format check ends style
arguments in review and keeps each diff to the change itself.

### USS-GO-004: Run go vet

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Refines:** USS-SAF-001
- **Verify:** CI runs `go vet ./...` over all of the project's packages, with
  its default analyzers, and a finding fails the job (`go vet` exits with a
  non-zero status when it reports a problem).
- **References:** Go Command Documentation, vet (Go 1.27.4)

CI MUST run `go vet ./...` with its default analyzers, and a finding MUST fail
the build.

**Rationale:** `go vet` finds constructs that compile and are almost always
wrong, such as a wrong format verb or a copied lock, and it is part of the
toolchain, so it costs nothing to run.

### USS-GO-005: Run tests with the race detector

- **Level:** SHOULD
- **Applies when:** uses-concurrency
- **Category:** Safety
- **Refines:** USS-SAF-007
- **Verify:** The CI step that runs the tests runs `go test -race` (the flag
  can be set in `GOFLAGS`), and the job fails when the detector reports a race
  (the detector exits with a non-zero status, 66 by default, after a race).
- **References:** Go Documentation, Data Race Detector (read 2026-11-24)

CI SHOULD run the tests with `go test -race`.

**Rationale:** A data race shows up only under some timings, so a test can
pass a hundred times and fail in production. The detector finds the race that
did not happen to hurt.

## Toolchain

### USS-GO-006: Pin the Go toolchain

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Refines:** USS-SUP-001
- **Verify:** Two parts, and both hold. First, `go.mod` names an exact Go
  release: a `toolchain` directive (such as `toolchain go1.27.4`), or a `go`
  directive that names a patch release (such as `go 1.27.4`). A `go` directive
  with no patch number (such as `go 1.27`) and no `toolchain` directive
  fails. Second, CI builds with the release that `go.mod` names: a CI step
  installs the version that `go.mod` names (when both directives are present,
  the `toolchain` one), or CI sets `GOTOOLCHAIN` to that release (such as
  `go1.27.4`) without the `+auto` or `+path` suffix.
- **References:** Go Toolchains (go.dev/doc/toolchain, read 2026-11-24)

`go.mod` MUST name the exact Go release (a `toolchain` directive, or a `go`
directive that names a patch release), and CI MUST build with that release:
CI installs the version that `go.mod` names, or sets `GOTOOLCHAIN` to that
release without `+auto` or `+path`.

**Rationale:** The directives alone are a minimum: the `go` command runs its
own toolchain when that one is newer than the directives, so a build on a
machine with a newer Go uses the newer Go. Installing or forcing the named
release makes the build repeatable and a toolchain upgrade a reviewed change.

### USS-GO-007: No unsupported Go releases

- **Level:** MUST NOT
- **Applies when:** always
- **Category:** Safety
- **Refines:** USS-SUP-006
- **Verify:** Take the Go release policy at the audit date: the two newest
  major releases are supported. The major release of the toolchain that the
  project builds with, which the `toolchain` directive of `go.mod` names (the
  `1.27` in `toolchain go1.27.4`; a `go.mod` with no `toolchain` directive has
  an implicit one with the release of its `go` directive), and the major
  release of each Go release in the CI test matrix are one of those two. An
  older major release fails. The `go` directive, as the minimum language
  version of the module, is not judged by this rule.
- **References:** Go Release Policy (go.dev/doc/devel/release, read
  2026-11-24); Go Toolchains (go.dev/doc/toolchain, read 2026-11-24)

The toolchain that the project builds with (the `toolchain` directive of
`go.mod`, written or implicit) and the Go releases in the CI matrix MUST NOT be
releases that the Go project no longer supports.

**Rationale:** An unsupported Go release gets no security fixes, so a project
that builds or tests with it keeps every flaw that is found after its end. A
declared minimum such as the `go` directive is a compatibility promise
(USS-DEP-016), not a platform that the project runs on.

### USS-GO-008: Build release binaries with -trimpath

- **Level:** SHOULD
- **Applies when:** publishes-releases
- **Category:** Maintainability
- **Refines:** USS-REL-003
- **Verify:** The commands or scripts that build the release binaries pass
  `-trimpath` to `go build` (or set it in `GOFLAGS` for that build).
- **References:** Go Command Documentation, go (Go 1.27.4), build flags

Release builds SHOULD use `-trimpath`, so binaries hold no paths of the build
machine.

**Rationale:** Without `-trimpath`, a binary holds the file system paths of
the machine that built it, so two machines build different bytes from the same
source, and the paths leak details of the build environment.

## Dependencies

### USS-GO-009: Keep go.mod and go.sum tidy

- **Level:** MUST
- **Applies when:** has-dependencies
- **Category:** Safety
- **Refines:** USS-DEP-004, USS-DEP-013
- **Verify:** A CI job runs `go mod tidy -diff` (which prints the changes that
  `go mod tidy` would make and exits with a non-zero status when there are
  any), or runs `go mod tidy` and then fails when `go.mod` or `go.sum` differ
  from the committed files. The job fails on a difference.
- **References:** Go Modules Reference, go mod tidy (read 2026-11-24)

CI MUST fail when `go mod tidy` would change `go.mod` or `go.sum`.

**Rationale:** `go mod tidy` adds the modules that the code imports and
removes the ones that it does not, so a tidy `go.mod` lists what the code uses
and nothing else.

### USS-GO-010: Keep checksum verification on

- **Level:** MUST
- **Applies when:** has-dependencies
- **Category:** Safety
- **Refines:** USS-DEP-002, USS-DEP-003
- **Verify:** `go.sum` is in version control. No file of the repository (CI
  configuration, build scripts, Dockerfiles, a `go env -w` setting that the
  repository documents) sets `GOSUMDB=off` (which turns the checksum database
  off for every module), a `GONOSUMDB` or `GOPRIVATE` pattern that matches a
  public module (`GONOSUMDB` defaults to `GOPRIVATE`), or
  `GOFLAGS=-mod=mod`. Without access to the build environment's settings,
  record `unverified` for them.
- **References:** Go Modules Reference, environment variables and
  authenticating modules (read 2026-11-24)

`go.sum` MUST be committed, and the project's settings MUST leave out
`GOSUMDB=off` (which turns the checksum database off for every module), a
`GONOSUMDB` or `GOPRIVATE` pattern that matches a public module, and
`GOFLAGS=-mod=mod`.

**Rationale:** The `go` command checks each module it downloads against
`go.sum` and the checksum database. A setting that turns the database off for
a module lets a changed module be accepted without notice. `GOFLAGS=-mod=mod`
does a different harm: it lets the `go` command rewrite `go.sum` during a
build, so the file no longer fixes what the build uses.

### USS-GO-011: No local replace directives in a released module

- **Level:** MUST NOT
- **Applies when:** has-dependencies and publishes-releases
- **Category:** Safety
- **Refines:** USS-DEP-003
- **Verify:** In `go.mod` at each released tag, no `replace` directive has a
  local path on the right of the arrow (a path that begins with `./` or
  `../`, or an absolute path). A `replace` to another module path passes.
- **References:** Go Modules Reference, replace directive and workspaces (read
  2026-11-24)

A released `go.mod` MUST NOT have a `replace` directive that points to a local
path. Local development uses `go.work`.

**Rationale:** A local path exists only on the machine that wrote it, so the
module cannot be built from its tag by anyone else. The `go` command ignores
`replace` directives in modules that are dependencies, so the directive also
misleads readers about what the release uses.

### USS-GO-012: Scan with govulncheck

- **Level:** SHOULD
- **Applies when:** has-dependencies
- **Category:** Safety
- **Refines:** USS-DEP-007
- **Verify:** CI runs `govulncheck` over the module (such as
  `govulncheck ./...`), and the job fails on a finding. With its text output
  and its default scan level (`symbol`), `govulncheck` exits with a non-zero
  status when the code calls a vulnerable function. With `-format json`,
  `-format sarif` or `-format openvex` it exits with status 0 whatever it
  finds, so a job that uses one of those formats fails on a finding only
  through a further step that does so; a job with no such step fails this
  rule. A project that uses another tool fails this rule (it records a
  Standard Deviation) and meets the outcome rule through that tool, configured
  to at least the baseline that USS-QLT-006 sets.
- **References:** govulncheck Documentation (pkg.go.dev, read 2026-11-24);
  golang/vuln source, `internal/scan/flags.go` and `internal/scan/text.go`
  (read 2026-11-24)

CI SHOULD scan the module with `govulncheck` in a job that fails on a finding.

**Rationale:** `govulncheck` runs from the module directory with the package
syntax of the `go` command, so it needs no separate manifest, and a scan costs
one CI step.

## Network

### USS-GO-013: No HTTP clients without a timeout

- **Level:** MUST NOT
- **Applies when:** uses-network
- **Category:** Reliability
- **Refines:** USS-RES-001
- **Verify:** Search the code for outbound HTTP calls. A call fails when it
  uses `http.DefaultClient`, a package-level helper (`http.Get`, `http.Head`,
  `http.Post` or `http.PostForm`), or an `http.Client` whose `Timeout` is zero
  or unset, unless the request carries a context with a deadline (such as one
  from `context.WithTimeout`, set with `http.NewRequestWithContext`).
- **References:** Go `net/http` package documentation (Go 1.27.4); CWE-400

Outbound HTTP calls MUST NOT use `http.DefaultClient`, the package-level
helpers such as `http.Get`, or an `http.Client` with no `Timeout`, unless each
request carries a context with a deadline.

**Rationale:** `DefaultClient` and the helpers that use it have no timeout,
and so has a `Client` whose `Timeout` is zero, so a server that stops
answering holds the call, its goroutine and its connection for good.

### USS-GO-014: Set server timeouts

- **Level:** MUST
- **Applies when:** is-network-service
- **Category:** Reliability
- **Refines:** USS-RES-001
- **Verify:** Each `http.Server` in the code sets `ReadHeaderTimeout` to a
  value above zero. It also sets `ReadTimeout`, `WriteTimeout` and
  `IdleTimeout` to values above zero, except for a timeout that its handlers
  do not allow (such as a `WriteTimeout` on a handler that streams a response
  for longer). A server that is started with `http.ListenAndServe`, which
  takes no timeouts, fails.
- **References:** Go `net/http` package documentation (Go 1.27.4), Server;
  CWE-400

Each `http.Server` MUST set `ReadHeaderTimeout`, and the read, write and idle
timeouts that its handlers allow, each to a value above zero.

**Rationale:** A server with no timeouts keeps each slow or silent connection
open, so a client that sends its request a byte at a time can use up the
server's connections.
