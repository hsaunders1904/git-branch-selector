use dialoguer::{console, theme::Theme};
use serde::Deserialize;
use std::fmt;

fn to_color(color: &str) -> Option<console::Color> {
    match color {
        "black" => Some(console::Color::Black),
        "red" => Some(console::Color::Red),
        "green" => Some(console::Color::Green),
        "yellow" => Some(console::Color::Yellow),
        "blue" => Some(console::Color::Blue),
        "magenta" => Some(console::Color::Magenta),
        "cyan" => Some(console::Color::Cyan),
        "white" => Some(console::Color::White),
        _ => None,
    }
}

fn default_as_false() -> bool {
    false
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct Style {
    #[serde(default)]
    foreground: Option<String>,
    #[serde(default)]
    background: Option<String>,
    #[serde(default = "default_as_false")]
    fg_bright: bool,
    #[serde(default = "default_as_false")]
    bg_bright: bool,
}

impl Style {
    pub fn to_console(&self) -> console::Style {
        let mut style = console::Style::new().for_stderr();
        if self.foreground.is_some() {
            style = match to_color(self.foreground.as_ref().unwrap()) {
                Some(color) => style.fg(color),
                None => style,
            };
        }
        if self.background.is_some() {
            style = match to_color(self.background.as_ref().unwrap()) {
                Some(color) => style.bg(color),
                None => style,
            };
        }
        if self.fg_bright {
            style = style.bright();
        }
        if self.bg_bright {
            style = style.on_bright();
        };
        style
    }
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct StyledString {
    #[serde(default)]
    value: Option<String>,
    #[serde(default)]
    foreground: Option<String>,
    #[serde(default)]
    background: Option<String>,
    #[serde(default = "default_as_false")]
    fg_bright: bool,
    #[serde(default = "default_as_false")]
    bg_bright: bool,
}

impl StyledString {
    pub fn to_console(&self) -> console::StyledObject<String> {
        let mut style_obj =
            console::style(self.value.clone().unwrap_or_else(|| "".to_string())).for_stderr();
        if self.foreground.is_some() {
            style_obj = match to_color(self.foreground.as_ref().unwrap()) {
                Some(color) => style_obj.fg(color),
                None => style_obj,
            };
        };
        if self.background.is_some() {
            style_obj = match to_color(self.background.as_ref().unwrap()) {
                Some(color) => style_obj.bg(color),
                None => style_obj,
            };
        };
        if self.fg_bright {
            style_obj = style_obj.bright();
        }
        if self.bg_bright {
            style_obj = style_obj.on_bright();
        };
        style_obj
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct GbsTheme {
    pub name: String,
    #[serde(default)]
    pub checked_item_prefix: StyledString,
    #[serde(default)]
    pub unchecked_item_prefix: StyledString,
    #[serde(default)]
    pub active_item_prefix: StyledString,
    #[serde(default)]
    pub inactive_item_prefix: StyledString,
    #[serde(default)]
    pub active_item_style: Style,
    #[serde(default)]
    pub inactive_item_style: Style,
}

impl Default for GbsTheme {
    fn default() -> GbsTheme {
        GbsTheme {
            name: "default".to_string(),
            checked_item_prefix: StyledString {
                value: Some("[x]".to_string()),
                foreground: None,
                background: None,
                fg_bright: false,
                bg_bright: false,
            },
            unchecked_item_prefix: StyledString {
                value: Some("[ ]".to_string()),
                foreground: None,
                background: None,
                fg_bright: false,
                bg_bright: false,
            },
            active_item_prefix: StyledString {
                value: Some("> ".to_string()),
                foreground: None,
                background: None,
                fg_bright: false,
                bg_bright: false,
            },
            inactive_item_prefix: StyledString {
                value: Some("  ".to_string()),
                foreground: None,
                background: None,
                fg_bright: false,
                bg_bright: false,
            },
            active_item_style: Style {
                foreground: None,
                background: None,
                fg_bright: false,
                bg_bright: false,
            },
            inactive_item_style: Style {
                foreground: None,
                background: None,
                fg_bright: false,
                bg_bright: false,
            },
        }
    }
}

impl Theme for GbsTheme {
    fn format_multi_select_prompt_item(
        &self,
        f: &mut dyn fmt::Write,
        text: &str,
        checked: bool,
        active: bool,
    ) -> fmt::Result {
        let details = match (checked, active) {
            (true, true) => (
                self.active_item_prefix.to_console(),
                self.checked_item_prefix.to_console(),
                self.active_item_style.to_console().apply_to(text),
            ),
            (true, false) => (
                self.inactive_item_prefix.to_console(),
                self.checked_item_prefix.to_console(),
                self.inactive_item_style.to_console().apply_to(text),
            ),
            (false, true) => (
                self.active_item_prefix.to_console(),
                self.unchecked_item_prefix.to_console(),
                self.active_item_style.to_console().apply_to(text),
            ),
            (false, false) => (
                self.inactive_item_prefix.to_console(),
                self.unchecked_item_prefix.to_console(),
                self.inactive_item_style.to_console().apply_to(text),
            ),
        };

        write!(f, "{}{} {}", details.0, details.1, details.2)
    }
}
