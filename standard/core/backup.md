# Backup

- **Prefix:** BAK
- **Kind:** area

This area covers getting data back after a loss, in three scopes: what the
software offers its operators, the deployments that the project runs itself,
and the project's source. Its rules count toward Reliability. The Data (DAT)
area holds the migrations that an upgrade runs (USS-DAT-001), the Privacy
(PRV) area holds when personal data is deleted (USS-PRV-003), and the
Distribution (DST) area points to the backup before an upgrade
(USS-BAK-003).

## What the software offers its operators

### USS-BAK-001: Provide backup and restore

- **Level:** MUST
- **Applies when:** stores-persistent-data and publishes-releases
- **Category:** Reliability
- **Verify:** The documentation describes how to back up and how to restore
  all the persistent data: built-in commands, or copy steps. List where the
  code and the default configuration write persistent data (a database,
  data directories, uploaded files, keys): the procedure covers each one.
  Where the procedure is a plain copy, the data is plain files, and the
  documentation says when a copy is consistent (such as with the software
  stopped). A persistent location that the procedure leaves out fails.
- **References:** NIST SP 800-34 Rev. 1 (May 2010, updated 2010-11-11),
  section 3.4.1

The software MUST offer a documented way to back up and restore all its
persistent data: built-in commands, or documented copy steps where the data is
plain files that a copy captures consistently.

**Rationale:** People who run the software are the ones who lose data, and
only the software's authors know every place where it keeps data. A documented
procedure turns a loss into a restore.

### USS-BAK-002: Make backups consistent

- **Level:** MUST
- **Applies when:** stores-persistent-data and publishes-releases
- **Category:** Reliability
- **Verify:** Take the documented backup (USS-BAK-001) of a test instance
  while a script writes to it, without stopping it unless the documentation
  says to pause writes. Restore it into a clean instance, and run the
  software's own integrity check (such as a database integrity check or a
  startup self-check): it reports no error, and each record that the backup
  holds is complete, with no half-written row or file. The backup comes from
  the database's own dump, a snapshot, or a pause in writes.
- **References:** NIST SP 800-34 Rev. 1 (May 2010, updated 2010-11-11),
  section 3.4.2

A backup taken while the software runs MUST be consistent, such as through the
database's own dump, a snapshot, or a pause in writes.

**Rationale:** A copy of files that change while it is read can hold half of
one change and half of another, and the first restore shows it, too late.

### USS-BAK-003: Back up before each upgrade

- **Level:** SHOULD
- **Applies when:** stores-persistent-data and publishes-releases
- **Category:** Reliability
- **Verify:** Read the upgrade path (the upgrade command, the installer or
  the start-up code that runs migrations). It takes a backup of the persistent
  data before the first step that changes stored data, or it stops and asks
  for one. Upgrade a test instance from the previous release: a backup exists,
  dated before the migrations ran. Restore it into the previous version: the
  data is as it was before the upgrade.
- **References:** NIST SP 800-34 Rev. 1 (May 2010, updated 2010-11-11),
  section 3.4.1

The upgrade SHOULD take a backup before it changes stored data, so that the
previous version can be restored.

**Rationale:** A migration that fails halfway, or a release that turns out
wrong, leaves a way back only when the data was saved first. The backup is what
makes a rollback possible.

### USS-BAK-004: Restore onto a fresh install

- **Level:** MUST
- **Applies when:** stores-persistent-data and publishes-releases
- **Category:** Reliability
- **Verify:** The CI configuration has a job that installs the current version
  on a clean machine or container, restores a backup into it, and checks the
  restored data (such as a count of records, or a known record). The job runs
  on each change, or at least on each release. Open the latest run, and the
  runs for the last 12 months or since the last audit: the job ran and passed.
  A restore test that is only run by hand fails.
- **References:** NIST SP 800-34 Rev. 1 (May 2010, updated 2010-11-11),
  section 3.4.1

