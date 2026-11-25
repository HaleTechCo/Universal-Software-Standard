//! The catalog as plain data, for tools that read it as JSON.

use serde::Serialize;

use super::{Catalog, Coverage, Fact, FactSource, Maintainer, Profile, Rule, RuleState};

/// The whole catalog, ready to serialise. Every field is always present; a
/// value that does not exist for an item is `null`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Export {
    /// The standard's version.
    pub version: String,
    /// The score categories, in registry order.
    pub categories: Vec<ExportCategory>,
    /// The applicability facts, sorted by ID.
    pub facts: Vec<ExportFact>,
    /// Every rule, retired ones included, sorted by ID.
    pub rules: Vec<ExportRule>,
    /// The vendor profiles, sorted by path.
    pub profiles: Vec<ExportProfile>,
}

/// A vendor profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportProfile {
    /// Path of the profile file.
    pub file: String,
    /// The tool's name.
    pub product: String,
    /// The vendor's name.
    pub vendor: String,
    /// `community` or `vendor`.
    pub maintainer: &'static str,
    /// The tool versions that the profile was checked against.
    pub versions: String,
    /// The mapping table, in file order.
    pub mappings: Vec<ExportMapping>,
}

/// One row of a profile's mapping table.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportMapping {
    /// The rule ID.
    pub rule: String,
    /// The rule's category name.
    pub category: String,
    /// The tool's rules or settings that satisfy the rule.
    pub tool: String,
    /// `full` or `partial`.
    pub coverage: &'static str,
    /// What else is needed, for partial coverage; `null` otherwise.
    pub needs: Option<String>,
    /// How an auditor checks it.
    pub verify: String,
}

/// A score category.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportCategory {
    /// Machine ID.
    pub id: String,
    /// Display name.
    pub name: String,
}

/// An applicability fact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportFact {
    /// The fact's ID.
    pub id: String,
    /// The question, when the developer is asked.
    pub question: Option<String>,
    /// The rule that sets the fact, when one does.
    pub enabled_by: Option<String>,
    /// When the answer is yes.
    pub definition: String,
    /// IDs of the facts that are also true when this one is.
    pub implies: Vec<String>,
}

/// A rule, active or retired.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportRule {
    /// The rule ID.
    pub id: String,
    /// The title.
    pub title: String,
    /// The level (`MUST`, `MUST NOT`, ...); `null` when retired.
    pub level: Option<String>,
    /// The effective condition (file and rule combined); `null` when retired.
    pub applies_when: Option<String>,
    /// The score category name; `null` when retired.
    pub category: Option<String>,
    /// How to check the rule; `null` when retired.
    pub verify: Option<String>,
    /// Sources of the rule; `null` when retired.
    pub references: Option<String>,
    /// IDs of the facts a `MAY` rule enables.
    pub enables: Vec<String>,
    /// IDs of the rules a module rule refines.
    pub refines: Vec<String>,
    /// `active` or `retired`.
    pub status: &'static str,
    /// The version that retired the rule.
    pub retired_in: Option<String>,
    /// The rule that replaces a retired rule.
    pub replaced_by: Option<String>,
    /// Path of the file the rule is in.
    pub file: String,
    /// The rule text; `null` when retired.
    pub body: Option<String>,
}

impl From<&Fact> for ExportFact {
    fn from(fact: &Fact) -> Self {
        let (question, enabled_by) = match &fact.source {
            FactSource::Question(q) => (Some(q.clone()), None),
            FactSource::EnabledBy(by) => (None, Some(by.clone())),
        };
        Self {
            id: fact.id.clone(),
            question,
            enabled_by,
            definition: fact.definition.clone(),
            implies: fact.implies.clone(),
        }
    }
}

impl From<&Rule> for ExportRule {
    fn from(rule: &Rule) -> Self {
        let base = Self {
            id: rule.id.clone(),
            title: rule.title.clone(),
            level: None,
            applies_when: None,
            category: None,
            verify: None,
            references: None,
            enables: Vec::new(),
            refines: Vec::new(),
            status: "retired",
            retired_in: None,
            replaced_by: None,
            file: rule.file.clone(),
            body: None,
        };
        match &rule.state {
            RuleState::Active(a) => Self {
                level: Some(a.level.to_string()),
                applies_when: Some(a.applies.to_string()),
                category: Some(a.category.clone()),
                verify: Some(a.verify.clone()),
                references: Some(a.references.clone()),
                enables: a.enables.clone(),
                refines: a.refines.clone(),
                status: "active",
                body: Some(a.body.clone()),
                ..base
            },
            RuleState::Retired {
                retired_in,
                replaced_by,
            } => Self {
                retired_in: Some(retired_in.clone()),
                replaced_by: replaced_by.clone(),
                ..base
            },
        }
    }
}

