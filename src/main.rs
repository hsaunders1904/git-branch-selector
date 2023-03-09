mod cli;
mod config;
mod git;
mod re;
mod select;

#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub enum Error {
    #[error("bselect: {0}")]
    Cli(String),
    #[error("bselect: {0}")]
    Config(String),
    #[error("bselect: {0}")]
    Git(String),
    #[error("bselect: {0}")]
    Regex(String),
    #[error("bselect: {0}")]
    Select(String),
    #[error("bselect: {0}")]
    Terminal(String),
}

fn main() {
    let args = parse_args(std::env::args());
    let config = read_config();
    let getter = git::gitoxide::GixBranchGetter {
        repo_dir: args.git_dir.clone(),
    };
    let selector = select::DialogueSelector {
        theme: config.theme(),
    };
    bselect(&args, getter, selector, &mut std::io::stdout()).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    })
}

fn bselect(
    args: &cli::Args,
    branch_getter: impl git::BranchGetter,
    selector: impl select::BranchSelector,
    stdout: &mut dyn std::io::Write,
) -> Result<(), Error> {
    if args.config {
        writeln!(stdout, "{}", config::file::config_path()?.to_string_lossy())
            .map_err(|e| Error::Terminal(format!("cannot write config path: {e}")))?;
        return Ok(());
    }
    let branches = filter_branches(branch_getter.branches()?, args.all, &args.filters)?;
    let selected_branches = selector.select_branches(branches)?;
    let branch_names = selected_branches
        .iter()
        .map(|b| b.to_string())
        .collect::<Vec<_>>();
    writeln!(stdout, "{}", branch_names.join(" "))
        .map_err(|e| Error::Terminal(format!("cannot write to stdout: {e}")))
}

fn parse_args(argv: impl Iterator<Item = String>) -> cli::Args {
    cli::parse_args(argv).unwrap_or_else(|_| std::process::exit(1))
}

fn read_config() -> config::Config {
    let file_path = match config::file::config_path() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{e}");
            return config::Config::default();
        }
    };
    match config::file::init_config(&file_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            config::Config::default()
        }
    }
}

