# EU Cyber Resilience Act

- **Prefix:** CRA
- **Kind:** module
- **Applies when:** sold-in-eu

This module covers Regulation (EU) 2024/2847 of 23 October 2024 (the Cyber
Resilience Act), published in the Official Journal of the European Union on
20 November 2024. Article 14 (the reporting obligations of manufacturers)
applies from 11 September 2026, and the rest of the Regulation applies from
11 December 2027 (Article 71(2)). A product placed on the market before
11 December 2027 is subject to the Regulation only if it is substantially
modified from that date, except that Article 14 applies to every such product
(Article 69(2) and (3)). The article and annex references were read in the
Official Journal text on 2026-11-24. The module maps the obligations of the
manufacturer to USS rules and adds nine rules for what the core rules leave
open. The role of the open-source software steward and the other special
regimes of the Regulation are not mapped here: a qualified person judges them.
The core rules that the module does not name apply as written.

## Disclaimer

USS compliance is not legal compliance, and this module is not legal advice.
The module shows where USS helps meet the Regulation and what is left. A
qualified person confirms each row, and what is left, for the product.

## Obligations

| Obligation | Source | Rules | Coverage |
|---|---|---|---|
| Design to a risk assessment | Art. 13(2)–(4); Annex I Part I(1) | USS-SEC-001 | partial: the risk assessment in the technical documentation |
| No known exploitable vulnerabilities | Annex I Part I(2)(a) | USS-DEP-007, USS-DEP-008 | partial: the scan covers known vulnerabilities in dependencies, not in the product's own code |
| Secure by default, with a reset | Annex I Part I(2)(b) | USS-SEC-002 | partial: reset to the original state |
| Automatic security updates | Annex I Part I(2)(c) | USS-CRA-008 | partial: where automatic updates do not fit |
| Access control | Annex I Part I(2)(d) | USS-SEC-015, USS-SEC-018 | partial: reporting on possible unauthorised access |
| Confidentiality | Annex I Part I(2)(e) | USS-SEC-022, USS-DAT-004 | partial: data that is neither sensitive nor sent over a network |
| Integrity | Annex I Part I(2)(f) | USS-SEC-009, USS-SUP-014 | partial: integrity of stored data and configuration, and reports on corruption |
| Data minimisation | Annex I Part I(2)(g) | USS-PRV-002 | partial: data that is not personal |
| Availability and resilience | Annex I Part I(2)(h)–(i) | USS-RES-004 | partial: availability after an incident and the effect on other devices or networks |
| Limited attack surface | Annex I Part I(2)(j) | USS-SEC-003 | partial: external interfaces |
| Exploit mitigation | Annex I Part I(2)(k) | USS-SAF-001, USS-SAF-002 | partial: mitigation outside native code |
| Security monitoring | Annex I Part I(2)(l) | USS-AUD-001 | partial: the opt-out for the user |
| Remove data and settings | Annex I Part I(2)(m) | USS-CRA-009 | full |
| SBOM | Annex I Part II(1) | USS-CRA-001, USS-SUP-013 | full |
| Remediate without delay; separate security updates | Annex I Part II(2) | USS-DEP-008, USS-CRA-006 | partial: vulnerabilities outside dependencies, and the delay |
| Tests and reviews | Annex I Part II(3) | USS-TST-001, USS-SEC-028 | partial: regular security reviews |
| Disclose fixed vulnerabilities | Annex I Part II(4) | USS-SEC-031 | partial: the description, impact, severity and remediation information |
| Coordinated vulnerability disclosure | Annex I Part II(5)–(6) | USS-SEC-029, USS-SEC-030 | partial: enforcing the policy, and sharing information about third-party components |
| Secure distribution of updates | Annex I Part II(7) | USS-SUP-014, USS-SEC-022 | partial: automatic distribution of security updates |
| Free security updates with advisories | Annex I Part II(8) | USS-CRA-005, USS-SEC-031 | full |
| Support period | Art. 13(8), (19) | USS-CRA-004, USS-REL-004 | partial: the notice at the end of support, and the reasoning in the technical documentation |
| Updates kept available | Art. 13(9) | USS-CRA-007 | full |
| Report exploited vulnerabilities | Art. 14(1)–(2) | USS-CRA-002 | full |
| Report severe incidents | Art. 14(3)–(4) | USS-CRA-003 | full |
| Information for users | Annex II | USS-CRA-004, USS-SEC-029, USS-DOC-001 | partial: the other items of Annex II |
| Technical documentation | Art. 31; Annex VII | none | none: a qualified person prepares it |
| Conformity assessment, EU declaration, CE marking | Arts. 28, 30, 32 | none | none: a qualified person handles them |

## Release

