use clap::Parser;

use crate::Error;

#[derive(clap::Parser, Debug)]
#[clap(
    about = "Interactively select git branches and print them to stdout",
    version
)]
pub struct Args {
    #[clap(
        value_parser,
        help = "List only the branches that match one of the given regex pattern(s)"
    )]
    pub filters: Vec<String>,
    #[clap(
        long,
        action,
        help = "List both remote-tracking branches and local branches"
    )]
    pub all: bool,
    #[clap(
        long,
        action,
        help = "Print the path to the configuration file and exit"
    )]
    pub config: bool,
    #[clap(
        value_parser,
        long,
        short = 'C',
        default_value = ".",
        help = "Path to git repository"
    )]
    pub git_dir: std::path::PathBuf,
}

pub fn parse_args<I, T>(argv: I) -> Result<Args, Error>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    Args::try_parse_from(argv).map_err(|e| match e.print() {
        Ok(_) => Error::Cli("".to_string()),
        Err(print_err) => Error::Cli(format!("could not print parser error: {}", print_err)),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_error_given_help() {
        let argv: Vec<&str> = vec!["bselect", "--help"];

        let args = parse_args(argv.iter());

        assert!(args.is_err());
    }

    #[test]
    fn no_error_given_no_args() {
        let argv: Vec<&str> = vec!["bselect"];

        let args = parse_args(argv.iter());

        assert!(args.is_ok());
    }

    #[test]
    fn first_arg_stored_in_filters() {
        let argv: Vec<&str> = vec!["bselect", "pattern"];

        let args = parse_args(argv.iter());

        assert_eq!(args.unwrap().filters, vec!["pattern"]);
    }

    #[test]
    fn all_positional_args_stored_in_filters() {
        let argv: Vec<&str> = vec!["bselect", "a", "b", "c"];

        let args = parse_args(argv.iter());

        assert_eq!(args.unwrap().filters, vec!["a", "b", "c"]);
    }

    #[test]
    fn all_is_false_given_no_args() {
        let argv: Vec<&str> = vec!["bselect"];

        let args = parse_args(argv.iter());

        assert!(!args.unwrap().all);
    }

    #[test]
    fn all_is_true_given_all_flag() {
        let argv: Vec<&str> = vec!["bselect", "--all"];

        let args = parse_args(argv.iter());

        assert!(args.unwrap().all);
    }

    #[test]
    fn config_is_false_given_flag_not_present() {
        let argv: Vec<&str> = vec!["bselect"];

        let args = parse_args(argv.iter());

        assert!(!args.unwrap().config);
    }

    #[test]
    fn config_is_true_given_flag() {
        let argv: Vec<&str> = vec!["bselect", "--config"];

        let args = parse_args(argv.iter());

        assert!(args.unwrap().config);
    }

    #[test]
    fn get_repo_set_given_value() {
        let argv: Vec<&str> = vec!["bselect", "--git-dir", "/some/path"];

        let args = parse_args(argv.iter());

        assert_eq!(args.unwrap().git_dir.to_string_lossy(), "/some/path");
    }

    #[test]
    fn get_repo_defaults_to_full_stop() {
        let argv: Vec<&str> = vec!["bselect"];

        let args = parse_args(argv.iter());

        assert_eq!(args.unwrap().git_dir.to_string_lossy(), ".");
    }
}
