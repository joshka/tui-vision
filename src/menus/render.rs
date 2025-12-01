use ratatui_core::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{StatefulWidget, Widget},
};
use ratatui_widgets::block::Block;
use unicode_width::UnicodeWidthStr;

use super::{Menu, MenuBar, MenuItem};

/// Clears an area in the buffer, properly handling CJK/wide character boundaries.
///
/// When a wide character (like CJK characters) spans 2 cells, and the clear area
/// boundary cuts through it, this function ensures both cells are cleared to
/// prevent display corruption.
fn clear_area_cjk_aware(area: Rect, buf: &mut Buffer) {
    let buf_area = buf.area();
    let buf_left = buf_area.left();
    let buf_right = buf_area.right();
    let buf_top = buf_area.top();
    let buf_bottom = buf_area.bottom();

    for y in area.top()..area.bottom() {
        if y < buf_top || y >= buf_bottom {
            continue;
        }

        // Check if the cell to the LEFT of our area contains a wide character
        // that extends into our area (we would be clearing its "continuation" cell)
        if area.left() > buf_left {
            let left_x = area.left() - 1;
            if left_x >= buf_left && left_x < buf_right {
                let symbol = buf[(left_x, y)].symbol().to_string();
                let width = symbol.width();
                // If the left cell has a wide character (width > 1), it extends into our area
                // We need to clear it to avoid leaving a "half character"
                if width > 1 {
                    buf[(left_x, y)].reset();
                }
            }
        }

        // Now clear all cells in the area for this row
        for x in area.left()..area.right() {
            if x >= buf_left && x < buf_right {
                buf[(x, y)].reset();
            }
        }
    }
}

/// Rendering implementation for the MenuBar widget.
///
/// This implementation renders the menu bar as a horizontal strip across the top
/// of the terminal, with menus displayed as clickable titles. When a menu is open,
/// it displays a dropdown with the menu items.
impl Widget for MenuBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        (&self).render(area, buf);
    }
}

impl Widget for &MenuBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // First render the dropdown if a menu is open (before the menu bar)
        if let Some(menu_index) = self.opened_menu {
            if let Some(menu) = self.menus.get(menu_index) {
                let dropdown_area = self.calculate_dropdown_area(area, menu_index);
                self.render_dropdown(menu, dropdown_area, buf);
            }
        }

        // Then render the menu bar on top
        self.render_menu_bar(area, buf);
    }
}

impl MenuBar {
    /// Clears an area in the buffer with the specified style.
    fn clear_area(&self, area: Rect, buf: &mut Buffer, style: Style) {
        for y in area.y..area.y + area.height {
            for x in area.x..area.x + area.width {
                buf.set_string(x, y, " ", style);
            }
        }
    }

    /// Renders the horizontal menu bar.
    ///
    /// Only affects the first line of the given area, leaving the rest of the buffer unchanged.
    fn render_menu_bar(&self, area: Rect, buf: &mut Buffer) {
        let menu_bar_style = self.theme.menu_bar;

        // Only clear and render the first line of the area
        let menu_bar_area = Rect {
            x: area.x,
            y: area.y,
            width: area.width,
            height: 1,
        };

        // Clear only the menu bar line
        self.clear_area(menu_bar_area, buf, menu_bar_style);

        let mut x_offset = area.x + 1; // Start with padding from left edge

        for (index, menu) in self.menus.iter().enumerate() {
            let is_open = self.opened_menu == Some(index);
            let menu_style = if is_open {
                self.theme.menu_bar_focused
            } else {
                self.theme.menu_bar
            };

            // Render menu title with padding
            let menu_text = format!(" {menu_title} ", menu_title = &menu.title);
            if x_offset + menu_text.len() as u16 <= area.x + area.width {
                buf.set_string(x_offset, area.y, &menu_text, menu_style);
                x_offset += menu_text.len() as u16;
            }

            // Add space between menus
            if index < self.menus.len() - 1 {
                buf.set_string(x_offset, area.y, " ", menu_bar_style);
                x_offset += 1;
            }
        }
    }