fn filter_branches(
    branches: Vec<git::Branch>,
    include_remotes: bool,
    patterns: &[String],
) -> Result<Vec<git::Branch>, Error> {
    let re_patterns = re::compile_filters(patterns)?;
    let out: Vec<git::Branch> = branches
        .into_iter()
        .filter(|b| include_remotes || b.branch_type == git::BranchType::Local)
        .filter(|b| re::matches_regex(b, &re_patterns))
        .collect();
    if out.is_empty() {
        return Err(Error::Select("no matching branches".to_string()));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! to_string_iter {
        ($element: expr) => {{
            $element.iter().map(|s| s.to_string())
        }};
    }

    fn make_branches() -> Vec<git::Branch> {
        vec![
            git::Branch {
                name: "feature/xyz".to_string(),
                branch_type: git::BranchType::Local,
            },
            git::Branch {
                name: "123-add_a_new_feature".to_string(),
                branch_type: git::BranchType::Local,
            },
            git::Branch {
                name: "ABC".to_string(),
                branch_type: git::BranchType::Remote,
            },
            git::Branch {
                name: "456-fix_a_bug".to_string(),
                branch_type: git::BranchType::Local,
            },
        ]
    }

    struct SimpleGetter {
        branches: Vec<git::Branch>,
    }
    impl git::BranchGetter for SimpleGetter {
        fn branches(&self) -> Result<Vec<git::Branch>, Error> {
            Ok(self.branches.clone())
        }
    }

    struct SimpleSelector {
        idxs: Vec<usize>,
    }
    impl select::BranchSelector for SimpleSelector {
        fn select_branches(&self, branches: Vec<git::Branch>) -> Result<Vec<git::Branch>, Error> {
            let out = self.idxs.iter().map(|i| branches[*i].clone());
            Ok(out.collect())
        }
    }

    #[test]
    fn bselect_writes_expected_branches_with_no_args() {
        let args = parse_args(to_string_iter!(["bselect"]));
        let branches = make_branches();
        let branch_getter = SimpleGetter { branches };
        let selector = SimpleSelector { idxs: vec![0, 2] };
        let mut stdout = Vec::new();

        let result = bselect(&args, branch_getter, selector, &mut stdout);

        assert!(result.is_ok());
        assert_eq!(
            String::from_utf8(stdout).unwrap(),
            "feature/xyz 456-fix_a_bug\n"
        );
    }

    #[test]
    fn bselect_writes_expected_branches_with_all_arg() {
        let args = parse_args(to_string_iter!(["bselect", "--all"]));
        let branches = make_branches();
        let branch_getter = SimpleGetter { branches };
        let selector = SimpleSelector {
            idxs: vec![0, 2, 3],
        };
        let mut stdout = Vec::new();

        let result = bselect(&args, branch_getter, selector, &mut stdout);

        assert!(result.is_ok());
        assert_eq!(
            String::from_utf8(stdout).unwrap(),
            "feature/xyz remotes/ABC 456-fix_a_bug\n"
        );
    }

    #[test]
    fn bselect_writes_expected_branches_with_filter() {
        let args = parse_args(to_string_iter!(["bselect", "feature/"]));
        let mut branches = make_branches();
        branches.push(git::Branch {
            name: "feature/no_2".to_string(),
            branch_type: git::BranchType::Local,
        });
        let branch_getter = SimpleGetter { branches };
        let selector = SimpleSelector { idxs: vec![0, 1] };
        let mut stdout = Vec::new();

        let result = bselect(&args, branch_getter, selector, &mut stdout);

        assert!(result.is_ok());
        assert_eq!(
            String::from_utf8(stdout).unwrap(),
            "feature/xyz feature/no_2\n"
        );
    }

    #[test]
    fn bselect_writes_expected_branches_with_multiple_filters() {
        let args = parse_args(to_string_iter!(["bselect", "feature/", "^[0-9]+.*$"]));
        let mut branches = make_branches();
        branches.insert(
            1,
            git::Branch {
                name: "some_other_branch-123".to_string(),
                branch_type: git::BranchType::Local,
            },
        );
        branches.push(git::Branch {
            name: "feature/no_2".to_string(),
            branch_type: git::BranchType::Local,
        });
        let branch_getter = SimpleGetter { branches };
        let selector = SimpleSelector {
            idxs: vec![0, 1, 2, 3],
        };
        let mut stdout = Vec::new();

        let result = bselect(&args, branch_getter, selector, &mut stdout);

        assert!(result.is_ok());
        assert_eq!(
            String::from_utf8(stdout).unwrap(),
            "feature/xyz 123-add_a_new_feature 456-fix_a_bug feature/no_2\n"
        );
    }

    #[test]
    fn bselect_writes_expected_branches_with_multiple_filters_and_all() {
        let args = parse_args(to_string_iter!([
            "bselect",
            "feature/",
            "^[0-9]+.*$",
            "--all"
        ]));
        let mut branches = make_branches();
        branches.insert(
            1,
            git::Branch {
                name: "some_other_branch-123".to_string(),
                branch_type: git::BranchType::Local,
            },
        );
        branches.push(git::Branch {
            name: "feature/no_2".to_string(),
            branch_type: git::BranchType::Local,
        });
        branches.push(git::Branch {
            name: "feature/remote_feature".to_string(),
            branch_type: git::BranchType::Remote,
        });
        let branch_getter = SimpleGetter { branches };
        let selector = SimpleSelector {
            idxs: vec![1, 2, 4],
        };
        let mut stdout = Vec::new();

        let result = bselect(&args, branch_getter, selector, &mut stdout);

        assert!(result.is_ok());
        assert_eq!(
            String::from_utf8(stdout).unwrap(),
            "123-add_a_new_feature 456-fix_a_bug remotes/feature/remote_feature\n"
        );
    }

    #[test]
    fn bselect_returns_err_given_no_branches_match_pattern() {
        let args = parse_args(to_string_iter!(["bselect", "no_match"]));
        let branches = make_branches();
        let branch_getter = SimpleGetter { branches };
        let selector = SimpleSelector { idxs: vec![0, 2] };
        let mut stdout = Vec::new();

        let result = bselect(&args, branch_getter, selector, &mut stdout);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("no matching branches"));
    }

    #[test]
    fn bselect_prints_config_path_given_config_arg_true() {
        let args = parse_args(to_string_iter!(["bselect", "--config"]));
        let branches = make_branches();
        let branch_getter = SimpleGetter { branches };
        let selector = SimpleSelector { idxs: vec![0, 2] };
        let mut stdout = Vec::new();

        bselect(&args, branch_getter, selector, &mut stdout).unwrap();

        assert!(String::from_utf8(stdout)
            .unwrap()
            .ends_with("config.json\n"));
    }
}
