//! Fact resolution.

use std::collections::BTreeSet;

use crate::catalog::{Catalog, Level, Rule};
use crate::record::Record;

/// Computes the set of true facts for `r`.
///
/// Starts from the facts the record answers yes and the stack entries
/// (`lang.<x>`, `framework.<x>`, `platform.<x>`, `tool.<x>`), then repeats
/// until nothing changes: adds the facts each true fact implies, and the
/// facts enabled by every applicable `MAY` rule the record adopts.
#[must_use]
pub fn resolve_facts(c: &Catalog, r: &Record) -> BTreeSet<String> {
    let mut facts: BTreeSet<String> = r
        .facts
        .iter()
        .filter(|(_, yes)| **yes)
        .map(|(id, _)| id.clone())
        .collect();
    let stack = &r.stack;
    for (prefix, ids) in [
        ("lang", &stack.languages),
        ("framework", &stack.frameworks),
        ("platform", &stack.platforms),
        ("tool", &stack.tools),
    ] {
        facts.extend(ids.iter().map(|x| format!("{prefix}.{x}")));
    }

    loop {
        let before = facts.len();
        let implied: Vec<String> = facts
            .iter()
            .filter_map(|id| c.facts.get(id))
            .flat_map(|f| f.implies.iter().cloned())
            .collect();
        facts.extend(implied);
        let enabled: Vec<String> = c
            .rules
            .iter()
            .filter_map(|rule| rule.active().map(|a| (rule, a)))
            .filter(|(rule, a)| {
                a.level == Level::May
                    && a.applies.eval(&facts)
                    && r.decisions.get(&rule.id).is_some_and(|d| d.adopt)
            })
            .flat_map(|(_, a)| a.enables.iter().cloned())
            .collect();
        facts.extend(enabled);
        if facts.len() == before {
            return facts;
        }
    }
}

/// Returns the active rules that apply to `r`, sorted by ID.
#[must_use]
pub fn applicable<'c>(c: &'c Catalog, r: &Record) -> Vec<&'c Rule> {
    let facts = resolve_facts(c, r);
    let mut rules: Vec<&Rule> = c
        .rules
        .iter()
        .filter(|rule| rule.active().is_some_and(|a| a.applies.eval(&facts)))
        .collect();
    rules.sort_by(|a, b| a.id.cmp(&b.id));
    rules
}