### USS-CRA-001: Draw up an SBOM for each release

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** Each release has an SBOM in SPDX or CycloneDX format, which is a
  machine-readable format. The SBOM lists at least the top-level dependencies
  of the release: each dependency that the project declares itself, and not
  only the ones that those dependencies bring in. A release without an SBOM, an
  SBOM in another format, or an SBOM that omits a declared dependency fails. For
  the releases since the last audit, or in the last 12 months when there is
  none, check each one.
- **References:** Regulation (EU) 2024/2847, Official Journal L, 20.11.2024,
  Annex I Part II(1) (read 2026-11-24); SPDX 3.0.1; CycloneDX 1.7

Each release MUST have an SBOM in a commonly used, machine-readable format
(SPDX or CycloneDX) that covers at least the top-level dependencies.

**Rationale:** The Regulation makes the SBOM a duty of the manufacturer, where
USS-SUP-013 only recommends one. The SBOM lets the manufacturer find the
components that a newly disclosed vulnerability touches.

## Reporting

### USS-CRA-002: Report actively exploited vulnerabilities

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** The project has a written reporting procedure that the auditor can
  read. It names the single reporting platform (Article 16) as the route, and
  gives each of the three submissions with its deadline: the early warning
  within 24 hours of becoming aware of the vulnerability, the vulnerability
  notification within 72 hours of becoming aware of it, and the final report no
  later than 14 days after a corrective or mitigating measure is available. A
  procedure without the platform or with any of the three missing, or with a
  later deadline, fails. A reportable event is an actively exploited
  vulnerability in the product that the project became aware of since the last
  audit, or in the last 12 months when there is none. For each such event, the
  project's records (the platform's submission receipts or the project's copies
  of them) show each submission, and each is within its deadline, counted from
  the time the project's records give for becoming aware of it. An event with a
  missing or late submission fails. When the procedure exists and the project's
  records show no such event in that period, the rule passes on the procedure,
  and the auditor records "no reportable event" as the evidence. Where the
  auditor cannot tell whether an event happened or when the project became
  aware of it, the outcome is `unverified`.
- **References:** Regulation (EU) 2024/2847, Official Journal L, 20.11.2024,
  Article 14(1) and (2), Article 16(1) and Article 71(2) (read 2026-11-24)

A written procedure MUST report each actively exploited vulnerability in the
product through the single reporting platform within the deadlines of
Article 14: an early warning within 24 hours of becoming aware of it, a
vulnerability notification within 72 hours of becoming aware of it and a final
report no later than 14 days after a corrective or mitigating measure is
available.

**Rationale:** The deadlines run from the moment the manufacturer becomes
aware, so a team that has no procedure in place cannot meet them. The
Regulation applies Article 14 from 11 September 2026.

### USS-CRA-003: Report severe incidents

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** The project has a written reporting procedure that the auditor can
  read. It names the single reporting platform (Article 16) as the route, and
  gives each of the three submissions with its deadline: the early warning
  within 24 hours of becoming aware of the incident, the incident notification
  within 72 hours of becoming aware of it, and the final report within one month
  after the incident notification was submitted. A procedure without the
  platform or with any of the three missing, or with a later deadline, fails.
  A reportable event is a severe incident having an impact on the security of
  the product, which the project became aware of since the last audit, or in
  the last 12 months when there is none. For each such event, the project's
  records show each submission, and each is within its deadline, counted from
  the time the project's records give for becoming aware of it (or, for the
  final report, from the incident notification). An event with a missing or
  late submission fails. When the procedure exists and the project's records
  show no such event in that period, the rule passes on the procedure, and the
  auditor records "no reportable event" as the evidence. Where the auditor
  cannot tell whether an event happened or when the project became aware of it,
  the outcome is `unverified`.
- **References:** Regulation (EU) 2024/2847, Official Journal L, 20.11.2024,
  Article 14(3), (4) and (5), Article 16(1) and Article 71(2) (read
  2026-11-24)

A written procedure MUST report each severe incident having an impact on the
security of the product through the single reporting platform within the
deadlines of Article 14: an early warning within 24 hours of becoming aware of
it, an incident notification within 72 hours of becoming aware of it and a final
report within one month after the incident notification.

**Rationale:** Article 14(5) calls an incident severe where it harms the
availability, authenticity, integrity or confidentiality of sensitive or
important data or functions, or has led or can lead to malicious code running
in the product or in a user's systems.

## Support and updates

### USS-CRA-004: State the support period

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** The product information (such as the README, the documentation,
  the download or store page, or the packaging) states the end date of the
  support period, with at least the month and the year. A support period with
  no end date, or an end date with no month, fails. The period from the date
  the product was placed on the market to the end date is at least five years,
  or the product information states an expected use time shorter than five
  years and the period corresponds to that time. Anything shorter fails.
