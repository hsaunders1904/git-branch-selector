use crate::Error;
use std::process::Command;

pub fn branch_list(working_dir: &str) -> Vec<String> {
    parse_branches(&call_branch_list(working_dir))
}

pub fn delete_branch(branch: &str, working_dir: &str) -> Result<(), Error> {
    match Command::new("git")
        .arg("branch")
        .arg("-d")
        .arg(branch)
        .current_dir(working_dir)
        .spawn()
    {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::Git(e.to_string())),
    }
}

fn call_branch_list(working_dir: &str) -> String {
    let output = Command::new("git")
        .arg("branch")
        .arg("--list")
        .current_dir(working_dir)
        .output()
        .expect("'git' command failed.")
        .stdout;
    String::from_utf8(output).expect("Could not decode git branch output.")
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
