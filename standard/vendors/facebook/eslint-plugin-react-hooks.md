# eslint-plugin-react-hooks

- **Kind:** profile
- **Product:** eslint-plugin-react-hooks
- **Vendor:** The React team at Meta
- **Maintainer:** community
- **Versions:** >=7.1.1, <8.0.0

eslint-plugin-react-hooks is the ESLint plugin of the React team. Its rule
`rules-of-hooks` checks the Rules of Hooks and its rule `exhaustive-deps` checks
the dependency arrays of hooks. Its `recommended` preset also holds the lint
rules of the React Compiler, such as `purity`, `refs` and `set-state-in-render`.
The React module recommends the plugin in USS-REACT-002. It is an ESLint plugin,
so its baseline goes in the `eslint.config.mjs` of the ESLint profile
(`standard/vendors/eslint/eslint.md`), as one more object in the array that
`defineConfig` receives, and the guards of that profile apply to it. The rules
and presets that the mappings name are those of eslint-plugin-react-hooks 7.1.1
(published 2026-04-17), from its README and the source of its package (read
2026-11-24). Version 7 of the plugin supports ESLint up to 10. The flat preset
`reactHooks.configs.flat.recommended` sets `rules-of-hooks` to `error` and
`exhaustive-deps` to `warn`, and some of the Compiler rules to `warn` (such as
`unsupported-syntax` and `incompatible-library`), so the baseline sets both hook
rules to `error` after the preset. The preset can gain rules in a minor release,
so an upgrade within the range can bring new findings. A rule at warn level
fails a CI job only when ESLint runs with `--max-warnings 0`; ESLint exits with
status 1 when a rule at error level fires.

Community-maintained: written by USS contributors until the vendor takes it
over.

## Baseline configuration

```js
// eslint.config.mjs: add this object to the array that defineConfig receives,
// next to the object of the ESLint profile.
import reactHooks from "eslint-plugin-react-hooks";
import { defineConfig } from "eslint/config";

export default defineConfig([
  // ... the baseline object of the ESLint profile ...
  {
    files: ["**/*.{js,jsx,mjs,cjs,ts,tsx,mts,cts}"],
    extends: [reactHooks.configs.flat.recommended],
    rules: {
      // The preset sets the second to "warn"; the baseline makes both errors.
      "react-hooks/rules-of-hooks": "error",
      "react-hooks/exhaustive-deps": "error",
    },
  },
]);
```

## Mappings

| Rule | Category | Tool rules or settings | Coverage | How to verify |
|---|---|---|---|---|
| USS-QLT-001 | Maintainability | the baseline `extends` (the `recommended` preset) and `rules` | partial: the plugin checks React hooks and the React Compiler's rules only, so CI must run `eslint .` on each pull request and each push to the default branch and fail on any violation (a rule at warn level needs `--max-warnings 0`), and the rest of the lint is the ESLint profile's | Read the CI job; run `eslint .` and expect exit status 0. |
| USS-REACT-001 | Maintainability | `react-hooks/rules-of-hooks` (the Rules of Hooks: hooks called in conditions, loops, nested functions and non-React functions), `react-hooks/exhaustive-deps` (missing and extra dependencies of `useEffect`, `useCallback`, `useMemo` and the other hooks that take a dependency array), both set to `"error"` | partial: CI must run ESLint over all components and hooks and fail on exit status 1 | Read the CI job; read the effective configuration for both rules at `"error"`; run `eslint .` and expect exit status 0. |

## Check-integrity guards

| Guard | Setting | Rule |
|---|---|---|
| Both hook rules stay errors | `react-hooks/rules-of-hooks` and `react-hooks/exhaustive-deps` set to `"error"` after the preset | USS-QLT-006 |
| Disable comments that suppress nothing are reported as errors | the ESLint profile's `linterOptions.reportUnusedDisableDirectives: "error"` and `linterOptions.reportUnusedInlineConfigs: "error"` | USS-QLT-012 |
| A directive comment needs a description | the ESLint profile's `@eslint-community/eslint-comments/require-description` | USS-QLT-009 |
| A directive comment cannot disable every rule | the ESLint profile's `@eslint-community/eslint-comments/no-unlimited-disable` | USS-QLT-010 |
