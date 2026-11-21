# USS Foundation Implementation Plan (Plan 1 of 5)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the repository skeleton, the machine-readable framework of the
standard (registry, fact vocabulary, schemas, templates, conventions) and the
`uss` validator CLI with all its commands, so rule content can be written and
checked from Plan 2 on.

**Architecture:** One Go module at the repository root, so `go:embed` can
carry `standard/`, `schema/` and `templates/` into the binary (binary version =
standard version). The CLI lives in `tools/uss`; logic lives in small internal
packages: `expr` (applicability expressions), `catalog` (parse and lint the
standard), `record` (load and schema-validate project records and audit
results), `evaluate` (applicability, record checks, compliance, scores),
`report` (Markdown rendering), `release` (git checks). The Markdown parser is a
strict line-based parser for USS's own fixed format, not a general Markdown
library.

**Tech Stack:** Go 1.27.4 (installed); `go.yaml.in/yaml/v3`;
`github.com/santhosh-tekuri/jsonschema/v6`; golangci-lint v2 and govulncheck
run through `go tool`; GitHub Actions.

**Spec:** `docs/superpowers/specs/2026-11-21-universal-software-standard-design.md`

**Roadmap** (each later plan is written when the previous one is done; Plan 2's
review calibrates the content plans):

1. **This plan:** foundation and `uss` tool (spec §13 step 1).
2. Security and Supply chain areas, then **stop for the user's review**.
3. Remaining core areas, in batches.
4. Modules, vendor profiles and regulation modules, plus validator support for
   the profile and regulation mapping format.
5. AGENTS.md workflows, Claude plugin and skills, adopter templates
   (`AGENTS.snippet.md`, `templates/ci/`), CODEOWNERS, community files
   (CONTRIBUTING, SECURITY, CODE_OF_CONDUCT), CHANGELOG, this repo's
   `.uss/` self-audit, release 0.1.0.

## Global Constraints

- Keywords: exactly `MUST`, `MUST NOT`, `SHOULD`, `SHOULD NOT`, `MAY` (RFC 2119
  as clarified by RFC 8174). `SHALL`, `SHALL NOT`, `REQUIRED`, `RECOMMENDED`,
  `NOT RECOMMENDED`, `OPTIONAL` are never used. One rule, one level.
- Rule IDs: `USS-<PREFIX>-<NNN>`, prefix `^[A-Z][A-Z0-9]{1,7}$`, three digits.
  Never reused or renumbered; retired in place.
- Audit results: exactly `pass`, `fail`, `deviation`, `declined`,
  `unverified`, `not-applicable`.
- Score categories, in this order: Safety, Reliability, Efficiency,
  Maintainability, Contributability, Compatibility, Fairness,
  Accessibility & Usability, Features.
- Score per category = floor(passed × 100 ÷ applicable); `n/a` when nothing
  applies; equal weight per rule; **no overall score**.
- Compliant = every applicable MUST/MUST NOT `pass`; every applicable
  SHOULD/SHOULD NOT `pass` or `deviation`; every applicable MAY has a recorded
  decision; nothing `unverified`.
- Standard version starts at `0.1.0`; 0.x until the user declares 1.0.0.
- Dependencies: only those named in Tech Stack; any other needs a written
  reason in the commit message. Never modify system-level installs or global
  config; tools run through `go tool`.
- CI actions pinned by full commit SHA with the version in a comment;
  workflow `permissions: contents: read`.
- Must work on Windows, Linux and macOS. Repository text files use LF
  (`.gitattributes`), but the tool accepts CRLF and a UTF-8 BOM in any input.
- Check integrity applies to this repo's Go code: every `//nolint` names the
  linter and gives a reason (golangci-lint `nolintlint`).
- Step-by-step instructions in README files are written in ASD-STE100 style
  (one instruction per sentence, imperative, active voice).
- The content of `Initial_Idea.md` and `todo.txt` is never modified
  (line-ending normalisation by `.gitattributes` is the only allowed change).
- Licence: MIT, `Copyright (c) 2026 HaleTechCo`.

## Plan-level decisions (refining the spec, for the builder)

- `go.mod` sits at the repository root rather than in `tools/uss/`, because
  `go:embed` cannot reach outside its module. The Go code is still under
  `tools/uss/`. Install path: `github.com/HaleTechCo/Universal-Software-Standard/tools/uss@vX.Y.Z`.
- Area and module prefixes are declared in each standard file's header and
  checked for uniqueness by `uss lint`, rather than listed again in
  `conventions.md`. That avoids two sources of truth. Categories and stack IDs
  live in `standard/registry.yaml`.
- Facts set by a MAY (`Enabled by`) are added to `applicability.md` together
  with the rule that enables them (Plan 3). Plan 1 writes only the asked facts.
- Facts gain an `Implies` field (e.g. `has-web-ui` implies `has-ui`), so rules
  for `has-ui` also apply to web UIs without repeating `or` everywhere.
- Report template: `templates/audit-report.md.tmpl` (Go `text/template`).
- Audit results record the full 40-character SHA of the audited commit.
- The flag `--standard-root DIR` (a USS checkout) replaces the built-in copy
  for every command.

## Review Focus

1. **CRLF line endings** in standard files, records and results (Windows
   editors, adopters without `.gitattributes`): parsed the same as LF.
   Tests: Task 4 `TestParseDoc_CRLFAndBOM`, Task 5
   `TestParseRuleFile_CRLFAndBOM`, Task 7 `TestLoadRecord_CRLFAndBOM`.
2. **UTF-8 BOM** at the start of Markdown or YAML (Notepad): ignored. Same
   tests as 1.
3. **Headings, fields and keywords inside fenced code blocks**: not parsed as
   rules or fields, and not keyword-scanned. Tests: Task 4
   `TestParseDoc_Fences`, Task 5 `TestParseRuleFile_CodeFence`.
4. **Shallow CI clones** (`actions/checkout` defaults to depth 1), where the
   audited commit is missing: `release-check` says to fetch full history
   instead of passing on a raw git error. Test: Task 12
   `TestReleaseCheck_ShallowClone`.
5. **Windows path separators** in record paths (`docs\deviations\X.md`) and
   git's forward-slash output: deviation documents resolve on every OS, and
   `.uss/` detection works for nested components. Tests: Task 9
   `TestCheckRecord_BackslashPath`, Task 12 `TestOnlyUSSChanges`.

---

### Task 1: Repository bootstrap and CLI entry point

**Files:**
- Create: `go.mod`, `.gitattributes`, `.gitignore`, `.editorconfig`, `LICENSE`, `README.md`
- Create: `tools/uss/main.go`, `tools/uss/main_test.go`

**Interfaces:**
- Produces: `func run(args []string, stdout, stderr io.Writer) int` in
  `tools/uss/main.go`. Every later command is a case in its dispatch. Exit
  codes: `0` OK, `1` problems found, `2` usage error.

- [ ] **Step 1: Create `go.mod`**

```
module github.com/HaleTechCo/Universal-Software-Standard

go 1.27

toolchain go1.27.4
```

- [ ] **Step 2: Create repository hygiene files**

`.gitattributes`: `* text=auto eol=lf`. `.gitignore`: `/uss`, `/uss.exe`,
`/dist/`, `*.out`, `uss-report.md`. `.editorconfig`: root, utf-8, lf, final
newline, trim trailing whitespace; `[*.go]` tabs; `[*.{md,yaml,yml,json}]`
2 spaces.