    /// Calculates the area for the dropdown menu.
    fn calculate_dropdown_area(&self, area: Rect, menu_index: usize) -> Rect {
        let mut x_offset = area.x + 1;

        // Calculate x position based on menu titles
        for (index, menu) in self.menus.iter().enumerate() {
            if index == menu_index {
                break;
            }
            x_offset += format!(" {menu_title} ", menu_title = &menu.title).len() as u16 + 1;
        }

        let menu = &self.menus[menu_index];

        // Calculate dropdown dimensions with proper width accounting for shortcuts and arrows
        let max_item_width = menu
            .items
            .iter()
            .map(|item| match item {
                MenuItem::Action(action) => {
                    let label_len = action.label.len();
                    let shortcut_len = action.shortcut.as_ref().map_or(0, |s| s.len() + 1);
                    label_len + shortcut_len
                }
                MenuItem::SubMenu(submenu) => {
                    submenu.label.len() + 2 // +2 for arrow and space
                }
                MenuItem::Separator(_) => 0, // Separators use full width
            })
            .max()
            .unwrap_or(10) as u16;

        let dropdown_width = (max_item_width + 4).min(40); // Padding + max width
        let dropdown_height = menu.items.len() as u16 + 2; // Items + borders

        Rect {
            x: x_offset,
            y: area.y + 1,
            width: dropdown_width,
            height: dropdown_height.min(area.height - 1),
        }
    }

    /// Calculates the area for a submenu dropdown.
    fn calculate_submenu_area(
        &self,
        parent_area: Rect,
        _parent_x: u16,
        parent_y: u16,
        submenu: &super::SubMenuItem,
    ) -> Rect {
        // Calculate submenu dimensions
        let max_item_width = submenu
            .items
            .iter()
            .map(|item| match item {
                MenuItem::Action(action) => {
                    let label_len = action.label.len();
                    let shortcut_len = action.shortcut.as_ref().map_or(0, |s| s.len() + 1);
                    label_len + shortcut_len
                }
                MenuItem::SubMenu(sub) => {
                    sub.label.len() + 2 // +2 for arrow and space
                }
                MenuItem::Separator(_) => 0,
            })
            .max()
            .unwrap_or(10) as u16;

        let submenu_width = (max_item_width + 4).min(30); // Padding + max width (smaller than main menu)
        let submenu_height = submenu.items.len() as u16 + 2; // Items + borders

        // Position submenu to the right of the parent menu item
        // Don't constrain by parent area height - let submenu extend as needed
        Rect {
            x: parent_area.x + parent_area.width,
            y: parent_y,
            width: submenu_width,
            height: submenu_height,
        }
    }

    /// Renders the dropdown menu.
    ///
    /// The dropdown renders on top of existing buffer content, clearing only its own area.
    /// This allows menus to overlay other content in the terminal.
    fn render_dropdown(&self, menu: &Menu, area: Rect, buf: &mut Buffer) {
        let dropdown_style = self.theme.dropdown;
        let border_style = self.theme.dropdown_border;

        // Use CJK-aware clear to handle wide character boundaries properly
        clear_area_cjk_aware(area, buf);

        // Use Block widget for the border
        let block = Block::bordered()
            .border_style(border_style)
            .style(dropdown_style);

        // Calculate content area before rendering the block
        let content_area = block.inner(area);
        block.render(area, buf);

        // Render menu items
        for (index, item) in menu.items.iter().enumerate() {
            if index as u16 >= content_area.height {
                break; // Don't render beyond dropdown height
            }

            let y = content_area.y + index as u16;
            let is_focused = menu.focused_item == Some(index);

            // Special handling for separators - they span the full dropdown width
            if matches!(item, MenuItem::Separator(_)) {
                let separator_style = self.theme.separator;

                // Render separator line across the full dropdown width
                buf.set_string(area.x, y, "├", separator_style);
                for x in area.x + 1..area.x + area.width - 1 {
                    buf.set_string(x, y, "─", separator_style);
                }
                buf.set_string(area.x + area.width - 1, y, "┤", separator_style);
            } else {
                self.render_menu_item(item, content_area.x, y, content_area.width, is_focused, buf);

                // If this is an open submenu, render its dropdown
                if let MenuItem::SubMenu(submenu) = item {
                    if submenu.is_open {
                        let submenu_area =
                            self.calculate_submenu_area(area, content_area.x, y, submenu);
                        self.render_submenu_dropdown(submenu, submenu_area, buf);
                    }
                }
            }
        }
    }

