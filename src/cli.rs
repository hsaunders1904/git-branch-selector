use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version)]
pub struct Args {
    #[clap(value_parser, default_value_t = get_working_dir())]
    pub git_dir: String,
}

fn get_working_dir() -> String {
    match std::env::current_dir() {
        Ok(x) => match x.to_str() {
            Some(x) => x.to_string(),
            None => String::from('.'),
        },
        Err(_) => String::from('.'),
    }
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

            assert_eq!(
                args.git_dir,
                std::env::current_dir()
                    .expect("Couldn't get working directory.")
                    .to_str()
                    .expect("Couldn't convert working directory to string.")
            );
        }
    }
}
