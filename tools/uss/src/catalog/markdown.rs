//! A strict, line-based scanner for the Markdown files of the standard.

use std::collections::BTreeMap;
use std::sync::LazyLock;

use regex::Regex;

use super::Diagnostic;
use super::registry::constant_regex;
use crate::text;

static REF_RE: LazyLock<Regex> =
    LazyLock::new(|| constant_regex(r"\bUSS-[A-Z][A-Z0-9]{1,7}-\d{3}\b"));

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

/// One line of a `##` group.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Line {
    /// 1-based line number.
    pub n: usize,
    /// The text of the line.
    pub text: String,
    /// Whether the line is part of a fenced code block, fences included.
    pub fenced: bool,
}

/// A `##` heading with the lines up to the next `##` or `###` heading.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Group {
    /// Heading text after `## `.
    pub heading: String,
    /// 1-based line of the heading.
    pub line: usize,
    /// The lines under the heading.
    pub lines: Vec<Line>,
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
    /// The `##` groups, in order.
    pub groups: Vec<Group>,
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
    let mut in_group = false;
    // Lines of the current section: (text, fenced).
    let mut lines: Vec<(&str, bool)> = Vec::new();

    for (i, line) in text.lines().enumerate() {
        let n = i + 1;
        if let Some(marker) = fence {
            if line.starts_with(marker) {
                fence = None;
            }
            lines.push((line, true));
            note(&mut doc, in_group, (line, n), true);
            continue;
        }
        if let Some(marker) = ["```", "~~~"].into_iter().find(|m| line.starts_with(m)) {
            fence = Some(marker);
            fence_line = n;
            block = Block::Closed;
            lines.push((line, true));
            note(&mut doc, in_group, (line, n), true);
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
            in_group = false;
            continue;
        }
        if let Some(rest) = line.strip_prefix("## ") {
            finish(&mut doc, target, &mut lines);
            doc.groups.push(Group {
                heading: rest.trim().to_string(),
                line: n,
                lines: Vec::new(),
            });
            target = Target::None;
            block = Block::Closed;
            in_group = true;
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
            note(&mut doc, in_group, (line, n), false);
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

/// Adds a line to the open `##` group, if there is one.
fn note(doc: &mut Doc, in_group: bool, (text, n): (&str, usize), fenced: bool) {
    if in_group && let Some(g) = doc.groups.last_mut() {
        g.lines.push(Line {
            n,
            text: text.to_string(),
            fenced,
        });
    }
}

/// A table of a `##` group.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Table {
    /// The header cells.
    pub header: Vec<String>,
    /// The rows under the separator line.
    pub rows: Vec<Row>,
}

/// One row of a [`Table`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Row {
    /// 1-based line of the row.
    pub line: usize,
    /// The cells, trimmed.
    pub cells: Vec<String>,
}

/// Splits a table line into trimmed cells. A `|` escaped as `\|` stays in the
/// cell as `|`.
fn split_cells(line: &str) -> Vec<String> {
    let mut cells = Vec::new();
    let mut cur = String::new();
    let mut chars = line.trim().chars().peekable();
    // The leading pipe starts the first cell.
    chars.next_if_eq(&'|');
    while let Some(c) = chars.next() {
        match c {
            '\\' if chars.peek() == Some(&'|') => {
                chars.next();
                cur.push('|');
            }
            '|' => cells.push(std::mem::take(&mut cur).trim().to_string()),
            _ => cur.push(c),
        }
    }
    if !cur.trim().is_empty() {
        cells.push(cur.trim().to_string());
    }
    cells
}

/// Reads the first table in `lines`: the first run of unfenced lines that
/// start with `|`. Its second line is the separator.
///
/// # Errors
///
/// Returns the line of the problem (`0` when there is no table) and what is
/// wrong: no table, no separator line, or a row with the wrong number of
/// cells.
pub(crate) fn parse_table(lines: &[Line]) -> Result<Table, (usize, String)> {
    let is_row = |l: &&Line| !l.fenced && l.text.starts_with('|');
    let start = lines
        .iter()
        .position(|l| is_row(&l))
        .ok_or((0, "no table".to_string()))?;
    let run: Vec<&Line> = lines[start..].iter().take_while(is_row).collect();
    let header = split_cells(&run[0].text);
    let separator = run.get(1).filter(|l| {
        let cells = split_cells(&l.text);
        cells.len() == header.len()
            && cells.iter().all(|c| {
                let c = c.strip_prefix(':').unwrap_or(c);
                let c = c.strip_suffix(':').unwrap_or(c);
                !c.is_empty() && c.chars().all(|ch| ch == '-')
            })
    });
    if separator.is_none() {
        let at = run.get(1).map_or(run[0].n, |l| l.n);
        return Err((at, "table has no separator line".to_string()));
    }
    let mut rows = Vec::new();
    for l in &run[2..] {
        let cells = split_cells(&l.text);
        if cells.len() != header.len() {
            return Err((
                l.n,
                format!(
                    "table row has {} cells, the header has {}",
                    cells.len(),
                    header.len()
                ),
            ));
        }
        rows.push(Row { line: l.n, cells });
    }
    Ok(Table { header, rows })
}

