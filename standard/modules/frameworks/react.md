# React

- **Prefix:** REACT
- **Kind:** module
- **Applies when:** framework.react

This module covers projects that use React: the lint rules that only React code
needs, the safe use of `dangerouslySetInnerHTML` and `StrictMode`. The
TypeScript and JavaScript module (`standard/modules/languages/typescript.md`)
holds the rest of the lint, so a React project meets it as well. The module
uses two vendor profiles. The first is
`standard/vendors/facebook/eslint-plugin-react-hooks.md`, for an ESLint plugin
whose guards are those of the ESLint profile
(`standard/vendors/eslint/eslint.md`). The second is
`standard/vendors/oxc-project/oxlint.md`, for a separate linter with guards of
its own. The module names no tool without a profile. The rules were checked
against React 19.3.1, eslint-plugin-react-hooks 7.1.1 and Oxlint 1.95.0, the
current releases on 2026-11-24. USS-REACT-006 is the React safety check that
USS-SAF-001 makes binding for a React project, because a framework module has
no language module of its own to name it. The core rules that this module does
not refine apply as written.

### USS-REACT-001: Lint the Rules of Hooks

- **Level:** MUST
- **Applies when:** always
- **Category:** Maintainability
- **Refines:** USS-QLT-001
- **Verify:** The CI configuration runs a linter over the project's components
  and hooks (all of its `.jsx` and `.tsx` files, and each `.js` or `.ts` file
  that defines a component or a hook), and the job fails on a violation. The
  effective configuration of each such file enables a check of the Rules of
  Hooks and a check of the dependencies of hooks at error level. For ESLint,
  these are `react-hooks/rules-of-hooks` and `react-hooks/exhaustive-deps` set
  to `"error"` (the preset of eslint-plugin-react-hooks sets the second to
  `"warn"`), and the job fails when ESLint exits with status 1, which it does
  when a rule at error level fires. A file that the linter does not read, a
  check at warn level that the job does not count as a failure, and a check
  that is off fail.
- **References:** React documentation, Rules of Hooks (read 2026-11-24);
  eslint-plugin-react-hooks 7.1.1, README and source of the `recommended`
  configuration (read 2026-11-24)

CI MUST lint components and hooks against the Rules of Hooks and the
dependencies of hooks, with both checks as errors.

**Rationale:** A hook that runs in a condition or a loop, or that misses a
dependency, breaks without a syntax error: state attaches to the wrong hook or
an effect runs on stale values. The two checks find the mistake as the code is
written, but only an error fails the build, so a check at warn level is soon
unread.

### USS-REACT-002: Lint with eslint-plugin-react-hooks

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Maintainability
- **Refines:** USS-QLT-001
- **Verify:** CI runs `eslint` on the project's components and hooks, and the
  job fails when ESLint exits with status 1 (which it does when a rule at error
  level fires). The ESLint configuration includes the baseline of the profile
  `standard/vendors/facebook/eslint-plugin-react-hooks.md`: the flat
  `recommended` configuration of eslint-plugin-react-hooks, and then
  `react-hooks/rules-of-hooks` and `react-hooks/exhaustive-deps` set to
  `"error"`. A project that uses another tool fails this rule (it records a
  Standard Deviation) and meets the outcome rule through that tool, configured
  to at least the baseline that USS-QLT-006 sets.
- **References:** eslint-plugin-react-hooks 7.1.1, README (read 2026-11-24);
  ESLint 10.15.0 documentation, Command Line Interface (exit codes)

The project SHOULD lint with eslint-plugin-react-hooks, configured as its
vendor profile says.

**Rationale:** The plugin is the one that the React documentation names for
the Rules of Hooks, and it is written by the team that writes React. The
profile sets both hook checks to errors, which the preset leaves at warn for
the dependency check.

### USS-REACT-003: Lint JSX for accessibility

- **Level:** MUST
- **Applies when:** has-web-ui
- **Category:** Accessibility & Usability
- **Refines:** USS-ACC-002
- **Verify:** The CI configuration runs a linter over all of the project's JSX
  files (`.jsx` and `.tsx`, and each `.js` or `.ts` file that contains JSX),
  and the effective configuration turns on the linter's accessibility rules
  for JSX at error level, with at least a rule for each of these: the text
  alternative of an image (`alt-text`), the label of a form control
  (`label-has-associated-control`), valid ARIA attributes (`aria-props`) and
  roles (`aria-role`), and keyboard handlers on elements with click handlers
  (`click-events-have-key-events`). The names are those of Oxlint's `jsx-a11y`
  plugin and of the recommended set of eslint-plugin-jsx-a11y, which both have
  these rules. The job fails when the linter reports a violation, as the
  linter's documentation states for its exit status (Oxlint exits with status
  1). A file that the linter does not read, a rule at warn level that the job
  does not count as a failure and a job that tolerates failures fail.
- **References:** Oxlint 1.95.0, documentation, Configuration and jsx-a11y
  rules (read 2026-11-24); eslint-plugin-jsx-a11y 6.10.2, source of the
  `recommended` configuration (read 2026-11-24)

