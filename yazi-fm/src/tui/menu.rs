use crate::tui::theme::ChatTheme;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Widget},
};
use tachyonfx::{
    Duration, Effect,
    Interpolation::*,
    Motion, SimpleRng, color_from_hsl,
    fx::{self, ExpandDirection},
};

pub struct Menu {
    pub active_effect: (&'static str, Effect),
    pub last_tick: Duration,
    pub auto_cycle_timer: Duration,
    effects: EffectsRepository,
    theme: ChatTheme,
    rng: SimpleRng,
    pub selected_menu_item: usize,
    pub hovered_menu_item: Option<usize>, // Track hovered item
    pub menu_items: Vec<(String, String)>, // Changed to owned Strings
    pub scroll_offset: usize,
    pub menu_item_areas: Vec<Rect>, // Store clickable areas for each visible item
    pub menu_area: Rect, // Store the overall menu area
    pub current_submenu: Option<usize>, // Track which submenu we're in (None = main menu)
    main_menu: Vec<(&'static str, &'static str)>,
    submenus: Vec<Vec<(&'static str, &'static str)>>,
}

impl Menu {
    pub fn new(theme: ChatTheme) -> Self {
        let mut rng = SimpleRng::default();
        let effects = EffectsRepository::new(theme.clone(), &mut rng);
        let active_effect = effects.get_random_opening_effect(&mut rng);

        // Main menu - simple flat structure with reorganized categories
        let main_menu = vec![
            ("1. Providers", ""),
            ("2. Theme", ""),
            ("3. Keyboard Shortcuts", ""),
            ("4. Worktree", ""),
            ("5. Sandbox", ""),
            ("6. Model Configuration", ""),
            ("7. Approval Policy", ""),
            ("8. Web Search", ""),
            ("9. MCP Servers", ""),
            ("10. Profiles", ""),
            ("11. Feature Flags", ""),
            ("12. Voice / Realtime", ""),
            ("13. Notifications", ""),
            ("14. Memory & History", ""),
            ("15. Shell Environment", ""),
            ("16. Multi-Agent", ""),
            ("17. Skills", ""),
            ("18. Execution Rules", ""),
            ("19. Authentication", ""),
            ("20. Developer Instructions", ""),
            ("21. Image & Vision", ""),
            ("22. Project Trust", ""),
            ("23. Plugins & Apps", ""),
            ("24. Session Resume", ""),
            ("25. Network & Proxy", ""),
            ("26. Hooks & Events", ""),
        ];

        // Submenus for each category (indexed by main menu position)
        let submenus = vec![
            // 1. Providers submenu (index 0)
            vec![
                ("1. OpenAI Provider", ""),
                ("2. Anthropic Provider", ""),
                ("3. Local LLM Provider", ""),
                ("4. Custom Provider", ""),
                ("5. Provider Priority", ""),
                ("6. API Key Management", ""),
                ("7. Model Selection", ""),
                ("8. Token Limits", ""),
                ("9. Rate Limiting", ""),
            ],
            // 2. Theme submenu (index 1)
            vec![
                ("1. Theme Selector", ""),
                ("2. Dark Themes", ""),
                ("3. Light Themes", ""),
                ("4. Custom Theme", ""),
                ("5. Syntax Highlighting", ""),
                ("6. UI Colors", ""),
                ("7. Font Settings", ""),
                ("8. Icon Theme", ""),
                ("9. Transparency", ""),
            ],
            // 3. Keyboard Shortcuts submenu (index 2)
            vec![
                ("1. View Shortcuts", ""),
                ("2. Edit Shortcuts", ""),
                ("3. Reset Shortcuts", ""),
                ("4. Import Keybindings", ""),
                ("5. Export Keybindings", ""),
                ("6. Vim Mode", ""),
                ("7. Emacs Mode", ""),
                ("8. Shortcut Conflicts", ""),
            ],
            // 4. Worktree submenu (index 3)
            vec![
                ("1. Worktree Manager", ""),
                ("2. Create Worktree", ""),
                ("3. Switch Worktree", ""),
                ("4. Remove Worktree", ""),
                ("5. Worktree Status", ""),
                ("6. Branch Management", ""),
                ("7. Worktree Settings", ""),
            ],
            // 5. Sandbox submenu (index 4)
            vec![
                ("1. Sandbox Environment", ""),
                ("2. Container Settings", ""),
                ("3. Resource Limits", ""),
                ("4. Network Access", ""),
                ("5. File System Access", ""),
                ("6. Security Policies", ""),
                ("7. Execution Timeout", ""),
                ("8. Language Runtimes", ""),
            ],
            // 6. Model Configuration submenu (index 5)
            vec![
                ("1. Default Model", ""),
                ("2. Reasoning Effort", ""),
                ("3. Model Personality", ""),
                ("4. Review Model", ""),
                ("5. Service Tier", ""),
                ("6. Model Catalog JSON", ""),
            ],
            // 7. Approval Policy submenu (index 6)
            vec![
                ("1. Policy Mode", ""),
                ("2. Untrusted Mode", ""),
                ("3. On-Request Mode", ""),
                ("4. Never Mode", ""),
                ("5. Granular Permissions", ""),
            ],
            // 8. Web Search submenu (index 7)
            vec![
                ("1. Search Mode", ""),
                ("2. Context Size", ""),
                ("3. Allowed Domains", ""),
                ("4. User Location", ""),
            ],
            // 9. MCP Servers submenu (index 8)
            vec![
                ("1. STDIO Servers", ""),
                ("2. HTTP Servers", ""),
                ("3. OAuth Credentials", ""),
                ("4. OAuth Callback Port", ""),
                ("5. Server Management", ""),
            ],
            // 10. Profiles submenu (index 9)
            vec![
                ("1. Active Profile", ""),
                ("2. Create Profile", ""),
                ("3. Edit Profile", ""),
                ("4. Delete Profile", ""),
                ("5. Profile Settings", ""),
            ],
            // 11. Feature Flags submenu (index 10)
            vec![
                ("1. Unified Exec", ""),
                ("2. Shell Snapshot", ""),
                ("3. Request Rule", ""),
                ("4. Undo Support", ""),
                ("5. Search Tool", ""),
                ("6. Git Commit", ""),
                ("7. Runtime Metrics", ""),
                ("8. SQLite State", ""),
                ("9. Child Agents MD", ""),
                ("10. Image Detail Original", ""),
                ("11. Request Compression", ""),
                ("12. Collaboration", ""),
                ("13. Spawn CSV", ""),
                ("14. Apps & Connectors", ""),
                ("15. Tool Suggest", ""),
                ("16. Plugins", ""),
                ("17. Image Generation", ""),
                ("18. MCP Dependency Install", ""),
                ("19. Env Var Prompt", ""),
                ("20. Steer Mode", ""),
                ("21. PowerShell UTF8", ""),
                ("22. Windows Sandbox", ""),
                ("23. Windows Sandbox Elevated", ""),
                ("24. JS REPL", ""),
                ("25. Auto Approval Agent", ""),
                ("26. Prevent Sleep", ""),
                ("27. Suppress Warnings", ""),
            ],
            // 12. Voice / Realtime submenu (index 11)
            vec![
                ("1. Microphone Device", ""),
                ("2. Speaker Device", ""),
                ("3. Audio Format", ""),
                ("4. Voice Pipeline", ""),
                ("5. TTS Voice", ""),
                ("6. Realtime Mode", ""),
            ],
            // 13. Notifications submenu (index 12)
            vec![
                ("1. Notify Script", ""),
                ("2. TUI Notifications", ""),
                ("3. Turn Completion", ""),
            ],
            // 14. Memory & History submenu (index 13)
            vec![
                ("1. Memories Path", ""),
                ("2. Tool Output Budget", ""),
                ("3. Session Persistence", ""),
                ("4. Clear Memories", ""),
            ],
            // 15. Shell Environment submenu (index 14)
            vec![
                ("1. Policy Type", ""),
                ("2. Exclude List", ""),
                ("3. Include Only", ""),
            ],
            // 16. Multi-Agent submenu (index 15)
            vec![
                ("1. Max Threads", ""),
                ("2. Max Depth", ""),
                ("3. Job Max Runtime", ""),
                ("4. Role Definitions", ""),
            ],
            // 17. Skills submenu (index 16)
            vec![
                ("1. Per-Skill Toggle", ""),
                ("2. Skill Path", ""),
                ("3. Scan Directories", ""),
            ],
            // 18. Execution Rules submenu (index 17)
            vec![
                ("1. Prefix Rules", ""),
                ("2. Justification", ""),
                ("3. Rule Management", ""),
            ],
            // 19. Authentication submenu (index 18)
            vec![
                ("1. Credential Store", ""),
                ("2. Auth File Path", ""),
            ],
            // 20. Developer Instructions submenu (index 19)
            vec![
                ("1. Inline Instructions", ""),
                ("2. Instructions File", ""),
                ("3. Project Instructions", ""),
            ],
            // 21. Image & Vision submenu (index 20)
            vec![
                ("1. View Image Tool", ""),
                ("2. Image Generation", ""),
                ("3. Image Detail Original", ""),
            ],
            // 22. Project Trust submenu (index 21)
            vec![
                ("1. Root Markers", ""),
                ("2. Trust Mode", ""),
            ],
            // 23. Plugins & Apps submenu (index 22)
            vec![
                ("1. Plugin Management", ""),
                ("2. Marketplace Discovery", ""),
                ("3. Connector Apps", ""),
                ("4. Suggestion Allowlist", ""),
            ],
            // 24. Session Resume submenu (index 23)
            vec![
                ("1. Resume Last", ""),
                ("2. Resume All", ""),
                ("3. Fork Session", ""),
            ],
            // 25. Network & Proxy submenu (index 24)
            vec![
                ("1. OpenAI Base URL", ""),
                ("2. Sandbox Network", ""),
                ("3. Custom Providers", ""),
            ],
            // 26. Hooks & Events submenu (index 25)
            vec![
                ("1. User Prompt Submit", ""),
                ("2. Notify Hook", ""),
                ("3. Request Permissions", ""),
            ],
        ];

        Self {
            active_effect,
            last_tick: Duration::ZERO,
            auto_cycle_timer: Duration::ZERO,
            effects,
            theme,
            rng,
            selected_menu_item: 0, // Start at first item
            hovered_menu_item: None,
            menu_items: {
                // Just convert main menu to owned Strings
                main_menu.iter().map(|(a, b)| (a.to_string(), b.to_string())).collect()
            },
            scroll_offset: 0,
            menu_item_areas: Vec::new(),
            menu_area: Rect::default(),
            current_submenu: None,
            main_menu,
            submenus,
        }
    }

