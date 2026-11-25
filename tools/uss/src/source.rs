//! Read-only access to USS's own files (the standard, schemas, templates).

use std::borrow::Cow;
use std::collections::BTreeMap;

include!(concat!(env!("OUT_DIR"), "/embedded.rs"));

/// Directories of the repository root that hold USS's own files. A copy is in
/// `build.rs`.
pub const USS_DIRS: [&str; 3] = ["standard", "schema", "templates"];

/// Files at the repository root that are part of USS (the workflows and the
/// changelog). A copy is in `build.rs`.
pub const ROOT_FILES: [&str; 2] = ["AGENTS.md", "CHANGELOG.md"];

/// A read-only set of USS files addressed by forward-slash paths relative to
/// the USS root (for example `standard/registry.yaml`).
pub trait Source {
    /// Returns the bytes of the file at `path`, or `None` when it is absent.
    fn read(&self, path: &str) -> Option<Cow<'_, [u8]>>;

    /// Lists every file under `dir`, recursively, as forward-slash paths
    /// relative to the root, sorted.
    fn list(&self, dir: &str) -> Vec<String>;
}

/// Lists the USS files in `src`: every file under [`USS_DIRS`], and each of
/// [`ROOT_FILES`] that `src` can read, sorted.
#[must_use]
pub fn uss_files(src: &dyn Source) -> Vec<String> {
    let mut files: Vec<String> = USS_DIRS.iter().flat_map(|dir| src.list(dir)).collect();
    files.extend(
        ROOT_FILES
            .iter()
            .filter(|name| src.read(name).is_some())
            .map(|name| (*name).to_string()),
    );
    files.sort();
    files
}

/// Turns a path typed by a person into a USS path: `\` becomes `/`, and each
/// leading `./` is removed.
#[must_use]
pub fn uss_path(input: &str) -> String {
    let mut path = input.replace('\\', "/");
    while let Some(rest) = path.strip_prefix("./") {
        path = rest.to_string();
    }
    path
}

/// Returns the sorted subset of `paths` that lie under `dir`.
fn under<'a>(paths: impl Iterator<Item = &'a str>, dir: &str) -> Vec<String> {
    let dir = dir.trim_end_matches('/');
    let prefix = format!("{dir}/");
    let mut found: Vec<String> = paths
        .filter(|p| dir.is_empty() || p.starts_with(&prefix))
        .map(str::to_string)
        .collect();
    found.sort();
    found
}

/// A [`Source`] held in memory, keyed by path. Used by tests and by callers
/// that already have the files.
#[derive(Debug, Clone, Default)]
pub struct MemSource(pub BTreeMap<String, Vec<u8>>);

impl Source for MemSource {
    fn read(&self, path: &str) -> Option<Cow<'_, [u8]>> {
        self.0.get(path).map(|b| Cow::Borrowed(b.as_slice()))
    }

    fn list(&self, dir: &str) -> Vec<String> {
        under(self.0.keys().map(String::as_str), dir)
    }
}

/// The copy of the standard compiled into this binary.
#[derive(Debug, Clone, Copy, Default)]
pub struct Embedded;

impl Source for Embedded {
    fn read(&self, path: &str) -> Option<Cow<'_, [u8]>> {
        FILES
            .binary_search_by_key(&path, |(p, _)| p)
            .ok()
            .map(|i| Cow::Borrowed(FILES[i].1))
    }

    fn list(&self, dir: &str) -> Vec<String> {
        under(FILES.iter().map(|(p, _)| *p), dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_contains_registry() {
        assert!(Embedded.read("standard/registry.yaml").is_some());
        assert!(Embedded.read("standard/nope.yaml").is_none());
        assert!(
            Embedded
                .list("standard")
                .contains(&"standard/registry.yaml".to_string())
        );
    }

    #[test]
    fn embedded_has_agents_md() {
        assert!(Embedded.read("AGENTS.md").is_some());
    }

    #[test]
    fn uss_files_lists_dirs_and_root_files() {
        let files = [
            "standard/a.md",
            "schema/s.json",
            "templates/t.yaml",
            "AGENTS.md",
            "README.md",
            "other/x.md",
        ]
        .map(|p| (p.to_string(), b"x".to_vec()));
        let src = MemSource(files.into_iter().collect());
        assert_eq!(
            uss_files(&src),
            [
                "AGENTS.md",
                "schema/s.json",
                "standard/a.md",
                "templates/t.yaml"
            ]
        );
    }

    #[test]
    fn uss_path_normalises() {
        assert_eq!(uss_path(".\\standard\\core\\a.md"), "standard/core/a.md");
        assert_eq!(uss_path("./templates/record.yaml"), "templates/record.yaml");
        assert_eq!(uss_path("standard/x.md"), "standard/x.md");
    }

    #[test]
    fn mem_source_lists_recursively_sorted() {
        let mut files = BTreeMap::new();
        for p in ["a/z.md", "a/b/y.md", "ab/x.md", "c.md"] {
            files.insert(p.to_string(), b"x".to_vec());
        }
        let src = MemSource(files);
        assert_eq!(src.list("a"), vec!["a/b/y.md", "a/z.md"]);
        assert_eq!(src.list("missing"), Vec::<String>::new());
        assert_eq!(src.read("c.md").as_deref(), Some(&b"x"[..]));
    }
}
