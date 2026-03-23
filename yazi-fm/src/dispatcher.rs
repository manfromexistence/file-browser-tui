use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::{KeyEvent, MouseEvent, KeyCode, KeyModifiers};
use tracing::warn;
use yazi_actor::Ctx;
use yazi_config::keymap::Key;
use yazi_macro::{act, emit, succ};
use yazi_shared::{data::Data, event::{ActionCow, Event, NEED_RENDER}};
use yazi_widgets::input::InputMode;

use crate::{Executor, Router, app::App};

// Helper function to format key events into readable shortcut strings
fn format_key_event(key: &KeyEvent) -> String {
    let mut parts = Vec::new();
    
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        parts.push("Ctrl");
    }
    if key.modifiers.contains(KeyModifiers::SHIFT) {
        parts.push("Shift");
    }
    if key.modifiers.contains(KeyModifiers::ALT) {
        parts.push("Alt");
    }
    
    let key_str = match key.code {
        KeyCode::Char(c) => c.to_uppercase().to_string(),
        KeyCode::F(n) => format!("F{}", n),
        KeyCode::Backspace => "Backspace".to_string(),
        KeyCode::Enter => "Enter".to_string(),
        KeyCode::Left => "Left".to_string(),
        KeyCode::Right => "Right".to_string(),
        KeyCode::Up => "Up".to_string(),
        KeyCode::Down => "Down".to_string(),
        KeyCode::Home => "Home".to_string(),
        KeyCode::End => "End".to_string(),
        KeyCode::PageUp => "PageUp".to_string(),
        KeyCode::PageDown => "PageDown".to_string(),
        KeyCode::Tab => "Tab".to_string(),
        KeyCode::BackTab => "BackTab".to_string(),
        KeyCode::Delete => "Delete".to_string(),
        KeyCode::Insert => "Insert".to_string(),
        KeyCode::Esc => "Esc".to_string(),
        _ => return "Unknown".to_string(),
    };
    
    parts.push(&key_str);
    parts.join("+")
}

pub(super) struct Dispatcher<'a> {
	app: &'a mut App,
}

