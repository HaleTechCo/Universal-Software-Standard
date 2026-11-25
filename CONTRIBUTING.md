# Contributing to USS

Thank you for helping. This guide tells you how to set up, change the
repository and get a change merged. The Universal Software Standard (USS) is
a set of rules in `standard/`, with a tool, `uss`, in `tools/uss`.

Follow the [Code of Conduct](CODE_OF_CONDUCT.md) in every space of the
project. Read [GOVERNANCE.md](GOVERNANCE.md) to learn who decides.

## Set up

1. Install rustup from <https://rustup.rs>.
2. Clone the repository. Run all commands from its root.
3. Run `rustup toolchain install`. It installs the toolchain that
   `rust-toolchain.toml` pins.
4. Run `git config core.hooksPath .githooks`. It turns on the pre-commit hook.
5. Run `cargo xtask check`. The first run also installs cargo-deny under
   `.tools`.

The pre-commit hook runs `cargo fmt --all --check` and the `uss` lint.

## Make a change

Issues that suit a first contribution carry the label `good first issue`.
None is open at present (2026-11-25).

1. Open an issue first for a large change. Use the bug or rule template.
2. Create a branch from `main`.
3. Make the change. Keep each pull request to one subject.
4. Run the checks that the next section lists.
5. Write each commit message as `type(scope): summary`, as in `git log`.
6. Push the branch. Open a pull request. Fill in its template.

## Run the checks

- `cargo xtask check` sets up and runs every check that CI runs.
- `cargo run --locked -p uss -- lint` checks the standard files.
- `cargo test --locked -p uss` runs the tests of `uss`.

How long `cargo xtask check` takes, on Windows 11 (x86-64):

- Warm run, with the build cache present: about 9 seconds.
- Cold run, after `cargo clean`: about 140 seconds (2 minutes 17 seconds).
  The cargo-deny tool was already under `.tools`. Without it, the first run
  also downloads and builds it.

Each time comes from one run, measured with the `time` command of a POSIX
shell. Other machines give other times. A run in CI takes longer, because it
starts with no build cache.

## What makes a change acceptable

A change is done when it meets the Definition of Done in
[AGENTS.md](AGENTS.md#definition-of-done). That list is the one source. The
pull request template repeats it as a checklist.

Also follow these rules:

- Write steps in the ASD-STE100 style (USS-DOC-007).
- Do not reuse or renumber a rule ID. See section 4 of
  [`standard/conventions.md`](standard/conventions.md).
- Keep the version at 0.1.0 until the first release.

## Add to the standard

`standard/conventions.md` tells you how. Do not copy its text into a change.

| To add or change    | Read in `standard/conventions.md`                       |
| ------------------- | ------------------------------------------------------- |
| A rule              | Section 3 (format) and section 7 (writing rules)        |
| A rule ID           | Section 4 (IDs, and how to retire a rule)               |
| An area             | Section 2 (standard files)                              |
| A module            | Section 2 (standard files) and section 7 (module rules) |
| A vendor profile    | Section 12 (vendor profiles)                            |
| A regulation module | Section 13 (regulation modules)                         |

Read each source that a rule cites before you add or change the reference.
Run `cargo run --locked -p uss -- lint` after each change.

## Vendor profiles

A vendor profile says how one checking tool meets the rules. The vendor of the
tool may send and maintain its own profile. The profile lives in
`standard/vendors/<vendor>/`. `.github/CODEOWNERS` assigns that directory to
the maintainer, so the maintainer reviews each profile change.

## Add a dependency

Check the health of a dependency before you add it (USS-DEP-012). Put the
result in the description of the pull request that adds it. Answer these four
points:

1. Is the dependency maintained?
2. Is it deprecated or archived?
3. Does it have more than one maintainer?
4. Is the standard library enough instead?

Give a reason for each point that the dependency fails. Check the licence too
(USS-LIC-003). `cargo xtask check` runs cargo-deny for the licence check.

Dependabot opens pull requests for updates. It waits some days after a
release before it does. The `cooldown` settings in `.github/dependabot.yml`
give the number of days.

## Review

- The maintainer reviews each pull request.
- A change needs the approval of the maintainer, and the checks must pass,
  before it merges. See [GOVERNANCE.md](GOVERNANCE.md).
- You get a first response within 7 days. If you hear nothing after 7 days,
  comment on the pull request to ask.
- Report a security problem as [SECURITY.md](SECURITY.md) says. Do not open a
  public issue for it.

## Credit

The maintainer names each outside contributor in the `CHANGELOG.md` entry of
the release that holds the change.

## Licence

USS is under the MIT licence (see [LICENSE](LICENSE)). Your contribution is
under the same licence. USS asks for no other agreement.
