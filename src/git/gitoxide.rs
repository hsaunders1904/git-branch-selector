use std::path::PathBuf;

use crate::git::{Branch, BranchGetter, BranchType};
use crate::Error;

pub struct GixBranchGetter {
    pub repo_dir: PathBuf,
}

impl BranchGetter for GixBranchGetter {
    fn branches(&self) -> Result<Vec<Branch>, Error> {
        let repo = gix::discover(&self.repo_dir)
            .map_err(|e| Error::Git(format!("could not read repository: {e}")))?;

        let refs = repo
            .references()
            .map_err(|e| Error::Git(format!("could not parse refs: {e}")))?;

        let locals = refs
            .local_branches()
            .map_err(|e| Error::Git(format!("{e}")))?
            .map(|r| Branch {
                name: r.unwrap().name().file_name().to_string(),
                branch_type: BranchType::Local,
            });
        let remotes = refs
            .remote_branches()
            .map_err(|e| Error::Git(format!("{e}")))?
            .map(|r| Branch {
                name: r.unwrap().name().file_name().to_string(),
                branch_type: BranchType::Remote,
            });
        Ok(locals.chain(remotes).collect())
    }

    // fn ref_to_branch(reference: gix::Reference) -> Option<Branch> {
    //     Some(Branch {name: reference.try_id()?.to_string(),
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn branch_getter_finds_main_branch() {
        // The main branch is the only branch we should always have, hence we
        // use it here
        let getter = GixBranchGetter {
            repo_dir: ".".into(),
        };

        let branches = getter.branches().unwrap();
        println!("{:?}", branches);

        let main_branch = Branch {
            name: "main".to_string(),
            branch_type: BranchType::Local,
        };
        assert!(branches.contains(&main_branch));
    }

    #[test]
    fn error_given_repo_dir_does_not_exist() {
        let getter = GixBranchGetter {
            repo_dir: "/not/a/dir".into(),
        };

        let branches = getter.branches();

        assert!(branches.is_err());
        let err_msg = format!("{}", branches.err().unwrap());
        assert!(err_msg.contains("could not read repository"))
    }
}
