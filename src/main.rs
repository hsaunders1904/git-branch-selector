use dialoguer::MultiSelect;

mod cli;
mod config;
mod git;
mod theme;

#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub enum Error {
    #[error("config error: {0}")]
    Config(String),
    #[error("git error: {0}")]
    Git(String),
    #[error("input error: {0}")]
    Input(String),
    #[error("output error: {0}")]
    Write(String),
}

fn main() {
    let conf = config::init_config().unwrap_or_else(|e| {
        eprint!("{}", e);
        std::process::exit(1)
    });
    let args = cli::parse_args(std::env::args());
    let branch_outputter = git::GitBranchOutputter {
        working_dir: args.git_dir,
    };
    let branch_selector = InteractiveBranchSelector {
        theme: conf.theme(),
    };
    match select_and_print_branches(branch_outputter, std::io::stdout(), branch_selector) {
        Ok(_) => (),
        Err(e) => {
            eprint!("{}", e);
            std::process::exit(1);
        }
    }
}

fn select_and_print_branches(
    branch_outputter: impl git::Outputter,
    writer: impl std::io::Write,
    branch_selector: impl Selector,
) -> Result<(), Error> {
    let branches = git::branch_list(branch_outputter)?;
    let selected = branch_selector.select(&branches)?;
    write_branches(&selected, writer)?;
    Ok(())
}

pub trait Selector {
    fn select(&self, options: &[String]) -> Result<Vec<String>, Error>;
}

struct InteractiveBranchSelector {
    theme: theme::GbsTheme,
}

impl Selector for InteractiveBranchSelector {
    fn select(&self, options: &[String]) -> Result<Vec<String>, Error> {
        match MultiSelect::with_theme(&self.theme)
            .items(options)
            .interact_opt()
        {
            Ok(x) => match x {
                Some(choosen_idxs) => Ok(choosen_idxs
                    .iter()
                    .map(|i| options[*i].to_owned())
                    .collect::<Vec<_>>()),
                None => Ok(vec![]),
            },
            Err(e) => Err(Error::Input(e.to_string())),
        }
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
    mod select_and_print_branches {
        use super::super::*;
        use crate::git::Outputter;

        struct FakeOutputter {
            success: bool,
            stdout: String,
            stderr: String,
        }
        impl Outputter for FakeOutputter {
            fn get_output(&self) -> Result<(bool, Vec<u8>, Vec<u8>), Error> {
                Ok((
                    self.success,
                    self.stdout.as_bytes().to_vec(),
                    self.stderr.as_bytes().to_vec(),
                ))
            }
        }

        struct FakeSelector {
            selection: Vec<String>,
        }
        impl Selector for FakeSelector {
            fn select(&self, _: &[String]) -> Result<Vec<String>, Error> {
                Ok(self.selection.clone())
            }
        }

        #[test]
        fn prints_selected_branches() {
            let outputter = FakeOutputter {
                success: true,
                stdout: "main\n*develop\nfeature/123\n".to_string(),
                stderr: "".to_string(),
            };
            let selector = FakeSelector {
                selection: vec!["main".to_string(), "feature/123".to_string()],
            };
            let mut writer = Vec::new();

            let result = select_and_print_branches(outputter, &mut writer, selector);

            assert!(result.is_ok());
            assert_eq!(writer, b"main feature/123");
        }
    }
}