impl<'a> Dispatcher<'a> {
	#[inline]
	pub(super) fn new(app: &'a mut App) -> Self { Self { app } }

	#[inline]
	pub(super) fn dispatch(&mut self, event: Event) -> Result<()> {
		let result = match event {
			Event::Call(action) => self.dispatch_call(action),
			Event::Seq(actions) => self.dispatch_seq(actions),
			Event::Render(partial) => self.dispatch_render(partial),
			Event::Key(key) => self.dispatch_key(key),
			Event::Mouse(mouse) => self.dispatch_mouse(mouse),
			Event::Resize => self.dispatch_resize(),
			Event::Focus => self.dispatch_focus(),
			Event::Paste(str) => self.dispatch_paste(str),
			Event::Timer => self.dispatch_timer(),
		};

		if let Err(err) = result {
			warn!("Event dispatch error: {err:?}");
		}
		Ok(())
	}

	#[inline]
	fn dispatch_call(&mut self, action: ActionCow) -> Result<Data> {
		Executor::new(self.app).execute(action)
	}

	#[inline]
	fn dispatch_seq(&mut self, mut actions: Vec<ActionCow>) -> Result<Data> {
		if let Some(last) = actions.pop() {
			self.dispatch_call(last)?;
		}
		if !actions.is_empty() {
			emit!(Seq(actions));
		}
		succ!();
	}

	#[inline]
	fn dispatch_render(&mut self, partial: bool) -> Result<Data> {
		if partial {
			_ = NEED_RENDER.compare_exchange(0, 2, Ordering::Relaxed, Ordering::Relaxed);
		} else {
			NEED_RENDER.store(1, Ordering::Relaxed);
		}
		succ!()
	}

	#[inline]
	fn dispatch_key(&mut self, key: KeyEvent) -> Result<Data> {
		use crossterm::event::KeyCode;
		use crate::tui::input::InputAction;
		
		// If in animation mode, handle navigation keys but allow typing
		if self.app.bridge.chat_state.animation_mode {
			let animations = crate::tui::AnimationType::all();
			
			// Handle navigation keys for animation carousel
			match key.code {
				KeyCode::Left => {
					// Previous animation
					if self.app.bridge.chat_state.current_animation_index == 0 {
						self.app.bridge.chat_state.current_animation_index = animations.len() - 1;
					} else {
						self.app.bridge.chat_state.current_animation_index -= 1;
					}
					self.app.bridge.chat_state.animation_start_time = Some(Instant::now());
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				KeyCode::Right => {
					// Next animation (but not Enter - Enter submits input)
					self.app.bridge.chat_state.current_animation_index = 
						(self.app.bridge.chat_state.current_animation_index + 1) % animations.len();
					self.app.bridge.chat_state.animation_start_time = Some(Instant::now());
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				_ => {
					// For other keys, fall through to handle input
					// This allows typing while viewing animations
				}
			}
		}
		
		// Global menu navigation keys - work when menu is visible on ANY screen
		if self.app.bridge.chat_state.show_tachyon_menu {
			// Check if we're in recording mode in keyboard shortcuts submenu
			if self.app.bridge.chat_state.menu.recording_mode 
				&& self.app.bridge.chat_state.menu.current_submenu == Some(1)
			{
				// Get the selected shortcut index (skip "Back" and "Toggle Recording Mode")
				if let Some(action_index) = self.app.bridge.chat_state.menu.get_selected_shortcut_index() {
					// Format the key press into a shortcut string
					let shortcut = format_key_event(&key);
					
					// Don't record navigation keys or special menu keys
					if !matches!(key.code, 
						KeyCode::Up | KeyCode::Down | KeyCode::PageUp | KeyCode::PageDown | 
						KeyCode::Home | KeyCode::End | KeyCode::Esc | KeyCode::Enter |
						KeyCode::Char('j') | KeyCode::Char('k') | KeyCode::Char('g') | KeyCode::Char('G')
					) {
						// Update the keyboard shortcut
						self.app.bridge.chat_state.menu.update_keyboard_shortcut(action_index, shortcut);
						NEED_RENDER.store(1, Ordering::Relaxed);
						succ!()
					}
				}
			}
			
			match key.code {
				KeyCode::Up | KeyCode::Char('k') => {
					self.app.bridge.chat_state.menu.select_prev_menu_item();
					// Apply theme preview if in theme submenu
					if let Some(theme_name) = self.app.bridge.chat_state.menu.get_highlighted_theme_name() {
						self.app.bridge.chat_state.apply_theme(&theme_name, self.app.bridge.chat_state.theme_mode);
					}
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				KeyCode::Down | KeyCode::Char('j') => {
					self.app.bridge.chat_state.menu.select_next_menu_item();
					// Apply theme preview if in theme submenu
					if let Some(theme_name) = self.app.bridge.chat_state.menu.get_highlighted_theme_name() {
						self.app.bridge.chat_state.apply_theme(&theme_name, self.app.bridge.chat_state.theme_mode);
					}
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				KeyCode::PageUp => {
					self.app.bridge.chat_state.menu.page_up(10);
					// Apply theme preview if in theme submenu
					if let Some(theme_name) = self.app.bridge.chat_state.menu.get_highlighted_theme_name() {
						self.app.bridge.chat_state.apply_theme(&theme_name, self.app.bridge.chat_state.theme_mode);
					}
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				KeyCode::PageDown => {
					self.app.bridge.chat_state.menu.page_down(10);
					// Apply theme preview if in theme submenu
					if let Some(theme_name) = self.app.bridge.chat_state.menu.get_highlighted_theme_name() {
						self.app.bridge.chat_state.apply_theme(&theme_name, self.app.bridge.chat_state.theme_mode);
					}
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				KeyCode::Home | KeyCode::Char('g') => {
					self.app.bridge.chat_state.menu.jump_to_top();
					// Apply theme preview if in theme submenu
					if let Some(theme_name) = self.app.bridge.chat_state.menu.get_highlighted_theme_name() {
						self.app.bridge.chat_state.apply_theme(&theme_name, self.app.bridge.chat_state.theme_mode);
					}
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				KeyCode::End | KeyCode::Char('G') => {
					self.app.bridge.chat_state.menu.jump_to_bottom();
					// Apply theme preview if in theme submenu
					if let Some(theme_name) = self.app.bridge.chat_state.menu.get_highlighted_theme_name() {
						self.app.bridge.chat_state.apply_theme(&theme_name, self.app.bridge.chat_state.theme_mode);
					}
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				KeyCode::Char('t') | KeyCode::Char('T') => {
					// Toggle light/dark mode when in theme submenu
					if self.app.bridge.chat_state.menu.current_submenu == Some(0) {
						self.app.bridge.chat_state.toggle_theme_mode();
						NEED_RENDER.store(1, Ordering::Relaxed);
						succ!()
					}
				}
				KeyCode::Enter => {
					// Check if toggle mode button is selected
					if self.app.bridge.chat_state.menu.is_toggle_mode_selected() {
						// Toggle the theme mode
						self.app.bridge.chat_state.toggle_theme_mode();
						NEED_RENDER.store(1, Ordering::Relaxed);
						succ!()
					}
					
					// Check if toggle recording button is selected
					if self.app.bridge.chat_state.menu.is_toggle_recording_selected() {
						// Toggle the recording mode
						self.app.bridge.chat_state.menu.toggle_recording_mode();
						NEED_RENDER.store(1, Ordering::Relaxed);
						succ!()
					}
					
					// Get the current theme name before selecting
					let theme_name = self.app.bridge.chat_state.menu.get_selected_theme_name();
					
					// Select current menu item (enter submenu or execute action)
					let _should_close = !self.app.bridge.chat_state.menu.select_current_item();
					
					// If we were in theme submenu and selected a theme, just close the menu
					// (theme is already applied from navigation/hover)
					if theme_name.is_some() {
						self.app.bridge.chat_state.menu_is_closing = true;
						self.app.bridge.chat_state.menu.pick_closing_effect();
						self.app.bridge.chat_state.show_tachyon_menu = false;
					}
					
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				KeyCode::Esc => {
					// Go back to main menu if in submenu, otherwise close menu
					if self.app.bridge.chat_state.menu.current_submenu.is_some() {
						self.app.bridge.chat_state.menu.go_back_to_main();
						NEED_RENDER.store(1, Ordering::Relaxed);
						succ!()
					} else {
						// Close menu
						self.app.bridge.chat_state.menu_is_closing = true;
						self.app.bridge.chat_state.menu.pick_closing_effect();
						self.app.bridge.chat_state.show_tachyon_menu = false;
						NEED_RENDER.store(1, Ordering::Relaxed);
						succ!()
					}
				}
				_ => {}
			}
		}
		
		// Global '0' key handler - toggle menu overlay on ANY screen
		if key.code == KeyCode::Char('0') {
			if self.app.bridge.chat_state.show_tachyon_menu {
				// Closing menu - pick random closing animation
				self.app.bridge.chat_state.menu_is_closing = true;
				self.app.bridge.chat_state.menu.pick_closing_effect();
				self.app.bridge.chat_state.show_tachyon_menu = false;
			} else {
				// Opening menu - pick random opening animation
				self.app.bridge.chat_state.menu_is_closing = false;
				self.app.bridge.chat_state.show_tachyon_menu = true;
				self.app.bridge.chat_state.menu.pick_opening_effect();
			}
			
			NEED_RENDER.store(1, Ordering::Relaxed);
			succ!()
		}
		
		// Global keyboard shortcuts - check if any registered shortcut matches
		if !self.app.bridge.chat_state.show_tachyon_menu {
			use crate::tui::menu::MenuAction;
			
			let pressed_key = format_key_event(&key);
			let mappings = &self.app.bridge.chat_state.menu.keyboard_mappings;
			
			// Check each action to see if its shortcut matches
			for action in MenuAction::all_actions() {
				let shortcut = mappings.get(action);
				if shortcut == pressed_key {
					// Match found! Open menu and navigate to the corresponding submenu
					let submenu_index = match action {
						MenuAction::ContextControlPanel => 0, // Special case - just open menu
						MenuAction::Theme => 0,
						MenuAction::KeyboardShortcuts => 1,
						MenuAction::Providers => 2,
						MenuAction::PluginsApps => 3,
						MenuAction::Skills => 4,
						MenuAction::Sandbox => 5,
						MenuAction::WebSearch => 6,
						MenuAction::McpServers => 7,
						MenuAction::MemoryHistory => 8,
						MenuAction::MultiAgent => 9,
						MenuAction::Notifications => 10,
						MenuAction::VoiceRealtime => 11,
						MenuAction::ImageVision => 12,
						MenuAction::Profiles => 13,
						MenuAction::Worktree => 14,
						MenuAction::Authentication => 15,
						MenuAction::NetworkProxy => 16,
						MenuAction::HooksEvents => 17,
						MenuAction::SessionResume => 18,
						MenuAction::ApprovalPolicy => 19,
						MenuAction::ShellEnvironment => 20,
						MenuAction::ExecutionRules => 21,
						MenuAction::ProjectTrust => 22,
						MenuAction::DeveloperInstructions => 23,
						MenuAction::FeatureFlags => 24,
					};
					
					// Open menu if not already open
					if !self.app.bridge.chat_state.show_tachyon_menu {
						self.app.bridge.chat_state.menu_is_closing = false;
						self.app.bridge.chat_state.show_tachyon_menu = true;
						self.app.bridge.chat_state.menu.pick_opening_effect();
					}
					
					// Navigate to the submenu (unless it's ContextControlPanel which just opens menu)
					if !matches!(action, MenuAction::ContextControlPanel) {
						self.app.bridge.chat_state.menu.enter_submenu(submenu_index);
					}
					
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
			}
		}
		
		// Handle chat input when in Chat mode or FilePicker mode (chat input is visible)
		if self.app.bridge.mode == crate::tui::AppMode::Chat 
			|| self.app.bridge.mode == crate::tui::AppMode::FilePicker 
		{
			// Handle scrolling when messages exist and input is empty
			if !self.app.bridge.chat_state.messages.is_empty() 
				&& self.app.bridge.chat_state.input.content.is_empty() 
			{
				match key.code {
					KeyCode::Up => {
						self.app.bridge.chat_state.chat_scroll_offset = 
							self.app.bridge.chat_state.chat_scroll_offset.saturating_sub(1);
						NEED_RENDER.store(1, Ordering::Relaxed);
						succ!()
					}
					KeyCode::Down => {
						self.app.bridge.chat_state.chat_scroll_offset += 1;
						NEED_RENDER.store(1, Ordering::Relaxed);
						succ!()
					}
					_ => {}
				}
			}
			// Route key to chat input
			let action = self.app.bridge.chat_state.input.handle_key(key);
			
			match action {
				InputAction::Submit(msg) => {
					// Add message to chat - this exits animation mode
					self.app.bridge.chat_state.add_user_message(msg);
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				InputAction::Exit => {
					// Show farewell train animation
					crate::tui::exit_animation::show_train_farewell();
					// Exit the application
					std::process::exit(0);
				}
				InputAction::Changed => {
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				InputAction::PreviousHistory | InputAction::NextHistory => {
					// TODO: Implement history navigation
					succ!()
				}
				InputAction::None => {
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
			}
		}
		// Route to yazi's normal key handling
		Router::new(self.app).route(Key::from(key))?;
		succ!();
	}

	#[inline]
	fn dispatch_mouse(&mut self, mouse: MouseEvent) -> Result<Data> {
		use crossterm::event::MouseEventKind;
		
		// Handle menu mouse events globally when menu is visible
		if self.app.bridge.chat_state.show_tachyon_menu {
			match mouse.kind {
				MouseEventKind::Moved => {
					// Handle hover - always process and render if state changed
					if self.app.bridge.chat_state.menu.handle_mouse(mouse.column, mouse.row, false) {
						// Apply theme preview if hovering over a theme
						if let Some(theme_name) = self.app.bridge.chat_state.menu.get_hovered_theme_name() {
							self.app.bridge.chat_state.apply_theme(&theme_name, self.app.bridge.chat_state.theme_mode);
						}
					}
					NEED_RENDER.store(1, Ordering::Relaxed);
				}
				MouseEventKind::Down(_) => {
					// Handle click - select and potentially enter submenu
					if self.app.bridge.chat_state.menu.handle_mouse(mouse.column, mouse.row, true) {
						// Check if toggle mode button is clicked
						if self.app.bridge.chat_state.menu.is_toggle_mode_selected() {
							// Toggle the theme mode
							self.app.bridge.chat_state.toggle_theme_mode();
							NEED_RENDER.store(1, Ordering::Relaxed);
							succ!()
						}
						
						// Check if toggle recording button is clicked
						if self.app.bridge.chat_state.menu.is_toggle_recording_selected() {
							// Toggle the recording mode
							self.app.bridge.chat_state.menu.toggle_recording_mode();
							NEED_RENDER.store(1, Ordering::Relaxed);
							succ!()
						}
						
						// Get the current theme name before selecting
						let theme_name = self.app.bridge.chat_state.menu.get_selected_theme_name();
						
						// Item was clicked - now select it (enter submenu or execute)
						let _should_close = !self.app.bridge.chat_state.menu.select_current_item();
						
						// If we were in theme submenu and clicked a theme, just close the menu
						// (theme is already applied from hover)
						if theme_name.is_some() {
							self.app.bridge.chat_state.menu_is_closing = true;
							self.app.bridge.chat_state.menu.pick_closing_effect();
							self.app.bridge.chat_state.show_tachyon_menu = false;
						}
						
						NEED_RENDER.store(1, Ordering::Relaxed);
					}
				}
				MouseEventKind::ScrollUp => {
					// Scroll up (previous items)
					self.app.bridge.chat_state.menu.select_prev_menu_item();
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				MouseEventKind::ScrollDown => {
					// Scroll down (next items)
					self.app.bridge.chat_state.menu.select_next_menu_item();
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				_ => {}
			}
		}
		
		let cx = &mut Ctx::active(&mut self.app.core, &mut self.app.term);
		act!(app:mouse, cx, mouse)
	}

	#[inline]
	fn dispatch_resize(&mut self) -> Result<Data> {
		let cx = &mut Ctx::active(&mut self.app.core, &mut self.app.term);
		act!(app:resize, cx, crate::Root::reflow as fn(_) -> _)
	}

	#[inline]
	fn dispatch_focus(&mut self) -> Result<Data> {
		let cx = &mut Ctx::active(&mut self.app.core, &mut self.app.term);
		act!(app:focus, cx)
	}

	#[inline]
	fn dispatch_paste(&mut self, str: String) -> Result<Data> {
		if self.app.core.input.visible {
			let input = &mut self.app.core.input;
			if input.mode() == InputMode::Insert {
				input.type_str(&str)?;
			} else if input.mode() == InputMode::Replace {
				input.replace_str(&str)?;
			}
		}
		succ!();
	}

	#[inline]
	fn dispatch_timer(&mut self) -> Result<Data> {
		// Timer tick for animations - just trigger a render
		// The effects are time-based and will automatically show updated colors
		
		// Update chat state (process LLM responses)
		self.app.bridge.chat_state.update();
		
		// Update splash font cycling (every 3 seconds)
		if self.app.bridge.chat_state.animation_mode 
			&& self.app.bridge.chat_state.last_font_change.elapsed() >= Duration::from_secs(3)
		{
			let animations = crate::tui::AnimationType::all();
			let current_anim = animations[self.app.bridge.chat_state.current_animation_index];
			if current_anim == crate::tui::AnimationType::Splash {
				self.app.bridge.chat_state.splash_font_index = 
					(self.app.bridge.chat_state.splash_font_index + 1) % 113; // 113 valid fonts
				self.app.bridge.chat_state.last_font_change = Instant::now();
			}
		}
		
		// Update Menu timing
		let elapsed = self.app.bridge.chat_state.last_frame_instant.elapsed();
		self.app.bridge.chat_state.menu.update(elapsed);
		self.app.bridge.chat_state.last_frame_instant = Instant::now();
		
		NEED_RENDER.store(1, Ordering::Relaxed);
		succ!();
	}
}
