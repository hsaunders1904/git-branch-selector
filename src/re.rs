use crate::git;
use crate::Error;

pub fn compile_filters(filters: &[String]) -> Result<Vec<regex::Regex>, Error> {
    let mut patterns = vec![];
    for f in filters {
        let pattern = string_to_regex(f)?;
        patterns.push(pattern);
    }
    Ok(patterns)
}

fn string_to_regex(s: &str) -> Result<regex::Regex, Error> {
    regex::Regex::new(s).map_err(|e| Error::Regex(format!("{}", e)))
}

pub fn matches_regex(branch: &git::Branch, patterns: &[regex::Regex]) -> bool {
    if patterns.is_empty() {
        return true;
    }
    for pattern in patterns {
        if pattern.is_match(&format!("{}", branch)) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    mod matches_regex {
        use super::super::*;

        #[test]
        fn returns_true_given_no_patterns() {
            let branch = git::Branch {
                name: "x".to_string(),
                branch_type: git::BranchType::Local,
            };

            assert!(matches_regex(&branch, &[]));
        }

        #[test]
        fn returns_true_given_one_pattern_matches() {
            let branch = git::Branch {
                name: "a_name".to_string(),
                branch_type: git::BranchType::Local,
            };
            let patterns = ["x", "[0-9]+", "a_"]
                .into_iter()
                .map(|p| regex::Regex::new(p).unwrap())
                .collect::<Vec<_>>();

            assert!(matches_regex(&branch, &patterns));
        }

        #[test]
        fn returns_true_given_remotes_prefix_pattern_for_remote_branch() {
            let branch = git::Branch {
                name: "a_name".to_string(),
                branch_type: git::BranchType::Remote,
            };
            let patterns = ["remotes/.+"]
                .into_iter()
                .map(|p| regex::Regex::new(p).unwrap())
                .collect::<Vec<_>>();

            assert!(matches_regex(&branch, &patterns));
        }

        #[test]
        fn returns_false_if_no_patterns_matched() {
            let branch = git::Branch {
                name: "a_branch".to_string(),
                branch_type: git::BranchType::Local,
            };
            let patterns = ["x", "[0-9]+"]
                .into_iter()
                .map(|p| regex::Regex::new(p).unwrap())
                .collect::<Vec<_>>();

            assert!(!matches_regex(&branch, &patterns));
        }
    }

    mod string_to_regex {
        use super::super::string_to_regex;

        #[test]
        fn returns_err_given_invalid_regex_pattern() {
            let result = string_to_regex("(abc");

            assert!(result.is_err());
        }
    }
}
