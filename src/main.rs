use colored::Colorize;
use dialoguer::MultiSelect;
use std::str;
mod cli;
mod git;

fn main() -> Result<(), Error> {
    let args = cli::parse_args(std::env::args());
    let branches = git::branch_list(&args.root_dir);
    let to_delete = match select_branches(&branches)? {
        Some(x) => x,
        None => return Ok(()),
    };
    print_selection(&branches, &to_delete);
    match confirm_action("Delete branches?", true)? {
        false => return Ok(()),
        true => (),
    }
    act_on_branches(|x: &str| git::delete_branch(x, &args.root_dir), &to_delete);
    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Git process failed: {0}")]
    Git(String),
    #[error("Error getting user input: {0}")]
    Interactive(String),
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

fn print_selection(full_collection: &[String], selected: &[String]) {
    for item in full_collection {
        if selected.contains(item) {
            println!("❌ {}", item)
        } else {
            println!("✔️ {}", item);
        }
    }
}

fn confirm_action(message: &str, default: bool) -> Result<bool, Error> {
    match dialoguer::Confirm::new()
        .with_prompt(message)
        .default(default)
        .interact_opt()
    {
        Ok(x) => match x {
            Some(_) => Ok(true),
            None => Ok(false),
        },
        Err(e) => Err(Error::Interactive(e.to_string())),
    }
}

fn act_on_branches<T>(func: T, branches: &Vec<String>)
where
    T: Fn(&str) -> Result<(), Error>,
{
    for branch in branches {
        match func(branch) {
            Ok(_) => (),
            Err(e) => println!("⚠️ {} - {}", branch, format!("{}", e).yellow()),
        }
    }
}