    /// Renders a submenu dropdown.
    fn render_submenu_dropdown(&self, submenu: &super::SubMenuItem, area: Rect, buf: &mut Buffer) {
        let dropdown_style = self.theme.dropdown;
        let border_style = self.theme.dropdown_border;

        // Use CJK-aware clear to handle wide character boundaries properly
        clear_area_cjk_aware(area, buf);

        // Use Block widget for the border
        let block = Block::bordered()
            .border_style(border_style)
            .style(dropdown_style);

        // Calculate content area before rendering the block
        let content_area = block.inner(area);
        block.render(area, buf);

        // Render submenu items
        for (index, item) in submenu.items.iter().enumerate() {
            if index as u16 >= content_area.height {
                break; // Don't render beyond submenu height
            }

            let y = content_area.y + index as u16;
            let is_focused = submenu.focused_item == Some(index);

            // Special handling for separators - they span the full submenu width
            if matches!(item, MenuItem::Separator(_)) {
                let separator_style = self.theme.separator;

                // Render separator line across the full submenu width
                buf.set_string(area.x, y, "├", separator_style);
                for x in area.x + 1..area.x + area.width - 1 {
                    buf.set_string(x, y, "─", separator_style);
                }
                buf.set_string(area.x + area.width - 1, y, "┤", separator_style);
            } else {
                self.render_menu_item(item, content_area.x, y, content_area.width, is_focused, buf);
            }
        }
    }

    /// Renders a single menu item.
    fn render_menu_item(
        &self,
        item: &MenuItem,
        x: u16,
        y: u16,
        width: u16,
        is_focused: bool,
        buf: &mut Buffer,
    ) {
        match item {
            MenuItem::Action(action) => {
                let base_style = if is_focused {
                    self.theme.item_focused
                } else if action.enabled {
                    self.theme.item
                } else {
                    self.theme.item_disabled
                };

                self.render_action_item(action, x, y, width, base_style, buf);
            }
            MenuItem::Separator(_) => {
                // Separators are handled separately in the main render loop
                // to allow them to span the full dropdown width
            }
            MenuItem::SubMenu(submenu) => {
                let base_style = if is_focused {
                    self.theme.item_focused
                } else if submenu.enabled {
                    self.theme.item
                } else {
                    self.theme.item_disabled
                };

                self.render_submenu_item(submenu, x, y, width, base_style, buf);
            }
        }
    }

    /// Renders an action menu item with proper hotkey underlining.
    fn render_action_item(
        &self,
        action: &super::ActionItem,
        x: u16,
        y: u16,
        width: u16,
        style: Style,
        buf: &mut Buffer,
    ) {
        let label = &action.label;
        let shortcut = &action.shortcut;

        // First, fill the entire line with the background color
        for i in 0..width {
            buf.set_string(x + i, y, " ", style);
        }

        // Render label with hotkey underlining
        let mut current_x = x;
        if let Some(hotkey) = action.hotkey {
            if let Some(pos) = label
                .to_lowercase()
                .find(&hotkey.to_lowercase().to_string())
            {
                // Render text before hotkey
                let before = &label[..pos];
                buf.set_string(current_x, y, before, style);
                current_x += before.len() as u16;

                // Render hotkey with underline
                let hotkey_char = &label[pos..pos + 1];
                let hotkey_style = style.add_modifier(Modifier::UNDERLINED);
                buf.set_string(current_x, y, hotkey_char, hotkey_style);
                current_x += 1;

                // Render text after hotkey
                let after = &label[pos + 1..];
                buf.set_string(current_x, y, after, style);
                current_x += after.len() as u16;
            } else {
                // Hotkey not found in label, render normally
                buf.set_string(current_x, y, label, style);
                current_x += label.len() as u16;
            }
        } else {
            // No hotkey, render normally
            buf.set_string(current_x, y, label, style);
            current_x += label.len() as u16;
        }

        // Render shortcut if available, positioned from the right
        if let Some(shortcut) = shortcut {
            let shortcut_x = x + width.saturating_sub(shortcut.len() as u16);
            if shortcut_x > current_x {
                buf.set_string(shortcut_x, y, shortcut, style);
            }
        }
    }

