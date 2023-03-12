use crate::git::{Branch, BranchGetter, BranchType};
use crate::Error;

use std::path::PathBuf;

const GIT_DIR: &str = ".git";

pub struct FsBranchGetter {
    pub repo_dir: PathBuf,
}

impl BranchGetter for FsBranchGetter {
    fn branches(&self) -> Result<Vec<Branch>, Error> {
        let git_dir = discover_repo(&self.repo_dir)?;
        let refs_dir = git_dir.join("refs");
        let mut branches: Vec<Branch> = vec![];
        for local_ref in parse_refs(&refs_dir.join("heads"))? {
            branches.push(Branch {
                name: local_ref,
                branch_type: BranchType::Local,
            });
        }
        for remote_ref in parse_refs(&refs_dir.join("remotes"))? {
            branches.push(Branch {
                name: remote_ref,
                branch_type: BranchType::Remote,
            });
        }
        Ok(branches)
    }
}

fn parse_refs(dir: &PathBuf) -> Result<Vec<String>, Error> {
    if !dir.is_dir() {
        return Ok(vec![]);
    }
    parse_refs_inner(dir).map_err(|e| Error::Git(format!("could not parse refs: {e}")))
}

fn parse_refs_inner(dir: &PathBuf) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut refs: Vec<String> = vec![];
    for res_item in std::fs::read_dir(dir)? {
        let item = res_item?;
        if item.file_type()?.is_file() {
            // we skip symbolic refs (i.e., refs that point to other refs)
            if !std::fs::read_to_string(item.path())?.contains('/') {
                refs.push(item.file_name().to_string_lossy().to_string());
            }
        } else if item.file_type()?.is_dir() {
            let inner_refs = parse_refs(&item.path())?;
            refs.extend(
                inner_refs
                    .iter()
                    .map(|r| format!("{}/{r}", item.file_name().to_string_lossy())),
            );
        }
    }
    Ok(refs)
}

fn discover_repo(dir: &PathBuf) -> Result<PathBuf, Error> {
    let mut current_dir = std::fs::canonicalize(dir)
        .map_err(|_| Error::Git(format!("'{}' not a directory", dir.to_string_lossy())))?;
    while !current_dir.join(GIT_DIR).is_dir() {
        current_dir = match current_dir.parent() {
            Some(p) => p.to_path_buf(),
            None => {
                return Err(Error::Git(format!(
                    "not a git repository (or any of its parents): '{}'",
                    dir.to_string_lossy()
                )))
            }
        }
    }
    Ok(current_dir.join(GIT_DIR))
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use same_file::is_same_file;

    use super::*;

    fn make_test_git_dir() -> Result<tempfile::TempDir, Box<dyn std::error::Error>> {
        let temp_dir = tempfile::tempdir()?;
        let git_dir = temp_dir.path().join(GIT_DIR);

        // make local branches
        let heads_dir = git_dir.join("refs").join("heads");
        std::fs::create_dir_all(&heads_dir).unwrap();
        std::fs::File::create(heads_dir.join("main"))?
            .write_all("e2bf29060f42743538be07c164820cdeca0d9d2b".as_bytes())?;
        std::fs::File::create(heads_dir.join("other_branch"))?
            .write_all("a9c68440003151dd3cf7ffa4eaedd425d221d268".as_bytes())?;
        std::fs::create_dir(heads_dir.join("user")).unwrap();
        std::fs::File::create(heads_dir.join("user").join("some_dev_branch"))?
            .write_all("da7d6bf0955fa4d511067c00551fee04c613079d".as_bytes())?;

        // make remote branches in 'origin'
        let origin_dir = git_dir.join("refs").join("remotes").join("origin");
        std::fs::create_dir_all(&origin_dir)?;
        std::fs::File::create(origin_dir.join("main"))?
            .write_all("e2bf29060f42743538be07c164820cdeca0d9d2b".as_bytes())?;
        std::fs::File::create(origin_dir.join("remote_branch"))?
            .write_all("e2bf29060f42743538be07c164820cdeca0d9d2b".as_bytes())?;
        std::fs::File::create(origin_dir.join("HEAD"))?
            .write_all("refs/remotes/origin/main".as_bytes())?;

        // make remote branches in 'upstream'
        let upstream_dir = git_dir.join("refs").join("remotes").join("upstream");
        std::fs::create_dir_all(&upstream_dir)?;
        std::fs::File::create(upstream_dir.join("main"))?
            .write_all("707a178071655bed661318a5344557fe3e9a6ce1".as_bytes())?;
        Ok(temp_dir)
    }

    fn expected_branches() -> Vec<Branch> {
        // sort these alphabetically for easier comparison in tests
        vec![
            Branch {
                name: "main".to_string(),
                branch_type: BranchType::Local,
            },
            Branch {
                name: "origin/main".to_string(),
                branch_type: BranchType::Remote,
            },
            Branch {
                name: "origin/remote_branch".to_string(),
                branch_type: BranchType::Remote,
            },
            Branch {
                name: "other_branch".to_string(),
                branch_type: BranchType::Local,
            },
            Branch {
                name: "upstream/main".to_string(),
                branch_type: BranchType::Remote,
            },
            Branch {
                name: "user/some_dev_branch".to_string(),
                branch_type: BranchType::Local,
            },
        ]
    }

    #[test]
    fn fs_branch_getter_retrieves_all_branches() {
        let temp_dir = make_test_git_dir().unwrap();

        let getter = FsBranchGetter {
            repo_dir: temp_dir.path().to_path_buf(),
        };
        let mut branches = getter.branches().unwrap();

        branches.sort_by(|a, b| a.name.cmp(&b.name));
        assert_eq!(branches, expected_branches());
    }

    #[test]
    fn fs_branch_getter_returns_empty_vec_given_no_branches() {
        let temp_dir = tempfile::tempdir().unwrap();
        let git_dir = temp_dir.path().join(GIT_DIR);
        std::fs::create_dir_all(git_dir).unwrap();

        let getter = FsBranchGetter {
            repo_dir: temp_dir.path().to_path_buf(),
        };
        let branches = getter.branches().unwrap();

        assert!(branches.is_empty());
    }

    #[test]
    fn discover_repo_returns_git_dir_if_in_repo_root() {
        let temp_dir = make_test_git_dir().unwrap();
        let git_path = temp_dir.path().to_path_buf();

        let git_dir = discover_repo(&git_path).unwrap();

        assert!(is_same_file(git_dir, temp_dir.path().join(".git")).unwrap());
    }

    #[test]
    fn discover_repo_returns_git_dir_if_path_not_in_repo_root() {
        let temp_dir = make_test_git_dir().unwrap();
        let src_dir = tempfile::tempdir_in(temp_dir.path()).unwrap();

        let git_dir = discover_repo(&PathBuf::from(src_dir.path())).unwrap();

        assert!(is_same_file(git_dir, temp_dir.path().join(".git")).unwrap());
    }

    #[test]
    fn discover_repo_returns_err_given_not_in_git_dir() {
        let not_git_dir = tempfile::tempdir().unwrap();

        let git_dir = discover_repo(&PathBuf::from(not_git_dir.path()));

        assert!(git_dir.is_err());
        assert!(git_dir
            .unwrap_err()
            .to_string()
            .contains("not a git repository"));
    }

    #[test]
    fn discover_repo_returns_err_given_path_does_not_exist() {
        let git_dir = discover_repo(&PathBuf::from("/not/a/dir"));

        assert!(git_dir.is_err());
        assert!(git_dir.unwrap_err().to_string().contains("not a directory"));
    }
}
