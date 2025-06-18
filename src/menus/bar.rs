use crate::menus::{Menu, MenuItem};

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
        }
    }

    /// Adds a menu to the menu bar (builder pattern).
    pub fn add_menu(&mut self, menu: Menu) -> &mut Self {
        self.menus.push(menu);
        self
    }
}