- [ ] **Step 3: Renormalise line endings**

Run: `git add --renormalize . && git status --short`
Expected: only the new files appear; `Initial_Idea.md` and `todo.txt` content
is unchanged (`git diff --cached --ignore-cr-at-eol -- Initial_Idea.md todo.txt` prints nothing).

- [ ] **Step 4: Create `LICENSE` and `README.md`**

`LICENSE`: standard MIT text, `Copyright (c) 2026 HaleTechCo`. `README.md`:
title "Universal Software Standard (USS)", one paragraph from spec §1 (the
promise and the two uses), "Status: 0.x, in development; rules not yet
published", and a link to the design spec. No install steps yet (Plan 5).

- [ ] **Step 5: Write the failing tests**

```go
func TestRunNoArgsPrintsUsage(t *testing.T) {
	var out, errb bytes.Buffer
	if code := run(nil, &out, &errb); code != 2 { t.Fatalf("exit %d, want 2", code) }
	if !strings.Contains(errb.String(), "usage: uss <command>") { t.Fatalf("stderr %q", errb.String()) }
}
func TestRunUnknownCommand(t *testing.T) {
	var out, errb bytes.Buffer
	if code := run([]string{"bogus"}, &out, &errb); code != 2 { t.Fatalf("exit %d", code) }
	if !strings.Contains(errb.String(), `unknown command "bogus"`) { t.Fatalf("stderr %q", errb.String()) }
}
```

- [ ] **Step 6: Run the tests to verify they fail**

Run: `go test ./tools/uss/ -v`
Expected: FAIL (`run` undefined).

- [ ] **Step 7: Implement `run` and `main`**

`main` calls `os.Exit(run(os.Args[1:], os.Stdout, os.Stderr))`. The usage text
lists `lint`, `applicable`, `check`, `report`, `release-check`, `export`,
`version`, one line each.

- [ ] **Step 8: Run the tests to verify they pass**

Run: `go test ./tools/uss/ -v && go vet ./...`
Expected: PASS, no vet output.

- [ ] **Step 9: Commit**

```bash
git add -A
git commit -m "chore: bootstrap repository and uss CLI entry point"
```

---

### Task 2: Registry, embedded standard and `uss version`

**Files:**
- Create: `standard/registry.yaml`, `embed.go`
- Create: `tools/uss/internal/catalog/diag.go`, `registry.go`, `registry_test.go`
- Modify: `tools/uss/main.go`, `tools/uss/main_test.go`

**Interfaces:**
- Consumes: `run` (Task 1).
- Produces:
  - `package uss` at the repo root: `var Files embed.FS` with `//go:embed standard`
    (Task 7 adds `schema`, Task 11 adds `templates`).
  - `catalog.Diagnostic{File string; Line int; Message string}` with
    `String() string` → `"file:line: message"`, or `"file: message"` when
    `Line == 0`.
  - `catalog.Category{ID, Name string}`;
    `catalog.Registry{Version string; Categories []Category; Stack map[string][]string}`;
    `func LoadRegistry(fsys fs.FS) (Registry, []Diagnostic)` reading
    `standard/registry.yaml`; `func (r Registry) CategoryByName(name string) (Category, bool)`;
    `func (r Registry) HasStack(kind, id string) bool`.
  - In `main.go`: `func loadFS(standardRoot string) fs.FS` returning
    `uss.Files` when `standardRoot == ""`, else `os.DirFS(standardRoot)`.

- [ ] **Step 1: Create `standard/registry.yaml`**

```yaml
version: 0.1.0
categories:
  - { id: safety, name: Safety }
  - { id: reliability, name: Reliability }
  - { id: efficiency, name: Efficiency }
  - { id: maintainability, name: Maintainability }
  - { id: contributability, name: Contributability }
  - { id: compatibility, name: Compatibility }
  - { id: fairness, name: Fairness }
  - { id: accessibility-usability, name: Accessibility & Usability }
  - { id: features, name: Features }
stack:
  lang: [go, rust, typescript, javascript, python, java]
  framework: [react]
  platform: [docker-compose, kubernetes, helm, postgresql, sqlite, mariadb, mysql]
  tool: [maven]
```

- [ ] **Step 2: Add the YAML dependency**

Run: `go get go.yaml.in/yaml/v3@latest`
Expected: `go.mod` gains the requirement. If this module path does not
resolve, stop and report. Do not fall back to the archived `gopkg.in/yaml.v3`.

- [ ] **Step 3: Write the failing tests**

`TestLoadRegistry_Valid` (MapFS holding the real file contents): no
diagnostics; `Version == "0.1.0"`; `len(Categories) == 9`;
`Categories[7].Name == "Accessibility & Usability"`; `HasStack("lang","go")`;
`!HasStack("lang","cobol")`.

`TestLoadRegistry_Errors`, a table of `{yaml, wantSubstring}`:
`version: "1.0"` → `"version"`; duplicate category name → `"duplicate category"`;
stack kind `os` → `"unknown stack kind \"os\""`; unknown top-level key
`colour:` → `"field colour not found"`; missing file → `"standard/registry.yaml"`.

`TestRunVersion` in `main_test.go`: `run([]string{"version"})` → exit 0,
stdout `"uss 0.1.0\n"`.

- [ ] **Step 4: Run the tests to verify they fail**

Run: `go test ./tools/uss/... -v`
Expected: FAIL (undefined `LoadRegistry`, and `version` is an unknown command).

- [ ] **Step 5: Implement**

Decode strictly (`KnownFields(true)`). Version regex
`^\d+\.\d+\.\d+(-[0-9A-Za-z.-]+)?$`. Stack kinds: `lang`, `framework`,
`platform`, `tool`. IDs kebab-case `^[a-z0-9]+(-[a-z0-9]+)*$`. `version`
prints `uss <Registry.Version>`.

- [ ] **Step 6: Run the tests to verify they pass**

Run: `go test ./tools/uss/... -v`
Expected: PASS.

- [ ] **Step 7: Commit**

```bash
git add -A
git commit -m "feat(uss): add standard registry, embedded files and version command"
```

---

### Task 3: Applicability expressions

**Files:**
- Create: `tools/uss/internal/expr/expr.go`, `tools/uss/internal/expr/expr_test.go`

**Interfaces:**
- Produces:
  - `type Expr interface { Eval(facts map[string]bool) bool; Idents() []string; String() string }`
  - `func Parse(s string) (Expr, error)`
  - `var Always Expr`
  - `func And(a, b Expr) Expr`, with `And(Always, e)` and `And(e, Always)` returning `e`.
    Otherwise `String()` is `"(" + a + ") and (" + b + ")"`.
  - `Idents()` returns the sorted unique identifiers, excluding `always`.

Grammar (pinned):

```
expr  = and { "or" and }
and   = unary { "and" unary }
unary = "not" unary | atom
atom  = "always" | ident | "(" expr ")"
ident = fact | kind "." fact
kind  = "lang" | "framework" | "platform" | "tool"
fact  = [a-z0-9]+ ( "-" [a-z0-9]+ )*
```

Keywords are lowercase only; tokens are separated by whitespace or parentheses.

- [ ] **Step 1: Write the failing tests**

