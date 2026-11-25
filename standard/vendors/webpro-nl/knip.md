# knip

- **Kind:** profile
- **Product:** knip
- **Vendor:** knip project
- **Maintainer:** community
- **Versions:** >=6.48.0, <7.0.0

knip finds unused files, exports, dependencies and binaries in a JavaScript or
TypeScript project, and the dependencies that the code uses and `package.json`
does not list. It reads the project's entry files, follows the imports, and
reads the configuration of other tools through its plugins. The TypeScript
module recommends it in USS-TS-014. Put the baseline below in `knip.jsonc`,
a format that allows comments, so that each ignore entry carries its reason.
knip also reads `knip.json` and `knip.ts`, which suit a project that has no
ignore entry. The issue types, settings and exit status that the profile names
are from the knip 6.48.0 documentation and its JSON schema (read 2026-11-24).
knip exits with status 1 when it reports at least one issue, with status 0
when it reports none, and with status 2 when it cannot run. A rule set to
`warn` is printed and not counted, so the baseline sets every rule to `error`.
Three issue types (`nsExports`, `nsTypes` and `cycles`) are not reported by
default: the baseline adds the first two with `include`, and turns `cycles` on
with its rule (its default is `warn`). CI runs `knip` and fails the job on
exit status 1. A second run, `knip --production`, with the production files
marked by a trailing `!` in `entry` and `project`, finds the packages in
`dependencies` that production code does not use.

Community-maintained: written by USS contributors until the vendor takes it
over.

## Baseline configuration

```jsonc
{
  "$schema": "https://unpkg.com/knip@6/schema-jsonc.json",
  // Adds the two issue types that are off by default to the report.
  "include": ["nsExports", "nsTypes"],
  "rules": {
    "files": "error",
    "subpaths": "error",
    "dependencies": "error",
    "devDependencies": "error",
    "optionalPeerDependencies": "error",
    "unlisted": "error",
    "binaries": "error",
    "unresolved": "error",
    "exports": "error",
    "types": "error",
    "nsExports": "error",
    "nsTypes": "error",
    "enumMembers": "error",
    "namespaceMembers": "error",
    "duplicates": "error",
    "catalog": "error",
    "catalogReferences": "error",
    "cycles": "error"
  }
  // Each ignore entry below carries a comment that says why. An entry looks
  // like this one, which is a comment and ignores nothing:
  //
  // "ignoreDependencies": [
  //   // <why knip cannot see that the package is used>
  //   "<package name>"
  // ]
}
```

## Mappings

| Rule | Category | Tool rules or settings | Coverage | How to verify |
|---|---|---|---|---|
| USS-DEP-004 | Safety | `unlisted` (a package that the code uses and `package.json` does not list), `binaries` (a binary of a dependency that `package.json` does not list) | partial: CI must run `knip` and fail on exit status 1, knip checks only the files that its `entry` and `project` settings reach, and in a workspace a dependency that only the root `package.json` lists is not reported unless `--strict` is on | Read the CI job and the `entry` and `project` settings; run `knip` and expect exit status 0. |
| USS-DEP-005 | Safety | `dependencies` in a run with `--production` (a package in `dependencies` that production code does not use, such as one that only tests and tooling use) | partial: the baseline run alone does not decide the rule, so CI must also run `knip --production` with `!`-marked entry patterns, and a runtime package that is wrongly in `devDependencies` is not reported | Run `knip --production` and expect exit status 0; read the `devDependencies` list for runtime packages. |
| USS-TS-012 | Safety | `dependencies` in a run with `--production` (a package in `dependencies` that production code does not use, such as one that only tests and tooling use) | partial: the baseline run alone does not decide the rule, so CI must also run `knip --production` with `!`-marked entry patterns, and a runtime package that is wrongly in `devDependencies` is not reported | Run `knip --production` and expect exit status 0; read the `devDependencies` list for runtime packages. |
| USS-TS-013 | Safety | `unlisted` (a package that the code uses and `package.json` does not list) | partial: CI must run `knip` and fail on exit status 1, knip checks only the files that its `entry` and `project` settings reach, and in a workspace a dependency that only the root `package.json` lists is not reported unless `--strict` is on | Read the CI job and the `entry` and `project` settings; run `knip` and expect exit status 0. |
| USS-DEP-013 | Maintainability | `dependencies` (a dependency, or a devDependency, that knip finds no reference to) | partial: a dependency that the program loads by name at run time (by reflection or a plugin lookup) looks unused to knip, and needs an ignore entry with its reason | Run `knip` and expect exit status 0; read each `ignoreDependencies` entry for its reason. |
| USS-QLT-004 | Maintainability | `files` (unused files), `exports` and `types` (unused exports and exported types), `enumMembers` and `namespaceMembers` (unused members) | partial: exports of entry files are not reported unless `includeEntryExports` is on, and unreachable code, unused local variables and commented-out code are not covered (the ESLint profile covers the first two) | Run `knip` and expect exit status 0; search for commented-out code. |

## Check-integrity guards

knip has no check-integrity settings of its own. The baseline sets every rule
to `error`, and it asks for a comment on each ignore entry in its
configuration.
