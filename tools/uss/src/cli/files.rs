//! File-system access for the CLI (the only place the library touches disk).

use std::borrow::Cow;
use std::path::{Component, Path, PathBuf};

use crate::evaluate::{self, ProjectFiles};
use crate::record::{Record, parse_record};
use crate::source::Source;

/// A [`Source`] over a USS checkout directory on disk.
#[derive(Debug, Clone)]
pub struct DirSource {
    root: PathBuf,
}

impl DirSource {
    /// Creates a source rooted at `root`.
    #[must_use]
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    fn resolve(&self, path: &str) -> PathBuf {
        path.split('/')
            .filter(|p| !p.is_empty())
            .fold(self.root.clone(), |acc, part| acc.join(part))
    }
}

/// Appends every file under `dir` to `out`, named relative to the root via `rel`.
fn collect(dir: &Path, rel: &str, out: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        let child = if rel.is_empty() {
            name
        } else {
            format!("{rel}/{name}")
        };
        let path = entry.path();
        if path.is_dir() {
            collect(&path, &child, out);
        } else {
            out.push(child);
        }
    }
}

impl Source for DirSource {
    fn read(&self, path: &str) -> Option<Cow<'_, [u8]>> {
        std::fs::read(self.resolve(path)).ok().map(Cow::Owned)
    }

    fn list(&self, dir: &str) -> Vec<String> {
        let dir = dir.trim_matches('/');
        let mut out = Vec::new();
        collect(&self.resolve(dir), dir, &mut out);
        out.sort();
        out
    }
}

/// An adopting project's files on disk, under `root`.
#[derive(Debug, Clone)]
pub struct OsProject {
    /// The project root.
    pub root: PathBuf,
}

impl OsProject {
    /// Joins `rel` onto the root. Returns `None` when any part is not a plain
    /// name (`..`, a drive prefix such as `C:`, a root), so a path never leaves the root.
    fn resolve(&self, rel: &str) -> Option<PathBuf> {
        let mut path = self.root.clone();
        for part in rel.split('/').filter(|p| !p.is_empty() && *p != ".") {
            let mut components = Path::new(part).components();
            match (components.next(), components.next()) {
                (Some(Component::Normal(name)), None) => path.push(name),
                _ => return None,
            }
        }
        Some(path)
    }
}

impl ProjectFiles for OsProject {
    fn exists(&self, rel: &str) -> bool {
        self.resolve(rel).is_some_and(|p| p.exists())
    }

    fn read(&self, rel: &str) -> Option<Vec<u8>> {
        std::fs::read(self.resolve(rel)?).ok()
    }
}

/// Returns the root of the project that holds the record at `record_path`:
/// the parent of the `.uss` directory the record is in, or the record's own
/// directory.
#[must_use]
pub fn project_root(record_path: &Path) -> PathBuf {
    PathBuf::from(evaluate::project_root(&record_path.to_string_lossy()))
}

/// Reads and parses the record at `path`, validating it against the schema in `src`.
///
/// The record's `path` is `path` with forward slashes.
///
/// # Errors
///
/// Returns a message naming the path when the file cannot be read or the
/// record is invalid.
pub fn read_record(src: &dyn Source, path: &Path) -> Result<Record, String> {
    let shown = path.to_string_lossy().replace('\\', "/");
    let data = std::fs::read(path).map_err(|e| format!("{shown}: cannot read the record: {e}"))?;
    parse_record(src, &shown, &data).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("uss-files-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn dir_source_reads_and_lists() {
        let root = temp_dir("dirsource");
        std::fs::create_dir_all(root.join("standard/core")).unwrap();
        std::fs::write(root.join("standard/registry.yaml"), b"v").unwrap();
        std::fs::write(root.join("standard/core/b.md"), b"b").unwrap();
        std::fs::write(root.join("standard/core/a.md"), b"a").unwrap();
        let src = DirSource::new(&root);
        assert_eq!(src.read("standard/core/a.md").as_deref(), Some(&b"a"[..]));
        assert!(src.read("standard/missing").is_none());
        assert_eq!(
            src.list("standard"),
            vec![
                "standard/core/a.md",
                "standard/core/b.md",
                "standard/registry.yaml"
            ]
        );
        assert_eq!(src.list("nothing"), Vec::<String>::new());
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn os_project_reads_and_checks() {
        let root = temp_dir("osproject");
        std::fs::create_dir_all(root.join("docs")).unwrap();
        std::fs::write(root.join("docs/a.md"), b"a").unwrap();
        let p = OsProject { root: root.clone() };
        assert!(p.exists("docs/a.md"));
        assert!(!p.exists("docs/b.md"));
        assert_eq!(p.read("docs/a.md"), Some(b"a".to_vec()));
        assert_eq!(p.read("docs/b.md"), None);
        assert!(!p.exists("../x"));
        assert!(!p.exists(".."));
        assert!(!p.exists("docs/../docs/a.md"));
        assert_eq!(p.read("a/C:x"), None);
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn project_root_cases() {
        let tmp = Path::new("tmp-base");
        assert_eq!(
            project_root(&tmp.join("proj/.uss/record.yaml")),
            tmp.join("proj")
        );
        assert_eq!(project_root(&tmp.join("x/record.yaml")), tmp.join("x"));
        assert_eq!(project_root(Path::new("record.yaml")), PathBuf::from("."));
    }

    #[test]
    fn read_record_errors_name_the_path() {
        let root = temp_dir("readrecord");
        let missing = root.join("missing.yaml");
        let err = read_record(&crate::source::Embedded, &missing).unwrap_err();
        assert!(err.contains("missing.yaml"), "error: {err}");
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn read_record_sets_forward_slash_path() {
        let root = temp_dir("readrecord-ok");
        let file = root.join("record.yaml");
        std::fs::write(
            &file,
            "standard: { version: 0.1.0 }\nproject: { name: p }\nfacts: {}\n",
        )
        .unwrap();
        let r = read_record(&crate::source::Embedded, &file).unwrap();
        assert_eq!(r.path, file.to_str().unwrap().replace('\\', "/"));
        std::fs::remove_dir_all(&root).unwrap();
    }
}
