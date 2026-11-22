//! Reading input text the same way on every platform.

/// Returns `text` without a leading UTF-8 byte order mark and with every
/// CRLF line ending turned into LF, so a file saved on Windows or by an
/// editor that writes a BOM reads the same as one with LF endings.
pub(crate) fn normalise(text: &str) -> String {
    text.strip_prefix('\u{feff}')
        .unwrap_or(text)
        .replace("\r\n", "\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalise_strips_bom_and_crlf() {
        assert_eq!(normalise("\u{feff}a\r\nb\r\n"), "a\nb\n");
        assert_eq!(normalise("a\nb"), "a\nb");
        // Only a leading BOM and only CRLF pairs change.
        assert_eq!(normalise("a\u{feff}\rb"), "a\u{feff}\rb");
    }
}
