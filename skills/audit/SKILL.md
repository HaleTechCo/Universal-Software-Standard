---
description: Audit an existing project against the Universal Software Standard (USS). Also runs release audits and version upgrades.
argument-hint: "[release | upgrade <version> | headless]"
---

# Audit with USS

Read `${CLAUDE_PLUGIN_ROOT}/AGENTS.md`.

Start at "Before every workflow". Follow that section and "Questions to the
developer" for the whole workflow.

Arguments: $ARGUMENTS

Choose the workflow from the arguments:

- No argument: follow "Audit an existing project".
- `release`: follow "Release audit".
- `upgrade <version>`: follow "Upgrade the pinned version" for that version.
- `headless`: no person is present. Follow "Audit an existing project". Use the
  rules for no person present in "Release audit".

Do not copy the workflow steps into your answer.
Follow "Agent conduct" in the same file.
