# Fair dealing

- **Prefix:** TRU
- **Kind:** area

This area covers the promises that software keeps to its users from one
version to the next: what was free stays free, the licence does not tighten,
security essentials are not sold as extras, users' own data is never held
back, and changes of price or licence terms are announced ahead. Its rules
count toward Fairness; the rule on security essentials counts toward Safety.
The Commercial licensing (COM) area holds the terms of sale (prices, price
increases, licence delivery). The Licensing (LIC) area holds the declaration
of the licence (USS-LIC-001) and the choice of an OSI-approved one
(USS-LIC-002). The Security area holds the authentication that the software
offers (USS-SEC-015, USS-SEC-017), and the Audit logging area holds the audit
trail (USS-AUD-001 to USS-AUD-009); this area only holds that none of them is
sold as an extra. The Data area holds the export of users' data
(USS-DAT-006).

## Promises across versions

### USS-TRU-001: Keep free features free

- **Level:** SHOULD NOT
- **Applies when:** publishes-releases or operates-service
- **Category:** Fairness
- **Verify:** Take the releases (or, for a service, the changes announced)
  since the last audit, or the last 12 months when there is none, together
  with the latest release before that period. For each, list the features
  that a user had at no charge, from the documentation, the feature list, the
  pricing page and the changelog of that version. Compare the list of the
  earlier version with the list of the latest version. A feature that is
  gone, or that is now behind a payment, a paid tier or a paid licence, fails.
  A feature that is free in the latest version passes, whatever its place in
  the product.
- **References:** none

A feature that users once had at no charge SHOULD NOT be removed, or moved
behind a payment or a paid tier, in a later version.

**Rationale:** Users choose software, and build on it, because of what it
offers today. A feature that is later removed, or sold back to them, turns
their earlier choice and work into a trap. This is the "rug-pull" that users
of free and open software fear most.

### USS-TRU-002: Do not relicense to more restrictive terms

- **Level:** SHOULD NOT
- **Applies when:** publishes-releases
- **Category:** Fairness
- **Verify:** Find the licence text of the latest release and of each earlier
  release (the licence file at each release tag, or in each package). Compare
  the latest with each earlier one. A later licence is more restrictive when
  it gives users fewer of the permissions that the earlier one gives (to use,
  copy, modify, distribute or sublicense, or to use commercially), or when it
  adds a condition, a limit on the field of use, or a charge. A later release
  under such a licence fails. A later release under the same licence, or under
  one that gives the same permissions with no added condition, passes.
- **References:** none

A later version SHOULD NOT be released under a licence that is more
restrictive than the licence of the earlier versions.

**Rationale:** Users and contributors accepted the terms in force when they
adopted or contributed to the software. Tightening them later takes back what
they were given. The declaration of the licence (USS-LIC-001) and the use of an
OSI-approved one (USS-LIC-002) are held by the Licensing area.

## Security essentials

### USS-TRU-003: Do not charge extra for security essentials

- **Level:** SHOULD NOT
- **Applies when:** is-commercial and handles-authentication
- **Category:** Safety
- **Verify:** Take each of these that the software provides: single sign-on
  through OIDC, SAML or LDAP; multi-factor authentication; audit logging. In
  the price list, the tier comparison and the order forms, check that none of
  them is a separate item with its own price, and that none is listed only in
  a tier above the lowest one that is sold. In the code and the licence
  entitlements, check that none sits behind a plan or licence check that the
  lowest sold tier fails. Any of them sold on its own, or only in a higher
  tier, fails.
- **References:** CISA, Shifting the Balance of Cybersecurity Risk:
  Principles and Approaches for Secure by Design Software (revised
  2023-10-25), Principle 1, Pro-Security Business Practices (logging at no
  additional charge; eliminate hidden taxes, the "SSO tax" and MFA), and
  Secure by Default Tactics (single sign-on and secure logging at no
  additional cost)

Single sign-on (OIDC, SAML or LDAP), multi-factor authentication and audit
logging SHOULD NOT be sold separately, or only in higher tiers.

**Rationale:** When these are priced as a luxury, the customers with the least
money have the weakest security. CISA calls this the "SSO tax" and asks
manufacturers to treat security as a customer right. Whether the software
offers these at all is for the Security and Audit logging areas.

## Users' data and notice

### USS-TRU-004: Never hold users' data hostage

- **Level:** MUST NOT
- **Applies when:** is-commercial and stores-user-data
- **Category:** Fairness
- **Verify:** List each way in which a user can export their own data in an
  open and documented format (a screen, a command, an API call, a documented
  file copy). For each, search the code and the configuration for a check of
  the state of the subscription, the licence or the payment on the path of
  the export: such a check fails. Then run the export with a test account
  whose subscription has lapsed or been cancelled (or whose licence has
  expired): it works as it does for a paying account, and its output is as
  complete. An export that is refused, cut down or delayed by that state
  fails.
- **References:** none

Users' export of their own data, in an open and documented format, MUST NOT
depend on the state of their subscription or licence.

**Rationale:** Data that a user can take out only while they pay keeps them as
a customer by force. Users who stop paying keep the right to their own data.
This rule does not make the software offer an export; the optional export is
USS-DAT-006.

### USS-TRU-005: Announce pricing and licence changes in advance

- **Level:** SHOULD
- **Applies when:** is-commercial
- **Category:** Fairness
- **Verify:** List the changes to pricing or to licence terms since the last
  audit, or in the last 12 months when there is none, from the history of the
  price list, the terms and the licence files. For each, find the changelog
  entry that announces it, and compare its date with the date on which the
  change takes effect. A change with no changelog entry, or with an entry
  dated on or after the day it takes effect, fails.
- **References:** none

Each change to pricing or to licence terms SHOULD be announced in the
changelog before it takes effect.

**Rationale:** Customers plan budgets and contracts ahead. A change that they
learn of after it applies leaves them no time to decide. The format of the
changelog is held by the Releases area (USS-REL-005); the notice period of a
price increase is held by the Commercial licensing area (USS-COM-008).
