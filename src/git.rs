use crate::Error;
use std::process::Command;

pub fn branch_list(working_dir: &str) -> Result<Vec<String>, Error> {
    match call_branch_list(working_dir) {
        Ok(x) => Ok(parse_branches(&x)),
        Err(e) => Err(e),
    }
}

fn call_branch_list(working_dir: &str) -> Result<String, Error> {
    let output = match Command::new("git")
        .arg("branch")
        .arg("--list")
        .current_dir(working_dir)
        .output()
    {
        Ok(x) => x,
        Err(e) => return Err(Error::Git(e.to_string())),
    };
    if !output.status.success() {
        return Err(Error::Git(format!(
            "Error getting git branches: {}",
            String::from_utf8(output.stderr).unwrap()
        )));
    }
    match String::from_utf8(output.stdout) {
        Ok(x) => Ok(x),
        Err(e) => Err(Error::Git(format!(
            "Could not decode git branch output: {}",
            e
        ))),
    }
}

fn clean_branch(branch: &str) -> &str {
    match branch.trim().strip_prefix('*') {
        Some(x) => x,
        None => branch,
    }
    .trim()
}

fn parse_branches(branch_list: &str) -> Vec<String> {
    branch_list
        .split('\n')
        .filter(|x| !x.trim().is_empty())
        .map(clean_branch)
        .map(|x| x.to_owned())
        .collect()
}

#[cfg(test)]
mod tests {
    mod parse_branches {
        use super::super::*;

        #[test]
        fn returns_all_listed() {
            let branches_str = "  hsaunders1904/branch1
        * main
          branch3
          _some-branch";

            let mut branches = parse_branches(branches_str);

            // Sort as we don't care about order
            branches.sort();
            let expected = vec!["_some-branch", "branch3", "hsaunders1904/branch1", "main"];
            assert_eq!(branches, expected);
        }

        #[test]
        fn ignores_empty_lines() {
            let branches_str = "
        hsaunders1904/branch1

        * main

          branch3


          _some-branch
          ";

            let mut branches = parse_branches(branches_str);

            // Sort as we don't care about order
            branches.sort();
            let expected = vec!["_some-branch", "branch3", "hsaunders1904/branch1", "main"];
            assert_eq!(branches, expected);
        }
    }

    mod clean_branch {
        use super::super::*;

        #[test]
        fn removes_whitespace_from_beginning_and_end() {
            assert_eq!(clean_branch("  some branch   "), "some branch");
        }

        #[test]
        fn removes_asterisk_from_beginning() {
            assert_eq!(clean_branch("* some branch*"), "some branch*");
        }

        #[test]
        fn removes_whitespace_and_asterisk_from_beginning() {
            assert_eq!(clean_branch(" * some branch"), "some branch");
        }
    }
}
