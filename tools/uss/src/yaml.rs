//! The single place the YAML crate is called (see ADR 0002).

use serde::de::DeserializeOwned;
use serde_saphyr::{DefaultMessageFormatter, MessageFormatter};

use crate::text;

/// Parses YAML bytes into `T`.
///
/// A leading UTF-8 byte order mark is ignored and CRLF line endings are
/// treated as LF, so files edited on any platform parse the same way.
///
/// # Errors
///
/// Returns the parser's message when the input is not valid UTF-8, not valid
/// YAML, or does not match `T`.
pub fn from_slice<T: DeserializeOwned>(data: &[u8]) -> Result<T, String> {
    let raw = std::str::from_utf8(data).map_err(|e| format!("not valid UTF-8: {e}"))?;
    serde_saphyr::from_str(&text::normalise(raw)).map_err(|e| one_line(&e))
}

/// Renders a parse error as one line: `line L column C: <message>`, or just
/// `<message>` when the crate gives no location. No snippet, no source name.
fn one_line(err: &serde_saphyr::Error) -> String {
    let message = DefaultMessageFormatter.format_message(err.without_snippet());
    let message = message.lines().next().unwrap_or_default();
    match err.location() {
        Some(at) => format!("line {} column {}: {message}", at.line(), at.column()),
        None => message.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize, Debug, PartialEq)]
    #[serde(deny_unknown_fields)]
    struct Strict {
        a: String,
    }

    #[test]
    fn strict_struct_rejects_unknown_fields() {
        let ok: Strict = from_slice(b"a: x\n").unwrap();
        assert_eq!(ok, Strict { a: "x".into() });
        let err = from_slice::<Strict>(b"a: x\nb: y\n").unwrap_err();
        assert!(err.contains('b'), "{err}");
    }

    #[test]
    fn errors_are_one_line_with_location() {
        let inputs: [&[u8]; 2] = [b"a: x\nb: y\n", b"a: x\n  b: [y\n"];
        for input in inputs {
            let err = from_slice::<Strict>(input).unwrap_err();
            assert!(!err.contains('\n'), "{err:?}");
            assert!(!err.contains("<input>"), "{err:?}");
            assert!(!err.starts_with("error:"), "{err:?}");
            assert!(err.starts_with("line "), "{err:?}");
        }
    }

    #[test]
    fn parses_into_json_value() {
        let v: serde_json::Value = from_slice(b"a: [1, two]\nb: {c: true}\n").unwrap();
        assert_eq!(v["a"][1], "two");
        assert_eq!(v["b"]["c"], true);
    }

    #[test]
    fn bom_and_crlf_are_ignored() {
        let v: Strict = from_slice(b"\xEF\xBB\xBFa: x\r\n").unwrap();
        assert_eq!(v.a, "x");
        let v: serde_json::Value = from_slice(b"a: |\r\n  one\r\n  two\r\n").unwrap();
        assert_eq!(v["a"], "one\ntwo\n");
    }
}
