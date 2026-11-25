---
description: Audit an existing project against the Universal Software Standard (USS). Also runs release audits and version upgrades.
argument-hint: "[release | upgrade <version> | headless]"
---

# Audit with USS

Read `${CLAUDE_PLUGIN_ROOT}/AGENTS.md`.

Arguments: $ARGUMENTS

Choose the workflow in `AGENTS.md` from the arguments:

- No argument: follow "Audit an existing project".
- `release`: follow "Release audit".
- `upgrade <version>`: follow "Upgrade the pinned version" for that version.
  If the developer gave no version, ask for it.
- `headless`: no person is present. Follow "Audit an existing project". Use the
  rules for no person present in "Release audit".

Each workflow starts with "Before every workflow". Ask each question as
"Questions to the developer" says.
Do not copy the workflow steps into your answer.
Follow "Agent conduct" in `AGENTS.md`.
