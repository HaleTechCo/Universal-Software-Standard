# deptry

- **Kind:** profile
- **Product:** deptry
- **Vendor:** Osprey OSS
- **Maintainer:** community
- **Versions:** >=0.25.1, <0.26.0

deptry is a command-line tool that compares the modules that a Python
project's files import with the dependencies that the project declares. It
reports missing, unused, transitive and misplaced dependencies, and standard
library modules that are declared as dependencies. Its source repository,
`https://github.com/osprey-oss/deptry`, is in the `osprey-oss` GitHub
organisation (the address of the older `fpgmaas/deptry` repository leads
there). The Python module recommends it in USS-PY-014. Put the baseline below
in `pyproject.toml`. The rules that the mappings name use the codes of the
deptry 0.25.1 documentation (read 2026-11-24). In the 0.25.1 source
(`python/deptry/core.py`), `deptry .` exits with a non-zero status when it
finds any violation.

Community-maintained: written by USS contributors until the vendor takes it
over.

## Baseline configuration

```toml
# Every rule (DEP001 to DEP005) is on. Set no `ignore`, no
# `per_rule_ignores` and no inline `# deptry: ignore[...]`: with none of
# them, deptry reports all five kinds of violation.
[tool.deptry]
```

## Mappings

| Rule | Category | Tool rules or settings | Coverage | How to verify |
|---|---|---|---|---|
| USS-DEP-004 | Safety | `DEP001` (missing dependency), `DEP003` (transitive dependency) | partial: deptry reads only static imports in Python files, so an import made at run time and a use in a file of another kind are not seen | Run `deptry .`; search the code for `importlib` and `__import__`. |
| USS-DEP-005 | Safety | `DEP004` (misplaced development dependency) | partial: the rule checks one half only, a development dependency that non-development code imports; it does not find a tool-only package that sits in the runtime list, so the auditor checks that no tool-only package sits in the runtime list | Run `deptry .`; check the runtime list for packages that only tests or tools use. |
| USS-PY-013 | Safety | `DEP001`, `DEP003` | partial: CI must run `deptry .` and fail the job on its non-zero exit code, and a static import check misses imports made at run time | Read the CI job; confirm that it runs `deptry .` and fails on a non-zero exit code. |
| USS-DEP-013 | Maintainability | `DEP002` (unused dependency) | partial: a dependency that the program loads by reflection or a plugin lookup looks unused to deptry | Run `deptry .`; check each reported dependency in the code and the configuration. |
| USS-QLT-006 | Maintainability | the baseline above: no `ignore`, no `per_rule_ignores` | partial: the project's configuration must keep all five rules on, and a rule that is turned off for the whole project is judged by USS-QLT-007 | Read `[tool.deptry]` for `ignore` and `per_rule_ignores`, and the code for inline ignores. |

## Check-integrity guards

deptry has no setting that reports a disabled rule or an ignore that no longer
ignores anything, in the settings that its documentation lists (read
2026-11-24). A project that sets `ignore` or `per_rule_ignores` gives a reason
for each, as USS-QLT-007 requires.
