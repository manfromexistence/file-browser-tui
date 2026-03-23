// Menu navigation and selection logic
use crate::tui::theme::ChatTheme;
use super::menu_data::Menu;

impl Menu {
    pub fn enter_submenu(&mut self, index: usize) {
        // Special handling for Theme submenu (index 0)
        if index == 0 {
            let available_themes = ChatTheme::available_themes();
            
            let mut submenu_items = vec![
                ("Back".to_string(), String::new()),
                ("Toggle Light/Dark Mode".to_string(), "TOGGLE_MODE".to_string()),
            ];
            
            for (i, (name, title)) in available_themes.iter().enumerate() {
                submenu_items.push((format!("{}. {}", i + 1, title), name.clone()));
            }
            
            self.menu_items = submenu_items;
            self.current_submenu = Some(index);
            self.selected_menu_item = 0;
            self.scroll_offset = 0;
            self.hovered_menu_item = None;
        } 
        // Special handling for Keyboard Shortcuts submenu (index 1)
        else if index == 1 {
            let mut submenu_items = vec![
                ("Back".to_string(), String::new()),
                ("Toggle Recording Mode".to_string(), "TOGGLE_RECORDING".to_string()),
            ];
            
            submenu_items.extend(self.submenus[index].iter().map(|(a, b)| (a.to_string(), b.to_string())));
            
            self.menu_items = submenu_items;
            self.current_submenu = Some(index);
            self.selected_menu_item = 0;
            self.scroll_offset = 0;
            self.hovered_menu_item = None;
        }
        else if index < self.submenus.len() {
            self.current_submenu = Some(index);
            
            let mut submenu_items = vec![("Back".to_string(), String::new())];
            submenu_items.extend(self.submenus[index].iter().map(|(a, b)| (a.to_string(), b.to_string())));
            self.menu_items = submenu_items;
            self.selected_menu_item = 0;
            self.scroll_offset = 0;
            self.hovered_menu_item = None;
        }
    }

    pub fn go_back_to_main(&mut self) {
        self.current_submenu = None;
        self.menu_items = self.main_menu.iter().map(|(a, b)| (a.to_string(), b.to_string())).collect();
        self.selected_menu_item = 0;
        self.scroll_offset = 0;
        self.hovered_menu_item = None;
    }

    pub fn select_current_item(&mut self) -> bool {
        if self.current_submenu.is_none() {
            self.enter_submenu(self.selected_menu_item);
            true
        } else {
            if self.selected_menu_item == 0 {
                self.go_back_to_main();
                true
            } else {
                false
            }
        }
    }

    pub fn select_next_menu_item(&mut self) {
        self.selected_menu_item = (self.selected_menu_item + 1) % self.menu_items.len();
    }

    pub fn select_prev_menu_item(&mut self) {
        if self.selected_menu_item == 0 {
            self.selected_menu_item = self.menu_items.len() - 1;
        } else {
            self.selected_menu_item -= 1;
        }
    }

    pub fn page_down(&mut self, visible_items: usize) {
        self.selected_menu_item = (self.selected_menu_item + visible_items).min(self.menu_items.len() - 1);
    }

    pub fn page_up(&mut self, visible_items: usize) {
        self.selected_menu_item = self.selected_menu_item.saturating_sub(visible_items);
    }

    pub fn jump_to_top(&mut self) {
        self.selected_menu_item = 0;
    }

    pub fn jump_to_bottom(&mut self) {
        self.selected_menu_item = self.menu_items.len() - 1;
    }

    // Theme-related methods
    pub fn get_selected_theme_name(&self) -> Option<String> {
        if self.current_submenu == Some(0) && self.selected_menu_item > 0 {
            Some(self.menu_items[self.selected_menu_item].1.clone())
        } else {
            None
        }
    }

    pub fn get_highlighted_theme_name(&self) -> Option<String> {
        if self.current_submenu == Some(0) {
            let item = &self.menu_items[self.selected_menu_item];
            if !item.1.is_empty() && item.1 != "TOGGLE_MODE" {
                return Some(item.1.clone());
            }
        }
        None
    }

    pub fn get_hovered_theme_name(&self) -> Option<String> {
        if self.current_submenu == Some(0) {
            if let Some(hovered_idx) = self.hovered_menu_item {
                if hovered_idx < self.menu_items.len() {
                    let item = &self.menu_items[hovered_idx];
                    if !item.1.is_empty() && item.1 != "TOGGLE_MODE" {
                        return Some(item.1.clone());
                    }
                }
            }
        }
        None
    }

    pub fn is_toggle_mode_selected(&self) -> bool {
        if self.current_submenu == Some(0) && self.selected_menu_item < self.menu_items.len() {
            self.menu_items[self.selected_menu_item].1 == "TOGGLE_MODE"
        } else {
            false
        }
    }

    // Recording mode methods
    pub fn is_toggle_recording_selected(&self) -> bool {
        if self.current_submenu == Some(1) && self.selected_menu_item < self.menu_items.len() {
            self.menu_items[self.selected_menu_item].1 == "TOGGLE_RECORDING"
        } else {
            false
        }
    }

    pub fn toggle_recording_mode(&mut self) {
        self.recording_mode = !self.recording_mode;
    }
}
