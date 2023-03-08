pub mod file;

use serde::{Deserialize, Serialize};

use crate::select::theme::{ConsoleTheme, DEFAULT_THEME};
use crate::Error;

const COULD_NOT_PARSE: &str = "could not parse config file";
const COULD_NOT_READ: &str = "could not parse config file";
const COULD_NOT_SERIALIZE: &str = "could not serialize config";

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct Config {
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_themes")]
    pub themes: Vec<ConsoleTheme>,
}

fn default_theme() -> String {
    "default".to_string()
}

fn default_themes() -> Vec<ConsoleTheme> {
    vec![ConsoleTheme::default()]
}

impl Default for Config {
    fn default() -> Self {
        Config {
            theme: crate::select::theme::DEFAULT_THEME.to_string(),
            themes: vec![ConsoleTheme::default()],
        }
    }
}

impl Config {
    pub fn theme(&self) -> ConsoleTheme {
        for t in &self.themes {
            if t.name == self.theme {
                return t.clone();
            }
        }
        ConsoleTheme::default()
    }

    pub fn to_json(&self) -> Result<String, Error> {
        serde_json::to_string_pretty(self)
            .map_err(|e| Error::Config(format!("{COULD_NOT_SERIALIZE}: {e}")))
    }

    pub fn from_json(to_read: &mut impl std::io::Read) -> Result<Config, Error> {
        let mut json_str = String::new();
        to_read
            .read_to_string(&mut json_str)
            .map_err(|e| Error::Config(format!("{COULD_NOT_READ}: {e}")))?;
        let mut config: Config = serde_json::from_str(&json_str)
            .map_err(|e| Error::Config(format!("{COULD_NOT_PARSE}: {e}")))?;
        if !config.themes.iter().any(|t| t.name == DEFAULT_THEME) {
            config.themes.push(ConsoleTheme::default());
        }
        Ok(config)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::select::theme::style::Style;
    use crate::select::theme::styled_string::StyledString;

    #[test]
    fn empty_json_returns_default_config() {
        let mut to_read = "{}".as_bytes();
        let conf = Config::from_json(&mut to_read).unwrap();

        assert_eq!(conf, Config::default());
    }

    #[test]
    fn theme_returns_theme_with_name_from_config() {
        let json = r#"
        {
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
        }
        "#;

        let config = Config::from_json(&mut json.as_bytes()).unwrap();

        let expected_theme = ConsoleTheme {
            name: "custom_theme_B".to_string(),
            active_item_prefix: StyledString {
                value: "-> ".to_string(),
                style: Style {
                    background: Some("red".to_string()),
                    ..Default::default()
                },
            },
            ..Default::default()
        };
        assert_eq!(config.theme().name, "custom_theme_B");
        assert_eq!(config.theme(), expected_theme);
    }

    #[test]
    fn theme_returns_default_theme_if_name_from_config_is_not_a_theme() {
        let json = r#"
        {
            "theme": "not_a theme",
            "themes": [
                {
                    "name": "A",
                    "checked_item_prefix": {
                        "value": "X",
                        "foreground": "green"
                    }
                }
            ]
        }
        "#;

        let config = Config::from_json(&mut json.as_bytes()).unwrap();

        assert_eq!(config.theme(), ConsoleTheme::default());
    }

    #[test]
    fn custom_theme_read_from_json() {
        let json = r#"{
            "theme": "my_theme",
            "themes": [
                {
                    "name": "my_theme",
                    "checked_item_prefix": {
                        "value": "X",
                        "foreground": "green"
                    },
                    "active_item_prefix": {
                        "value": "-> "
                    },
                    "inactive_item_prefix": {
                        "value": "   "
                    }
                }
            ]
        }
        "#;
        let mut to_read = json.as_bytes();
        let conf = Config::from_json(&mut to_read).unwrap();

        let expected_theme = ConsoleTheme {
            checked_item_prefix: StyledString {
                value: "X".to_string(),
                style: Style {
                    foreground: Some("green".to_string()),
                    ..Default::default()
                },
            },
            active_item_prefix: StyledString {
                value: "-> ".to_string(),
                ..Default::default()
            },
            inactive_item_prefix: StyledString {
                value: "   ".to_string(),
                ..Default::default()
            },
            name: "my_theme".to_string(),
            ..Default::default()
        };
        let expected_conf = Config {
            theme: "my_theme".to_string(),
            themes: vec![expected_theme, ConsoleTheme::default()],
        };
        assert_eq!(conf, expected_conf);
    }

    #[test]
    fn read_json_equal_to_generated_json() {
        let json = r#"{
            "base": {
              "theme": "emoji"
            },
            "themes": [
              {
                "name": "default",
                "checked_item_prefix": {
                  "value": "[x]",
                  "foreground": null,
                  "background": null,
                  "fg_bright": false,
                  "bg_bright": false
                },
                "unchecked_item_prefix": {
                  "value": "[ ]",
                  "foreground": null,
                  "background": null,
                  "fg_bright": false,
                  "bg_bright": false
                },
                "active_item_prefix": {
                  "value": "> ",
                  "foreground": null,
                  "background": null,
                  "fg_bright": false,
                  "bg_bright": false
                },
                "inactive_item_prefix": {
                  "value": "  ",
                  "foreground": null,
                  "background": null,
                  "fg_bright": false,
                  "bg_bright": false
                },
                "active_item_style": {
                  "foreground": null,
                  "background": null,
                  "fg_bright": false,
                  "bg_bright": false
                },
                "inactive_item_style": {
                  "foreground": null,
                  "background": null,
                  "fg_bright": false,
                  "bg_bright": false
                }
              },
              {
                "name": "emoji",
                "checked_item_prefix": {
                  "value": "X",
                  "foreground": "green"
                },
                "unchecked_item_prefix": {
                  "value": " ",
                  "foreground": "red"
                },
                "active_item_prefix": {
                  "value": "-> "
                },
                "inactive_item_prefix": {
                  "value": "   "
                }
              }
            ]
          }
        "#;

        let config = Config::from_json(&mut json.as_bytes()).unwrap();
        let read_json = config.to_json().unwrap();
        let read_config = Config::from_json(&mut read_json.as_bytes());

        assert_eq!(config, read_config.unwrap());
    }

    #[test]
    fn default_theme_available_if_not_in_themes_list() {
        let json = r#"{
            "theme": "default",
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
        let config = Config::from_json(&mut json.as_bytes()).unwrap();

        assert_eq!(config.theme(), ConsoleTheme::default());
    }
}
