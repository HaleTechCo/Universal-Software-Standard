# USS audit report: p

Not compliant with USS 0.1.0 as of 2026-11-21: 1 documented MUST deviation, 1 SHOULD failure, 1 unverified rule, 1 missing MAY decision.

**Audited commit:** `aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa` · **Date:** 2026-11-21 · **Auditor:** tester

## Scores

| Category | Score | Passed | Applicable |
| --- | --- | --- | --- |
| Safety | 0% | 0 | 2 |
| Features | 0% | 0 | 1 |

## Results

### Fail

| Rule | Level | Category | Title | Evidence |
| --- | --- | --- | --- | --- |
| USS-SEC-002 | SHOULD | Safety | Use a least-privilege database account | e |

### Unverified

| Rule | Level | Category | Title | Evidence |
| --- | --- | --- | --- | --- |
| USS-EXT-001 | MAY | Features | Offer a plugin interface | no result recorded |

### Deviation

| Rule | Level | Category | Title | Evidence |
| --- | --- | --- | --- | --- |
| USS-SEC-001 | MUST | Safety | Validate input | e |

## Standard Deviations

| Rule | Document | Approved by | Date | Review by |
| --- | --- | --- | --- | --- |
| USS-SEC-001 | d.md | a | 2026-11-01 |  |

## Suppressions

| File | Line | Rule | Reason |
| --- | --- | --- | --- |
| src/db.go | 42 | USS-SEC-002 | legacy \| account |