A backup MUST restore onto a fresh install of the same version, and a CI test
proves it.

**Rationale:** Someone who has lost a machine restores onto a new one, not
onto the one that held the data. A test in CI keeps the procedure true as the
software changes.

## The project's own deployments

### USS-BAK-005: Set recovery objectives

- **Level:** MUST
- **Applies when:** operates-service and stores-persistent-data
- **Category:** Reliability
- **Verify:** List each production data store from the deployment
  configuration (databases, object storage, volumes, queues that keep data).
  The documentation gives each one a recovery point objective (RPO: how much
  recent data it can lose) and a recovery time objective (RTO: how long its
  recovery can take), each as a duration. A store without both values fails.
- **References:** NIST SP 800-34 Rev. 1 (May 2010, updated 2010-11-11),
  section 3.4.1

Each production data store MUST have a documented recovery point objective
(RPO) and recovery time objective (RTO).

**Rationale:** The objectives decide the schedule, the storage and the cost of
the backups, and they say when a restore has taken too long.

### USS-BAK-006: Back up on a schedule

- **Level:** MUST
- **Applies when:** operates-service and stores-persistent-data
- **Category:** Reliability
- **Verify:** For each production data store (USS-BAK-005), the deployment
  configuration or the backup tool's configuration has an automatic schedule
  (such as a cron job, a managed backup policy or a CI schedule), and the
  interval between runs is no longer than the store's RPO. The listing of the
  backups shows runs at that interval over the last 30 days. Settings in a
  hosting console that the auditor cannot open are `unverified`. A backup that
  a person starts by hand fails.
- **References:** CIS Critical Security Controls v8.1, safeguard 11.2

Backups MUST run automatically, on a schedule that meets the RPO.

**Rationale:** A backup that depends on someone remembering stops the day that
person is away. A schedule that fits the RPO is what keeps the RPO true.

### USS-BAK-007: Keep three copies, one immutable

- **Level:** MUST
- **Applies when:** operates-service and stores-persistent-data
- **Category:** Reliability
- **Verify:** The documentation or the backup configuration shows, for each
  production data store, three copies of the data (the production data counts
  as one), on two kinds of storage (such as disk and object storage, or
  object storage and tape), with one copy in another location than production
  (another site, region or provider account). One copy is immutable (such as
  object lock or WORM storage with a retention period) or offline (such as
  detached media). Without access to the storage settings, the auditor records
  `unverified`.
- **References:** CISA #StopRansomware Guide (2023-09); CIS Critical Security
  Controls v8.1, safeguard 11.4

Backups MUST follow the 3-2-1 rule (three copies, on two kinds of storage, one
off-site), and one copy MUST be immutable or offline.

**Rationale:** Attackers who reach production look for the backups and delete
or encrypt them. A copy that nobody can change, wherever the attacker is,
leaves a way back.

### USS-BAK-008: Encrypt backups

- **Level:** MUST
- **Applies when:** operates-service and stores-persistent-data
- **Category:** Reliability
- **Verify:** The backup configuration turns on encryption for each copy
  (USS-BAK-007), by the backup tool or by the storage layer. Take one backup
  file from storage: it cannot be read without a key (opening it gives no
  plain data). Without access to the storage settings, the auditor records
  `unverified`.
- **References:** CISA #StopRansomware Guide (2023-09); CIS Critical Security
  Controls v8.1, safeguard 11.3

Backups MUST be encrypted.

**Rationale:** A backup holds all the data in one place, often with less
protection than production. Keeping the keys apart from the backups is good
practice, because a key stored next to the data protects nothing.

### USS-BAK-009: Keep backup credentials apart from production

- **Level:** MUST
- **Applies when:** operates-service and stores-persistent-data
- **Category:** Reliability
- **Verify:** List the credentials that can change or delete backups (storage
  keys, tokens and roles with write or delete rights on the backup storage or
  the backup tool). Search the production deployment configuration, secrets
  and environment: none of those credentials is in them, and the roles that
  production workloads run as have no such right. Reading the policies of the
  hosting account needs access, and without it the auditor records
  `unverified`.
