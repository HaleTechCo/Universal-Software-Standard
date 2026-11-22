//! Evaluating a project record against the catalog.

mod check;
mod facts;
mod project;

pub use check::check_record;
pub use facts::{applicable, resolve_facts};
pub use project::{MemProject, ProjectFiles, SubProject};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::{Catalog, Rule, load};
    use crate::record::{Record, parse_record};
    use crate::source::Embedded;
    use crate::testfixture::standard;

    fn catalog() -> Catalog {
        load(&standard()).unwrap()
    }

    fn record(extra: &str) -> Record {
        let yaml = format!("standard: {{ version: 0.1.0 }}\nproject: {{ name: p }}\n{extra}");
        let yaml = if extra.contains("facts:") {
            yaml
        } else {
            format!("{yaml}facts: {{}}\n")
        };
        parse_record(&Embedded, "r.yaml", yaml.as_bytes()).unwrap()
    }

    fn ids(rules: &[&Rule]) -> Vec<String> {
        rules.iter().map(|r| r.id.clone()).collect()
    }

    #[test]
    fn resolve_facts_rules() {
        let c = catalog();
        let r = record(
            "stack: { languages: [go] }\nfacts: { has-web-ui: true, uses-database: false }\n",
        );
        let f = resolve_facts(&c, &r);
        assert!(f.contains("has-web-ui"));
        assert!(f.contains("has-ui"));
        assert!(f.contains("lang.go"));
        assert!(!f.contains("uses-database"));
        assert!(!f.contains("has-plugin-interface"));

        let r = record("decisions: { USS-EXT-001: { adopt: true } }\n");
        assert!(resolve_facts(&c, &r).contains("has-plugin-interface"));
        let r = record("decisions: { USS-EXT-001: { adopt: false } }\n");
        assert!(!resolve_facts(&c, &r).contains("has-plugin-interface"));
    }

    #[test]
    fn resolve_facts_stack_kinds() {
        let c = catalog();
        let r = record("stack: { frameworks: [react], platforms: [docker], tools: [make] }\n");
        let f = resolve_facts(&c, &r);
        for want in ["framework.react", "platform.docker", "tool.make"] {
            assert!(f.contains(want), "{want} missing from {f:?}");
        }
    }

    #[test]
    fn applicable_sets() {
        let c = catalog();
        let a = applicable(&c, &record(""));
        assert_eq!(ids(&a), ["USS-EXT-001", "USS-SEC-001"]);

        let a = applicable(&c, &record("decisions: { USS-EXT-001: { adopt: true } }\n"));
        assert_eq!(ids(&a), ["USS-EXT-001", "USS-EXT-002", "USS-SEC-001"]);

        let a = applicable(&c, &record("stack: { languages: [go] }\n"));
        assert_eq!(ids(&a), ["USS-EXT-001", "USS-GO-001", "USS-SEC-001"]);
        assert!(!ids(&a).contains(&"USS-SEC-004".to_string()));
    }

    #[test]
    fn applicable_follows_implied_facts() {
        let c = catalog();
        let a = applicable(&c, &record("facts: { has-web-ui: true }\n"));
        assert!(ids(&a).contains(&"USS-SEC-003".to_string()));
    }
}