`TestEval`, a table of `{src, facts, want}`: `"always"`→true;
`"has-ui"` with `{has-ui:true}`→true; `"a or b and c"` with `{b:true}`→false
(and binds tighter); `"not a and b"` with `{b:true}`→true; `"(a or b) and c"`
with `{a:true,c:true}`→true; `"lang.go"` with `{lang.go:true}`→true; a missing
fact is false.

`TestParseErrors`, each of these must fail: `""`, `"a and"`, `"(a"`, `"a b"`,
`"AND"`, `"has_ui"`, `"lang."`, `"os.linux"` (message contains
`unknown stack kind "os"`).

`TestIdents`: `Parse("not b or (a and lang.go) or always").Idents()` ==
`[]string{"a","b","lang.go"}`.

`TestAndSimplifies`: `And(Always, e).String() == e.String()`.

- [ ] **Step 2: Run the tests to verify they fail**

Run: `go test ./tools/uss/internal/expr/ -v`
Expected: FAIL (package has no code).

- [ ] **Step 3: Implement the recursive-descent parser and AST in `expr.go`**

- [ ] **Step 4: Run the tests to verify they pass**

Run: `go test ./tools/uss/internal/expr/ -v`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add tools/uss/internal/expr
git commit -m "feat(uss): parse and evaluate applicability expressions"
```

---

### Task 4: Markdown scanner and fact vocabulary

**Files:**
- Create: `tools/uss/internal/catalog/markdown.go`, `markdown_test.go`
- Create: `tools/uss/internal/catalog/facts.go`, `facts_test.go`
- Create: `standard/applicability.md`

**Interfaces:**
- Produces (package-internal, used by Tasks 5 and 6):
  - `type fieldVal struct{ Value string; Line int }`; `type fields map[string]fieldVal`
  - `type mdSection struct{ Heading string; Line int; Fields fields; Body string; BodyLine int }`
  - `type mdDoc struct{ File, Title string; Header fields; Sections []mdSection }`
  - `func parseDoc(file string, data []byte) (mdDoc, []Diagnostic)`
- Produces (exported):
  - `type Fact struct{ ID, Question, Definition, EnabledBy string; Implies []string; File string; Line int }`
  - `func parseFacts(file string, data []byte) (map[string]Fact, []Diagnostic)`

Scanner rules (pinned):
- Strip a leading UTF-8 BOM; turn `\r\n` into `\n`.
- Lines starting with three backticks or `~~~` open and close a fence. Inside
  a fence nothing is a heading or a field; the lines belong to the body.
- The first `# ` line is the title. The header field block follows it.
- `## ` lines are grouping headings: they end the current section and are
  otherwise ignored.
- `### ` lines start a section.
- A field block starts on the first non-blank line after a heading and
  continues while lines match `^- \*\*([A-Za-z ]+):\*\* (.*)$` or are
  continuation lines (two-space indent, non-empty; appended with one space).
- A duplicate field name gives the diagnostic `duplicate field "<name>"`.

Facts file rules: `### <fact-id>`, with fields `Question`, `Definition`,
`Enabled by` and `Implies` (comma-separated) only. `Definition` is required.
Exactly one of `Question` or `Enabled by` is present. Implied facts must
exist, and implications must not form a cycle.

- [ ] **Step 1: Write the failing scanner tests**

`TestParseDoc_Basic`: title, header fields, two sections with fields and body,
and a `##` grouping heading between them that is not a section.
`TestParseDoc_Fences`: a section body containing a fenced block with
`### USS-X-001: y` and `- **Level:** MAY` → one section, and the fenced lines
appear verbatim in `Body`. `TestParseDoc_CRLFAndBOM`: the same input as `_Basic`
with BOM and CRLF → a `mdDoc` deep-equal to `_Basic`'s (except `File`).
`TestParseDoc_Continuation`: a two-line `Verify` value joined with one space.
`TestParseDoc_DuplicateField` → diagnostic with line number.

- [ ] **Step 2: Run to verify they fail; implement `markdown.go`; run to verify they pass**

Run: `go test ./tools/uss/internal/catalog/ -run TestParseDoc -v`
Expected: FAIL before implementation, PASS after.

- [ ] **Step 3: Write the failing fact tests**

`TestParseFacts_Valid`: two facts, with `has-web-ui` implying `has-ui`.
`TestParseFacts_Errors`, a table: bad ID `Has_UI`; both `Question` and
`Enabled by`; neither; `Implies: nope` → `unknown fact "nope"`; cycle `a`→`b`→`a`
→ `implication cycle`; unknown field `Colour` → `unknown field "Colour"`.
`TestApplicabilityFile`: `parseFacts` on the embedded
`standard/applicability.md` gives zero diagnostics and exactly the 27 IDs in
Step 5.

- [ ] **Step 4: Implement `facts.go`**

- [ ] **Step 5: Write `standard/applicability.md`**

H1 "Applicability facts". An intro paragraph explains that facts are yes/no
statements about a project, that agents infer them from the code and then
confirm them with the developer, and that rules use them in `Applies when`.
Then one `###` section per fact. Each has a `Question` (one short question)
and a `Definition` (when the answer is yes). `Implies` is given where listed:

| Fact | Implies | Yes when |
|---|---|---|
| has-ui | | people interact with it through any UI (GUI, web, TUI) |
| has-web-ui | has-ui | it serves a UI in a web browser |
| has-cli | | it has a command-line interface |
| stores-persistent-data | | it keeps data across restarts |
| stores-user-data | stores-persistent-data | it stores data about or created by its users |
| uses-database | stores-persistent-data | it uses a database engine |
| uses-relational-database | uses-database | that database is relational (SQL) |
| exposes-network-api | | it accepts requests over a network |
| is-network-service | exposes-network-api | it runs as a long-lived network service others connect to |
| operates-service | | the project itself runs a production deployment |
| handles-authentication | | it authenticates users or clients |
| handles-sensitive-data | | it handles personal, financial, health, credential or similar data |
| high-assurance | | failure could cause serious harm (safety, critical infrastructure, large financial loss) |
| uses-cryptography | | it encrypts, signs, hashes for security, or manages keys |
| requires-fips | uses-cryptography | it must run in a FIPS 140-3 validated configuration |
| ships-container-image | | it publishes an OCI container image |
| deploys-to-kubernetes | | it ships Kubernetes manifests or Helm charts |
| is-library | | others use it as a library or SDK |
| distributed-as-package | | it is released as an installable package or binary |
| accepts-external-contributions | | people outside the core team may contribute |
| uses-ai-models | | it calls or runs AI models |
| exposes-mcp-server | | it provides an MCP server |
| has-multiple-users | | more than one person or account uses one installation |
| is-commercial | | any edition, tier, feature or licence is sold |
| is-foss | | it is released under an OSI-approved licence |
| sold-in-eu | | it is placed on the EU market |
| processes-eu-personal-data | | it processes personal data of people in the EU |

- [ ] **Step 6: Run all catalog tests**

Run: `go test ./tools/uss/internal/catalog/ -v`
Expected: PASS.

- [ ] **Step 7: Commit**

```bash
git add -A
git commit -m "feat(uss): add Markdown scanner and applicability fact vocabulary"
```

---

### Task 5: Rule file parsing

**Files:**
- Create: `tools/uss/internal/catalog/rules.go`, `rules_test.go`

