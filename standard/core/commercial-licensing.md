# Commercial licensing

- **Prefix:** COM
- **Kind:** area
- **Applies when:** is-commercial

This area covers the terms on which software is sold: published prices,
price increases, the licence terms and the way a licence is delivered, and
the licence models on offer. Its rules count toward Fairness. The Fair
dealing (TRU) area holds the promises across versions, including that users'
export of their own data never depends on their licence (USS-TRU-004) and
that price and licence changes are announced in the changelog
(USS-TRU-005). The Independence (IND) area holds running the software on the
customer's own infrastructure (USS-IND-001); the rules on a licence server
here build on it.

## Transparent pricing

### USS-COM-001: Publish prices

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Fairness
- **Verify:** List each licence model, tier and offering (each edition, tier,
  feature, add-on and support plan that is sold) from the pricing page, the
  order forms and the licence files. For each, find a published list price
  (an amount, or a formula with published rates) and a published list of what
  it includes, both public: readable by anyone on the project's website or in
  its documentation, with no sign-in and no contact with sales. An offering
  priced only as "contact sales", or with no list of its contents, fails.
- **References:** none

Each licence model, tier and offering SHOULD have a published list price and
a published list of what it includes, public to anyone.

**Rationale:** A buyer who cannot work out the full cost before talking to
sales cannot compare products or plan a budget. Published prices also make
the other rules of this area checkable.

### USS-COM-002: Keep negotiated terms no worse than published ones

- **Level:** SHOULD NOT
- **Applies when:** always
- **Category:** Fairness
- **Verify:** Read the negotiated agreements that the project makes available
  (or, when it offers none for review, its approved agreement templates and
  its discount rules). For each, compare each term with the published terms
  for the same thing (price, what is included, notice, term, grace period).
  A term worse than the published one fails. Then check the agreement
  against the other rules of this area: a term that removes or contradicts
  what another rule of this area asks for, such as dropping the cap of
  USS-COM-005 or the grace period of USS-COM-013, fails. Record `unverified`
  when the auditor can read neither the agreements nor the templates.
- **References:** none

A negotiated agreement SHOULD NOT have a term worse than the published terms
for the same thing, nor a term that sets aside another rule of this area.

**Rationale:** Volume and corporate agreements are welcome on top of the
published prices. They stop being fair when they are a way around the rules
that protect other customers, or when the published price is only a decoy.

### USS-COM-003: Sell each feature on its own

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Fairness
- **Verify:** List the features that the documentation, the pricing page and
  the licence entitlements name (each flag or entitlement that the code
  checks before it enables something). For each, find a published list price
  for buying it on its own. A feature with no stand-alone price fails.
- **References:** none

Each feature SHOULD be available to buy on its own.

**Rationale:** A customer who needs one feature should not pay for ten that
they do not use. Bundles can still be offered next to the single prices.

### USS-COM-004: Do not penalise buying features separately

- **Level:** SHOULD NOT
- **Applies when:** always
- **Category:** Fairness
- **Verify:** For each feature that is sold on its own and is also in one or
  more bundles, compare its stand-alone list price with the list price of the
  cheapest bundle that includes it, for the same unit and the same period. A
  feature whose stand-alone price is above that bundle price fails.
- **References:** none

A feature SHOULD NOT cost more on its own than the cheapest bundle that
includes it.

**Rationale:** A single price above a bundle that contains the feature makes
the single price a decoy: buying only what you need becomes a penalty.

## Price changes

### USS-COM-005: State a cap on price increases

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Fairness
- **Verify:** Open the published terms, each agreement template and each
  agreement that the auditor can read. Each states a cap on future price
  increases (a percentage, or a formula based on a named index). An agreement
  with no stated cap fails.
- **References:** none

Each agreement SHOULD state a cap on future price increases.

**Rationale:** A customer who builds a process on the software is locked in
over time. A stated cap lets them judge the cost of that over the years.

### USS-COM-006: Keep increases within inflation plus 3 points