/// Finds the rule IDs that `data` names outside fenced code blocks, each with
/// its 1-based line.
pub(crate) fn rule_refs(data: &[u8]) -> Vec<(usize, String)> {
    let raw = String::from_utf8_lossy(data);
    let text = text::normalise(&raw);
    let mut fence: Option<&str> = None;
    let mut out = Vec::new();
    for (i, line) in text.lines().enumerate() {
        if let Some(marker) = fence {
            if line.starts_with(marker) {
                fence = None;
            }
        } else if let Some(marker) = ["```", "~~~"].into_iter().find(|m| line.starts_with(m)) {
            fence = Some(marker);
        } else {
            out.extend(
                REF_RE
                    .find_iter(line)
                    .map(|m| (i + 1, m.as_str().to_string())),
            );
        }
    }
    out
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
    fn rule_refs_skips_fences() {
        let src = "See USS-SEC-001 and USS-A1-042.
```
USS-SEC-002
```
Not XUSS-SEC-003 or USS-SEC-0044.
~~~
USS-SEC-005
~~~
USS-GO-007
";
        let refs = rule_refs(src.as_bytes());
        let want = [(1, "USS-SEC-001"), (1, "USS-A1-042"), (9, "USS-GO-007")];
        assert_eq!(refs, want.map(|(n, id)| (n, id.to_string())).to_vec());
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

    #[test]
    fn parse_doc_groups() {
        let src = "# T\n\n- **Prefix:** X\n\n## One\n\ntext\n```\n## not a heading\n```\n\n### rule\n\n- **Level:** MUST\n\nBody.\n\n## Two\n\nmore\n";
        let (doc, diags) = parse_doc("a.md", src.as_bytes());
        assert!(diags.is_empty(), "{diags:?}");
        let g = &doc.groups;
        assert_eq!(g.len(), 2);
        assert_eq!((g[0].heading.as_str(), g[0].line), ("One", 5));
        let shown: Vec<_> = g[0]
            .lines
            .iter()
            .map(|l| (l.n, l.text.as_str(), l.fenced))
            .collect();
        assert_eq!(
            shown,
            [
                (6, "", false),
                (7, "text", false),
                (8, "```", true),
                (9, "## not a heading", true),
                (10, "```", true),
                (11, "", false),
            ]
        );
        assert_eq!((g[1].heading.as_str(), g[1].line), ("Two", 18));
        assert_eq!(g[1].lines.last().map(|l| l.text.as_str()), Some("more"));
        assert_eq!(doc.sections.len(), 1);
        assert!(doc.header.contains_key("Prefix"));
    }

    fn lines_of(text: &str) -> Vec<Line> {
        let (doc, _) = parse_doc("a.md", format!("# T\n\n## G\n{text}").as_bytes());
        doc.groups[0].lines.clone()
    }

    #[test]
    fn parse_table_cases() {
        let ok = parse_table(&lines_of(
            "\ntext\n| A | B |\n|---|:-:|\n| 1 | a \\| b |\n|2|x|\n\nafter\n",
        ))
        .unwrap();
        assert_eq!(ok.header, ["A", "B"]);
        assert_eq!(ok.rows.len(), 2);
        assert_eq!(ok.rows[0].line, 8);
        assert_eq!(ok.rows[0].cells, ["1", "a | b"]);
        assert_eq!(ok.rows[1].cells, ["2", "x"]);

        let no_sep = parse_table(&lines_of("| A | B |\n| 1 | 2 |\n"));
        assert_eq!(no_sep, Err((5, "table has no separator line".to_string())));
        let only_header = parse_table(&lines_of("| A | B |\n"));
        assert_eq!(
            only_header,
            Err((4, "table has no separator line".to_string()))
        );

        let wide = parse_table(&lines_of("| A | B |\n|---|---|\n| 1 | 2 | 3 |\n"));
        assert_eq!(
            wide,
            Err((6, "table row has 3 cells, the header has 2".to_string()))
        );

        let fenced = parse_table(&lines_of("```\n| A | B |\n|---|---|\n```\n"));
        assert_eq!(fenced, Err((0, "no table".to_string())));
        let after = parse_table(&lines_of("```\n| X |\n```\n| A |\n|---|\n| 1 |\n")).unwrap();
        assert_eq!(after.header, ["A"]);
    }
}
