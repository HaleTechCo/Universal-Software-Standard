//! Command-line interface: argument parsing and command dispatch.

use std::io::Write;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

use crate::catalog::{Catalog, Rule, load, load_registry};
use crate::evaluate::applicable;
use crate::record::Record;
use crate::source::{Embedded, Source};

mod files;

pub use files::{DirSource, project_root, read_record};

/// Exit code for a usage error.
const EXIT_USAGE: i32 = 2;

/// The `uss` command line.
#[derive(Parser, Debug)]
#[command(
    name = "uss",
    version,
    about = "Universal Software Standard tooling",
    arg_required_else_help = true
)]
struct Cli {
    /// Use a USS checkout at DIR instead of the standard embedded in this binary.
    #[arg(long, global = true, value_name = "DIR")]
    standard_root: Option<PathBuf>,

    #[command(subcommand)]
    command: Command,
}

/// The `uss` subcommands.
#[derive(Subcommand, Debug)]
enum Command {
    /// Check the standard's files for problems.
    Lint,
    /// Print the version of the standard.
    Version,
    /// List the rules that apply to a project.
    Applicable {
        /// Print a JSON array instead of tab-separated lines.
        #[arg(long)]
        json: bool,
        /// The project record (`.uss/record.yaml`).
        record: PathBuf,
    },
}

/// Returns the source of the standard's files: the embedded copy, or the
/// checkout at `standard_root`.
#[must_use]
pub fn source_for(standard_root: Option<&Path>) -> Box<dyn Source> {
    match standard_root {
        Some(root) => Box::new(DirSource::new(root)),
        None => Box::new(Embedded),
    }
}

/// Prints `uss <version>`, the version in the standard's registry.
fn run_version(src: &dyn Source, out: &mut dyn Write, err: &mut dyn Write) -> i32 {
    match load_registry(src) {
        Ok(registry) => {
            if writeln!(out, "uss {}", registry.version).is_err() {
                return EXIT_USAGE;
            }
            0
        }
        Err(diagnostics) => {
            for d in diagnostics {
                if writeln!(err, "{d}").is_err() {
                    break;
                }
            }
            1
        }
    }
}

/// Loads the standard and prints each problem, then a summary line, to `out`.
fn run_lint(src: &dyn Source, out: &mut dyn Write) -> i32 {
    let written = match load(src) {
        Ok(catalog) => writeln!(
            out,
            "uss lint: ok ({} rules, {} files)",
            catalog.rules.len(),
            catalog.file_count
        )
        .map(|()| 0),
        Err(diagnostics) => {
            let n = diagnostics.len();
            let noun = if n == 1 { "problem" } else { "problems" };
            diagnostics
                .iter()
                .try_for_each(|d| writeln!(out, "{d}"))
                .and_then(|()| writeln!(out, "uss lint: {n} {noun}"))
                .map(|()| 1)
        }
    };
    written.unwrap_or(EXIT_USAGE)
}

/// The message for a record that pins another version of the standard.
#[must_use]
pub fn version_error(pinned: &str, carried: &str) -> String {
    format!(
        "record pins USS {pinned}; this uss carries USS {carried}. Use uss {pinned}: download it from the v{pinned} release, or run: cargo install --locked --git https://github.com/HaleTechCo/Universal-Software-Standard --tag v{pinned} uss"
    )
}

/// Loads the catalog from `src` and the record at `record_path`, and checks
/// that the record pins the version this standard carries.
///
/// # Errors
///
/// Returns the exit code (`1`) after writing each problem to `err`.
pub fn load_project(
    src: &dyn Source,
    record_path: &Path,
    err: &mut dyn Write,
) -> Result<(Catalog, Record), i32> {
    let catalog = load(src).map_err(|diagnostics| {
        for d in diagnostics {
            if writeln!(err, "{d}").is_err() {
                break;
            }
        }
        1
    })?;
    let record = read_record(src, record_path).map_err(|e| {
        let _ = writeln!(err, "{e}");
        1
    })?;
    if record.standard.version != catalog.registry.version {
        let _ = writeln!(
            err,
            "{}",
            version_error(&record.standard.version, &catalog.registry.version)
        );
        return Err(1);
    }
    Ok((catalog, record))
}

