# Squawk

- **Kind:** profile
- **Product:** Squawk
- **Vendor:** sbdchd
- **Maintainer:** community
- **Versions:** >=2.74.0, <3.0.0

Squawk is a linter for PostgreSQL migration files. It reads SQL without a
database and reports statements that lose data, hold long locks or break
clients that are still running. The PostgreSQL module recommends it in
USS-PG-002. Put the baseline below in `.squawk.toml` in the repository root
(Squawk looks for that name from the current directory upwards, and `--config`
names another file). The rules and settings that the mappings name are those of
Squawk 2.74.0, from its documentation and source (read 2026-11-24). `squawk`
(or `squawk check`) exits with status 1 when it reports any violation, and
with status 0 when it reports none.

The baseline is Squawk's default rule set with no exclusions, and the PostgreSQL
version set. Squawk has 27 opt-in rules that are off by default (26 rules and
the alias `require-timeout-settings`), and the baseline turns on none of them:
no opt-in rule tests a requirement of USS-DAT-002 or of a rule of the PostgreSQL
module. The checks that ban dropping a column, a table, a schema or a database
and truncating with `CASCADE` (`ban-drop-column`, `ban-drop-table`,
`ban-drop-schema`, `ban-drop-database`, `ban-truncate-cascade`) are on by
default, and so is `require-lock-timeout`. The opt-in rules are
`require-table-schema`, `ban-drop-trigger`, `ban-alter-generated-expression`,
`ban-drop-index`, `ban-set-default`, `ban-disable-trigger`,
`ban-replica-identity`, `ban-drop-policy`, `ban-revoke`,
`ban-replace-view-function`, `ban-alter-policy-condition`,
`ban-alter-policy-roles`, `ban-create-policy`, `ban-alter-function-options`,
`ban-alter-view-options`, `ban-alter-role-options`,
`ban-alter-database-options`, `ban-alter-row-level-security`,
`ban-new-write-restriction`, `ban-add-enum-value`,
`ban-add-composite-attribute`, `ban-add-column`, `ban-detach-inheritance`,
`ban-alter-sequence-values`, `ban-alter-system-options`, `ban-alter-extension`
and the alias `require-timeout-settings` (its two parts, `require-lock-timeout`
and `require-statement-timeout`, are on by default). Some of them ban a whole
kind of statement: `ban-revoke` reports `REVOKE`, the change of an object's
owner and the drop of a role, which the role setup of USS-PG-005 can need. A
project may turn more of them on with `included_rules`.

Set `pg_version` to the oldest PostgreSQL major version that the project
supports: some checks depend on it (`adding-field-with-default` reports a
constant or non-volatile column default only before version 11). Squawk's own
default is 15.0. A project whose migration tool runs each file in a transaction
also sets `assume_in_transaction = true`, which the baseline leaves out because
it depends on the migration tool. Run Squawk over each migration file that the
change adds (or over all of them).

Squawk has no setting that reports a suppression that suppresses nothing: its
`unused-ignore` check reports only a rule name that does not exist. The auditor
lists each `squawk-ignore` comment as a suppression, with the rule's name.

Community-maintained: written by USS contributors until the vendor takes it
over.

## Baseline configuration

```toml
# .squawk.toml, in the repository root.
# Run with: squawk <the migration files>
# Set to the oldest PostgreSQL major version that the project supports.
pg_version = "15.0"
# Squawk's default rule set, with no rule excluded and no path skipped.
# A command-line --exclude or --exclude-path replaces these two lists, so
# the CI job does not pass them. A single exception is a squawk-ignore comment
# that names the rule, with a reason.
excluded_rules = []
excluded_paths = []
```

## Mappings

| Rule | Category | Tool rules or settings | Coverage | How to verify |
|---|---|---|---|---|
| USS-DAT-002 | Reliability | `ban-drop-column` (reports `DROP COLUMN` of a table or a foreign table, and the drop of an attribute of a type), `ban-drop-table` (reports `DROP TABLE` and `DROP FOREIGN TABLE`), `ban-drop-schema`, `ban-drop-database`, `ban-truncate-cascade` (reports `TRUNCATE ... CASCADE`) | partial: no check reports `DELETE`, a plain `TRUNCATE` (only the `CASCADE` form), or the removal of a field from stored documents, and the rule also needs the separate step that an operator runs on purpose, which Squawk does not read | Run `squawk` over the migration files and expect no finding of these checks; read each migration for `DELETE` and `TRUNCATE`; find the separate script or command for each structure that is removed. |
| USS-PG-001 | Reliability | data loss: `ban-drop-column`, `ban-drop-table`, `ban-drop-schema`, `ban-drop-database`, `ban-truncate-cascade`; long locks: `require-concurrent-index-creation`, `require-concurrent-index-deletion`, `require-concurrent-reindex`, `require-concurrent-partition-detach`, `constraint-missing-not-valid`, `adding-foreign-key-constraint`, `adding-not-nullable-field`, `adding-field-with-default`, `adding-serial-primary-key-field`, `changing-column-type`, `disallowed-unique-constraint` | partial: CI must run `squawk` over each new migration file and fail on exit status 1, and the checks report only the statements that they name, so a statement that loses data or holds a lock for long and that no check names (such as `DELETE`, or a plain `TRUNCATE`) is found only by reading the migration | Read the CI job for the `squawk` step and its files; run `squawk` over the migration files and expect exit status 0; read each migration for statements that no check names. |
| USS-PG-003 | Reliability | `require-lock-timeout` (reports the first statement of a file that can hold a lock for a long time when no `SET lock_timeout` comes before it; a value of 0 counts as no timeout) | partial: the check does not report a file with no such statement, and it does not see a timeout that the connection sets (the rule accepts one), so a migration file with only quick statements, or a connection with a default, is judged by reading the file and the migration role | Run `squawk` over the migration files and expect no `require-lock-timeout` finding; read the migration tool's connection options and the role defaults for a timeout set outside the files. |

## Check-integrity guards

| Guard | Setting | Rule |
|---|---|---|
| No rule is excluded | `excluded_rules` is empty (or absent) and the CI command passes no `--exclude`/`-e`, which would replace the list of the file | USS-QLT-006, USS-QLT-007 |
| No path is skipped | `excluded_paths` is empty (or absent) and the CI command passes no `--exclude-path` | USS-QLT-006 |
| A suppression names its rules and can give the reason | the comment `-- squawk-ignore <rule>[, <rule>]`, with the reason after a second `--`: Squawk ignores that text and requires none, so the auditor reads each reason; an unknown rule name in a comment is reported as `unused-ignore` | USS-QLT-009 |
| No blanket suppression | the comments `-- squawk-ignore` and `-- squawk-ignore-file` with no rule name suppress every rule and are not used | USS-QLT-010 |
