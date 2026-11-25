//! Access to an adopting project's files, relative to its root.

use std::collections::BTreeMap;

/// The files of an adopting project. Paths are forward-slash paths relative
/// to the project root.
pub trait ProjectFiles {
    /// Returns whether a file or directory exists at `rel`.
    fn exists(&self, rel: &str) -> bool;
    /// Returns the contents of the file at `rel`, or `None` when it cannot be read.
    fn read(&self, rel: &str) -> Option<Vec<u8>>;
}

/// A project held in memory, by path.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MemProject(pub BTreeMap<String, Vec<u8>>);

impl ProjectFiles for MemProject {
    fn exists(&self, rel: &str) -> bool {
        self.0.contains_key(rel)
    }

    fn read(&self, rel: &str) -> Option<Vec<u8>> {
        self.0.get(rel).cloned()
    }
}

/// Returns the root of the project that holds the record at `record_path`:
/// the parent of the `.uss` directory the record is in, or the record's own
/// directory. The result has forward slashes, and is `.` when it is empty.
#[must_use]
pub fn project_root(record_path: &str) -> String {
    let path = record_path.replace('\\', "/");
    let mut parts: Vec<&str> = path.split('/').collect();
    parts.pop();
    if parts.last() == Some(&".uss") {
        parts.pop();
    }
    let root = parts.join("/");
    if !root.is_empty() {
        // A drive root such as `C:` alone would be a drive-relative path.
        return if root.ends_with(':') {
            format!("{root}/")
        } else {
            root
        };
    }
    if path.starts_with('/') {
        "/".to_string()
    } else {
        ".".to_string()
    }
}

/// A component's view of its parent's files: every path is looked up under `prefix`.
pub struct SubProject<'a> {
    /// The parent's files.
    pub inner: &'a dyn ProjectFiles,
    /// The component's directory, relative to the parent's root.
    pub prefix: String,
}

impl SubProject<'_> {
    fn full(&self, rel: &str) -> String {
        format!("{}/{rel}", self.prefix)
    }
}

impl ProjectFiles for SubProject<'_> {
    fn exists(&self, rel: &str) -> bool {
        self.inner.exists(&self.full(rel))
    }

    fn read(&self, rel: &str) -> Option<Vec<u8>> {
        self.inner.read(&self.full(rel))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_root_strings() {
        for (record, want) in [
            (".uss/record.yaml", "."),
            ("proj/.uss/record.yaml", "proj"),
            ("proj\\.uss\\record.yaml", "proj"),
            ("x/record.yaml", "x"),
            ("record.yaml", "."),
        ] {
            assert_eq!(project_root(record), want, "record path {record:?}");
        }
    }
}
