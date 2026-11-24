# Data

- **Prefix:** DAT
- **Kind:** area
- **Applies when:** stores-persistent-data

This area covers data that the software keeps between runs: how its structure
changes, how it is documented and protected, and what users can do with their
own data. Its rules count toward Reliability by default; the encryption rule
counts toward Safety and the export and import rules toward Features. The
Security (SEC) area holds encryption of data in transit (USS-SEC-022), the
Privacy (PRV) area holds retention and the personal-data inventory, and the
Testing (TST) area holds the test of each migration (USS-TST-007).

## Changing stored data

### USS-DAT-001: Change stored data only through versioned migrations

- **Level:** MUST
- **Applies when:** always
- **Category:** Reliability
- **Verify:** The repository holds the migrations (such as numbered SQL
  files, or files of a migration tool) in order, and the software or its
  installer runs the pending ones when it starts or upgrades, with no manual
  step. Take each change to a schema, a file format or a stored structure
  since the last audit, or the last 12 months when there is none: it comes
  with a migration in the same change. Search the code for schema statements
  (such as `CREATE`, `ALTER` and `DROP`) outside the migration files: none
  changes a production schema. A change made by hand on a running database
  fails.
- **References:** none

Each change to the structure of stored data, a database schema or a file
format, MUST be made by a versioned migration in the repository, which runs
automatically on upgrade.

**Rationale:** A change that is made by hand differs between installations, so
nobody can say what state a given database is in. A versioned migration gives
every installation the same path, and the repository records it.

### USS-DAT-002: No data loss in automatic migrations

- **Level:** MUST NOT
- **Applies when:** always
- **Category:** Reliability
- **Verify:** Read each migration that runs on upgrade (USS-DAT-001), for those
  added since the last audit, or the last 12 months when there is none. None
  drops a table or a column, removes a field from stored documents, truncates a
  table or deletes rows. Where the structure must shrink, the repository has a
  separate script or documented command, outside the automatic migrations, that
  an operator runs on purpose.
- **References:** Martin Fowler, ParallelChange (read 2026-11-24), as an
  example

A migration that runs automatically MUST NOT delete stored data: it never
drops a table, a column or a field, or truncates. Deleting data needs a
separate step that an operator runs on purpose.

**Rationale:** An upgrade that runs on its own cannot ask whether the data is
still wanted, and a failed upgrade cannot get back what it has deleted. An
operator who deletes on purpose has had the chance to take a backup first.
Where a structure must go, the usual path (expand and contract) adds the new
structure, moves the data, and removes the old one in a later, separate step.

### USS-DAT-003: Document the data model

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Reliability
- **Verify:** The repository has the structure of the stored data as schema
  files, or as documentation (written or generated from the schema): each
  table, collection or file kind, with its fields and their types. Compare
  with the current migrations (USS-DAT-001): each stored structure that they
  create appears in the documentation.
- **References:** none

The structure of stored data SHOULD be documented, such as by schema files or
generated documentation.

**Rationale:** Whoever reads, migrates or backs up the data needs to know what
is in it. Generated documentation cannot fall behind the schema.

## Protecting and returning data

### USS-DAT-004: Encrypt sensitive data at rest

- **Level:** MUST
- **Applies when:** handles-sensitive-data
- **Category:** Safety
- **Verify:** For each store that holds sensitive data (a database, files, a
  cache), the documentation, the deployment configuration or the code shows
  encryption of the stored data: by the storage layer (such as disk, volume or
  database encryption) or by the software (such as encrypted fields). Without
  access to the hosting settings, the auditor records `unverified` for storage
  that the project does not configure itself. Traffic over the network is a
  separate rule (USS-SEC-022), and so are backups (USS-BAK-008).
- **References:** CWE-311; CWE-312; OWASP ASVS 5.0, V14 (Data Protection);
  NIST SP 800-111 (2007-11)

Sensitive data MUST be encrypted where it is stored, by the storage layer or
by the software.

**Rationale:** Stored data outlives the process that wrote it, and it reaches
readers who never pass through the software, such as a lost disk, a copied
backup or an operator with file access. Encryption leaves them with data that
they cannot read, as long as the keys are kept apart from the data.

### USS-DAT-005: Delete a user's data on request

- **Level:** MUST
- **Applies when:** stores-user-data
- **Category:** Reliability
- **Verify:** The software has an operation, a command or a documented
  procedure that deletes all the data of one user. Run it on a test user in a
  test instance, and then search each store (the primary database, search
  indexes and caches, and the files) for the user's identifier and name: no
  record remains outside the backups and the audit records (USS-AUD-002).
  Backups are covered by their expiry, which the documentation states, and
  audit records by their retention period (USS-AUD-007), until which they are
  kept. An operator, acting for the user, can start it.
- **References:** none

The software MUST be able to delete all the data of one user, including copies
in indexes and caches, when the user, or an operator acting for them, asks.
Audit records (USS-AUD-002) are kept until their retention period ends
(USS-AUD-007), as backups are kept until they expire.

**Rationale:** Data that was given to the software belongs to the user. A
delete that leaves copies in an index or a cache gives only the look of
deletion. An audit record that vanished on request would no longer show who
did what, so it leaves with its retention period instead.

### USS-DAT-006: Offer data export

- **Level:** MAY
- **Applies when:** stores-user-data
- **Category:** Features
- **Verify:** Adoption means that the software has an export that writes all
  of a user's data. It is adopted when the documentation describes the export
  and its format, the format is open (its specification is public, such as
  JSON, CSV or SQLite), and an export of a test user holds each kind of data
  that the data model (USS-DAT-003) lists for users. An export that leaves out
  a kind of data fails once the MAY is adopted.
- **References:** none

Users MAY be able to export all their data in a documented, open format.

**Rationale:** An export lets users leave, keep a copy and move to other
software, so the software holds them by its worth and not by their data.

### USS-DAT-007: Offer data import

- **Level:** MAY
- **Applies when:** stores-user-data
- **Category:** Features
- **Verify:** Adoption means that the software has an import. It is adopted
  when the documentation describes the import and the documented format that
  it reads. Import a sample file in that format into a clean instance: the
  data in the file appears, as the documentation says. Where the export
  (USS-DAT-006) is also adopted, that format is the export's, and importing
  the export of a test user into a clean instance gives data that matches the
  original. An import that fails on a file in the documented format fails once
  the MAY is adopted.
- **References:** none

Users MAY be able to import data in the format that the export writes.

**Rationale:** An import closes the loop of an export: it lets a user restore
their data, or bring it from another installation.
