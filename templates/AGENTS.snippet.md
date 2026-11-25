## Universal Software Standard (USS)

This project follows the Universal Software Standard (USS). The version of USS
is the version that `.uss/record.yaml` pins.

- Before you change the project, run `uss applicable --full .uss/record.yaml`.
- Follow every applicable MUST and MUST NOT rule. A Standard Deviation from
  one makes the project not compliant.
- Follow every applicable SHOULD and SHOULD NOT rule. Skip one only if the
  record lists a Standard Deviation for it.
- Do not change `facts`, `decisions` or `deviations` in the record without the
  developer.
- To audit the project, follow "Audit an existing project" in the output of
  `uss show AGENTS.md`.

### Agent conduct

- Do not reduce the scope of a task without telling the developer.
- Report only results that you saw from commands that you ran. The result in
  CI decides whether a check passes.
- Do not change files, settings or systems outside the task.
- Do not turn off a check or skip a test to make it pass.
