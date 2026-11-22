//! Release checks that need no I/O: which changed paths break an audit's
//! claim to describe the release commit.

/// Returns the paths in `paths` that are not under a `.uss` directory.
///
/// A path is allowed when any `/`-separated segment equals `.uss`, so
/// `svc/.uss/record.yaml` of a nested component is allowed but `.ussx/a` is not.
#[must_use]
pub fn only_uss_changes(paths: &[String]) -> Vec<String> {
    paths
        .iter()
        .filter(|path| !path.split('/').any(|segment| segment == ".uss"))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_uss_changes_table() {
        let paths: Vec<String> = [
            ".uss/audits/a.yaml",
            "svc/.uss/record.yaml",
            "src/main.rs",
            ".ussx/a",
        ]
        .map(String::from)
        .to_vec();
        assert_eq!(
            only_uss_changes(&paths),
            vec!["src/main.rs".to_string(), ".ussx/a".to_string()]
        );
        assert_eq!(only_uss_changes(&[]), Vec::<String>::new());
    }
}
