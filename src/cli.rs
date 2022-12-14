use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version)]
#[clap(about = "Interactively select git branches and print them to stdout")]
pub struct Args {
    #[clap(
        value_parser,
        help = "List only the branches that match the given pattern(s), uses same syntax as 'git branch --list'"
    )]
    pub filters: Vec<String>,

    #[clap(
        value_parser,
        long,
        default_value = ".",
        help = "Path to git repository"
    )]
    pub git_dir: String,

    #[clap(
        long,
        action,
        help = "Print the path to the configuration file and exit"
    )]
    pub config: bool,

    #[clap(
        long,
        action,
        help = "List both remote-tracking branches and local branches"
    )]
    pub all: bool,
}

pub fn parse_args(args: impl Iterator<Item = String>) -> Args {
    Args::parse_from(args)
}

#[cfg(test)]
mod tests {
    mod parse_args {
        use crate::cli::parse_args;

        macro_rules! to_string_iter {
            ($element: expr) => {{
                $element.iter().map(|s| s.to_string())
            }};
        }

        #[test]
        fn git_dir_is_first_positional_arg() {
            let cli_args = to_string_iter!(["", "--git-dir", "/some/dir"]);

            let args = parse_args(cli_args);

            assert_eq!(args.git_dir, "/some/dir");
        }

        #[test]
        fn git_dir_is_working_dir_by_default() {
            let cli_args = to_string_iter!([""]);

            let args = parse_args(cli_args);

            assert_eq!(args.git_dir, ".");
        }

        #[test]
        fn config_false_by_default() {
            let cli_args = to_string_iter!([""]);

            let args = parse_args(cli_args);

            assert!(!args.config);
        }

        #[test]
        fn config_true_given_flag() {
            let cli_args = to_string_iter!(["", "--config"]);

            let args = parse_args(cli_args);

            assert!(args.config);
        }

        #[test]
        fn filter_is_empty_if_not_given() {
            let cli_args = to_string_iter!([""]);

            let args = parse_args(cli_args);

            assert!(args.filters.is_empty());
        }

        #[test]
        fn filter_contains_single_given_string() {
            let cli_args = to_string_iter!(["", "origin/*"]);

            let args = parse_args(cli_args);

            assert_eq!(args.filters, ["origin/*"]);
        }

        #[test]
        fn multiple_filters_can_be_set() {
            let cli_args = to_string_iter!(["", "origin/*", "upstream/*"]);

            let args = parse_args(cli_args);

            assert_eq!(args.filters, ["origin/*", "upstream/*"]);
        }

        #[test]
        fn all_is_false_if_flag_not_given() {
            let cli_args = to_string_iter!(["", "origin/*"]);

            let args = parse_args(cli_args);

            assert!(!args.all);
        }

        #[test]
        fn all_is_true_given_flag() {
            let cli_args = to_string_iter!(["", "--all"]);

            let args = parse_args(cli_args);

            assert!(args.all);
        }
    }
}
