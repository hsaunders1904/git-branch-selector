use serde::Deserialize;
use std::fs;
use std::io::Read;

use crate::{theme::GbsTheme, Error};

const COULD_NOT_OPEN: &str = "could not open config file";
const COULD_NOT_PARSE: &str = "could not parse config file";
const COULD_NOT_READ: &str = "could not parse config file";

#[derive(Deserialize, Debug)]
pub struct BaseConfig {
    theme: String,
}

impl Default for BaseConfig {
    fn default() -> Self {
        BaseConfig {
            theme: GbsTheme::default().name,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub base: BaseConfig,
    pub themes: Vec<GbsTheme>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            base: BaseConfig::default(),
            themes: vec![GbsTheme::default()],
        }
    }
}

impl Config {
    pub fn theme(&self) -> GbsTheme {
        for t in &self.themes {
            if t.name == self.base.theme {
                return t.clone();
            }
        }
        GbsTheme::default()
    }

    pub fn from_toml(reader: &mut impl Read) -> Result<Config, Error> {
        let mut toml_str = String::new();
        if let Err(e) = reader.read_to_string(&mut toml_str) {
            return Err(Error::Config(format!("{}: {}", COULD_NOT_READ, e)));
        };
        match toml::from_str(&toml_str) {
            Ok(config) => Ok(config),
            Err(e) => Err(Error::Config(format!("{}: {}", COULD_NOT_PARSE, e))),
        }
    }

    pub fn from_toml_file(file_path: &str) -> Result<Config, Error> {
        let mut reader = match fs::File::open(file_path) {
            Ok(x) => x,
            Err(e) => return Err(Error::Config(format!("{}: {}", COULD_NOT_OPEN, e))),
        };
        Self::from_toml(&mut reader)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::{Style, StyledString};

    #[test]
    fn theme_returns_theme_set_in_base() {
        let new_theme = GbsTheme {
            name: "new_theme".to_string(),
            checked_item_prefix: StyledString::default(),
            unchecked_item_prefix: StyledString::default(),
            active_item_prefix: StyledString::default(),
            inactive_item_prefix: StyledString::default(),
            active_item_style: Style::default(),
            inactive_item_style: Style::default(),
        };
        let conf = Config {
            base: BaseConfig {
                theme: "new_theme".to_string(),
            },
            themes: vec![GbsTheme::default(), new_theme],
        };

        assert_eq!(conf.theme().name, "new_theme");
    }

    mod from_toml {
        use super::super::*;

        #[test]
        fn config_err_given_invalid_toml() {
            let mut reader = std::io::BufReader::new("not valid toml\n= not valid toml".as_bytes());

            let err = Config::from_toml(&mut reader);

            assert!(matches!(err, Err(Error::Config(_))));
            assert!(err.unwrap_err().to_string().contains(COULD_NOT_PARSE));
        }

        #[test]
        fn config_err_given_reader_does_not_contain_valid_utf8() {
            let bytes: Vec<u8> = vec![240, 40, 140, 188]; // \xf0\x28\x8c\xbc
            let mut reader = std::io::BufReader::new(&*bytes);

            let err = Config::from_toml(&mut reader);

            assert!(matches!(err, Err(Error::Config(_))));
            assert!(err.unwrap_err().to_string().contains(COULD_NOT_READ));
        }
    }
}
