use crate::command::Command;

// Forward declaration for MenuItem enum (to avoid circular dependency)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MenuItem {
    Action(ActionItem),
    Separator(SeparatorItem),
    SubMenu(SubMenuItem),
}

impl MenuItem {
    /// Gets the text of the menu item (if applicable).
    pub fn text(&self) -> Option<&str> {
        match self {
            Self::Action(action) => Some(&action.label),
            Self::SubMenu(submenu) => Some(&submenu.label),
            Self::Separator(_) => None,
        }
    }

    /// Checks if the menu item is enabled.
    pub fn is_enabled(&self) -> bool {
        match self {
            Self::Action(action) => action.enabled,
            Self::SubMenu(submenu) => submenu.enabled,
            Self::Separator(_) => false, // Separators are not selectable
        }
    }

    /// Returns the label of this menu item, if any.
    pub fn label(&self) -> Option<&str> {
        match self {
            Self::Action(action) => Some(&action.label),
            Self::SubMenu(submenu) => Some(&submenu.label),
            Self::Separator(_) => None,
        }
    }

    /// Returns the hotkey of this menu item, if any.
    pub fn hotkey(&self) -> Option<char> {
        match self {
            Self::Action(action) => action.hotkey,
            Self::SubMenu(submenu) => submenu.hotkey,
            Self::Separator(_) => None,
        }
    }

    /// Returns whether this menu item is selectable.
    pub fn is_selectable(&self) -> bool {
        !matches!(self, Self::Separator(_))
    }
}

/// An action menu item that executes a command when selected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionItem {
    /// Text label displayed to the user
    pub label: String,

    /// Command to execute when this item is selected
    pub command: Command,

    /// Whether the action is enabled
    pub enabled: bool,

    /// Optional hotkey for quick access
    pub hotkey: Option<char>,

    /// Optional keyboard shortcut display (e.g., "Ctrl+S")
    pub shortcut: Option<String>,

    /// Help context for additional information
    pub help_context: Option<String>,
}

/// A separator menu item for visual grouping.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeparatorItem;

/// A submenu containing other menu items.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubMenuItem {
    /// Text label displayed to the user
    pub label: String,

    /// Collection of menu items in the submenu
    pub items: Vec<MenuItem>,

    /// Whether the submenu is enabled
    pub enabled: bool,

    /// Optional hotkey for quick access
    pub hotkey: Option<char>,

    /// Help context for additional information
    pub help_context: Option<String>,

    /// Index of the currently focused item when this submenu is open
    pub focused_item: Option<usize>,

    /// Whether this submenu is currently open
    pub is_open: bool,
}

impl Default for SeparatorItem {
    fn default() -> Self {
        Self::new()
    }
}

impl SeparatorItem {
    /// Creates a new separator item.
    pub fn new() -> Self {
        Self
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
