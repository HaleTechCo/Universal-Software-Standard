# Audit logging

- **Prefix:** AUD
- **Kind:** area
- **Applies when:** handles-authentication or has-multiple-users or
  operates-service

This area covers the audit trail: the record of who did what in the software,
kept so that people can reconstruct and prove it later. Its rules count toward
Safety. In USS, "audit" alone means a compliance audit, an assessment of a
project against this standard; the records that this area describes are always
called audit records or the audit trail. Offering audit logging at no extra
charge belongs to the Fair dealing (TRU) area. The Observability (OBS) area
holds the structured application logs (USS-OBS-001), and the Privacy (PRV)
area holds keeping secrets and sensitive data out of logs (USS-PRV-004), a
rule that leaves the identities of an audit record to this area. The rule on
retention names the personal-data retention rule (USS-PRV-003).

### USS-AUD-001: Record security-relevant events

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** Take these event types: sign-in success, sign-in failure,
  authorization failure, change of privileges or roles, configuration change,
  administrative action, access to sensitive data, and access to the audit
  trail. For each, find in the code where the software handles it, and check
  that an audit record is written there. An administrative action is an
  operation that only a role with administrative rights can do (list the admin
  commands, the admin endpoints and the admin screens). Sensitive data is the
  data that the project's answer to the handles-sensitive-data fact covers
  (personal, financial, health and credential data). Then trigger each event in
  a test instance: a record appears for each. An event type that the software
  can produce and that leaves no record fails.
- **References:** NIST SP 800-53 Rev. 5 (release 5.2.0), AU-2; OWASP ASVS
  5.0.0, V16.3.1 and V16.3.2; CWE-778

The software MUST record an audit event for each sign-in success and failure,
authorization failure, change of privileges or roles, configuration change,
administrative action, access to sensitive data, and access to the audit trail.

**Rationale:** An investigation can only use what was recorded before the
event. The listed events are the ones that show who got in, who tried to get
further, who changed the rules and who read what matters. The record of an
access to the audit trail is what lets the trail itself be watched.

### USS-AUD-002: Record who, what, when, where and the outcome

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** Take one audit record of each event type that the software
  writes (from the test instance of USS-AUD-001, or from a sample of the
  trail). Each holds, as a field that a parser reads, the actor (a user or
  client identity), the action, the target (the object that the action was
  done to), the time, the source (such as the client address or the host) and
  the outcome (success or failure). A record that lacks one of the six fails.
- **References:** NIST SP 800-53 Rev. 5 (release 5.2.0), AU-3; OWASP ASVS
  5.0.0, V16.2.1

Each audit record MUST name the actor, the action, the target, the time, the
source (such as the client address) and the outcome.

**Rationale:** A record that says only that something happened cannot say whom
to ask. The six fields are what turn an entry into evidence. The actor is the
identity that the Privacy area leaves to this area.

### USS-AUD-003: Keep audit records apart from other logs

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** Find where the code writes audit records and where it writes the
  application log. They go to different streams or stores (a separate file,
  table, topic or log destination), and the audit destination has no filter
  that reads the application's log level. Run the software with the quietest
  log level that it offers (such as off, or error only) and trigger an audit
  event: the record is still written. A single stream that holds both, or a
  record that a level setting can turn off, fails.
- **References:** OWASP ASVS 5.0.0, V16 introduction (control objective)

Audit records MUST go to their own stream or store, which log-level settings
cannot turn off.

**Rationale:** Debug settings are changed to cut noise, and the audit trail
disappears with the noise. A stream of its own can be given its own access,
its own retention and its own protection.

### USS-AUD-004: Make the audit trail tamper-evident

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** Read the permissions that the software's own credentials have on
  the audit store (such as database grants, file modes or object-store
  policies): they allow adding records and do not allow changing or deleting
  them. Then, in a test copy of the trail, change one record and remove
  another whose retention period (USS-AUD-007) has not ended, and run the
  project's check for it (such as a hash chain, a signature, or a write-once
  store with its report): the check reports both. Where records are removed at
  the end of their retention period, the step that removes them runs outside
  the software's own role (such as a storage lifecycle rule or a separate
  retention process), removes only records past that period, and the check
  reports the removal as expiry and not as tampering. A trail that the
  software can rewrite, or a change that no check finds, fails.
- **References:** NIST SP 800-53 Rev. 5 (release 5.2.0), AU-9; OWASP ASVS
  5.0.0, V16.4.2

The audit trail MUST be append-only for the software, and a change to a record,
or its removal, MUST be detectable. The one removal that is allowed is that of
records whose retention period (USS-AUD-007) has ended, by a step outside the
software's own role, and it is recognisable as expiry and not as tampering.

**Rationale:** A person who breaks in, or who abuses their rights, first wants
to remove what shows it. A trail that shows its own damage keeps its value
even when it cannot be kept out of reach.

