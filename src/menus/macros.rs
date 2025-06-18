// Export macros for convenience
#[macro_export]
macro_rules! menu_bar {
    ($($menu:expr),* $(,)?) => {
        {
            let mut menu_bar = $crate::menus::MenuBar::new();
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
            let menu = $crate::menus::Menu::with_hotkey($title, $hotkey);
            $(
                let menu = menu.item($item);
            )*
            menu
        }
    };
}

#[macro_export]
macro_rules! item {
    (action: $text:expr, command: $command:expr $(, hotkey: $hotkey:expr)? $(, shortcut: $shortcut:expr)?) => {
        {
            let item = $crate::menus::MenuItem::action($text, $command);
            $(let item = item.with_hotkey($hotkey);)?
            $(let item = item.with_shortcut($shortcut);)?
            item
        }
    };
    (separator) => {
        $crate::menus::MenuItem::separator()
    };
    (submenu: $text:expr, items: [$($sub_item:expr),*] $(, hotkey: $hotkey:expr)?) => {
        {
            let submenu = $crate::menus::MenuItem::submenu($text, vec![$($sub_item),*]);
            $(let submenu = submenu.with_hotkey($hotkey);)?
            submenu
        }
    };
}

#[cfg(test)]
mod tests {
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
}
