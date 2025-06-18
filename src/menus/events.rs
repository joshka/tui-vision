use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::{MenuBar, MenuItem};

/// Result of handling a key event in the menu system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MenuEventResult {
    /// Event was not handled by the menu system.
    NotHandled,
    /// Event was handled but no specific action occurred.
    Handled,
    /// A menu was opened.
    MenuOpened { menu_index: usize },
    /// A menu was closed.
    MenuClosed,
    /// Navigation occurred within the menu.
    NavigationChanged,
    /// A menu item was selected.
    ItemSelected { command: String },
    /// A submenu was opened.
    SubmenuOpened { submenu_label: String },
    /// A submenu was closed.
    SubmenuClosed { submenu_label: String },
}

impl MenuBar {
    /// Handles a keyboard event for the menu system.
    ///
    /// Returns a `MenuEventResult` indicating what happened as a result of the key press.
    /// The caller can use this to update status messages, handle commands, etc.
    pub fn handle_key_event(&mut self, key: KeyEvent) -> MenuEventResult {
        match key.code {
            // Menu activation with Alt/Ctrl key combinations
            KeyCode::Char(c)
                if key.modifiers.contains(KeyModifiers::ALT)
                    || key.modifiers.contains(KeyModifiers::CONTROL) =>
            {
                self.handle_menu_hotkey(c)
            }

            // Arrow key navigation
            KeyCode::Left => self.handle_left_arrow(),
            KeyCode::Right => self.handle_right_arrow(),
            KeyCode::Down => self.handle_down_arrow(),
            KeyCode::Up => self.handle_up_arrow(),

            // Enter to select item or open submenu
            KeyCode::Enter => self.handle_enter(),

            // Escape to close menu
            KeyCode::Esc => self.handle_escape(),

            // Tab navigation
            KeyCode::Tab => self.handle_tab(key.modifiers.contains(KeyModifiers::SHIFT)),

            // Space bar to activate menu system
            KeyCode::Char(' ') => self.handle_space(),

            // Direct hotkey access (without modifiers)
            KeyCode::Char(c) => self.handle_item_hotkey(c),

            _ => MenuEventResult::NotHandled,
        }
    }

    fn handle_menu_hotkey(&mut self, hotkey: char) -> MenuEventResult {
        // Check for menu hotkeys (case insensitive)
        for (index, menu) in self.menus.iter().enumerate() {
            if let Some(menu_hotkey) = menu.hotkey {
                if menu_hotkey.to_ascii_lowercase() == hotkey.to_ascii_lowercase() {
                    self.open_menu(index);
                    return MenuEventResult::MenuOpened { menu_index: index };
                }
            }
        }
        MenuEventResult::NotHandled
    }

    fn handle_left_arrow(&mut self) -> MenuEventResult {
        if let Some(menu) = self.opened_menu_mut() {
            // Check if we're in a submenu and should close it
            if let Some(focused_index) = menu.focused_item {
                if let Some(item) = menu.items.get_mut(focused_index) {
                    if let MenuItem::SubMenu(submenu) = item {
                        if submenu.is_open {
                            // Close the submenu
                            submenu.is_open = false;
                            submenu.focused_item = None;
                            return MenuEventResult::SubmenuClosed {
                                submenu_label: submenu.label.clone(),
                            };
                        }
                    }
                }
            }
            // Move to previous menu
            self.open_previous_menu();
            MenuEventResult::NavigationChanged
        } else if self.has_open_menu() {
            self.open_previous_menu();
            MenuEventResult::NavigationChanged
        } else {
            MenuEventResult::NotHandled
        }
    }

    fn handle_right_arrow(&mut self) -> MenuEventResult {
        if let Some(menu) = self.opened_menu_mut() {
            // Check if current focused item is a submenu
            if let Some(focused_index) = menu.focused_item {
                if let Some(item) = menu.items.get_mut(focused_index) {
                    if let MenuItem::SubMenu(submenu) = item {
                        if !submenu.is_open {
                            // Open the submenu
                            submenu.is_open = true;
                            // Focus first selectable item in submenu
                            submenu.focused_item = submenu
                                .items
                                .iter()
                                .position(|item| !matches!(item, MenuItem::Separator(_)));

                            return MenuEventResult::SubmenuOpened {
                                submenu_label: submenu.label.clone(),
                            };
                        } else {
                            // Submenu is already open, move to next menu
                            self.open_next_menu();
                            return MenuEventResult::NavigationChanged;
                        }
                    }
                }
            }
            // Move to next menu
            self.open_next_menu();
            MenuEventResult::NavigationChanged
        } else if self.has_open_menu() {
            self.open_next_menu();
            MenuEventResult::NavigationChanged
        } else {
            MenuEventResult::NotHandled
        }
    }

