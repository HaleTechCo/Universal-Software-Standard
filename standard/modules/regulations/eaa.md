# European Accessibility Act

- **Prefix:** EAA
- **Kind:** module
- **Applies when:** covered-by-eaa

This module covers Directive (EU) 2019/882 of 17 April 2019 on the accessibility
requirements for products and services (the European Accessibility Act),
published in the Official Journal of the European Union on 7 June 2019 (L
151/70). A directive binds through each member state's law: the member states
had to adopt their laws by 28 June 2022 and apply them from 28 June 2025
(Article 31(1) and (2)), so the rules that bind a project are those of the
member states where it offers its product or service. The Directive covers
products placed on the market after 28 June 2025 and services provided to
consumers after 28 June 2025 (Article 2(1) and (2)). A member state may apply
the rules on answering the emergency number 112 as late as 28 June 2027
(Article 31(3)); that does not concern software. Three exclusions matter for
software: a service provider may keep using products that it lawfully used
before 28 June 2025 for similar services until 28 June 2030, and a service
contract agreed before 28 June 2025 may continue without alteration until it
expires, but not beyond 28 June 2030 (Article 32(1)); a microenterprise (fewer
than 10 persons, and an annual turnover or balance sheet total of at most EUR 2
million) that provides services is exempt from the accessibility requirements
for services (Article 4(5) and Article 3(23)); and the requirements apply only
to the extent that they do not need a fundamental alteration of the product or
service or impose a disproportionate burden, which the operator assesses and
documents (Article 14). The Directive also leaves out pre-recorded time-based
media and office file formats published before 28 June 2025, third-party
content, and archives (Article 2(4)). Compliance with a harmonised standard
whose reference the Official Journal publishes gives a presumption of
conformity (Article 15(1)). When this module was written, no reference to EN
301 549 had been published for this Directive: EN 301 549 V3.2.1 (2021-03) is
referenced only for Directive (EU) 2016/2102 (Implementing Decision (EU)
2021/1339), and V4.1.1 is the latest version that the standards bodies have
published (ETSI lists it from 2 September 2026). The article and annex
references were read in the Official Journal text on 2026-11-24. The module
maps the obligations of the Directive to USS rules and adds three rules for
what the core rules leave open. The core rules that the module does not name
apply as written.

## Disclaimer

USS compliance is not legal compliance, and this module is not legal advice.
The module shows where USS helps meet the Directive and what is left. A
qualified person confirms each row, and what is left, for the product or
service, and for each member state where it is offered.

## Obligations

| Obligation | Source | Rules | Coverage |
|---|---|---|---|
| Meet the accessibility requirements | Art. 4(1)–(3); Annex I Sections I–IV | USS-ACC-001, USS-EAA-001 | partial: the hardware, self-service terminal and packaging requirements, the requirements for specific services (Section IV), and the functional performance criteria (Section VII) |
| Explain how the service meets them | Art. 13(2); Annex V points 1–3 | USS-EAA-002, USS-ACC-005 | partial: the oral format of the information, keeping it for as long as the service is in operation, and the information on how the delivery process and its monitoring ensure compliance |
| Accessible information and instructions | Annex I Section I point 1, Section II point (b) and Section III point (b) | USS-EAA-003 | partial: labelling on a product, packaging, the description of the user interface and of the interfaces with assistive devices, and information about a service's link to its products |
| Support services | Annex I Section I point 3 and Section III point (d) | none | none: a qualified person sets up support services that give accessibility information |
| Conformity, assessment and market surveillance | Arts. 7, 13(3)–(5), 14, 16–23 | none | none: a qualified person handles the conformity assessment, the EU declaration, the CE marking, the Article 14 assessment and the dealings with the authorities |

## Requirements

### USS-EAA-001: Meet EN 301 549