CI MUST lint JSX against accessibility rules, and a violation MUST fail the
build.

**Rationale:** Many accessibility faults are visible in the JSX itself: an
image with no text alternative, a click handler on an element that a keyboard
cannot reach, a label that belongs to no control. A linter finds them as the
code is written, and before a rendered page exists. It reads the source only,
so the checker of USS-ACC-002 on the rendered pages is still needed.

### USS-REACT-004: Lint JSX with Oxlint

- **Level:** SHOULD
- **Applies when:** has-web-ui
- **Category:** Accessibility & Usability
- **Refines:** USS-ACC-002
- **Verify:** CI runs `oxlint` over all of the project's JSX files, and the job
  fails when Oxlint exits with status 1, which it does when a rule at error
  level fires, when warnings are denied and a rule at warn level fires, or
  when the warnings exceed `--max-warnings`. Oxlint reads `.oxlintrc.json`,
  `.oxlintrc.jsonc`, `oxlint.config.ts` or `oxlint.config.mts` from the current
  directory, or the file that `--config` names, and the configuration includes
  the baseline of the profile
  `standard/vendors/oxc-project/oxlint.md`: `plugins` lists `jsx-a11y`, each
  `jsx-a11y` rule of the baseline is set to `"error"` (or `"deny"`), and
  `options.denyWarnings` is `true` (or `--deny-warnings` is passed). A project
  that uses another tool fails this rule (it records a Standard Deviation) and
  meets the outcome rule through that tool, configured to at least the
  baseline that USS-QLT-006 sets.
- **References:** Oxlint 1.95.0, documentation, Configuration and CLI (read
  2026-11-24); Oxlint 1.95.0, source of the exit status in `result.rs` and
  `lint.rs` (read 2026-11-24)

The project SHOULD lint JSX with Oxlint, configured as its vendor profile
says.

**Rationale:** Oxlint has the rules of eslint-plugin-jsx-a11y under the same
names and runs by itself, so it checks JSX whichever ESLint major version the
project is on. The profile fixes the rule set and the exit status that make
"no findings" mean the same in every project.

### USS-REACT-005: No unsanitized HTML

- **Level:** MUST NOT
- **Applies when:** always
- **Category:** Safety
- **Refines:** USS-SEC-014
- **Verify:** Search the project's code for `dangerouslySetInnerHTML`. For each
  use, trace the value of `__html` back to its source. It passes when it is a
  string literal written in the project's source code with no interpolated
  value (a quoted string, or a template literal with no `${...}`), which is
  static markup that the project authored. Any other value passes only when it
  is the result of a call to a vetted HTML sanitizer (a maintained library
  built to sanitize HTML, such as DOMPurify), and nothing is added to it after
  that call. A value that comes from a regular expression, an escape function
  or a hand-written filter, from a Markdown or other converter with no
  sanitizer after it, from a file or a response, or from a template literal
  with an interpolated value, fails.
- **References:** React documentation, Common components,
  `dangerouslySetInnerHTML` (read 2026-11-24); DOMPurify 3.5.0, README (read
  2026-11-24); CWE-79

`dangerouslySetInnerHTML` MUST NOT receive a value that a vetted HTML sanitizer,
such as DOMPurify, has not cleaned, unless the value is a string literal written
in the project's source code with no interpolated value. A vetted sanitizer is
a maintained library built to sanitize HTML; a regular expression or a
hand-written filter is not one.

**Rationale:** The prop sets the `innerHTML` of an element, so any script in the
value runs with the rights of the page, which is cross-site scripting (CWE-79).
React escapes text that it renders itself, and this prop is the way around that
escape. A sanitizer removes the markup that can run code, and a check that
the author wrote by hand can miss a form that it does not know. A string literal
with no interpolated value is markup that the project wrote and reviews like its
code, so no outside input can reach it; any other value can carry input that
the author did not write.

### USS-REACT-006: Render the root in StrictMode

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Refines:** USS-SAF-001
- **Verify:** Find where the application creates its root (the calls of
  `createRoot` or `hydrateRoot`). The element that the root renders has
  `<StrictMode>` (or `<React.StrictMode>`) around the whole application. In a
  framework that creates the root itself, the framework's setting turns it on.
  For Next.js, an App Router application (Next.js 13.5.1 or later) passes
  unless `reactStrictMode` is `false` in the Next.js configuration, and a Pages
  Router application passes only when `reactStrictMode` is `true` there. A
  `StrictMode` around only a part of the application fails.
- **References:** React documentation, `StrictMode` (read 2026-11-24); Next.js
  documentation, `reactStrictMode` (read 2026-11-24)

The application MUST render its root inside `<StrictMode>`, itself or through
the setting of the framework that creates the root.

**Rationale:** `StrictMode` re-runs renders, effects and ref callbacks, and
warns about deprecated APIs, to expose impure rendering and missing cleanup.
Its checks run only in development and not in the production build, so the
safety that it gives is that those defects are found in development and in the
tests that run development builds, before release. USS-SAF-001 makes the
toolchain's safety checks binding, and this is the one that React provides.
