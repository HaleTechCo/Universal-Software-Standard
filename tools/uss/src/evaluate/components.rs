//! Walking the component records of a project, once, for every user of them.

use crate::catalog::Diagnostic;
use crate::evaluate::check::{leaves_project, normalise};
use crate::evaluate::project::ProjectFiles;
use crate::record::{Record, parse_record};
use crate::source::Source;

/// One record of a project and the directory it sits in.
#[derive(Debug, Clone)]
pub struct Found {
    /// The record's directory relative to the root record's directory, with
    /// forward slashes. Empty for the root record.
    pub dir: String,
    /// The record. A component's `path` is its name relative to the root
    /// record's directory, such as `a/.uss/record.yaml`.
    pub record: Record,
}

/// The records of a project, and the problems found while finding them.
#[derive(Debug, Clone)]
pub struct Walk {
    /// The root record first, then each component depth first, in `components` order.
    pub records: Vec<Found>,
    /// A component path that leaves the project or is the record's own
    /// directory, a component record that is missing, invalid or pins another
    /// version. Each is on the record that lists the component.
    pub problems: Vec<Diagnostic>,
}

/// Joins two forward-slash paths, skipping an empty `base`.
fn join(base: &str, rel: &str) -> String {
    if base.is_empty() {
        rel.to_string()
    } else {
        format!("{base}/{rel}")
    }
}

/// Finds `root` and, depth first, the records of its components, which are
/// read through `project`. A component that cannot be used is a problem and
/// is not walked.
#[must_use]
pub fn walk(src: &dyn Source, root: &Record, project: &dyn ProjectFiles) -> Walk {
    let mut walked = Walk {
        records: vec![Found {
            dir: String::new(),
            record: root.clone(),
        }],
        problems: Vec::new(),
    };
    walk_components(src, root, project, "", &mut walked);
    walked
}

/// Walks the components of `r`, whose directory is `prefix` below the root
/// record's directory. `project` is rooted at the directory of the root record.
fn walk_components(
    src: &dyn Source,
    r: &Record,
    project: &dyn ProjectFiles,
    prefix: &str,
    walked: &mut Walk,
) {
    let file = r.path.as_str();
    for raw in &r.components {
        if leaves_project(raw) {
            walked.problems.push(Diagnostic::new(
                file,
                format!("component path \"{raw}\" leaves the project"),
            ));
            continue;
        }
        let dir = normalise(raw);
        if dir.is_empty() {
            walked.problems.push(Diagnostic::new(
                file,
                format!("component path \"{raw}\" is the record's own directory"),
            ));
            continue;
        }
        let full = join(prefix, &dir);
        let named = format!("{full}/.uss/record.yaml");
        let Some(data) = project.read(&named) else {
            walked.problems.push(Diagnostic::new(
                file,
                format!("component record \"{named}\" not found"),
            ));
            continue;
        };
        let child = match parse_record(src, &named, &data) {
            Ok(child) => child,
            Err(e) => {
                walked.problems.extend(
                    e.problems
                        .iter()
                        .map(|p| Diagnostic::new(&named, p.clone())),
                );
                continue;
            }
        };
        if child.standard.version != r.standard.version {
            walked.problems.push(Diagnostic::new(
                file,
                format!(
                    "component record \"{named}\" pins USS {}; the root record pins USS {}",
                    child.standard.version, r.standard.version
                ),
            ));
            continue;
        }
        walked.records.push(Found {
            dir: full.clone(),
            record: child.clone(),
        });
        walk_components(src, &child, project, &full, walked);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evaluate::project::MemProject;
    use crate::source::Embedded;

    fn record_text(version: &str, body: &str) -> String {
        format!("standard: {{ version: {version} }}\nproject: {{ name: p }}\n{body}")
    }

    fn root(body: &str) -> Record {
        parse_record(
            &Embedded,
            ".uss/record.yaml",
            record_text("0.1.0", body).as_bytes(),
        )
        .unwrap()
    }

    fn project(files: &[(&str, String)]) -> MemProject {
        MemProject(
            files
                .iter()
                .map(|(p, t)| ((*p).to_string(), t.clone().into_bytes()))
                .collect(),
        )
    }

    #[test]
    fn walk_orders_records_depth_first() {
        let leaf = record_text("0.1.0", "facts: {}\n");
        let a = record_text("0.1.0", "facts: {}\ncomponents: [c]\n");
        let r = root("facts: {}\ncomponents: [a, b]\n");
        let p = project(&[
            ("a/.uss/record.yaml", a),
            ("a/c/.uss/record.yaml", leaf.clone()),
            ("b/.uss/record.yaml", leaf),
        ]);
        let walked = walk(&Embedded, &r, &p);
        let dirs: Vec<&str> = walked.records.iter().map(|f| f.dir.as_str()).collect();
        assert_eq!(dirs, ["", "a", "a/c", "b"]);
        assert_eq!(walked.problems, []);
        assert_eq!(walked.records[0].record.path, ".uss/record.yaml");
        assert_eq!(walked.records[2].record.path, "a/c/.uss/record.yaml");
    }

    #[test]
    fn walk_reports_bad_components() {
        let r = root("facts: {}\ncomponents: [\"../x\", \".\", missing]\n");
        let walked = walk(&Embedded, &r, &project(&[]));
        let messages: Vec<&str> = walked.problems.iter().map(|d| d.message.as_str()).collect();
        assert_eq!(
            messages,
            [
                "component path \"../x\" leaves the project",
                "component path \".\" is the record's own directory",
                "component record \"missing/.uss/record.yaml\" not found",
            ]
        );
        assert!(walked.problems.iter().all(|d| d.file == ".uss/record.yaml"));
        assert_eq!(walked.records.len(), 1);

        let r = root("facts: {}\ncomponents: [svc]\n");
        let other = record_text("9.9.9", "facts: {}\n");
        let walked = walk(&Embedded, &r, &project(&[("svc/.uss/record.yaml", other)]));
        let messages: Vec<&str> = walked.problems.iter().map(|d| d.message.as_str()).collect();
        assert_eq!(
            messages,
            [
                "component record \"svc/.uss/record.yaml\" pins USS 9.9.9; the root record pins USS 0.1.0"
            ]
        );
        assert_eq!(walked.records.len(), 1);
    }
}
