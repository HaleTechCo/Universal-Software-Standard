//! Build script: embeds the standard's files into the library.
//!
//! Walks `standard/`, `schema/` and `templates/` at the repository root and
//! writes `$OUT_DIR/embedded.rs`, a sorted table of `(path, bytes)`.

use std::error::Error;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

/// Directories (relative to the repository root) that are embedded.
const DIRS: [&str; 3] = ["standard", "schema", "templates"];

/// Collects every file under `dir` as `(forward-slash path, absolute path)`.
fn walk(dir: &Path, rel: &str, found: &mut Vec<(String, PathBuf)>) -> Result<(), Box<dyn Error>> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        let child_rel = format!("{rel}/{name}");
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            walk(&path, &child_rel, found)?;
        } else {
            found.push((child_rel, path));
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let root = manifest.join("../..");
    let out = PathBuf::from(std::env::var("OUT_DIR")?).join("embedded.rs");

    println!("cargo:rerun-if-changed=build.rs");
    let mut found = Vec::new();
    for dir in DIRS {
        let path = root.join(dir);
        println!("cargo:rerun-if-changed={}", path.display());
        if path.is_dir() {
            walk(&path, dir, &mut found)?;
        }
    }
    found.sort();

    let mut table = String::from("pub(crate) static FILES: &[(&str, &[u8])] = &[\n");
    for (rel, path) in &found {
        // Debug-format a string (not a `Path`) so backslashes are escaped.
        let absolute = path.canonicalize()?.to_string_lossy().into_owned();
        writeln!(table, "    ({rel:?}, include_bytes!({absolute:?})),")?;
    }
    table.push_str("];\n");
    std::fs::write(out, table)?;
    Ok(())
}
