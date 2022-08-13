use clap::Parser;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Action {
    Delete,
}

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