### USS-AUD-005: Use synchronized UTC time

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** Take the sample of audit records (USS-AUD-002): each timestamp
  is in UTC (written with a Z or with an offset of +00:00), and the code takes
  it from the system clock and not from a client's value. For the project's own
  deployment, the configuration of the hosts or the platform names a time
  source that they synchronize with (such as NTP, chrony or a cloud time
  service). For software that others run, the documentation tells operators to
  synchronize the clock of the host and names the time source that the
  software relies on. A local-time timestamp, or a clock with no time source,
  fails. Settings that the auditor cannot open are `unverified`.
- **References:** NIST SP 800-53 Rev. 5 (release 5.2.0), AU-8; OWASP ASVS
  5.0.0, V16.2.2

Audit timestamps MUST be in UTC, from a clock that is synchronized to a time
source.

**Rationale:** Records from several systems are put in one order by their
time. A different zone, or a clock that drifts, puts effects before their
causes. UTC has no daylight-saving change to confuse the order. NIST allows a
fixed offset as well; this standard picks UTC so that every project reads
records the same way.

### USS-AUD-006: Restrict access to the audit trail

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** The documentation or the code names the role or permission that
  grants access to the audit trail, and lists who holds it. Try to read the
  trail (through the UI, the API, the command line and the store) as a user
  without that permission: each is refused. Then read the routes, commands
  and screens of the software, for every role including the most privileged:
  none changes or deletes an audit record, except that records whose
  retention period (USS-AUD-007) has ended may be removed by a step outside
  the software's own role (such as a storage lifecycle rule or a separate
  retention process). A way to read the trail that needs no audit permission,
  or one to change or delete any other record through the software, fails.
- **References:** NIST SP 800-53 Rev. 5 (release 5.2.0), AU-9; OWASP ASVS
  5.0.0, V16.4.2

The software MUST let only roles granted audit access read the audit trail, and
MUST give no role a way to change or delete records through the software. The
one exception is the removal of records whose retention period (USS-AUD-007)
has ended, by a step outside the software's own role.

**Rationale:** The trail names people and what they did, so reading it is a
power of its own, and the most powerful role is the one whose actions it must
be able to show. A trail cannot grow for ever, so expiry is the one removal
that is left, and no role steers it.

### USS-AUD-007: Make audit retention configurable

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** The documentation names the setting (a configuration key, an
  environment variable or an option) that sets how long audit records are
  kept, and gives its default as a duration. Read the code: the setting is the
  one that the expiry or archive step reads, and no fixed value overrides it.
  Where the audit records hold personal data (such as a user identity or a
  client address), they are listed in the project's personal-data inventory
  (USS-PRV-001) as a kind of data with a retention period, and the default is
  no longer than that period (USS-PRV-003). Where they hold none, nothing more
  is checked. A retention that operators cannot set, or a default longer than
  the inventory's period, fails.
- **References:** NIST SP 800-53 Rev. 5 (release 5.2.0), AU-11

Operators MUST be able to set how long audit records are kept, and the
documented default MUST fit the personal-data retention rule (USS-PRV-003).

**Rationale:** Investigations need records for months or years, and privacy
rules want personal data gone when its purpose ends. Only the operator knows
which duty is stronger in the place where the software runs, so the period is
theirs to set, from a default that is already lawful.

### USS-AUD-008: Alert when audit logging fails

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** Read the code that writes audit records: a failed write (a full
  disk, a refused connection, a denied permission) is caught and raises an
  alert, and is not swallowed or sent only to the application log. In a test
  instance, make the audit store unwritable and trigger an audit event: an
  alert reaches a person or the monitoring system that the operator has
  configured (such as a pager, an email, a chat message, or a metric with an
  alert rule). A log line alone does not count. A failure that raises no alert
  fails.
- **References:** NIST SP 800-53 Rev. 5 (release 5.2.0), AU-5; CWE-778

A failure to write audit records MUST raise an alert to an operator. This alert
is required for that named condition and is outside the symptom rule for paging
alerts (USS-OPS-002).

**Rationale:** When writing stops, the trail has a hole that nobody knows
about, and the events in it are lost. The alert tells the operator while the
hole is small.

### USS-AUD-009: Export audit records in a standard format

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Safety
- **Verify:** The documentation describes how to export the audit records or
  stream them to a SIEM (a command, an API, a file drop or a forwarding
  setting), and names the format, a published specification such as syslog
  per RFC 5424, OCSF or CEF. Run the export on a test instance and take a
  sample of 20 records (all, if fewer): each parses with a parser or schema
  validator of the named format (such as an RFC 5424 parser, or the OCSF schema
  of the named version) with no error. An export that only the software's own
  tools read, or one with a format that has no published specification, fails.
- **References:** RFC 5424 (March 2009); OCSF schema 1.10.0 (read 2026-11-24);
  OWASP ASVS 5.0.0, V16.2.4 and V16.4.3

Audit records SHOULD be exportable or streamable to a SIEM in a standard
format (such as syslog per RFC 5424, OCSF or CEF).

**Rationale:** Operators watch many systems in one place. A common format lets
the audit records join that place without a converter written for each
product, and a copy in another system also protects the trail if the first is
lost.
