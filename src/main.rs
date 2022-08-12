use colored::Colorize;
use dialoguer::MultiSelect;
use std::process::Command;
use std::str;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Git process failed: {0}")]
    Git(String),
    #[error("Error getting user input: {0}")]
    Interactive(String),
}

fn clean_branch(branch: &str) -> &str {
    match branch.strip_prefix("*") {
        Some(x) => x,
        None => branch,
    }
    .trim()
}

fn git_branch_list(working_dir: &str) -> Vec<String> {
    let output = Command::new("git")
        .arg("branch")
        .arg("--list")
        .current_dir(working_dir)
        .output()
        .expect("'git' command failed.")
        .stdout;
    let stdout = str::from_utf8(&output).expect("Could not decode git branch output.");

    let branch_list = stdout
        .split("\n")
        .filter(|x| !x.trim().is_empty())
        .map(clean_branch)
        .map(|x| x.to_owned())
        .collect();
    return branch_list;
}

fn delete_branch(branch: &str, working_dir: &str) -> Result<(), Error> {
    match Command::new("git")
        .arg("branch")
        .arg("-d")
        .arg(branch)
        .current_dir(working_dir)
        .spawn()
    {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::Git(e.to_string())),
    }
}

fn select_branches(branches: &Vec<String>) -> Result<Option<Vec<String>>, Error> {
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

fn print_selection(full_collection: &Vec<String>, selected: &Vec<String>) {
    for item in full_collection {
        if selected.contains(&item) {
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

fn parse_args(mut args: impl Iterator<Item = String>) -> String {
    match args.nth(1) {
        Some(x) => x.clone(),
        None => std::env::current_dir()
            .expect("Could not get working directory.")
            .to_str()
            .unwrap()
            .to_owned(),
    }
}

fn main() -> Result<(), Error> {
    let root_dir = parse_args(std::env::args());
    let branches = git_branch_list(&root_dir);
    let to_delete = match select_branches(&branches)? {
        Some(x) => x,
        None => return Ok(()),
    };
    print_selection(&branches, &to_delete);
    match confirm_action("Delete branches?", true)? {
        false => return Ok(()),
        true => (),
    }
    act_on_branches(|x: &str| delete_branch(x, &root_dir), &to_delete);
    return Ok(());
}
