//! Git queries for `uss release-check`, run through the `git` program.

use std::path::Path;
use std::process::{Command, Output};

/// Runs `git -C repo args…`.
fn run(repo: &Path, args: &[&str]) -> Result<Output, String> {
    Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .output()
        .map_err(|e| format!("cannot run git {}: {e}", args.join(" ")))
}

/// The error for a git command that failed unexpectedly.
fn failed(args: &[&str], output: &Output) -> String {
    let stderr = String::from_utf8_lossy(&output.stderr);
    format!("git {} failed: {}", args.join(" "), stderr.trim())
}

/// Returns whether commit `sha` exists in the repository at `repo`.
///
/// # Errors
///
/// Returns a message when git cannot run, or `repo` is not a git repository.
pub fn commit_exists(repo: &Path, sha: &str) -> Result<bool, String> {
    let object = format!("{sha}^{{commit}}");
    if run(repo, &["cat-file", "-e", &object])?.status.success() {
        return Ok(true);
    }
    // A missing commit and a missing repository both make `cat-file` fail.
    let probe = ["rev-parse", "--git-dir"];
    let output = run(repo, &probe)?;
    if output.status.success() {
        Ok(false)
    } else {
        Err(failed(&probe, &output))
    }
}

/// Returns whether the repository at `repo` is a shallow clone.
///
/// # Errors
///
/// Returns a message when git cannot run or fails.
pub fn is_shallow(repo: &Path) -> Result<bool, String> {
    let args = ["rev-parse", "--is-shallow-repository"];
    let output = run(repo, &args)?;
    if !output.status.success() {
        return Err(failed(&args, &output));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim() == "true")
}

/// Returns whether commit `sha` is an ancestor of `HEAD`.
///
/// # Errors
///
/// Returns a message when git cannot run or fails.
pub fn is_ancestor(repo: &Path, sha: &str) -> Result<bool, String> {
    let args = ["merge-base", "--is-ancestor", sha, "HEAD"];
    let output = run(repo, &args)?;
    match output.status.code() {
        Some(0) => Ok(true),
        Some(1) => Ok(false),
        _ => Err(failed(&args, &output)),
    }
}

/// Returns the paths changed between commit `sha` and `HEAD`, as git prints
/// them: relative to the repository root, with `/` separators.
///
/// # Errors
///
/// Returns a message when git cannot run or fails.
pub fn changed_files(repo: &Path, sha: &str) -> Result<Vec<String>, String> {
    let args = ["diff", "--name-only", "--no-renames", "-z", sha, "HEAD"];
    let output = run(repo, &args)?;
    if !output.status.success() {
        return Err(failed(&args, &output));
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .split('\0')
        .filter(|path| !path.is_empty())
        .map(String::from)
        .collect())
}