/// Prints the rules that apply to the record, as lines or as JSON.
fn run_applicable(
    src: &dyn Source,
    record_path: &Path,
    json: bool,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> i32 {
    let (catalog, record) = match load_project(src, record_path, err) {
        Ok(loaded) => loaded,
        Err(code) => return code,
    };
    let rows: Vec<(&Rule, String, &str)> = applicable(&catalog, &record)
        .into_iter()
        .filter_map(|rule| {
            rule.active()
                .map(|a| (rule, a.level.to_string(), a.category.as_str()))
        })
        .collect();
    let written = if json {
        let items: Vec<serde_json::Value> = rows
            .iter()
            .map(|(rule, level, category)| {
                serde_json::json!({
                    "id": rule.id,
                    "title": rule.title,
                    "level": level,
                    "category": category,
                    "file": rule.file,
                })
            })
            .collect();
        writeln!(out, "{}", serde_json::Value::Array(items))
    } else {
        rows.iter().try_for_each(|(rule, level, category)| {
            writeln!(out, "{}\t{level}\t{category}\t{}", rule.id, rule.title)
        })
    };
    if written.is_err() {
        return EXIT_USAGE;
    }
    0
}

/// Runs the `uss` CLI with `args` (including the program name).
///
/// Returns the process exit code: `0` for success, `1` when problems were
/// found, `2` for a usage error. Normal output goes to `out`, diagnostics to `err`.
pub fn run(args: &[String], out: &mut dyn Write, err: &mut dyn Write) -> i32 {
    match Cli::try_parse_from(args) {
        Ok(cli) => {
            let src = source_for(cli.standard_root.as_deref());
            match cli.command {
                Command::Version => run_version(src.as_ref(), out, err),
                Command::Lint => run_lint(src.as_ref(), out),
                Command::Applicable { json, record } => {
                    run_applicable(src.as_ref(), &record, json, out, err)
                }
            }
        }
        Err(e) => {
            let rendered = e.render().to_string();
            let written = if e.use_stderr() {
                write!(err, "{rendered}")
            } else {
                write!(out, "{rendered}")
            };
            if written.is_err() {
                return EXIT_USAGE;
            }
            e.exit_code()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_capture(args: &[&str]) -> (i32, String, String) {
        let mut full = vec!["uss".to_string()];
        full.extend(args.iter().map(|a| (*a).to_string()));
        let (mut out, mut err) = (Vec::new(), Vec::new());
        let code = run(&full, &mut out, &mut err);
        (
            code,
            String::from_utf8(out).unwrap(),
            String::from_utf8(err).unwrap(),
        )
    }

    #[test]
    fn run_no_args_prints_usage() {
        let (code, _out, err) = run_capture(&[]);
        assert_eq!(code, 2);
        assert!(err.contains("Usage: uss"), "stderr: {err}");
    }

    #[test]
    fn run_version() {
        let (code, out, _err) = run_capture(&["version"]);
        assert_eq!(code, 0);
        assert_eq!(out, "uss 0.1.0\n");
    }

    #[test]
    fn run_version_standard_root() {
        let root = std::env::temp_dir().join(format!("uss-cli-root-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("standard")).unwrap();
        let root_arg = root.to_str().unwrap();

        let (code, _out, err) = run_capture(&["--standard-root", root_arg, "version"]);
        assert_eq!(code, 1);
        assert!(err.contains("standard/registry.yaml"), "stderr: {err}");

        let yaml = "version: 7.8.9\ncategories: []\nstack: {}\n";
        std::fs::write(root.join("standard/registry.yaml"), yaml).unwrap();
        let (code, out, _err) = run_capture(&["version", "--standard-root", root_arg]);
        assert_eq!(code, 0);
        assert_eq!(out, "uss 7.8.9\n");
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn run_lint_embedded() {
        let (code, out, _err) = run_capture(&["lint"]);
        assert_eq!(code, 0);
        assert!(out.starts_with("uss lint: ok ("), "stdout: {out}");
    }

    #[test]
    fn run_lint_standard_root() {
        let dir = tempfile::TempDir::new().unwrap();
        for (path, bytes) in &crate::testfixture::standard().0 {
            let bytes = if path == "standard/core/security.md" {
                String::from_utf8(bytes.clone())
                    .unwrap()
                    .replace(
                        "- **Verify:** Check that external input is validated.\n",
                        "",
                    )
                    .into_bytes()
            } else {
                bytes.clone()
            };
            let target = dir.path().join(path);
            std::fs::create_dir_all(target.parent().unwrap()).unwrap();
            std::fs::write(target, bytes).unwrap();
        }
        let root = dir.path().to_str().unwrap();
        let (code, out, _err) = run_capture(&["--standard-root", root, "lint"]);
        assert_eq!(code, 1);
        assert!(out.contains("standard/core/security.md:"), "stdout: {out}");
        assert!(out.ends_with("uss lint: 1 problem\n"), "stdout: {out}");
    }

    /// Writes the fixture standard and the record schema to a temp dir.
    fn fixture_root() -> tempfile::TempDir {
        let dir = tempfile::TempDir::new().unwrap();
        let schema = Embedded
            .read("schema/record.schema.json")
            .unwrap()
            .into_owned();
        let files = crate::testfixture::standard()
            .0
            .into_iter()
            .chain([("schema/record.schema.json".to_string(), schema)]);
        for (path, bytes) in files {
            let target = dir.path().join(path);
            std::fs::create_dir_all(target.parent().unwrap()).unwrap();
            std::fs::write(target, bytes).unwrap();
        }
        dir
    }

    fn write_record(dir: &Path, version: &str) -> String {
        let path = dir.join("record.yaml");
        let text = format!(
            concat!(
                "standard: {{ version: {} }}\n",
                "project: {{ name: p }}\n",
                "stack: {{ languages: [go] }}\n",
                "facts: {{ has-ui: false, has-web-ui: false, uses-database: false }}\n",
                "decisions: {{ USS-EXT-001: {{ adopt: false }} }}\n",
            ),
            version
        );
        std::fs::write(&path, text).unwrap();
        path.to_str().unwrap().to_string()
    }

    #[test]
    fn run_applicable_ok() {
        let root = fixture_root();
        let rec = write_record(root.path(), "0.1.0");
        let root_arg = root.path().to_str().unwrap();
        let (code, out, err) = run_capture(&["--standard-root", root_arg, "applicable", &rec]);
        assert_eq!(code, 0, "stderr: {err}");
        assert_eq!(
            out,
            concat!(
                "USS-EXT-001\tMAY\tFeatures\tOffer a plugin interface\n",
                "USS-GO-001\tMUST\tSafety\tHandle errors\n",
                "USS-SEC-001\tMUST\tSafety\tValidate input\n",
            )
        );
    }

    #[test]
    fn run_applicable_json() {
        let root = fixture_root();
        let rec = write_record(root.path(), "0.1.0");
        let root_arg = root.path().to_str().unwrap();
        let (code, out, _err) =
            run_capture(&["--standard-root", root_arg, "applicable", "--json", &rec]);
        assert_eq!(code, 0);
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        let arr = v.as_array().unwrap();
        let ids: Vec<&str> = arr.iter().map(|o| o["id"].as_str().unwrap()).collect();
        assert_eq!(ids, ["USS-EXT-001", "USS-GO-001", "USS-SEC-001"]);
        let first = arr[0].as_object().unwrap();
        let mut keys: Vec<&str> = first.keys().map(String::as_str).collect();
        keys.sort_unstable();
        assert_eq!(keys, ["category", "file", "id", "level", "title"]);
        assert_eq!(first["level"], "MAY");
        assert_eq!(first["file"], "standard/core/extensibility.md");
    }

    #[test]
    fn run_applicable_version_mismatch() {
        let root = fixture_root();
        let rec = write_record(root.path(), "9.9.9");
        let root_arg = root.path().to_str().unwrap();
        let (code, out, err) = run_capture(&["--standard-root", root_arg, "applicable", &rec]);
        assert_eq!(code, 1);
        assert_eq!(out, "");
        assert!(err.contains("--tag v9.9.9 uss"), "stderr: {err}");
    }

    #[test]
    fn run_applicable_missing_record() {
        let root = fixture_root();
        let root_arg = root.path().to_str().unwrap();
        let missing = root.path().join("nope.yaml");
        let (code, _out, err) = run_capture(&[
            "--standard-root",
            root_arg,
            "applicable",
            missing.to_str().unwrap(),
        ]);
        assert_eq!(code, 1);
        assert!(err.contains("nope.yaml"), "stderr: {err}");
    }

    #[test]
    fn version_error_text() {
        assert_eq!(
            version_error("1.2.3", "0.1.0"),
            "record pins USS 1.2.3; this uss carries USS 0.1.0. Use uss 1.2.3: download it from the v1.2.3 release, or run: cargo install --locked --git https://github.com/HaleTechCo/Universal-Software-Standard --tag v1.2.3 uss"
        );
    }

    #[test]
    fn run_unknown_command() {
        let (code, _out, err) = run_capture(&["bogus"]);
        assert_eq!(code, 2);
        assert!(err.contains("bogus"), "stderr: {err}");
    }
}
