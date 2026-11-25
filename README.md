# Universal Software Standard (USS)

USS is an engineering standard for all software. Its rules use the RFC 2119
levels (MUST, SHOULD, MAY). If a project follows every rule that applies to
it, the project is as good as software can be made.

The `uss` tool calculates which rules apply, the scores and compliance. An
agent does the rest. It follows the workflows in [AGENTS.md](AGENTS.md).

- **Adopt**: for a new project. The agent profiles the project, asks about
  each applicable SHOULD, SHOULD NOT and MAY rule, and records the answers.
  Then it builds to the result.
- **Audit**: for an existing project. The agent scans the code, asks the same
  questions, audits each applicable rule, writes a report and fixes the code.

Status: version 0.x, in development. No version is released yet. See the
[design spec](docs/superpowers/specs/2026-11-21-universal-software-standard-design.md).

## Start

Use the Claude plugin, or tell any agent to follow the workflow in
`AGENTS.md`:

- `/uss:adopt` adopts USS in a new project.
- `/uss:audit` audits an existing project.
- `/uss:audit release` audits a release.
- `/uss:audit upgrade <version>` moves a project to another USS version.
- `/uss:audit headless` audits with no person present.

Without the plugin, say "follow the workflow in AGENTS.md to adopt USS" or
"... to audit this project".

You can also run the tool. This command lists the rules that apply to a
project:

```sh
uss applicable .uss/record.yaml
```

See [docs/cli.md](docs/cli.md) for each command.

## Try USS before its first release

Use these steps to try USS on one of your projects from a local checkout of
this repository. They work on Windows, Linux and macOS.

1. Install Rust with rustup from <https://rustup.rs>.
2. In the USS checkout, install the tool:

   ```sh
   cargo install --locked --path tools/uss
   ```

3. In Claude Code, add the checkout as a marketplace. Use the path of the
   checkout:

   ```text
   /plugin marketplace add <path of the checkout>
   ```

4. Install the plugin:

   ```text
   /plugin install uss@universal-software-standard
   ```

5. Open the other project in Claude Code. Run `/uss:audit`.

After you change USS, do these steps:

1. In the USS checkout, install the tool again:

   ```sh
   cargo install --locked --path tools/uss --force
   ```

   The binary carries the standard and the workflows. A change reaches it only
   through a new install.
2. In Claude Code, run `/reload-plugins`. It loads the changed skills.

If a step fails, see [docs/troubleshooting.md](docs/troubleshooting.md).

## Install after the first release

Each method below installs the tool `uss`. Replace `0.1.0` with the version
that you need.

### Install the plugin

1. Install the tool, as described below.
2. In Claude Code, run this command (Claude Code 2.1.275 or later):

   ```text
   /plugin install uss --marketplace HaleTechCo/Universal-Software-Standard
   ```

### Install the tool from a release tag

1. Install Rust with rustup from <https://rustup.rs>.
2. Run this command:

   ```sh
   cargo install --locked --git https://github.com/HaleTechCo/Universal-Software-Standard --tag v0.1.0 uss
   ```

3. Run `uss version`. It prints the version of the standard.

### Upgrade

1. Read [CHANGELOG.md](CHANGELOG.md) for the changes since your version.
2. Install the new tag over the old version:

   ```sh
   cargo install --locked --force --git https://github.com/HaleTechCo/Universal-Software-Standard --tag v0.2.0 uss
   ```

3. Update the plugin. Run this command in Claude Code:

   ```text
   /plugin marketplace update universal-software-standard
   ```

4. To move a project to the new version, run `/uss:audit upgrade 0.2.0` in the
   project.

### Roll back

1. Install the earlier tag over the new version:

   ```sh
   cargo install --locked --force --git https://github.com/HaleTechCo/Universal-Software-Standard --tag v0.1.0 uss
   ```

2. If you moved a project to the new version, restore its `.uss` directory
   from version control. The record pins the version that it was written for.
3. Run `uss version`. Check that it prints the earlier version.
4. Keep the plugin as it is. It has no version of its own: its skills follow
   the workflows of the installed `uss`, which `uss show AGENTS.md` prints.

### Uninstall

1. Run `cargo uninstall uss`.
2. In Claude Code, run `/plugin uninstall uss@universal-software-standard`.
3. Run `/plugin marketplace remove universal-software-standard`.

USS keeps no other files on your machine. The `.uss` directory of a project
belongs to that project. Delete it only if you want to drop the record and the
audit history.

## Development

See [AGENTS.md](AGENTS.md), "Work on this repository", and
[CONTRIBUTING.md](CONTRIBUTING.md). Run `cargo xtask check` to do all checks.

More documents:

- [docs/cli.md](docs/cli.md): the commands, options and exit codes.
- [docs/architecture.md](docs/architecture.md): how the tool is built.
- [docs/dependencies.md](docs/dependencies.md): why each dependency exists.
- [docs/troubleshooting.md](docs/troubleshooting.md): common problems.
- [SECURITY.md](SECURITY.md): how to report a vulnerability.
- [CHANGELOG.md](CHANGELOG.md): the changes in each release.

USS is licensed under the [MIT licence](LICENSE).
