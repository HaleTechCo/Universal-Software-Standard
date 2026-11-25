# GDPR

- **Prefix:** GDPR
- **Kind:** module
- **Applies when:** processes-eu-personal-data

This module covers Regulation (EU) 2016/679 of 27 April 2016, the General Data
Protection Regulation, published in the Official Journal of the European Union
on 4 May 2016 (L 119/1). It applies from 25 May 2018 (Article 99(2)). The
Regulation binds the controller, who decides why and how personal data is
processed, and the processor, who processes it for the controller. Who is the
controller of a given processing is for a qualified person to judge. For
software that others run, these rules ask that the software lets its operator
meet each right that the Regulation gives to people; the operator remains
responsible for the law. The article references were read in the Official
Journal text on 2026-11-24. The module maps the obligations of the Regulation
to USS rules and adds seven rules for what the core rules leave open. The core
rules that the module does not name apply as written.

## Disclaimer

USS compliance is not legal compliance, and this module is not legal advice.
The module shows where USS helps meet the Regulation and what is left. A
qualified person confirms each row, and what is left, for the project.

## Obligations

| Obligation | Source | Rules | Coverage |
|---|---|---|---|
| Lawful basis for processing | Art. 6 | none | none: a qualified person chooses and records the lawful basis of each processing |
| Consent | Art. 7(1) and (3) | USS-GDPR-004, USS-GDPR-005 | partial: the form of the request for consent (Art. 7(2)), whether consent is freely given (Art. 7(4)) and the conditions for a child's consent (Art. 8) |
| Information to people | Arts. 13–14 | none | none: a qualified person writes the privacy notice |
| Access and portability | Arts. 15, 20 | USS-GDPR-001 | partial: the further information that Art. 15(1) and (2) lists, and the direct transmission to another controller (Art. 20(2)) |
| Rectification | Art. 16 | USS-GDPR-002 | partial: the time in which the controller acts, and telling each recipient (Art. 19) |
| Erasure | Art. 17 | USS-GDPR-003, USS-DAT-005 | partial: informing other controllers of the erasure (Art. 17(2)), and telling each recipient (Art. 19) |
| Data protection by design and by default | Art. 25(1) and (2) | USS-PRV-002, USS-PRV-005, USS-SEC-002 | partial: the other measures of Art. 25(1), such as pseudonymisation, and the limits on storage period and accessibility of Art. 25(2) |
| Records of processing | Art. 30(1)–(4) | USS-GDPR-007, USS-PRV-001 | partial: the content of the record that the project assembles, and making it available to the supervisory authority on request |
| Security of processing | Art. 32(1) | USS-SEC-022, USS-DAT-004 | partial: pseudonymisation, resilience, restoring availability and regular testing of the measures |
| Breach notification | Arts. 33–34 | USS-GDPR-006 | partial: the content of the notification, the documentation of each breach, and the processor's notice to the controller |
| Impact assessment | Art. 35 | none | none: a qualified person judges whether an assessment is needed and carries it out |
| Transfers to third countries | Chapter V (Arts. 44–49) | none | none: a qualified person chooses the transfer mechanism |

## Rights of people

### USS-GDPR-001: Give each person a copy of their data

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** The software has an operation, a command or a documented
  procedure that exports the personal data about one person. Run it on a test
  person with data in each store (the primary database, the files and the
  search indexes) and compare the result with the stores: every item of
  personal data about that person is in the export, except data that would
  adversely affect the rights and freedoms of others. The export is in a
  structured, commonly used and machine-readable format (such as JSON, CSV or
  XML), and not in a format that only the software can read, or only an image
  or a PDF of text. The operator can run it. An export
  that omits an item, or is in another format, fails.
- **References:** Regulation (EU) 2016/679, Official Journal L 119, 4.5.2016,
  Articles 15(3) and (4) and 20(1) and (4) (read 2026-11-24)

The software MUST let its operator export all personal data about one person,
when the person asks, in a structured, commonly used and machine-readable
format, except where the export would adversely affect the rights and freedoms
of others.

**Rationale:** Article 15 gives each person a copy of their personal data (in
"a commonly used electronic form"), and Article 20 the right to receive the
data that they provided in a structured, commonly used and machine-readable
format. Applying the Article 20 format to all personal data, so that one export
serves both rights, is USS's reading, for a qualified person to confirm.

### USS-GDPR-002: Let personal data be corrected

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** The software has an operation, a screen, a command or a
  documented procedure that corrects, or completes, any item of personal data
  about one person. Run it on a test person for one item in each store that
  holds personal data: the item changes, and a later export or read shows the
  new value in every store. The operator can run it. A store whose item cannot
  be corrected or completed fails.
- **References:** Regulation (EU) 2016/679, Official Journal L 119, 4.5.2016,
  Article 16 (read 2026-11-24)

The software MUST let its operator correct the personal data about one person,
in every store that holds it, and complete incomplete data, when the person
asks.

**Rationale:** Wrong data leads to wrong decisions about the person, and only
the software can change what it stores.

