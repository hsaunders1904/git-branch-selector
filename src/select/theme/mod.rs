pub mod style;
pub mod styled_string;

use serde::{Deserialize, Serialize};

use style::Style;
use styled_string::StyledString;

pub const DEFAULT_THEME: &str = "default";

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(default)]
pub struct ConsoleTheme {
    pub name: String,
    pub checked_item_prefix: StyledString,
    pub unchecked_item_prefix: StyledString,
    pub active_item_prefix: StyledString,
    pub inactive_item_prefix: StyledString,
    pub active_item_style: Style,
    pub inactive_item_style: Style,
}

fn default_name() -> String {
    DEFAULT_THEME.to_string()
}

impl Default for ConsoleTheme {
    fn default() -> ConsoleTheme {
        ConsoleTheme {
            name: default_name(),
            checked_item_prefix: StyledString {
                value: "[x]".to_string(),
                ..Default::default()
            },
            unchecked_item_prefix: StyledString {
                value: "[ ]".to_string(),
                ..Default::default()
            },
            active_item_prefix: StyledString {
                value: "> ".to_string(),
                ..Default::default()
            },
            inactive_item_prefix: StyledString {
                value: "  ".to_string(),
                ..Default::default()
            },
            active_item_style: Style::default(),
            inactive_item_style: Style::default(),
        }
    }
}

impl dialoguer::theme::Theme for ConsoleTheme {
    fn format_multi_select_prompt_item(
        &self,
        f: &mut dyn std::fmt::Write,
        text: &str,
        checked: bool,
        active: bool,
    ) -> std::fmt::Result {
        let details = match (checked, active) {
            (true, true) => (
                self.active_item_prefix.clone(),
                self.checked_item_prefix.clone(),
                self.active_item_style.apply_to(text),
            ),
            (true, false) => (
                self.inactive_item_prefix.clone(),
                self.checked_item_prefix.clone(),
                self.inactive_item_style.apply_to(text),
            ),
            (false, true) => (
                self.active_item_prefix.clone(),
                self.unchecked_item_prefix.clone(),
                self.active_item_style.apply_to(text),
            ),
            (false, false) => (
                self.inactive_item_prefix.clone(),
                self.unchecked_item_prefix.clone(),
                self.inactive_item_style.apply_to(text),
            ),
        };

        write!(f, "{}{} {}", details.0, details.1, details.2)
    }
}

#[cfg(test)]
mod tests {
    use dialoguer::theme::Theme;

    use super::*;

    #[test]
    fn default_theme_formats_to_expected_string_given_selected_and_checked() {
        let theme = ConsoleTheme::default();
        let mut out = String::new();

        theme
            .format_multi_select_prompt_item(&mut out, "text", true, true)
            .unwrap();

        assert_eq!(out, "> [x] text");
    }

    #[test]
    fn default_theme_formats_to_expected_string_given_selected_and_unchecked() {
        let theme = ConsoleTheme::default();
        let mut out = String::new();

        theme
            .format_multi_select_prompt_item(&mut out, "text", false, true)
            .unwrap();

        assert_eq!(out, "> [ ] text");
    }

    #[test]
    fn default_theme_formats_to_expected_string_given_unchecked() {
        let theme = ConsoleTheme::default();
        let mut out = String::new();

        theme
            .format_multi_select_prompt_item(&mut out, "text", false, false)
            .unwrap();

        assert_eq!(out, "  [ ] text");
    }

    #[test]
    fn default_theme_formats_to_expected_string_given_checked() {
        let theme = ConsoleTheme::default();
        let mut out = String::new();

        theme
            .format_multi_select_prompt_item(&mut out, "text", true, false)
            .unwrap();

        assert_eq!(out, "  [x] text");
    }

    #[test]
    fn theme_read_from_json_formats_to_expected_string() {
        let json = r#"
        {
            "name": "emoji",
            "checked_item_prefix": {
                "value": "✓",
                "foreground": "green"
            },
            "unchecked_item_prefix": {
                "value": " ",
                "foreground": "red"
            },
            "active_item_prefix": {
                "value": "👉 "
            },
            "inactive_item_prefix": {
                "value": "   "
            }
        }
        "#;
        let theme: ConsoleTheme = serde_json::from_str(json).unwrap();
        let mut out = String::new();

        theme
            .format_multi_select_prompt_item(&mut out, "some_branch", true, true)
            .unwrap();

        assert_eq!(out, "👉 \u{1b}[32m✓\u{1b}[0m some_branch");
    }
}