**Interfaces:**
- Consumes: `parseDoc` (Task 4), `expr.Parse` / `expr.And` (Task 3).
- Produces:
  - `type Level string` with constants `Must = "MUST"`, `MustNot = "MUST NOT"`,
    `Should = "SHOULD"`, `ShouldNot = "SHOULD NOT"`, `May = "MAY"`;
    `func ParseLevel(s string) (Level, bool)`.
  - `type Rule struct{ ID, Title string; Level Level; AppliesWhen string; Applies expr.Expr; Category, Verify, References string; Enables, Refines []string; Retired bool; RetiredIn, ReplacedBy string; Body string; Kind string; File string; Line int }`.
    `Applies` is `expr.And(file Applies, rule Applies)`; `Kind` is copied from the file.
  - `type RuleFile struct{ File, Title, Prefix, Kind, AppliesWhen string; Applies expr.Expr; Rules []Rule }`
  - `func parseRuleFile(file string, data []byte) (RuleFile, []Diagnostic)`

File-local checks (pinned):
- Header: `Prefix` (required, `^[A-Z][A-Z0-9]{1,7}$`), `Kind` (required,
  `area` or `module`), `Applies when` (optional, default `always`).
- Every `###` heading matches `^(USS-([A-Z][A-Z0-9]{1,7})-(\d{3})): (.+)$`,
  otherwise `heading is not a rule`. The prefix equals the file's `Prefix`.
- Allowed fields: Level, Applies when, Category, Verify, References, Enables,
  Refines, Status, Retired in, Replaced by.
- Active rules require Level, Applies when, Category, Verify, References, and
  a non-empty body. Retired rules (`Status: Retired`) require `Retired in`
  (semver) and `Replaced by` (a rule ID or `none`).
- `Enables` only when the level is `MAY`. `Refines` only when the file kind is
  `module`.
- **Keyword scan** of the body, outside fences, with the regex
  `\b(MUST NOT|SHOULD NOT|SHALL NOT|NOT RECOMMENDED|MUST|SHOULD|SHALL|MAY|REQUIRED|RECOMMENDED|OPTIONAL)\b`:
  a forbidden synonym gives `forbidden keyword "<kw>"`; any keyword other than
  the rule's level gives `keyword "<kw>" in a <level> rule`; the level keyword
  must appear at least once, otherwise `body does not state the rule with "<level>"`.

- [ ] **Step 1: Write the failing tests**

`TestParseRuleFile_Valid`: the spec §5.2 example (`USS-SEC-012`, MUST,
`uses-database`, Safety) in a file with `Prefix: SEC`, `Kind: area` → one rule
with each field equal to the spec's values.
`TestParseRuleFile_ModuleAppliesWhen`: a module file with
`Applies when: lang.go` and a rule with `always` → `Applies.String() == "lang.go"`.
`TestParseRuleFile_Errors`, a table with one bad input per check above, each
with its expected message substring.
`TestParseRuleFile_CodeFence`: a rule body with a fenced example containing
`### USS-SEC-999: x` and `SHOULD` → one rule, no diagnostics.
`TestParseRuleFile_CRLFAndBOM`: the `_Valid` input with BOM and CRLF → equal
`Rule` values. `TestParseRuleFile_Retired`: a retired rule without Verify is
valid; one missing `Replaced by` is not.

- [ ] **Step 2: Run to verify they fail**

Run: `go test ./tools/uss/internal/catalog/ -run TestParseRuleFile -v`
Expected: FAIL.

- [ ] **Step 3: Implement `rules.go`**

- [ ] **Step 4: Run to verify they pass**

Run: `go test ./tools/uss/internal/catalog/ -v`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add tools/uss/internal/catalog
git commit -m "feat(uss): parse and check rule files"
```

---

### Task 6: Catalog loading, cross-file lint and `uss lint`

**Files:**
- Create: `tools/uss/internal/catalog/catalog.go`, `catalog_test.go`
- Create: `tools/uss/internal/testfixture/fixture.go`
- Modify: `tools/uss/main.go`, `tools/uss/main_test.go`

**Interfaces:**
- Consumes: `LoadRegistry`, `parseFacts`, `parseRuleFile`.
- Produces:
  - `type Catalog struct{ Registry Registry; Facts map[string]Fact; Files []RuleFile; Rules []Rule }`.
    `Rules` holds every rule, retired ones included, sorted by ID.
  - `func Load(fsys fs.FS) (*Catalog, []Diagnostic)`. It reads
    `standard/registry.yaml`, `standard/applicability.md`, every `.md` under
    `standard/core/` (Kind must be `area`), and every `.md` under
    `standard/modules/` recursively (Kind must be `module`). A missing
    directory is not an error. Diagnostics are sorted by File, then Line.
  - `func (c *Catalog) Rule(id string) (Rule, bool)`
  - `func (c *Catalog) KnownIdent(id string) bool`: true for a defined fact or
    a `kind.id` whose ID is in the registry stack.
  - `testfixture.Standard() fstest.MapFS`: a valid mini-standard used by all
    later tests (below).
  - CLI: `uss lint [--standard-root DIR]`.

Cross-file checks: duplicate rule IDs; the same prefix in two files; an
unregistered category; an expression identifier that `KnownIdent` rejects; an
`Enables` fact that does not exist or whose `Enabled by` differs; a fact whose
`Enabled by` is not a MAY rule listing it in `Enables`; a `Refines` target that
is missing or not in an `area` file; a `Replaced by` target that is missing.

Fixture `testfixture.Standard()`: registry version `0.1.0`; categories
Safety and Features; stack `lang: [go]`; facts `has-ui`, `uses-database`,
`has-web-ui` (implies `has-ui`) with Questions, and `has-plugin-interface`
(`Enabled by: USS-EXT-001`). Rules:

| ID | Level | Applies when | Category | Notes |
|---|---|---|---|---|
| USS-SEC-001 | MUST | always | Safety | |
| USS-SEC-002 | SHOULD | uses-database | Safety | |
| USS-SEC-003 | MUST NOT | has-ui | Safety | |
| USS-SEC-004 | (retired) | | | Replaced by USS-SEC-001 |
| USS-EXT-001 | MAY | always | Features | Enables has-plugin-interface |
| USS-EXT-002 | MUST | has-plugin-interface | Features | |
| USS-GO-001 | MUST | always (file: lang.go) | Safety | module, Refines USS-SEC-001 |

- [ ] **Step 1: Write the failing tests**

`TestLoad_Fixture`: no diagnostics; 7 rules; `Rules[0].ID == "USS-EXT-001"`;
`Rule("USS-GO-001").Applies.String() == "lang.go"`.
`TestLoad_CrossFileErrors`: a table that alters the fixture once per
cross-file check, each with its expected message substring.
`TestRunLint_Embedded`: `run([]string{"lint"})` → exit 0, and stdout starts
with `"uss lint: ok ("` (counts change as rules are added in later plans).
`TestRunLint_StandardRoot`: write the fixture to `t.TempDir()` with one rule
missing `Verify` → exit 1, stdout contains `standard/core/security.md:` and
ends with `uss lint: 1 problem\n`.

- [ ] **Step 2: Run to verify they fail**

Run: `go test ./tools/uss/... -v`
Expected: FAIL.

- [ ] **Step 3: Implement `Load`, the fixture and the `lint` command**

Output: one diagnostic per line, then `uss lint: ok (N rules, M files)` or
`uss lint: N problem(s)` ("problem" when N is 1, "problems" otherwise).

- [ ] **Step 4: Run to verify they pass**

Run: `go test ./tools/uss/... -v`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add -A
git commit -m "feat(uss): load the catalog and add the lint command"
```

