use super::core::{Menu, MenuBar};
use super::items::{ActionItem, MenuItem, SeparatorItem, SubMenuItem};
use crate::command::Command;

// MenuBar builders
impl MenuBar {
    /// Creates a new empty menu bar.
    pub fn new() -> Self {
        Self {
            menus: Vec::new(),
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

    /// Adds a menu to the menu bar (builder pattern).
    pub fn add_menu(&mut self, menu: Menu) -> &mut Self {
        self.menus.push(menu);
        self
    }
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

// MenuItem builders
impl MenuItem {
    /// Creates a new action menu item.
    pub fn new_action<S: Into<String>, C: Into<Command>>(label: S, command: C) -> Self {
        Self::Action(ActionItem::new(label, command))
    }

    /// Creates a new action menu item with hotkey.
    pub fn action_with_hotkey<S: Into<String>, C: Into<Command>>(
        label: S,
        command: C,
        hotkey: char,
    ) -> Self {
        Self::Action(ActionItem::with_hotkey(label, command, hotkey))
    }

    /// Creates a new action menu item with all properties.
    pub fn action_with_all<S: Into<String>, C: Into<Command>>(
        label: S,
        command: C,
        hotkey: Option<char>,
        shortcut: Option<S>,
    ) -> Self {
        Self::Action(ActionItem::with_all(label, command, hotkey, shortcut))
    }

    /// Creates a new separator menu item.
    pub fn separator() -> Self {
        Self::Separator(SeparatorItem::new())
    }

    /// Creates a new submenu item.
    pub fn new_submenu<S: Into<String>>(label: S) -> Self {
        Self::SubMenu(SubMenuItem::new(label))
    }

    /// Creates a new submenu item with hotkey.
    pub fn submenu_with_hotkey<S: Into<String>>(label: S, hotkey: char) -> Self {
        Self::SubMenu(SubMenuItem::with_hotkey(label, hotkey))
    }

    /// Creates a new submenu item with all properties.
    pub fn submenu_with_items<S: Into<String>>(
        label: S,
        hotkey: Option<char>,
        items: Vec<MenuItem>,
    ) -> Self {
        Self::SubMenu(SubMenuItem::with_items(label, hotkey, items))
    }

    // Legacy compatibility methods
    /// Creates a new action menu item (legacy).
    pub fn action<S: Into<String>, C: Into<Command>>(text: S, command: C) -> Self {
        Self::new_action(text, command)
    }

    /// Creates a new submenu (legacy).
    pub fn submenu<S: Into<String>>(text: S, items: Vec<MenuItem>) -> Self {
        Self::SubMenu(SubMenuItem::with_items(text, None, items))
    }

    /// Sets the hotkey for this menu item (legacy builder pattern).
    pub fn with_hotkey(mut self, hotkey: char) -> Self {
        match &mut self {
            Self::Action(action) => action.hotkey = Some(hotkey),
            Self::SubMenu(submenu) => submenu.hotkey = Some(hotkey),
            Self::Separator(_) => {} // Separators don't have hotkeys
        }
        self
    }

    /// Sets the shortcut display for this menu item (legacy builder pattern).
    pub fn with_shortcut<S: Into<String>>(mut self, shortcut: S) -> Self {
        if let Self::Action(action) = &mut self {
            action.shortcut = Some(shortcut.into());
        }
        self
    }

    /// Sets the help context for the menu item (legacy builder pattern).
    pub fn with_help_context<S: Into<String>>(mut self, help_context: S) -> Self {
        match &mut self {
            Self::Action(action) => action.help_context = Some(help_context.into()),
            Self::SubMenu(submenu) => submenu.help_context = Some(help_context.into()),
            Self::Separator(_) => {} // Separators don't have help context
        }
        self
    }

    /// Sets the enabled state of the menu item (legacy builder pattern).
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        match &mut self {
            Self::Action(action) => action.enabled = enabled,
            Self::SubMenu(submenu) => submenu.enabled = enabled,
            Self::Separator(_) => {} // Separators don't have enabled state
        }
        self
    }
}

// ActionItem builders
impl ActionItem {
    /// Creates a new action item with the given label and command.
    pub fn new<S: Into<String>, C: Into<Command>>(label: S, command: C) -> Self {
        Self {
            label: label.into(),
            command: command.into(),
            enabled: true,
            hotkey: None,
            shortcut: None,
            help_context: None,
        }
    }

    /// Creates a new action item with label, command, and hotkey.
    pub fn with_hotkey<S: Into<String>, C: Into<Command>>(
        label: S,
        command: C,
        hotkey: char,
    ) -> Self {
        Self {
            label: label.into(),
            command: command.into(),
            enabled: true,
            hotkey: Some(hotkey),
            shortcut: None,
            help_context: None,
        }
    }

    /// Creates a new action item with all properties.
    pub fn with_all<S: Into<String>, C: Into<Command>>(
        label: S,
        command: C,
        hotkey: Option<char>,
        shortcut: Option<S>,
    ) -> Self {
        Self {
            label: label.into(),
            command: command.into(),
            enabled: true,
            hotkey,
            shortcut: shortcut.map(|s| s.into()),
            help_context: None,
        }
    }

    /// Sets the hotkey for this action item (builder pattern).
    pub fn hotkey(mut self, hotkey: char) -> Self {
        self.hotkey = Some(hotkey);
        self
    }

    /// Sets the keyboard shortcut for this action item (builder pattern).
    pub fn shortcut<S: Into<String>>(mut self, shortcut: S) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    /// Sets the help context for this action item (builder pattern).
    pub fn help_context<S: Into<String>>(mut self, help_context: S) -> Self {
        self.help_context = Some(help_context.into());
        self
    }

    /// Sets the enabled state for this action item (builder pattern).
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

// SeparatorItem builders (already implemented in items.rs)

// SubMenuItem builders
impl SubMenuItem {
    /// Creates a new submenu item with the given label.
    pub fn new<S: Into<String>>(label: S) -> Self {
        Self {
            label: label.into(),
            items: Vec::new(),
            enabled: true,
            hotkey: None,
            help_context: None,
            focused_item: None,
            is_open: false,
        }
    }

    /// Creates a new submenu item with label and hotkey.
    pub fn with_hotkey<S: Into<String>>(label: S, hotkey: char) -> Self {
        Self {
            label: label.into(),
            items: Vec::new(),
            enabled: true,
            hotkey: Some(hotkey),
            help_context: None,
            focused_item: None,
            is_open: false,
        }
    }

    /// Creates a new submenu item with title, hotkey, and items.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tui_vision::menus::{SubMenuItem, MenuItem};
    ///
    /// let submenu = SubMenuItem::with_items("Find", Some('F'), vec![
    ///     MenuItem::new_action("Find", "edit.find"),
    ///     MenuItem::new_action("Find Next", "edit.find_next"),
    /// ]);
    /// ```
    pub fn with_items<S: Into<String>>(
        label: S,
        hotkey: Option<char>,
        items: Vec<MenuItem>,
    ) -> Self {
        Self {
            label: label.into(),
            items,
            enabled: true,
            hotkey,
            help_context: None,
            focused_item: None,
            is_open: false,
        }
    }

    /// Sets the hotkey for this submenu item (builder pattern).
    pub fn hotkey(mut self, hotkey: char) -> Self {
        self.hotkey = Some(hotkey);
        self
    }

    /// Adds an item to this submenu (builder pattern).
    pub fn item(mut self, item: MenuItem) -> Self {
        self.items.push(item);
        self
    }

    /// Sets the help context for this submenu item (builder pattern).
    pub fn help_context<S: Into<String>>(mut self, help_context: S) -> Self {
        self.help_context = Some(help_context.into());
        self
    }

    /// Sets the enabled state for this submenu item (builder pattern).
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Add a menu item to this submenu.
    pub fn add_item(&mut self, item: MenuItem) {
        self.items.push(item);
    }
}
