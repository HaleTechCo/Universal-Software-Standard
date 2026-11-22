//! A strict, line-based scanner for the Markdown files of the standard.

use std::collections::BTreeMap;
use std::sync::LazyLock;

use regex::Regex;

use super::Diagnostic;
use super::registry::constant_regex;
use crate::text;

static FIELD_RE: LazyLock<Regex> =
    LazyLock::new(|| constant_regex(r"^- \*\*([A-Za-z ]+):\*\* (.*)$"));

/// A `- **Name:** value` field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Field {
    /// The value, with continuation lines joined by one space.
    pub value: String,
    /// 1-based line of the field's first line.
    pub line: usize,
}

/// A `###` section.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Section {
    /// Heading text after `### `.
    pub heading: String,
    /// 1-based line of the heading.
    pub line: usize,
    /// The section's field block.
    pub fields: BTreeMap<String, Field>,
    /// The text after the field block, fenced blocks included, with the blank
    /// lines at both ends trimmed. The field block is not part of it.
    pub body: String,
    /// `body` with fenced blocks removed.
    pub prose: String,
}

/// A scanned document.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Doc {
    /// Path of the file.
    pub file: String,
    /// Text of the first `# ` heading, or empty.
    pub title: String,
    /// The field block after the title.
    pub header: BTreeMap<String, Field>,
    /// The `###` sections, in order.
    pub sections: Vec<Section>,
}

/// Where the scanner is putting lines.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Target {
    /// Before the title, or after a `##` heading.
    None,
    Header,
    Section,
}

/// State of the field block after a heading.
#[derive(PartialEq, Eq)]
enum Block {
    /// Nothing seen after the heading yet (blank lines are skipped).
    Waiting,
    /// Reading field lines; holds the last field name.
    Open(Option<String>),
    Closed,
}

/// Scans `data` into a [`Doc`], reporting malformed fields.
pub(crate) fn parse_doc(file: &str, data: &[u8]) -> (Doc, Vec<Diagnostic>) {
    let raw = String::from_utf8_lossy(data);
    let text = text::normalise(&raw);

    let mut doc = Doc {
        file: file.to_string(),
        ..Doc::default()
    };
    let mut diags = Vec::new();
    let mut fence: Option<&str> = None;
    let mut fence_line = 0;
    let mut target = Target::None;
    let mut block = Block::Closed;
    let mut have_title = false;
    // Lines of the current section: (text, fenced).
    let mut lines: Vec<(&str, bool)> = Vec::new();

    for (i, line) in text.lines().enumerate() {
        let n = i + 1;
        if let Some(marker) = fence {
            if line.starts_with(marker) {
                fence = None;
            }
            lines.push((line, true));
            continue;
        }
        if let Some(marker) = ["```", "~~~"].into_iter().find(|m| line.starts_with(m)) {
            fence = Some(marker);
            fence_line = n;
            block = Block::Closed;
            lines.push((line, true));
            continue;
        }
        if let Some(rest) = line.strip_prefix("### ") {
            finish(&mut doc, target, &mut lines);
            doc.sections.push(Section {
                heading: rest.trim().to_string(),
                line: n,
                fields: BTreeMap::new(),
                body: String::new(),
                prose: String::new(),
            });
            target = Target::Section;
            block = Block::Waiting;
            continue;
        }
        if line.starts_with("## ") {
            finish(&mut doc, target, &mut lines);
            target = Target::None;
            block = Block::Closed;
            continue;
        }
        if let Some(rest) = line.strip_prefix("# ")
            && !have_title
        {
            have_title = true;
            doc.title = rest.trim().to_string();
            target = Target::Header;
            block = Block::Waiting;
            continue;
        }

        if block == Block::Waiting && !line.trim().is_empty() {
            block = Block::Open(None);
        }
        let mut consumed = false;
        if let Block::Open(last) = &mut block {
            let fields = match target {
                Target::Header => Some(&mut doc.header),
                Target::Section => doc.sections.last_mut().map(|s| &mut s.fields),
                Target::None => None,
            };
            consumed =
                fields.is_some_and(|fields| read_field(fields, last, (line, n), file, &mut diags));
            if !consumed {
                block = Block::Closed;
            }
        }
        if !consumed {
            lines.push((line, false));
        }
    }
    if fence.is_some() {
        let mut d = Diagnostic::new(file, "unclosed code fence");
        d.line = fence_line;
        diags.push(d);
    }
    finish(&mut doc, target, &mut lines);
    (doc, diags)
}

/// Reads `line` (with its 1-based number) as a field or a continuation of the
/// last field. Returns `false` when the line ends the field block.
fn read_field(
    fields: &mut BTreeMap<String, Field>,
    last: &mut Option<String>,
    (line, n): (&str, usize),
    file: &str,
    diags: &mut Vec<Diagnostic>,
) -> bool {
    if let Some(caps) = FIELD_RE.captures(line) {
        let name = caps[1].to_string();
        if fields.contains_key(&name) {
            let mut d = Diagnostic::new(file, format!("duplicate field \"{name}\""));
            d.line = n;
            diags.push(d);
            *last = None;
        } else {
            fields.insert(
                name.clone(),
                Field {
                    value: caps[2].trim_end().to_string(),
                    line: n,
                },
            );
            *last = Some(name);
        }
        true
    } else if line.starts_with("  ") && !line.trim().is_empty() {
        if let Some(f) = last.as_ref().and_then(|name| fields.get_mut(name)) {
            f.value.push(' ');
            f.value.push_str(line.trim());
        }
        last.is_some()
    } else {
        false
    }
}