---

### Task 7: Schemas, records and audit results

**Files:**
- Create: `schema/record.schema.json`, `schema/audit-results.schema.json`
- Modify: `embed.go` (pattern `standard schema`)
- Create: `tools/uss/internal/record/record.go`, `results.go`, `record_test.go`, `testdata/`

**Interfaces:**
- Produces (package `record`):
  - `type StandardRef struct{ Version string }`; `type Project struct{ Name string }`
  - `type Stack struct{ Languages, Frameworks, Platforms, Tools []string }`
  - `type Decision struct{ Adopt bool }`
  - `type Deviation struct{ Rule, Document, ApprovedBy, Date, ReviewBy string }`
    (YAML keys `rule`, `document`, `approved-by`, `date`, `review-by`)
  - `type AuditRef struct{ Date, Results, Report string }`
  - `type Record struct{ Standard StandardRef; Project Project; Stack Stack; Facts map[string]bool; Decisions map[string]Decision; Deviations []Deviation; Audits []AuditRef; Components []string; Path, Root string }`.
    `Path` and `Root` are not read from YAML. `Root` is the parent of the
    `.uss` directory holding the record; if the record is not inside `.uss`,
    it is the record's own directory.
  - `func LoadRecord(uss fs.FS, path string) (*Record, error)`. Schemas come
    from `uss` at `schema/*.json`.
  - `func (r *Record) LatestAudit() (AuditRef, bool)`: the greatest `Date`;
    on a tie, the later list entry.
  - Result constants `Pass`, `Fail`, `Deviation`, `Declined`, `Unverified`,
    `NotApplicable` with the Global Constraints values.
  - `type Result struct{ Result, Evidence string }`;
    `type Suppression struct{ File string; Line int; Rule, Reason string }`
  - `type Results struct{ Standard StandardRef; Commit, Date, Auditor string; Results map[string]Result; Suppressions []Suppression; Path string }`
  - `func LoadResults(uss fs.FS, path string) (*Results, error)`

Schema content (draft 2020-12, `additionalProperties: false` everywhere, `$id`
under `https://github.com/HaleTechCo/Universal-Software-Standard/schema/`):
- record: required `standard`, `project`, `facts`. Version uses the semver
  pattern. Facts have kebab-case keys and boolean values. Decision keys use
  the rule-ID pattern, with values `{adopt: boolean}` (required). Deviation
  items require `rule`, `document`, `approved-by`, `date`. Dates match
  `^\d{4}-\d{2}-\d{2}$`. Audit items require `date`, `results`, `report`.
  `components` and stack arrays hold unique strings.
- audit-results: required `standard`, `commit`, `date`, `results`. `commit`
  matches `^[0-9a-f]{40}$`. Result keys use the rule-ID pattern; `result` is
  the six-value enum. `evidence` is required (minLength 1) when the result is
  `pass`, `fail` or `deviation`. Suppression items require `file`, `rule`,
  `reason`; `line` is an integer ≥ 1.

Loading: decode YAML into `any`, normalise it to JSON-compatible values
(`time.Time` becomes `YYYY-MM-DD`), validate against the schema, then decode
strictly into the struct. Errors name the JSON pointer of each failure.

- [ ] **Step 1: Add the schema dependency**

Run: `go get github.com/santhosh-tekuri/jsonschema/v6@latest`
Expected: `go.mod` updated.

- [ ] **Step 2: Write the failing tests**

`TestLoadRecord_Valid` (`testdata/record-valid.yaml`, with an unquoted
`date: 2026-11-21`): fields populated; `Deviations[0].Date == "2026-11-21"`.
`TestLoadRecord_Root`: a record at `<tmp>/proj/.uss/record.yaml` →
`Root == <tmp>/proj`. `TestLoadRecord_CRLFAndBOM`: the same file with BOM and
CRLF → an equal `Record` (except `Path`/`Root`). `TestLoadRecord_SchemaErrors`,
a table: unknown top-level key `colour` → error contains `colour`; `has-ui: "yes"` →
`"/facts/has-ui"`; decision key `SEC-1` → `"/decisions"`; deviation without
`approved-by` → `"/deviations/0"`; `date: 21/11/2026` → `"/deviations/0/date"`.
`TestLoadResults_Valid`; `TestLoadResults_Errors`: commit `1ab35de` → `"/commit"`;
result `passed` → `"/results/USS-SEC-001/result"`; `pass` without evidence →
`"/results/USS-SEC-001"`. `TestLatestAudit`: three entries, tie on the latest
date → the later entry.

- [ ] **Step 3: Run to verify they fail**

Run: `go test ./tools/uss/internal/record/ -v`
Expected: FAIL.

- [ ] **Step 4: Write the schemas, extend `embed.go`, implement `record.go` and `results.go`**

- [ ] **Step 5: Run to verify they pass**

