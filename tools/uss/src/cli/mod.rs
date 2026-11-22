//! Command-line interface: argument parsing and command dispatch.

use std::io::Write;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

use crate::catalog::{load, load_registry};
use crate::source::{Embedded, Source};

mod files;

pub use files::DirSource;

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

    #[test]
    fn run_unknown_command() {
        let (code, _out, err) = run_capture(&["bogus"]);
        assert_eq!(code, 2);
        assert!(err.contains("bogus"), "stderr: {err}");
    }
}
