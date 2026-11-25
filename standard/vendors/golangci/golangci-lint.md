# golangci-lint

- **Kind:** profile
- **Product:** golangci-lint
- **Vendor:** golangci
- **Maintainer:** community
- **Versions:** >=2.15.0, <3.0.0

golangci-lint runs many Go linters and formatters from one configuration. Its
`standard` linter set is `errcheck`, `govet`, `ineffassign`, `staticcheck` and
`unused`; the baseline adds linters for security, error handling, context use
and style, and turns on the formatters `gofmt` and `goimports`. The Go module
recommends it in USS-GO-002. Put the baseline below in `.golangci.yml` in the
repository root. The linters, settings and rule codes that the mappings name
are those of golangci-lint 2.15.0 (which embeds gosec 2.29.0), from its
documentation (read 2026-11-24). `golangci-lint run` runs the enabled
formatters and reports a file that they would change, and exits with status 1
when it reports any issue. The baseline uses the version 2 configuration
format.

Community-maintained: written by USS contributors until the vendor takes it
over.

## Baseline configuration

```yaml
version: "2"

linters:
  default: standard
  enable:
    - bodyclose
    - contextcheck
    - errorlint
    - forcetypeassert
    - gocritic
    - gomoddirectives
    - gosec
    - nilerr
    - noctx
    - nolintlint
    - revive
    - rowserrcheck
    - sqlclosecheck
  settings:
    gosec:
      config:
        global:
          nosec: true
    nolintlint:
      allow-unused: false
      require-explanation: true
      require-specific: true

formatters:
  enable:
    - gofmt
    - goimports

issues:
  max-issues-per-linter: 0
  max-same-issues: 0
  uniq-by-line: false
```

## Mappings

