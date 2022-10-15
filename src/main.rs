use dialoguer::MultiSelect;

mod cli;
mod config;
mod git;
mod theme;

#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub enum Error {
    #[error("bselect: config: {0}")]
    Config(String),
    #[error("bselect: git: {0}")]
    Git(String),
    #[error("bselect: input: {0}")]
    Input(String),
    #[error("bselect: write: {0}")]
    Write(String),
}

fn main() {
    let args = cli::parse_args(std::env::args());
    let conf = config::init_config().unwrap_or_else(|e| {
        eprintln!("{}", e);
        config::Config::default()
    });
    if args.config {
        match print_config_path() {
            Ok(_) => std::process::exit(0),
            Err(e) => {
                eprint!("{}", e);
                std::process::exit(1);
            }
        }
    }
    let branch_outputter = git::GitBranchOutputter {
        working_dir: args.git_dir,
        filters: args.filters,
        all: args.all,
    };
    let branch_selector = InteractiveBranchSelector {
        theme: conf.theme(),
    };
    match select_and_print_branches(branch_outputter, std::io::stdout(), branch_selector) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

fn select_and_print_branches(
    branch_outputter: impl git::Outputter,
    writer: impl std::io::Write,
    branch_selector: impl Selector,
) -> Result<(), Error> {
    let all_branches = git::branch_list(branch_outputter)?;
    if all_branches.is_empty() {
        return Err(Error::Git("no matching branches to list".to_string()));
    }
    let selected_branches = select_branches(&all_branches, branch_selector)?;
    write_branches(&selected_branches, writer)?;
    Ok(())
}

fn select_branches(
    branches: &[String],
    branch_selector: impl Selector,
) -> Result<Vec<String>, Error> {
    let selected_idxs = branch_selector.select(branches)?;
    let selected_branches = selected_idxs
        .iter()
        .map(|i| branches[*i].to_owned())
        .collect::<Vec<_>>();
    Ok(selected_branches)
}

fn print_config_path() -> Result<(), Error> {
    match config::config_path() {
        Some(x) => {
            println!("{}", x.to_string_lossy());
            Ok(())
        }
        None => Err(Error::Config("could not build config path.".to_string())),
    }
}

trait Selector {
    fn select(&self, options: &[String]) -> Result<Vec<usize>, Error>;
}

struct InteractiveBranchSelector {
    theme: theme::GbsTheme,
}

impl Selector for InteractiveBranchSelector {
    fn select(&self, options: &[String]) -> Result<Vec<usize>, Error> {
        match MultiSelect::with_theme(&self.theme)
            .items(options)
            .interact_opt()
        {
            Ok(idxs) => match idxs {
                Some(x) => Ok(x),
                None => Ok(vec![]),
            },
            Err(e) => Err(Error::Input(e.to_string())),
        }
    }
}

fn write_branches(branches: &[String], mut writer: impl std::io::Write) -> Result<(), Error> {
    match writeln!(writer, "{}", branches.join(" ")) {
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
            selection: Vec<usize>,
        }
        impl Selector for FakeSelector {
            fn select(&self, _: &[String]) -> Result<Vec<usize>, Error> {
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
                selection: vec![0, 2],
            };
            let mut writer = Vec::new();

            let result = select_and_print_branches(outputter, &mut writer, selector);

            assert!(result.is_ok());
            assert_eq!(writer, b"main feature/123\n");
        }

        #[test]
        fn error_given_no_matching_branches() {
            let outputter = FakeOutputter {
                success: true,
                stdout: "\n".to_string(),
                stderr: "".to_string(),
            };
            let selector = FakeSelector { selection: vec![] };
            let mut writer = Vec::new();

            let result = select_and_print_branches(outputter, &mut writer, selector);

            assert!(result.is_err());
            assert!(result
                .unwrap_err()
                .to_string()
                .contains("no matching branches to list"));
        }
    }
}
