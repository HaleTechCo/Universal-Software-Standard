# Oxlint

- **Kind:** profile
- **Product:** Oxlint
- **Vendor:** Oxc project
- **Maintainer:** community
- **Versions:** >=1.95.0, <2.0.0

Oxlint is a linter for JavaScript and TypeScript, written in Rust and run by
itself, with no ESLint. Its `jsx-a11y` plugin has the accessibility rules for
JSX, under the names of the rules of eslint-plugin-jsx-a11y. The React module
recommends it in USS-REACT-004 (eslint-plugin-jsx-a11y is not used: its latest
release, 6.10.2, has a peer range that stops at ESLint 9). This profile covers
the `jsx-a11y` plugin only: the baseline lists no other plugin, so a project
lints the rest with ESLint (`standard/vendors/eslint/eslint.md`). Put the
baseline below in `.oxlintrc.json` in the repository root (Oxlint also reads
`.oxlintrc.jsonc`, `oxlint.config.ts` and `oxlint.config.mts`, one file for each
directory, and `--config` names another file). The rules and settings that the
mappings name are those of Oxlint 1.95.0, from its documentation, its
configuration schema and its source (read 2026-11-24). The baseline lists every
`jsx-a11y` rule of that version (36 rules, `anchor-ambiguous-text` included,
which is a `restriction` rule and so off by default) at error level. The
baseline also sets the `correctness` category to error, so a rule that a later
release adds to that category in the `jsx-a11y` plugin is an error too. Setting
`plugins` replaces the default plugin set, which is why the baseline names
`jsx-a11y` alone. A project that renames its components (such as `Link` for `a`)
maps them in `settings` for the `jsx-a11y` plugin, as the Oxlint configuration
documents. `oxlint` exits with status 1 when a rule at error level fires, when
`denyWarnings` (or `--deny-warnings`) is on and a rule at warn level fires, when
the warnings exceed `maxWarnings` (or `--max-warnings`), when no file is found
and when its options or configuration are invalid; it exits with status 0
otherwise. The rules read the JSX source, not the rendered page.

Community-maintained: written by USS contributors until the vendor takes it
over.

## Baseline configuration

```json
{
  "$schema": "./node_modules/oxlint/configuration_schema.json",
  "plugins": ["jsx-a11y"],
  "categories": {
    "correctness": "error"
  },
  "rules": {
    "jsx-a11y/alt-text": "error",
    "jsx-a11y/anchor-ambiguous-text": "error",
    "jsx-a11y/anchor-has-content": "error",
    "jsx-a11y/anchor-is-valid": "error",
    "jsx-a11y/aria-activedescendant-has-tabindex": "error",
    "jsx-a11y/aria-props": "error",
    "jsx-a11y/aria-proptypes": "error",
    "jsx-a11y/aria-role": "error",
    "jsx-a11y/aria-unsupported-elements": "error",
    "jsx-a11y/autocomplete-valid": "error",
    "jsx-a11y/click-events-have-key-events": "error",
    "jsx-a11y/control-has-associated-label": "error",
    "jsx-a11y/heading-has-content": "error",
    "jsx-a11y/html-has-lang": "error",
    "jsx-a11y/iframe-has-title": "error",
    "jsx-a11y/img-redundant-alt": "error",
    "jsx-a11y/interactive-supports-focus": "error",
    "jsx-a11y/label-has-associated-control": "error",
    "jsx-a11y/lang": "error",
    "jsx-a11y/media-has-caption": "error",
    "jsx-a11y/mouse-events-have-key-events": "error",
    "jsx-a11y/no-access-key": "error",
    "jsx-a11y/no-aria-hidden-on-focusable": "error",
    "jsx-a11y/no-autofocus": "error",
    "jsx-a11y/no-distracting-elements": "error",
    "jsx-a11y/no-interactive-element-to-noninteractive-role": "error",
    "jsx-a11y/no-noninteractive-element-interactions": "error",
    "jsx-a11y/no-noninteractive-element-to-interactive-role": "error",
    "jsx-a11y/no-noninteractive-tabindex": "error",
    "jsx-a11y/no-redundant-roles": "error",
    "jsx-a11y/no-static-element-interactions": "error",
    "jsx-a11y/prefer-tag-over-role": "error",
    "jsx-a11y/role-has-required-aria-props": "error",
    "jsx-a11y/role-supports-aria-props": "error",
    "jsx-a11y/scope": "error",
    "jsx-a11y/tabindex-no-positive": "error"
  },
  "options": {
    "denyWarnings": true,
    "reportUnusedDisableDirectives": "error"
  }
}
```

