use super::core::{Menu, MenuBar};
use super::items::MenuItem;

// MenuBar state management
impl MenuBar {
    /// Check if any menu is currently open.
    pub fn has_open_menu(&self) -> bool {
        self.opened_menu.is_some()
    }

    /// Get the currently opened menu.
    pub fn opened_menu(&self) -> Option<&Menu> {
        self.opened_menu.and_then(|index| self.menus.get(index))
    }

    /// Get the currently opened menu mutably.
    pub fn opened_menu_mut(&mut self) -> Option<&mut Menu> {
        self.opened_menu.and_then(|index| self.menus.get_mut(index))
    }

    /// Open a specific menu by index.
    pub fn open_menu(&mut self, index: usize) {
        if index < self.menus.len() {
            // Close the previously opened menu
            if let Some(current_index) = self.opened_menu {
                if let Some(menu) = self.menus.get_mut(current_index) {
                    menu.focused_item = None;
                    menu.close_all_submenus();
                }
            }

            // Open the new menu
            self.opened_menu = Some(index);
            if let Some(menu) = self.menus.get_mut(index) {
                // Platform-specific behavior: some focus first item, others don't
                menu.focused_item = None; // macOS/Electron style - can be configured
            }
        }
    }

    /// Close the currently opened menu.
    pub fn close_menu(&mut self) {
        if let Some(index) = self.opened_menu {
            if let Some(menu) = self.menus.get_mut(index) {
                menu.focused_item = None;
                menu.close_all_submenus();
            }
        }
        self.opened_menu = None;
    }

    /// Open the next menu.
    pub fn open_next_menu(&mut self) {
        if self.menus.is_empty() {
            return;
        }

        let next_index = if let Some(current) = self.opened_menu {
            (current + 1) % self.menus.len()
        } else {
            0
        };
        self.open_menu(next_index);
    }

    /// Open the previous menu.
    pub fn open_previous_menu(&mut self) {
        if self.menus.is_empty() {
            return;
        }

        let prev_index = if let Some(current) = self.opened_menu {
            if current == 0 {
                self.menus.len() - 1
            } else {
                current - 1
            }
        } else {
            self.menus.len() - 1
        };
        self.open_menu(prev_index);
    }
}

// Menu state management
impl Menu {
    /// Sets the enabled state of the menu.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Focus the next selectable item.
    pub fn focus_next_item(&mut self) {
        let next = if let Some(current) = self.focused_item {
            self.find_next_selectable_item(current)
        } else {
            self.find_first_selectable_item()
        };
        self.focused_item = next;
    }

    /// Focus the previous selectable item.
    pub fn focus_previous_item(&mut self) {
        let previous = if let Some(current) = self.focused_item {
            self.find_previous_selectable_item(current)
        } else {
            self.find_last_selectable_item()
        };
        self.focused_item = previous;
    }

    /// Get the currently focused menu item.
    pub fn get_focused_item(&self) -> Option<&MenuItem> {
        self.focused_item.and_then(|index| self.items.get(index))
    }

    /// Close all open submenus.
    pub fn close_all_submenus(&mut self) {
        for item in &mut self.items {
            if let MenuItem::SubMenu(submenu) = item {
                submenu.is_open = false;
                submenu.focused_item = None;
                // Recursively close nested submenus
                Self::close_submenus_recursive(&mut submenu.items);
            }
        }
    }

    /// Recursively close submenus in a list of menu items.
    fn close_submenus_recursive(items: &mut [MenuItem]) {
        for item in items {
            if let MenuItem::SubMenu(submenu) = item {
                submenu.is_open = false;
                submenu.focused_item = None;
                Self::close_submenus_recursive(&mut submenu.items);
            }
        }
    }

    /// Find the first selectable (non-separator) menu item.
    pub fn find_first_selectable_item(&self) -> Option<usize> {
        self.items
            .iter()
            .position(|item| matches!(item, MenuItem::Action(_) | MenuItem::SubMenu(_)))
    }

    /// Find the last selectable menu item.
    pub fn find_last_selectable_item(&self) -> Option<usize> {
        self.items
            .iter()
            .rposition(|item| matches!(item, MenuItem::Action(_) | MenuItem::SubMenu(_)))
    }

    /// Find the next selectable menu item after the given index.
    pub fn find_next_selectable_item(&self, current: usize) -> Option<usize> {
        self.items
            .iter()
            .skip(current + 1)
            .position(|item| matches!(item, MenuItem::Action(_) | MenuItem::SubMenu(_)))
            .map(|pos| pos + current + 1)
            .or_else(|| self.find_first_selectable_item())
    }

    /// Find the previous selectable menu item before the given index.
    pub fn find_previous_selectable_item(&self, current: usize) -> Option<usize> {
        self.items
            .iter()
            .take(current)
            .rposition(|item| matches!(item, MenuItem::Action(_) | MenuItem::SubMenu(_)))
            .or_else(|| self.find_last_selectable_item())
    }
}
