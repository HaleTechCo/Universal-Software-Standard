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

- Fix, option 2: read the checkout without an install. Give its path in
  place of `<checkout>`:

  ```sh
  uss --standard-root <checkout> version
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

- Cause: the Rust toolchain is missing or too old. The workspace needs Rust
  1.100 or later.
- Fix: run `rustup update`. Run the install command again.

## The skills `/uss:adopt` and `/uss:audit` do not appear

1. Run `/reload-plugins`.
2. Run `/plugin marketplace list`. Check that `universal-software-standard`
   is in the list.
3. Run `/plugin install uss@universal-software-standard`.
4. Type `/uss:`. The two skills must appear.
5. If they do not, run `claude plugin validate .` in the checkout. Fix each
   error that it reports.
