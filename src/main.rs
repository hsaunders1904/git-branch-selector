use colored::Colorize;
use dialoguer::MultiSelect;
use std::error::Error;
use std::process::{Child, Command};
use std::str;

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

fn delete_branch(branch: &str, working_dir: &str) -> std::io::Result<Child> {
    Command::new("git")
        .arg("branch")
        .arg("-d")
        .arg(branch)
        .current_dir(working_dir)
        .spawn()
}

fn main() -> Result<(), Box<dyn Error>> {
    let root_dir = match std::env::args().nth(1) {
        Some(x) => x,
        None => std::env::current_dir()
            .expect("Could not get working directory.")
            .to_str()
            .unwrap()
            .to_owned(),
    };

    let branches = git_branch_list(&root_dir);
    let chosen_idxs: Vec<usize> = MultiSelect::new().items(&branches).interact()?;
    let to_delete = chosen_idxs
        .iter()
        .map(|i| branches[*i].to_owned())
        .collect::<Vec<_>>();
    if to_delete.is_empty() {
        return Ok(());
    }

    for branch in &branches {
        if to_delete.contains(&branch) {
            println!("❌ {}", branch)
        } else {
            println!("✔️ {}", branch);
        }
    }

    match dialoguer::Confirm::new()
        .with_prompt("Delete branches?")
        .default(true)
        .interact()?
    {
        false => return Ok(()),
        true => (),
    }

    for branch in branches {
        if to_delete.contains(&branch) {
            match delete_branch(&branch, &root_dir) {
                Ok(_) => (),
                Err(e) => println!("⚠️ {} - {}", branch, format!("{}", e.to_string()).yellow()),
            }
        }
    }
    return Ok(());
}
