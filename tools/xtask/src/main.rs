//! Repository automation. `cargo xtask check` runs every check.

use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

/// The cargo-deny release that `cargo xtask check` installs under `.tools`.
const CARGO_DENY_VERSION: &str = "0.22.0";

/// Returns a message when the active compiler is not the pinned rustup toolchain.
///
/// A sysroot comes from rustup when it has a `.rustup` path component (the
/// default `RUSTUP_HOME`), or when its parent directory is named `toolchains`
/// (rustup's `<RUSTUP_HOME>/toolchains/<name>` layout for any `RUSTUP_HOME`).
fn toolchain_problem(channel: &str, release: &str, sysroot: &str) -> Option<String> {
    let parts: Vec<&str> = sysroot
        .trim_end_matches(['/', '\\'])
        .split(['/', '\\'])
        .collect();
    let from_rustup = parts.contains(&".rustup")
        || parts
            .len()
            .checked_sub(2)
            .and_then(|parent| parts.get(parent))
            .is_some_and(|parent| *parent == "toolchains");
    if release == channel && from_rustup {
        return None;
    }
    Some(format!(
        "The active rustc is {release} at {sysroot}. This repository needs the rustup toolchain {channel}. Put the rustup bin directory first on PATH. Then run: rustup toolchain install"
    ))
}

/// Reads the `channel` value from the text of `rust-toolchain.toml`.
fn channel_from(toolchain_toml: &str) -> Option<String> {
    toolchain_toml.lines().find_map(|line| {
        let (key, value) = line.split_once('=')?;
        (key.trim() == "channel").then(|| value.trim().trim_matches('"').to_owned())
    })
}

/// Reads the `release` value from the output of `rustc -vV`.
fn release_from(version_output: &str) -> Option<String> {
    version_output
        .lines()
        .find_map(|line| line.strip_prefix("release: "))
        .map(|release| release.trim().to_owned())
}

/// The directory that holds the root `Cargo.toml`.
fn workspace_root() -> PathBuf {
    let xtask_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    xtask_dir
        .ancestors()
        .nth(2)
        .map_or_else(|| xtask_dir.to_path_buf(), Path::to_path_buf)
}

/// The cargo that started this run, so every step uses the pinned toolchain.
fn cargo_program() -> String {
    std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned())
}

