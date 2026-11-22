//! A mini-standard used by the tests of every module.

use crate::source::MemSource;

const REGISTRY: &str = "version: 0.1.0
categories:
  - { id: safety, name: Safety }
  - { id: features, name: Features }
stack:
  lang: [go]
";

const APPLICABILITY: &str = "# Applicability facts

Facts the rules use.

### has-ui

- **Question:** Do people use it through a user interface?
- **Definition:** Yes when people interact with it through any UI.

### uses-database

- **Question:** Does it use a database?
- **Definition:** Yes when it reads or writes a database.

### has-web-ui

- **Question:** Does it serve a UI in a web browser?
- **Definition:** Yes when it serves a UI that people open in a browser.
- **Implies:** has-ui

### has-plugin-interface

- **Enabled by:** USS-EXT-001
- **Definition:** Yes when the project exposes a plugin interface.
";

const SECURITY: &str = "# Security

- **Prefix:** SEC
- **Kind:** area

### USS-SEC-001: Validate input

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** Check that external input is validated.
- **References:** CWE-20

Every project MUST validate external input.

### USS-SEC-002: Use a least-privilege database account

- **Level:** SHOULD
- **Applies when:** uses-database
- **Category:** Safety
- **Verify:** Check the database account's grants.
- **References:** CWE-250

A project SHOULD connect with a least-privilege database account.

### USS-SEC-003: Do not render raw input

- **Level:** MUST NOT
- **Applies when:** has-ui
- **Category:** Safety
- **Verify:** Check that the UI escapes values.
- **References:** CWE-79

A UI MUST NOT render external input without escaping it.

### USS-SEC-004: Retired example

- **Status:** Retired
- **Retired in:** 0.1.0
- **Replaced by:** USS-SEC-001
";

const EXTENSIBILITY: &str = "# Extensibility

- **Prefix:** EXT
- **Kind:** area

### USS-EXT-001: Offer a plugin interface

- **Level:** MAY
- **Applies when:** always
- **Category:** Features
- **Verify:** Check for a documented plugin interface.
- **References:** Project design notes
- **Enables:** has-plugin-interface

A project MAY offer a plugin interface.

### USS-EXT-002: Secure the plugin interface

- **Level:** MUST
- **Applies when:** has-plugin-interface
- **Category:** Features
- **Verify:** Check that plugins run with limited rights.
- **References:** CWE-269

A plugin interface MUST run plugins with limited rights.
";

const GO: &str = "# Go

- **Prefix:** GO
- **Kind:** module
- **Applies when:** lang.go

### USS-GO-001: Handle errors

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** Check that no error value is discarded.
- **References:** Effective Go
- **Refines:** USS-SEC-001

Go code MUST handle every returned error.
";

/// The mini-standard: registry, facts and seven rules in three files.
pub(crate) fn standard() -> MemSource {
    let mut m = MemSource::default();
    for (path, text) in [
        ("standard/registry.yaml", REGISTRY),
        ("standard/applicability.md", APPLICABILITY),
        ("standard/core/security.md", SECURITY),
        ("standard/core/extensibility.md", EXTENSIBILITY),
        ("standard/modules/languages/go.md", GO),
    ] {
        m.0.insert(path.to_string(), text.as_bytes().to_vec());
    }
    m
}