- **Level:** MUST
- **Applies when:** has-ui
- **Category:** Accessibility & Usability
- **Verify:** Take the screens, pages, documents and dialogs that the user
  documentation describes (the main workflows) and the screens that those
  workflows pass through. Find the version of EN 301 549 that the project
  names (V4.1.1 or a later version). For each one, take the requirements of
  that version that the standard says apply to its kind (web content,
  documents or software) and check each one. A requirement that is not met,
  and that the standard does not mark as not applicable to it, fails the rule.
  A requirement that the project's written Article 14 assessment shows is
  lifted (a fundamental alteration, or a disproportionate burden) is not
  checked; where the assessment is missing, the requirement is checked. A
  project that names no version, or a version earlier than V4.1.1, fails.
- **References:** Directive (EU) 2019/882, Official Journal L 151, 7.6.2019,
  Articles 2, 14 and 15 and Annex I (read 2026-11-24); EN 301 549 V4.1.1,
  ETSI (listed by ETSI, 2026-09-02; the full text was not read)

The product or service MUST meet the requirements of EN 301 549, V4.1.1 or a
later version, that apply to it, except to the extent that Article 14 of the
Directive lifts them (a fundamental alteration, or a disproportionate burden).

**Rationale:** Article 15 of the Directive presumes a product or service to be
in conformity when it meets a harmonised standard whose reference the Official
Journal publishes. No such reference to EN 301 549 was found for this
Directive on 2026-11-24, so the presumption does not yet exist, and the
Annex I requirements remain the legal test. EN 301 549 was drafted for them
under the Commission's request M/587, so meeting it is the nearest technical
route to those requirements. The rule names V4.1.1 because ETSI lists it as the
latest version. USS-ACC-001 asks for WCAG 2.2 for the user interface; this rule
asks for the wider scope of the standard, such as documents, software and
documentation.

### USS-EAA-002: Publish how the service meets the accessibility requirements

- **Level:** MUST
- **Applies when:** always
- **Category:** Accessibility & Usability
- **Verify:** Where the software is provided as a service, find the terms and
  conditions, or an equivalent public document or page. It describes the
  accessibility requirements that apply to the service, and says how the
  service meets them, with a general description of the service and the
  explanations that a reader needs to understand how it operates. A service
  with no such text, a text that names no requirement or says nothing about
  how the service meets them, or a text that users cannot reach, fails.
- **References:** Directive (EU) 2019/882, Official Journal L 151, 7.6.2019,
  Article 13(2) and Annex V point 1 (read 2026-11-24)

Where the software is provided as a service, the terms and conditions, or an
equivalent public document, MUST say how the service meets the applicable
accessibility requirements.

**Rationale:** A user who relies on assistive technology learns from this text,
before using the service, what the service does for them. The Directive asks
for the text in the general terms and conditions or an equivalent document.

### USS-EAA-003: Make product information and instructions accessible

- **Level:** MUST
- **Applies when:** always
- **Category:** Accessibility & Usability
- **Verify:** Find the information on the use of the software and its
  instructions (the user guide, the help pages, the installation and
  maintenance instructions). Each is publicly available, at the latest when
  the software is placed on the market or provided. Each is in a text format
  that can be used to generate other formats (such as HTML, or a PDF with a
  structure that assistive technology reads), and is available via more than
  one sensory channel (for example, text that assistive technology can present
  as speech or in braille, together with the visual presentation). Each item
  of non-text content in it (an image, a diagram, a video) comes with an
  alternative presentation. Information that is only an image of text, behind a
  login for non-users, or without an alternative for a non-text item, fails.
- **References:** Directive (EU) 2019/882, Official Journal L 151, 7.6.2019,
  Annex I Section I point 1(b), Section II point (b) and Section III point (b)
  (read 2026-11-24)

Information on the use of the software and its instructions MUST be publicly
available, when the software is placed on the market or provided, in text
formats that can be used to generate other formats, via more than one sensory
channel, with an alternative presentation of any non-text content.

**Rationale:** Instructions that a person cannot perceive or operate with their
assistive technology leave the accessibility features unknown. The reading of
"more than one sensory channel" as text that assistive technology can present
in other ways is USS's own, for a qualified person to confirm.
