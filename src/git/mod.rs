mod branch_getter;
mod gitoxide;

use std::fmt::Display;

pub use crate::git::branch_getter::BranchGetter;
pub use crate::git::gitoxide::GixBranchGetter;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum BranchType {
    Local,
    Remote,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Branch {
    pub name: String,
    pub branch_type: BranchType,
}

impl Display for Branch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.branch_type == BranchType::Remote {
            return write!(f, "remotes/{}", self.name);
        }
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod tests {

    mod branch {
        use super::super::{Branch, BranchType};

        #[test]
        fn to_string_prepends_remotes_if_remote_branch() {
            let branch = Branch {
                name: "some_name".to_string(),
                branch_type: BranchType::Remote,
            };

            let branch_str = branch.to_string();

            assert_eq!(branch_str, "remotes/some_name")
        }

        #[test]
        fn to_string_returns_name_if_local_branch() {
            let branch = Branch {
                name: "some_name".to_string(),
                branch_type: BranchType::Local,
            };

            let branch_str = branch.to_string();

            assert_eq!(branch_str, "some_name")
        }
    }
}