Run: `go test ./tools/uss/... -v`
Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add -A
git commit -m "feat(uss): add record and audit-results schemas and loaders"
```

---

### Task 8: Fact resolution and `uss applicable`

**Files:**
- Create: `tools/uss/internal/evaluate/facts.go`, `facts_test.go`
- Modify: `tools/uss/main.go`, `tools/uss/main_test.go`

**Interfaces:**
- Consumes: `catalog.Catalog`, `record.Record`.
- Produces:
  - `type Facts map[string]bool`
  - `func ResolveFacts(c *catalog.Catalog, r *record.Record) Facts`
  - `func Applicable(c *catalog.Catalog, r *record.Record) []catalog.Rule`:
    active rules whose `Applies` is true under `ResolveFacts`, sorted by ID.
  - In `main.go`: `func versionError(pinned, carried, command, arg string) string`
    returning
    `record pins USS <pinned>; this uss carries USS <carried>. Run: go run github.com/HaleTechCo/Universal-Software-Standard/tools/uss@v<pinned> <command> <arg>`.
  - CLI: `uss applicable [--standard-root DIR] [--json] RECORD`.

Resolution (pinned): start from the record's facts; add `lang.<x>` for each
`Stack.Languages`, `framework.<x>`, `platform.<x>` and `tool.<x>` likewise.
Then repeat until nothing changes: add `Implies` closures; for each active MAY
rule that applies and has `Decisions[id].Adopt == true`, set its `Enables`
facts. Unset facts are false.

Output: one line per rule, `ID<TAB>LEVEL<TAB>Category<TAB>Title`. With
`--json`: an array of `{"id","title","level","category","file"}`.

- [ ] **Step 1: Write the failing tests** (all on `testfixture.Standard()`)

`TestResolveFacts`: `{has-web-ui:true}` → `has-ui` true; stack `[go]` →
`lang.go` true; `has-plugin-interface` true only with `USS-EXT-001` adopted.
`TestApplicable`: all facts false, no stack, no decisions → IDs
`[USS-EXT-001, USS-SEC-001]`; with `USS-EXT-001` adopted → adds `USS-EXT-002`;
with stack `[go]` → adds `USS-GO-001`; `USS-SEC-004` (retired) never appears.
`TestRunApplicable_VersionMismatch`: record pins `9.9.9` → exit 1, stderr
contains `tools/uss@v9.9.9 applicable`.

- [ ] **Step 2: Run to verify they fail; implement; run to verify they pass**

Run: `go test ./tools/uss/... -v`
Expected: FAIL before implementation, PASS after.

- [ ] **Step 3: Commit**

```bash
git add -A
git commit -m "feat(uss): resolve facts and add the applicable command"
```

---

### Task 9: Record checks and `uss check`

**Files:**
- Create: `tools/uss/internal/evaluate/check.go`, `check_test.go`
- Modify: `tools/uss/main.go`, `tools/uss/main_test.go`

**Interfaces:**
- Consumes: `ResolveFacts`, `Applicable`, `record.LoadRecord`.
- Produces:
  - `func CheckRecord(uss fs.FS, c *catalog.Catalog, r *record.Record) []catalog.Diagnostic`.
    `File` is `r.Path`. Component records are loaded and checked recursively,
    and their diagnostics carry their own path.
  - CLI: `uss check [--standard-root DIR] RECORD`. It prints the diagnostics,
    then `uss check: ok` (exit 0) or `uss check: N problem(s)` (exit 1).

Checks and messages (pinned):
- A fact key that is not defined → `unknown fact "<id>"`.
- A fact set by a rule → `fact "<id>" is set by <rule>, not answered directly`.
- An asked fact missing → `fact "<id>" is not answered`.
- A decision for an unknown rule → `decision for unknown rule <id>`.
- A decision for a non-MAY rule → `decision for <id>, a <LEVEL> rule; only MAY rules take decisions`.
- A decision for a MAY that does not apply → `decision for <id>, which does not apply`.
- An applicable MAY without a decision → `no decision for applicable MAY rule <id>`.
- A deviation for an unknown rule, a MAY rule, a non-applicable rule, or the
  same rule twice → messages naming the rule.
- A missing deviation document → `deviation document "<doc>" not found`.
  Resolve the path with `filepath.Join(r.Root, filepath.FromSlash(strings.ReplaceAll(doc, "\\", "/")))`.
- `review-by` earlier than `date` → `review-by is before date for <id>`.
- A missing audit results file → `audit results "<path>" not found`.
- A component path that is absolute or contains `..` → `component path "<p>" leaves the project`.
  A missing `<root>/<p>/.uss/record.yaml` → `component record "<path>" not found`.

- [ ] **Step 1: Write the failing tests**

`TestCheckRecord_Valid`: a complete fixture record in `t.TempDir()` (all three
asked facts answered, a decision for `USS-EXT-001`, a deviation for
`USS-SEC-002` with `uses-database: true` and the document present) → no
diagnostics. `TestCheckRecord_Errors`: a table, one case per check, each with
its message substring. `TestCheckRecord_BackslashPath`: deviation document
`docs\deviations\USS-SEC-002.md` exists as `docs/deviations/USS-SEC-002.md` →
no diagnostic, on every OS. `TestCheckRecord_Components`: root record with
`components: [svc]` whose `svc/.uss/record.yaml` leaves `has-ui` unanswered →
one diagnostic whose `File` ends with `svc/.uss/record.yaml` (compare with
forward slashes).

- [ ] **Step 2: Run to verify they fail; implement; run to verify they pass**

Run: `go test ./tools/uss/... -v`
Expected: FAIL before implementation, PASS after.

- [ ] **Step 3: Commit**

```bash
git add -A
git commit -m "feat(uss): check project records and add the check command"
```

---

### Task 10: Compliance status and category scores

**Files:**
- Create: `tools/uss/internal/evaluate/evaluate.go`, `evaluate_test.go`

**Interfaces:**
- Consumes: `Applicable`, `record.Results`.
- Produces:
  - `type RuleResult struct{ Rule catalog.Rule; Result, Evidence string }`
  - `type Status struct{ Compliant bool; MustFailures, MustDeviations, ShouldFailures, Unverified, MissingDecisions int }`
  - `type Score struct{ Category string; Pass, Applicable, Percent int }`
    (`Percent` is `-1` for n/a)
  - `type Evaluation struct{ Version, Date, Commit, Auditor string; Status Status; Scores []Score; Rules []RuleResult; Suppressions []record.Suppression; Deviations []record.Deviation }`
  - `func Evaluate(c *catalog.Catalog, r *record.Record, res *record.Results) (*Evaluation, []catalog.Diagnostic)`
  - `func (e *Evaluation) Statement() string`
  - `func (e *Evaluation) ScoreLine() string`

Rules (pinned):
- Each applicable rule takes its result from `res`; a missing result is
  `unverified` with evidence `no result recorded`.
- Consistency diagnostics (`File` = `res.Path`): a result for an unknown,
  retired or non-applicable rule; `not-applicable` for an applicable rule;
  `deviation` on a MAY or without a record deviation for that rule; `declined`
  on a non-MAY or without `adopt: false`; a MAY with `adopt: false` and any
  result other than `declined`; results version ≠ record version.
- Counting: MUST/MUST NOT `fail` → MustFailures, `deviation` → MustDeviations;
  SHOULD/SHOULD NOT `fail` → ShouldFailures; `unverified` at any level →
  Unverified; an applicable MAY without a decision → MissingDecisions.
  `Compliant` means every count is 0.
- Scores: for each registry category in order, `Percent = Pass*100/Applicable`
  (integer division), or `-1` when `Applicable == 0`.
- `Statement()`: `Compliant with USS <v> as of <date>.` or
  `Not compliant with USS <v> as of <date>: <parts>.` The parts are, in this
  order and only when non-zero, joined by `, `: `N MUST failure(s)`,
  `N documented MUST deviation(s)`, `N SHOULD failure(s)`, `N unverified rule(s)`,
  `N missing MAY decision(s)`. Singular when N is 1.
- `ScoreLine()`: `<Name> <P>%` or `<Name> n/a` for every category, joined by
  `, `, ending with `.`.

- [ ] **Step 1: Write the failing tests** (fixture standard and records)

`TestEvaluate_CompliantWithDeclinedMay`: facts all false; `USS-EXT-001`
declined (`adopt: false`, result `declined`); `USS-SEC-001` pass →
`Statement() == "Compliant with USS 0.1.0 as of 2026-11-21."`,
`ScoreLine() == "Safety 100%, Features 0%."`.
`TestEvaluate_NotCompliant`: `uses-database: true`; `USS-SEC-001` deviation
(recorded); `USS-SEC-002` fail; no decision for `USS-EXT-001` →
`"Not compliant with USS 0.1.0 as of 2026-11-21: 1 documented MUST deviation, 1 SHOULD failure, 1 unverified rule, 1 missing MAY decision."`
(`USS-EXT-001` has no result, so it is unverified too).
`TestEvaluate_FloorRounding`: `uses-database` and `has-ui` true; SEC-001 pass,
SEC-002 pass, SEC-003 fail → the Safety score is 66 and the statement contains
`1 MUST failure`. `TestEvaluate_MissingResultIsUnverified`.
`TestEvaluate_ConsistencyErrors`: a table, one case per consistency rule.

- [ ] **Step 2: Run to verify they fail; implement; run to verify they pass**

Run: `go test ./tools/uss/internal/evaluate/ -v`
Expected: FAIL before implementation, PASS after.

- [ ] **Step 3: Commit**

```bash
git add tools/uss/internal/evaluate
git commit -m "feat(uss): compute compliance status and category scores"
```

---

### Task 11: Audit report and `uss report`

**Files:**
- Create: `templates/audit-report.md.tmpl`
- Modify: `embed.go` (pattern `standard schema templates`)
- Create: `tools/uss/internal/report/report.go`, `report_test.go`, `testdata/report.golden.md`
- Modify: `tools/uss/main.go`, `tools/uss/main_test.go`

**Interfaces:**
- Consumes: `evaluate.Evaluate`, `Evaluation`.
- Produces:
  - `type Data struct{ Project string; Eval *evaluate.Evaluation }`
  - `func Render(w io.Writer, tmpl string, d Data) error`. Template functions:
    `percent` (int → `"N%"` or `"n/a"`) and `cell` (escapes `|` as `\|` and
    turns newlines into `<br>`).
  - CLI: `uss report [--standard-root DIR] [--record PATH] [--out PATH] RESULTS`.
    `--record` defaults to `.uss/record.yaml`; `--out` defaults to RESULTS
    with `.yaml` replaced by `.md`. Consistency diagnostics → print them,
    exit 1, write nothing. Otherwise write the report, print `Statement()`
    and `ScoreLine()`, and exit 0, even when not compliant.

Report layout (pinned headings, sections omitted when empty):

```
# USS audit report: <project>
<Statement>
**Audited commit:** `<commit>` · **Date:** <date> · **Auditor:** <auditor>
## Scores            table: Category | Score | Passed | Applicable
## Results
### Fail | ### Unverified | ### Deviation | ### Declined | ### Pass
                     each a table: Rule | Level | Category | Title | Evidence
