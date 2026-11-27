# USS follow-ups 4.9 and USS-DOC-007: weak references and vendor profile baselines (read 2026-11-25)

Method note: web search and fetch tools, `curl` with the default User-Agent, and read-only `gh api -X GET`. EU law was fetched with `Accept: application/xhtml+xml` from the ELI URI `http://data.europa.eu/eli/reg/2024/2847/oj`. The ELI resolver redirected to the EUR-Lex host and still returned the full XHTML (HTTP 200, no block), so the text below is the Official Journal text. I did not reach the Cellar host directly; this is a small deviation from "Cellar only". Nothing in the repository was changed except this file.

## 1. EN 301 549 (EAA module, USS-EAA-001)

### Takeaway
USS cites "EN 301 549 V4.1.1" in `standard/modules/regulations/eaa.md` (lines 33-37, 69, 77-78, 80, 86-91). V4.1.1 is real and is the latest version (published 2026-09-02), but it is NOT yet cited in the Official Journal under Directive (EU) 2019/882 as far as I could find; V3.2.1 is the only version cited (for Directive 2016/2102 only). The current "no reference was found" statement is therefore still right but unproven. Recommended: keep V4.1.1, change the reference to a text that was read, and state the OJ position with a date.

### Cited Findings
- USS cites it in eaa.md only (grep of `standard/` for "EN 301 549"; the text says "the full text was not read" and cites "listed by ETSI, 2026-09-02").
- The published document is "EN 301 549 V4.1.1 (2026-09), HARMONISED EUROPEAN STANDARD, Accessibility requirements for ICT products and services", by CEN, CENELEC and ETSI; 278 pages. I downloaded and read its cover, foreword, clause A.2 and Annex ZB listing. — [ETSI PDF](https://www.etsi.org/deliver/etsi_en/301500_301599/301549/04.01.01_60/en_301549v040101p.pdf)
- Its foreword says it was prepared under Commission request M/587 and gives "one voluntary means of conforming to the essential requirements of Directive (EU) 2019/882"; the new Annex ZB maps it to the Directive's Annex I; and "Once the present document is cited in the Official Journal of the European Union under that Directive, compliance ... confers ... a presumption of conformity". Clause A.2 repeats this and adds: "Presumption of conformity stays valid only as long as a reference ... is maintained in the list published in the Official Journal". — [ETSI PDF](https://www.etsi.org/deliver/etsi_en/301500_301599/301549/04.01.01_60/en_301549v040101p.pdf)
- It was developed from V3.2.1 (2021-03). — same PDF
- Do not use the search-result URL `.../04.01.00_20/en_301549v040100ev.pdf`; `_20` and the `ev` suffix mark the pre-publication (approval) draft. The published file is `04.01.01_60/en_301549v040101p.pdf` (HTTP 200 on 2026-11-25).
- Adopted 2026-08-24, published 2026-09-02; legal effect depends on the Official Journal citation; V3.2.1 stays the cited version until then. — [NDA Ireland](https://nda.ie/news/en301549-published); [Davis Wright Tremaine, 2026-09-18](https://www.dwt.com/insights/2026/09/european-accessibility-act-ict-standards-update)
- Expected OJ citation date: Deque first said about 2026-11-30, then updated the post to 2027-01-22. It is a vendor blog, not an official source. — [Deque](https://www.deque.com/blog/en-301-549-v4-1-1-is-final-what-changed-what-it-means-and-what-you-should-do/)
- The Commission's standards page still names V3.2.1 as the harmonised ICT version (harmonised 2021-08-18, for the Web Accessibility Directive). — [Commission, accessibility standardisation](https://commission.europa.eu/strategy-and-policy/policies/justice-and-fundamental-rights/disability/accessibility-standardisation_en)
- The M/587 request is the Commission Implementing Decision of 2022-09-14. — [CEPT copy](https://cept.org/documents/wg-nan/73985/nan-2022-11-033_mandate_587_-_sr_eaa)
- Germany's federal accessibility agency advises that, until harmonised standards are in the OJ, the current EN 301 549 and WCAG serve as guidance (reported in a search summary, not read at source).

### Recommendation (ONE change)
1. Cite the standard as: `EN 301 549 V4.1.1 (2026-09), Accessibility requirements for ICT products and services, ETSI/CEN/CENELEC (cover, foreword, clause A.2 and Annex ZB read 2026-11-25)`. Replace the "full text was not read" wording in the USS-EAA-001 References and in the module intro.
2. Replace the intro sentence "When this module was written, no reference ... was found" with a dated statement: `As of 2026-11-25 no reference to EN 301 549 under Directive (EU) 2019/882 was found in the Official Journal; V4.1.1 says that its presumption of conformity starts only when the Official Journal cites it (foreword, clause A.2). An implementer re-checks the list before relying on the presumption.` Keep the Rationale of USS-EAA-001 as is.
3. Before release, search the Official Journal once more (Commission implementing decisions on harmonised standards under 2019/882) and update the date. I did not find a reference; two searches of EUR-Lex-linked results found only unrelated 2026 decisions (2026/803, 2026/901, 2026/1736 contain neither "301 549" nor "2019/882").

Alternatives: (a) cite V3.2.1 as the "legally cited" version and accept V4.1.1 as a later one (wrong for the Directive: V3.2.1 has no EAA mapping); (b) drop the version from the Verify and say "the latest version"; (c) wait for the OJ citation and then reword (the citation could land 2026-11-30 to 2027-01-22, so a release of 0.1.0 could straddle it).

### Inferences
- "Not yet cited" rests on absence of evidence plus two secondary sources; no OJ list was read. The wording should keep "was found" phrasing, not "is not".
- The rule text "V4.1.1 or a later version ... A version earlier than V4.1.1 fails" is stricter than the law (V3.2.1 gives no presumption under the EAA either way). That is a USS choice and is fine, but the Rationale could say so.

### Gaps
- No official Commission or OJ page confirming absence or presence of the citation; the Commission's EAA harmonised-standards page returned 404 at the URL I guessed and search found no consolidated list.

## 2. USS-CRA-004 against Regulation (EU) 2024/2847

### Takeaway
The rule matches the law on the length (at least five years, shorter only for an expected use time under five years) and on the end date (month and year). The start of the five years is NOT stated by the Regulation; USS's "from the date the release was placed on the market" is a reading, and the follow-up is right to mark it so. Recommended: keep the reading but word the Verify and the rule as a reading with its legal anchor.

### Cited Findings
- Article 13(8): manufacturers "shall ensure, when placing a product with digital elements on the market, and for the support period, that vulnerabilities ... are handled effectively"; the support period must reflect the expected use time (reasonable user expectations, nature and intended purpose, relevant Union law on lifetime); "Without prejudice to the second subparagraph, the support period shall be at least five years. Where the product ... is expected to be in use for less than five years, the support period shall correspond to the expected use time." The manufacturer puts the information used to set the period in the technical documentation (Annex VII). The Commission may set minimum periods by delegated act. — [Regulation (EU) 2024/2847, OJ L, 20.11.2024, Art. 13(8), via ELI](http://data.europa.eu/eli/reg/2024/2847/oj)
- Article 13(19): the end date of the support period, "including at least the month and the year", must be "clearly and understandably specified at the time of purchase in an easily accessible manner and, where applicable, on the product ..., its packaging or by digital means"; where technically feasible a notification at the end of support is shown. — same
- Article 13(8) names no start date and no definition of "support period" exists; the word "from" never ties the period to a date in the Article. The only anchors are "when placing ... on the market, and for the support period". — same, text search of all 62 uses of "support period"
- Recital 60: support period "no less than five years, unless the lifetime ... is less than five years"; "Where the time ... reasonably expected to be in use is longer than five years ... manufacturers should accordingly ensure longer support periods"; a shorter period only where justified by the nature of the product (examples: a pandemic contact-tracing app, subscription-only software). Recital 59: the period reflects the expected use time, with proportionality. — same
- Recital 40 (subsequent versions): the manufacturer may give security updates for the support period only for the latest version of software if users get it free. — same
- Annex II point 7: users are told "the end-date of the support period during which users can expect vulnerabilities to be handled and to receive security updates". — same
- Related retention duty (Art. 13(13), (18)): documentation kept "at least 10 years after the product ... has been placed on the market or for the support period, whichever is longer" — here the Regulation does tie a period to the placing on the market, which supports the reading but is a different period. — same

### Recommendation (ONE change)
Keep the rule and Verify as written. Change the Rationale sentence to: `Article 13(8) sets a support period of at least five years (or the shorter expected use time) and requires vulnerability handling "when placing a product ... on the market, and for the support period", but it names no start date for the five years. USS counts them from the placing on the market of the release under audit; the Regulation does not say this, and a qualified person confirms it.` Also add Annex II point 7 and Recital 60 to the References line (`Article 13(8) and (19), Annex II point 7, Recital 60 (read 2026-11-25)`). No change to the rule level or Verify.

Alternatives: (a) require the end date to be at least five years after the first placing on the market of the product (more lenient, since later releases would not restart the clock); (b) drop the start-date test and check only "end date stated" plus a documented basis; (c) add a Standard Deviation mechanism note (not needed).

### Inferences
- The Verify counts from the "release under audit"; for a project that ships a new release every month the test is stricter than the law's "last placed" version logic in recital 40. A project may reasonably say the product, not each release, carries the period. This is the strongest reason to call the start a USS reading and keep the wording of alternative (a) in mind.
- Art. 13(8) also requires the period to reflect expected use; for operating-system-like software, five years is a floor and the rule's Verify (at least five years) is consistent with recital 60.

### Gaps
- No Commission guidance or ADCO statistics on support periods were read (the Commission/ADCO guidance is mentioned in the text; I did not search for it).

## 3. CIS Controls v8.1, Safeguard 11.3

### Takeaway
USS cites "CIS Critical Security Controls v8.1, safeguard 11.3" in `standard/core/backup.md` (lines 116, 222; 11.2 at 158; 11.5 at 264; also 179). The title and text are correct and confirmed at the publisher's own documentation site. A citation by the publisher's version is enough; the download form needs registration but the content is obtainable and quotable in short.

### Cited Findings
- Safeguard 11.3 is titled "Protect Recovery Data": "Protect recovery data with equivalent controls to the original data. Reference encryption or data separation, based on requirements." Asset class Data; security function Protect; IG1, IG2, IG3. — [CIS Controls Assessment Specification, Control 11 (cisecurity.org docs), read 2026-11-25](https://cas.docs.cisecurity.org/en/latest/source/Controls11/) (the page does not state which Controls version it documents)
- The v8.1 PDF, Excel, change log and implementation groups are offered from the CIS page "Download CIS Controls v8.1", which links to a form (first name, last name, organisation, email, sector; privacy notice). The page text does not state the licence. — [CIS Controls v8.1](https://www.cisecurity.org/controls/v8-1); [download form](https://learn.cisecurity.org/cis-controls-download)
- CIS also publishes "CIS Controls v8.1 Data Recovery Policy Template" on cisecurity.org. — [CIS white paper](https://www.cisecurity.org/insights/white-papers/controls-v8-1-data-recovery-policy-template)
- Third-party summaries (Netwrix, Tripwire) mislabel text: Netwrix puts the "tested quarterly" wording under 11.3 but it belongs to 11.5. Treat third-party copies as unreliable. — search summary, not read at source

### Recommendation (ONE change)
Cite it as: `CIS Critical Security Controls v8.1, Safeguard 11.3 "Protect Recovery Data" (CIS Controls Assessment Specification, read 2026-11-25)` and use that for the three places (backup.md lines 116, 222 and the 11.2 and 11.5 lines with their own titles). The registration form does not matter for the reference, because the Assessment Specification page is free and open, and the citation names the control number and title, which are stable. Do not rely on the Controls PDF text.

Alternatives: (a) keep "v8.1, safeguard 11.3" with no title; (b) cite the PDF and note "download needs a form"; (c) cite NIST SP 800-34 or ISO 27001 A.8.13 instead (changes the meaning).

### Inferences
- The v8.1 PDF's licence (reported by CIS as Creative Commons in the past) was not confirmed from the pages I read. I did not quote beyond one sentence.
- Check that the other two safeguards in the file (11.2 and 11.5) use the same page; I read only 11.3.

### Gaps
- I could not read the v8.1 PDF itself (registration form), so the claim "title and text are the same in the PDF" is unverified. The follow-up says it was "read through a third-party copy"; the Assessment Specification is the publisher's own, so this replaces the third-party copy.
- Release date of v8.1 not stated on the pages I read.

## 4. USS-AUD-009: source for CEF

### Takeaway
The current official document is "Implementing ArcSight Common Event Format (CEF) - Version 27", published by OpenText/Micro Focus with SmartConnectors 24.1 (January 2024), still unchanged in the 24.3 doc set (July 2024). I found no Version 28.

### Cited Findings
- The PDF cover reads: "ArcSight SmartConnectors, Software Version: 24.1, Implementing ArcSight Common Event Format (CEF) - Version 27, Document Release Date: January 2024"; copyright Open Text Corporation, Waterloo, Canada. — [PDF in SmartConnectors 24.1 docs](https://www.microfocus.com/documentation/arcsight/arcsight-smartconnectors-24.1/pdfdoc/cef-implementation-standard/cef-implementation-standard.pdf) (downloaded and read 2026-11-25)
- The 25.1 doc set's copy is still "Version 27", cover "Software Version: 24.3, Document Release Date: July 2024". — [PDF in SmartConnectors 25.1 docs](https://www.microfocus.com/documentation/arcsight/arcsight-smartconnectors-25.1/pdfdoc/cef-implementation-standard/cef-implementation-standard.pdf)
- The URLs for 25.2, 25.3 and 26.1 return 404 (checked 2026-11-25), so I found no later edition.
- Format header per the document: `CEF:Version|Device Vendor|Device Product|Device Version|Device Event Class ID|Name|Severity|[Extension]`; example `Sep 19 08:26:10 host CEF:0|Security|threatmanager|1.0|100|worm successfully stopped|10|...`. — same PDF
- Version 26 (February 2022) introduced "the 1.2 version of the CEF specification" (search summary of an older edition, not read at source).

### Recommendation (ONE change)
Add to the USS-AUD-009 References: `ArcSight Common Event Format (CEF) Implementation Standard, Version 27 (OpenText, January 2024)`. Also change the Verify's "a published specification such as syslog per RFC 5424, OCSF or CEF" to keep as is, and say in the Verify: `for CEF, a CEF parser that follows the header and extension rules of the Implementation Standard`.

Alternatives: (a) cite "CEF 1.2" (the format version inside the guide; the document edition is clearer); (b) drop CEF from the rule (it is a vendor format, not a standards-body one); (c) cite the ArcSight "SmartConnectors 24.1" help-centre HTML (`.../arcsight-smartconnectors-24.1/cef-implementation-standard/`, HTTP 200) with the PDF.

### Gaps
- No stable permalink; the Micro Focus documentation host redirects to OpenText over time. The PDF URL may move.
- Whether the "Version 27" is the newest at OpenText's own portal (docs.opentext.com) was not confirmed; only the microfocus.com paths were tested.

## 5. USS-DOC-007 and ASD-STE100

### Takeaway
The current issue is ASD-STE100 Issue 9 (January 2025, matches the USS reference). It is free, on request through a form on asd-ste100.org (not "from ASD-STAN"; the owner is ASD, the maintainer is the STEMG). It is copyright and trademark of ASD, all rights reserved, with no licence to quote or build tools; no free checker exists that ASD endorses. An audit cannot verify the approved-word list without the copy, so the rule's Verify (writing rules by the auditor, dictionary `unverified` without the specification) should stay, with a rewording that says who may hold the copy.

### Cited Findings
- Current issue: "Request a free official copy of the current issue of the ASD-STE100 standard ... (Issue 9, January 2025)". The request goes through a web form; the form collects statistics. — [STE Downloads](https://www.asd-ste100.org/STE_downloads.html); [ASD news on Issue 9](https://asd-europe.org/news-media/news-events/news/simplified-technical-english-asd-ste100-issue-9)
- Copyright and rights: "ASD-STE100 ... is a registered trademark of ASD. ... All rights are reserved." STEMG and ASD "do not issue individual permissions"; requests for permission "will not receive an individual reply"; "The Copyright Statement of the current edition remains the only reference"; use of the content, "including for building dedicated tools, some of them AI-based", is "at their sole risk and responsibility"; no tool may be presented as approved by STEMG or ASD. — [STE Downloads, notice about usage rights](https://www.asd-ste100.org/STE_downloads.html)
- STE has two parts: writing rules (grammar and style) and a controlled dictionary of approved words; users can add technical nouns and verbs. — [STE FAQ](https://www.asd-ste100.org/STE_faq.html)
- Tools: "Is a language checking tool necessary to write in STE? No ... no tool can replace the standard itself." ASD and STEMG do not endorse or certify any tool claimed to be "fully compliant". — [STE FAQ](https://www.asd-ste100.org/STE_faq.html)
- AI: the STEMG white paper warns that AI text can "appear clear, authoritative, and consistent with STE, even when it does not correctly apply the rules and vocabulary"; "Plausibility must not be confused with verified compliance". — [STE Downloads](https://www.asd-ste100.org/STE_downloads.html); [ASD white paper page](https://www.asd-europe.org/news-media/news-events/news/white-paper-asd-ste100-and-artificial-intelligence/)
- Checkers exist as commercial products only; I found one example: a term checker for Issue 9, £400 per user per year with a 30-day trial. — [simplified-english.co.uk](https://simplified-english.co.uk/) (price from search summary, not read at source)
- Quoting: a free copy is a distribution by the copyright holder, not an open licence (search summary of the STEMG site). The rules and word list therefore must not be copied into USS or into a project.

### Recommendation (ONE change)
Keep USS-DOC-007's Verify (rules checked by the auditor; dictionary conformance `unverified` unless the auditor holds a copy of Issue 9 or a checker) and change the last two sentences to: `Without a copy of ASD-STE100 Issue 9, the auditor checks the writing rules that this rule names and records conformance to the approved-word dictionary as unverified. With a copy (free from the owner on request) the auditor also checks each word of the sample against the dictionary. A checker's report counts only as evidence from that tool, not as ASD certification.` The rule itself stays SHOULD. Add to the References: `ASD-STE100 Simplified Technical English, Issue 9 (2025-01), free official copy on request at asd-ste100.org`.

Alternatives: (a) change the rule to refer to "the writing rules of USS-DOC-007 listed in the rule" (then the sentence limits are USS's own and no licence is needed; the dictionary becomes optional); (b) drop the dictionary part completely and call the rule "plain instructions"; (c) have the developer request the copy and mark each audit with its issue (the developer, not the agent, files the request because the form collects a name and an email).

### Inferences
- An agent can read a copy the developer provides in the audit's temporary folder, but it must not copy it into the repository or the report (all rights reserved; no permission is granted individually).
- Because the audit evidence line already says "unverified because the specification is not available", the current audit result is honest under either Verify wording.
- The rule's existing sentence-length limits (20 words in a step, 25 elsewhere) come from the STE writing rules; they are in the rule text, so quoting a number is not quoting the specification.

### Gaps
- The Copyright Statement inside the PDF was not read (no copy). Whether short quotations are allowed there is unknown. Issue-9-specific sentence limits (20 and 25) were not checked against the specification.
- "Free from ASD-STAN": I found no ASD-STAN page; the form is run by the STEMG at asd-ste100.org. The ASD-STAN name in the task is not supported by what I read.

## 6. Vendor profile baselines

### Takeaway
There are 17 profiles. Every profile's `Versions` lower bound equals the current release of its tool on 2026-11-25, so no tool is outside its range now (golangci-lint is v2.15.0, released 2026-10-16, which is the profile's lower bound; the local 2.12.2 run was the problem, not the profile). All can run on GitHub-hosted `ubuntu-latest` runners at no cost on a public repository. Recommended: one scheduled and path-triggered workflow with a matrix, one fixture per profile, and a script that extracts the single fenced block of each profile. Effort: about 3 to 5 days, plus ongoing fixes.

### Cited Findings: profiles, ranges and the current releases (read 2026-11-25)

| Profile (path under `standard/vendors/`) | Tool | `Versions` range | Current release | In range? | Source |
|---|---|---|---|---|---|
| astral-sh/ruff.md | Ruff | >=0.17.6, <0.18.0 | 0.17.6 (2026-11-19) | yes | [GitHub releases](https://github.com/astral-sh/ruff/releases), PyPI |
| embarkstudios/cargo-deny.md | cargo-deny | >=0.22.1, <0.23.0 | 0.22.1 (2026-11-24) | yes | [releases](https://github.com/EmbarkStudios/cargo-deny/releases) |
| eslint/eslint.md | ESLint, @eslint/js, typescript-eslint, @eslint-community/eslint-plugin-eslint-comments | 10.15.0 / 10.0.1 / 8.74.0 / 4.8.1 lower bounds, each <next major | 10.15.0 (2026-11-13), 10.0.1, 8.74.0 (2026-11-23), 4.8.1 | yes | releases and npm |
| facebook/eslint-plugin-react-hooks.md | eslint-plugin-react-hooks | >=7.1.1, <8.0.0 | 7.1.1 (npm) | yes | npm |
| golangci/golangci-lint.md | golangci-lint | >=2.15.0, <3.0.0 | v2.15.0 (2026-10-16) | yes | [releases](https://github.com/golangci/golangci-lint/releases) |
| google/error-prone.md | Error Prone | >=2.53.0, <3.0.0 | v2.53.0 (2026-11-18) | yes | [releases](https://github.com/google/error-prone/releases) |
| google/osv-scanner.md | OSV-Scanner | >=2.7.1, <3.0.0 | v2.7.1 (2026-11-09) | yes | [releases](https://github.com/google/osv-scanner/releases) |
| osprey-oss/deptry.md | deptry | >=0.25.1, <0.26.0 | 0.25.1 (2026-03-18) | yes | releases, PyPI |
| ossf/scorecard.md | Scorecard (and ossf/scorecard-action 2.6.0 named in the text) | >=5.7.0, <6.0.0 | v5.7.0 (2026-11-19) | yes | [releases](https://github.com/ossf/scorecard/releases) |
| oxc-project/oxlint.md | Oxlint | >=1.95.0, <2.0.0 | 1.95.0 (2026-11-23, npm and `oxlint_v1.95.0`) | yes | releases, npm |
| python/mypy.md | mypy | >=2.4.1, <3.0.0 | 2.4.1 (PyPI) | yes | [PyPI](https://pypi.org/project/mypy/) |
| rust-lang/clippy.md | Clippy (ships with Rust) | >=1.100.0, <1.101.0 | Rust 1.100.0 (2026-11-12) | yes | [Rust releases](https://github.com/rust-lang/rust/releases) |
| sbdchd/squawk.md | Squawk | >=2.74.0, <3.0.0 | v2.74.0 (2026-11-23) | yes | [releases](https://github.com/sbdchd/squawk/releases) |
| spotbugs/spotbugs.md | spotbugs-maven-plugin; findsecbugs-plugin | 4.10.5.0 / 1.14.0 lower bounds, each <next major | 4.10.5.0 (2026-10-12); findsecbugs-plugin 1.14.0 (Maven Central) | yes | releases, Maven Central |
| stackrox/kube-linter.md | kube-linter | >=0.8.3, <0.9.0 | v0.8.3 (2026-03-10) | yes | [releases](https://github.com/stackrox/kube-linter/releases) |
| uber/nullaway.md | NullAway | >=0.14.4, <0.15.0 | v0.14.4 (2026-11-16) | yes | [releases](https://github.com/uber/NullAway/releases) |
| webpro-nl/knip.md | knip | >=6.48.0, <7.0.0 | 6.48.0 (2026-11-22) | yes | releases, npm |

(Release data from read-only `gh api -X GET repos/<owner>/<repo>/releases/latest`, `npm view` and `pip index`, all on 2026-11-25.)

- Each profile has exactly one fenced code block (the baseline) in `## Baseline configuration` (count of "```" lines is 2 per file for all 17 profiles), which makes extraction in CI mechanical. Section 12 of conventions says a project may run a version outside `Versions` and then the auditor checks that each named rule exists. — repo `standard/conventions.md` lines 712-760
- Dependency-bumped tools will leave their range within days: nearly all ranges start at the newest release, so any new release inside `<next-minor` (ruff, cargo-deny, deptry, clippy 1.101.0 due about six weeks after 1.100.0, kube-linter, nullaway) makes the check useful but the range stale. (Inference from the dates above.)

### Which tools run free on GitHub-hosted runners
All 17 do on `ubuntu-latest` (free and unlimited minutes for public repositories; this repository is on GitHub and public, according to the audit's use of releases and `gh api`). Setup notes:
- Rust (Clippy 1.100.0, cargo-deny): `rustup toolchain install 1.100.0` (Clippy needs the matching toolchain, not "latest"); `cargo install --locked cargo-deny --version 0.22.1` or a release binary.
- Python (Ruff, mypy, deptry): `pip install ruff==0.17.6 mypy==2.4.1 deptry==0.25.1`.
- Node (ESLint set, react-hooks plugin, Oxlint, knip): `npm install --save-exact` the pinned versions; flat config for ESLint 10.
- Go (golangci-lint): release binary or `go install`; needs a Go module in the fixture.
- Java (Error Prone, NullAway, SpotBugs plus Find Security Bugs): Temurin JDK through `actions/setup-java`, then Maven; one fixture with all three plugins, or one per tool.
- Others (osv-scanner, kube-linter, squawk): release binaries from GitHub.
- OpenSSF Scorecard: the baseline is a workflow file for `ossf/scorecard-action`, not a tool configuration. It cannot be exercised on a fixture in the same way (the action runs on the repository itself and needs `id-token` and a token). Validate it with `actionlint` plus a YAML schema check, and test the CLI separately with `scorecard --repo=<the repo itself>` using `GITHUB_TOKEN`. (Inference.)

### Recommendation (ONE approach)
Add `.github/workflows/vendor-profiles.yml`, run on a weekly schedule, on `workflow_dispatch`, and on pull requests that change `standard/vendors/**` or the fixtures.

Layout:
```
tools/vendor-check/
  run.py                       # or an xtask subcommand: for a profile, read Versions and the one fenced block
  matrix.yaml                  # per profile: tool, install command, command to run, expected exit
  fixtures/<vendor>/<product>/
    clean/                     # a small project that must pass with exit 0 (a file or two)
    dirty/                     # a small project with one known violation per mapped rule that the profile claims to detect
    expected.txt               # rule codes or message ids that the dirty run must print
```
Procedure for each profile (matrix job per tool, `fail-fast: false`):
1. Parse the `Versions` header; install the lowest version inside the range (the lower bound), and, as a second matrix leg, the newest version in the range (the leg that says "range is stale" when the latest is outside the range).
2. Write the fenced baseline block to the tool's config file name in the fixture copy.
3. Run the tool on `clean/`; expect exit 0 and no config error (an unknown rule, key or option is an error in most of these tools).
4. Run the tool on `dirty/`; expect the codes in `expected.txt` (this checks the mappings' rule names exist and fire).
5. A "latest release" step opens an issue (read-only for contents; `issues: write`) when the newest version is outside the range, or the job fails when the check fails.
Pin every action by commit (USS-SUP-003), `permissions: contents: read`, and no secrets.

Effort: about 0.5 day for the extractor and the matrix harness; 0.25 to 0.5 day per ecosystem fixture (Rust 2, Python 3, Node 4, Go 1, Java 3, binaries 3, Scorecard 1) for about 3 to 4 days; 1 day for the scheduled-issue step and docs (AGENTS.md and a section in CONTRIBUTING). Total 4 to 6 days. Smaller first step: only the `clean/` run (config validity; 2 days), with `dirty/` added tool by tool.

Alternatives (one line each): (a) a single weekly job running only the `clean/` fixture with tool latest versions (cheapest, but no check at the profile's bound); (b) container images pinned per tool and run locally with `cargo xtask` (reproducible, slower and no scheduling); (c) a manual checklist before each release (the status quo; it already failed for golangci-lint).

### Inferences
- The only reason golangci-lint was "outside its range" was the local version (2.12.2): the profile says `>=2.15.0`. The profile text says the rule codes are those of golangci-lint 2.15.0 read from documentation, so the baseline was never executed; the CI job above is what closes this.
- A fixture proves the baseline configuration loads and the named rules exist. It does not prove the Coverage claims (what the mapped rules find); that stays a documentation reading.
- Rust profile ranges `>=1.100.0, <1.101.0` move every six weeks, so Clippy will leave its range soonest; consider widening the range only if the job shows the baseline is valid on the next version.

### Gaps
- I did not run any tool (research only, nothing installed), so no baseline was proven valid or invalid.
- eslint-plugin-react-hooks release was read from npm only (the GitHub "latest" for the React repository is the React release 19.3.1, not the plugin).
- The effort estimate is my estimate, not measured.
