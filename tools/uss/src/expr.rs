//! Applicability expressions (`Applies when`).

use std::collections::BTreeSet;
use std::fmt;

use crate::catalog::registry::STACK_KINDS;

/// Deepest nesting the parser accepts, so hostile input cannot overflow the stack.
const MAX_DEPTH: usize = 64;

/// An applicability expression over facts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    /// Always true.
    Always,
    /// A fact, such as `has-ui` or `lang.go`.
    Ident(String),
    /// Negation.
    Not(Box<Expr>),
    /// Conjunction.
    And(Box<Expr>, Box<Expr>),
    /// Disjunction.
    Or(Box<Expr>, Box<Expr>),
}

impl Expr {
    /// Evaluates the expression; a fact missing from `facts` is false.
    #[must_use]
    pub fn eval(&self, facts: &BTreeSet<String>) -> bool {
        match self {
            Self::Always => true,
            Self::Ident(name) => facts.contains(name),
            Self::Not(x) => !x.eval(facts),
            Self::And(a, b) => a.eval(facts) && b.eval(facts),
            Self::Or(a, b) => a.eval(facts) || b.eval(facts),
        }
    }

    /// The identifiers the expression mentions, sorted and unique, without `always`.
    #[must_use]
    pub fn idents(&self) -> BTreeSet<String> {
        let mut out = BTreeSet::new();
        self.collect_idents(&mut out);
        out
    }

    fn collect_idents(&self, out: &mut BTreeSet<String>) {
        match self {
            Self::Always => {}
            Self::Ident(name) => {
                out.insert(name.clone());
            }
            Self::Not(x) => x.collect_idents(out),
            Self::And(a, b) | Self::Or(a, b) => {
                a.collect_idents(out);
                b.collect_idents(out);
            }
        }
    }

    /// Conjunction of two expressions; `Always` on either side yields the other side.
    #[must_use]
    pub fn and(a: Self, b: Self) -> Self {
        match (a, b) {
            (Self::Always, other) | (other, Self::Always) => other,
            (a, b) => Self::And(Box::new(a), Box::new(b)),
        }
    }

    /// Writes `self`, in parentheses when it is an `And` or an `Or`.
    fn fmt_operand(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if matches!(self, Self::And(..) | Self::Or(..)) {
            write!(f, "({self})")
        } else {
            write!(f, "{self}")
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Always => f.write_str("always"),
            Self::Ident(name) => f.write_str(name),
            Self::Not(x) => {
                f.write_str("not ")?;
                x.fmt_operand(f)
            }
            Self::And(a, b) => {
                a.fmt_operand(f)?;
                f.write_str(" and ")?;
                b.fmt_operand(f)
            }
            Self::Or(a, b) => {
                a.fmt_operand(f)?;
                f.write_str(" or ")?;
                b.fmt_operand(f)
            }
        }
    }
}

/// Parses an applicability expression.
///
/// # Errors
///
/// Returns a message when the text is not a valid expression: empty input,
/// a dangling operator, unbalanced parentheses, an invalid identifier or an
/// unknown stack kind.
pub fn parse(s: &str) -> Result<Expr, String> {
    let tokens = tokenize(s);
    let mut parser = Parser {
        tokens: &tokens,
        pos: 0,
        depth: 0,
    };
    let expr = parser.expr()?;
    match parser.peek() {
        None => Ok(expr),
        Some(t) => Err(format!("unexpected `{t}`")),
    }
}

/// Splits on whitespace; each parenthesis is a token of its own.
fn tokenize(s: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut start: Option<usize> = None;
    for (i, c) in s.char_indices() {
        let paren = c == '(' || c == ')';
        if paren || c.is_whitespace() {
            if let Some(st) = start.take() {
                out.push(&s[st..i]);
            }
            if paren {
                out.push(&s[i..=i]);
            }
        } else if start.is_none() {
            start = Some(i);
        }
    }
    if let Some(st) = start {
        out.push(&s[st..]);
    }
    out
}

