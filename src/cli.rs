use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version)]
pub struct Args {
    #[clap(value_parser, default_value = ".", help = "Path to git repository")]
    pub git_dir: String,
    #[clap(long, action, help = "Print the path to the configuration file")]
    pub config: bool,
    #[clap(
        value_parser,
        long,
        help = "Filter listed branches, uses same syntax as 'git branch --list'"
    )]
    pub filter: Option<String>,
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
            let cli_args = to_string_iter!(["", "/some/dir"]);

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
        fn filter_is_none_if_not_given() {
            let cli_args = to_string_iter!([""]);

            let args = parse_args(cli_args);

            assert!(args.filter.is_none());
        }

        #[test]
        fn filter_is_some_if_given() {
            let cli_args = to_string_iter!(["", "--filter", "origin/*"]);

            let args = parse_args(cli_args);

            assert!(args.filter.is_some());
            assert_eq!(args.filter.unwrap(), "origin/*");
        }
    }
}