    pub fn update(&mut self, elapsed: std::time::Duration) {
        self.last_tick = Duration::from_millis(elapsed.as_millis() as u32);
        // Removed auto-cycling - animations only change on menu toggle
    }

    pub fn pick_opening_effect(&mut self) {
        // Regenerate effects with new random colors
        self.effects = EffectsRepository::new(self.theme.clone(), &mut self.rng);
        self.active_effect = self.effects.get_random_opening_effect(&mut self.rng);
        self.auto_cycle_timer = Duration::ZERO;
    }

    pub fn pick_closing_effect(&mut self) {
        // Regenerate effects with new random colors
        self.effects = EffectsRepository::new(self.theme.clone(), &mut self.rng);
        self.active_effect = self.effects.get_random_closing_effect(&mut self.rng);
        self.auto_cycle_timer = Duration::ZERO;
    }

    // Removed old effect methods - now using pick_opening_effect() and pick_closing_effect()

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

    pub fn enter_submenu(&mut self, index: usize) {
        if index < self.submenus.len() {
            self.current_submenu = Some(index);
            
            // Build submenu with "Back" at top, then items
            let mut submenu_items = vec![
                ("Back".to_string(), String::new()),
            ];
            submenu_items.extend(self.submenus[index].iter().map(|(a, b)| (a.to_string(), b.to_string())));
            self.menu_items = submenu_items;
            self.selected_menu_item = 0; // Start at "Back"
            self.scroll_offset = 0;
            self.hovered_menu_item = None;
        }
    }