## Standard Deviations   table: Rule | Document | Approved by | Date | Review by
## Suppressions          table: File | Line | Rule | Reason
```

Within each result section, sort by level (MUST, MUST NOT, SHOULD, SHOULD NOT,
MAY), then by ID, so MUST failures come first.

- [ ] **Step 1: Write the failing tests**

`TestRender_Golden`: the `TestEvaluate_NotCompliant` evaluation, plus one
suppression → output equals `testdata/report.golden.md`. Regenerate it with
`go test ./tools/uss/internal/report/ -update`, then review the golden file by
eye against the layout above before committing.
`TestRender_EscapesCells`: evidence `a|b\nc` renders as `a\|b<br>c`.
`TestRunReport_WritesFile`: record and results in `t.TempDir()` → the `.md`
file exists, stdout starts with `Compliant with USS 0.1.0`, exit 0.
`TestRunReport_ConsistencyError`: a result for an unknown rule → exit 1, no
`.md` written.

- [ ] **Step 2: Run to verify they fail; implement; run to verify they pass**

Run: `go test ./tools/uss/... -v`
Expected: FAIL before implementation, PASS after.

- [ ] **Step 3: Commit**

```bash
git add -A
git commit -m "feat(uss): render audit reports and add the report command"
```

---

### Task 12: `uss release-check`

**Files:**
- Create: `tools/uss/internal/release/git.go`, `release.go`, `release_test.go`
- Modify: `tools/uss/main.go`, `tools/uss/main_test.go`

**Interfaces:**
- Consumes: `record.LoadRecord`, `LatestAudit`, `record.LoadResults`,
  `evaluate.Evaluate`, `report.Render`.
- Produces (package `release`, all running `git -C <repoDir> …`):
  - `func CommitExists(repoDir, commit string) (bool, error)` (`git cat-file -e <commit>^{commit}`)
  - `func IsShallow(repoDir string) (bool, error)` (`git rev-parse --is-shallow-repository`)
  - `func IsAncestor(repoDir, commit string) (bool, error)` (`git merge-base --is-ancestor <commit> HEAD`)
  - `func ChangedFiles(repoDir, commit string) ([]string, error)` (`git diff --name-only <commit> HEAD`, forward slashes)
  - `func OnlyUSSChanges(paths []string) []string`: returns the offending
    paths. A path is allowed when any `/`-separated segment equals `.uss`.
  - CLI: `uss release-check [--standard-root DIR] [--record PATH] [--out PATH] [--require-compliant]`.
    `--out` defaults to `uss-report.md`.

Behaviour (pinned): for the root record and each component record: no audit
→ `no audit recorded in <record>`. A missing commit → if the clone is shallow,
`audited commit <sha> is not in this clone; the clone is shallow. Fetch full history (actions/checkout: fetch-depth: 0).`,
otherwise `audited commit <sha> not found`. Not an ancestor →
`audited commit <sha> is not an ancestor of HEAD`. Offending changes →
`changes since the audit outside .uss/:` followed by one path per line. Any
of these → exit 1. Otherwise evaluate (consistency errors → exit 1), render
each record's report and join them (root first) with `\n---\n\n` into `--out`,
and print each `Statement()`. With `--require-compliant`, exit 1 if any record
is not compliant.

- [ ] **Step 1: Write the failing tests**

Helper `newRepo(t)`: `git init`, then commits made with
`git -c user.name=uss-test -c user.email=uss-test@example.invalid commit`.
`TestOnlyUSSChanges`: table: `.uss/audits/a.yaml` allowed;
`svc/.uss/record.yaml` allowed; `src/main.go` offending; `.ussx/a` offending.
`TestReleaseCheck_OK`: commit A holds the source and the record. Commit B adds
`.uss/audits/a.yaml` (results with A's full SHA) and the record's `audits`
entry, and changes nothing outside `.uss/` → exit 0 and the report file exists.
`TestReleaseCheck_ChangesOutsideUSS` → exit 1; output lists `src/main.go`.
`TestReleaseCheck_NoAudit` → exit 1. `TestReleaseCheck_ShallowClone`:
`git clone --depth 1 <file URL of the repo>` where the audit is at an older
commit → exit 1; message contains `fetch-depth: 0`. Use a `file://` URL,
because git ignores `--depth` for plain local paths. On Windows the URL is
`file:///C:/...` (build it from `filepath.ToSlash` with a leading `/`). Each test skips when
`exec.LookPath("git")` fails.

- [ ] **Step 2: Run to verify they fail; implement; run to verify they pass**

Run: `go test ./tools/uss/... -v`
Expected: FAIL before implementation, PASS after.

- [ ] **Step 3: Commit**

```bash
git add -A
git commit -m "feat(uss): add release-check tying audits to release commits"
```

---

### Task 13: `uss export --json`

**Files:**
- Create: `tools/uss/internal/catalog/export.go`, `export_test.go`
- Modify: `tools/uss/main.go`, `tools/uss/main_test.go`

**Interfaces:**
- Consumes: `Catalog`.
- Produces: `type Export struct{ Version string; Categories []Category; Facts []Fact; Rules []ExportRule }`
  and `func (c *Catalog) Export() Export`. JSON keys in camelCase.
  `ExportRule` fields: `id`, `title`, `level`, `appliesWhen` (the effective
  `Applies.String()`), `category`, `verify`, `references`, `enables`,
  `refines`, `status` (`active` or `retired`), `retiredIn`, `replacedBy`,
  `file`, `body`. Facts are sorted by ID; rules by ID.
  CLI: `uss export [--standard-root DIR] --json` prints indented JSON
  (2 spaces). Without `--json` → exit 2, `export: only --json is supported`.

