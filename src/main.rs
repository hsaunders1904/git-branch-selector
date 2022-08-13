use dialoguer::MultiSelect;

mod cli;
mod git;

fn main() -> Result<(), Error> {
    let args = cli::parse_args(std::env::args());
    let branches = git::branch_list(&args.git_dir);
    let selected = match select_branches(&branches)? {
        Some(x) => x,
        None => return Ok(()),
    };
    write_branches(&selected, std::io::stdout())?;
    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Git process failed: {0}")]
    Git(String),
    #[error("Error getting user input: {0}")]
    Interactive(String),
    #[error("Error writing to output stream: {0}")]
    Write(String),
}

fn select_branches(branches: &[String]) -> Result<Option<Vec<String>>, Error> {
    match MultiSelect::new().items(branches).interact_opt() {
        Ok(x) => match x {
            Some(choosen_idxs) => Ok(Some(
                choosen_idxs
                    .iter()
                    .map(|i| branches[*i].to_owned())
                    .collect::<Vec<_>>(),
            )),
            None => Ok(None),
        },
        Err(e) => Err(Error::Interactive(e.to_string())),
    }
}

fn write_branches(branches: &[String], mut writer: impl std::io::Write) -> Result<(), Error> {
    match write!(writer, "{}", branches.join(" ")) {
        Ok(_) => (),
        Err(e) => return Err(Error::Write(e.to_string())),
    }
    match writer.flush() {
        Ok(_) => (),
        Err(e) => return Err(Error::Write(e.to_string())),
    }
    Ok(())
}