| Rule | Category | Tool rules or settings | Coverage | How to verify |
|---|---|---|---|---|
| USS-GO-004 | Safety | `govet` (the `go vet` analyzers, in the `standard` set) | partial: `govet` runs the vet analyzers of its own release, and the rule needs `go vet ./...` itself in CI with its default analyzers | Read the CI job for `go vet ./...`; run `golangci-lint run` and expect no `govet` finding. |
| USS-GO-011 | Safety | `gomoddirectives` (reports a local `replace`, with `replace-local` at its default `false`) | partial: CI must run the linter on each released module, and the linter reads only the `go.mod` of the module that it lints | Run `golangci-lint run`; read `go.mod` at each release tag for `replace` directives. |
| USS-SAF-001 | Safety | `govet` | partial: `govet` is one of the safety checks that USS-GO-004 names, and the other checks that the Go module names (such as the toolchain pin) are outside the tool | Run `golangci-lint run`; check the other safety checks of the Go module. |
| USS-SAF-003 | Safety | `gosec` `G103` (use of the `unsafe` package) | partial: the rule reports each use but does not judge the comment that says why the use is sound, or its size, and the baseline makes each suppression carry a reason | Run `golangci-lint run`; read each `unsafe` use and its comment or suppression reason. |
| USS-SEC-002 | Safety | `gosec` `G102` (binding to all network interfaces) | partial: the rule finds only a listener that binds to all interfaces, so other insecure defaults need a review | Run `golangci-lint run`; read the default values of the security settings. |
| USS-SEC-004 | Safety | `gosec` `G101` (hard-coded credentials) | partial: the rule finds password-like names with high-entropy values, so a secret scan of the working tree and the history is still needed | Run `golangci-lint run`; a clean run shows that `G101` does not fire. |
| USS-SEC-006 | Safety | `gosec` `G101` | partial: the rule finds secrets in the code, not secret values in a built artifact or an image | Run `golangci-lint run`; inspect the built artifacts and images for secrets. |
| USS-SEC-010 | Safety | `gosec` `G204` (audit of command execution), `G702` (command injection by taint analysis) | partial: the rules find calls that start a process with a variable command or arguments, so the auditor judges each call that remains and each suppression | Run `golangci-lint run`; read each suppression of these rules. |
| USS-SEC-011 | Safety | `gosec` `G709` (unsafe deserialization of untrusted data) | partial: the taint analysis knows only the deserializers that gosec lists, so other formats and libraries that create arbitrary types need a search | Run `golangci-lint run`; search the code for other deserializers. |
| USS-SEC-013 | Safety | `gosec` `G201` (SQL built with a format string), `G202` (SQL built by concatenation), `G701` (SQL injection by taint analysis) | partial: the rules find SQL text that is built from strings, so a query that another code path builds is not seen | Run `golangci-lint run`; read the query code for other ways to build SQL. |
| USS-SEC-014 | Safety | `gosec` `G203` (unescaped data in HTML templates), `G705` (XSS by taint analysis) | partial: only HTML output is seen, so output to a URL, a log or a terminal needs a review | Run `golangci-lint run`; read the code that writes data into another interpreter or format. |
| USS-SEC-016 | Safety | `gosec` `G101` | partial: the rule finds credential-like names with high-entropy values in code, not a default key or token in a configuration file or an image | Run `golangci-lint run`; search the configuration, images and installers for default credentials. |
| USS-SEC-022 | Safety | `gosec` `G402` (weak TLS connection settings) | partial: the rule finds weak TLS settings in Go code only, so cleartext protocols and connections that another library makes need a review | Run `golangci-lint run`; read each connection to another host and a TLS scan of the service. |
| USS-SEC-023 | Safety | `gosec` `G402` (bad TLS connection settings) | partial: only the settings that `G402` checks are seen, so other settings and libraries that turn validation off need a search | Run `golangci-lint run`; search for other settings that turn validation off. |
| USS-SEC-026 | Safety | `gosec` `G401` (MD5 or SHA1), `G403` (RSA key below 2048 bits), `G405` (DES or RC4), `G501`, `G502`, `G503`, `G505` (blocked imports of `crypto/md5`, `crypto/des`, `crypto/rc4`, `crypto/sha1`) | partial: the rules do not know the purpose of a hash or the full list of disallowed algorithms and key sizes | Run `golangci-lint run`; read each use of a hash, cipher or key against the rule. |
| USS-SEC-028 | Safety | `gosec` (the static security analysis linter) | partial: the CI job must run on each pull request and each push to the default branch and fail on a finding, and `gosec` covers only part of what a static security analysis tool checks | Read the CI job that runs `golangci-lint run`; confirm `gosec` in `enable`. |
| USS-GO-013 | Reliability | `noctx` (reports `http.Get`, `http.Head`, `http.Post` and `http.PostForm`) | partial: `noctx` reports the package-level helpers only; it does not report `http.DefaultClient`, an `http.Client` with no `Timeout`, or a context with no deadline | Run `golangci-lint run`; search the code for `http.DefaultClient` and for each `http.Client` without `Timeout`. |
| USS-GO-014 | Reliability | `gosec` `G112` (`ReadHeaderTimeout` not set) | partial: the rule checks only `ReadHeaderTimeout`, not the read, write and idle timeouts, and not a server that `http.ListenAndServe` starts | Run `golangci-lint run`; read each `http.Server` for the other timeouts. |
| USS-DOC-005 | Maintainability | `revive` rule `exported` (an exported name with no comment, in its default rules) | partial: the rule checks that a comment exists, not what it says, and it does not report in a `main` package | Run `golangci-lint run`; read the reference documentation of the public items. |
| USS-GO-001 | Maintainability | the baseline linters: `govet`, `staticcheck`, `errcheck` (correctness), `revive`, `gocritic` (style), `goimports` (imports), `gosec` (security) | partial: CI must run `golangci-lint run` over all Go code and fail on any violation | Read the baseline and the CI job; run `golangci-lint run` and expect exit status 0. |
| USS-GO-003 | Maintainability | the formatter `gofmt` in `formatters.enable` (`simplify` on by default), `goimports` | partial: CI must fail on a finding of `golangci-lint run`, or run `gofmt -l` itself and fail on its output | Read the CI job; run `golangci-lint run` and expect no `gofmt` finding. |
| USS-QLT-001 | Maintainability | the baseline `linters` list | partial: CI must run `golangci-lint run` and fail on any violation, and golangci-lint lints only Go | Read the CI job; run `golangci-lint run` and expect exit status 0. |
| USS-QLT-002 | Maintainability | the formatters `gofmt` and `goimports` | partial: CI must run `golangci-lint run` and fail when a formatter would change a file, and the formatters read only Go files | Read the CI job; run `golangci-lint run` and expect no formatter finding. |
| USS-QLT-004 | Maintainability | `unused` (unused constants, variables, functions and types), `govet` `unreachable`, `revive` `unreachable-code` | partial: commented-out code is not covered | Run `golangci-lint run`; search for commented-out code. |
| USS-QLT-005 | Maintainability | `errcheck` (an unchecked error), `nilerr` (nil returned when an error was checked) | partial: `errcheck` accepts an error that is assigned to `_` on purpose (its `check-blank` setting is off by default), so the auditor reads each such line for a comment | Run `golangci-lint run`; search for `_ =` and `_,` that drop errors. |
| USS-QLT-006 | Maintainability | the `enable` list and `default` of the baseline, with none of them turned off in `disable` | partial: the project's configuration must enable every linter and formatter of the baseline with the baseline's settings, and a linter that is turned off for the whole project is judged by USS-QLT-007 | Compare the project's `linters` and `formatters` with the baseline above. |
| USS-QLT-009 | Maintainability | `nolintlint` with `require-specific` and `require-explanation`; `gosec` `config.global.nosec` | partial: only `//nolint` directives are checked, so a `//lint:ignore` directive of `staticcheck` and a rule in `linters.exclusions` are other forms that the auditor reads | Run `golangci-lint run`; search for `//lint:ignore` and for `exclusions` in the configuration. |
| USS-QLT-010 | Maintainability | `nolintlint` with `require-specific` (rejects `//nolint` and `//nolint:all`) | partial: a directive that names one linter and sits before `package` covers the whole file, and `exclusions` paths and `//lint:ignore` are not checked | Run `golangci-lint run`; search for `//nolint` before `package`, `//lint:ignore` and `exclusions`. |
| USS-QLT-012 | Maintainability | `nolintlint` with `allow-unused: false` | partial: a directive that names a linter that is not enabled is not reported, and unused `//lint:ignore` directives and `exclusions` rules are not covered | Run `golangci-lint run`; read each `//nolint` that names a linter outside the baseline. |

## Check-integrity guards

| Guard | Setting | Rule |
|---|---|---|
| A `//nolint` directive needs an explanation | `linters.settings.nolintlint.require-explanation: true` | USS-QLT-009 |
| A `//nolint` directive names a linter | `linters.settings.nolintlint.require-specific: true` | USS-QLT-009, USS-QLT-010 |
| A `//nolint` directive that suppresses nothing is reported | `linters.settings.nolintlint.allow-unused: false` | USS-QLT-012 |
| `#nosec` comments do not suppress gosec findings | `linters.settings.gosec.config.global.nosec: true` | USS-QLT-009, USS-QLT-010 |
| No issue is left out of the report | `issues.max-issues-per-linter: 0`, `issues.max-same-issues: 0`, `issues.uniq-by-line: false` | USS-QLT-001 |