- [ ] **Step 1: Write the failing tests**

`TestExport_Fixture`: 7 rules; `USS-GO-001.AppliesWhen == "lang.go"`;
`USS-SEC-004.Status == "retired"`; 4 facts.
`TestRunExport_JSON`: stdout parses with `encoding/json`, and `version` is
`0.1.0`. `TestRunExport_NoFormat` → exit 2.

- [ ] **Step 2: Run to verify they fail; implement; run to verify they pass**

Run: `go test ./tools/uss/... -v`
Expected: FAIL before implementation, PASS after.

- [ ] **Step 3: Commit**

```bash
git add -A
git commit -m "feat(uss): export the rule catalog as JSON"
```

---

### Task 14: Conventions and adopter templates

**Files:**
- Create: `standard/conventions.md`, `templates/record.yaml`, `templates/standard-deviation.md`
- Create: `tools/uss/internal/record/templates_test.go`

**Interfaces:**
- Consumes: `record.LoadRecord`, `catalog.Load` (the embedded standard).

`standard/conventions.md` is the normative home of everything below. The spec
is design history; where both exist, write it here once.
1. Keywords: RFC 2119 + RFC 8174; the five keywords; synonyms not used; one
   rule, one level.
2. Standard files: area and module files; header fields Prefix, Kind,
   Applies when.
3. Rule format: the spec §5.2 field table plus `Refines`; body; `**Rationale:**`;
   the keyword-scan rules from Task 5. Use the spec's `USS-SEC-012` as the
   example (in a code fence).
4. Rule IDs and retiring rules (spec §5.4).
5. Applicability: facts (`applicability.md`), `Implies`, stack identifiers, the
   Task 3 grammar, `Enables`, module-level `Applies when`.
6. Score categories: the nine, with the "Measures" text from spec §5.5.
7. Choosing a level, reference don't reinvent, and pairing "zero violations"
   rules with check integrity (spec §6.4).
8. Compliance and scores: the result values, the compliant definition,
   scoring, "as perfect as it can be", and the exact statement formats from
   Task 10.
9. Standard Deviations (spec §5.6), and that a MUST deviation means not
   compliant.
10. Versioning of the standard (spec §5.7, including the 0.x rule).
11. Records and audit results: `.uss/record.yaml`,
    `.uss/audits/<date>.yaml` and `.md`, the schemas, components.

`templates/record.yaml`: a starter record with a comment above each key. It
lists every asked fact as `false`, each preceded by its Question as a comment
and the line `# Answer this. false is not a default.` Version `0.1.0`;
project name `example`; empty stack lists, decisions, deviations and audits.

`templates/standard-deviation.md`: headings Rule, Level, USS version, Scope,
Reason, Risks and impact, Compensating controls, Approval (Approver, Date;
"follow your organisation's approval policy"), Review by, Remediation plan.

- [ ] **Step 1: Write the failing test**

`TestRecordTemplateMatchesStandard`: `LoadRecord(uss.Files, "templates/record.yaml")`
(read through a temp copy when the loader needs an OS path) succeeds; its
`Standard.Version` equals the embedded registry version; its fact keys equal
exactly the set of facts with a `Question` in the embedded catalog.

- [ ] **Step 2: Run to verify it fails**

Run: `go test ./tools/uss/internal/record/ -run TestRecordTemplate -v`
Expected: FAIL (the template does not exist).

- [ ] **Step 3: Write the three documents**

- [ ] **Step 4: Run all tests and lint the embedded standard**

Run: `go test ./... && go run ./tools/uss lint`
Expected: PASS; `uss lint: ok (0 rules, 0 files)`.

- [ ] **Step 5: Commit**

```bash
git add -A
git commit -m "docs: add standard conventions and adopter templates"
```

---

### Task 15: Strict linting, one-command checks and CI

**Files:**
- Create: `tools/tools.mod`, `tools/tools.sum`, `.golangci.yml`
- Create: `tools/devcheck/main.go`
- Create: `.github/workflows/ci.yml`
- Modify: `README.md` (a "Development" section)

**Interfaces:**
- Produces: `go run ./tools/devcheck [-race]`, the single command that runs
  every check, used both locally and in CI.

- [ ] **Step 1: Pin the tools in a separate modfile**

Create `tools/tools.mod` containing only the `module` line and `go 1.27`. Then:
`go get -modfile=tools/tools.mod -tool github.com/golangci/golangci-lint/v2/cmd/golangci-lint@<latest v2 release>`
`go get -modfile=tools/tools.mod -tool golang.org/x/vuln/cmd/govulncheck@<latest release>`
Expected: `go tool -modfile=tools/tools.mod golangci-lint version` prints the
pinned version. If `-modfile` tool resolution does not work with this Go
version, add the tools to the main `go.mod` with `go get -tool` instead, and
note that in the commit message.

- [ ] **Step 2: Create `.golangci.yml`**

`version: "2"`. Linters: the default set plus `errorlint`, `gosec`, `revive`,
`nolintlint`, `misspell`, `unparam`, `exhaustive`, `gocritic`. `nolintlint`:
`require-explanation: true`, `require-specific: true`, `allow-unused: false`.
Formatters: `gofmt`, `goimports`.

- [ ] **Step 3: Implement `tools/devcheck/main.go`**

It runs these in order, prints `==> <step>` before each, and stops with exit 1
at the first failure:
1. `go vet ./...`
2. `go tool -modfile=tools/tools.mod golangci-lint run ./...`
3. `go test ./...` (with `-race` when the `-race` flag is given)
4. `go tool -modfile=tools/tools.mod govulncheck ./...`
5. `go run ./tools/uss lint`

- [ ] **Step 4: Run devcheck and fix every finding**

Run: `go run ./tools/devcheck`
Expected: all five steps pass. Fix findings in the code. Suppress only with
`//nolint:<linter> // <reason>`.

- [ ] **Step 5: Create `.github/workflows/ci.yml`**

`on: push` (branch `main`) and `pull_request`; `permissions: contents: read`.
Job `check`: matrix `os: [ubuntu-latest, windows-latest, macos-latest]`.
Steps: `actions/checkout` (`persist-credentials: false`), `actions/setup-go`
(`go-version-file: go.mod`), then
`go run ./tools/devcheck ${{ matrix.os == 'ubuntu-latest' && '-race' || '' }}`.
Pin each action to its latest release's full commit SHA, with the tag in a
trailing comment. Resolve the SHA with
`gh api repos/<owner>/<repo>/git/ref/tags/<tag> --jq .object` and, when
`type` is `tag`, dereference it with
`gh api repos/<owner>/<repo>/git/tags/<sha> --jq .object.sha`.

- [ ] **Step 6: Add the README "Development" section (STE style)**

"Install Go 1.27.4 or later. Run `go run ./tools/devcheck` to do all checks.
Run `go run ./tools/uss lint` to check the standard files."

- [ ] **Step 7: Verify**

Run: `go run ./tools/devcheck`
Expected: exit 0. (CI itself runs once the repository is pushed to GitHub.)

- [ ] **Step 8: Commit**

```bash
git add -A
git commit -m "ci: add strict linting, one-command checks and CI workflow"
```
