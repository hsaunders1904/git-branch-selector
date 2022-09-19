use crate::Error;
use std::process::Command;

pub fn branch_list(outputter: impl Outputter) -> Result<Vec<String>, Error> {
    let (success, stdout, stderr) = outputter.get_output()?;
    match check_output(success, stdout, stderr) {
        Ok(x) => Ok(parse_branches(&x)),
        Err(e) => Err(e),
    }
}

pub trait Outputter {
    fn get_output(&self) -> Result<(bool, Vec<u8>, Vec<u8>), Error>;
}

pub struct GitBranchOutputter {
    pub working_dir: String,
    pub filter: String,
}

impl Outputter for GitBranchOutputter {
    fn get_output(&self) -> Result<(bool, Vec<u8>, Vec<u8>), Error> {
        let mut command = Command::new("git");
        command.arg("branch").arg("--list");
        if !self.filter.is_empty() {
            command.arg(&self.filter);
        }
        match command.current_dir(&self.working_dir).output() {
            Ok(x) => Ok((x.status.success(), x.stdout, x.stderr)),
            Err(e) => Err(Error::Git(e.to_string())),
        }
    }
}

fn check_output(success: bool, stdout: Vec<u8>, stderr: Vec<u8>) -> Result<String, Error> {
    if !success {
        return Err(Error::Git(format!(
            "Error getting git branches: {}",
            match String::from_utf8(stderr) {
                Ok(x) => x,
                Err(e) =>
                    return Err(Error::Git(format!(
                        "Could not decode git branch output: {}",
                        e
                    ))),
            }
        )));
    }
    match String::from_utf8(stdout) {
        Ok(x) => Ok(x),
        Err(e) => Err(Error::Git(format!(
            "Could not decode git branch output: {}",
            e
        ))),
    }
}

fn clean_branch(branch: &str) -> &str {
    branch.trim().strip_prefix('*').unwrap_or(branch).trim()
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
    mod branch_list {
        use super::super::*;

        struct FakeOutputter {
            success: bool,
            stdout: Vec<u8>,
            stderr: Vec<u8>,
        }

        impl Outputter for FakeOutputter {
            fn get_output(&self) -> Result<(bool, Vec<u8>, Vec<u8>), Error> {
                Ok((self.success, self.stdout.clone(), self.stderr.clone()))
            }
        }

        #[test]
        fn parses_list_of_branches() {
            let outputter = FakeOutputter {
                success: true,
                stdout: " main\n* develop  \n \n   other/branch\n"
                    .as_bytes()
                    .to_vec(),
                stderr: "".as_bytes().to_vec(),
            };

            let mut branches = branch_list(outputter).unwrap();

            branches.sort();
            assert_eq!(branches, vec!["develop", "main", "other/branch"]);
        }

        #[test]
        fn error_if_outputter_does_not_succeed() {
            let outputter = FakeOutputter {
                success: false,
                stdout: "".as_bytes().to_vec(),
                stderr: "error message".as_bytes().to_vec(),
            };

            let branches = branch_list(outputter);

            assert!(branches.is_err());
            assert!(branches.unwrap_err().to_string().contains("error message"));
        }

        #[test]
        fn error_if_output_not_valid_utf8() {
            let outputter = FakeOutputter {
                success: true,
                stdout: vec![240, 40, 140, 188], // \xf0\x28\x8c\xbc
                stderr: "error message".as_bytes().to_vec(),
            };

            let branches = branch_list(outputter);

            assert!(branches.is_err());
            let err = branches.unwrap_err();
            assert!(
                err.to_string()
                    .contains("Could not decode git branch output"),
                "{}",
                err
            );
        }

        #[test]
        fn error_if_output_fails_with_not_valid_utf8() {
            let outputter = FakeOutputter {
                success: false,
                stdout: "".as_bytes().to_vec(),
                stderr: vec![240, 40, 140, 188], // \xf0\x28\x8c\xbc
            };

            let branches = branch_list(outputter);

            assert!(branches.is_err());
            let err = branches.unwrap_err();
            assert!(
                err.to_string()
                    .contains("Could not decode git branch output"),
                "{}",
                err
            );
        }
    }
}