- **References:** CISA #StopRansomware Guide (2023-09); CIS Critical Security
  Controls v8.1, safeguard 11.3

The credentials that can change or delete backups MUST stay out of reach of the
production systems.

**Rationale:** Anyone who takes over a production system gets what that system
can do. If it can delete the backups, the same attack destroys the data and its
copies.

### USS-BAK-010: Alert on failed backups

- **Level:** MUST
- **Applies when:** operates-service and stores-persistent-data
- **Category:** Reliability
- **Verify:** The monitoring or backup configuration has an alert for each
  production data store that fires when a backup run fails and when no backup
  has completed within its schedule interval (USS-BAK-006). The alert goes to
  a person (such as a pager, a chat channel that the team reads, or an email
  address), and a log line alone does not count. Where the project has a
  record of a failed or missed backup in the last 12 months or since the last
  audit, the alert that it raised is on record. Settings in a console that the
  auditor cannot open are `unverified`.
- **References:** CIS Critical Security Controls v8.1, safeguard 11.2

A failed or missed backup MUST raise an alert to a person.

**Rationale:** A backup job that fails in silence gives months of false
safety. The failure is found on the day the restore is needed.

### USS-BAK-011: Test restores on a schedule

- **Level:** MUST
- **Applies when:** operates-service and stores-persistent-data
- **Category:** Reliability
- **Verify:** The documentation sets a schedule for restore tests, with an
  interval of at most three months. The project holds a record of each test
  (a date, the backup that was restored, the result): there is at least one
  record in each quarter of the last 12 months, or since the last audit when
  that is shorter. Each record shows a restore into a separate environment
  and a check of the restored data.
- **References:** CIS Critical Security Controls v8.1, safeguard 11.5

A restore from backup MUST be tested on a documented schedule, at least once a
quarter, and the results MUST be recorded.

**Rationale:** A backup that has never been restored is a hope and not a
backup. The quarterly floor is a number that USS adds, as CIS safeguard 11.5
does, because the safeguard sets quarterly or more often for a sample of
assets.

### USS-BAK-012: Expire personal data from backups

- **Level:** MUST
- **Applies when:** operates-service and stores-persistent-data and
  handles-personal-data
- **Category:** Reliability
- **Verify:** The documentation states how long backups are kept (a retention
  period). The retention setting of the backup tool or the storage (an expiry
  rule, a lifecycle rule or a pruning policy) is no longer than that period.
  List the stored backups: the oldest is no older than the period. Personal
  data that is deleted from production (USS-PRV-003) is therefore out of every
  backup within that period. Settings in a console that the auditor cannot open
  are `unverified`.
- **References:** none

Backup retention MUST be set so that deleted personal data leaves the backups
within the documented retention period (USS-PRV-003).

**Rationale:** Deleting data from production while old backups keep it only
postpones the deletion. A fixed, short retention gives a date by which the data
is gone, and the documentation can state that date to the people concerned.

## The source

### USS-BAK-013: Mirror the repository

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Reliability
- **Verify:** The project has an automatic mirror of the repository (such as a
  push mirror, a scheduled job or a webhook) to a second location on a host
  that does not depend on the primary host (another provider, not another
  repository on the same one). Compare the branches and tags of the mirror
  with the primary (such as with `git ls-remote` on both): they are the same,
  and the mirror holds the full history. Settings in the host that the auditor
  cannot open are `unverified`.
- **References:** none

The repository, with its full history and tags, SHOULD be mirrored
automatically to a second location that does not depend on its primary host.

**Rationale:** A host can lose a repository, lock an account or go away, and
the history is the project's most valuable asset. A mirror kept up to date on
its own is a copy that nobody has to remember to make.
