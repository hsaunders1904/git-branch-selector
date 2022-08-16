use std::fmt;

use dialoguer::{console, theme::Theme};
use serde::Serialize;

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

#[derive(Serialize)]
pub struct Style {
    foreground: Option<String>,
    background: Option<String>,
    fg_bright: bool,
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

#[derive(Serialize)]
pub struct StyledString {
    value: Option<String>,
    foreground: Option<String>,
    background: Option<String>,
    fg_bright: bool,
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

pub struct GbsTheme {
    pub checked_item_prefix: StyledString,
    pub unchecked_item_prefix: StyledString,
    pub active_item_prefix: StyledString,
    pub inactive_item_prefix: StyledString,
    pub active_item_style: Style,
    pub inactive_item_style: Style,
}

impl Default for GbsTheme {
    fn default() -> GbsTheme {
        GbsTheme {
            checked_item_prefix: StyledString {
                value: Some("✓".to_string()),
                foreground: Some("green".to_string()),
                background: None,
                fg_bright: true,
                bg_bright: false,
            },
            unchecked_item_prefix: StyledString {
                value: Some("✗".to_string()),
                foreground: Some("white".to_string()),
                background: None,
                fg_bright: false,
                bg_bright: false,
            },
            active_item_prefix: StyledString {
                value: Some("> ".to_string()),
                foreground: Some("white".to_string()),
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
                foreground: Some("cyan".to_string()),
                background: None,
                fg_bright: true,
                bg_bright: false,
            },
            inactive_item_style: Style {
                foreground: Some("white".to_string()),
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
