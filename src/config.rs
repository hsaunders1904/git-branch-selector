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

    mod base_config {
        use crate::config::BaseConfig;

        #[test]
        fn default_config_uses_default_theme() {
            let config = BaseConfig::default();

            assert_eq!(config.theme, "default");
        }
    }

    mod config {

        use crate::config::Config;

        #[test]
        fn has_one_theme_by_default() {
            let config = Config::default();

            assert_eq!(config.themes.len(), 1);
        }

        mod theme {
            use crate::theme::{Style, StyledString};
            use crate::{
                config::{BaseConfig, Config},
                theme::GbsTheme,
            };

            #[test]
            fn returns_default_if_set_theme_not_found() {
                let config = Config {
                    base: BaseConfig {
                        theme: "not_a_theme".to_string(),
                    },
                    themes: vec![GbsTheme::default()],
                };

                assert_eq!(config.theme(), GbsTheme::default());
            }

            #[test]
            fn returns_theme_set_in_base() {
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
        }

        mod from_toml {
            use super::super::super::*;

            #[test]
            fn config_err_given_invalid_toml() {
                let mut reader =
                    std::io::BufReader::new("not valid toml\n= not valid toml".as_bytes());

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

            #[test]
            fn valid_toml_sets_config_options() {
                let toml_str = r#"[base]
            theme = "tester"

            [[themes]]
            name = "tester"
            checked_item_prefix.value = "tick"
            unchecked_item_prefix.value = "cross"
            active_item_prefix.value = "point"
            inactive_item_prefix.value = "spaces"
            active_item_style.foreground = "cyan"
            active_item_style.fg_bright = true

            [[themes]]
            name = "tester2"
            checked_item_prefix.value = "tick"
            unchecked_item_prefix.value = "cross"
            active_item_prefix.value = "point"
        "#;
                let mut reader = std::io::BufReader::new(toml_str.as_bytes());

                let config = Config::from_toml(&mut reader).unwrap();

                assert_eq!(config.theme().name, "tester");
                assert_eq!(config.theme().checked_item_prefix.value.unwrap(), "tick");
                assert_eq!(config.theme().unchecked_item_prefix.value.unwrap(), "cross");
                assert_eq!(config.theme().active_item_prefix.value.unwrap(), "point");
                assert_eq!(config.theme().inactive_item_prefix.value.unwrap(), "spaces");
                assert_eq!(config.theme().active_item_style.foreground.unwrap(), "cyan");
                assert_eq!(config.theme().active_item_style.background, None);
                assert!(config.theme().active_item_style.fg_bright);
                assert!(!config.theme().active_item_style.bg_bright);
                assert_eq!(config.themes.len(), 2);
            }
        }

        mod from_toml_file {
            use super::super::super::*;

            #[test]
            fn returns_error_given_file_does_not_exist() {
                let result = Config::from_toml_file("not a file");

                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains(COULD_NOT_OPEN));
            }
        }
    }
}