/// Runs a program in the workspace root. Returns its stdout when it succeeds.
fn capture(root: &Path, program: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(program)
        .args(args)
        .current_dir(root)
        .output()
        .map_err(|error| format!("Cannot run {program}: {error}"))?;
    if !output.status.success() {
        return Err(format!("{program} {} failed.", args.join(" ")));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

/// Runs a program in the workspace root and passes its output through.
fn run(root: &Path, program: &str, args: &[&str]) -> Result<(), String> {
    let status = Command::new(program)
        .args(args)
        .current_dir(root)
        .status()
        .map_err(|error| format!("Cannot run {program}: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{program} {} failed.", args.join(" ")))
    }
}

/// Step 0: the active compiler must be the pinned rustup toolchain.
fn guard_toolchain(root: &Path) -> Result<(), String> {
    let text = std::fs::read_to_string(root.join("rust-toolchain.toml"))
        .map_err(|error| format!("Cannot read rust-toolchain.toml: {error}"))?;
    let channel =
        channel_from(&text).ok_or_else(|| "rust-toolchain.toml has no channel.".to_owned())?;
    let version = capture(root, "rustc", &["-vV"])?;
    let release = release_from(&version).ok_or_else(|| "rustc -vV has no release.".to_owned())?;
    let sysroot = capture(root, "rustc", &["--print", "sysroot"])?;
    toolchain_problem(&channel, &release, &sysroot).map_or(Ok(()), Err)
}

/// Step 5: run cargo-deny from `.tools`, and install it first when it is missing.
fn deny(root: &Path, cargo: &str) -> Result<(), String> {
    let tools = root.join(".tools");
    let binary = tools
        .join("bin")
        .join(format!("cargo-deny{}", std::env::consts::EXE_SUFFIX));
    if !binary.exists() {
        run(
            root,
            cargo,
            &[
                "install",
                "cargo-deny",
                "--locked",
                "--version",
                CARGO_DENY_VERSION,
                "--root",
                &tools.to_string_lossy(),
            ],
        )?;
    }
    run(root, &binary.to_string_lossy(), &["check"])
}

/// Runs every check in order and stops at the first failure.
fn check() -> Result<(), String> {
    let root = workspace_root();
    let cargo = cargo_program();
    let cargo = cargo.as_str();

    println!("==> toolchain guard");
    guard_toolchain(&root)?;
    println!("==> cargo fmt");
    run(&root, cargo, &["fmt", "--all", "--check"])?;
    println!("==> cargo clippy");
    run(
        &root,
        cargo,
        &[
            "clippy",
            "--locked",
            "--workspace",
            "--all-targets",
            "--all-features",
            "--",
            "-D",
            "warnings",
        ],
    )?;
    println!("==> cargo test");
    run(
        &root,
        cargo,
        &["test", "--locked", "--workspace", "--all-features"],
    )?;
    println!("==> wasm32 library clippy");
    run(
        &root,
        cargo,
        &[
            "clippy",
            "--locked",
            "-p",
            "uss",
            "--lib",
            "--no-default-features",
            "--target",
            "wasm32-unknown-unknown",
            "--",
            "-D",
            "warnings",
        ],
    )?;
    println!("==> wasm32 library build");
    run(
        &root,
        cargo,
        &[
            "build",
            "--locked",
            "-p",
            "uss",
            "--lib",
            "--no-default-features",
            "--target",
            "wasm32-unknown-unknown",
        ],
    )?;
    println!("==> cargo deny");
    deny(&root, cargo)?;
    println!("==> uss lint");
    run(
        &root,
        cargo,
        &["run", "--locked", "--quiet", "-p", "uss", "--", "lint"],
    )?;
    println!("==> uss check");
    run(
        &root,
        cargo,
        &[
            "run",
            "--locked",
            "--quiet",
            "-p",
            "uss",
            "--",
            "check",
            ".uss/record.yaml",
        ],
    )
}

fn main() -> ExitCode {
    if std::env::args().nth(1).as_deref() != Some("check") {
        println!("Usage: cargo xtask check");
        return ExitCode::FAILURE;
    }
    match check() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{channel_from, release_from, toolchain_problem};

    #[test]
    fn toolchain_match_is_ok() {
        assert_eq!(
            toolchain_problem(
                "1.100.0",
                "1.100.0",
                r"C:\Users\zac\.rustup\toolchains\1.100.0-x86_64-pc-windows-msvc"
            ),
            None
        );
    }

    #[test]
    fn toolchain_unix_sysroot_is_ok() {
        assert_eq!(
            toolchain_problem(
                "1.100.0",
                "1.100.0",
                "/home/runner/.rustup/toolchains/1.100.0-x86_64-unknown-linux-gnu"
            ),
            None
        );
    }

    #[test]
    fn toolchain_custom_rustup_home_is_ok() {
        assert_eq!(
            toolchain_problem(
                "1.100.0",
                "1.100.0",
                "/usr/local/rustup/toolchains/1.100.0-x86_64-unknown-linux-gnu"
            ),
            None
        );
        assert_eq!(
            toolchain_problem(
                "1.100.0",
                "1.100.0",
                r"D:\rustup\toolchains\1.100.0-x86_64-pc-windows-msvc\"
            ),
            None
        );
    }

    #[test]
    fn toolchain_wrong_release() {
        let problem = toolchain_problem("1.100.0", "1.99.0", "/home/a/.rustup/toolchains/x");
        assert!(problem.is_some_and(|m| m.contains("The active rustc is 1.99.0")));
    }

    #[test]
    fn toolchain_standalone_sysroot() {
        let problem = toolchain_problem(
            "1.100.0",
            "1.100.0",
            r"C:\Program Files\Rust stable LLVM 1.100",
        );
        assert!(problem.is_some_and(|m| m.ends_with("Then run: rustup toolchain install")));
    }

    #[test]
    fn parses_channel_and_release() {
        assert_eq!(
            channel_from("[toolchain]\nchannel = \"1.100.0\"\n").as_deref(),
            Some("1.100.0")
        );
        assert_eq!(
            release_from("rustc 1.100.0\nrelease: 1.100.0\n").as_deref(),
            Some("1.100.0")
        );
    }
}
