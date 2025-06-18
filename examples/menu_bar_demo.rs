/// Example demonstrating the MenuBar widget rendering.
///
/// This example shows how to create and render a menu bar with dropdowns
/// using the tui-vision library. It includes comprehensive keyboard navigation.
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Layout},
    widgets::{Block, Borders, Paragraph},
};
use tui_vision::menus::{MenuBar, MenuEventResult};
use tui_vision::{item, menu, menu_bar};

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    result
}

fn run(terminal: &mut ratatui::DefaultTerminal) -> Result<()> {
    let mut menu_bar = create_demo_menu_bar();
    let mut status_message = String::from("Press Space, Alt+Menu, or menu hotkeys to get started!");

    loop {
        terminal.draw(|frame| {
            let area = frame.area();

            // Create a layout for the menu bar at the top
            let layout = Layout::vertical([
                Constraint::Length(1), // Menu bar height
                Constraint::Min(0),    // Rest of the screen
            ])
            .split(area);

            // Create help text with current menu state
            let help_text = create_help_text(&menu_bar, &status_message);

            // Render some content below
            let content = Paragraph::new(help_text).block(
                Block::new()
                    .borders(Borders::ALL)
                    .title("Menu Bar Demo - Keyboard Navigation"),
            );
            frame.render_widget(content, layout[1]);

            // Render the menu bar
            frame.render_widget(&menu_bar, area);
        })?;

        // Handle input
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => break,
                    _ => {
                        // Handle menu events using the library's event handling
                        let result = menu_bar.handle_key_event(key);
                        status_message = match result {
                            MenuEventResult::NotHandled => status_message,
                            MenuEventResult::Handled => "Event handled".to_string(),
                            MenuEventResult::MenuOpened { menu_index } => {
                                let menu_name = menu_bar
                                    .menus
                                    .get(menu_index)
                                    .map(|m| m.title.as_str())
                                    .unwrap_or("Unknown");
                                format!("{menu_name} menu opened")
                            }
                            MenuEventResult::MenuClosed => "Menu closed".to_string(),
                            MenuEventResult::NavigationChanged => {
                                if let Some(menu) = menu_bar.opened_menu() {
                                    if let Some(item) = menu.get_focused_item() {
                                        if let Some(label) = item.label() {
                                            format!("Focused: {label}")
                                        } else {
                                            "Navigation changed".to_string()
                                        }
                                    } else {
                                        format!("Menu: {}", menu.title)
                                    }
                                } else {
                                    "Navigation changed".to_string()
                                }
                            }
                            MenuEventResult::ItemSelected { command } => {
                                format!("Selected: {command}")
                            }
                            MenuEventResult::SubmenuOpened { submenu_label } => {
                                format!("Opened submenu: {submenu_label}")
                            }
                            MenuEventResult::SubmenuClosed { submenu_label } => {
                                format!("Closed submenu: {submenu_label}")
                            }
                        };
                    }
                }
            }
        }
    }

    Ok(())
}

