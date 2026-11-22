//! File-system access for the CLI (the only place the library touches disk).

use std::borrow::Cow;
use std::path::{Path, PathBuf};

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
}