    pub fn go_back_to_main(&mut self) {
        self.current_submenu = None;
        // Rebuild main menu
        self.menu_items = self.main_menu.iter().map(|(a, b)| (a.to_string(), b.to_string())).collect();
        self.selected_menu_item = 0; // Start at first item
        self.scroll_offset = 0;
        self.hovered_menu_item = None;
    }

    pub fn select_current_item(&mut self) -> bool {
        if self.current_submenu.is_none() {
            // In main menu - enter submenu
            self.enter_submenu(self.selected_menu_item);
            true
        } else {
            // In submenu
            if self.selected_menu_item == 0 {
                // "Back" item selected
                self.go_back_to_main();
                true
            } else {
                // Actual submenu item selected - return false to indicate action needed
                false
            }
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

    pub fn render_in_area(&mut self, area: Rect, buf: &mut Buffer) {
            // Use theme colors for consistency
            let content_bg = self.theme.card;

            // Create a centered content area
            let content_width = (area.width * 7 / 10).min(80);
            let content_height = (area.height * 75 / 100).min(32);

            let x_offset = (area.width - content_width) / 2;
            let y_offset = (area.height - content_height) / 2;

            let content_area = Rect {
                x: area.x + x_offset,
                y: area.y + y_offset,
                width: content_width,
                height: content_height,
            };

            // Store menu area for mouse detection
            self.menu_area = content_area;

            // Determine menu title based on context
            let menu_title = if let Some(submenu_idx) = self.current_submenu {
                // In submenu - show parent menu name and item count (excluding "Back")
                let parent_name = self.main_menu[submenu_idx].0.trim_start_matches(|c: char| c.is_numeric() || c == '.' || c.is_whitespace());
                let item_count = self.menu_items.len() - 1; // Exclude "Back" from count
                format!("{} ({} items)", parent_name, item_count)
            } else {
                // In main menu
                let item_count = self.menu_items.len();
                format!("Command Palette ({} items)", item_count)
            };

            Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .border_style(Style::default().fg(self.theme.border))
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title(Span::styled(
                    format!(" {} ", menu_title),
                    Style::default().fg(self.theme.accent).add_modifier(Modifier::BOLD)
                ))
                .style(Style::default().bg(content_bg))
                .render(content_area, buf);

            let padded_area = Rect {
                x: content_area.x + 2,
                y: content_area.y + 1,
                width: content_area.width.saturating_sub(4),
                height: content_area.height.saturating_sub(2),
            };

            let text_fg = self.theme.fg;
            let selected_bg = self.theme.accent;
            let selected_fg = self.theme.bg;
            let hover_bg = self.theme.primary;

            let visible_items = padded_area.height as usize; // Each item is 1 line

            if self.selected_menu_item < self.scroll_offset {
                self.scroll_offset = self.selected_menu_item;
            } else if self.selected_menu_item >= self.scroll_offset + visible_items {
                self.scroll_offset = self.selected_menu_item - visible_items + 1;
            }

            let mut lines = Vec::new();
            let end_idx = (self.scroll_offset + visible_items).min(self.menu_items.len());
            let exact_width = padded_area.width as usize;

            self.menu_item_areas.clear();
            let mut current_y = padded_area.y;

            for idx in self.scroll_offset..end_idx {
                let (title, _description) = &self.menu_items[idx];
                let is_selected = idx == self.selected_menu_item;
                let is_hovered = self.hovered_menu_item == Some(idx);

                self.menu_item_areas.push(Rect {
                    x: padded_area.x,
                    y: current_y,
                    width: padded_area.width,
                    height: 1, // Single line per item
                });
                current_y += 1;

                // Format the line
                let item_text = format!(" {}", title);
                let line_text = if item_text.len() > exact_width {
                    let truncate_at = exact_width.saturating_sub(3);
                    format!("{}...", &item_text[..truncate_at])
                } else {
                    let padding = exact_width.saturating_sub(item_text.len());
                    format!("{}{}", item_text, " ".repeat(padding))
                };

                let (fg, bg) = if is_selected {
                    (selected_fg, selected_bg)
                } else if is_hovered {
                    (self.theme.bg, hover_bg)
                } else {
                    (text_fg, content_bg)
                };

                let style = if is_selected {
                    Style::default().fg(fg).bg(bg).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(fg).bg(bg)
                };

                lines.push(Line::from(Span::styled(line_text, style)));
            }

            // Fill remaining lines with empty background to cover content behind
            let items_shown = end_idx - self.scroll_offset;
            for _ in items_shown..visible_items {
                let empty_line = " ".repeat(exact_width);
                lines.push(Line::from(Span::styled(empty_line, Style::default().bg(content_bg))));
            }

            let main_text = Text::from(lines);
            ratatui::widgets::Paragraph::new(main_text)
                .render(padded_area, buf);
        }

}

struct EffectsRepository {
    opening_effects: Vec<(&'static str, Effect)>,
    closing_effects: Vec<(&'static str, Effect)>,
}

impl EffectsRepository {
    fn new(theme: ChatTheme, rng: &mut SimpleRng) -> Self {
        // Use theme colors for effects
        let screen_bg = theme.bg;

        let slow = Duration::from_millis(2000); // Faster animations
        let medium = Duration::from_millis(1200); // Faster animations

        // Generate random colors using HSL
        let mut random_color = || {
            let hue = (rng.r#gen() % 360) as f32;
            color_from_hsl(hue, 70.0, 60.0)
        };

        let color1 = random_color();
        let color2 = random_color();
        let color3 = random_color();
        let color4 = random_color();

        // Opening effects - animations that reveal/show content (only highly visible ones)
        let opening_effects = vec![
            (
                "sweep in left to right",
                fx::sweep_in(Motion::LeftToRight, 30, 0, screen_bg, (slow, QuadOut)),
            ),
            (
                "sweep in right to left",
                fx::sweep_in(Motion::RightToLeft, 30, 0, screen_bg, (slow, QuadOut)),
            ),
            (
                "sweep in top to bottom",
                fx::sweep_in(Motion::DownToUp, 30, 0, screen_bg, (slow, QuadOut)),
            ),
            (
                "sweep in bottom to top",
                fx::sweep_in(Motion::UpToDown, 30, 0, screen_bg, (slow, QuadOut)),
            ),
            (
                "slide in from bottom",
                fx::slide_in(Motion::UpToDown, 20, 0, screen_bg, (medium, QuadOut)),
            ),
            (
                "slide in from top",
                fx::slide_in(Motion::DownToUp, 20, 0, screen_bg, (medium, QuadOut)),
            ),
            (
                "slide in from left",
                fx::slide_in(Motion::LeftToRight, 20, 0, screen_bg, (medium, QuadOut)),
            ),
            (
                "slide in from right",
                fx::slide_in(Motion::RightToLeft, 20, 0, screen_bg, (medium, QuadOut)),
            ),
            (
                "expand vertical",
                fx::expand(
                    ExpandDirection::Vertical,
                    Style::new().fg(color1).bg(screen_bg),
                    1200,
                ),
            ),
            (
                "expand horizontal",
                fx::expand(
                    ExpandDirection::Horizontal,
                    Style::new().fg(color2).bg(screen_bg),
                    1200,
                ),
            ),
            (
                "coalesce",
                fx::coalesce((medium, CubicOut)),
            ),
        ];

        // Closing effects - animations that hide/dismiss content (only highly visible ones)
        let closing_effects = vec![
            (
                "sweep out down to up",
                fx::sweep_out(Motion::DownToUp, 45, 0, color1, (slow, QuadOut)),
            ),
            (
                "sweep out up to down",
                fx::sweep_out(Motion::UpToDown, 45, 0, color2, (slow, QuadOut)),
            ),
            (
                "sweep out left to right",
                fx::sweep_out(Motion::LeftToRight, 45, 0, color3, (slow, QuadOut)),
            ),
            (
                "sweep out right to left",
                fx::sweep_out(Motion::RightToLeft, 45, 0, color4, (slow, QuadOut)),
            ),
            (
                "slide out to right",
                fx::slide_out(Motion::LeftToRight, 80, 0, screen_bg, (medium, QuadIn)),
            ),
            (
                "slide out to left",
                fx::slide_out(Motion::RightToLeft, 80, 0, screen_bg, (medium, QuadIn)),
            ),
            (
                "slide out to top",
                fx::slide_out(Motion::DownToUp, 80, 0, screen_bg, (medium, QuadIn)),
            ),
            (
                "slide out to bottom",
                fx::slide_out(Motion::UpToDown, 80, 0, screen_bg, (medium, QuadIn)),
            ),
            (
                "shrink vertical",
                fx::expand(
                    ExpandDirection::Vertical,
                    Style::new().fg(color2).bg(screen_bg),
                    1200,
                )
                .reversed(),
            ),
            (
                "shrink horizontal",
                fx::expand(
                    ExpandDirection::Horizontal,
                    Style::new().fg(color3).bg(screen_bg),
                    1200,
                )
                .reversed(),
            ),
        ];

        Self { opening_effects, closing_effects }
    }

    fn get_random_opening_effect(&self, rng: &mut SimpleRng) -> (&'static str, Effect) {
        let idx = (rng.r#gen() % self.opening_effects.len() as u32) as usize;
        self.opening_effects[idx].clone()
    }

    fn get_random_closing_effect(&self, rng: &mut SimpleRng) -> (&'static str, Effect) {
        let idx = (rng.r#gen() % self.closing_effects.len() as u32) as usize;
        self.closing_effects[idx].clone()
    }
}

impl Menu {
    /// Handle mouse events for the menu
    pub fn handle_mouse(&mut self, x: u16, y: u16, is_click: bool) -> bool {
        // Check if mouse is within menu area
        if x < self.menu_area.x || x >= self.menu_area.x + self.menu_area.width ||
           y < self.menu_area.y || y >= self.menu_area.y + self.menu_area.height {
            // Mouse outside menu - clear hover
            if self.hovered_menu_item.is_some() {
                self.hovered_menu_item = None;
                return true; // Return true to trigger re-render
            }
            return false;
        }

        // Check which menu item is being hovered/clicked
        for (visible_idx, area) in self.menu_item_areas.iter().enumerate() {
            if x >= area.x && x < area.x + area.width &&
               y >= area.y && y < area.y + area.height {
                let actual_idx = self.scroll_offset + visible_idx;
                
                if is_click {
                    // Click: select the item
                    self.selected_menu_item = actual_idx;
                    return true;
                } else {
                    // Hover: just highlight if different from current hover
                    if self.hovered_menu_item != Some(actual_idx) {
                        self.hovered_menu_item = Some(actual_idx);
                        return true; // Trigger re-render
                    }
                    return false; // No change, no need to re-render
                }
            }
        }

        // Mouse is in menu area but not over any item - clear hover
        if self.hovered_menu_item.is_some() {
            self.hovered_menu_item = None;
            return true;
        }
        false
    }
}
