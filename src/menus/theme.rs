use ratatui_core::style::{Color, Style};

/// Color scheme for menu rendering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenuTheme {
    /// Style for the menu bar background and text.
    pub menu_bar: Style,
    /// Style for a focused menu title in the menu bar.
    pub menu_bar_focused: Style,
    /// Style for dropdown background and borders.
    pub dropdown: Style,
    /// Style for dropdown borders specifically.
    pub dropdown_border: Style,
    /// Style for normal menu items.
    pub item: Style,
    /// Style for focused/selected menu items.
    pub item_focused: Style,
    /// Style for disabled menu items.
    pub item_disabled: Style,
    /// Style for separators.
    pub separator: Style,
    /// Style for submenu indicators (arrow).
    pub submenu_indicator: Style,
}

impl MenuTheme {
    /// Creates a classic MS-DOS edit.com style theme (cyan/blue).
    pub fn classic() -> Self {
        Self {
            menu_bar: Style::default().bg(Color::Cyan).fg(Color::Black),
            menu_bar_focused: Style::default().bg(Color::Blue).fg(Color::White),
            dropdown: Style::default().bg(Color::Blue).fg(Color::White),
            dropdown_border: Style::default().bg(Color::Blue).fg(Color::White),
            item: Style::default().bg(Color::Blue).fg(Color::White),
            item_focused: Style::default().bg(Color::White).fg(Color::Black),
            item_disabled: Style::default().bg(Color::Blue).fg(Color::DarkGray),
            separator: Style::default().bg(Color::Blue).fg(Color::White),
            submenu_indicator: Style::default().bg(Color::Blue).fg(Color::White),
        }
    }

    /// Creates a modern dark theme.
    pub fn dark() -> Self {
        Self {
            menu_bar: Style::default().bg(Color::Rgb(45, 45, 45)).fg(Color::White),
            menu_bar_focused: Style::default()
                .bg(Color::Rgb(70, 130, 180))
                .fg(Color::White),
            dropdown: Style::default().bg(Color::Rgb(60, 60, 60)).fg(Color::White),
            dropdown_border: Style::default().bg(Color::Rgb(60, 60, 60)).fg(Color::White),
            item: Style::default().bg(Color::Rgb(60, 60, 60)).fg(Color::White),
            item_focused: Style::default()
                .bg(Color::Rgb(70, 130, 180))
                .fg(Color::White),
            item_disabled: Style::default()
                .bg(Color::Rgb(60, 60, 60))
                .fg(Color::Rgb(120, 120, 120)),
            separator: Style::default().bg(Color::Rgb(60, 60, 60)).fg(Color::White),
            submenu_indicator: Style::default().bg(Color::Rgb(60, 60, 60)).fg(Color::White),
        }
    }

    /// Creates a light modern theme.
    pub fn light() -> Self {
        Self {
            menu_bar: Style::default()
                .bg(Color::Rgb(248, 248, 248))
                .fg(Color::Black),
            menu_bar_focused: Style::default()
                .bg(Color::Rgb(70, 130, 180))
                .fg(Color::White),
            dropdown: Style::default().bg(Color::White).fg(Color::Black),
            dropdown_border: Style::default()
                .bg(Color::Rgb(200, 200, 200))
                .fg(Color::Black),
            item: Style::default()
                .bg(Color::Rgb(200, 200, 200))
                .fg(Color::Black),
            item_focused: Style::default()
                .bg(Color::Rgb(70, 130, 180))
                .fg(Color::White),
            item_disabled: Style::default()
                .bg(Color::White)
                .fg(Color::Rgb(150, 150, 150)),
            separator: Style::default()
                .bg(Color::Rgb(200, 200, 200))
                .fg(Color::Black),
            submenu_indicator: Style::default().bg(Color::White).fg(Color::Black),
        }
    }

    /// Creates a vibrant terminal theme with bright colors.
    pub fn terminal() -> Self {
        Self {
            menu_bar: Style::default().bg(Color::Black).fg(Color::Green),
            menu_bar_focused: Style::default().bg(Color::Green).fg(Color::Black),
            dropdown: Style::default().bg(Color::Black).fg(Color::Green),
            dropdown_border: Style::default().bg(Color::Black).fg(Color::Green),
            item: Style::default().bg(Color::Black).fg(Color::Green),
            item_focused: Style::default().bg(Color::Green).fg(Color::Black),
            item_disabled: Style::default().bg(Color::Black).fg(Color::DarkGray),
            separator: Style::default().bg(Color::Black).fg(Color::Green),
            submenu_indicator: Style::default().bg(Color::Black).fg(Color::Green),
        }
    }
}

impl Default for MenuTheme {
    fn default() -> Self {
        Self::classic()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_presets_have_different_colors() {
        let classic = MenuTheme::classic();
        let dark = MenuTheme::dark();
        let light = MenuTheme::light();
        let terminal = MenuTheme::terminal();

        // Ensure themes are actually different
        assert_ne!(classic.menu_bar, dark.menu_bar);
        assert_ne!(classic.menu_bar, light.menu_bar);
        assert_ne!(classic.menu_bar, terminal.menu_bar);
        assert_ne!(dark.menu_bar, light.menu_bar);
    }

    #[test]
    fn default_theme_is_classic() {
        assert_eq!(MenuTheme::default(), MenuTheme::classic());
    }
}
