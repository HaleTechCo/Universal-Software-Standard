//! Diagnostics: a problem found in a standard file.

use std::fmt;

/// A problem in a file, reported as `file:line: message`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    /// Path of the file, relative to the USS root.
    pub file: String,
    /// 1-based line number, or `0` when the problem has no line.
    pub line: usize,
    /// What is wrong.
    pub message: String,
}

impl Diagnostic {
    /// Creates a diagnostic with no line number.
    #[must_use]
    pub fn new(file: &str, message: impl Into<String>) -> Self {
        Self {
            file: file.to_string(),
            line: 0,
            message: message.into(),
        }
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.line == 0 {
            write!(f, "{}: {}", self.file, self.message)
        } else {
            write!(f, "{}:{}: {}", self.file, self.line, self.message)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_with_and_without_line() {
        let mut d = Diagnostic::new("a.md", "bad");
        assert_eq!(d.to_string(), "a.md: bad");
        d.line = 3;
        assert_eq!(d.to_string(), "a.md:3: bad");
    }
}
