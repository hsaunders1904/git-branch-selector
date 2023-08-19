use crate::git::Branch;
use crate::Error;

pub trait BranchGetter {
    fn branches(&self) -> Result<Vec<Branch>, Error>;
}
