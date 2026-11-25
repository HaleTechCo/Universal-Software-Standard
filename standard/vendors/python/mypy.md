# mypy

- **Kind:** profile
- **Product:** mypy
- **Vendor:** Python
- **Maintainer:** community
- **Versions:** >=2.4.1, <3.0.0

mypy is a static type checker for Python. Its source repository is in the
`python` GitHub organisation. The Python module recommends it in
USS-PY-004. Put the baseline below in `pyproject.toml`. The settings and
error codes that the baseline and the mappings name are from the mypy 2.4.1
documentation. `strict = true` turns on a list of checks that mypy can change
in a later release, so the baseline also sets `warn_unused_ignores` itself.
The error codes of the baseline are optional checks that `strict` does not
turn on.

Community-maintained: written by USS contributors until the vendor takes it
over.

## Baseline configuration

```toml
[tool.mypy]
strict = true
warn_unused_ignores = true
warn_unreachable = true
enable_error_code = [
    "ignore-without-code",
    "redundant-expr",
    "truthy-bool",
    "possibly-undefined",
]
```

## Mappings

| Rule | Category | Tool rules or settings | Coverage | How to verify |
|---|---|---|---|---|
| USS-PY-003 | Safety | `strict = true` | partial: CI must run mypy on all Python code and fail on any error, and no file may be left out or set to a lower mode | Read the CI job and the mypy settings for `exclude` and per-module overrides; run `mypy .` and expect exit code 0. |
| USS-SAF-001 | Safety | `strict = true`, `enable_error_code` | partial: the baseline covers the type-checking options that the Python module names, and CI must run mypy on each build | Read the mypy settings and the CI job; run `mypy .`. |
| USS-QLT-003 | Maintainability | `strict = true`, `warn_unreachable = true` (checks that report as errors) | partial: CI must fail the job on any mypy error, and the project's other compilers and build tools need the same | Read the CI job; run `mypy .` and expect no error. |
| USS-QLT-004 | Maintainability | `warn_unreachable = true` | partial: only unreachable code that type analysis can see is reported, not unused functions or commented-out code | Run `mypy .`; search for unused definitions. |
| USS-QLT-006 | Maintainability | the baseline settings above | partial: the project's configuration must set each baseline setting and error code at least as strictly, and one that is turned off for the whole project is judged by USS-QLT-007 | Compare the project's mypy settings with the baseline above. |
| USS-QLT-009 | Maintainability | the `ignore-without-code` error code | partial: the code checks that each `type: ignore` names an error code, not that it gives a reason on the same line | Run `mypy .`; read each `type: ignore` for its reason. |
| USS-QLT-010 | Maintainability | the `ignore-without-code` error code | partial: the code reports a `type: ignore` with no error code, but not a file-level directive such as `# mypy: ignore-errors` | Run `mypy .`; search for file-level directives. |
| USS-QLT-012 | Maintainability | `warn_unused_ignores = true` | full | Read the mypy settings for `warn_unused_ignores = true` and no override that turns it off. |

## Check-integrity guards

| Guard | Setting | Rule |
|---|---|---|
| A `type: ignore` that suppresses nothing is reported | `warn_unused_ignores` | USS-QLT-012 |
| A `type: ignore` with no error code is reported | the `ignore-without-code` error code | USS-QLT-009 |
