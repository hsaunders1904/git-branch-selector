use dialoguer::console;
use serde::{Deserialize, Serialize};

use crate::select::theme::style::Style;

use std::convert::From;

#[derive(Deserialize, Serialize, Clone, Debug, Default, Eq, PartialEq)]
pub struct StyledString {
    #[serde(default)]
    pub value: String,
    #[serde(default, flatten)]
    pub style: Style,
}

impl std::fmt::Display for StyledString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.style.apply_to(&self.value))
    }
}

impl From<StyledString> for console::StyledObject<String> {
    fn from(item: StyledString) -> console::StyledObject<String> {
        let style = console::Style::from(item.style);
        style.apply_to(item.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_construction_gives_expected_defaults() {
        let ss = StyledString::default();

        assert_eq!(ss.value, "");
        assert_eq!(ss.style, Style::default());
    }

    #[test]
    fn converting_to_style_object_from_default_gives_expected_value() {
        let ss = StyledString::default();

        let styled_obj = console::StyledObject::from(ss);

        let expected_style = console::Style::new().for_stderr();
        let expected_obj = expected_style.apply_to("");
        assert_eq!(format!("{}", styled_obj), format!("{}", expected_obj));
    }

    #[test]
    fn converting_to_console_style_given_expected_value() {
        let style = Style {
            background: Some("green".to_string()),
            foreground: Some("red".to_string()),
            bg_bright: true,
            fg_bright: true,
        };
        let ss = StyledString {
            value: "my_text".to_string(),
            style,
        };

        let expected_style = console::Style::new()
            .for_stderr()
            .red()
            .on_green()
            .bright()
            .on_bright();
        let expected_obj = expected_style.apply_to("my_text");
        assert_eq!(format!("{}", ss), format!("{}", expected_obj));
    }
}
