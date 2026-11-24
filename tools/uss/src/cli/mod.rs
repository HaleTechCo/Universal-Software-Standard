//! Command-line interface: argument parsing and command dispatch.

use std::io::Write;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

use crate::catalog::{Catalog, Rule, load, load_registry};
use crate::evaluate::{applicable, check_record, evaluate, leaves_project, normalise};
use crate::record::{Record, Results, parse_results};
use crate::release::only_uss_changes;
use crate::report::render;
use crate::source::{Embedded, Source};

mod files;
mod git;

pub use files::{DirSource, OsProject, project_root, read_record};

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
    /// Export the rule catalog for other tools.
    Export {
        /// Print the catalog as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Check a project's record against the standard and the project's files.
    Check {
        /// The project record (`.uss/record.yaml`).
        record: PathBuf,
    },
    /// Write the Markdown audit report for an audit's results.
    Report {
        /// The project record.
        #[arg(long, default_value = ".uss/record.yaml")]
        record: PathBuf,
        /// Where to write the report (default: RESULTS with `.yaml` replaced by `.md`).
        #[arg(long)]
        out: Option<PathBuf>,
        /// The audit results file.
        results: PathBuf,
    },
    /// Check that the latest audits are for the release commit, and write the reports.
    ReleaseCheck {
        /// The project record.
        #[arg(long, default_value = ".uss/record.yaml")]
        record: PathBuf,
        /// Where to write the reports.
        #[arg(long, default_value = "uss-report.md")]
        out: PathBuf,
        /// Exit with 1 when any record is not compliant.
        #[arg(long)]
        require_compliant: bool,
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

/// Prints the whole catalog as indented JSON.
fn run_export(src: &dyn Source, json: bool, out: &mut dyn Write, err: &mut dyn Write) -> i32 {
    if !json {
        let _ = writeln!(err, "export: only --json is supported");
        return EXIT_USAGE;
    }
    let catalog = match load(src) {
        Ok(catalog) => catalog,
        Err(diagnostics) => {
            for d in diagnostics {
                if writeln!(err, "{d}").is_err() {
                    break;
                }
            }
            return 1;
        }
    };
    let text = match serde_json::to_string_pretty(&catalog.export()) {
        Ok(text) => text,
        Err(e) => {
            let _ = writeln!(err, "export: {e}");
            return 1;
        }
    };
    writeln!(out, "{text}").map_or(EXIT_USAGE, |()| 0)
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

/// Checks the record and prints each problem, then a summary line, to `out`.
fn run_check(
    src: &dyn Source,
    record_path: &Path,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> i32 {
    let (catalog, record) = match load_project(src, record_path, err) {
        Ok(loaded) => loaded,
        Err(code) => return code,
    };
    let project = OsProject {
        root: project_root(record_path),
    };
    let diagnostics = check_record(src, &catalog, &record, &project);
    let n = diagnostics.len();
    let noun = if n == 1 { "problem" } else { "problems" };
    let written = diagnostics
        .iter()
        .try_for_each(|d| writeln!(out, "{d}"))
        .and_then(|()| {
            if n == 0 {
                writeln!(out, "uss check: ok").map(|()| 0)
            } else {
                writeln!(out, "uss check: {n} {noun}").map(|()| 1)
            }
        });
    written.unwrap_or(EXIT_USAGE)
}

/// The report path next to `results`: `.yaml` replaced by `.md`, or `.md` appended.
fn default_report_path(results: &str) -> String {
    format!("{}.md", results.strip_suffix(".yaml").unwrap_or(results))
}

/// Reads and parses the audit results file at `path`.
fn read_results(src: &dyn Source, path: &Path) -> Result<Results, String> {
    let name = path.to_string_lossy().replace('\\', "/");
    let data =
        std::fs::read(path).map_err(|e| format!("{name}: cannot read the results file: {e}"))?;
    parse_results(src, &name, &data).map_err(|e| e.to_string())
}

/// Evaluates the audit `results_path` against the record and writes the
/// Markdown report to `out_path`, then prints the status and the scores.
/// A record that `uss check` rejects stops it: the problems go to `out`.
fn run_report(
    src: &dyn Source,
    record_path: &Path,
    results_path: &Path,
    out_path: Option<&Path>,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> i32 {
    let (catalog, record) = match load_project(src, record_path, err) {
        Ok(loaded) => loaded,
        Err(code) => return code,
    };
    let project = OsProject {
        root: project_root(record_path),
    };
    let diagnostics = check_record(src, &catalog, &record, &project);
    if !diagnostics.is_empty() {
        let written = diagnostics.iter().try_for_each(|d| writeln!(out, "{d}"));
        return written.map_or(EXIT_USAGE, |()| 1);
    }
    let results_name = results_path.to_string_lossy().replace('\\', "/");
    let results = match read_results(src, results_path) {
        Ok(results) => results,
        Err(e) => {
            let _ = writeln!(err, "{e}");
            return 1;
        }
    };
    let evaluation = match evaluate(&catalog, &record, &results) {
        Ok(evaluation) => evaluation,
        Err(diagnostics) => {
            let written = diagnostics.iter().try_for_each(|d| writeln!(out, "{d}"));
            return written.map_or(EXIT_USAGE, |()| 1);
        }
    };
    let report = match render(src, &record.project.name, &evaluation) {
        Ok(report) => report,
        Err(e) => {
            let _ = writeln!(err, "{e}");
            return 1;
        }
    };
    let target = out_path.map_or_else(
        || PathBuf::from(default_report_path(&results_name)),
        Path::to_path_buf,
    );
    if let Err(e) = std::fs::write(&target, report) {
        let name = target.to_string_lossy().replace('\\', "/");
        let _ = writeln!(err, "{name}: cannot write the report: {e}");
        return 1;
    }
    let written = writeln!(out, "{}", evaluation.statement())
        .and_then(|()| writeln!(out, "{}", evaluation.score_line()));
    written.map_or(EXIT_USAGE, |()| 0)
}

/// What `release-check` keeps of one audited record.
struct Audited {
    report: String,
    statement: String,
    compliant: bool,
}

/// Returns `path` with forward slashes, for messages.
fn shown(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

/// Collects `record` and, depth first, the records of its components.
/// Problems with a component or its record go to `problems`.
fn collect_records(
    src: &dyn Source,
    path: &Path,
    record: Record,
    records: &mut Vec<(PathBuf, Record)>,
    problems: &mut Vec<String>,
) {
    let name = shown(path);
    let root = project_root(path);
    let components = record.components.clone();
    let version = record.standard.version.clone();
    records.push((path.to_path_buf(), record));
    for raw in &components {
        if leaves_project(raw) {
            problems.push(format!(
                "{name}: component path \"{raw}\" leaves the project"
            ));
            continue;
        }
        let dir = normalise(raw);
        if dir.is_empty() {
            problems.push(format!(
                "{name}: component path \"{raw}\" is the record's own directory"
            ));
            continue;
        }
        let child_path = root.join(dir).join(".uss").join("record.yaml");
        match read_record(src, &child_path) {
            Err(e) => problems.push(format!("{name}: {e}")),
            Ok(child) if child.standard.version != version => problems.push(format!(
                "{name}: component record \"{}\" pins USS {}; the root record pins USS {version}",
                shown(&child_path),
                child.standard.version
            )),
            Ok(child) => collect_records(src, &child_path, child, records, problems),
        }
    }
}

/// Checks that the audit commit `sha` is in the clone and an ancestor of
/// `HEAD`, and that nothing outside `.uss/` changed since. Problems go to `problems`.
fn check_commit(
    repo: &Path,
    name: &str,
    sha: &str,
    problems: &mut Vec<String>,
) -> Result<(), String> {
    if !git::commit_exists(repo, sha)? {
        problems.push(if git::is_shallow(repo)? {
            format!(
                "{name}: audited commit {sha} is not in this clone; the clone is shallow. Fetch full history (actions/checkout: fetch-depth: 0)."
            )
        } else {
            format!("{name}: audited commit {sha} not found")
        });
    } else if !git::is_ancestor(repo, sha)? {
        problems.push(format!(
            "{name}: audited commit {sha} is not an ancestor of HEAD"
        ));
    } else {
        let offending = only_uss_changes(&git::changed_files(repo, sha)?);
        if !offending.is_empty() {
            let list = offending.join("\n  ");
            problems.push(format!(
                "{name}: changes since the audit outside .uss/:\n  {list}"
            ));
        }
    }
    Ok(())
}

/// Checks the latest audit of one record against the git history, then
/// evaluates and renders it. Returns `None` when a problem stops it.
fn audit_record(
    src: &dyn Source,
    catalog: &Catalog,
    repo: &Path,
    path: &Path,
    record: &Record,
    problems: &mut Vec<String>,
) -> Result<Option<Audited>, String> {
    let name = shown(path);
    let Some(audit) = record.latest_audit() else {
        problems.push(format!("{name}: no audit recorded in {name}"));
        return Ok(None);
    };
    if leaves_project(&audit.results) {
        problems.push(format!(
            "{name}: audit results path \"{}\" leaves the project",
            audit.results
        ));
        return Ok(None);
    }
    let results_path = project_root(path).join(normalise(&audit.results));
    let results = match read_results(src, &results_path) {
        Ok(results) => results,
        Err(e) => {
            problems.push(format!("{name}: {e}"));
            return Ok(None);
        }
    };
    check_commit(repo, &name, &results.commit, problems)?;
    let evaluation = match evaluate(catalog, record, &results) {
        Ok(evaluation) => evaluation,
        Err(diagnostics) => {
            problems.extend(diagnostics.iter().map(|d| format!("{name}: {}", d.message)));
            return Ok(None);
        }
    };
    Ok(Some(Audited {
        report: render(src, &record.project.name, &evaluation)?,
        statement: evaluation.statement(),
        compliant: evaluation.status.compliant,
    }))
}

/// Proves the latest audits are for the release commit, then writes the reports.
/// Every problem that `uss check` finds in the records is a problem here too.
fn run_release_check(
    src: &dyn Source,
    record_path: &Path,
    out_path: &Path,
    require_compliant: bool,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> i32 {
    let (catalog, root) = match load_project(src, record_path, err) {
        Ok(loaded) => loaded,
        Err(code) => return code,
    };
    let project = OsProject {
        root: project_root(record_path),
    };
    let mut problems: Vec<String> = check_record(src, &catalog, &root, &project)
        .iter()
        .map(ToString::to_string)
        .collect();
    let mut records = Vec::new();
    collect_records(src, record_path, root, &mut records, &mut problems);
    let repo = project_root(record_path);
    let mut audited = Vec::new();
    for (path, record) in &records {
        match audit_record(src, &catalog, &repo, path, record, &mut problems) {
            Ok(Some(a)) => audited.push(a),
            Ok(None) => {}
            Err(e) => {
                let _ = writeln!(err, "{e}");
                return 1;
            }
        }
    }
    if !problems.is_empty() {
        // `check_record` and `collect_records` both report a bad component path.
        let mut seen = std::collections::BTreeSet::new();
        problems.retain(|p| seen.insert(p.clone()));
        let written = problems.iter().try_for_each(|p| writeln!(out, "{p}"));
        return written.map_or(EXIT_USAGE, |()| 1);
    }
    let reports: Vec<&str> = audited.iter().map(|a| a.report.as_str()).collect();
    if let Err(e) = std::fs::write(out_path, reports.join("\n---\n\n")) {
        let _ = writeln!(err, "{}: cannot write the report: {e}", shown(out_path));
        return 1;
    }
    if audited
        .iter()
        .try_for_each(|a| writeln!(out, "{}", a.statement))
        .is_err()
    {
        return EXIT_USAGE;
    }
    i32::from(require_compliant && audited.iter().any(|a| !a.compliant))
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
                Command::Export { json } => run_export(src.as_ref(), json, out, err),
                Command::Check { record } => run_check(src.as_ref(), &record, out, err),
                Command::Report {
                    record,
                    out: out_path,
                    results,
                } => run_report(
                    src.as_ref(),
                    &record,
                    &results,
                    out_path.as_deref(),
                    out,
                    err,
                ),
                Command::ReleaseCheck {
                    record,
                    out: out_path,
                    require_compliant,
                } => run_release_check(
                    src.as_ref(),
                    &record,
                    &out_path,
                    require_compliant,
                    out,
                    err,
                ),
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
        // The standard's version and the crate's version stay equal.
        assert_eq!(out, format!("uss {}\n", env!("CARGO_PKG_VERSION")));
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

    /// Writes the fixture standard, the schemas and the report template to a temp dir.
    fn fixture_root() -> tempfile::TempDir {
        let dir = tempfile::TempDir::new().unwrap();
        let shipped = [
            "schema/record.schema.json",
            "schema/audit-results.schema.json",
            "templates/audit-report.md.j2",
        ]
        .map(|path| (path.to_string(), Embedded.read(path).unwrap().into_owned()));
        let files = crate::testfixture::standard().0.into_iter().chain(shipped);
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

    /// Writes a project (`.uss/record.yaml` plus `files`) and returns the record path.
    fn write_project(dir: &Path, deviation_doc: &str, files: &[&str]) -> String {
        std::fs::create_dir_all(dir.join(".uss")).unwrap();
        let text = format!(
            concat!(
                "standard: {{ version: 0.1.0 }}\n",
                "project: {{ name: p }}\n",
                "facts: {{ has-ui: false, has-web-ui: false, uses-database: true }}\n",
                "decisions: {{ USS-EXT-001: {{ adopt: false }} }}\n",
                "deviations:\n",
                "  - {{ rule: USS-SEC-002, document: '{}', approved-by: z, date: 2026-01-01 }}\n",
            ),
            deviation_doc
        );
        let record = dir.join(".uss").join("record.yaml");
        std::fs::write(&record, text).unwrap();
        for f in files {
            let target = dir.join(f);
            std::fs::create_dir_all(target.parent().unwrap()).unwrap();
            std::fs::write(target, "doc").unwrap();
        }
        record.to_str().unwrap().to_string()
    }

    #[test]
    fn check_ok_and_problems_cli() {
        let std_root = fixture_root();
        let std_arg = std_root.path().to_str().unwrap();
        let proj = tempfile::TempDir::new().unwrap();
        let rec = write_project(proj.path(), "docs/d.md", &["docs/d.md"]);
        let (code, out, err) = run_capture(&["--standard-root", std_arg, "check", &rec]);
        assert_eq!(
            (code, out.as_str(), err.as_str()),
            (0, "uss check: ok\n", "")
        );

        std::fs::remove_file(proj.path().join("docs/d.md")).unwrap();
        let (code, out, _err) = run_capture(&["--standard-root", std_arg, "check", &rec]);
        assert_eq!(code, 1);
        assert!(
            out.contains("deviation document \"docs/d.md\" not found"),
            "stdout: {out}"
        );
        assert!(out.ends_with("uss check: 1 problem\n"), "stdout: {out}");
    }

    #[test]
    fn check_backslash_path_cli() {
        let std_root = fixture_root();
        let std_arg = std_root.path().to_str().unwrap();
        let proj = tempfile::TempDir::new().unwrap();
        let rec = write_project(proj.path(), "docs\\d.md", &["docs/d.md"]);
        let (code, out, err) = run_capture(&["--standard-root", std_arg, "check", &rec]);
        assert_eq!(code, 0, "stdout: {out} stderr: {err}");
        assert_eq!(out, "uss check: ok\n");
    }

    /// Writes an audit results file with `entries` into `dir`; returns its path.
    fn write_results(dir: &Path, entries: &str) -> String {
        let path = dir.join("audit.yaml");
        let text = format!(
            "standard: {{ version: 0.1.0 }}\ncommit: {}\ndate: 2026-11-21\nauditor: tester\nresults:\n{entries}",
            "a".repeat(40)
        );
        std::fs::write(&path, text).unwrap();
        path.to_str().unwrap().to_string()
    }

    const GOOD_RESULTS: &str = concat!(
        "  USS-SEC-001: { result: pass, evidence: checked }\n",
        "  USS-SEC-002: { result: deviation, evidence: approved }\n",
    );

    #[test]
    fn run_report_writes_file() {
        let std_root = fixture_root();
        let std_arg = std_root.path().to_str().unwrap();
        let proj = tempfile::TempDir::new().unwrap();
        let rec = write_project(proj.path(), "docs/d.md", &["docs/d.md"]);
        let res = write_results(proj.path(), GOOD_RESULTS);
        let (code, out, err) =
            run_capture(&["--standard-root", std_arg, "report", "--record", &rec, &res]);
        assert_eq!(code, 0, "stderr: {err}");
        assert!(out.starts_with("Compliant with USS 0.1.0"), "stdout: {out}");
        assert!(out.ends_with("Safety 50%, Features 0%.\n"), "stdout: {out}");
        let report = std::fs::read_to_string(proj.path().join("audit.md")).unwrap();
        assert!(report.starts_with("# USS audit report: p\n"), "{report}");
        assert!(report.contains("| USS-SEC-002 | SHOULD |"), "{report}");
    }

    #[test]
    fn run_report_out_flag_and_not_compliant() {
        let std_root = fixture_root();
        let std_arg = std_root.path().to_str().unwrap();
        let proj = tempfile::TempDir::new().unwrap();
        let rec = write_project(proj.path(), "docs/d.md", &["docs/d.md"]);
        let res = write_results(
            proj.path(),
            "  USS-SEC-001: { result: fail, evidence: nope }\n",
        );
        let target = proj.path().join("custom.txt");
        let (code, out, err) = run_capture(&[
            "--standard-root",
            std_arg,
            "report",
            "--record",
            &rec,
            "--out",
            target.to_str().unwrap(),
            &res,
        ]);
        assert_eq!(code, 0, "stderr: {err}");
        assert!(
            out.starts_with("Not compliant with USS 0.1.0"),
            "stdout: {out}"
        );
        assert!(target.is_file());
        assert!(!proj.path().join("audit.md").exists());
    }

    #[test]
    fn run_report_consistency_error() {
        let std_root = fixture_root();
        let std_arg = std_root.path().to_str().unwrap();
        let proj = tempfile::TempDir::new().unwrap();
        let rec = write_project(proj.path(), "docs/d.md", &["docs/d.md"]);
        let res = write_results(
            proj.path(),
            "  USS-SEC-001: { result: pass, evidence: ok }\n  USS-NOPE-001: { result: pass, evidence: ok }\n",
        );
        let (code, out, _err) =
            run_capture(&["--standard-root", std_arg, "report", "--record", &rec, &res]);
        assert_eq!(code, 1);
        assert!(
            out.contains("audit.yaml: USS-NOPE-001: unknown rule"),
            "stdout: {out}"
        );
        assert!(!proj.path().join("audit.md").exists());
    }

    #[test]
    fn run_report_rejects_invalid_record() {
        let std_root = fixture_root();
        let std_arg = std_root.path().to_str().unwrap();
        let proj = tempfile::TempDir::new().unwrap();
        // A misspelled fact is an unknown fact, and the fact it was meant to
        // answer is left unanswered, so the record checks reject the record.
        let typo = RECORD_HEAD.replace("uses-database", "uses-databse");
        write_file(proj.path(), ".uss/record.yaml", &typo);
        let rec = proj.path().join(".uss/record.yaml");
        let res = write_results(
            proj.path(),
            "  USS-SEC-001: { result: pass, evidence: checked }\n",
        );
        let (code, out, err) = run_capture(&[
            "--standard-root",
            std_arg,
            "report",
            "--record",
            rec.to_str().unwrap(),
            &res,
        ]);
        assert_eq!(code, 1, "stdout: {out} stderr: {err}");
        assert!(
            out.contains("record.yaml: unknown fact \"uses-databse\""),
            "stdout: {out}"
        );
        assert!(
            out.contains("record.yaml: fact \"uses-database\" is not answered"),
            "stdout: {out}"
        );
        assert!(!out.contains("Compliant"), "stdout: {out}");
        assert!(!proj.path().join("audit.md").exists());
    }

    #[test]
    fn run_report_missing_and_invalid_results() {
        let std_root = fixture_root();
        let std_arg = std_root.path().to_str().unwrap();
        let proj = tempfile::TempDir::new().unwrap();
        let rec = write_project(proj.path(), "docs/d.md", &["docs/d.md"]);
        let missing = proj.path().join("nope.yaml");
        let (code, out, err) = run_capture(&[
            "--standard-root",
            std_arg,
            "report",
            "--record",
            &rec,
            missing.to_str().unwrap(),
        ]);
        assert_eq!((code, out.as_str()), (1, ""));
        assert!(err.contains("nope.yaml"), "stderr: {err}");

        let bad = proj.path().join("bad.yaml");
        std::fs::write(&bad, "standard: 1\n").unwrap();
        let (code, out, err) = run_capture(&[
            "--standard-root",
            std_arg,
            "report",
            "--record",
            &rec,
            bad.to_str().unwrap(),
        ]);
        assert_eq!((code, out.as_str()), (1, ""));
        assert!(err.contains("bad.yaml"), "stderr: {err}");
        assert!(!proj.path().join("bad.md").exists());
    }

    #[test]
    fn default_report_path_swaps_extension() {
        assert_eq!(default_report_path("a/2026.yaml"), "a/2026.md");
        assert_eq!(default_report_path("a/2026.yml"), "a/2026.yml.md");
        assert_eq!(default_report_path("a/2026"), "a/2026.md");
    }

    #[test]
    fn version_error_text() {
        assert_eq!(
            version_error("1.2.3", "0.1.0"),
            "record pins USS 1.2.3; this uss carries USS 0.1.0. Use uss 1.2.3: download it from the v1.2.3 release, or run: cargo install --locked --git https://github.com/HaleTechCo/Universal-Software-Standard --tag v1.2.3 uss"
        );
    }

    #[test]
    fn run_export_json() {
        let (code, out, err) = run_capture(&["export", "--json"]);
        assert_eq!(code, 0, "stderr: {err}");
        assert!(out.ends_with("}\n"), "stdout: {out}");
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["version"], env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn run_export_no_format() {
        let (code, out, err) = run_capture(&["export"]);
        assert_eq!((code, out.as_str()), (2, ""));
        assert_eq!(err, "export: only --json is supported\n");
    }

    #[test]
    fn run_unknown_command() {
        let (code, _out, err) = run_capture(&["bogus"]);
        assert_eq!(code, 2);
        assert!(err.contains("bogus"), "stderr: {err}");
    }

    // ---- release-check ----

    /// Returns a `git` command without the variables that point git at
    /// another repository. A test run from a git hook inherits them, and
    /// they would make git act on the developer's repository.
    fn git_command() -> std::process::Command {
        let mut cmd = std::process::Command::new("git");
        for var in git::REPOSITORY_ENV {
            cmd.env_remove(var);
        }
        cmd
    }

    /// Runs git in `dir` with the user's global configuration and hooks kept
    /// out of the way: the hooks path is a directory in `dir` that does not exist.
    fn git(dir: &Path, args: &[&str]) -> String {
        let hooks = dir.join("no-hooks").to_string_lossy().replace('\\', "/");
        let output = git_command()
            .arg("-C")
            .arg(dir)
            .arg("-c")
            .arg(format!("core.hooksPath={hooks}"))
            .args([
                "-c",
                "user.name=uss-test",
                "-c",
                "user.email=uss-test@example.invalid",
                "-c",
                "commit.gpgsign=false",
                "-c",
                "core.autocrlf=false",
            ])
            .args(args)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "git {args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).unwrap().trim().to_string()
    }

    /// Returns a new git repository in a temp dir, or `None` when git is not installed.
    fn new_repo() -> Option<tempfile::TempDir> {
        if git_command().arg("--version").output().is_err() {
            println!("git is not installed; skipping");
            return None;
        }
        let dir = tempfile::TempDir::new().unwrap();
        git(dir.path(), &["init", "-q"]);
        Some(dir)
    }

    fn write_file(root: &Path, rel: &str, text: &str) {
        let target = root.join(rel);
        std::fs::create_dir_all(target.parent().unwrap()).unwrap();
        std::fs::write(target, text).unwrap();
    }

    fn commit_all(root: &Path, message: &str) -> String {
        git(root, &["add", "-A"]);
        git(root, &["commit", "-q", "--no-verify", "-m", message]);
        git(root, &["rev-parse", "HEAD"])
    }

    const RECORD_HEAD: &str = concat!(
        "standard: { version: 0.1.0 }
",
        "project: { name: p }
",
        "facts: { has-ui: false, has-web-ui: false, uses-database: false }
",
        "decisions: { USS-EXT-001: { adopt: false } }
",
    );
    const PASS: &str = "{ USS-SEC-001: { result: pass, evidence: checked } }";
    const AUDIT_ENTRY: &str = "audits:\n  - { date: 2026-11-21, results: .uss/audits/a.yaml, report: .uss/audits/a.md }\n";

    fn audit_yaml(commit: &str, entries: &str) -> String {
        format!(
            "standard: {{ version: 0.1.0 }}\ncommit: {commit}\ndate: 2026-11-21\nauditor: t\nresults: {entries}\n"
        )
    }

    /// Commits the source and a record with no audit; returns the commit.
    fn first_commit(root: &Path, extra_record: &str) -> String {
        write_file(root, "src/main.rs", "fn main() {}\n");
        write_file(
            root,
            ".uss/record.yaml",
            &format!("{RECORD_HEAD}{extra_record}"),
        );
        commit_all(root, "first")
    }

    /// Records an audit of `commit` in a second commit that touches only `.uss/`.
    fn audit_commit(root: &Path, commit: &str, extra_record: &str) -> String {
        write_file(root, ".uss/audits/a.yaml", &audit_yaml(commit, PASS));
        write_file(
            root,
            ".uss/record.yaml",
            &format!("{RECORD_HEAD}{extra_record}{AUDIT_ENTRY}"),
        );
        commit_all(root, "audit")
    }

    fn release_check(root: &Path, extra: &[&str]) -> (i32, String, String, PathBuf) {
        let record = root.join(".uss/record.yaml");
        let out_file = root.join("uss-report.md");
        let standard = fixture_root();
        let mut args = vec![
            "--standard-root",
            standard.path().to_str().unwrap(),
            "release-check",
            "--record",
            record.to_str().unwrap(),
            "--out",
            out_file.to_str().unwrap(),
        ];
        args.extend_from_slice(extra);
        let (code, out, err) = run_capture(&args);
        (code, out, err, out_file)
    }

    #[test]
    fn release_check_ok() {
        let Some(repo) = new_repo() else { return };
        let a = first_commit(repo.path(), "");
        audit_commit(repo.path(), &a, "");
        let (code, out, err, report) = release_check(repo.path(), &[]);
        assert_eq!(code, 0, "stdout: {out} stderr: {err}");
        assert_eq!(out.lines().count(), 1, "stdout: {out}");
        assert!(out.starts_with("Compliant with USS 0.1.0"), "stdout: {out}");
        assert!(report.is_file());
    }

    #[test]
    fn release_check_rejects_invalid_record() {
        let Some(repo) = new_repo() else { return };
        // A misspelled fact is an unknown fact, and the fact it was meant to
        // answer is left unanswered, so the record checks reject the record.
        let typo = RECORD_HEAD.replace("uses-database", "uses-databse");
        write_file(repo.path(), "src/main.rs", "fn main() {}\n");
        write_file(repo.path(), ".uss/record.yaml", &typo);
        let a = commit_all(repo.path(), "first");
        write_file(repo.path(), ".uss/audits/a.yaml", &audit_yaml(&a, PASS));
        write_file(
            repo.path(),
            ".uss/record.yaml",
            &format!("{typo}{AUDIT_ENTRY}"),
        );
        commit_all(repo.path(), "audit");
        let (code, out, err, report) = release_check(repo.path(), &["--require-compliant"]);
        assert_eq!(code, 1, "stdout: {out} stderr: {err}");
        assert!(
            out.contains("record.yaml: unknown fact \"uses-databse\""),
            "stdout: {out}"
        );
        assert!(!out.contains("Compliant"), "stdout: {out}");
        assert!(!report.exists());
    }

    #[test]
    fn release_check_changes_outside_uss() {
        let Some(repo) = new_repo() else { return };
        let a = first_commit(repo.path(), "");
        write_file(repo.path(), "src/main.rs", "fn main() { /* changed */ }\n");
        audit_commit(repo.path(), &a, "");
        let (code, out, _err, report) = release_check(repo.path(), &[]);
        assert_eq!(code, 1, "stdout: {out}");
        assert!(
            out.contains("changes since the audit outside .uss/:\n  src/main.rs\n"),
            "stdout: {out}"
        );
        assert!(!report.exists());
    }

    #[test]
    fn release_check_rename_into_uss_is_a_change() {
        let Some(repo) = new_repo() else { return };
        let a = first_commit(repo.path(), "");
        git(repo.path(), &["mv", "src/main.rs", "src/moved.rs"]);
        std::fs::create_dir_all(repo.path().join(".uss/src")).unwrap();
        git(repo.path(), &["mv", "src/moved.rs", ".uss/src/main.rs"]);
        audit_commit(repo.path(), &a, "");
        let (code, out, _err, report) = release_check(repo.path(), &[]);
        assert_eq!(code, 1, "stdout: {out}");
        assert!(out.contains("\n  src/main.rs"), "stdout: {out}");
        assert!(!report.exists());
    }

    #[test]
    fn release_check_no_audit() {
        let Some(repo) = new_repo() else { return };
        first_commit(repo.path(), "");
        let (code, out, _err, report) = release_check(repo.path(), &[]);
        assert_eq!(code, 1);
        assert!(
            out.contains("record.yaml: no audit recorded in "),
            "stdout: {out}"
        );
        assert!(!report.exists());
    }

    #[test]
    fn release_check_missing_commit() {
        let Some(repo) = new_repo() else { return };
        first_commit(repo.path(), "");
        let missing = "b".repeat(40);
        audit_commit(repo.path(), &missing, "");
        let (code, out, _err, _report) = release_check(repo.path(), &[]);
        assert_eq!(code, 1);
        assert!(
            out.contains(&format!("audited commit {missing} not found")),
            "stdout: {out}"
        );
    }

    #[test]
    fn release_check_not_an_ancestor() {
        let Some(repo) = new_repo() else { return };
        first_commit(repo.path(), "");
        git(repo.path(), &["checkout", "-q", "-b", "other"]);
        write_file(repo.path(), "src/other.rs", "\n");
        let side = commit_all(repo.path(), "side");
        git(repo.path(), &["checkout", "-q", "-"]);
        audit_commit(repo.path(), &side, "");
        let (code, out, _err, _report) = release_check(repo.path(), &[]);
        assert_eq!(code, 1);
        assert!(
            out.contains(&format!("audited commit {side} is not an ancestor of HEAD")),
            "stdout: {out}"
        );
    }

    #[test]
    fn release_check_shallow_clone() {
        let Some(repo) = new_repo() else { return };
        let a = first_commit(repo.path(), "");
        audit_commit(repo.path(), &a, "");
        write_file(repo.path(), "src/main.rs", "fn main() { /* later */ }\n");
        commit_all(repo.path(), "later");

        let path = repo.path().to_str().unwrap().replace('\\', "/");
        let url = if path.starts_with('/') {
            format!("file://{path}")
        } else {
            format!("file:///{path}")
        };
        let holder = tempfile::TempDir::new().unwrap();
        let clone = holder.path().join("clone");
        git(
            holder.path(),
            &["clone", "-q", "--depth", "1", &url, clone.to_str().unwrap()],
        );
        let (code, out, _err, report) = release_check(&clone, &[]);
        assert_eq!(code, 1, "stdout: {out}");
        assert!(out.contains("fetch-depth: 0"), "stdout: {out}");
        assert!(
            out.contains(&format!(
                "audited commit {a} is not in this clone; the clone is shallow."
            )),
            "stdout: {out}"
        );
        assert!(!report.exists());
    }

    #[test]
    fn release_check_not_a_repository() {
        if git_command().arg("--version").output().is_err() {
            println!("git is not installed; skipping");
            return;
        }
        let dir = tempfile::TempDir::new().unwrap();
        write_file(
            dir.path(),
            ".uss/record.yaml",
            &format!("{RECORD_HEAD}{AUDIT_ENTRY}"),
        );
        write_file(
            dir.path(),
            ".uss/audits/a.yaml",
            &audit_yaml(&"c".repeat(40), PASS),
        );
        let (code, out, err, _report) = release_check(dir.path(), &[]);
        assert_eq!(code, 1, "stdout: {out}");
        assert!(err.contains("git"), "stderr: {err}");
    }

    #[test]
    fn release_check_components() {
        let Some(repo) = new_repo() else { return };
        write_file(repo.path(), "svc/.uss/record.yaml", RECORD_HEAD);
        let a = first_commit(repo.path(), "components: [svc]\n");
        write_file(
            repo.path(),
            "svc/.uss/record.yaml",
            &format!("{RECORD_HEAD}{AUDIT_ENTRY}"),
        );
        write_file(repo.path(), "svc/.uss/audits/a.yaml", &audit_yaml(&a, PASS));
        audit_commit(repo.path(), &a, "components: [svc]\n");
        let (code, out, err, report) = release_check(repo.path(), &[]);
        assert_eq!(code, 0, "stdout: {out} stderr: {err}");
        assert_eq!(out.lines().count(), 2, "stdout: {out}");
        let text = std::fs::read_to_string(report).unwrap();
        assert_eq!(text.matches("\n---\n\n").count(), 1);
    }

    #[test]
    fn release_check_component_problems_are_all_reported() {
        let Some(repo) = new_repo() else { return };
        write_file(repo.path(), "svc/.uss/record.yaml", RECORD_HEAD);
        let a = first_commit(repo.path(), "components: [svc, ../x]\n");
        audit_commit(repo.path(), &a, "components: [svc, ../x]\n");
        let (code, out, _err, report) = release_check(repo.path(), &[]);
        assert_eq!(code, 1);
        assert_eq!(
            out.matches("component path \"../x\" leaves the project")
                .count(),
            1,
            "stdout: {out}"
        );
        assert!(
            out.contains("svc/.uss/record.yaml: no audit recorded in "),
            "stdout: {out}"
        );
        assert!(!report.exists());
    }

    #[test]
    fn release_check_require_compliant() {
        let Some(repo) = new_repo() else { return };
        let std_root = fixture_root();
        let std_arg = std_root.path().to_str().unwrap();
        let record = concat!(
            "standard: { version: 0.1.0 }\n",
            "project: { name: p }\n",
            "facts: { has-ui: false, has-web-ui: false, uses-database: false }\n",
            "decisions: { USS-EXT-001: { adopt: false } }\n",
        );
        write_file(repo.path(), "src/main.rs", "\n");
        write_file(repo.path(), ".uss/record.yaml", record);
        let a = commit_all(repo.path(), "first");
        let entries = "{ USS-SEC-001: { result: fail, evidence: missing } }";
        write_file(repo.path(), ".uss/audits/a.yaml", &audit_yaml(&a, entries));
        write_file(
            repo.path(),
            ".uss/record.yaml",
            &format!("{record}{AUDIT_ENTRY}"),
        );
        commit_all(repo.path(), "audit");

        let record_path = repo.path().join(".uss/record.yaml");
        let out_file = repo.path().join("uss-report.md");
        let base = [
            "--standard-root",
            std_arg,
            "release-check",
            "--record",
            record_path.to_str().unwrap(),
            "--out",
            out_file.to_str().unwrap(),
        ];
        let (code, out, err) = run_capture(&base);
        assert_eq!(code, 0, "stdout: {out} stderr: {err}");
        assert!(out.starts_with("Not compliant"), "stdout: {out}");
        assert!(out_file.is_file());
        std::fs::remove_file(&out_file).unwrap();

        let mut strict = base.to_vec();
        strict.push("--require-compliant");
        let (code, out, _err) = run_capture(&strict);
        assert_eq!(code, 1);
        assert!(out.starts_with("Not compliant"), "stdout: {out}");
        assert!(out_file.is_file());
    }
}
