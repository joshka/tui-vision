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

/// A menu containing a title and a collection of menu items.
///
/// Menus are displayed as dropdowns when opened and support keyboard navigation
/// with hotkeys for quick access.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Menu {
    /// Title text displayed in the menu bar
    pub title: String,

    /// Collection of menu items in this menu
    pub items: Vec<super::items::MenuItem>,

    /// Whether the menu is enabled
    pub enabled: bool,

    /// Optional hotkey for quick access to this menu
    pub hotkey: Option<char>,

    /// Index of the currently focused item when this menu is open
    pub focused_item: Option<usize>,
}

/// The type of menu item for pattern matching and type checking.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuItemType {
    Action,
    Separator,
    SubMenu,
}

/// Common trait for all menu items, enabling polymorphic behavior.
pub trait MenuItemTrait: std::fmt::Debug + Clone {
    /// Returns the type of this menu item.
    fn item_type(&self) -> MenuItemType;

    /// Returns the label text of the menu item, if applicable.
    fn label(&self) -> Option<&str>;

    /// Returns the hotkey for the menu item, if any.
    fn hotkey(&self) -> Option<char>;

    /// Returns whether the menu item is enabled and selectable.
    fn is_enabled(&self) -> bool;

    /// Returns whether the menu item is selectable (not a separator).
    fn is_selectable(&self) -> bool {
        !matches!(self.item_type(), MenuItemType::Separator)
    }

    /// Returns help context for the menu item, if any.
    fn help_context(&self) -> Option<&str>;

    /// Sets the enabled state of the menu item.
    fn set_enabled(&mut self, enabled: bool);
}

/// Type alias for menu items using trait objects or concrete enum.
///
/// We provide both approaches - trait objects for maximum flexibility,
/// and an enum for convenience and pattern matching.
/// This is re-exported from the items module to avoid circular dependencies.
pub type MenuItemAlias = super::items::MenuItem;
