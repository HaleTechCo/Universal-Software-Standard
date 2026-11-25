# OSV-Scanner

- **Kind:** profile
- **Product:** OSV-Scanner
- **Vendor:** Google
- **Maintainer:** community
- **Versions:** >=2.7.1, <3.0.0

OSV-Scanner reads a project's lock files and manifests and reports the
packages in them that have known vulnerabilities in the OSV database. The
Python module recommends it in USS-PY-015. The TypeScript module and the Maven
module recommend it too, each for its own lock files. Put the baseline below
in an `osv-scanner.toml` file in the directory of each lock file, because the
file applies only to the files in its own directory. The commands, the
settings and the exit codes that the profile names are from the OSV-Scanner V2
documentation (read 2026-11-24), and the version range is from release 2.7.1.

Community-maintained: written by USS contributors until the vendor takes it
over.

## Baseline configuration

```toml
# Scan every lock file of the repository, and fail the CI job on any
# finding (exit code 1):
#
#   osv-scanner scan source --recursive .
#
# Each ignore entry below carries a reason and an expiry date. An entry
# looks like this one, which is a comment and ignores nothing:
#
# [[IgnoredVulns]]
# id = "<vulnerability ID>"
# effectiveUntil = <expiry date, such as 2026-12-24>
# reason = "<why the project is not affected, or the issue that tracks the fix>"
```

## Mappings

| Rule | Category | Tool rules or settings | Coverage | How to verify |
|---|---|---|---|---|
| USS-DEP-007 | Safety | `osv-scanner scan source --recursive .` over every supported lock file | partial: the CI job must run on each pull request, on each push to the default branch and on a schedule, and vendored code needs its own record | Read the CI job and its triggers; confirm that it fails on exit code 1. |
| USS-DEP-008 | Safety | `IgnoredVulns` entries in `osv-scanner.toml`, each with `reason` and `effectiveUntil` | partial: an ignore for an affected product must link an issue and expire at most 30 days after it is recorded, and each finding that is not ignored must be fixed | Run the scan and expect exit code 0; read each `IgnoredVulns` entry for its reason and its date. |

## Check-integrity guards

| Guard | Setting | Rule |
|---|---|---|
| Each ignore entry says why | `reason` of each `IgnoredVulns` entry | USS-DEP-008 |
| Each ignore entry expires | `effectiveUntil` of each `IgnoredVulns` entry | USS-DEP-008 |
