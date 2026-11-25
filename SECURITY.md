# Security policy

## Report a vulnerability

Report a vulnerability privately. Do not open a public issue.

1. Open the repository on GitHub:
   <https://github.com/HaleTechCo/Universal-Software-Standard>.
2. Select the "Security" tab.
3. Select "Report a vulnerability". This is GitHub private vulnerability
   reporting.
4. Describe the problem. Give the affected version, the steps to reproduce it
   and its effect.

If the "Report a vulnerability" button is missing, the maintainers have not
yet turned the feature on. Open a public issue that asks for a private
channel. Do not put details of the vulnerability in that issue.

## What happens after a report

- The maintainers acknowledge the report within 7 days.
- They assess it within 14 days. They tell you if they accept it as a
  vulnerability, and what they plan to do.
- They fix or mitigate it within 90 days.
- A fix comes with a GitHub security advisory. The advisory names the
  affected and fixed versions and a CVE or other public ID.
- They credit you in the advisory, unless you ask them not to.

These are target times. The maintainers tell you if they cannot meet one.

## Supported versions

| Version                | Fixes and security fixes | Support ends                       |
| ---------------------- | ------------------------ | ---------------------------------- |
| The latest 0.x release | Yes                      | When the next release is published |
| Any earlier release    | No                       | Ended                              |

Before 1.0, only the latest 0.x release gets fixes. To get a fix, upgrade to
the latest release. See "Upgrade" in the [README](README.md).

## Scope

This policy covers the `uss` tool, the standard, the templates and the Claude
plugin in this repository.