impl From<&Profile> for ExportProfile {
    fn from(p: &Profile) -> Self {
        Self {
            file: p.file.clone(),
            product: p.product.clone(),
            vendor: p.vendor.clone(),
            maintainer: match p.maintainer {
                Maintainer::Community => "community",
                Maintainer::Vendor => "vendor",
            },
            versions: p.versions.clone(),
            mappings: p
                .mappings
                .iter()
                .map(|m| {
                    let (coverage, needs) = match &m.coverage {
                        Coverage::Full => ("full", None),
                        Coverage::Partial(text) => ("partial", Some(text.clone())),
                    };
                    ExportMapping {
                        rule: m.rule.clone(),
                        category: m.category.clone(),
                        tool: m.tool.clone(),
                        coverage,
                        needs,
                        verify: m.verify.clone(),
                    }
                })
                .collect(),
        }
    }
}

impl Catalog {
    /// Returns the catalog as plain data for serialisation.
    #[must_use]
    pub fn export(&self) -> Export {
        Export {
            version: self.registry.version.clone(),
            categories: self
                .registry
                .categories
                .iter()
                .map(|c| ExportCategory {
                    id: c.id.clone(),
                    name: c.name.clone(),
                })
                .collect(),
            facts: self.facts.values().map(ExportFact::from).collect(),
            rules: self.rules.iter().map(ExportRule::from).collect(),
            profiles: self.profiles.iter().map(ExportProfile::from).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::catalog::load;
    use crate::testfixture;

    fn keys(value: &serde_json::Value) -> Vec<&str> {
        let mut keys: Vec<&str> = value
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        keys.sort_unstable();
        keys
    }

    #[test]
    fn export_fixture() {
        let catalog = load(&testfixture::standard()).unwrap();
        let json = serde_json::to_value(catalog.export()).unwrap();
        assert_eq!(json["version"], "0.1.0");
        assert_eq!(json["categories"][0].as_object().unwrap().len(), 2);
        let rules = json["rules"].as_array().unwrap();
        assert_eq!(rules.len(), 8);
        let by_id = |id: &str| rules.iter().find(|r| r["id"] == id).unwrap();

        let go = by_id("USS-GO-001");
        assert_eq!(go["appliesWhen"], "lang.go");
        assert_eq!(go["status"], "active");
        assert_eq!(go["level"], "MUST");
        assert_eq!(go["retiredIn"], serde_json::Value::Null);
        assert_eq!(go["refines"][0], "USS-SEC-001");
        assert_eq!(
            keys(go),
            [
                "appliesWhen",
                "body",
                "category",
                "enables",
                "file",
                "id",
                "level",
                "references",
                "refines",
                "replacedBy",
                "retiredIn",
                "status",
                "title",
                "verify"
            ]
        );

        let retired = by_id("USS-SEC-004");
        assert_eq!(retired["status"], "retired");
        assert_eq!(retired["level"], serde_json::Value::Null);
        assert_eq!(retired["appliesWhen"], serde_json::Value::Null);
        assert!(retired["retiredIn"].is_string());

        let ids: Vec<&str> = rules.iter().map(|r| r["id"].as_str().unwrap()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        assert_eq!(ids, sorted);

        let facts = json["facts"].as_array().unwrap();
        assert_eq!(facts.len(), 4);
        assert_eq!(
            keys(&facts[0]),
            ["definition", "enabledBy", "id", "implies", "question"]
        );
        let enabled = facts.iter().find(|f| f["enabledBy"].is_string()).unwrap();
        assert_eq!(enabled["question"], serde_json::Value::Null);
        let asked = facts.iter().find(|f| f["question"].is_string()).unwrap();
        assert_eq!(asked["enabledBy"], serde_json::Value::Null);

        let profiles = json["profiles"].as_array().unwrap();
        assert_eq!(profiles.len(), 1);
        let p = &profiles[0];
        assert_eq!(
            keys(p),
            [
                "file",
                "maintainer",
                "mappings",
                "product",
                "vendor",
                "versions"
            ]
        );
        assert_eq!(p["file"], "standard/vendors/acme/lint.md");
        assert_eq!(p["product"], "Lint");
        assert_eq!(p["vendor"], "Acme");
        assert_eq!(p["maintainer"], "community");
        assert_eq!(p["versions"], ">=1.0.0, <2.0.0");
        let maps = p["mappings"].as_array().unwrap();
        assert_eq!(maps.len(), 2);
        assert_eq!(
            keys(&maps[0]),
            ["category", "coverage", "needs", "rule", "tool", "verify"]
        );
        assert_eq!(maps[0]["rule"], "USS-SEC-001");
        assert_eq!(maps[0]["category"], "Safety");
        assert_eq!(maps[0]["coverage"], "full");
        assert_eq!(maps[0]["needs"], serde_json::Value::Null);
        assert_eq!(maps[1]["coverage"], "partial");
        assert_eq!(maps[1]["needs"], "a reviewer checks the plugin rights");
        assert_eq!(maps[1]["verify"], "Read the plugin settings.");
        assert_eq!(
            by_id("USS-LAW-001")["file"],
            "standard/modules/regulations/law.md"
        );
    }
}
