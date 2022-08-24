use serde::Deserialize;

use crate::theme::GbsTheme;

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
