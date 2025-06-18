pub use self::bar::*;
pub use self::events::*;
pub use self::item::*;
pub use self::menu::*;
pub use self::theme::*;

mod bar;
mod events;
mod item;
mod menu;
mod render;
mod state;
mod theme;

#[macro_use]
mod macros;

// Re-export macros
pub use crate::{item, menu, menu_bar};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_hierarchy() {
        // Create a typical application menu bar using macros
        let menu_bar = menu_bar![
            menu![
                "File",
                'F',
                item![action: "New", command: "file.new", hotkey: 'N'],
                item![action: "Open", command: "file.open", hotkey: 'O', shortcut: "Ctrl+O"],
                item![separator],
                item![action: "Save", command: "file.save", hotkey: 'S', shortcut: "Ctrl+S"],
                item![action: "Save As...", command: "file.save_as", hotkey: 'A'],
                item![separator],
                item![action: "Exit", command: "file.exit", hotkey: 'x', shortcut: "Alt+F4"],
            ],
            menu![
                "Edit",
                'E',
                item![action: "Undo", command: "edit.undo", hotkey: 'U', shortcut: "Ctrl+Z"],
                item![action: "Redo", command: "edit.redo", hotkey: 'R', shortcut: "Ctrl+Y"],
                item![separator],
                item![action: "Cut", command: "edit.cut", hotkey: 't', shortcut: "Ctrl+X"],
                item![action: "Copy", command: "edit.copy", hotkey: 'C', shortcut: "Ctrl+C"],
                item![action: "Paste", command: "edit.paste", hotkey: 'P', shortcut: "Ctrl+V"],
                item![separator],
                item![submenu: "Find", items: [
                    item![action: "Find", command: "edit.find", hotkey: 'F', shortcut: "Ctrl+F"],
                    item![action: "Find Next", command: "edit.find_next", hotkey: 'N'],
                    item![action: "Replace", command: "edit.replace", hotkey: 'R', shortcut: "Ctrl+H"]
                ], hotkey: 'i'],
            ],
            menu![
                "Help",
                'H',
                item![action: "Help Topics", command: "help.topics", hotkey: 'T'],
                item![separator],
                item![action: "About", command: "help.about", hotkey: 'A'],
            ],
        ];

        // Test the hierarchy
        assert_eq!(menu_bar.menus.len(), 3);
        assert_eq!(menu_bar.menus[0].title, "File");
        assert_eq!(menu_bar.menus[1].title, "Edit");
        assert_eq!(menu_bar.menus[2].title, "Help");

        // Test menu opening (create a mutable copy for testing state changes)
        let mut test_menu_bar = menu_bar.clone();
        test_menu_bar.open_menu(1);
        assert_eq!(test_menu_bar.opened_menu, Some(1));
        assert_eq!(test_menu_bar.opened_menu().unwrap().title, "Edit");

        // Test submenu structure
        let edit_menu = &menu_bar.menus[1];
        if let MenuItem::SubMenu(submenu) = &edit_menu.items[7] {
            assert_eq!(submenu.label, "Find");
            assert_eq!(submenu.items.len(), 3);
            if let MenuItem::Action(action) = &submenu.items[0] {
                assert_eq!(action.label, "Find");
                assert_eq!(action.command.as_str(), "edit.find");
            }
        }
    }

    #[test]
    fn test_menu_item_properties() {
        let item = item![action: "Test", command: "test.command"];

        assert_eq!(item.text(), Some("Test"));
        assert!(item.is_enabled());

        let separator = item![separator];
        assert_eq!(separator.text(), None);
        assert!(!separator.is_enabled());
    }

    #[test]
    fn test_hotkey_functionality() {
        // Test menu hotkeys
        let file_menu = menu!["File", 'F',];
        assert_eq!(file_menu.hotkey, Some('F'));

        let edit_menu = menu!["Edit", 'E',];
        assert_eq!(edit_menu.hotkey, Some('E'));

        // Test menu item hotkeys
        let new_item = item![action: "New", command: "file.new", hotkey: 'N'];
        if let MenuItem::Action(action) = new_item {
            assert_eq!(action.hotkey, Some('N'));
        } else {
            panic!("Expected Action menu item");
        }

        // Test submenu hotkeys
        let submenu = item![submenu: "Recent Files", items: [
            item![action: "Document1.txt", command: "file.open_recent", hotkey: '1'],
            item![action: "Document2.txt", command: "file.open_recent", hotkey: '2']
        ], hotkey: 'R'];

        if let MenuItem::SubMenu(submenu_data) = submenu {
            assert_eq!(submenu_data.hotkey, Some('R'));

            // Test hotkeys in submenu items
            if let MenuItem::Action(action) = &submenu_data.items[0] {
                assert_eq!(action.hotkey, Some('1'));
            }

            if let MenuItem::Action(action) = &submenu_data.items[1] {
                assert_eq!(action.hotkey, Some('2'));
            }
        } else {
            panic!("Expected SubMenu item");
        }

        // Test action item with both hotkey and shortcut
        let save_item =
            item![action: "Save", command: "file.save", hotkey: 'S', shortcut: "Ctrl+S"];

        if let MenuItem::Action(action) = save_item {
            assert_eq!(action.hotkey, Some('S'));
            assert_eq!(action.shortcut, Some("Ctrl+S".to_string()));
        } else {
            panic!("Expected Action menu item");
        }
    }

    #[test]
    fn test_hotkey_case_sensitivity() {
        // Test that hotkeys preserve case
        let lower_case = item![action: "lowercase", command: "test.lower", hotkey: 'l'];
        let upper_case = item![action: "UPPERCASE", command: "test.upper", hotkey: 'L'];

        if let MenuItem::Action(action) = lower_case {
            assert_eq!(action.hotkey, Some('l'));
        }

        if let MenuItem::Action(action) = upper_case {
            assert_eq!(action.hotkey, Some('L'));
        }

        // Test menu hotkeys with different cases
        let menu_lower = menu!["test", 't',];
        let menu_upper = menu!["Test", 'T',];

        assert_eq!(menu_lower.hotkey, Some('t'));
        assert_eq!(menu_upper.hotkey, Some('T'));
    }

    #[test]
    fn test_new_constructor_api() {
        // Test new constructor patterns
        let menu_bar = MenuBar::new();
        assert_eq!(menu_bar.menus.len(), 0);

        // Test fluent menu construction using macros and manual construction
        let file_menu = menu![
            "File",
            'F',
            item![action: "New", command: "file.new"],
            item![separator],
            item![action: "Exit", command: "file.exit"],
        ];

        assert_eq!(file_menu.title, "File");
        assert_eq!(file_menu.hotkey, Some('F'));
        assert_eq!(file_menu.items.len(), 3);

        // Test action item construction
        let action = ActionItem::new("Save", "file.save")
            .hotkey('S')
            .shortcut("Ctrl+S");

        assert_eq!(action.label, "Save");
        assert_eq!(action.hotkey, Some('S'));
        assert_eq!(action.shortcut, Some("Ctrl+S".to_string()));

        // Test submenu construction
        let submenu = SubMenuItem::new("Recent")
            .hotkey('R')
            .item(MenuItem::new_action("File1.txt", "file.open"));

        assert_eq!(submenu.label, "Recent");
        assert_eq!(submenu.hotkey, Some('R'));
        assert_eq!(submenu.items.len(), 1);
    }

    #[test]
    fn test_enum_pattern_matching() {
        // Test direct pattern matching usage (idiomatic Rust)
        let items: Vec<MenuItem> = vec![
            item![action: "New", command: "file.new"],
            item![separator],
            item![submenu: "Recent", items: []],
        ];

        // Test direct pattern matching (more idiomatic than item_type())
        assert!(matches!(items[0], MenuItem::Action(_)));
        assert!(matches!(items[1], MenuItem::Separator(_)));
        assert!(matches!(items[2], MenuItem::SubMenu(_)));

        assert_eq!(items[0].label(), Some("New"));
        assert_eq!(items[1].label(), None);
        assert_eq!(items[2].label(), Some("Recent"));

        assert!(items[0].is_selectable());
        assert!(!items[1].is_selectable());
        assert!(items[2].is_selectable());
    }
}
