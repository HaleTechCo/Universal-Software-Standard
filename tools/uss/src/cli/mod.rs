//! Command-line interface: argument parsing and command dispatch.

use std::io::Write;

use clap::Parser;

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
struct Cli {}

/// Runs the `uss` CLI with `args` (including the program name).
///
/// Returns the process exit code: `0` for success, `1` when problems were
/// found, `2` for a usage error. Normal output goes to `out`, diagnostics to `err`.
pub fn run(args: &[String], out: &mut dyn Write, err: &mut dyn Write) -> i32 {
    match Cli::try_parse_from(args) {
        Ok(Cli {}) => 0,
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
    fn run_unknown_command() {
        let (code, _out, err) = run_capture(&["bogus"]);
        assert_eq!(code, 2);
        assert!(err.contains("bogus"), "stderr: {err}");
    }
}
