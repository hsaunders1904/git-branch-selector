use crate::git::*;
use crate::Error;

use ::git2;

use std::path::PathBuf;

pub struct Git2BranchGetter {
    pub repo_dir: PathBuf,
}

impl BranchGetter for Git2BranchGetter {
    fn branches(&self) -> Result<Vec<Branch>, Error> {
        let repo = self.open_repo()?;
        let references = repo
            .references()
            .map_err(|e| Error::Git(format!("could not parse refs: {}", e)))?;
        let branches = Git2BranchGetter::refs_to_branches(references)?;
        Ok(branches)
    }
}

impl Git2BranchGetter {
    fn open_repo(&self) -> Result<git2::Repository, Error> {
        git2::Repository::open(&self.repo_dir)
            .map_err(|e| Error::Git(format!("could not read repository: {}", e)))
    }

    fn make_branch(reference: &git2::Reference) -> Option<Branch> {
        if reference.kind()? == git2::ReferenceType::Symbolic {
            return None;
        }
        let branch_type = if reference.is_branch() {
            BranchType::Local
        } else if reference.is_remote() {
            BranchType::Remote
        } else {
            return None;
        };
        let name = reference.shorthand()?.to_string();
        Some(Branch { name, branch_type })
    }

    fn refs_to_branches(references: git2::References) -> Result<Vec<Branch>, Error> {
        let mut branches = vec![];
        for ref_result in references {
            let reference =
                ref_result.map_err(|e| Error::Git(format!("could not parse ref: {}", e)))?;
            if let Some(branch) = Git2BranchGetter::make_branch(&reference) {
                branches.push(branch);
            }
        }
        Ok(branches)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn branch_getter_finds_main_branch() {
        // The main branch is the only branch we should always have, hence we
        // use it here
        let getter = Git2BranchGetter {
            repo_dir: ".".into(),
        };

        let branches = getter.branches().unwrap();

        let main_branch = Branch {
            name: "main".to_string(),
            branch_type: BranchType::Local,
        };
        assert!(branches.contains(&main_branch));
    }

    #[test]
    fn error_given_repo_dir_does_not_exist() {
        let getter = Git2BranchGetter {
            repo_dir: "/not/a/dir".into(),
        };

        let branches = getter.branches();

        assert!(branches.is_err());
        let err_msg = format!("{}", branches.err().unwrap());
        assert!(err_msg.contains("could not read repository"))
    }
}
