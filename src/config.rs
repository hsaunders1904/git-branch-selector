use serde::Deserialize;
use std::fs;
use std::io::Read;

use crate::{theme::GbsTheme, Error};

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
            return Err(Error::Config(format!("could not read config file: {}", e)));
        };
        match toml::from_str(&toml_str) {
            Ok(config) => Ok(config),
            Err(e) => Err(Error::Config(format!("could not parse config file: {}", e))),
        }
    }

    pub fn from_toml_file(file_path: &str) -> Result<Config, Error> {
        let mut reader = match fs::File::open(file_path) {
            Ok(x) => x,
            Err(e) => return Err(Error::Config(format!("could not open config file: {}", e))),
        };
        Self::from_toml(&mut reader)
    }
}
