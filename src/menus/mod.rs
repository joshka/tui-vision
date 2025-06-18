pub use self::core::*;
pub use self::items::*;

mod builders;
mod core;
mod items;
mod state;

// Export macros for convenience
#[macro_export]
macro_rules! menu_bar {
    ($($menu:expr),* $(,)?) => {
        {
            let mut menu_bar = MenuBar::new();
            $(
                menu_bar.add_menu($menu);
            )*
            menu_bar
        }
    };
}

#[macro_export]
macro_rules! menu {
    ($title:expr, $hotkey:expr, $($item:expr),* $(,)?) => {
        {
            let mut menu = Menu::with_hotkey($title, $hotkey);
            $(
                menu.add_item($item);
            )*
            menu
        }
    };
}

#[macro_export]
macro_rules! item {
    (action: $text:expr, command: $command:expr $(, hotkey: $hotkey:expr)? $(, shortcut: $shortcut:expr)?) => {
        {
            let mut item = MenuItem::action($text, $command);
            $(item = item.with_hotkey($hotkey);)?
            $(item = item.with_shortcut($shortcut);)?
            item
        }
    };
    (separator) => {
        MenuItem::separator()
    };
    (submenu: $text:expr, items: [$($sub_item:expr),*] $(, hotkey: $hotkey:expr)?) => {
        {
            let mut submenu = MenuItem::submenu($text, vec![$($sub_item),*]);
            $(submenu = submenu.with_hotkey($hotkey);)?
            submenu
        }
    };
}