    fn handle_down_arrow(&mut self) -> MenuEventResult {
        if let Some(menu) = self.opened_menu_mut() {
            // Check if we're in a submenu
            if let Some(focused_index) = menu.focused_item {
                if let Some(MenuItem::SubMenu(submenu)) = menu.items.get_mut(focused_index) {
                    if submenu.is_open {
                        // Navigate within submenu
                        if let Some(current) = submenu.focused_item {
                            // Find next selectable item in submenu
                            let next = submenu.items.iter().enumerate().skip(current + 1).find_map(
                                |(i, item)| {
                                    if !matches!(item, MenuItem::Separator(_)) {
                                        Some(i)
                                    } else {
                                        None
                                    }
                                },
                            );
                            submenu.focused_item = next.or(submenu.focused_item);
                        } else {
                            // Focus first item in submenu
                            submenu.focused_item = submenu
                                .items
                                .iter()
                                .position(|item| !matches!(item, MenuItem::Separator(_)));
                        }
                        return MenuEventResult::NavigationChanged;
                    }
                }
            }

            // Regular menu navigation
            menu.focus_next_item();
            MenuEventResult::NavigationChanged
        } else {
            MenuEventResult::NotHandled
        }
    }

    fn handle_up_arrow(&mut self) -> MenuEventResult {
        if let Some(menu) = self.opened_menu_mut() {
            // Check if we're in a submenu
            if let Some(focused_index) = menu.focused_item {
                if let Some(MenuItem::SubMenu(submenu)) = menu.items.get_mut(focused_index) {
                    if submenu.is_open {
                        // Navigate within submenu
                        if let Some(current) = submenu.focused_item {
                            if current > 0 {
                                // Find previous selectable item in submenu
                                let prev = submenu
                                    .items
                                    .iter()
                                    .enumerate()
                                    .take(current)
                                    .rev()
                                    .find_map(|(i, item)| {
                                        if !matches!(item, MenuItem::Separator(_)) {
                                            Some(i)
                                        } else {
                                            None
                                        }
                                    });
                                submenu.focused_item = prev.or(submenu.focused_item);
                            }
                        }
                        return MenuEventResult::NavigationChanged;
                    }
                }
            }

            // Regular menu navigation
            menu.focus_previous_item();
            MenuEventResult::NavigationChanged
        } else {
            MenuEventResult::NotHandled
        }
    }

    fn handle_enter(&mut self) -> MenuEventResult {
        if let Some(menu) = self.opened_menu_mut() {
            if let Some(focused_index) = menu.focused_item {
                if let Some(item) = menu.items.get_mut(focused_index) {
                    match item {
                        MenuItem::Action(action) => {
                            let command = action.command.to_string();
                            self.close_menu();
                            MenuEventResult::ItemSelected { command }
                        }
                        MenuItem::SubMenu(submenu) => {
                            // Toggle submenu open/close
                            submenu.is_open = !submenu.is_open;
                            if submenu.is_open {
                                // Find first selectable item in submenu
                                submenu.focused_item = submenu
                                    .items
                                    .iter()
                                    .position(|item| !matches!(item, MenuItem::Separator(_)));
                                MenuEventResult::SubmenuOpened {
                                    submenu_label: submenu.label.clone(),
                                }
                            } else {
                                submenu.focused_item = None;
                                MenuEventResult::SubmenuClosed {
                                    submenu_label: submenu.label.clone(),
                                }
                            }
                        }
                        MenuItem::Separator(_) => MenuEventResult::NotHandled,
                    }
                } else {
                    MenuEventResult::NotHandled
                }
            } else {
                MenuEventResult::NotHandled
            }
        } else {
            MenuEventResult::NotHandled
        }
    }

    fn handle_escape(&mut self) -> MenuEventResult {
        if self.has_open_menu() {
            self.close_menu();
            MenuEventResult::MenuClosed
        } else {
            MenuEventResult::NotHandled
        }
    }

    fn handle_tab(&mut self, shift_pressed: bool) -> MenuEventResult {
        if shift_pressed {
            self.open_previous_menu();
        } else {
            self.open_next_menu();
        }
        MenuEventResult::NavigationChanged
    }

    fn handle_space(&mut self) -> MenuEventResult {
        if !self.has_open_menu() {
            self.open_menu(0);
            MenuEventResult::MenuOpened { menu_index: 0 }
        } else {
            MenuEventResult::NotHandled
        }
    }

