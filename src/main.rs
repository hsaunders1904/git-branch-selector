use dialoguer::{console::Term, theme::Theme, MultiSelect};

mod cli;
mod git;
mod theme;

fn main() {
    match select_and_print_branches(
        std::env::args(),
        std::io::stdout(),
        Term::stderr(),
        theme::GbsTheme::default(),
    ) {
        Ok(_) => (),
        Err(e) => {
            eprint!("{}", e);
            std::process::exit(1);
        }
    }
}

fn select_and_print_branches(
    cli_args: impl Iterator<Item = String>,
    writer: impl std::io::Write,
    terminal: Term,
    theme: impl Theme,
) -> Result<(), Error> {
    let args = cli::parse_args(cli_args);
    let branches = git::branch_list(&args.git_dir)?;
    let selected = match select_branches(&branches, &terminal, theme)? {
        Some(x) => x,
        None => return Ok(()),
    };
    write_branches(&selected, writer)?;
    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("git error: {0}")]
    Git(String),
    #[error("user input error: {0}")]
    Interactive(String),
    #[error("output error: {0}")]
    Write(String),
    #[error("")]
    Base,
}

fn select_branches(
    branches: &[String],
    terminal: &Term,
    theme: impl Theme,
) -> Result<Option<Vec<String>>, Error> {
    match MultiSelect::with_theme(&theme)
        .items(branches)
        .interact_on_opt(terminal)
    {
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

#[cfg(test)]
mod tests {
    mod write_branches {

        use crate::write_branches;

        #[test]
        fn delimits_branches_with_space() {
            let branches = vec![
                "a".to_string(),
                "branch".to_string(),
                "c/branch".to_string(),
            ];
            let mut writer = Vec::new();

            write_branches(&branches, &mut writer).unwrap();

            assert_eq!(writer, b"a branch c/branch");
        }
    }
}