- **References:** Regulation (EU) 2024/2847, Official Journal L, 20.11.2024,
  Article 13(8) and (19) (read 2026-11-24)

The product information MUST state the end date of the support period, with at
least the month and the year, and the support period MUST be at least five
years unless the product is expected to be in use for less than five years, in
which case it corresponds to the expected use time.

**Rationale:** Users plan their own upgrades from the end date. USS-REL-004
asks for a support policy but sets no minimum period.

### USS-CRA-005: Provide security updates free of charge

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** Each security update that the project made available during the
  support period, since the last audit or in the last 12 months when there is
  none, was disseminated without delay, was free of charge to the users, and
  came with an advisory message that gives users the relevant information,
  including any action they can take. An update behind a payment, a licence
  upgrade or a subscription tier fails. A tailor-made product, whose security
  updates are agreed in a contract between the manufacturer and a business
  user, is judged by that agreement.
- **References:** Regulation (EU) 2024/2847, Official Journal L, 20.11.2024,
  Annex I Part II(8) (read 2026-11-24)

Security updates during the support period MUST be disseminated without delay,
free of charge, and accompanied by advisory messages that give users the
relevant information, including on any action they can take.

**Rationale:** A security fix that costs money leaves the users who do not pay
exposed, and the exposed installations can be attacked.

### USS-CRA-006: Ship security fixes apart from new features

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** For each security fix released since the last audit, or in the
  last 12 months when there is none, there is a release that has the fix and
  adds no new features (such as a patch release of the supported version). A
  fix that is only in a release that adds features fails, unless the project's
  records show that a separate release was not technically feasible, and give
  the reason.
- **References:** Regulation (EU) 2024/2847, Official Journal L, 20.11.2024,
  Annex I Part II(2) (read 2026-11-24)

Where it is technically feasible, each security fix MUST be available in a
release that adds no new features.

**Rationale:** Users who cannot accept a feature change, or its risk, still
need the fix.

### USS-CRA-007: Keep each security update available

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** Each security update that the project made available during the
  support period is still available for download or installation, from where it
  was published, for the longer of 10 years after it was issued and the
  remainder of the support period. For the security updates of the releases
  since the last audit, or in the last 12 months when there is none, check each
  one at its published location. A security update that is gone, or a
  retention rule or clean-up job that removes one earlier than that, fails.
- **References:** Regulation (EU) 2024/2847, Official Journal L, 20.11.2024,
  Article 13(9) (read 2026-11-24)

Each security update that was made available to users during the support period
MUST remain available after it was issued for at least 10 years or for the
remainder of the support period, whichever is longer.

**Rationale:** A user who falls behind can still install the fix, instead of
rebuilding it or running an exposed version.

### USS-CRA-008: Install security updates automatically by default

- **Level:** MUST
- **Applies when:** has-ui or has-cli
- **Category:** Safety
- **Verify:** Where automatic security updates apply to the product, the default
  configuration installs them automatically, within an appropriate timeframe.
  The product has a clear and easy-to-use way to turn this off, tells users
  that an update is available, and lets them postpone an update temporarily. A
  default that does not install security updates, or any of the three
  user-facing parts missing, fails. A product for which the project's technical
  documentation gives the reason that automatic updates do not apply to it
  passes this rule on that reason.
- **References:** Regulation (EU) 2024/2847, Official Journal L, 20.11.2024,
  Annex I Part I(2)(c) (read 2026-11-24)

Where automatic security updates apply to the product, they MUST be on by
default, with a clear opt-out, a notice to users that an update is available,
and a way to postpone an update temporarily.

**Rationale:** Most users never change a default, and a security update that
waits for the user often never arrives.

## Data

### USS-CRA-009: Let users remove all their data and settings

- **Level:** MUST
- **Applies when:** stores-persistent-data
- **Category:** Safety
- **Verify:** The product gives users a way (a command, a menu item, an
  uninstaller option or a documented procedure) that removes all the data and
  settings that the product stores, on a permanent basis, with no recoverable
  copy left by the product itself. The way is secure and easy for a user to
  find and run. Where the data can be transferred to other products or systems,
  the transfer is done in a secure manner. A product that leaves data or
  settings behind, or that has no such way, fails.
- **References:** Regulation (EU) 2024/2847, Official Journal L, 20.11.2024,
  Annex I Part I(2)(m) (read 2026-11-24)

Users MUST be able to remove all their data and settings permanently, securely
and easily and, where the data can be transferred to other products or systems,
the transfer MUST be done in a secure manner.

**Rationale:** A product that is sold, handed on or retired keeps its user's
data unless the user can remove it.
