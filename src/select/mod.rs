pub mod theme;

use dialoguer as dlg;

use crate::git;
use crate::Error;

pub trait BranchSelector {
    fn select_branches(&self, branches: Vec<git::Branch>) -> Result<Vec<git::Branch>, Error>;
}

pub struct DialogueSelector {
    pub theme: theme::ConsoleTheme,
}

impl BranchSelector for DialogueSelector {
    fn select_branches(&self, branches: Vec<git::Branch>) -> Result<Vec<git::Branch>, Error> {
        let idxs = match dlg::MultiSelect::with_theme(&self.theme)
            .items(&branches)
            .interact_opt()
        {
            Ok(opt) => match opt {
                Some(x) => x,
                None => return Ok(vec![]),
            },
            Err(e) => return Err(Error::Select(format!("{e}"))),
        };
        let selected = idxs.iter().map(|i| branches[*i].to_owned());
        Ok(selected.collect())
    }
}
