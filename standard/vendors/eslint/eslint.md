# ESLint with typescript-eslint

- **Kind:** profile
- **Product:** ESLint with typescript-eslint
- **Vendor:** ESLint and typescript-eslint
- **Maintainer:** community
- **Versions:** eslint >=10.15.0, <11.0.0; @eslint/js >=10.0.1, <11.0.0;
  typescript-eslint >=8.74.0, <9.0.0;
  @eslint-community/eslint-plugin-eslint-comments >=4.8.1, <5.0.0

ESLint is a linter for JavaScript and, with typescript-eslint, for TypeScript.
typescript-eslint adds the rules that read the type information of the
TypeScript compiler. The ESLint comments plugin adds rules for the directive
comments that suppress findings. The TypeScript module recommends the
combination in USS-TS-002. Put the baseline below in `eslint.config.mjs` (any
of the flat config file names that ESLint reads works). The type-aware rules
need each linted file to be in a TypeScript project, and a config file such as
`eslint.config.mjs` usually is not, so the baseline lists it in
`allowDefaultProject`, which typescript-eslint offers for config files (a
glob there cannot use `**`, and a file in it must not be in the nearest
`tsconfig.json`). Another config file at the project root goes in the same
list, or gets typescript-eslint's `disableTypeChecked` configuration in a
block of its own (`{ files: ["**/*.js"], extends:
[tseslint.configs.disableTypeChecked] }`). The profile has four version ranges
because it covers four packages that release on their own schedules; each
range runs from the version that the profile was checked
against to the next major version. The rules and settings that the mappings
name are those of ESLint 10.15.0, typescript-eslint 8.74.0 and the comments
plugin 4.8.1, from their documentation and sources (read 2026-11-24).
typescript-eslint 8.74.0 supports TypeScript versions from 4.8.4 up to but not
including 6.1.0, so a project on TypeScript 7 cannot use this baseline until a
typescript-eslint release supports it. The `strictTypeChecked` configuration
is not stable under semantic versioning: typescript-eslint can change its
rules in a minor release, so an upgrade within the range can bring new
findings. The baseline turns on no rule from a third-party import plugin;
import order is left to the project. ESLint exits with status 1 when a rule
at error level fires or when the warnings exceed `--max-warnings`, and the
baseline sets every rule to error.

Community-maintained: written by USS contributors until the vendor takes it
over.

## Baseline configuration

```js
// eslint.config.mjs
import comments from "@eslint-community/eslint-plugin-eslint-comments/configs";
import js from "@eslint/js";
import { defineConfig } from "eslint/config";
import tseslint from "typescript-eslint";

export default defineConfig({
  files: ["**/*.{js,jsx,mjs,cjs,ts,tsx,mts,cts}"],
  extends: [
    js.configs.recommended,
    tseslint.configs.strictTypeChecked,
    tseslint.configs.stylisticTypeChecked,
    comments.recommended,
  ],
  languageOptions: {
    parserOptions: {
      // Config files that no tsconfig.json covers; add each other one here.
      projectService: { allowDefaultProject: ["eslint.config.mjs"] },
    },
  },
  linterOptions: {
    reportUnusedDisableDirectives: "error",
    reportUnusedInlineConfigs: "error",
  },
  rules: {
    // Each suppression names its rule and gives a reason after "--".
    "@eslint-community/eslint-comments/require-description": "error",
    "@eslint-community/eslint-comments/no-unlimited-disable": "error",
    // Imports.
    "no-duplicate-imports": ["error", { allowSeparateTypeImports: true }],
    "@typescript-eslint/consistent-type-imports": "error",
    "@typescript-eslint/no-import-type-side-effects": "error",
    // Code built from strings (no-implied-eval is in strictTypeChecked).
    "no-eval": "error",
    "no-new-func": "error",
    "no-script-url": "error",
  },
});
```

## Mappings

