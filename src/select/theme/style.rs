use dialoguer::console;
use serde::{Deserialize, Serialize};

use std::convert::From;

#[derive(Deserialize, Serialize, Clone, Debug, Default, Eq, PartialEq)]
pub struct Style {
    #[serde(default)]
    pub foreground: Option<String>,
    #[serde(default)]
    pub background: Option<String>,
    #[serde(default)]
    pub fg_bright: bool,
    #[serde(default)]
    pub bg_bright: bool,
}

impl Style {
    pub fn apply_to(&self, text: &str) -> String {
        format!("{}", console::Style::from(self.clone()).apply_to(text))
    }
}

impl From<Style> for console::Style {
    fn from(item: Style) -> console::Style {
        let mut style = console::Style::new().for_stderr();
        if let Some(fg) = item.foreground {
            if let Some(fg_color) = to_color(&fg) {
                style = style.fg(fg_color);
            }
        }
        if let Some(bg) = item.background {
            if let Some(bg_color) = to_color(&bg) {
                style = style.bg(bg_color);
            }
        }
        if item.fg_bright {
            style = style.bright();
        }
        if item.bg_bright {
            style = style.on_bright();
        };
        style
    }
}

fn to_color(color: &str) -> Option<console::Color> {
    match color.to_lowercase().as_ref() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_construction_sets_expected_defaults() {
        let style = Style::default();

        assert_eq!(style.foreground, None);
        assert_eq!(style.background, None);
        assert!(!style.fg_bright);
        assert!(!style.bg_bright);
    }

    #[test]
    fn can_convert_to_dialoguer_console_style_with_defaults() {
        let style = Style::default();

        let console_style = console::Style::from(style);

        let expected_style = console::Style::new().for_stderr();
        assert_eq!(console_style, expected_style);
    }

    #[test]
    fn can_convert_to_dialoguer_console_style() {
        let style = Style {
            foreground: Some("red".to_string()),
            background: Some("green".to_string()),
            fg_bright: true,
            bg_bright: false,
        };

        let console_style = console::Style::from(style);

        let expected_style = console::Style::new()
            .fg(console::Color::Red)
            .bg(console::Color::Green)
            .bright()
            .for_stderr();
        assert_eq!(console_style, expected_style);
    }

    #[test]
    fn can_convert_to_dialoguer_console_style_with_none_values() {
        let style = Style {
            foreground: None,
            background: None,
            fg_bright: false,
            bg_bright: true,
        };

        let console_style = console::Style::from(style);

        let expected_style = console::Style::new().on_bright().for_stderr();
        assert_eq!(console_style, expected_style);
    }

    #[test]
    fn converting_to_dialoguer_style_sets_unknown_colors_to_defaults() {
        let style = Style {
            foreground: Some("not_a_color".to_string()),
            background: Some("green".to_string()),
            fg_bright: true,
            bg_bright: true,
        };

        let console_style = console::Style::from(style);

        let expected_style = console::Style::new()
            .bg(console::Color::Green)
            .bright()
            .on_bright()
            .for_stderr();
        assert_eq!(console_style, expected_style);
    }

    #[test]
    fn apply_to_formats_string_with_style() {
        let style = Style {
            foreground: Some("RED".to_string()),
            background: Some("green".to_string()),
            fg_bright: true,
            bg_bright: false,
        };

        let styled_str = style.apply_to("text");

        assert_eq!(styled_str, "\u{1b}[38;5;9m\u{1b}[42mtext\u{1b}[0m");
    }
}
