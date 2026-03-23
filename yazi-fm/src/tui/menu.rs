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
    pub menu_items: Vec<(&'static str, &'static str)>,
    pub scroll_offset: usize,
    pub menu_item_areas: Vec<Rect>, // Store clickable areas for each visible item
    pub menu_area: Rect, // Store the overall menu area
}

impl Menu {
    pub fn new(theme: ChatTheme) -> Self {
        let mut rng = SimpleRng::default();
        let effects = EffectsRepository::new(theme.clone(), &mut rng);
        let active_effect = effects.get_random_opening_effect(&mut rng);

        let menu_items = vec![
            ("AI CLI Agent", "Autonomous coding assistant"),
            ("Command Palette", "Quick access to all commands"),
            ("Editor Control Panel", "Manage editor settings"),
            ("Code Search", "Find across all files"),
            ("Terminal Integration", "Built-in shell access"),
            ("Auto-complete & Suggestions", "Intelligent code completion"),
            ("Git Integration", "Version control operations"),
            ("File Explorer", "Navigate project structure"),
            ("Debug Console", "Interactive debugging tools"),
            ("Task Runner", "Execute build and test scripts"),
            ("Extension Manager", "Install and manage plugins"),
            ("Workspace Settings", "Configure project preferences"),
            ("Keyboard Shortcuts", "Customize key bindings"),
            ("Snippet Manager", "Create and manage code snippets"),
            ("Refactoring Tools", "Automated code refactoring"),
            ("Code Formatter", "Format code with style guides"),
            ("Linter Integration", "Real-time code quality checks"),
            ("Test Runner", "Execute and manage unit tests"),
            ("Code Coverage", "View test coverage reports"),
            ("Performance Profiler", "Analyze code performance"),
            ("Memory Analyzer", "Debug memory usage"),
            ("Network Monitor", "Track API calls and requests"),
            ("Database Explorer", "Browse and query databases"),
            ("Docker Integration", "Manage containers and images"),
            ("Kubernetes Dashboard", "Monitor cluster resources"),
            ("CI/CD Pipeline", "Continuous integration tools"),
            ("Code Review", "Collaborative code reviews"),
            ("Issue Tracker", "Manage bugs and features"),
            ("Documentation Generator", "Auto-generate API docs"),
            ("Markdown Preview", "Live preview markdown files"),
            ("REST Client", "Test HTTP endpoints"),
            ("GraphQL Playground", "Interactive GraphQL IDE"),
            ("WebSocket Tester", "Test real-time connections"),
            ("SSH Manager", "Manage remote connections"),
            ("Environment Variables", "Configure env settings"),
            ("Secret Manager", "Secure credential storage"),
            ("API Key Vault", "Manage API keys safely"),
            ("Theme Customizer", "Personalize editor appearance"),
            ("Font Settings", "Configure editor fonts"),
        ];

        Self {
            active_effect,
            last_tick: Duration::ZERO,
            auto_cycle_timer: Duration::ZERO,
            effects,
            theme,
            rng,
            selected_menu_item: 0,
            hovered_menu_item: None,
            menu_items,
            scroll_offset: 0,
            menu_item_areas: Vec::new(),
            menu_area: Rect::default(),
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

            Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .border_style(Style::default().fg(self.theme.border))
                .border_type(ratatui::widgets::BorderType::Rounded)
                .style(Style::default().bg(content_bg))
                .render(content_area, buf);

            let padded_area = Rect {
                x: content_area.x + 2,
                y: content_area.y + 1,
                width: content_area.width.saturating_sub(2),
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
                let (title, description) = self.menu_items[idx];
                let is_selected = idx == self.selected_menu_item;
                let is_hovered = self.hovered_menu_item == Some(idx);

                self.menu_item_areas.push(Rect {
                    x: padded_area.x,
                    y: current_y,
                    width: padded_area.width,
                    height: 1, // Single line per item
                });
                current_y += 1;

                let line_text = format!(" {} — {}", title, description);
                let final_text = if line_text.len() > exact_width {
                    let truncate_at = exact_width.saturating_sub(3);
                    format!("{}...", &line_text[..truncate_at])
                } else {
                    let padding = exact_width - line_text.len();
                    format!("{}{}", line_text, " ".repeat(padding))
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

                lines.push(Line::from(Span::styled(final_text, style)));
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
