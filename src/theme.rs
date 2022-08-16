use std::fmt;

use dialoguer::{
    console::{style, Style, StyledObject},
    theme::Theme,
};

pub struct GbsTheme {
    pub checked_item_prefix: StyledObject<String>,
    pub unchecked_item_prefix: StyledObject<String>,
    pub active_item_prefix: StyledObject<String>,
    pub inactive_item_prefix: StyledObject<String>,
    pub active_item_style: Style,
    pub inactive_item_style: Style,
}

impl Default for GbsTheme {
    fn default() -> GbsTheme {
        GbsTheme {
            checked_item_prefix: style("✓".to_string()).for_stderr().green(),
            unchecked_item_prefix: style("✗".to_string()).for_stderr().white(),
            active_item_style: Style::new().for_stderr().cyan().bright(),
            inactive_item_style: Style::new().for_stderr().white(),
            active_item_prefix: style(">".to_string()).for_stderr().white(),
            inactive_item_prefix: style(" ".to_string()).for_stderr(),
        }
    }
}

impl Theme for GbsTheme {
    /// Formats a multi select prompt item.
    fn format_multi_select_prompt_item(
        &self,
        f: &mut dyn fmt::Write,
        text: &str,
        checked: bool,
        active: bool,
    ) -> fmt::Result {
        let details = match (checked, active) {
            (true, true) => (
                &self.active_item_prefix,
                &self.checked_item_prefix,
                self.active_item_style.apply_to(text),
            ),
            (true, false) => (
                &self.inactive_item_prefix,
                &self.checked_item_prefix,
                self.inactive_item_style.apply_to(text),
            ),
            (false, true) => (
                &self.active_item_prefix,
                &self.unchecked_item_prefix,
                self.active_item_style.apply_to(text),
            ),
            (false, false) => (
                &self.inactive_item_prefix,
                &self.unchecked_item_prefix,
                self.inactive_item_style.apply_to(text),
            ),
        };

        write!(f, "{}{} {}", details.0, details.1, details.2)
    }
}
