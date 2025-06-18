use crate::{
    command::Command,
    menus::{ActionItem, MenuItem, SeparatorItem, SubMenuItem},
};

/// A menu containing a title and a collection of menu items.
///
/// Menus are displayed as dropdowns when opened and support keyboard navigation
/// with hotkeys for quick access.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Menu {
    /// Title text displayed in the menu bar
    pub title: String,

    /// Collection of menu items in this menu
    pub items: Vec<super::MenuItem>,

    /// Whether the menu is enabled
    pub enabled: bool,

    /// Optional hotkey for quick access to this menu
    pub hotkey: Option<char>,

    /// Index of the currently focused item when this menu is open
    pub focused_item: Option<usize>,
}

// Menu builders
impl Menu {
    /// Creates a new menu with the given title.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            items: Vec::new(),
            enabled: true,
            hotkey: None,
            focused_item: None,
        }
    }

    /// Creates a new menu with a hotkey.
    pub fn with_hotkey(title: impl Into<String>, hotkey: char) -> Self {
        Self {
            title: title.into(),
            items: Vec::new(),
            enabled: true,
            hotkey: Some(hotkey),
            focused_item: None,
        }
    }

    /// Creates a new menu with title, hotkey, and items.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tui_vision::menus::{Menu, MenuItem};
    ///
    /// let menu = Menu::with_items("File", Some('F'), vec![
    ///     MenuItem::new_action("New", "file.new"),
    ///     MenuItem::separator(),
    /// ]);
    /// ```
    pub fn with_items<S: Into<String>>(
        title: S,
        hotkey: Option<char>,
        items: Vec<MenuItem>,
    ) -> Self {
        Self {
            title: title.into(),
            items,
            enabled: true,
            hotkey,
            focused_item: None,
        }
    }

    /// Sets the hotkey for this menu (builder pattern).
    pub fn hotkey(mut self, hotkey: char) -> Self {
        self.hotkey = Some(hotkey);
        self
    }

    /// Adds an item to this menu (builder pattern).
    pub fn item(mut self, item: MenuItem) -> Self {
        self.items.push(item);
        self
    }

    /// Adds multiple items to this menu (builder pattern).
    pub fn items(mut self, items: Vec<MenuItem>) -> Self {
        self.items.extend(items);
        self
    }

    /// Adds an action item as a convenience method.
    pub fn add_action<S: Into<String>, C: Into<Command>>(
        &mut self,
        label: S,
        command: C,
        hotkey: Option<char>,
    ) -> &mut Self {
        let action = ActionItem::new(label, command);
        let action = if let Some(hk) = hotkey {
            action.hotkey(hk)
        } else {
            action
        };
        self.items.push(MenuItem::Action(action));
        self
    }

    /// Adds a separator as a convenience method.
    pub fn add_separator(&mut self) -> &mut Self {
        self.items.push(MenuItem::Separator(SeparatorItem::new()));
        self
    }

    /// Adds a submenu as a convenience method.
    pub fn add_submenu<S: Into<String>>(&mut self, label: S, hotkey: Option<char>) -> &mut Self {
        self.items.push(MenuItem::SubMenu(SubMenuItem::with_items(
            label,
            hotkey,
            vec![],
        )));
        self
    }

    /// Adds a menu item to this menu.
    pub fn add_item(&mut self, item: MenuItem) {
        self.items.push(item);
    }
}
