# Troubleshooting

Each entry gives a problem, its cause and its fix.

## The record pins another version

The message is `record pins USS X; this uss carries USS Y`.

- Cause: the record in `.uss/record.yaml` names version X, and the installed
  `uss` carries version Y.
- Fix:
  1. Install `uss` at version X:

     ```sh
     cargo install --locked --force --git https://github.com/HaleTechCo/Universal-Software-Standard --tag v<X> uss
     ```

  2. Run `uss version`. Check that it prints X.
  3. To move the project to version Y instead, run `/uss:audit upgrade Y`.

## The standard changed, but `uss` shows the old one

- Cause: the binary carries a copy of the standard. A change in a checkout
  does not reach it.
- Fix, option 1: install again from the checkout.

  ```sh
  cargo install --locked --path tools/uss --force
  ```

- Fix, option 2: read the checkout without an install. Give
  `--standard-root <checkout>` to each `uss` command. Give the path of the
  checkout in place of `<checkout>`. For example:

  ```sh
  uss --standard-root <checkout> version
  uss --standard-root <checkout> check .uss/record.yaml
  ```

## `uss check` reports problems

- Cause: the record or the project's files break a check. Each line names the
  file and the problem.
- Fix:
  1. Read the first problem. Fix it in the record or in the file that it
     names.
  2. Run `uss check .uss/record.yaml` again.
  3. Repeat until the last line says "ok".
- If a file is not found, run the command from the project root, or give the
  path of the record.

## `uss release-check` fails in a shallow clone

The message is `audited commit <sha> is not in this clone; the clone is
shallow`.

- Cause: the clone has only part of the history, so it lacks the audited
  commit.
- Fix:
  1. Run `git fetch --unshallow`.
  2. In GitHub Actions, set `fetch-depth: 0` on `actions/checkout`.
  3. Run `uss release-check` again.

## `uss release-check` reports changed files

- Cause: a file outside `.uss` changed after the audited commit.
- Fix: audit the release commit again. Commit the results. Publish the
  release from that commit.

## `cargo install` fails

- Cause: the Rust toolchain is missing or too old. `uss` needs Rust 1.100 or
  later.
- Cause: the message says that `uss` requires rustc 1.100 or newer, but the
  default toolchain is newer. A `rust-toolchain.toml` file in the current
  directory, or in a directory above it, selects an older toolchain. It
  overrides the default toolchain.
- Fix:
  1. Run `rustup toolchain install stable`. It installs the stable toolchain,
     or updates it.
  2. Run the install command again with `+stable` after `cargo`. The
     toolchain that you give in the command overrides `rust-toolchain.toml`.
     For example:

     ```sh
     cargo +stable install --locked --git https://github.com/HaleTechCo/Universal-Software-Standard --tag v<version> uss
     ```

  3. In GitHub Actions, give the toolchain in the same way. The template
     `templates/ci/uss-release.yml` does this with `USS_RUST_TOOLCHAIN`.

## `uss` says that a file is not valid UTF-8

The message is `<file>: /: not valid UTF-8: ...`.

- Cause: the file has another encoding. In Windows PowerShell 5.1, a
  redirection (`>`) writes UTF-16.
- Fix:
  1. Open the file in an editor. Save it with the encoding UTF-8.
  2. To copy a template in Windows PowerShell 5.1, use `Out-File`:

     ```powershell
     uss show templates/record.yaml | Out-File -Encoding utf8 .uss/record.yaml
     ```

  3. Run `uss check .uss/record.yaml` again.

## The skills `/uss:adopt` and `/uss:audit` do not appear

- Cause: Claude Code has not loaded the plugin. The marketplace or the plugin
  is missing, Claude Code did not reload, or the plugin has an error.
- Fix:
  1. Run `/reload-plugins`.
  2. Run `/plugin marketplace list`. Check that `universal-software-standard`
     is in the list.
  3. Run `/plugin install uss@universal-software-standard`.
  4. Type `/uss:`. The two skills must appear.
  5. If they do not, run `claude plugin validate .` in the checkout. Fix each
     error that it reports.