    fn handle_item_hotkey(&mut self, hotkey: char) -> MenuEventResult {
        if let Some(menu) = self.opened_menu_mut() {
            // Check for item hotkeys within open menu (case insensitive)
            if let Some(index) = find_item_by_hotkey(menu, hotkey) {
                menu.focused_item = Some(index);
                if let Some(item) = menu.get_focused_item() {
                    match item {
                        MenuItem::Action(action) => {
                            let command = action.command.to_string();
                            self.close_menu();
                            MenuEventResult::ItemSelected { command }
                        }
                        MenuItem::SubMenu(_) => MenuEventResult::NavigationChanged,
                        _ => MenuEventResult::NotHandled,
                    }
                } else {
                    MenuEventResult::NotHandled
                }
            } else {
                MenuEventResult::NotHandled
            }
        } else {
            // Check for menu hotkeys (case insensitive)
            for (index, menu) in self.menus.iter().enumerate() {
                if let Some(menu_hotkey) = menu.hotkey {
                    if menu_hotkey.to_ascii_lowercase() == hotkey.to_ascii_lowercase() {
                        self.open_menu(index);
                        return MenuEventResult::MenuOpened { menu_index: index };
                    }
                }
            }
            MenuEventResult::NotHandled
        }
    }
}

/// Finds a menu item by its hotkey character (case insensitive).
fn find_item_by_hotkey(menu: &super::Menu, hotkey: char) -> Option<usize> {
    menu.items.iter().position(|item| {
        if let Some(item_hotkey) = item.hotkey() {
            item_hotkey.to_ascii_lowercase() == hotkey.to_ascii_lowercase()
        } else {
            false
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{item, menu, menu_bar};

    fn create_test_menu_bar() -> MenuBar {
        menu_bar![
            menu![
                "File",
                'F',
                item![action: "New", command: "file.new", hotkey: 'N'],
                item![action: "Open", command: "file.open", hotkey: 'O'],
                item![submenu: "Export", items: [
                    item![action: "PDF", command: "file.export.pdf", hotkey: 'P'],
                    item![action: "HTML", command: "file.export.html", hotkey: 'H']
                ], hotkey: 'E'],
            ],
            menu![
                "Edit",
                'E',
                item![action: "Undo", command: "edit.undo", hotkey: 'U'],
                item![action: "Redo", command: "edit.redo", hotkey: 'R'],
            ]
        ]
    }

    #[test]
    fn menu_hotkey_with_alt_opens_menu() {
        let mut menu_bar = create_test_menu_bar();
        let key = KeyEvent::new(KeyCode::Char('f'), KeyModifiers::ALT);

        let result = menu_bar.handle_key_event(key);

        assert_eq!(result, MenuEventResult::MenuOpened { menu_index: 0 });
        assert!(menu_bar.has_open_menu());
        assert_eq!(menu_bar.opened_menu, Some(0));
    }

    #[test]
    fn item_hotkey_selects_action() {
        let mut menu_bar = create_test_menu_bar();
        menu_bar.open_menu(0); // Open File menu

        let key = KeyEvent::new(KeyCode::Char('n'), KeyModifiers::NONE);
        let result = menu_bar.handle_key_event(key);

        assert_eq!(
            result,
            MenuEventResult::ItemSelected {
                command: "file.new".to_string()
            }
        );
        assert!(!menu_bar.has_open_menu());
    }

    #[test]
    fn arrow_keys_navigate_menus() {
        let mut menu_bar = create_test_menu_bar();
        menu_bar.open_menu(0); // Open File menu

        let result = menu_bar.handle_key_event(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
        assert_eq!(result, MenuEventResult::NavigationChanged);
        assert_eq!(menu_bar.opened_menu, Some(1)); // Should move to Edit menu
    }

    #[test]
    fn escape_closes_menu() {
        let mut menu_bar = create_test_menu_bar();
        menu_bar.open_menu(0);

        let result = menu_bar.handle_key_event(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        assert_eq!(result, MenuEventResult::MenuClosed);
        assert!(!menu_bar.has_open_menu());
    }

    #[test]
    fn enter_opens_submenu() {
        let mut menu_bar = create_test_menu_bar();
        menu_bar.open_menu(0);

        // Focus the Export submenu (index 2)
        if let Some(menu) = menu_bar.opened_menu_mut() {
            menu.focused_item = Some(2);
        }

        let result = menu_bar.handle_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert_eq!(
            result,
            MenuEventResult::SubmenuOpened {
                submenu_label: "Export".to_string()
            }
        );

        // Check that submenu is open
        if let Some(menu) = menu_bar.opened_menu() {
            if let Some(MenuItem::SubMenu(submenu)) = menu.items.get(2) {
                assert!(submenu.is_open);
            } else {
                panic!("Expected submenu at index 2");
            }
        }
    }

    #[test]
    fn space_activates_menu_system() {
        let mut menu_bar = create_test_menu_bar();

        let result =
            menu_bar.handle_key_event(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE));
        assert_eq!(result, MenuEventResult::MenuOpened { menu_index: 0 });
        assert!(menu_bar.has_open_menu());
    }
}
