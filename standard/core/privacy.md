# Privacy

- **Prefix:** PRV
- **Kind:** area

This area covers personal data and telemetry: what the software collects, why,
for how long, and what it leaves out of its logs. Its rules count toward
Safety by default. The Data (DAT) area holds deleting the data of one user on
request, the Security (SEC) area holds the protection of the data in storage
and transit, and the Audit logging (AUD) area holds what an audit record must
hold. Personal data is any information about an identified or identifiable
person.

### USS-PRV-001: Keep a personal-data inventory

- **Level:** MUST
- **Applies when:** handles-personal-data
- **Category:** Safety
- **Verify:** The repository or the project's documentation holds an inventory
  of personal data. For each kind of personal data (such as a name, an email
  address, an IP address or a device identifier) it gives the purpose, each
  place where the data is stored (a database, a log, a cache, a backup, a
  third party) and how long it is kept. Search the code for fields and
  columns that hold personal data, and for the personal data in each schema
  (USS-DAT-003, where it applies): each one appears in the inventory. A kind
  of personal data that the code handles and the inventory omits fails.
- **References:** NIST Privacy Framework 1.0 (2020-01-16), ID.IM-P5 and
  ID.IM-P6; ISO/IEC 29100:2024

A maintained inventory MUST list each kind of personal data that the software
handles, with its purpose, where it is stored and how long it is kept.

**Rationale:** The software cannot limit, delete or protect data that nobody
has listed. The inventory is also the base for each answer to a user who asks
what the project holds about them.

### USS-PRV-002: Collect only what each purpose needs

- **Level:** MUST
- **Applies when:** handles-personal-data
- **Category:** Safety
- **Verify:** For each kind of personal data in the inventory (USS-PRV-001),
  the inventory names a purpose, and the code that uses the data serves that
  purpose. Take each form, API field, event and log field that collects
  personal data: each maps to an entry in the inventory. For each entry,
  ask whether the purpose can be met without the field or with less detail
  (such as a year of birth in place of a date, or an IP address that is
  truncated): where the answer is yes and the software still collects the
  more, the rule fails.
- **References:** NIST Privacy Framework 1.0 (2020-01-16), ID.IM-P5 and
  ID.IM-P6; ISO/IEC 29100:2024

The software MUST collect no personal data beyond what the purposes in the
inventory need.

**Rationale:** Data that is not collected cannot leak, be misused or need to
be deleted. Each extra field is a risk that has no purpose to balance it.

### USS-PRV-003: Delete personal data when its retention ends

- **Level:** MUST
- **Applies when:** handles-personal-data and stores-persistent-data
- **Category:** Safety
- **Verify:** For each kind of personal data in the inventory (USS-PRV-001), a
  retention period is given, and a job, a database expiry setting or a time
  to live (TTL) deletes the data when the period ends, with no manual step.
  Read the job's code or the setting. In a test instance, create data older
  than the period and run the job, or wait for the setting to act: the data
  is gone, from indexes and caches too. A retention period of "indefinitely",
  or one that only an operator's action ends, fails, unless the inventory
  states the purpose that needs the data for as long as the software runs.
- **References:** NIST Privacy Framework 1.0 (2020-01-16), CT.DM-P5;
  ISO/IEC 29100:2024

Stored personal data MUST be deleted automatically when its retention period
in the inventory ends.

**Rationale:** A retention period that nobody enforces is a wish. Data that
stays after its purpose has ended is held for no reason and can still leak.

### USS-PRV-004: Keep sensitive data out of logs and telemetry

- **Level:** MUST NOT
- **Applies when:** handles-sensitive-data or uses-secrets
- **Category:** Safety
- **Verify:** Search the logging calls, the trace and metric labels, the crash
  report and telemetry code for secrets, credentials and sensitive data, such
  as passwords, tokens, keys, session identifiers, connection strings, card
  numbers and the fields that the inventory (USS-PRV-001) lists as sensitive.
  A call that writes a request body, a header map or a whole object in one
  piece is a finding unless it filters those fields. Run the software, trigger
  an error and a login, and read the logs: no secret or sensitive value
  appears in clear text. A value that is masked, hashed or removed passes.
- **References:** OWASP Logging Cheat Sheet (read 2026-11-24), the section
  Data to exclude; CWE-532; CWE-359

Logs, traces, metrics, crash reports and telemetry MUST NOT hold secrets,
credentials or other sensitive data in clear text. The identities that an
audit record must name are outside this rule, and the Audit logging area holds
them.

**Rationale:** Logs are copied, kept long and read by more people and tools
than the data store is. A secret in a log reaches each of them.

### USS-PRV-005: Make telemetry opt-in

- **Level:** MUST
- **Applies when:** collects-telemetry
- **Category:** Safety
- **Verify:** The default configuration sends no telemetry. Run a fresh
  installation with no setting changed, with the network traffic watched: no
  telemetry request leaves. Telemetry starts only after the user turns it on
  (a setting, a prompt or a flag), and the code that sends it checks that
  choice first. Telemetry that is on by default, with an opt-out, fails.
- **References:** none

Telemetry MUST be off until the user turns it on.

**Rationale:** A user who has not been asked has not agreed. Opt-out
telemetry reaches the users who never find the setting.

### USS-PRV-006: Document what telemetry sends

- **Level:** MUST
- **Applies when:** collects-telemetry
- **Category:** Safety
- **Verify:** The documentation lists each item that telemetry sends (each
  field of each event) and each address that receives it, including a third
  party that processes it. Turn telemetry on in a test instance and capture
  what it sends: each field and each destination appears in the list. A field
  or a destination that the list omits fails.
- **References:** none

The documentation MUST list every item that telemetry sends, and where it
sends it.

**Rationale:** A user can agree to telemetry only if they know what it
contains. A list that is out of date takes that choice from them.