fn create_demo_menu_bar() -> MenuBar {
    menu_bar![
        menu![
            "File",
            'F',
            item![action: "New", command: "file.new", hotkey: 'N', shortcut: "Ctrl+N"],
            item![action: "Open", command: "file.open", hotkey: 'O', shortcut: "Ctrl+O"],
            item![action: "Open Recent", command: "file.open_recent", hotkey: 'R'],
            item![separator],
            item![action: "Save", command: "file.save", hotkey: 'S', shortcut: "Ctrl+S"],
            item![action: "Save As...", command: "file.save_as", hotkey: 'A', shortcut: "Ctrl+Shift+S"],
            item![action: "Save All", command: "file.save_all", hotkey: 'L'],
            item![separator],
            item![submenu: "Export", items: [
                item![action: "Export as PDF", command: "file.export.pdf", hotkey: 'P'],
                item![action: "Export as HTML", command: "file.export.html", hotkey: 'H'],
                item![action: "Export as Text", command: "file.export.txt", hotkey: 'T']
            ], hotkey: 'E'],
            item![separator],
            item![action: "Print", command: "file.print", hotkey: 'P', shortcut: "Ctrl+P"],
            item![separator],
            item![action: "Exit", command: "file.exit", hotkey: 'X', shortcut: "Alt+F4"],
        ],
        menu![
            "Edit",
            'E',
            item![action: "Undo", command: "edit.undo", hotkey: 'U', shortcut: "Ctrl+Z"],
            item![action: "Redo", command: "edit.redo", hotkey: 'R', shortcut: "Ctrl+Y"],
            item![separator],
            item![action: "Cut", command: "edit.cut", hotkey: 'T', shortcut: "Ctrl+X"],
            item![action: "Copy", command: "edit.copy", hotkey: 'C', shortcut: "Ctrl+C"],
            item![action: "Paste", command: "edit.paste", hotkey: 'P', shortcut: "Ctrl+V"],
            item![action: "Paste Special", command: "edit.paste_special", hotkey: 'S'],
            item![separator],
            item![action: "Select All", command: "edit.select_all", hotkey: 'A', shortcut: "Ctrl+A"],
            item![action: "Find", command: "edit.find", hotkey: 'F', shortcut: "Ctrl+F"],
            item![action: "Replace", command: "edit.replace", hotkey: 'H', shortcut: "Ctrl+H"],
        ],
        menu![
            "View",
            'V',
            item![action: "Zoom In", command: "view.zoom_in", hotkey: 'I', shortcut: "Ctrl++"],
            item![action: "Zoom Out", command: "view.zoom_out", hotkey: 'O', shortcut: "Ctrl+-"],
            item![action: "Reset Zoom", command: "view.zoom_reset", hotkey: 'R', shortcut: "Ctrl+0"],
            item![separator],
            item![action: "Full Screen", command: "view.fullscreen", hotkey: 'F', shortcut: "F11"],
            item![action: "Toggle Sidebar", command: "view.toggle_sidebar", hotkey: 'S', shortcut: "Ctrl+B"],
            item![action: "Toggle Status Bar", command: "view.toggle_statusbar", hotkey: 'T'],
            item![separator],
            item![submenu: "Theme", items: [
                item![action: "Light Theme", command: "view.theme.light", hotkey: 'L'],
                item![action: "Dark Theme", command: "view.theme.dark", hotkey: 'D'],
                item![action: "Auto Theme", command: "view.theme.auto", hotkey: 'A']
            ], hotkey: 'H'],
        ],
        menu![
            "Tools",
            'T',
            item![action: "Preferences", command: "tools.preferences", hotkey: 'P', shortcut: "Ctrl+,"],
            item![action: "Keyboard Shortcuts", command: "tools.shortcuts", hotkey: 'K'],
            item![separator],
            item![action: "Command Palette", command: "tools.command_palette", hotkey: 'C', shortcut: "Ctrl+Shift+P"],
            item![action: "Developer Tools", command: "tools.devtools", hotkey: 'D', shortcut: "F12"],
        ],
        menu![
            "Help",
            'H',
            item![action: "Help Topics", command: "help.topics", hotkey: 'T', shortcut: "F1"],
            item![action: "Keyboard Shortcuts", command: "help.shortcuts", hotkey: 'K'],
            item![separator],
            item![action: "Check for Updates", command: "help.updates", hotkey: 'U'],
            item![action: "Report Issue", command: "help.report", hotkey: 'R'],
            item![separator],
            item![action: "About", command: "help.about", hotkey: 'A'],
        ],
    ]
}

/// Creates help text showing current menu state and available commands.
fn create_help_text(menu_bar: &MenuBar, status_message: &str) -> String {
    let mut help = String::new();
    help.push_str("Menu Bar Demo - Comprehensive Keyboard Navigation\n\n");

    help.push_str("Menu Access:\n");
    help.push_str("- Alt+F, Alt+E, Alt+V, Alt+T, Alt+H: Open specific menus\n");
    help.push_str("- F, E, V, T, H: Open menus by hotkey (when no menu is open)\n");
    help.push_str("- Space: Activate menu system (opens File menu)\n");
    help.push_str("- Tab / Shift+Tab: Navigate between menus\n\n");

    help.push_str("Menu Navigation:\n");
    help.push_str("- Left/Right arrows: Switch between menus\n");
    help.push_str("- Up/Down arrows: Navigate menu items\n");
    help.push_str("- Enter: Select focused item\n");
    help.push_str("- Escape: Close menu\n");
    help.push_str("- Letter keys: Select items by hotkey\n\n");

    help.push_str("Features to explore:\n");
    help.push_str("- Multiple submenus (File > Export, View > Theme)\n");
    help.push_str("- Rich set of menu items with shortcuts\n");
    help.push_str("- Separators for visual grouping\n");
    help.push_str("- Hotkey highlighting in menu items\n\n");

    help.push_str("General:\n");
    help.push_str("- Q: Quit application\n\n");

    if menu_bar.has_open_menu() {
        if let Some(menu) = menu_bar.opened_menu() {
            help.push_str(&format!(
                "Current Menu: {} ({} items)\n",
                menu.title,
                menu.items.len()
            ));
            if let Some(item) = menu.get_focused_item() {
                if let Some(label) = item.label() {
                    help.push_str(&format!("Focused Item: {}\n", label));
                    if let Some(hotkey) = item.hotkey() {
                        help.push_str(&format!("Hotkey: {}\n", hotkey.to_uppercase()));
                    }
                }
            } else {
                help.push_str("No item focused (use Up/Down to focus)\n");
            }
        }
    } else {
        help.push_str("No menu open - try Alt+F or press F\n");
    }

    help.push_str(&format!("\nStatus: {}", status_message));

    help
}
