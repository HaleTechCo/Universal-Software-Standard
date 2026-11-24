# Accessibility

- **Prefix:** ACC
- **Kind:** area
- **Applies when:** has-ui

This area covers use of the software's user interface by people with
disabilities and by people who use assistive technology. Its rules count
toward Accessibility & Usability. It is the one home of accessibility testing
in this standard: the Testing area has no rule for it. Speed of web pages
(USS-PRF-003) belongs to the Performance and efficiency area, and the
confirmation of destructive actions and the other rules of the interface
itself belong to the Usability (UX) area.

### USS-ACC-001: Meet WCAG 2.2 Level AA

- **Level:** MUST
- **Applies when:** always
- **Category:** Accessibility & Usability
- **Verify:** Take the screens, pages and dialogs that the user documentation
  describes (the main workflows) and the screens that those workflows pass
  through. For web content, check each one against every success criterion of
  WCAG 2.2 Level A and Level AA. For a UI that is not web content (such
  as a desktop, mobile or terminal UI), check it against the same criteria as
  WCAG2ICT applies them to non-web software. A criterion that a screen does not
  meet, and that is not marked as not applicable to it (for example, a
  criterion for video on a screen with no video), fails the rule.
- **References:** WCAG 2.2 (W3C Recommendation, 12 December 2024); WCAG2ICT
  (W3C Group Note, 11 December 2025)

Every user interface MUST meet WCAG 2.2 at Level AA; a UI that is not web
content MUST meet it as WCAG2ICT applies it.

**Rationale:** Level AA is the level that laws and procurement rules name most
often, and the criteria are free, tested and common to the whole industry. A
UI that people cannot perceive, operate or understand shuts them out of the
software. WCAG2ICT is the W3C's own reading of the criteria for software that
is not a web page, so the rule does not leave native and terminal UIs outside.

### USS-ACC-002: Check accessibility automatically in CI

- **Level:** MUST
- **Applies when:** has-web-ui
- **Category:** Accessibility & Usability
- **Verify:** Read the CI configuration: a job runs an automated accessibility
  checker (such as axe-core, in a browser test or through a command-line
  wrapper) on the pages of the web UI. List the pages the UI serves (from the
  routes, the sitemap or the list of screens in the user documentation): each
  is in the checker's input, or is a state of one that the tests open. A
  violation that the checker reports makes the job fail and so fails the build
  (the job sets no mode that reports without failing). In a test branch, add an
  image with no text alternative to a page: the job fails. A page that the
  checker does not run on, or a job that does not fail on a violation, fails.
- **References:** WCAG 2.2 (W3C Recommendation, 12 December 2024); axe-core
  (as an example of a checker, read 2026-11-24)

CI MUST run an automated accessibility checker (such as axe-core) on the UI's
pages, and a violation MUST fail the build.

**Rationale:** An automated checker finds only a part of the problems that
WCAG describes, but it finds that part at once, on every change, at the cost
of a few seconds. The manual tests of the next rule cover the rest.

### USS-ACC-003: Test by hand with a keyboard and a screen reader

- **Level:** MUST
- **Applies when:** always
- **Category:** Accessibility & Usability
- **Verify:** List the main workflows: those that the user documentation
  describes. Find the records of manual tests: each is dated, names the
  workflow, the method (keyboard only, or a named screen reader) and the
  result. Take the
  period since the last audit, or the last 12 months when there is none. The
  project passes when, for every release in the period, or for every calendar
  quarter in it, a record exists for every main workflow with both methods.
  A workflow with no record for a release or quarter, a record with no date,
  or a record of only one of the two methods fails.
- **References:** WCAG 2.2 (W3C Recommendation, 12 December 2024)

The main workflows MUST be tested by hand, with a keyboard only and with a
screen reader, at least once per release or once a quarter, and the dated
results MUST be recorded.

**Rationale:** Tools cannot judge whether the focus order makes sense, whether
a label says what a control does, or whether a screen reader user can finish a
task. Only a person with the keyboard or the screen reader can. A dated record
shows that the test happened and lets the next tester see what changed.

### USS-ACC-004: Respect reduced-motion settings

- **Level:** MUST
- **Applies when:** always
- **Category:** Accessibility & Usability
- **Verify:** List the animations and transitions of the UI (in the style
  sheets, the scripts and the platform animation calls). Mark each one that is
  not essential: removing it leaves every task possible and loses no
  information. Find where the UI reads the system's reduced-motion setting (for
  a web UI, the `prefers-reduced-motion` media query; for another UI, the
  platform's setting for reduced motion). Then, with the setting turned on in
  the operating system or emulated in the browser's developer tools, open the
  screens that hold the marked animations: each one is off, or replaced by a
  change with no movement. An animation that is not essential and that still
  moves, or a UI that never reads the setting, fails.
- **References:** CSS Media Queries Level 5 (W3C Working Draft, 19 February
  2026), 12.1 prefers-reduced-motion; WCAG 2.2 (W3C Recommendation, 12
  December 2024), 2.3.3 Animation from Interactions

When the system asks for reduced motion, the UI MUST turn off animation that
is not essential.

**Rationale:** For people with vestibular disorders, moving and scrolling
effects can cause dizziness and nausea. The setting is the way in which they
tell software about it. WCAG 2.2 asks for this at Level AAA (2.3.3), above the
level of the first rule of this area, so this rule makes it a requirement by
itself.

### USS-ACC-005: Publish an accessibility statement

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Accessibility & Usability
- **Verify:** Find the accessibility statement: a page of the UI or of the
  project's website, or a document that the UI links to. It names the standard
  and the level that the UI meets (such as WCAG 2.2 Level AA), has a section
  for known gaps (with each gap named, or an explicit statement that none is
  known), and tells users how to report a problem (a contact address, a form
  or an issue tracker). A statement with no standard or level, no known-gaps
  section, or no way to report a problem fails, and so does a UI with no
  statement that users can reach.
- **References:** W3C WAI, Developing an Accessibility Statement (updated
  2021-03-11)

An accessibility statement SHOULD name the standard and level that the UI
meets, its known gaps, and how to report a problem.

**Rationale:** People who rely on assistive technology want to know before
they start whether the software works for them, and whom to tell when it does
not. The statement is also the place where a project is honest about what it
has not yet done.