    /// Renders a submenu item with proper hotkey underlining.
    fn render_submenu_item(
        &self,
        submenu: &super::SubMenuItem,
        x: u16,
        y: u16,
        width: u16,
        style: Style,
        buf: &mut Buffer,
    ) {
        let label = &submenu.label;

        // First, fill the entire line with the background color
        for i in 0..width {
            buf.set_string(x + i, y, " ", style);
        }

        // Reserve space for arrow (positioned 1 cell from the right edge)
        let arrow = "►";
        let arrow_x = x + width.saturating_sub(2); // 2 positions from right edge

        // Render label with hotkey underlining
        let mut current_x = x;
        if let Some(hotkey) = submenu.hotkey {
            if let Some(pos) = label
                .to_lowercase()
                .find(&hotkey.to_lowercase().to_string())
            {
                // Render text before hotkey
                let before = &label[..pos];
                buf.set_string(current_x, y, before, style);
                current_x += before.len() as u16;

                // Render hotkey with underline
                let hotkey_char = &label[pos..pos + 1];
                let hotkey_style = style.add_modifier(Modifier::UNDERLINED);
                buf.set_string(current_x, y, hotkey_char, hotkey_style);
                current_x += 1;

                // Render text after hotkey
                let after = &label[pos + 1..];
                buf.set_string(current_x, y, after, style);
                current_x += after.len() as u16;
            } else {
                // Hotkey not found in label, render normally
                buf.set_string(current_x, y, label, style);
                current_x += label.len() as u16;
            }
        } else {
            // No hotkey, render normally
            buf.set_string(current_x, y, label, style);
            current_x += label.len() as u16;
        }

        // Render arrow indicator (positioned 1 cell from the right edge)
        if arrow_x > current_x {
            buf.set_string(arrow_x, y, arrow, style);
        }
    }
}

/// Stateful widget implementation for cases where you need to pass state.
impl StatefulWidget for MenuBar {
    type State = ();