### USS-GDPR-003: Erase one person's data on request

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** The software has an operation, a command or a documented
  procedure that the operator can run and that erases all personal data about
  one person. Run it on a test person in a test instance, and then search each
  store that the software controls (the primary database, search indexes,
  caches and files) for the person's identifier and name: no record remains,
  except data that Article 17(3) lets the controller keep (for example, data
  that a legal obligation requires). A store that still holds other data about
  the person fails. Whether an erasure request is granted under Article 17(1)
  is the controller's judgement and is not checked.
- **References:** Regulation (EU) 2016/679, Official Journal L 119, 4.5.2016,
  Article 17(1) and (3) (read 2026-11-24)

The software MUST give its operator a way to erase all personal data about one
person, in every store that it controls, when an erasure request is granted
under Article 17(1), except data that Article 17(3) allows the controller to
keep. The controller judges the grounds.

**Rationale:** The right to erasure is empty when a copy survives in an index
or a cache. USS-DAT-005 asks the same of user data in general, with the backup
and audit-record cases; this rule applies the Article 17 exceptions to the
personal data.

### USS-GDPR-004: Record each consent

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** Software that asks for no consent passes, with that as the
  evidence. Otherwise find where consent is recorded. For each consent given
  since the last audit, or in the last 12 months when there is none, and for a
  test consent that you give, the record shows who consented (the person's
  identifier), when (a date and time) and to what (the purpose, or the version
  of the text that the person saw). A consent with any of the three missing, or
  a consent that the software relies on but does not record, fails.
- **References:** Regulation (EU) 2016/679, Official Journal L 119, 4.5.2016,
  Article 7(1) (read 2026-11-24)

When the software relies on consent, it MUST record who consented, when, and to
what.

**Rationale:** Article 7(1) makes the controller able to demonstrate that the
person consented, and only a record shows it. That the record holds who, when
and to what is USS's reading of what "demonstrate" needs, for a qualified
person to confirm.

### USS-GDPR-005: Make withdrawing consent as easy as giving it

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** Software that asks for no consent passes, with that as the
  evidence. Otherwise, for each consent that the software asks for, compare the
  way to give it with the way to withdraw it: the person withdraws it in the
  same place or channel as they gave it, with no more steps than giving it
  took, and with no extra requirement (such as a letter, a call or an account
  that giving it did not need). A consent that cannot be withdrawn by the
  person, or that takes more steps or another channel, fails.
- **References:** Regulation (EU) 2016/679, Official Journal L 119, 4.5.2016,
  Article 7(3) (read 2026-11-24)

When the software relies on consent, a person MUST be able to withdraw it at any
time as easily as they gave it.

**Rationale:** Article 7(3) says that it is as easy to withdraw as to give
consent. The comparison of channel and steps is USS's reading of "as easy", for
a qualified person to confirm.

## Operation

### USS-GDPR-006: Write down the breach notification procedure

- **Level:** MUST
- **Applies when:** operates-service
- **Category:** Safety
- **Verify:** The project has a written procedure that the auditor can read. It
  says that the supervisory authority is notified without undue delay and,
  where feasible, not later than 72 hours after becoming aware of a
  personal-data breach, unless the breach is unlikely to result in a risk to
  the rights and freedoms of people. It says that the people affected are
  told, as Article 34 requires, when a breach is likely to result in a high
  risk to them (the exceptions of Article 34(3) may be stated). A procedure
  with no deadline, a deadline later than 72 hours with no "where feasible"
  condition, or no communication to the people affected where Article 34
  requires it, fails.
- **References:** Regulation (EU) 2016/679, Official Journal L 119, 4.5.2016,
  Articles 33(1) and 34(1) (read 2026-11-24)

The project MUST have a written procedure that notifies the supervisory
authority of a personal-data breach without undue delay and, where feasible,
within 72 hours of becoming aware of it, unless the breach is unlikely to
result in a risk to the rights and freedoms of people, and that notifies the
people affected when Article 34 requires it.

**Rationale:** The 72 hours run from the moment the controller becomes aware,
so a team with no procedure in place cannot meet them.

### USS-GDPR-007: Keep a record of processing activities

- **Level:** MUST
- **Applies when:** operates-service
- **Category:** Safety
- **Verify:** The project has a record of processing activities in writing
  (a document or a file, on paper or in electronic form). For a controller, it
  holds each item that Article 30(1) lists; for a processor, each item that
  Article 30(2) lists. A record that is missing, not in writing, or that lacks
  one of the items fails. A project that Article 30(5) lifts the duty
  for passes, and the auditor records the facts that show it.
- **References:** Regulation (EU) 2016/679, Official Journal L 119, 4.5.2016,
  Article 30(1), (2), (3) and (5) (read 2026-11-24)

Unless Article 30(5) lifts the duty, the project MUST keep, in writing, the
record of processing activities that Article 30 describes.

**Rationale:** The record is the document that a supervisory authority asks for
first, and it shows the team what personal data it processes and why.