// Make macros available to users of this module
pub use {item, menu, menu_bar};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menu_bar() {
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

        assert_eq!(menu_bar.menus.len(), 3);
        assert_eq!(menu_bar.menus[0].title, "File");
        assert_eq!(menu_bar.menus[1].title, "Edit");
        assert_eq!(menu_bar.menus[2].title, "Help");
    }

    #[test]
    fn test_menu_hierarchy() {
        // Create a typical application menu bar using the shortest convenience methods
        let menu_bar = MenuBar::from_menus([
            // File menu - using action() and builder pattern
            (
                "File",
                Some('F'),
                vec![
                    MenuItem::action("New", "file.new").with_hotkey('N'),
                    MenuItem::action("Open", "file.open")
                        .with_hotkey('O')
                        .with_shortcut("Ctrl+O"),
                    MenuItem::separator(),
                    MenuItem::action("Save", "file.save")
                        .with_hotkey('S')
                        .with_shortcut("Ctrl+S"),
                    MenuItem::action("Save As...", "file.save_as").with_hotkey('A'),
                    MenuItem::separator(),
                    MenuItem::action("Exit", "file.exit")
                        .with_hotkey('x')
                        .with_shortcut("Alt+F4"),
                ],
            ),
            // Edit menu with submenu using shortest methods
            (
                "Edit",
                Some('E'),
                vec![
                    MenuItem::action("Undo", "edit.undo")
                        .with_hotkey('U')
                        .with_shortcut("Ctrl+Z"),
                    MenuItem::action("Redo", "edit.redo")
                        .with_hotkey('R')
                        .with_shortcut("Ctrl+Y"),
                    MenuItem::separator(),
                    MenuItem::action("Cut", "edit.cut")
                        .with_hotkey('t')
                        .with_shortcut("Ctrl+X"),
                    MenuItem::action("Copy", "edit.copy")
                        .with_hotkey('C')
                        .with_shortcut("Ctrl+C"),
                    MenuItem::action("Paste", "edit.paste")
                        .with_hotkey('P')
                        .with_shortcut("Ctrl+V"),
                    MenuItem::separator(),
                    MenuItem::submenu(
                        "Find",
                        vec![
                            MenuItem::action("Find", "edit.find")
                                .with_hotkey('F')
                                .with_shortcut("Ctrl+F"),
                            MenuItem::action("Find Next", "edit.find_next").with_hotkey('N'),
                            MenuItem::action("Replace", "edit.replace")
                                .with_hotkey('R')
                                .with_shortcut("Ctrl+H"),
                        ],
                    )
                    .with_hotkey('i'),
                ],
            ),
            // Help menu - demonstrating optional help context
            (
                "Help",
                Some('H'),
                vec![
                    MenuItem::action("Help Topics", "help.topics")
                        .with_hotkey('T')
                        .with_help_context("Display help documentation"),
                    MenuItem::separator(),
                    MenuItem::action("About", "help.about")
                        .with_hotkey('A')
                        .with_help_context("Show application information"),
                ],
            ),
        ]);

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
        let item = MenuItem::action("Test", "test.command")
            .with_help_context("Test help")
            .with_enabled(false);

        assert_eq!(item.text(), Some("Test"));
        assert!(!item.is_enabled());

        let separator = MenuItem::separator();
        assert_eq!(separator.text(), None);
        assert!(!separator.is_enabled());
    }

    #[test]
    fn test_hotkey_functionality() {
        // Test menu hotkeys
        let file_menu = Menu::with_hotkey("File", 'F');
        assert_eq!(file_menu.hotkey, Some('F'));

        let edit_menu = Menu::with_hotkey("Edit", 'E');
        assert_eq!(edit_menu.hotkey, Some('E'));

        // Test menu item hotkeys
        let new_item = MenuItem::action("New", "file.new").with_hotkey('N');
        if let MenuItem::Action(action) = new_item {
            assert_eq!(action.hotkey, Some('N'));
        } else {
            panic!("Expected Action menu item");
        }

        // Test submenu hotkeys
        let submenu = MenuItem::submenu(
            "Recent Files",
            vec![
                MenuItem::action("Document1.txt", "file.open_recent").with_hotkey('1'),
                MenuItem::action("Document2.txt", "file.open_recent").with_hotkey('2'),
            ],
        )
        .with_hotkey('R');

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
        let save_item = MenuItem::action("Save", "file.save")
            .with_hotkey('S')
            .with_shortcut("Ctrl+S");

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
        let lower_case = MenuItem::action("lowercase", "test.lower").with_hotkey('l');
        let upper_case = MenuItem::action("UPPERCASE", "test.upper").with_hotkey('L');

        if let MenuItem::Action(action) = lower_case {
            assert_eq!(action.hotkey, Some('l'));
        }

        if let MenuItem::Action(action) = upper_case {
            assert_eq!(action.hotkey, Some('L'));
        }

        // Test menu hotkeys with different cases
        let menu_lower = Menu::with_hotkey("test", 't');
        let menu_upper = Menu::with_hotkey("Test", 'T');

        assert_eq!(menu_lower.hotkey, Some('t'));
        assert_eq!(menu_upper.hotkey, Some('T'));
    }

    #[test]
    fn test_new_constructor_api() {
        // Test new constructor patterns
        let menu_bar = MenuBar::new();
        assert_eq!(menu_bar.menus.len(), 0);

        // Test fluent menu construction
        let file_menu = Menu::new("File")
            .hotkey('F')
            .item(MenuItem::new_action("New", "file.new"))
            .item(MenuItem::separator())
            .item(MenuItem::new_action("Exit", "file.exit"));

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
    fn test_trait_functionality() {
        // Test trait object usage
        let items: Vec<MenuItem> = vec![
            MenuItem::new_action("New", "file.new"),
            MenuItem::separator(),
            MenuItem::new_submenu("Recent"),
        ];

        assert_eq!(items[0].item_type(), MenuItemType::Action);
        assert_eq!(items[1].item_type(), MenuItemType::Separator);
        assert_eq!(items[2].item_type(), MenuItemType::SubMenu);

        assert_eq!(items[0].label(), Some("New"));
        assert_eq!(items[1].label(), None);
        assert_eq!(items[2].label(), Some("Recent"));

        assert!(items[0].is_selectable());
        assert!(!items[1].is_selectable());
        assert!(items[2].is_selectable());
    }
}