    fn render(self, area: Rect, buf: &mut Buffer, _state: &mut Self::State) {
        Widget::render(self, area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{item, menu, menu_bar};
    use ratatui::style::{Color, Style};
    use ratatui_core::buffer::Buffer;
    use ratatui_core::layout::Rect;

    #[test]
    fn empty_menu_bar_rendering() {
        let menu_bar = MenuBar::new();
        let area = Rect::new(0, 0, 20, 1);
        let mut buffer = Buffer::empty(area);

        let menu_bar_style = menu_bar.theme.menu_bar;
        Widget::render(menu_bar, area, &mut buffer);

        // An empty menu bar should render as all spaces with menu bar style
        let mut expected = Buffer::with_lines(["                    "]);
        for x in 0..20 {
            expected[(x, 0)].set_style(menu_bar_style);
        }
        assert_eq!(buffer, expected);
    }

    #[test]
    fn menu_bar_with_menus_rendering() {
        let menu_bar = menu_bar![menu!["File", 'F',], menu!["Edit", 'E',],];
        let area = Rect::new(0, 0, 15, 1);
        let mut buffer = Buffer::empty(area);

        let menu_bar_style = menu_bar.theme.menu_bar;
        Widget::render(menu_bar, area, &mut buffer);

        // Menu bar should render "  File   Edit  " (with proper spacing)
        let mut expected = Buffer::with_lines(["  File   Edit  "]);
        // Apply the menu bar style to the entire line
        for x in 0..15 {
            expected[(x, 0)].set_style(menu_bar_style);
        }
        assert_eq!(buffer, expected);
    }

    #[test]
    fn menu_bar_with_opened_dropdown() {
        let mut menu_bar = menu_bar![
            menu![
                "File",
                'F',
                item![action: "New", command: "file.new"],
                item![action: "Open", command: "file.open"],
                item![separator],
                item![action: "Save", command: "file.save"],
                item![action: "Exit", command: "file.exit"],
            ],
            menu!["Edit", 'E',],
        ];
        menu_bar.open_menu(0);

        // Use larger area to accommodate full dropdown
        let area = Rect::new(0, 0, 20, 8);
        let mut buffer = Buffer::empty(area);

        Widget::render(menu_bar, area, &mut buffer);

        // Create expected buffer with the actual rendered content
        let mut expected = Buffer::with_lines([
            "  File   Edit       ", // Menu bar (with padding)
            " ┌──────┐           ", // Dropdown top border
            " │New   │           ", // First menu item
            " │Open  │           ", // Second menu item
            " ├──────┤           ", // Separator spanning full width
            " │Save  │           ", // Third menu item
            " │Exit  │           ", // Fourth menu item
            " └──────┘           ", // Dropdown bottom border
        ]);

        // Reset styles on both buffers to focus on content only
        use ratatui_core::style::Style;
        buffer.set_style(area, Style::reset());
        expected.set_style(area, Style::reset());

        assert_eq!(buffer, expected);
    }

    #[test]
    fn menu_item_display() {
        let action_item = item![action: "New", command: "file.new"];
        let separator_item = item![separator];

        // Test action item display
        assert_eq!(action_item.label(), Some("New"));
        assert!(action_item.is_selectable());

        // Test separator item display
        assert_eq!(separator_item.label(), None);
        assert!(!separator_item.is_selectable());
    }

    #[test]
    fn dropdown_area_calculation() {
        let menu_bar = menu_bar![menu!["File", 'F',], menu!["Edit", 'E',],];

        let area = Rect::new(0, 0, 80, 25);
        let dropdown_area = menu_bar.calculate_dropdown_area(area, 1);

        // Edit menu should be positioned after "File" menu
        assert!(dropdown_area.x > 1);
        assert_eq!(dropdown_area.y, 1);
    }

    #[test]
    fn menu_width_calculation() {
        let short_menu = Menu::new("Hi");
        let long_menu = Menu::new("File Operations");

        assert!(short_menu.title.len() < long_menu.title.len());

        // Test that menu width affects dropdown positioning
        let menu_bar = menu_bar![Menu::new("Short"), Menu::new("VeryLongMenuTitle"),];

        let area = Rect::new(0, 0, 80, 25);
        let first_dropdown = menu_bar.calculate_dropdown_area(area, 0);
        let second_dropdown = menu_bar.calculate_dropdown_area(area, 1);

        // Second dropdown should be positioned after the first menu
        assert!(second_dropdown.x > first_dropdown.x);
    }

    #[test]
    fn clear_area_functionality() {
        let menu_bar = MenuBar::new();
        let area = Rect::new(0, 0, 5, 3);
        let mut buffer = Buffer::empty(area);
        let style = Style::default().bg(Color::Red).fg(Color::White);

        // Initially buffer should be empty
        let initial_content = buffer[(0, 0)].symbol();
        assert_eq!(initial_content, " ");

        // Clear the area with a specific style
        menu_bar.clear_area(area, &mut buffer, style);

        // Verify the area is cleared with spaces and proper style
        for y in 0..3 {
            for x in 0..5 {
                let cell = &buffer[(x, y)];
                assert_eq!(cell.symbol(), " ");
                assert_eq!(cell.bg, Color::Red);
                assert_eq!(cell.fg, Color::White);
            }
        }
    }

    #[test]
    fn menu_bar_only_affects_first_line() {
        use ratatui_core::style::{Color, Style};

        let menu_bar = menu_bar![menu!["File", 'F',], menu!["Edit", 'E',],];

        // Create a larger area with some existing content
        let area = Rect::new(0, 0, 20, 5);
        let mut buffer = Buffer::empty(area);

        // Fill the buffer with some existing content
        let existing_style = Style::default().bg(Color::Blue).fg(Color::Yellow);
        for y in 0..5 {
            for x in 0..20 {
                buffer.set_string(x, y, "X", existing_style);
            }
        }

        // Render the menu bar
        Widget::render(menu_bar, area, &mut buffer);

        // First line should be styled as menu bar
        for x in 0..20 {
            let cell = &buffer[(x, 0)];
            // Content should be menu bar content or spaces, not the original "X"
            assert_ne!(cell.symbol(), "X");
        }

        // Lines below should retain original content and style
        for y in 1..5 {
            for x in 0..20 {
                let cell = &buffer[(x, y)];
                assert_eq!(cell.symbol(), "X");
                assert_eq!(cell.bg, Color::Blue);
                assert_eq!(cell.fg, Color::Yellow);
            }
        }
    }

    #[test]
    fn dropdown_renders_on_top_of_content() {
        use ratatui_core::style::{Color, Style};

        let mut menu_bar = menu_bar![menu![
            "File",
            'F',
            item![action: "New", command: "file.new"],
            item![action: "Open", command: "file.open"],
        ],];
        menu_bar.open_menu(0);

        // Create area with existing content
        let area = Rect::new(0, 0, 15, 6);
        let mut buffer = Buffer::empty(area);

        // Fill with background content
        let bg_style = Style::default().bg(Color::Red).fg(Color::White);
        for y in 0..6 {
            for x in 0..15 {
                buffer.set_string(x, y, "Z", bg_style);
            }
        }

        // Render the menu bar
        Widget::render(menu_bar, area, &mut buffer);

        // First line should be menu bar (overwriting background)
        for x in 0..15 {
            let cell = &buffer[(x, 0)];
            assert_ne!(cell.symbol(), "Z");
        }

        // Dropdown area should be cleared and contain menu content
        // (approximately x=1-7, y=1-4 based on dropdown size)
        for y in 1..4 {
            for x in 1..8 {
                let cell = &buffer[(x, y)];
                // Should not be the background "Z" in dropdown area
                assert_ne!(cell.symbol(), "Z");
            }
        }

        // Areas outside dropdown should retain background content
        for x in 10..15 {
            for y in 1..6 {
                let cell = &buffer[(x, y)];
                assert_eq!(cell.symbol(), "Z");
                assert_eq!(cell.bg, Color::Red);
            }
        }
    }

    #[test]
    fn separator_styling_test() {
        use ratatui_core::buffer::Buffer;
        use ratatui_core::layout::Rect;
        use ratatui_core::style::Color;
        use ratatui_core::widgets::Widget;

        let mut menu_bar = menu_bar![menu![
            "Edit",
            'E',
            item![action: "Cut", command: "edit.cut"],
            item![separator],
            item![action: "Paste", command: "edit.paste"],
        ],];
        menu_bar.open_menu(0);

        let area = Rect::new(0, 0, 20, 8);
        let mut buffer = Buffer::empty(area);
        Widget::render(menu_bar, area, &mut buffer);

        // Check that the separator line exists and has proper styling
        // The separator should be on line 3 (0=menu bar, 1=border, 2=Cut, 3=separator)
        let separator_y = 3;

        // The separator now spans the full dropdown width, starting at dropdown's x position
        // Find the dropdown area first
        let mut dropdown_x = 0;
        let mut dropdown_width = 0;

        // Look for the dropdown border to determine its position and width
        for x in 0..20 {
            if buffer[(x, 1)].symbol() == "┌" {
                // Top-left corner of dropdown
                dropdown_x = x;
                break;
            }
        }

        // Find the width by looking for the top-right corner
        for x in dropdown_x + 1..20 {
            if buffer[(x, 1)].symbol() == "┐" {
                // Top-right corner
                dropdown_width = x - dropdown_x + 1;
                break;
            }
        }

        // Check the left connector of separator
        let left_cell = &buffer[(dropdown_x, separator_y)];
        assert_eq!(left_cell.symbol(), "├");
        assert_eq!(left_cell.bg, Color::Blue);
        assert_eq!(left_cell.fg, Color::White);

        // Check the horizontal line
        let middle_cell = &buffer[(dropdown_x + 1, separator_y)];
        assert_eq!(middle_cell.symbol(), "─");
        assert_eq!(middle_cell.bg, Color::Blue);
        assert_eq!(middle_cell.fg, Color::White);

        // Check the right connector
        let right_cell = &buffer[(dropdown_x + dropdown_width - 1, separator_y)];
        assert_eq!(right_cell.symbol(), "┤");
        assert_eq!(right_cell.bg, Color::Blue);
        assert_eq!(right_cell.fg, Color::White);
    }

    #[test]
    fn hotkey_underlining_test() {
        use ratatui_core::buffer::Buffer;
        use ratatui_core::layout::Rect;
        use ratatui_core::style::Modifier;
        use ratatui_core::widgets::Widget;

        let mut menu_bar = menu_bar![menu![
            "File",
            'F',
            item![action: "New File", command: "file.new", hotkey: 'N'],
            item![action: "Open", command: "file.open", hotkey: 'O'],
        ],];
        menu_bar.open_menu(0);

        let area = Rect::new(0, 0, 20, 6);
        let mut buffer = Buffer::empty(area);
        Widget::render(menu_bar, area, &mut buffer);

        // Check that hotkeys are underlined
        // "New File" should have 'N' underlined
        let mut found_underlined_n = false;
        for y in 0..6 {
            for x in 0..20 {
                let cell = &buffer[(x, y)];
                if cell.symbol() == "N" && cell.modifier.contains(Modifier::UNDERLINED) {
                    found_underlined_n = true;
                    break;
                }
            }
        }
        assert!(
            found_underlined_n,
            "Expected to find underlined 'N' in 'New File'"
        );

        // "Open" should have 'O' underlined
        let mut found_underlined_o = false;
        for y in 0..6 {
            for x in 0..20 {
                let cell = &buffer[(x, y)];
                if cell.symbol() == "O" && cell.modifier.contains(Modifier::UNDERLINED) {
                    found_underlined_o = true;
                    break;
                }
            }
        }
        assert!(
            found_underlined_o,
            "Expected to find underlined 'O' in 'Open'"
        );
    }

    #[test]
    fn selection_visibility_test() {
        use ratatui_core::buffer::Buffer;
        use ratatui_core::layout::Rect;
        use ratatui_core::style::Color;
        use ratatui_core::widgets::Widget;

        let mut menu_bar = menu_bar![menu![
            "File",
            'F',
            item![action: "New", command: "file.new"],
            item![action: "Open", command: "file.open"],
        ],];
        menu_bar.open_menu(0);

        // Focus the first item manually by setting focused_item
        if let Some(menu) = menu_bar.menus.get_mut(0) {
            menu.focused_item = Some(0);
        }

        let area = Rect::new(0, 0, 15, 6);
        let mut buffer = Buffer::empty(area);
        Widget::render(menu_bar, area, &mut buffer);

        // Check that the focused item has white background (selection highlighting)
        let mut found_white_bg = false;
        for y in 0..6 {
            for x in 0..15 {
                let cell = &buffer[(x, y)];
                if cell.bg == Color::White && cell.symbol() != " " {
                    found_white_bg = true;
                    break;
                }
            }
        }
        assert!(
            found_white_bg,
            "Expected to find white background for focused item"
        );

        // Check that non-focused areas have blue background
        let mut found_blue_bg = false;
        for y in 0..6 {
            for x in 0..15 {
                let cell = &buffer[(x, y)];
                if cell.bg == Color::Blue {
                    found_blue_bg = true;
                    break;
                }
            }
        }
        assert!(
            found_blue_bg,
            "Expected to find blue background for non-focused areas"
        );
    }

    #[test]
    fn submenu_rendering_test() {
        let mut menu_bar = menu_bar![menu![
            "View",
            'V',
            item![submenu: "Theme", items: [
                    item![action: "Light", command: "light", hotkey: 'L'],
                    item![action: "Dark", command: "dark", hotkey: 'D']
                ], hotkey: 'T']
        ]];

        // Open the View menu and the Theme submenu
        menu_bar.open_menu(0);
        if let Some(menu) = menu_bar.opened_menu_mut() {
            menu.focused_item = Some(0); // Focus the Theme submenu
            if let Some(MenuItem::SubMenu(submenu)) = menu.items.get_mut(0) {
                submenu.is_open = true;
                submenu.focused_item = Some(0); // Focus Light theme
            }
        }

        let mut buffer = Buffer::empty(Rect::new(0, 0, 40, 10));
        Widget::render(&menu_bar, Rect::new(0, 0, 40, 10), &mut buffer);

        // Check that the submenu items are rendered
        let content = buffer.content();
        let rendered = content.iter().map(|cell| cell.symbol()).collect::<String>();

        assert!(
            rendered.contains("Light"),
            "Should contain 'Light' theme option"
        );
        assert!(
            rendered.contains("Dark"),
            "Should contain 'Dark' theme option"
        );
    }
}