| Rule | Category | Tool rules or settings | Coverage | How to verify |
|---|---|---|---|---|
| USS-SAF-001 | Safety | `@typescript-eslint/no-unnecessary-condition` (in `strictTypeChecked`; it reports an error for each file when `strictNullChecks` is off) | partial: ESLint sees only that `strictNullChecks` is on, so `noUncheckedIndexedAccess`, `exactOptionalPropertyTypes`, `noImplicitOverride`, `noImplicitReturns`, `noFallthroughCasesInSwitch` and the strict mode of JavaScript are judged by the compiler options and the source (USS-TS-004, USS-TS-005) | Run `eslint .` and expect no error from the rule; read the effective TypeScript configuration for the other options. |
| USS-SEC-028 | Safety | `@typescript-eslint/no-implied-eval` (in `strictTypeChecked`), `no-eval`, `no-new-func`, `no-script-url` (the code-injection rules of the baseline) | partial: with the baseline, ESLint is not the SAST tool of the rule for TypeScript and JavaScript code, because its core and typescript-eslint rules are not a security rule set (they have no rules for injection into queries, shells or markup, for cryptography or for secrets); a SAST tool such as CodeQL or Semgrep is still needed, CI must run it on each pull request and each push to the default branch and fail on a finding, and each suppression needs a recorded reason | Read the CI job of the SAST tool and its triggers; confirm the tool's rule set; read the reason of each suppression. |
| USS-TS-004 | Safety | `@typescript-eslint/no-unnecessary-condition` (in `strictTypeChecked`; it reports an error for each file when `strictNullChecks` is off) | partial: ESLint sees only that `strictNullChecks` is on, so the other options of the rule are judged from the effective TypeScript configuration, and CI must run the type check | Run `eslint .` and expect no error from the rule; read `tsc --showConfig`; read the CI job. |
| USS-CON-001 | Maintainability | `@typescript-eslint/no-empty-function` (a function with an empty body, in `stylisticTypeChecked`) | partial: the rule finds only functions with an empty body, so stubs that return a constant, "not implemented" errors, `TODO` markers and sample data returned as a result need a search | Run `eslint .`; search the production code for the other placeholders. |
| USS-QLT-001 | Maintainability | the baseline `extends` and `rules`, with type information from `projectService` | partial: CI must run `eslint .` on each pull request and each push to the default branch and fail on any violation (a rule at warn level needs `--max-warnings 0`), and ESLint lints JavaScript and TypeScript only | Read the CI job; run `eslint .` and expect exit status 0. |
| USS-QLT-004 | Maintainability | `no-unreachable`, `@typescript-eslint/no-unused-vars` (unused variables, parameters, imports and functions), `no-unused-private-class-members` | partial: unused exports and unused files are not reported (the knip profile covers them), and commented-out code is not covered | Run `eslint .`; search for commented-out code. |
| USS-QLT-005 | Maintainability | `no-empty` (an empty block, an empty `catch` block included, with no comment in it), `@typescript-eslint/no-floating-promises` (a promise with no rejection handler) | partial: a `catch` block that has statements and does not handle the error, an error that is returned and ignored, and a rejection that `void` hides are not covered | Run `eslint .`; search for places that drop an error. |
| USS-QLT-009 | Maintainability | `@eslint-community/eslint-comments/require-description` (a directive comment needs a description after `--`), `@eslint-community/eslint-comments/no-unlimited-disable` (a directive must name its rules), `@typescript-eslint/ban-ts-comment` (`@ts-expect-error` needs a description of at least 10 characters, the value that `strictTypeChecked` sets for `minimumDescriptionLength` (the rule's own default is 3), and `@ts-ignore` and `@ts-nocheck` are reported) | partial: the rules check that a description exists, not what it says, `@ts-expect-error` names no rule because TypeScript has none, and the suppressions of other tools are outside them | Run `eslint .`; read each reason. |
| USS-QLT-010 | Maintainability | `@eslint-community/eslint-comments/no-unlimited-disable` (`eslint-disable`, `eslint-disable-line` and `eslint-disable-next-line` with no rule name), `@typescript-eslint/ban-ts-comment` (`@ts-ignore` and `@ts-nocheck`) | partial: a file-level `/* eslint-disable <rule> */` that names a rule is not reported, and the suppressions of other tools are outside the rules | Run `eslint .`; search for `eslint-disable` at the top of a file and for `@ts-nocheck`. |
| USS-QLT-011 | Maintainability | `@typescript-eslint/ban-ts-comment` (`@ts-ignore` is reported, `@ts-expect-error` is allowed with a description) | partial: the rule covers TypeScript's directives only, since ESLint's own directives have no form that fails when the problem is gone | Run `eslint .`; search for `@ts-ignore`. |
| USS-QLT-012 | Maintainability | `linterOptions.reportUnusedDisableDirectives: "error"`, `linterOptions.reportUnusedInlineConfigs: "error"` | full | Confirm both settings in `linterOptions`; run `eslint .` and expect exit status 0. |
| USS-TS-001 | Maintainability | `js.configs.recommended` and `strictTypeChecked` (correctness), `stylisticTypeChecked` (style), `no-duplicate-imports`, `@typescript-eslint/consistent-type-imports`, `@typescript-eslint/no-import-type-side-effects`, `@typescript-eslint/no-require-imports` (imports), `projectService` (type information) | partial: the baseline has no rule for import order or for unresolved imports, which a third-party import plugin offers, and CI must run `eslint .` on all code and fail on any violation | Read the configuration for the three groups; read the CI job; run `eslint .` and expect exit status 0. |

## Check-integrity guards

| Guard | Setting | Rule |
|---|---|---|
| Disable comments that suppress nothing are reported as errors | `linterOptions.reportUnusedDisableDirectives: "error"` | USS-QLT-012 |
| Inline configuration comments that change nothing are reported as errors | `linterOptions.reportUnusedInlineConfigs: "error"` | USS-QLT-012 |
| A directive comment needs a description | `@eslint-community/eslint-comments/require-description` | USS-QLT-009 |
| A directive comment cannot disable every rule | `@eslint-community/eslint-comments/no-unlimited-disable` | USS-QLT-010 |
