use std::io::Write;
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::Error;

const CONFIG_DIR_NAME: &str = "git-branch-selector";
const CONFIG_FILE_NAME: &str = "config";
const CONFIG_FILE_EXT: &str = "json";

pub fn init_config(path: &PathBuf) -> Result<Config, Error> {
    if !path.is_file() {
        create_config_dirs(path)?;
        return create_new_config_file(path);
    }
    read_config_file(path)
}

pub fn config_path() -> Result<PathBuf, Error> {
    let conf_dir = match directories::BaseDirs::new() {
        Some(x) => Ok(x.config_dir().join(CONFIG_DIR_NAME)),
        None => return Err(Error::Config("could not find config directory".to_string())),
    }?;
    Ok(conf_dir.join(format!("{CONFIG_FILE_NAME}.{CONFIG_FILE_EXT}")))
}

fn read_config_file(path: &PathBuf) -> Result<Config, Error> {
    let mut config_file = std::fs::File::open(path).map_err(|e| {
        Error::Config(format!(
            "{} ' {}': {e}",
            "could not open config file",
            path.to_string_lossy()
        ))
    })?;
    Config::from_json(&mut config_file)
}

fn create_config_dirs(path: &Path) -> Result<(), Error> {
    let conf_dir = match path.parent() {
        Some(p) => p,
        None => {
            return Err(Error::Config(
                "config path does not have a parent".to_string(),
            ))
        }
    };
    std::fs::create_dir_all(conf_dir).map_err(|e| {
        Error::Config(format!(
            "{} '{}': {}",
            "could not create config directory",
            conf_dir.to_string_lossy(),
            e
        ))
    })
}

fn create_new_config_file(path: &Path) -> Result<Config, Error> {
    let mut config_file = std::fs::File::create(path).map_err(|e| {
        Error::Config(format!(
            "{} '{}': {e}",
            "could not create config file",
            path.to_string_lossy()
        ))
    })?;
    let config = Config::default();
    config_file
        .write(config.to_json()?.as_bytes())
        .map_err(|e| {
            Error::Config(format!(
                "{} '{}': {}",
                "could not write config file",
                path.to_string_lossy(),
                e
            ))
        })?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_file_named_config() {
        assert!(config_path()
            .unwrap()
            .to_str()
            .unwrap()
            .ends_with("config.json"));
    }

    mod init_config {
        use super::*;

        #[test]
        fn default_config_file_created_if_dir_does_not_exist() {
            let base_dir = tempfile::tempdir().unwrap();
            let conf_path = base_dir
                .path()
                .join("a_dir")
                .join("bselect")
                .join("config.json");

            let conf = init_config(&conf_path).unwrap();

            assert!(conf_path.is_file());
            assert_eq!(conf, Config::default())
        }

        #[test]
        fn config_read_if_file_exists() {
            let base_dir = tempfile::tempdir().unwrap();
            let conf_path = base_dir.path().join("config.json");
            let file_content = r#"{
                "theme": "custom_theme_B",
                "themes": [
                    {
                        "name": "custom_theme_A",
                        "checked_item_prefix": {
                            "value": "X",
                            "foreground": "green"
                        }
                    },
                    {
                        "name": "custom_theme_B",
                        "active_item_prefix": {
                            "value": "-> ",
                            "background": "red"
                        }
                    }
                ]
            }"#;
            write!(std::fs::File::create(&conf_path).unwrap(), "{file_content}").unwrap();

            let conf = init_config(&conf_path).unwrap();

            let expected_config = Config::from_json(&mut file_content.as_bytes()).unwrap();
            assert_eq!(conf, expected_config);
        }
    }
}
