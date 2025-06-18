use super::core::{MenuItemTrait, MenuItemType};
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

    /// Returns the type of this menu item.
    pub fn item_type(&self) -> MenuItemType {
        match self {
            Self::Action(_) => MenuItemType::Action,
            Self::Separator(_) => MenuItemType::Separator,
            Self::SubMenu(_) => MenuItemType::SubMenu,
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

impl MenuItemTrait for ActionItem {
    fn item_type(&self) -> MenuItemType {
        MenuItemType::Action
    }

    fn label(&self) -> Option<&str> {
        Some(&self.label)
    }

    fn hotkey(&self) -> Option<char> {
        self.hotkey
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn help_context(&self) -> Option<&str> {
        self.help_context.as_deref()
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

impl MenuItemTrait for SeparatorItem {
    fn item_type(&self) -> MenuItemType {
        MenuItemType::Separator
    }

    fn label(&self) -> Option<&str> {
        None
    }

    fn hotkey(&self) -> Option<char> {
        None
    }

    fn is_enabled(&self) -> bool {
        false // Separators are never enabled/selectable
    }

    fn is_selectable(&self) -> bool {
        false // Separators are never selectable
    }

    fn help_context(&self) -> Option<&str> {
        None
    }

    fn set_enabled(&mut self, _enabled: bool) {
        // Separators don't have enabled state
    }
}

impl MenuItemTrait for SubMenuItem {
    fn item_type(&self) -> MenuItemType {
        MenuItemType::SubMenu
    }

    fn label(&self) -> Option<&str> {
        Some(&self.label)
    }

    fn hotkey(&self) -> Option<char> {
        self.hotkey
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn help_context(&self) -> Option<&str> {
        self.help_context.as_deref()
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
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