struct Parser<'a> {
    tokens: &'a [&'a str],
    pos: usize,
    depth: usize,
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<&'a str> {
        self.tokens.get(self.pos).copied()
    }

    fn eat(&mut self, word: &str) -> bool {
        if self.peek() == Some(word) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn enter(&mut self) -> Result<(), String> {
        self.depth += 1;
        if self.depth > MAX_DEPTH {
            return Err("expression is nested too deeply".to_string());
        }
        Ok(())
    }

    fn expr(&mut self) -> Result<Expr, String> {
        let mut left = self.and()?;
        while self.eat("or") {
            let right = self.and()?;
            left = Expr::Or(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn and(&mut self) -> Result<Expr, String> {
        let mut left = self.unary()?;
        while self.eat("and") {
            let right = self.unary()?;
            left = Expr::And(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.eat("not") {
            self.enter()?;
            let inner = self.unary();
            self.depth -= 1;
            return Ok(Expr::Not(Box::new(inner?)));
        }
        self.atom()
    }

    fn atom(&mut self) -> Result<Expr, String> {
        let Some(token) = self.peek() else {
            return Err("unexpected end of expression".to_string());
        };
        self.pos += 1;
        match token {
            "always" => Ok(Expr::Always),
            "(" => {
                self.enter()?;
                let inner = self.expr();
                self.depth -= 1;
                let inner = inner?;
                if self.eat(")") {
                    Ok(inner)
                } else {
                    Err("missing `)`".to_string())
                }
            }
            ")" | "and" | "or" | "not" => Err(format!("unexpected `{token}`")),
            _ => ident(token).map(Expr::Ident),
        }
    }
}

/// Validates `fact` or `kind.fact`.
fn ident(token: &str) -> Result<String, String> {
    if let Some((kind, fact)) = token.split_once('.') {
        if !STACK_KINDS.contains(&kind) {
            return Err(format!("unknown stack kind \"{kind}\" in `{token}`"));
        }
        if !is_fact(fact) {
            return Err(format!("invalid identifier `{token}`"));
        }
    } else if !is_fact(token) {
        return Err(format!("invalid identifier `{token}`"));
    }
    Ok(token.to_string())
}

/// `[a-z0-9]+ ( "-" [a-z0-9]+ )*`
fn is_fact(s: &str) -> bool {
    s.split('-').all(|part| {
        !part.is_empty()
            && part
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
    })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::{Expr, parse};

    fn facts(names: &[&str]) -> BTreeSet<String> {
        names.iter().map(|s| (*s).to_string()).collect()
    }

    fn table() -> Vec<(&'static str, Vec<&'static str>, bool)> {
        vec![
            ("always", vec![], true),
            ("has-ui", vec!["has-ui"], true),
            ("a or b and c", vec!["b"], false),
            ("not a and b", vec!["b"], true),
            ("(a or b) and c", vec!["a", "c"], true),
            ("lang.go", vec!["lang.go"], true),
            ("missing", vec![], false),
            ("not (a or b)", vec!["c"], true),
        ]
    }

    #[test]
    fn eval_table() {
        for (src, set, want) in table() {
            let e = parse(src).unwrap();
            assert_eq!(e.eval(&facts(&set)), want, "{src}");
            assert_eq!(parse(&e.to_string()).unwrap(), e, "round trip of {src}");
        }
    }

    #[test]
    fn parse_errors() {
        for src in ["", "a and", "(a", "a b", "AND", "has_ui", "lang."] {
            assert!(parse(src).is_err(), "{src:?} should fail");
        }
        let msg = parse("os.linux").unwrap_err();
        assert!(msg.contains("unknown stack kind \"os\""), "{msg}");
    }

    #[test]
    fn idents_sorted_unique() {
        let e = parse("not b or (a and lang.go) or always").unwrap();
        assert_eq!(e.idents(), facts(&["a", "b", "lang.go"]));
    }

    #[test]
    fn display_forms() {
        assert_eq!(parse("a and b").unwrap().to_string(), "a and b");
        let both = Expr::and(parse("lang.go").unwrap(), parse("has-ui or b").unwrap());
        assert_eq!(both.to_string(), "lang.go and (has-ui or b)");
        let same = Expr::and(Expr::Always, parse("x").unwrap());
        assert_eq!(same.to_string(), "x");
        assert_eq!(parse("not (a or b)").unwrap().to_string(), "not (a or b)");
    }
}
