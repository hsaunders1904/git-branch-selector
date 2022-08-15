use std::fmt;

use dialoguer::theme::Theme;

pub struct GlyphTheme {}

impl Theme for GlyphTheme {
    /// Formats a multi select prompt item.
    fn format_multi_select_prompt_item(
        &self,
        f: &mut dyn fmt::Write,
        text: &str,
        checked: bool,
        active: bool,
    ) -> fmt::Result {
        write!(
            f,
            "{} {}",
            match (checked, active) {
                (true, true) => "> ✓",
                (true, false) => "  ✓",
                (false, true) => "> ✗",
                (false, false) => "  ✗",
            },
            text
        )
    }
}