/// Moves the collected lines into the last section's `body` and `prose`,
/// without the blank lines at either end.
fn finish(doc: &mut Doc, target: Target, lines: &mut Vec<(&str, bool)>) {
    if target == Target::Section
        && let Some(s) = doc.sections.last_mut()
    {
        let blank = |l: &(&str, bool)| l.0.trim().is_empty();
        let start = lines.iter().position(|l| !blank(l)).unwrap_or(lines.len());
        let end = lines
            .iter()
            .rposition(|l| !blank(l))
            .map_or(start, |i| i + 1);
        let lines = &lines[start..end];
        s.body = lines.iter().map(|(l, _)| *l).collect::<Vec<_>>().join("\n");
        s.prose = lines
            .iter()
            .filter(|(_, fenced)| !fenced)
            .map(|(l, _)| *l)
            .collect::<Vec<_>>()
            .join("\n");
    }
    lines.clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC: &str = "# Title here\n\n- **Prefix:** SEC\n- **Name:** Security\n\n## Group\n\n### USS-A-001: one\n\n- **Level:** MUST\n\nBody one.\n\n## Other group\n\n### USS-A-002: two\n\nBody two.\n";

    fn field(value: &str, line: usize) -> Field {
        Field {
            value: value.to_string(),
            line,
        }
    }

    #[test]
    fn parse_doc_basic() {
        let (doc, diags) = parse_doc("a.md", BASIC.as_bytes());
        assert!(diags.is_empty(), "{diags:?}");
        assert_eq!(doc.file, "a.md");
        assert_eq!(doc.title, "Title here");
        assert_eq!(doc.header.len(), 2);
        assert_eq!(doc.header["Prefix"], field("SEC", 3));
        assert_eq!(doc.header["Name"], field("Security", 4));
        assert_eq!(doc.sections.len(), 2);
        let s = &doc.sections[0];
        assert_eq!(s.heading, "USS-A-001: one");
        assert_eq!(s.line, 8);
        assert_eq!(s.fields["Level"], field("MUST", 10));
        assert_eq!(s.body, "Body one.");
        assert!(!s.prose.contains("Level"));
        assert!(!s.body.contains("Body two."));
        assert_eq!(doc.sections[1].heading, "USS-A-002: two");
        assert!(doc.sections[1].fields.is_empty());
    }

    #[test]
    fn parse_doc_fences() {
        for marker in ["```", "~~~"] {
            let src = format!(
                "# T\n\n### USS-X-001: x\n\n- **Level:** MUST\n\nText.\n\n{marker}md\n### USS-X-001: y\n- **Level:** MAY\n{marker}\n\nAfter.\n"
            );
            let (doc, diags) = parse_doc("a.md", src.as_bytes());
            assert!(diags.is_empty(), "{diags:?}");
            assert_eq!(doc.sections.len(), 1);
            let s = &doc.sections[0];
            assert_eq!(s.fields["Level"].value, "MUST");
            assert!(s.body.contains("### USS-X-001: y"));
            assert!(s.body.contains("- **Level:** MAY"));
            assert!(!s.prose.contains("USS-X-001: y"));
            assert!(!s.prose.contains("MAY"));
            assert!(s.prose.contains("Text.") && s.prose.contains("After."));
        }
    }

    #[test]
    fn parse_doc_unclosed_fence() {
        let src = "# T\n\n### a\n\nText.\n\n```\ncode\n\n### b\n\n- **Question:** Q?\n";
        let (doc, diags) = parse_doc("a.md", src.as_bytes());
        assert_eq!(diags.len(), 1, "{diags:?}");
        assert_eq!(diags[0].message, "unclosed code fence");
        assert_eq!(diags[0].line, 7);
        assert_eq!(doc.sections.len(), 1);
    }

    #[test]
    fn parse_doc_crlf_and_bom() {
        let mut data = vec![0xEF, 0xBB, 0xBF];
        data.extend_from_slice(BASIC.replace('\n', "\r\n").as_bytes());
        let (a, da) = parse_doc("a.md", &data);
        let (b, db) = parse_doc("a.md", BASIC.as_bytes());
        assert!(da.is_empty() && db.is_empty());
        assert_eq!(a, b);
        assert_eq!(a.title, "Title here");
    }

    #[test]
    fn parse_doc_continuation() {
        let src = "# T\n\n### s\n\n- **Question:** First half\n  second half\n- **Other:** x\n\n  not a continuation\n";
        let (doc, diags) = parse_doc("a.md", src.as_bytes());
        assert!(diags.is_empty(), "{diags:?}");
        let f = &doc.sections[0].fields;
        assert_eq!(f["Question"].value, "First half second half");
        assert_eq!(f["Other"].value, "x");
    }

    #[test]
    fn parse_doc_duplicate_field() {
        let src = "# T\n\n### s\n\n- **Level:** MUST\n- **Level:** MAY\n";
        let (doc, diags) = parse_doc("a.md", src.as_bytes());
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].message, "duplicate field \"Level\"");
        assert_eq!(diags[0].line, 6);
        assert_eq!(diags[0].file, "a.md");
        assert_eq!(doc.sections[0].fields["Level"].value, "MUST");
    }
}
