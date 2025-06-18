use crate::menus::{Menu, MenuItem, MenuTheme};

/// A menu bar widget that will be displayed at the top of the screen.
///
/// Only one menu can be open at a time. When a menu is opened, it displays its dropdown
/// and becomes the active menu for keyboard navigation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenuBar {
    /// Collection of menus in the menu bar
    pub menus: Vec<Menu>,

    /// Index of the currently opened menu (None means no menu is open)
    pub opened_menu: Option<usize>,

    /// Theme configuration for rendering
    pub theme: MenuTheme,
}

impl Default for MenuBar {
    fn default() -> Self {
        Self::new()
    }
}

// MenuBar builders
impl MenuBar {
    /// Creates a new empty menu bar.
    pub fn new() -> Self {
        Self {
            menus: Vec::new(),
            opened_menu: None,
            theme: MenuTheme::default(),
        }
    }

    /// Creates a new menu bar from menu definitions.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tui_vision::menus::{MenuBar, MenuItem};
    ///
    /// let menu_bar = MenuBar::from_menus([
    ///     ("File", Some('F'), vec![
    ///         MenuItem::new_action("New", "file.new"),
    ///         MenuItem::separator(),
    ///         MenuItem::new_action("Exit", "file.exit"),
    ///     ]),
    ///     ("Edit", Some('E'), vec![
    ///         MenuItem::new_action("Undo", "edit.undo"),
    ///         MenuItem::new_action("Redo", "edit.redo"),
    ///     ]),
    /// ]);
    /// ```
    pub fn from_menus<I, S>(menus: I) -> Self
    where
        I: IntoIterator<Item = (S, Option<char>, Vec<MenuItem>)>,
        S: Into<String>,
    {
        let menus = menus
            .into_iter()
            .map(|(title, hotkey, items)| Menu::with_items(title, hotkey, items))
            .collect();
        Self {
            menus,
            opened_menu: None,
            theme: MenuTheme::default(),
        }
    }

    /// Creates a new menu bar with the given menus.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tui_vision::menus::{MenuBar, Menu};
    ///
    /// let menu_bar = MenuBar::with_menus(vec![
    ///     Menu::new("File").hotkey('F'),
    ///     Menu::new("Edit").hotkey('E'),
    /// ]);
    /// ```
    pub fn with_menus(menus: Vec<Menu>) -> Self {
        Self {
            menus,
            opened_menu: None,
            theme: MenuTheme::default(),
        }
    }

    /// Adds a menu to the menu bar (builder pattern).
    pub fn add_menu(&mut self, menu: Menu) -> &mut Self {
        self.menus.push(menu);
        self
    }

    /// Sets the theme for the menu bar.
    pub fn set_theme(&mut self, theme: MenuTheme) -> &mut Self {
        self.theme = theme;
        self
    }

    /// Sets the theme for the menu bar (builder pattern).
    pub fn theme(mut self, theme: MenuTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Gets a reference to the current theme.
    pub fn get_theme(&self) -> &MenuTheme {
        &self.theme
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_switching_works() {
        let mut menu_bar = MenuBar::new();

        // Default theme should be classic
        assert_eq!(menu_bar.get_theme(), &MenuTheme::classic());

        // Switch to dark theme
        menu_bar.set_theme(MenuTheme::dark());
        assert_eq!(menu_bar.get_theme(), &MenuTheme::dark());

        // Switch to light theme
        menu_bar.set_theme(MenuTheme::light());
        assert_eq!(menu_bar.get_theme(), &MenuTheme::light());

        // Switch to terminal theme
        menu_bar.set_theme(MenuTheme::terminal());
        assert_eq!(menu_bar.get_theme(), &MenuTheme::terminal());
    }

    #[test]
    fn theme_builder_pattern_works() {
        let menu_bar = MenuBar::new().theme(MenuTheme::dark());
        assert_eq!(menu_bar.get_theme(), &MenuTheme::dark());
    }

    #[test]
    fn theme_switching_affects_rendering() {
        use ratatui_core::{buffer::Buffer, layout::Rect, widgets::Widget};

        let mut menu_bar = MenuBar::new();
        let area = Rect::new(0, 0, 20, 1);

        // Test with classic theme (default)
        let mut buffer1 = Buffer::empty(area);
        Widget::render(&menu_bar, area, &mut buffer1);
        let classic_bg = buffer1[(0, 0)].bg;

        // Switch to dark theme
        menu_bar.set_theme(MenuTheme::dark());
        let mut buffer2 = Buffer::empty(area);
        Widget::render(&menu_bar, area, &mut buffer2);
        let dark_bg = buffer2[(0, 0)].bg;

        // The background colors should be different
        assert_ne!(classic_bg, dark_bg);
    }
}
