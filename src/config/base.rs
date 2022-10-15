use serde::{Deserialize, Serialize};

use crate::theme::GbsTheme;

#[derive(Deserialize, Serialize, Debug)]
pub struct BaseConfig {
    pub theme: String,
}

impl Default for BaseConfig {
    fn default() -> Self {
        BaseConfig {
            theme: GbsTheme::default().name,
        }
    }
}

#[cfg(test)]
mod tests {

    mod base_config {
        use crate::config::base::BaseConfig;

        #[test]
        fn default_uses_default_theme() {
            let config = BaseConfig::default();

            assert_eq!(config.theme, "default");
        }
    }
}
