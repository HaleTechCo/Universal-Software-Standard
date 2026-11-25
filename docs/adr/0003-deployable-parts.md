# 0003: Ship the standard inside `uss`, and the workflows as a Claude plugin

* Status: accepted
* Date: 2026-11-25

## Context and Problem Statement

USS reaches a project as three things: the standard's files, the `uss` tool
that calculates applicability, scores and compliance, and the workflows in
`AGENTS.md` that an agent follows. A project pins one USS version in its
record. The agent must read the standard and the workflows of that version,
and `uss` must calculate with the same version. The design spec left open how
a skill gets the pinned version when it differs from the installed one (spec
section 12). How should USS split these things into deployable parts?

This record was written after the decision. The decision is in the design
spec (sections 8 and 9), in the foundation plan (the embedded standard) and in
decisions 1 and 6 of the plan for the workflows and skills, all under
`docs/superpowers/`.

## Decision Drivers

* An agent reads the standard and the workflows of the version that the
  record pins.
* Only `uss` calculates scores and compliance (conventions section 11).
* The workflows are written once, in `AGENTS.md`, for any agent (spec
  section 8).
* One install gives a set of files that match, and an upgrade is one install
  over the old one.

## Considered Options

* One `uss` binary that embeds `standard/`, `schema/`, `templates/`,
  `AGENTS.md` and `CHANGELOG.md`, so that the tool and the standard have one
  version. A Claude plugin at the repository root holds two skills that send
  the agent to `AGENTS.md`.
* A `uss` binary without the standard. The agent clones USS at the pinned tag,
  and `uss` reads that checkout (the `--standard-root` option of the
  foundation plan).
* No tool for adopters. The agent reads the Markdown of the pinned tag and
  works without `uss`, as spec section 9 first allowed.
* Skills that carry their own copy of the workflows and of the standard.

## Decision Outcome

Chosen option: one binary with the standard and the workflows embedded, plus
the plugin. The version of `uss` is then the version of the standard and of
the workflows. An agent installs the `uss` release of the pinned version, and
reads the standard and the workflows through `uss show` and
`uss applicable --full`.

The other options were rejected:

* A checkout at the pinned tag needs a clone and a tool of the same version,
  so it is two installs that can disagree. `--standard-root` stays for work on
  the standard itself.
* Without the tool, an agent would calculate scores by hand, which the
  conventions forbid.
* A copy in the skills drifts from `AGENTS.md`, and spec section 8 forbids it.

### Consequences

* A change to the standard reaches users only through a new build of `uss`.
* The plugin's version is the version of the standard (a test ties
  `plugin.json` to it). Its skills read `AGENTS.md` at the plugin root, which
  tells the agent to follow the workflows of the installed `uss`
  (`uss show AGENTS.md`).
* The crate embeds files outside its own directory, so it cannot go to
  crates.io (`publish = false`). Users install it with `cargo install --git`
  from a release tag. Release binaries are a later step.
* The library in `tools/uss` is not a separate deployable part. It does no
  I/O, so a later extension can embed it (ADR 0001).
* The plugin is the repository root (`"source": "./"` in
  `.claude-plugin/marketplace.json`), so a Git marketplace copies the tracked
  files of the whole repository.
