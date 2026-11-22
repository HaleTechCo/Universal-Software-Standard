//! The single place the YAML crate is called (see ADR 0002).

use serde::de::DeserializeOwned;

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
    let data = data.strip_prefix(b"\xEF\xBB\xBF").unwrap_or(data);
    let text = std::str::from_utf8(data).map_err(|e| format!("not valid UTF-8: {e}"))?;
    let text = text.replace("\r\n", "\n");
    serde_saphyr::from_str(&text).map_err(|e| e.to_string())
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
