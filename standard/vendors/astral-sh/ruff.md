# Ruff

- **Kind:** profile
- **Product:** Ruff
- **Vendor:** Astral
- **Maintainer:** community
- **Versions:** >=0.17.6, <0.18.0

Ruff is a linter and formatter for Python. Its linter reimplements many
checks of other Python linters (Pyflakes, pycodestyle, isort, Bandit and
others) and groups them by code prefix. The Python module recommends it in
USS-PY-002. Put the baseline below in `pyproject.toml`. The rules that the
mappings name use the codes of Ruff 0.17.6, from its documentation (read
2026-11-24). The baseline turns on no preview rule, because a preview rule
can change or go in any release. So it leaves out the check for unreachable
code (`PLW0101`), which is a preview rule in this version. The formatter has
no rule set, so it needs no baseline: CI runs `ruff format --check`.

Community-maintained: written by USS contributors until the vendor takes it
over.

## Baseline configuration

```toml
[tool.ruff.lint]
select = [
    "F", "E", "W", "I", "N", "UP", "B", "S", "BLE", "A", "C4", "SIM",
    "PT", "RET", "ERA", "PGH", "PL", "TRY", "RUF",
]
preview = false
```

## Mappings

| Rule | Category | Tool rules or settings | Coverage | How to verify |
|---|---|---|---|---|
| USS-SEC-002 | Safety | `S201` (Flask debug mode on, in `S`) | partial: the rule finds only Flask's debug mode set on in a call, so other insecure defaults need a review | Run `ruff check .`; read the default values of the security settings. |
| USS-SEC-004 | Safety | `S105`, `S106`, `S107` (hard-coded passwords, in `S`) | partial: these rules find only password-like strings, so a secret scan of the working tree and the history is still needed | Run `ruff check .`; a clean run shows that none of the three rules fires. |
| USS-SEC-006 | Safety | `S105`, `S106`, `S107` (hard-coded passwords, in `S`) | partial: the rules find password-like strings in the code, not secret values in a built artifact or an image | Run `ruff check .`; inspect the built artifacts and images for secrets. |
| USS-SEC-010 | Safety | `S602`, `S604`, `S605` (a shell started by a call, in `S`) | partial: the rules find the calls that use a shell, not whether the command string holds untrusted input, so the auditor judges each call that remains | Run `ruff check .`; read each suppression of these rules. |
| USS-SEC-011 | Safety | `S301` (pickle), `S302` (marshal), `S506` (unsafe YAML load), in `S` | partial: other formats and libraries that can create arbitrary types (such as `shelve` or `jsonpickle`) are not covered | Run `ruff check .`; search the code for other deserializers. |
| USS-SEC-013 | Safety | `S608` (SQL built from strings, in `S`) | partial: the rule finds only SQL text that is built by string formatting, so a query that another code path builds is not seen | Run `ruff check .`; read the query code for other ways to build SQL. |
| USS-SEC-014 | Safety | `S701` (Jinja2 autoescape off), `S704` (unsafe use of `Markup`), `S308` (`mark_safe`), in `S` | partial: only these calls are seen, so output to a URL, a log, a terminal or another template engine needs a review | Run `ruff check .`; read the code that writes data into another interpreter or format. |
| USS-SEC-016 | Safety | `S105`, `S106`, `S107` (hard-coded passwords, in `S`) | partial: the rules find password-like strings and defaults in the code, not a default key or token, or one in a configuration file or an image | Run `ruff check .`; search the configuration, images and installers for default credentials. |
| USS-SEC-022 | Safety | `S312` (Telnet), `S321` (FTP), in `S` | partial: the rules find two cleartext protocols only, so other unencrypted connections and old TLS versions need a review | Run `ruff check .`; read each connection to another host and a TLS scan of the service. |
| USS-SEC-023 | Safety | `S501` (request with certificate validation off), `S323` (unverified SSL context), in `S` | partial: only these calls are seen, so other libraries and settings that turn validation off need a search | Run `ruff check .`; search for other settings that turn validation off. |
| USS-SEC-026 | Safety | `S324`, `S303` (insecure hash functions), `S505` (weak cryptographic key), in `S` | partial: the rules do not know the purpose of a hash or the full list of disallowed algorithms and key sizes | Run `ruff check .`; read each use of a hash, cipher or key against the rule. |
| USS-SEC-028 | Safety | the `S` prefix (the Bandit checks) | partial: the CI job must run on each pull request and each push to the default branch and fail on a finding, and Ruff covers only part of what a static security analysis tool checks | Read the CI job that runs `ruff check .`; confirm `S` in `select`. |
| USS-PY-001 | Maintainability | `F` (correctness), `E` and `W` (style), `I` (import order), `S` (security) in `select` | partial: CI must run `ruff check .` over all Python code and fail on any violation | Read `select`; read the CI job and a run on the default branch. |
| USS-QLT-001 | Maintainability | the baseline `select` list | partial: CI must run `ruff check .` and fail on any violation, and Ruff lints only Python | Read the CI job; run `ruff check .` and expect exit code 0. |
| USS-QLT-002 | Maintainability | `ruff format --check` (the formatter, which has no rule set) | partial: CI must run `ruff format --check .` and fail when it would change a file | Read the CI job; run `ruff format --check .` and expect exit code 0. |
| USS-QLT-004 | Maintainability | `F401` (unused import), `F841` (unused variable), `ERA001` (commented-out code) | partial: unreachable code (`PLW0101` is a preview rule) and unused functions and types are not covered | Run `ruff check .`; search for unreachable code and unused definitions. |
| USS-QLT-005 | Maintainability | `S110`, `S112` (an exception caught and dropped), `BLE001` (blind `except`) | partial: an error that is returned and ignored is not covered | Run `ruff check .`; search for dropped return values that hold errors. |
| USS-QLT-006 | Maintainability | `select` of the baseline, with no baseline prefix in `ignore` | partial: the project's configuration must enable every prefix of the baseline at the baseline's severity, and a prefix that is turned off for the whole project is judged by USS-QLT-007 | Compare the project's `select` and `ignore` with the baseline above. |
| USS-QLT-009 | Maintainability | `PGH004` (blanket `noqa`), `PGH003` (blanket `type: ignore`) | partial: the rules check that a suppression names a rule, not that it gives a reason on the same line | Run `ruff check .`; read each suppression for its reason. |
| USS-QLT-010 | Maintainability | `PGH004` (blanket `noqa`, a file-level `# ruff: noqa` with no code too), `PGH003` | partial: a file-level suppression that names a rule is not blanket for Ruff, and the rule forbids it | Run `ruff check .`; search for `# ruff: noqa` and other file-level directives. |
| USS-QLT-012 | Maintainability | `RUF100` (unused `noqa`) | full | Confirm `RUF` in `select` and no `RUF100` in `ignore`; run `ruff check .`. |

## Check-integrity guards

| Guard | Setting | Rule |
|---|---|---|
| Blanket `noqa` comments are reported | `PGH004` (in `PGH`) | USS-QLT-009, USS-QLT-010 |
| `noqa` comments that suppress nothing are reported | `RUF100` (in `RUF`) | USS-QLT-012 |