## Mappings

| Rule | Category | Tool rules or settings | Coverage | How to verify |
|---|---|---|---|---|
| USS-ACC-002 | Accessibility & Usability | the `jsx-a11y` rules of the baseline: `alt-text`, `anchor-ambiguous-text`, `anchor-has-content`, `anchor-is-valid`, `aria-activedescendant-has-tabindex`, `aria-props`, `aria-proptypes`, `aria-role`, `aria-unsupported-elements`, `autocomplete-valid`, `click-events-have-key-events`, `control-has-associated-label`, `heading-has-content`, `html-has-lang`, `iframe-has-title`, `img-redundant-alt`, `interactive-supports-focus`, `label-has-associated-control`, `lang`, `media-has-caption`, `mouse-events-have-key-events`, `no-access-key`, `no-aria-hidden-on-focusable`, `no-autofocus`, `no-distracting-elements`, `no-interactive-element-to-noninteractive-role`, `no-noninteractive-element-interactions`, `no-noninteractive-element-to-interactive-role`, `no-noninteractive-tabindex`, `no-redundant-roles`, `no-static-element-interactions`, `prefer-tag-over-role`, `role-has-required-aria-props`, `role-supports-aria-props`, `scope`, `tabindex-no-positive` | partial: the rules read the JSX source and not the rendered page, so a page that the source does not show (content from data, a component library or a script) is not checked, and the rule needs an automated checker such as axe-core on the pages that the UI serves, in CI, with each page in its input | Read the CI job for the checker on the rendered pages and for the page list; run `oxlint` and expect exit status 0. |
| USS-REACT-003 | Accessibility & Usability | the `jsx-a11y` rules of the baseline (listed in the row above), `plugins: ["jsx-a11y"]`, `categories.correctness: "error"`, `options.denyWarnings: true` | partial: CI must run `oxlint` on all JSX files of each pull request and each push to the default branch and fail on exit status 1, and Oxlint reads JSX files only | Read the CI job; run `oxlint` and expect exit status 0. |

## Check-integrity guards

| Guard | Setting | Rule |
|---|---|---|
| Every rule of the baseline is on, at error level | the `rules` and `categories` of the baseline, with none set to `"off"`, `"allow"` or `"warn"` (an `overrides` entry or a nested configuration file can turn one off for some files, so the auditor reads them too) | USS-QLT-006, USS-QLT-007 |
| Warnings fail the job | `options.denyWarnings: true` (or `--deny-warnings`), and no `maxWarnings` above 0 | USS-QLT-001 |
| No path is skipped | no `ignorePatterns` for files that hold JSX | USS-QLT-006 |
| Disable comments that suppress nothing are reported as errors | `options.reportUnusedDisableDirectives: "error"` (or `--report-unused-disable-directives-severity=error`); it covers `oxlint-disable*` and `eslint-disable*` comments | USS-QLT-012 |
| A suppression names its rules and can give the reason | the comment `// oxlint-disable-next-line <rule> -- <reason>` (Oxlint accepts a reason after `--` and requires none, so the auditor reads each one; `eslint-disable*` comments count too, because `options.respectEslintDisableDirectives` is on by default) | USS-QLT-009 |
| No blanket suppression | `oxlint-disable` and `eslint-disable` comments with no rule name, and the file `oxlint-suppressions.json` that `--suppress-all` writes, are not used | USS-QLT-010 |