- **Level:** SHOULD NOT
- **Applies when:** always
- **Category:** Fairness
- **Verify:** List the price increases announced since the last audit, or the
  last 12 months when there is none, from the price list history and the
  announcements. For each, find the inflation index that the terms or the
  announcement name (such as a consumer price index, CPI). Take the change in
  that index over the 12 months before the announcement, and add 3 percentage
  points. An increase above that sum, or an increase with no named index, fails.
- **References:** none

A price increase SHOULD NOT exceed a named inflation index (such as CPI) plus
3 percentage points a year.

**Rationale:** The figure of 3 points is a number that USS adds, to leave room
for the cost of keeping the software maintained over and above inflation. No
outside standard sets it; the indexes are examples, and each project names the
one that fits its market.

### USS-COM-007: Raise prices only at renewal

- **Level:** SHOULD NOT
- **Applies when:** always
- **Category:** Fairness
- **Verify:** List the price increases since the last audit, or the last 12
  months when there is none. For each, find the date from which it applies to
  existing customers, and the end date of the term that those customers were in:
  the term stated in their agreement, or for a subscription with no stated term,
  its current billing period. An increase that applies to an existing customer
  before the end of that term fails. An increase that applies only to new sales,
  or from a renewal, passes. Record `unverified` when the auditor cannot read
  the terms or the dates of the agreements.
- **References:** none

A price SHOULD NOT be increased during the term of an agreement.

**Rationale:** A customer agrees to a price for a term. Changing it in the
middle makes the agreement a one-sided option.

### USS-COM-008: Give 90 days' notice of price increases

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Fairness
- **Verify:** List the price increases since the last audit, or the last 12
  months when there is none. For each, find the date of its announcement to
  customers and the date from which it applies. An increase announced fewer than
  90 days before it applies fails.
- **References:** none

Each price increase SHOULD be announced at least 90 days before it applies.

**Rationale:** Ninety days is a figure that USS adds, enough for a customer to
plan a budget or to move to another product. The price increase is also a
change to pricing that the changelog announces (USS-TRU-005).

## Licence terms

### USS-COM-009: Do not penalise virtualisation

- **Level:** SHOULD NOT
- **Applies when:** publishes-releases
- **Category:** Fairness
- **Verify:** Read the licence terms and the licence check in the code, and
  list each resource that the licence metric counts (such as cores, sockets,
  memory, hosts or instances). Install the software in a virtual machine, and
  in a container, that has a part of the resources of its host. The count
  that the licence check reports follows the resources that the software can
  use there. A metric that counts the resources of the whole host, such as all
  its physical cores, when the software uses part of them, fails.
- **References:** none

Licence metrics SHOULD NOT count resources that the software does not use,
such as all the physical cores of a host that runs it in a virtual machine or
a container.

**Rationale:** A customer who runs the software in a small virtual machine on
a large host pays for the whole host under a physical metric, which punishes
the standard way of running software today.

### USS-COM-010: Show licence usage in the product

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Fairness
- **Verify:** Sign in to the product as an administrator of a test customer.
  For each metric of the licence (such as seats, cores or instances), a
  screen, a command or an API shows the current usage and the entitlement
  next to it. A metric with no such view fails.
- **References:** none

The product SHOULD show customers their current licence usage against their
entitlement.

**Rationale:** A customer who can see their usage is never surprised by a
true-up demand, and does not need an audit to learn where they stand.

### USS-COM-011: Offer an offline licence file

- **Level:** SHOULD
- **Applies when:** publishes-releases
- **Category:** Fairness
- **Verify:** Find in the documentation how a customer gets a licence as a
  file. Install the software on a machine with all outbound traffic blocked,
  apply a licence file, and use the licensed features: they work, with no
  connection to a licence service at any step. A licence that works only after
  a connection to a service fails.
- **References:** none

A licence SHOULD be deliverable as a file that works with no connection to a
licence service.

**Rationale:** Air-gapped and restricted networks are common where software is
bought. Without a licence file, the software cannot be used there at all.

### USS-COM-012: Make any licence server self-hostable

- **Level:** SHOULD
- **Applies when:** publishes-releases
- **Category:** Fairness
- **Verify:** Search the code and the configuration for each licence server
  that the software contacts. For each, find in the documentation how a
  customer runs it on their own infrastructure, and a setting that points the
  software at it. Install it as the documentation describes, point the
  software at it, and the licensed features work. A licence server that only
  the vendor can run fails.
