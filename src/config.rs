use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::theme::GbsTheme;
use crate::Error;

const COULD_NOT_CREATE: &str = "could not create config file";
const COULD_NOT_DIR: &str = "could not create directory";
const COULD_NOT_OPEN: &str = "could not open config file";
const COULD_NOT_PARSE: &str = "could not parse config file";
const COULD_NOT_READ: &str = "could not parse config file";
const COULD_NOT_WRITE: &str = "could not write config file";
const CONFIG_DIR_NAME: &str = "git-branch-selector";
const CONFIG_FILE_NAME: &str = "config";
const CONFIG_FILE_EXT: &str = "json";

pub fn init_config() -> Result<Config, Error> {
    let file_path = match config_path() {
        Some(x) => x,
        None => return Err(Error::Config("could not build config path.".to_string())),
    };
    make_dir_if_not_exist(file_path.parent().unwrap())?;
    let mut file: File;
    if !file_path.exists() {
        file = match File::create(&file_path) {
            Ok(x) => x,
            Err(e) => {
                return Err(Error::Config(format!(
                    "{} '{}': {}",
                    COULD_NOT_CREATE,
                    file_path.to_string_lossy().to_owned(),
                    e
                )))
            }
        };
        let default_config = Config::default();
        if let Err(e) = write!(&mut file, "{}", default_config.to_json()?) {
            return Err(Error::Config(format!("{}: {}", COULD_NOT_WRITE, e)));
        };
        Ok(default_config)
    } else {
        file = match File::open(file_path) {
            Ok(x) => x,
            Err(e) => return Err(Error::Config(format!("{}: {}", COULD_NOT_OPEN, e))),
        };
        Ok(Config::from_json(&mut file)?)
    }
}

pub fn config_path() -> Option<PathBuf> {
    let dirs = directories::BaseDirs::new();
    let config_dir = match dirs {
        Some(x) => x.config_dir().to_owned(),
        None => return None,
    };
    Some(
        config_dir
            .join(CONFIG_DIR_NAME)
            .join(format!("{}.{}", CONFIG_FILE_NAME, CONFIG_FILE_EXT)),
    )
}

fn make_dir_if_not_exist(dir: &Path) -> Result<(), Error> {
    match std::fs::create_dir_all(dir) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::Config(format!(
            "{} '{}': {}",
            COULD_NOT_DIR,
            dir.to_string_lossy().to_owned(),
            e
        ))),
    }
}

#[derive(Deserialize, Serialize, Debug)]
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

#[derive(Deserialize, Serialize, Debug)]
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

    pub fn to_json(&self) -> Result<String, Error> {
        match serde_json::to_string_pretty(self) {
            Ok(x) => Ok(x),
            Err(e) => Err(Error::Config(format!("could not serialize config: {}", e))),
        }
    }

    pub fn from_json(reader: &mut impl Read) -> Result<Config, Error> {
        let mut toml_str = String::new();
        if let Err(e) = reader.read_to_string(&mut toml_str) {
            return Err(Error::Config(format!("{}: {}", COULD_NOT_READ, e)));
        };
        match serde_json::from_str(&toml_str) {
            Ok(config) => Ok(config),
            Err(e) => Err(Error::Config(format!("{}: {}", COULD_NOT_PARSE, e))),
        }
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
    }
}