- **References:** none

Any licence server that the software uses SHOULD be able to run on the
customer's own infrastructure.

**Rationale:** A licence server run by the vendor is a dependency that the
customer cannot control or secure. A customer who runs the server keeps the
software working whatever happens to the vendor's service.

### USS-COM-013: Allow a grace period when the licence service is unreachable

- **Level:** SHOULD
- **Applies when:** publishes-releases
- **Category:** Fairness
- **Verify:** Find the documented grace period of each licence service that
  the software contacts. It is at least 30 days. Then read the code and the
  configuration for the value that is applied, and test it: block the
  connection to the licence service after a valid licence check, and the
  licensed features keep working for the documented period. A period shorter
  than 30 days, one that is not documented, or software that stops sooner
  fails.
- **References:** none

When the software cannot reach a licence service, it SHOULD keep working for
a documented grace period of at least 30 days.

**Rationale:** Networks fail, and a vendor's service can fail too. The figure
of 30 days is a number that USS adds, long enough for a customer to fix a
network or a firewall, or to move to an offline licence, without an outage.

### USS-COM-014: Degrade gracefully when a licence expires

- **Level:** SHOULD
- **Applies when:** publishes-releases
- **Category:** Fairness
- **Verify:** Apply an expired licence to a test installation that holds
  data. A user can sign in, open and read the data that they had, such as
  with a switch to read-only. Software that stops, locks users out or
  hides their data fails.
- **References:** none

When a licence expires, the software SHOULD keep users' access to their data,
such as by switching to read-only, rather than stopping.

**Rationale:** An expired licence is usually a delay in a purchase order, not
a decision to leave. Stopping at once turns it into a loss of the customer's
work. The export of the data is held by USS-TRU-004.

## Licence models

### USS-COM-015: Offer several licence models

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Fairness
- **Verify:** List the licence models on the pricing page and in the licence
  terms. A model is a distinct rule for what a licence counts or how long it
  lasts, such as named user, concurrent, node-locked, subscription or
  perpetual. Fewer than three models that a customer can choose from fails.
- **References:** none

Customers SHOULD be able to choose from at least three licence models, such as
named user, concurrent, node-locked, subscription and perpetual.

**Rationale:** Customers use software in different ways: a fixed team, a pool
of occasional users, a single machine. The number three is a figure that USS
adds, to make "several" checkable. One model rarely fits them all.

### USS-COM-016: Offer a perpetual or perpetual-fallback licence

- **Level:** SHOULD
- **Applies when:** publishes-releases
- **Category:** Fairness
- **Verify:** Find in the pricing page and the licence terms either a
  perpetual licence, or a perpetual-fallback licence. For the second, test it
  with a licence whose subscription has ended: the latest version that was
  released before the end of the subscription keeps working, with no end
  date. A subscription that gives no use of any version after it ends fails.
- **References:** none

Customers SHOULD be able to buy a perpetual licence, or get a
perpetual-fallback licence that keeps the last version they paid for.

**Rationale:** A customer who has paid for a version should not lose it when
they stop paying for updates. Paid maintenance can still be sold next to it.

### USS-COM-017: Offer a licence for CI/CD

- **Level:** SHOULD
- **Applies when:** publishes-releases
- **Category:** Fairness
- **Verify:** Find in the pricing page and the licence terms a service-account
  licence for automation. Check four things. It uses no human seat: the
  licence usage (USS-COM-010) does not count it as a user. It works on an
  ephemeral, containerised runner with no node lock and no hardware
  fingerprint: start two fresh containers in turn with the same licence. It
  activates from a secret token with no interaction: the activation runs in a
  job with no terminal and no browser. Its list price is set for automation
  (such as per runner, per pipeline or per job) and is not the price of a
  human seat. Any of the four missing fails.
- **References:** none

A service-account licence for automation SHOULD use no human seat, work on
ephemeral and containerised runners without a node lock, activate from a
secret token without interaction, and be priced for automation.

**Rationale:** Build and test systems create and destroy machines all day and
have no person at a terminal. A licence made for people and fixed machines
either does not work there or costs a seat for each run.
